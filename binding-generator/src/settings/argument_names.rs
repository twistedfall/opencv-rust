use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::element::UNNAMED;

/// List of C++ argument names that are allowed to be userdata for a callback
pub static ARGUMENT_NAMES_USERDATA: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["userdata", "userData", "cookie", UNNAMED]));

/// List of C++ argument names that are forbidden to be slice arguments
pub static ARGUMENT_NAMES_NOT_SLICE: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["rmsd"]));

/// List of C++ argument names that can hint on multiple connected slice arguments in a function
pub static ARGUMENT_NAMES_MULTIPLE_SLICE: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["a", "b", "src", "dst", "lut", "globalsize", "localsize"]));
