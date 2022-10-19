//! TODO doc

/// Structure storing informations about a partition.
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
#[derive(Default)]
pub struct InstallInfo {
	/// The system's language.
	pub lang: String,
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
		// TODO
		todo!();
	}
}

/// Structure representing the curren progress of the installation.
pub struct InstallProgress {
	/// Logs.
	logs: Vec<String>,

	/// Progress in percent, between 0 and 1000.
	progress: u16,
}
