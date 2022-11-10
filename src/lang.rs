//! TODO doc

use std::collections::HashMap;
use std::fmt;

/// Structure representing a language.
#[derive(Clone)]
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
	pub fn list() -> HashMap<String, Self> {
		// TODO
		todo!();
	}
}

impl fmt::Display for Language {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "{} - {}", self.name, self.display_name)
	}
}
