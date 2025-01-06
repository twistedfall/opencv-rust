use std::borrow::Cow;

use crate::{NameStyle, StrExt};

pub trait RustStringExt {
	fn rust_name_from_fullname(&self, style: NameStyle) -> Cow<str>;
}

impl RustStringExt for str {
	fn rust_name_from_fullname(&self, style: NameStyle) -> Cow<str> {
		match style {
			NameStyle::Declaration => self.localname().into(),
			NameStyle::Reference(fish) => fish.apply(self),
		}
	}
}
