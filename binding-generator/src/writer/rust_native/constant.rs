use std::borrow::Cow;

use clang::EntityKind;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Const,
	constant::ValueKind,
	Element,
	EntityElement,
	get_debug,
	settings,
	StrExt,
	type_ref::FishStyle,
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for Const<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname(FishStyle::No))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static STRING_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/const/string.tpl.rs").compile_interpolation()
		);

		static INT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/const/int.tpl.rs").compile_interpolation()
		);

		static UINT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/const/uint.tpl.rs").compile_interpolation()
		);

		static USIZE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/const/usize.tpl.rs").compile_interpolation()
		);

		static FLOAT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/const/float.tpl.rs").compile_interpolation()
		);

		let parent_is_class = self.entity().get_lexical_parent()
			.map_or(false, |p| matches!(p.get_kind(), EntityKind::ClassDecl | EntityKind::StructDecl));
		let name = if parent_is_class {
			self.rust_leafname(FishStyle::No)
		} else {
			self.rust_localname(FishStyle::No)
		};

		if let Some(value) = self.value() {
			let tpl = if settings::CONST_TYPE_USIZE.contains(name.as_ref()) {
				&USIZE_TPL
			} else {
				match value.kind {
					ValueKind::Integer => &INT_TPL,
					ValueKind::UnsignedInteger => &UINT_TPL,
					ValueKind::Float => &FLOAT_TPL,
					ValueKind::String => &STRING_TPL,
				}
			};
			tpl.interpolate(&hashmap! {
				"doc_comment" => Cow::Owned(self.rendered_doc_comment(opencv_version)),
				"debug" => get_debug(self).into(),
				"name" => name,
				"value" => value.to_string().into(),
			})
		} else {
			String::new()
		}
	}
}
