//! This module implements installation prompt from terminal.

use crate::InstallInfo;
use std::io::BufRead;
use std::io::Write;
use std::io;
use super::InstallPrompt;
use super::InstallStep;

/// Structure representing the terminal prompt.
pub struct TermPrompt {
	/// The current step.
	curr_step: Option<InstallStep>,

	/// Install informations.
	infos: InstallInfo,
}

impl TermPrompt {
	/// Creates a new instance.
	pub fn new() -> Self {
		Self {
			curr_step: Some(InstallStep::Welcome),

			infos: InstallInfo::default(),
		}
	}
}

impl InstallPrompt for TermPrompt {
	fn get_current_step(&self) -> Option<InstallStep> {
		self.curr_step
	}

	fn next_step(&mut self) {
		let curr_step = match &self.curr_step {
			Some(s) => s,
			None => return,
		};

		if let Some(step_name) = curr_step.get_name() {
			println!("|> Step {}: {}", curr_step.get_number(), step_name);
			println!();
		}

		let stdin = io::stdin();
		let mut lines_iter = stdin.lock().lines();

		match curr_step {
			InstallStep::Welcome => {
				println!("##     ##    ###    ########  ######  ######## ########   #######  
###   ###   ## ##   ##       ##    ##    ##    ##     ## ##     ## 
#### ####  ##   ##  ##       ##          ##    ##     ## ##     ## 
## ### ## ##     ## ######    ######     ##    ########  ##     ## 
##     ## ######### ##             ##    ##    ##   ##   ##     ## 
##     ## ##     ## ##       ##    ##    ##    ##    ##  ##     ## 
##     ## ##     ## ########  ######     ##    ##     ##  #######  ");

				println!();
				println!("Welcome to the maestro installer!");
				println!();
				println!("To begin the installation, press ENTER.");

				let _ = lines_iter.next();
			},

			InstallStep::Localization => {
				// TODO Language
				// TODO Contient/Country
				// TODO Timezone
			},

			InstallStep::SystemInfo => {
				// TODO Add a characters limit?
				print!("Type system hostname: ");
				io::stdout().flush();
				let _hostname = lines_iter.next();

				// TODO
			},

			InstallStep::CreateAdmin => {
				// TODO Add a characters limit?
				print!("Type admin username: ");
				io::stdout().flush();
				let _username = lines_iter.next();

				print!("Type admin/root password: ");
				io::stdout().flush();
				// TODO Disable prompting
				let _pass = lines_iter.next();
				// TODO Re-enable prompting

				print!("Confirm admin/root password: ");
				io::stdout().flush();
				// TODO Disable prompting
				let _pass_confirm = lines_iter.next();
				// TODO Re-enable prompting

				// TODO Check both passwords correspond
			},

			InstallStep::Partitions => {
				// TODO Detect partitions/systems that are already present
				// TODO
			},

			InstallStep::Install => {
				// TODO Ask for confirmation

				// TODO Perform install
			},

			InstallStep::Finished => {
				// TODO
			},
		}
		println!();

		self.curr_step = curr_step.get_next();
	}

	fn get_infos(&self) -> &InstallInfo {
		&self.infos
	}
}
