//! This module implements installation prompt from terminal.

use crate::install::InstallInfo;
use crate::install::InstallProgress;
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
				print!("Type `?` to get the list of available languages.");
				print!("Type the system's language: ");
				io::stdout().flush();
				let lang = lines_iter.next();
				self.infos.lang = lang;

				// TODO Contient/Country
				// TODO Timezone
			},

			InstallStep::SystemInfo => {
				// TODO Add a characters limit?
				print!("Type system hostname: ");
				io::stdout().flush();
				let hostname = lines_iter.next();
				self.infos.hostname = hostname;
			},

			InstallStep::CreateAdmin => {
				// TODO Add a characters limit?
				print!("Type admin username: ");
				io::stdout().flush();
				let username = lines_iter.next();
				self.infos.admin_user = username;

				loop {
					print!("Type admin/root password: ");
					io::stdout().flush();
					// TODO Disable prompting
					let pass = lines_iter.next();
					// TODO Re-enable prompting

					print!("Confirm admin/root password: ");
					io::stdout().flush();
					// TODO Disable prompting
					let pass_confirm = lines_iter.next();
					// TODO Re-enable prompting

					if pass == pass_confirm {
						self.infos.admin_pass = pass;
						break;
					}

					eprintln!("Passwords don't match!");
				}
			},

			InstallStep::Partitions => {
				// TODO Detect partitions/systems that are already present

				// TODO List disks
				// TODO Ask the disk to be selected for the system

				// TODO Ask whether:
				// - The old system(s) should be wiped and partitions creation is automatic
				// - The old system(s) should NOT be wiped and partitions creation is automatic
				// - Partitions should be created manualy

				// TODO If manual, list disks/partitions that are present and asks for
				// modifications
			},

			InstallStep::Install => {
				loop {
					print!("Confirm installation? (y/n) ");
					io::stdout().flush();
					let confirm = lines_iter.next();
					match confirm {
						"y" => break,

						"n" => {
							// TODO Abort
							todo!();
						}

						_ => {},
					}
				}
			},

			InstallStep::Finished => {
				println!("Installation is now finished!");
				println!("To start maestro, unplug your installation medium, then press ENTER");

				let _ = lines_iter.next();

				util::reboot();
			},
		}
		println!();

		self.curr_step = curr_step.get_next();
	}

	fn get_infos(&self) -> &InstallInfo {
		&self.infos
	}

	fn update_progress(progress: &InstallProgress) {
		// TODO
		todo!();
	}
}
