//! This module implements installation prompt from terminal.

use crate::install::InstallInfo;
use crate::install::InstallProgress;
use crate::lang::Language;
use crate::partition::Disk;
use crate::util;
use std::io::Write;
use std::io;
use std::process::exit;
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

				util::read_line();
			},

			InstallStep::Localization => {
				let available_langs = Language::list().unwrap(); // TODO Handle error

				while self.infos.lang.is_none() {
					println!("Type `?` to get the list of available languages.");
					print!("Type the system's language: ");
					let _ = io::stdout().flush();
					let lang = util::read_line();

					match lang.as_str() {
						"?" => {
							println!("Available languages:");
							for (_, l) in available_langs.iter() {
								println!("- {}", l);
							}

							println!();
						},

						"" => {},

						_ => {
							if let Some(lang) = available_langs.get(&lang) {
								self.infos.lang = Some(lang.clone());
							} else {
								eprintln!("\nInvalid language `{}`!\n", lang);
							}
						},
					}
				}

				// TODO Contient/Country
				// TODO Timezone
			},

			InstallStep::SystemInfo => {
				while self.infos.hostname.is_empty() {
					print!("Type system hostname: ");
					let _ = io::stdout().flush();

					let hostname = util::read_line();
					self.infos.hostname = hostname;
				}
			},

			InstallStep::CreateAdmin => {
				while self.infos.admin_user.is_empty() {
					print!("Type admin username: ");
					let _ = io::stdout().flush();

					let username = util::read_line();
					self.infos.admin_user = username;
				}

				while self.infos.admin_pass.is_empty() {
					print!("Type admin/root password: ");
					let _ = io::stdout().flush();

					// TODO Disable prompting
					let pass = util::read_line();
					// TODO Re-enable prompting

					if pass.is_empty() {
						continue;
					}

					print!("Confirm admin/root password: ");
					let _ = io::stdout().flush();

					// TODO Disable prompting
					let pass_confirm = util::read_line();
					// TODO Re-enable prompting

					if pass != pass_confirm {
						eprintln!("Passwords don't match!");
						continue;
					}

					self.infos.admin_pass = pass;
				}
			},

			InstallStep::Partitions => {
				let disks = Disk::list().unwrap(); // TODO Handle error

				println!("Available disks and partitions:");
				for d in disks {
					println!("- {}", d.get_dev_path().display());

					for p in &d.partitions {
						println!("\t- {}", p);
					}
				}

				// TODO Ask whether:
				// - The old system(s) should be wiped and partitions creation is automatic
				// - If free space is left on a disk: The old system(s) should NOT be wiped and
				// partitions creation is automatic
				// - Partitions should be created manualy

				// TODO Ask the disk to be selected for the system

				// TODO If manual, list disks/partitions that are present and asks for
				// modifications
			},

			InstallStep::Install => {
				// TODO Add option to export options to file

				loop {
					print!("Confirm installation? (y/n) ");
					let _ = io::stdout().flush();
					let confirm = util::read_line();
					match confirm.as_str() {
						"y" => break,

						"n" => {
							eprintln!("Installation cancelled.");
							exit(1);
						}

						_ => {},
					}
				}
			},

			InstallStep::Finished => {
				println!("Installation is now finished!");
				println!("To start maestro, unplug your installation medium, then press ENTER");

				util::read_line();
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
