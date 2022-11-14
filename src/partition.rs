//! TODO doc

use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::str;

/// Structure storing informations about a partition.
#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Partition {
	/// The start offset in sectors.
	pub start: u64,
	/// The size of the partition in sectors.
	pub size: u64,

	/// The partition type.
	pub part_type: String,

	/// The partition's UUID.
	pub uuid: Option<String>,

	/// Tells whether the partition is bootable.
	pub bootable: bool,
}

impl Partition {
	/// Serializes a partitions list into a sfdisk script.
	///
	/// Arguments:
	/// - `dev` is the path to the device file of the disk.
	/// - `parts` is the list of partitions.
	///
	/// The function returns the resulting script.
	pub fn serialize(dev: &str, parts: &[Self]) -> String {
		let mut script = String::new();

		// Writing header
		// TODO label
		// TODO label-id
		script += format!("device: {}\n", dev).as_str();
		script += "unit: sectors\n";
		script += "\n";

		// Writing partitions
		for (i, p) in parts.iter().enumerate() {
			script += &format!(
				"{}{} : start={}, size={}, type={}",
				dev,
				i,
				p.start,
				p.size,
				p.part_type
			);

			if p.bootable {
				script += ", bootable"
			}

			if let Some(ref uuid) = p.uuid {
				script += &format!(", uuid={}", uuid);
			}

			script += "\n";
		}

		script
	}

	/// Deserializes a partitions list from a given sfdisk script.
	///
	/// Arguments:
	/// - `data` is script.
	///
	/// The function returns the list of partitions.
	pub fn deserialize(data: &str) -> Vec<Self> {
		// Skip header
		let mut iter = data.split('\n');
		while let Some(line) = iter.next() {
			if line.trim().is_empty() {
				break;
			}
		}

		// Parse partitions
		let mut parts = vec![];
		for line in iter {
			if line.trim().is_empty() {
				continue;
			}

			let mut split = line.split(':').skip(1);
			let Some(values) = split.next() else {
				// TODO error
				todo!();
			};

			// Filling partition structure
			let mut part = Self::default();
			for v in values.split(',') {
				let mut split = v.split('=');
				let Some(name) = split.next() else {
					// TODO error
					todo!();
				};

				let name = name.trim();
				let value = split.next().map(|s| s.trim());

				match name {
					"start" => {
						let Some(val) = value else {
							// TODO error
							todo!();
						};
						let Ok(val) = val.parse() else {
							// TODO error
							todo!();
						};

						part.start = val;
					},

					"size" => {
						let Some(val) = value else {
							// TODO error
							todo!();
						};
						let Ok(val) = val.parse() else {
							// TODO error
							todo!();
						};

						part.size = val;
					},

					"type" => {
						let Some(val) = value else {
							// TODO error
							todo!();
						};

						part.part_type = val.to_string();
					},

					"uuid" => {
						let Some(val) = value else {
							// TODO error
							todo!();
						};

						part.uuid = Some(val.to_string());
					},

					"bootable" => part.bootable = true,

					_ => {
						// TODO error
						todo!();
					},
				}
			}

			parts.push(part);
		}

		parts
	}
}

/// Structure representing a disk, containing partitions.
pub struct Disk {
	/// The path to the disk's device file.
	dev_path: PathBuf,

	/// The disk's partitions.
	partitions: Vec<Partition>,
}

impl Disk {
	/// Lists disks present on the system.
	pub fn list() -> io::Result<Vec<Self>> {
		let mut disks = vec![];

		for dev in fs::read_dir("/dev")? {
			let dev = dev?;
			if dev.file_type()?.is_dir() {
				continue;
			}

			let dev_path = dev.path();
			let output = Command::new("sfdisk")
				.args(&[OsString::from("-d").as_os_str(), dev_path.as_os_str()])
				.stdout(Stdio::piped())
				.stderr(Stdio::null())
				.output()?;
			if !output.status.success() {
				continue;
			}

			let Ok(script) = str::from_utf8(&output.stdout) else {
				continue;
			};
			let partitions = Partition::deserialize(script);

			disks.push(Self {
				dev_path,

				partitions,
			});
		}

		Ok(disks)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn partitions_serialize0() {
		let parts0 = vec![];

		let script = Partition::serialize("/dev/sda", &parts0);
		let parts1 = Partition::deserialize(&script);

		assert!(parts1.is_empty());
	}

	#[test]
	fn partitions_serialize1() {
		let parts0 = vec![Partition {
			start: 0,
			size: 1,

			part_type: "foo".to_string(),

			uuid: Some("bar".to_string()),

			bootable: false,
		}];

		let script = Partition::serialize("/dev/sda", &parts0);
		let parts1 = Partition::deserialize(&script);

		for (p0, p1) in parts0.iter().zip(&parts1) {
			assert_eq!(p0, p1);
		}
	}

	// TODO More tests (especially invalid scripts)
}
