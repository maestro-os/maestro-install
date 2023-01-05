//! This module implements installation prompt from terminal.

use crate::install::InstallInfo;
use crate::install::InstallProgress;
use crate::lang::Language;
use crate::util;
use std::process::exit;
use super::InstallPrompt;
use super::InstallStep;

/// Prompts text from the user on the terminal.
///
/// Arguments:
/// - `prompt_text` is the text showed to the user while prompting.
/// - `hidden` tells whether the input must be hidden.
/// - `validator` is a function called to check whether the given input is valid. If not, the
/// function can return an error message which is printed, then the function prompts for input
/// again.
/// If no error message is provided, no message is printed and the function prompts for input again
/// directly.
fn prompt<V: Fn(&str) -> Result<(), Option<String>>> (
	prompt_text: &str,
	hidden: bool,
	validator: V,
) -> String {
	loop {
		let Some(input) = utils::prompt::prompt(Some(prompt_text), hidden) else {
			// TODO
			todo!();
		};

		match validator(&input) {
			Ok(()) => return input,
			Err(Some(e)) => eprintln!("{}", e),

			_ => {}
		}
	}
}

/// Validator for the `prompt` function which validates non-empty inputs.
fn non_empty_validator(input: &str) -> Result<(), Option<String>> {
	if !input.is_empty() {
		Ok(())
	} else {
		Err(None)
	}
}

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
				println!(
					"##     ##    ###    ########  ######  ######## ########   #######  
###   ###   ## ##   ##       ##    ##    ##    ##     ## ##     ## 
#### ####  ##   ##  ##       ##          ##    ##     ## ##     ## 
## ### ## ##     ## ######    ######     ##    ########  ##     ## 
##     ## ######### ##             ##    ##    ##   ##   ##     ## 
##     ## ##     ## ##       ##    ##    ##    ##    ##  ##     ## 
##     ## ##     ## ########  ######     ##    ##     ##  #######  "
				);

				println!();
				println!("Welcome to the maestro installer!");
				println!();
				println!("To begin the installation, press ENTER.");

				util::read_line();
			}

			InstallStep::Localization => {
				let available_langs = Language::list().unwrap(); // TODO Handle error

				while self.infos.lang.is_none() {
					println!("Type `?` to get the list of available languages.");
					let lang = prompt("Type the system's language: ", false, |_| Ok(()));

					match lang.as_str() {
						"?" => {
							println!("Available languages:");
							for (_, l) in available_langs.iter() {
								println!("- {}", l);
							}

							println!();
						}

						"" => {}

						_ => {
							if let Some(lang) = available_langs.get(&lang) {
								self.infos.lang = Some(lang.clone());
							} else {
								eprintln!("\nInvalid language `{}`!\n", lang);
							}
						}
					}
				}

				// TODO Contient/Country
				// TODO Timezone
			}

			InstallStep::SystemInfo => {
				self.infos.hostname = prompt("Type system hostname: ", false, non_empty_validator);
			}

			InstallStep::CreateAdmin => {
				self.infos.admin_user = prompt("Type admin username: ", false, non_empty_validator);

				loop {
					println!();
					let pass = prompt("Type admin/root password: ", true, non_empty_validator);
					let pass_confirm = prompt("Confirm admin/root password: ", true, |_| Ok(()));

					if pass != pass_confirm {
						eprintln!("Passwords don't match!");
						continue;
					}

					self.infos.admin_pass = pass;
					break;
				}
			}

			InstallStep::Partitions => {
				let disks = Disk::list().unwrap(); // TODO Handle error
				// TODO Filter out disks that don't have enough space
				if disks.is_empty() {
					eprintln!("No disks are available for installation. Exiting...");
					exit(1);
				}

				self.infos.selected_disk = loop {
					println!("Available disks and partitions:");
					for d in disks.iter() {
						println!(
							"- {} (sectors: {}, size: {})",
							d.get_dev_path().display(),
							d.get_size(),
							ByteSize::from_sectors_count(d.get_size()),
						);

						for p in &d.partitions {
							println!("\t- {}", p);
						}
					}

					// If only one disk is available, de facto select it
					if disks.len() == 1 {
						break disks
							.iter()
							.next()
							.unwrap()
							.get_dev_path()
							.to_str()
							.unwrap()
							.to_owned();
					}

					println!();
					let selected_disk = prompt(
						"Select the disk to install the system on: ",
						false,
						|input| {
							let exists = disks
								.iter()
								.filter(|d| d.get_dev_path().to_str() == Some(input))
								.next()
								.is_some();

							if input.is_empty() {
								Ok(())
							} else if !exists {
								Err(Some(format!("Disk `{}` doesn't exist", input)))
							} else {
								Ok(())
							}
						},
					);

					if !selected_disk.is_empty() {
						break selected_disk;
					}
				};

				println!();
				println!("Installing system on disk `{}`", self.infos.selected_disk);
				println!("Partitioning options:");
				println!("1 - Wipe disk and install system automaticaly (warning: this operation will destroy all data on the disk)");
				// TODO:
				//println!("2 - Manual partitioning (advanced)");
				//println!("3 - Use free space left on disk");
				// TODO Disable option 3 if not enough free space is left on disk
				println!();
				println!("NOTE: Other options are not yet available");
				println!();

				let option = prompt("Select an option: ", false, |input| match input {
					"1" => Ok(()),
					_ => Err(Some(format!("Invalid option `{}`", input))),
				});

				match option.as_str() {
					"1" => {
						let disk = disks
							.iter()
							.filter(|d| {
								d.get_dev_path().to_str() == Some(&self.infos.selected_disk)
							})
							.next()
							.unwrap();

						let boot_part = Partition {
							start: 2048,
							size: 262144,

							part_type: "C12A7328-F81F-11D2-BA4B-00A0C93EC93B".to_owned(),

							uuid: None,

							bootable: false,
						};
						let root_start = boot_part.start + boot_part.size;
						let root_part = Partition {
							start: root_start,
							size: disk.get_size() - root_start,

							// TODO (dependent on the filesystem type)
							part_type: "00000000-0000-0000-0000-000000000000".to_owned(),

							uuid: None,

							bootable: false,
						};

						self.infos.partitions = vec![
							boot_part, // TODO swap
							root_part,
						];
					}

					"2" => {
						// TODO Ask for modifications on existing partitions
						todo!();
					}

					"3" => {
						// TODO Build partitions table
						todo!();
					}

					_ => unreachable!(),
				}

				println!();
				println!("The following partitions will be created:");
				for p in self.infos.partitions.iter() {
					println!("- {}", p);
				}
			}

			InstallStep::Install => {
				// TODO Add option to export selected options to file

				loop {
					let confirm =
						prompt("Confirm installation? (y/n) ", false, non_empty_validator);
					match confirm.as_str() {
						"y" => break,

						"n" => {
							eprintln!("Installation cancelled.");
							exit(1);
						}

						_ => {}
					}
				}
			}

			InstallStep::Finished => {
				println!("Installation is now finished!");
				println!("To start maestro, unplug your installation medium, then press ENTER");

				util::read_line();
				util::reboot();
			}
		}
		println!();

		self.curr_step = curr_step.get_next();
	}

	fn get_infos(&self) -> InstallInfo {
		self.infos.clone()
	}

	fn update_progress(&mut self, progress: &InstallProgress) {
		// TODO
		todo!();
	}
}
