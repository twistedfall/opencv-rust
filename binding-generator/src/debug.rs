use core::fmt;
use core::fmt::Display;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use clang::Entity;
use dunce::canonicalize;

static DEBUG_CONFIG: OnceLock<Option<DebugConfig>> = OnceLock::new();

#[derive(Debug)]
pub struct DebugConfig {
	pub installation_root: PathBuf,
}

/// Enabled debug output like symbol location in the generator output
pub fn enable(debug_config: Option<DebugConfig>) {
	DEBUG_CONFIG.set(debug_config).expect("Debug config is already initialized");
}

#[inline(always)]
pub fn enabled() -> bool {
	DEBUG_CONFIG.get().is_some()
}

#[inline(always)]
pub fn config() -> Option<&'static DebugConfig> {
	DEBUG_CONFIG.get().and_then(|c| c.as_ref())
}

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
				let file = canonicalize(file).expect("Can't canonicalize path");
				let path = config()
					.and_then(|dbg_config| file.strip_prefix(&dbg_config.installation_root).ok())
					.unwrap_or(&file);
				write!(f, "{}:{line}", path.display())
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
		if enabled() {
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
		let mut name = self
			.get_display_name()
			.unwrap_or_else(|| "<unknown display name>".to_string());
		// special handling for unnamed enums inside classes that are reported like:
		// (unnamed enum at /path/to/file:line)
		if let Some(unnamed) = name.strip_prefix("(")
			&& unnamed.starts_with("unnamed ")
		{
			if let Some(parent_name) = self.get_semantic_parent().and_then(|p| p.get_display_name()) {
				// resolve parent name if possible
				name = parent_name;
			} else if let Some((unnamed, _)) = unnamed.split_once(" at ") {
				// just drop the location otherwise because we already write out the location separately
				name = unnamed.to_string();
			}
		}
		LocationName::new(
			DefinitionLocation::File(loc.file.map(|f| f.get_path()).expect("Can't get file for debug"), loc.line),
			name,
		)
	}
}
