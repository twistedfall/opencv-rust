use std::collections::HashMap;
use std::sync::LazyLock;

use crate::constant::ValueKind;

/// (rust_name, value_kind)
pub static CONST_TYPE_OVERRIDE: LazyLock<HashMap<&str, ValueKind>> =
	LazyLock::new(|| HashMap::from([("Mat_AUTO_STEP", ValueKind::Usize)]));
