use std::borrow::Cow;

use crate::type_ref::{Constness, Nullability};
use crate::NameStyle;

pub trait NullabilityExt {
	fn rust_wrap_nullable_decl(self, typ: Cow<str>, name_style: NameStyle) -> Cow<str>;
	fn rust_wrap_nullable_func_call<'call>(self, name: &str, call: Cow<'call, str>, constness: Constness) -> Cow<'call, str>;
}

impl NullabilityExt for Nullability {
	fn rust_wrap_nullable_decl(self, typ: Cow<str>, name_style: NameStyle) -> Cow<str> {
		match self {
			Nullability::Nullable => format!("Option{fish}<{typ}>", fish = name_style.rust_turbo_fish_qual()).into(),
			Nullability::NotNullable => typ,
		}
	}

	fn rust_wrap_nullable_func_call<'call>(self, name: &str, call: Cow<'call, str>, constness: Constness) -> Cow<'call, str> {
		match self {
			// unwrap_or doesn't work here because reference doesn't coerce to pointer in this case
			Nullability::Nullable => format!(
				"{name}.map_or({null_ptr}, |{name}| {call})",
				null_ptr = constness.rust_null_ptr()
			)
			.into(),
			Nullability::NotNullable => call,
		}
	}
}
