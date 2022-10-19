//! TODO doc

mod prompt;

use prompt::InstallPrompt;
use prompt::InstallStep;
use prompt::term::TermPrompt;

/// Structure storing installation informations.
#[derive(Default)]
pub struct InstallInfo {
	// TODO
}

impl InstallInfo {
	/// Performs the installation operation.
	pub fn perform_install(&self) {
		// TODO
		todo!();
	}
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
