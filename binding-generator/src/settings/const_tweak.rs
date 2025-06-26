use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::constant::ValueKind;

/// (rust_name, value_kind)
pub static CONST_TYPE_OVERRIDE: Lazy<HashMap<&str, ValueKind>> =
	Lazy::new(|| HashMap::from([("Mat_AUTO_STEP", ValueKind::Usize)]));
