//! This module implements utility functions.

use std::io::BufRead;
use std::io;
use std::process::Command;
use std::process::exit;

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
		},

		None => exit(0),
	}
}

/// Reboots the system.
/// If the current process doesn't have the permission to reboot the system, the function prints an
/// error, then exits the process.
pub fn reboot() -> ! {
	let _ = Command::new("reboot")
		.arg("now")
		.status();

	eprintln!("Failed to reboot the system. Exiting...");
	exit(1)
}
