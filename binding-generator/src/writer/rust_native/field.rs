use std::borrow::Cow;

use super::element::{DefaultRustNativeElement, RustElement};
use crate::field::Field;
use crate::type_ref::FishStyle;
use crate::writer::rust_native::element::DebugRust;
use crate::{reserved_rename, CowMapBorrowedExt, NameStyle, StrExt};

impl RustElement for Field<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		"".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		self.rust_leafname(style.turbo_fish_style())
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		match self {
			Self::Clang { .. } => DefaultRustNativeElement::rust_leafname(self).map_borrowed(|s| s.cpp_name_to_rust_fn_case()),
			Self::Desc(desc) => reserved_rename(desc.cpp_fullname.localname().cpp_name_to_rust_fn_case()),
		}
	}

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		match self {
			&Field::Clang { entity, .. } => DefaultRustNativeElement::rendered_doc_comment(entity, comment_marker, opencv_version),
			Field::Desc(_) => "".to_string(),
		}
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FieldDebug {
	rust_fullname: String,
}

impl DebugRust for &Field<'_, '_> {
	type DebugType = FieldDebug;

	fn dbg_rust(self) -> FieldDebug {
		FieldDebug {
			rust_fullname: self.rust_name(NameStyle::ref_()).into_owned(),
		}
	}
}
