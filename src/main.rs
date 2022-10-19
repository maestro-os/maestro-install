//! TODO doc

mod prompt;

use prompt::InstallPrompt;
use prompt::InstallStep;
use prompt::term::TermPrompt;

/// Structure storing informations about a partition.
pub struct Partition {
	/// The path of the device.
	dev: String,

	/// The start offset.
	start: u64,
	/// The end offset.
	end: u64,

	/// The partition type.
	part_type: u8,
}

/// Structure storing installation informations.
#[derive(Default)]
pub struct InstallInfo {
	/// The system's language.
	lang: String,
	/// The system's country.
	country: String,
	/// The system's timezone.
	tz: String,

	/// The system's hostname.
	hostname: String,

	/// The admin's username.
	admin_user: String,
	/// The admin's password.
	admin_pass: String,

	/// The partition scheme to be used.
	partitions_scheme: Vec<Partition>,
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

fn main() {
	let mut prompt = TermPrompt::new();

	while let Some(curr_step) = prompt.get_current_step() {
		prompt.next_step();

		if matches!(curr_step, InstallStep::Install) {
			let infos = prompt.get_infos();
			infos.perform_install();
		}
	}
}
