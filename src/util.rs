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

//! This module implements utility functions.

use std::{
	io,
	io::BufRead,
	process::{exit, Command},
};

/// Reads a line from the standard input and returns it.
///
/// If reading fails, the function exits the program.
pub fn read_line() -> String {
	let stdin = io::stdin();

	match stdin.lock().lines().next() {
		Some(Ok(line)) => line,

		Some(Err(_)) => {
			eprintln!("Failed to read line from input");
			exit(1);
		}

		None => exit(0),
	}
}

/// Reboots the system.
/// If the current process doesn't have the permission to reboot the system, the function prints an
/// error, then exits the process.
pub fn reboot() -> ! {
	let _ = Command::new("reboot").status();

	eprintln!("Failed to reboot the system. Exiting...");
	exit(1)
}
