use std::borrow::Cow;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use clang::Entity;
use dunce::canonicalize;
use once_cell::sync::Lazy;

static EMIT_DEBUG: Lazy<bool> = Lazy::new(|| env::var("OPENCV_BINDING_GENERATOR_EMIT_DEBUG").map_or(false, |v| v == "1"));

#[derive(Clone, Debug)]
pub struct LocationName<'me> {
	pub location: DefinitionLocation,
	pub name: Cow<'me, str>,
}

impl<'me> LocationName<'me> {
	pub fn new(location: DefinitionLocation, name: impl Into<Cow<'me, str>>) -> Self {
		Self {
			location,
			name: name.into(),
		}
	}
}

#[derive(Clone, Debug)]
pub enum DefinitionLocation {
	Generated,
	File(PathBuf, u32),
}

impl DefinitionLocation {
	pub fn as_file(&self) -> Option<(&Path, u32)> {
		match self {
			DefinitionLocation::Generated => None,
			DefinitionLocation::File(path, line) => Some((path, *line)),
		}
	}
}

impl Display for DefinitionLocation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Generated => f.write_str("generated"),
			Self::File(file, line) => {
				write!(f, "{}:{line}", canonicalize(file).expect("Can't canonicalize path").display())
			}
		}
	}
}

pub trait NameDebug<'me> {
	fn file_line_name(self) -> LocationName<'me>;

	fn get_debug(self) -> String
	where
		Self: Sized,
	{
		if *EMIT_DEBUG {
			let LocationName { location, name } = self.file_line_name();
			format!("// {name} {location}")
		} else {
			"".to_string()
		}
	}
}

impl NameDebug<'_> for &Entity<'_> {
	fn file_line_name(self) -> LocationName<'static> {
		let loc = self.get_location().expect("Can't get entity location").get_file_location();
		LocationName::new(
			DefinitionLocation::File(loc.file.map(|f| f.get_path()).expect("Can't get file for debug"), loc.line),
			self
				.get_display_name()
				.unwrap_or_else(|| "<unknown display name>".to_string()),
		)
	}
}
