//! This module implements utility functions.

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
