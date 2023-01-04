use std::borrow::Cow;

use clang::EntityKind;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::constant::ValueKind;
use crate::type_ref::{FishStyle, NameStyle};
use crate::{get_debug, settings, CompiledInterpolation, Const, CppNameStyle, Element, EntityElement, StrExt};

use super::element::{DefaultRustNativeElement, RustElement};
use super::RustNativeGeneratedElement;

impl RustElement for Const<'_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		let mut out = DefaultRustNativeElement::rust_name(self, style);
		const SUFFIX: &str = "_OCVRS_OVERRIDE";
		if out.ends_with(SUFFIX) {
			let suffix_start = out.len() - SUFFIX.len();
			out.to_mut().drain(suffix_start..);
		}
		out
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		self.cpp_name(CppNameStyle::Declaration)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Const<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static STRING_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/const/string.tpl.rs").compile_interpolation());

		static INT_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/const/int.tpl.rs").compile_interpolation());

		static UINT_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/const/uint.tpl.rs").compile_interpolation());

		static USIZE_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/const/usize.tpl.rs").compile_interpolation());

		static FLOAT_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/const/float.tpl.rs").compile_interpolation());

		let parent_is_class = self.entity().get_lexical_parent().map_or(false, |p| {
			matches!(p.get_kind(), EntityKind::ClassDecl | EntityKind::StructDecl)
		});
		let name = if parent_is_class {
			self.rust_leafname(FishStyle::No)
		} else {
			self.rust_name(NameStyle::decl())
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
