use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::element::UNNAMED;

/// List of C++ argument names that are allowed to be userdata for a callback
pub static ARGUMENT_NAMES_USERDATA: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["userdata", "userData", "cookie", UNNAMED]));
