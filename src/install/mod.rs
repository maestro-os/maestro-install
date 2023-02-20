//! TODO doc

use common::Environment;
use common::repository::Repository;
use crate::lang::Language;
use crate::prompt::InstallPrompt;
use fdisk::disk::Disk;
use fdisk::disk;
use fdisk::partition::GUID;
use fdisk::partition::Partition;
use fdisk::partition::PartitionTable;
use fdisk::partition::PartitionTableType;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::fs::Permissions;
use std::fs;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use utils::user::Group;
use utils::user::Shadow;
use utils::user::User;
use utils::user;
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
	pub mount_path: PathBuf,
}

impl fmt::Display for PartitionDesc {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.bootable {
			write!(
				fmt,
				"{} - start: {}; size: {} sectors, bootable",
				self.mount_path.display(), self.start, self.size
			)
		} else {
			write!(
				fmt,
				"{} - start: {}; size: {} sectors",
				self.mount_path.display(), self.start, self.size
			)
		}
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

	/// The admin's username.
	pub admin_user: String,
	/// The admin's password.
	pub admin_pass: String,

	/// The path to the disk on which the system is to be installed.
	pub selected_disk: PathBuf,
	/// The partition scheme to be used.
	pub partitions: Vec<PartitionDesc>,
}

impl InstallInfo {
	/// Creates partitions on the disk.
	fn partition_disks(&self) -> Result<(), Box<dyn Error>> {
		println!("Creating partition table on `{}`...", self.selected_disk.display());

		let partitions = self.partitions.iter()
			.map(|desc| {
				let uuid = GUID::random().unwrap(); // TODO handle error

				Partition {
					start: desc.start,
					size: desc.size,

					part_type: desc.part_type.as_str().try_into().unwrap(), // TODO handle error

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
			let part_nbr = i + 1;

			// TODO support nvme
			let mut dev_path = self.selected_disk.clone().into_os_string();
			dev_path.push(format!("{}", part_nbr));

			// TODO use ext4
			let status = Command::new("mkfs.ext2")
				.arg(dev_path)
				.status()?;

			if !status.success() {
				return Err(format!("Cannot create filesystem on partition number {}", i).into());
			}
		}

		Ok(())
	}

	/// Mounts filesystems to install the system on them.
	fn mount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// Ensure partitions are mount in the right order
		let mut parts: Vec<(usize, &PartitionDesc)> = self.partitions.iter()
			.enumerate()
			.collect();
		parts.sort_unstable_by(|(_, a), (_, b)| a.mount_path.cmp(&b.mount_path));

		for (i, part) in parts {
			let part_nbr = i + 1;

			// TODO support nvme
			let mut dev_path = self.selected_disk.clone().into_os_string();
			dev_path.push(format!("{}", part_nbr));

			let mnt_path = common::util::concat_paths(Path::new("/mnt"), &part.mount_path);
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
		let mut paths: Vec<PathBuf> = vec![
			"bin".into(),
			"boot".into(),
			"dev".into(),
			"etc".into(),
			"home".into(),
			"lib".into(),
			"media".into(),
			"mnt".into(),
			"opt".into(),
			"proc".into(),
			"root".into(),
			"run".into(),
			"sbin".into(),
			"srv".into(),
			"sys".into(),
			"tmp".into(),
			"usr".into(),
			"var".into(),
			"etc/opt".into(),
			"etc/sysconfig".into(),
			"lib/firmware".into(),
			"media/floppy".into(),
			"media/cdrom".into(),
			"run/lock".into(),
			"run/log".into(),
			"usr/bin".into(),
			"usr/include".into(),
			"usr/lib".into(),
			"usr/local".into(),
			"usr/sbin".into(),
			"usr/share".into(),
			"usr/src".into(),
			"usr/share/color".into(),
			"usr/share/dict".into(),
			"usr/share/doc".into(),
			"usr/share/info".into(),
			"usr/share/locale".into(),
			"usr/share/man".into(),
			"usr/share/misc".into(),
			"usr/share/terminfo".into(),
			"usr/share/zoneinfo".into(),
			"usr/local/bin".into(),
			"usr/local/include".into(),
			"usr/local/lib".into(),
			"usr/local/sbin".into(),
			"usr/local/share".into(),
			"usr/local/src".into(),
			"usr/local/share/color".into(),
			"usr/local/share/dict".into(),
			"usr/local/share/doc".into(),
			"usr/local/share/info".into(),
			"usr/local/share/locale".into(),
			"usr/local/share/man".into(),
			"usr/local/share/misc".into(),
			"usr/local/share/terminfo".into(),
			"usr/local/share/zoneinfo".into(),
			"var/cache".into(),
			"var/lib".into(),
			"var/local".into(),
			"var/log".into(),
			"var/mail".into(),
			"var/opt".into(),
			"var/spool".into(),
			"var/lib/color".into(),
			"var/lib/misc".into(),
			"var/lib/locate".into(),
		];
		for i in 1..=8 {
			paths.push(format!("usr/share/man/man{}", i).into());
			paths.push(format!("usr/local/share/man/man{}", i).into());
		}

		for path in paths {
			let path = mnt_path.clone().join(path);
			if !path.exists() {
				println!("Create directory `{}`", path.display());
				fs::create_dir(path)?;
			}
		}

		Ok(())
	}

	/// Installs packages on the system.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn install_packages(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		println!("Install packages...");
		println!();

		let env = Environment::with_root(mnt_path.into()).unwrap();
		// TODO add option to use remote repo
		let repo = Repository::load("/local_repo".into())?;

		for pkg in repo.list_packages()? {
			println!("Installing package `{}` (version {})...", pkg.get_name(), pkg.get_version());

			let archive_path = repo.get_archive_path(pkg.get_name(), pkg.get_version());
			env.install(&pkg, &archive_path)?;
		}

		Ok(())
	}

	/// Sets localization options.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn set_locales(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let path = mnt_path.join("etc").join("locale.conf");

		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)?;

		let locale = self.lang.as_ref().unwrap().get_locale();
		let content = format!("LC_ALL={}\nLANG={}\n", locale, locale);
		file.write(content.as_bytes())?;

		// TODO generate locale

		Ok(())
	}

	/// Creates the hostname file.
	///
	/// `mnt_path` is the path to the root filesystem's mountpoint.
	fn set_hostname(&self, mnt_path: &Path) -> Result<(), Box<dyn Error>> {
		let path = mnt_path.join("etc").join("hostname");

		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)?;
		file.write(self.hostname.as_bytes())?;

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
		// Write /etc/passwd
		let users = vec![
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
				uid: 1000,
				gid: 1000,
				comment: "".into(),
				home: format!("/home/{}", self.admin_user).into(),
				interpreter: "/bin/bash".into(),
			}
		];
		let passwd_path = mnt_path.join("etc/passwd");
		user::write_passwd(&passwd_path, &users)?;
		fs::set_permissions(passwd_path, Permissions::from_mode(0o644))?;

		// Write /etc/shadow
		let admin_pass = user::hash_password(&self.admin_pass);
		let last_change = (get_timestamp().as_secs() / 3600 / 24) as u32;
		let shadows = vec![
			Shadow {
				login_name: "root".into(),
				password: admin_pass.clone(),
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
				password: admin_pass,
				last_change,
				minimum_age: None,
				maximum_age: None,
				warning_period: None,
				inactivity_period: None,
				account_expiration: None,
				reserved: "".into(),
			}
		];
		let shadow_path = mnt_path.join("etc/shadow");
		user::write_shadow(&shadow_path, &shadows)?;
		fs::set_permissions(shadow_path, Permissions::from_mode(0o600))?;

		// Write /etc/group
		let groups = vec![
			Group {
				group_name: "root".into(),
				password: "x".into(),
				gid: 0,
				users_list: "root".into(),
			},

			Group {
				group_name: self.admin_user.clone(),
				password: "x".into(),
				gid: 1000,
				users_list: self.admin_user.clone(),
			}
		];
		let group_path = mnt_path.join("etc/group");
		user::write_group(&group_path, &groups)?;
		fs::set_permissions(group_path, Permissions::from_mode(0o644))?;

		Ok(())
	}

	/// Unmounts filesystems to finalize the installation.
	fn unmount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		let status = Command::new("umount")
			.args(&["-R", "mnt/"])
			.status()?;

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

		let mnt_path = PathBuf::from("/mnt");
		progress.log(&format!("Create directory `{}`\n", mnt_path.display()));
		fs::create_dir(&mnt_path)?;

		progress.log(&format!("\nPartition disk\n"));
		self.partition_disks()?;

		progress.log(&format!("\nCreate filesystems\n"));
		self.create_filesystems()?;

		progress.log(&format!("\nMount filesystems\n"));
		self.mount_filesystems()?;

		progress.log(&format!("\nCreate directory structure\n"));
		self.create_dirs(&mnt_path)?;

		progress.log(&format!("\nInstall packages\n"));
		self.install_packages(&mnt_path)?;

		progress.log(&format!("\nSet locales\n"));
		self.set_locales(&mnt_path)?;

		progress.log(&format!("\nSet hostname\n"));
		self.set_hostname(&mnt_path)?;

		progress.log(&format!("\nCreate users and groups\n"));
		self.create_users(&mnt_path)?;

		progress.log(&format!("\nUnmount filesystems\n"));
		self.unmount_filesystems()?;

		progress.log(&format!("\nDone!\n"));

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
		print!("{}", s);

		self.logs
			.append(&mut s.split('\n').map(|s| s.to_owned()).collect());
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
