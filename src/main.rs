//! Installation utility for the Maestro operating system.

#![feature(unix_chown)]

mod install;
mod lang;
mod prompt;
mod util;

use prompt::term::TermPrompt;
use prompt::InstallPrompt;
use prompt::InstallStep;
use std::env;
use std::process::exit;

fn main() {
	// Getting prompt type
	let prompt_type = env::args().nth(1).unwrap_or("term".to_owned());
	// Creating prompt
	let mut prompt = match prompt_type.as_str() {
		"term" => TermPrompt::new(),
		// TODO Add support for GUI
		_ => {
			eprintln!("Invalid prompt type: {}", prompt_type);
			exit(1);
		}
	};

	while let Some(curr_step) = prompt.get_current_step() {
		prompt.next_step();

		if matches!(curr_step, InstallStep::Install) {
			let infos = prompt.get_infos();
			infos.perform_install(&mut prompt).unwrap(); // TODO Handle error
		}
	}
}
