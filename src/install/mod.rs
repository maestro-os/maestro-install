//! TODO doc

use crate::lang::Language;
use crate::partition::Partition;
use crate::prompt::InstallPrompt;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
	pub selected_disk: String,
	/// The partition scheme to be used.
	pub partitions: Vec<Partition>,
}

impl InstallInfo {
	/// Creates partitions on the disk.
	fn partition_disks(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Creates a filesystem on each partition.
	fn create_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Mounts filesystems to install the system on them.
	fn mount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
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
			println!("Create directory `{}`", path.display());

			fs::create_dir(path)?;
		}

		Ok(())
	}

	/// Installs packages on the system.
	fn install_packages(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Sets localization options.
	fn set_locales(&self) -> Result<(), Box<dyn Error>> {
		// TODO
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
	fn create_users(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Unmounts filesystems to finalize the installation.
	fn unmount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
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

		let mnt_path = PathBuf::from("mnt/");
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
		self.install_packages()?;

		progress.log(&format!("\nSet locales\n"));
		self.set_locales()?;

		progress.log(&format!("\nSet hostname\n"));
		self.set_hostname(&mnt_path)?;

		progress.log(&format!("\nCreate users and groups\n"));
		self.create_users()?;

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
		self.logs.append(&mut s.split('\n').map(|s| s.to_owned()).collect());
		// FIXME self.prompt.update_progress(self);
	}

	/// Returns an immutable reference to the installation logs.
	pub fn get_logs(&self) -> &[String] {
		self.logs.as_slice()
	}

	/// Returns the current percentage of advancement of the installation, represented by a value between 0 and 1000.
	pub fn get_progress(&self) -> u16 {
		self.progress
	}

	/// Sets the current percentage of advancement of the installation, represented by a value between 0 and 1000.
	pub fn set_progress(&mut self, progress: u16) {
		self.progress = progress;
		// FIXME self.prompt.update_progress(self);
	}
}
