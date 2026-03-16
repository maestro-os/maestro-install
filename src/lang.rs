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

//! TODO doc

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

/// The path to the languages directory.
const LANGS_PATH: &str = "lang/"; // TODO Use an absolute path

/// Structure representing a language.
#[derive(Clone, Deserialize, Serialize)]
pub struct Language {
	/// The name of the language used to select it.
	name: String,
	/// The display name of the language.
	display_name: String,

	/// The locale corresponding to the language.
	locale: String,
}

impl Language {
	/// Returns the list of available languages.
	///
	/// The function returns a hashmap where the key is the name of the language and the value is
	/// the language itself.
	pub fn list() -> io::Result<HashMap<String, Self>> {
		let mut langs = HashMap::new();

		for e in fs::read_dir(LANGS_PATH)? {
			let e = e?;
			if !e.file_type()?.is_file() {
				continue;
			}

			let path = Path::new(LANGS_PATH).join(e.file_name());
			let file = File::open(path)?;
			let reader = BufReader::new(file);

			let lang: Self = serde_json::from_reader(reader)?;
			langs.insert(lang.name.clone(), lang);
		}

		Ok(langs)
	}

	/// Returns the locale associated with the language.
	pub fn get_locale(&self) -> &str {
		&self.locale
	}
}

impl fmt::Display for Language {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "{} - {}", self.name, self.display_name)
	}
}
