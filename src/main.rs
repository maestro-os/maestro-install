/*
 * Copyright 2026 Luc Lenôtre
 *
 * This file is part of Maestro.
 *
 * Maestro is free software: you can redistribute it and/or modify it under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * Maestro is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * Maestro. If not, see <https://www.gnu.org/licenses/>.
 */

//! Installation utility for the Maestro operating system.

mod install;
mod lang;
mod prompt;
mod util;

use prompt::{
	term::{TermPrompt, CODE_RED, CODE_RESET},
	InstallPrompt, InstallStep,
};
use std::{env, process::exit};

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
