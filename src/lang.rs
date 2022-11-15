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
}

impl fmt::Display for Language {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "{} - {}", self.name, self.display_name)
	}
}
