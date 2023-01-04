use std::borrow::Cow;

use crate::field::{Field, FieldTypeHint};
use crate::type_ref::FishStyle;
use crate::NameStyle;

use super::element::{DefaultRustNativeElement, RustElement};

impl RustElement for Field<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, style)
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		if matches!(self.type_hint(), FieldTypeHint::FieldSetter) {
			"val".into()
		} else {
			DefaultRustNativeElement::rust_leafname(self)
		}
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}
}
