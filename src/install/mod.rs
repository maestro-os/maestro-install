//! This module handles the installation procedure.

use crate::lang::Language;
use crate::prompt::InstallPrompt;
use common::repository::Repository;
use common::Environment;
use fdisk::disk;
use fdisk::disk::Disk;
use fdisk::guid::GUID;
use fdisk::partition::PartitionTable;
use fdisk::partition::PartitionTableType;
use fdisk::partition::{Partition, PartitionType};
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::fs::Permissions;
use std::io::ErrorKind;
use std::io::Write;
use std::os::unix::fs::chown;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use utils::user;
use utils::user::Group;
use utils::user::Shadow;
use utils::user::User;
use utils::util::get_timestamp;

// TODO Use InstallProgress instead of printing directly

/// Structure representing a partition to be created.
#[derive(Clone, Deserialize, Serialize)]
pub struct PartitionDesc {
	/// The start offset of the partition in sectors.
	pub start: u64,
	/// The size of the partition in sectors.
	pub size: u64,

	/// The partition type.
	pub part_type: String,

	/// Tells whether the partition is bootable.
	pub bootable: bool,

	/// The path at which the partition is to be mounted for installation.
	///
	/// If None, the partition shouldn't be mounted.
	pub mount_path: Option<PathBuf>,
}

impl fmt::Display for PartitionDesc {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "start: {}, size: {} sectors", self.start, self.size)?;
		if self.bootable {
			write!(fmt, ", bootable")?;
		}
		if let Some(mount_path) = &self.mount_path {
			write!(fmt, ", mount path: {} ", mount_path.display())?;
		}
		Ok(())
	}
}

/// Structure storing installation informations.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct InstallInfo {
	/// The system's language.
	pub lang: Option<Language>,
	/// The system's country.
	pub country: String,
	/// The system's timezone.
	pub tz: String,

	/// The system's hostname.
	pub hostname: String,

	/// Admin username.
	pub admin_user: String,
	/// Hashed admin password.
	pub admin_pass: String,

	/// The path to the disk on which the system is to be installed.
	pub selected_disk: PathBuf,
	/// The partition scheme to be used.
	pub partitions: Vec<PartitionDesc>,
}

impl InstallInfo {
	/// Creates partitions on the disk.
	fn partition_disks(&self) -> Result<(), Box<dyn Error>> {
		println!(
			"Create partition table on `{}`...",
			self.selected_disk.display()
		);

		let partitions = self
			.partitions
			.iter()
			.map(|desc| {
				// TODO handle error
				let part_type = PartitionType::from_str(desc.part_type.as_str()).unwrap();
				let uuid = GUID::random();
				Partition {
					start: desc.start,
					size: desc.size,

					part_type,

					uuid: Some(uuid),

					bootable: desc.bootable,
				}
			})
			.collect();
		let partition_table = PartitionTable {
			table_type: PartitionTableType::GPT,
			partitions,
		};

		let mut disk = Disk::read(self.selected_disk.clone())?.unwrap();
		disk.partition_table = partition_table;
		disk.write()?;
		disk::read_partitions(&self.selected_disk)?;

		Ok(())
	}

	/// Creates a filesystem on each partition.
	fn create_filesystems(&self) -> Result<(), Box<dyn Error>> {
		for (i, part) in self.partitions.iter().enumerate() {
			if part.mount_path.is_none() {
				continue;
			}

			// TODO support nvme
			let dev_path = format!("{}{}", self.selected_disk.display(), i + 1);

			println!("Create filesystem on `{dev_path}`");

			// TODO use ext4
			let status = Command::new("mkfs.ext2").arg(dev_path).status()?;
			if !status.success() {
				return Err(format!("Filesystem creation failed!").into());
			}
		}
		Ok(())
	}

	/// Mounts filesystems to install the system on them.
	fn mount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// Ensure partitions are mount in the right order
		let mut parts: Vec<(usize, &PartitionDesc)> = self.partitions.iter().enumerate().collect();
		parts.sort_unstable_by(|(_, a), (_, b)| a.mount_path.cmp(&b.mount_path));

		for (i, part) in parts {
			let Some(mnt_path) = &part.mount_path else {
				continue;
			};

			// TODO support nvme
			let dev_path = format!("{}{}", self.selected_disk.display(), i + 1);
			let mnt_path = common::util::concat_paths(Path::new("/mnt"), mnt_path);

			println!("Mount `{dev_path}` at `{}`", mnt_path.display());

			// Perform mount
			fs::create_dir_all(&mnt_path)?;
			let status = Command::new("mount")
				.arg(dev_path)
				.arg(&mnt_path)
				.status()?;
			if !status.success() {
				return Err(format!("Cannot mount partition at `{}`", mnt_path.display()).into());
			}
		}
		Ok(())
	}

	/// Creates the folder hierarchy on the disk.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn create_dirs(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let paths = &[
			"bin",
			"boot",
			"dev",
			"etc",
			"home",
			"lib",
			"media",
			"mnt",
			"opt",
			"proc",
			"root",
			"run",
			"sbin",
			"srv",
			"sys",
			"tmp",
			"usr",
			"var",
			"etc/opt",
			"etc/sysconfig",
			"lib/firmware",
			"run/lock",
			"run/log",
			"usr/bin",
			"usr/include",
			"usr/lib",
			"usr/local",
			"usr/sbin",
			"usr/share",
			"usr/src",
			"usr/share/doc",
			"usr/share/info",
			"usr/share/locale",
			"usr/share/man",
			"usr/share/misc",
			"usr/local/bin",
			"usr/local/include",
			"usr/local/lib",
			"usr/local/sbin",
			"usr/local/share",
			"usr/local/src",
			"usr/local/share/doc",
			"usr/local/share/info",
			"usr/local/share/locale",
			"usr/local/share/man",
			"usr/local/share/misc",
			"var/cache",
			"var/lib",
			"var/local",
			"var/log",
			"var/mail",
			"var/opt",
			"var/spool",
			"var/lib/misc",
		];
		for path in paths {
			println!("Create directory `{path}`");
			let path = mnt_path.join(path);
			match fs::create_dir(path) {
				Ok(_) => {}
				Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
				Err(e) => return Err(e.into()),
			}
		}
		Ok(())
	}

	/// Installs packages on the system.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn install_packages(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		fs::create_dir_all(mnt_path.join("usr/lib/blimp"))?;

		let env = Environment::with_root(mnt_path.into()).unwrap();
		// TODO add option to use remote repo
		let repo = Repository::load("/local_repo".into())?;

		for pkg in repo.list_packages()? {
			let name = pkg.get_name();
			let version = pkg.get_version();
			println!("Install `{name}` (version {version})...");
			let archive_path = repo.get_archive_path(name, version);
			env.install(&pkg, &archive_path)?;
		}
		Ok(())
	}

	/// Installs the bootloader.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn install_bootloader(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let status = Command::new("grub-install")
			.arg("--target=i386-pc")
			.arg(format!(
				"--boot-directory={}",
				mnt_path.join("boot").display()
			))
			.arg(&self.selected_disk)
			.status()?;
		if !status.success() {
			return Err("Cannot install bootloader".into());
		}

		// Write `grub.cfg`
		let mut file = OpenOptions::new()
			.create(true)
			.truncate(true)
			.write(true)
			.open(mnt_path.join("boot/grub/grub.cfg"))?;
		file.write_all(include_bytes!("grub.cfg"))?;
		Ok(())
	}

	/// Sets localization options.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn set_locales(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let locale = self.lang.as_ref().unwrap().get_locale();

		let path = mnt_path.join("etc/locale.conf");
		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)?;
		writeln!(file, "LC_ALL={locale}")?;
		writeln!(file, "LANG={locale}")?;

		// TODO generate locale

		Ok(())
	}

	/// Creates the hostname file.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn set_hostname(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let path = mnt_path.join("etc/hostname");
		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)?;
		file.write_all(self.hostname.as_bytes())?;

		Ok(())
	}

	/// Creates users and groups.
	///
	/// The function creates:
	/// - `/etc/passwd`
	/// - `/etc/shadow`
	/// - `/etc/group`
	/// - The home directory for each user
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn create_users(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		// Create admin user's home
		let admin_uid = 1000;
		let admin_home = format!("/home/{}", self.admin_user).into();
		let admin_home_mnt = mnt_path.join(format!("home/{}", self.admin_user));
		fs::create_dir_all(&admin_home_mnt)?;
		chown(&admin_home_mnt, Some(admin_uid), Some(admin_uid))?;

		// Write /etc/passwd
		let users = [
			User {
				login_name: "root".into(),
				password: "x".into(),
				uid: 0,
				gid: 0,
				comment: "".into(),
				home: "/root".into(),
				interpreter: "/bin/bash".into(),
			},
			User {
				login_name: self.admin_user.clone(),
				password: "x".into(),
				uid: admin_uid,
				gid: admin_uid,
				comment: "".into(),
				home: admin_home,
				interpreter: "/bin/bash".into(),
			},
		];
		let passwd_path = mnt_path.join("etc/passwd");
		user::write_passwd(&passwd_path, &users)?;
		fs::set_permissions(passwd_path, Permissions::from_mode(0o644))?;

		// Write /etc/shadow
		let last_change = (get_timestamp().as_secs() / 3600 / 24) as u32;
		let shadows = [
			Shadow {
				login_name: "root".into(),
				password: self.admin_pass.clone(),
				last_change,
				minimum_age: None,
				maximum_age: None,
				warning_period: None,
				inactivity_period: None,
				account_expiration: None,
				reserved: "".into(),
			},
			Shadow {
				login_name: self.admin_user.clone(),
				password: self.admin_pass.clone(),
				last_change,
				minimum_age: None,
				maximum_age: None,
				warning_period: None,
				inactivity_period: None,
				account_expiration: None,
				reserved: "".into(),
			},
		];
		let shadow_path = mnt_path.join("etc/shadow");
		user::write_shadow(&shadow_path, &shadows)?;
		fs::set_permissions(shadow_path, Permissions::from_mode(0o600))?;

		// Write /etc/group
		let groups = [
			Group {
				group_name: "root".into(),
				password: "x".into(),
				gid: 0,
				users_list: "root".into(),
			},
			Group {
				group_name: self.admin_user.clone(),
				password: "x".into(),
				gid: admin_uid,
				users_list: self.admin_user.clone(),
			},
		];
		let group_path = mnt_path.join("etc/group");
		user::write_group(&group_path, &groups)?;
		fs::set_permissions(group_path, Permissions::from_mode(0o644))?;

		Ok(())
	}

	/// Unmounts filesystems to finalize the installation.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn unmount_filesystems(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let status = Command::new("umount").arg("-R").arg(mnt_path).status()?;
		if status.success() {
			Ok(())
		} else {
			Err("Cannot unmount filesystems".into())
		}
	}

	/// Performs the installation operation.
	///
	/// `prompt` is the prompt associated with the installation procedure.
	pub fn perform_install(&self, prompt: &mut dyn InstallPrompt) -> Result<(), Box<dyn Error>> {
		let mut progress = InstallProgress {
			prompt,

			logs: vec![],
			progress: 0,
		};

		let mnt_path = Path::new("/mnt");
		progress.log(&format!("Create directory `{}`\n", mnt_path.display()));
		fs::create_dir(&mnt_path)?;

		progress.log("\nPartition disk\n");
		self.partition_disks()?;

		progress.log("\nCreate filesystems\n");
		self.create_filesystems()?;

		progress.log("\nMount filesystems\n");
		self.mount_filesystems()?;

		progress.log("\nCreate directory structure\n");
		self.create_dirs(&mnt_path)?;

		progress.log("\nInstall packages\n");
		self.install_packages(&mnt_path)?;

		progress.log("\nInstall bootloader\n");
		self.install_bootloader(&mnt_path)?;

		progress.log("\nSet locales\n");
		self.set_locales(&mnt_path)?;

		progress.log("\nSet hostname\n");
		self.set_hostname(&mnt_path)?;

		progress.log("\nCreate users and groups\n");
		self.create_users(&mnt_path)?;

		progress.log("\nUnmount filesystems\n");
		self.unmount_filesystems(&mnt_path)?;

		progress.log("\nDone!\n");

		Ok(())
	}
}

/// Structure representing the current progress of the installation.
pub struct InstallProgress<'p> {
	/// The installation prompt.
	prompt: &'p mut dyn InstallPrompt,

	/// Logs.
	logs: Vec<String>,
	/// Progress in percent, between 0 and 1000.
	progress: u16,
}

impl<'p> InstallProgress<'p> {
	/// Inserts the given logs.
	pub fn log(&mut self, s: &str) {
		print!("{s}");

		self.logs
			.append(&mut s.split('\n').map(str::to_owned).collect());
		// FIXME self.prompt.update_progress(self);
	}

	/// Returns an immutable reference to the installation logs.
	pub fn get_logs(&self) -> &[String] {
		self.logs.as_slice()
	}

	/// Returns the current percentage of advancement of the installation, represented by a value
	/// between 0 and 1000.
	pub fn get_progress(&self) -> u16 {
		self.progress
	}

	/// Sets the current percentage of advancement of the installation, represented by a value
	/// between 0 and 1000.
	pub fn set_progress(&mut self, progress: u16) {
		self.progress = progress;
		// FIXME self.prompt.update_progress(self);
	}
}
