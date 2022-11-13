//! TODO doc

use crate::lang::Language;
use serde::Deserialize;
use serde::Serialize;

/// Structure storing informations about a partition.
#[derive(Deserialize, Serialize)]
pub struct Partition {
	/// The path of the device.
	pub dev: String,

	/// The start offset.
	pub start: u64,
	/// The end offset.
	pub end: u64,

	/// The partition type.
	pub part_type: u8,
}

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
	/// Performs the installation operation.
	pub fn perform_install(&self) {
		// TODO Partition disks
		// TODO Create filesystems
		// TODO Mount filesystems
		// TODO Install packages
		// TODO Set locales
		// TODO Set hostname
		// TODO Create users and groups
		// TODO Unmount filesystems

		todo!();
	}
}

/// Structure representing the current progress of the installation.
pub struct InstallProgress {
	/// Logs.
	logs: Vec<String>,

	/// Progress in percent, between 0 and 1000.
	progress: u16,
}
