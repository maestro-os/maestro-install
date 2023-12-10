//! Installation utility for the Maestro operating system.

mod install;
mod lang;
mod prompt;
mod util;

use prompt::term::TermPrompt;
use prompt::term::{CODE_RED, CODE_RESET};
use prompt::InstallPrompt;
use prompt::InstallStep;
use std::env;
use std::process::exit;

fn main() {
	// Get prompt type
	let prompt_type = env::args().nth(1);
	let prompt_type = prompt_type.as_deref().unwrap_or("term");
	// Create prompt
	let mut prompt = match prompt_type {
		"term" => TermPrompt::new(),
		// TODO Add support for GUI
		_ => {
			eprintln!("Invalid prompt type: {prompt_type}");
			exit(1);
		}
	};

	while let Some(curr_step) = prompt.get_current_step() {
		prompt.next_step();
		if matches!(curr_step, InstallStep::Install) {
			let infos = prompt.get_infos();
			if let Err(e) = infos.perform_install(&mut prompt) {
				eprintln!("{CODE_RED}Installation failed: {e}{CODE_RESET}");
				exit(1);
			}
		}
	}
}
