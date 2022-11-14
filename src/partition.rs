//! TODO doc

use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io;
use std::path::Path;

/// Structure storing informations about a partition.
#[derive(Default, Deserialize, Serialize)]
pub struct Partition {
	/// The start offset in sectors.
	pub start: u64,
	/// The size of the partition in sectors.
	pub size: u64,

	/// The partition type.
	pub part_type: String,

	/// The partition's UUID.
	pub uuid: String,

	/// Tells whether the partition is bootable.
	pub bootable: bool,
}

impl Partition {
	/// Loads a partitions list from a given sfdisk script.
	///
	/// Arguments:
	/// - `path` is the path of the script.
	pub fn load(path: &Path) -> io::Result<Vec<Self>> {
        let file = File::open(path)?;
		let reader = BufReader::new(file);
		let iter = reader.lines();

		// Tells whether the loop is currently skipping the header
		let mut skipping = true;

		let mut parts = vec![];
		for line in iter {
			// Skipping header
			if skipping {
				if line?.trim().is_empty() {
					skipping = false;
				}

				continue;
			}

			let line = line?;
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

						part.uuid = val.to_string();
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

		Ok(parts)
	}

	/// Stores a partitions list into a sfdisk script.
	///
	/// Arguments:
	/// - `dev` is the path to the device file of the disk.
	/// - `parts` is the list of partitions.
	/// - `path` is the path of the script.
	pub fn store(dev: &str, parts: &[Self], path: &Path) -> io::Result<()> {
        let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)?;

		// Writing header
		// TODO label
		// TODO label-id
		file.write(format!("device: {}\n", dev).as_bytes())?;
		file.write(b"unit: sectors\n")?;
		file.write(b"\n")?;

		// Writing partitions
		for (i, p) in parts.iter().enumerate() {
			let s = if p.bootable {
				format!(
					"{}{} : start= {}, size= {}, type={}, bootable\n",
					dev,
					i,
					p.start,
					p.size,
					p.part_type
				)
			} else {
				format!(
					"{}{} : start= {}, size= {}, type={}\n",
					dev,
					i,
					p.start,
					p.size,
					p.part_type
				)
			};

			file.write(s.as_bytes())?;
		}

		Ok(())
	}
}

// TODO unit tests
