//! TODO doc

use crate::lang::Language;
use crate::partition::Partition;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

/// Structure storing installation informations.
#[derive(Default, Deserialize, Serialize)]
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

	/// The partition scheme to be used.
	pub partitions_scheme: Vec<Partition>,
}

impl InstallInfo {
	/// TODO doc
	fn partition_disks(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn create_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn mount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn install_packages(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn set_locales(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn set_hostname(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn create_users(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// TODO doc
	fn unmount_filesystems(&self) -> Result<(), Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Performs the installation operation.
	pub fn perform_install(&self) -> Result<(), Box<dyn Error>> {
		self.partition_disks()?;
		self.create_filesystems()?;
		self.mount_filesystems()?;
		self.install_packages()?;
		self.set_locales()?;
		self.set_hostname()?;
		self.create_users()?;
		self.unmount_filesystems()?;

		Ok(())
	}
}

/// Structure representing the current progress of the installation.
pub struct InstallProgress {
	/// Logs.
	logs: Vec<String>,

	/// Progress in percent, between 0 and 1000.
	progress: u16,
}
