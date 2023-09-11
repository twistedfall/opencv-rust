use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::debug::NameDebug;
use crate::type_ref::{FishStyle, NameStyle};
use crate::{CompiledInterpolation, CppNameStyle, Element, EntityElement, Enum, StrExt};

use super::element::{DefaultRustNativeElement, RustElement};
use super::RustNativeGeneratedElement;

impl RustElement for Enum<'_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self.entity())
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, self.entity(), style).into()
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		self.cpp_name(CppNameStyle::Declaration)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self.entity(), prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Enum<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static ENUM_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/enum/enum.tpl.rs").compile_interpolation());

		static CONST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/enum/const.tpl.rs").compile_interpolation());

		static CONST_IGNORED_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/enum/const_ignored.tpl.rs").compile_interpolation());

		let consts = self.consts();
		let mut generated_values = HashMap::with_capacity(consts.len());
		let consts = consts
			.into_iter()
			.map(|c| {
				let name = c.rust_leafname(FishStyle::No).into_owned();
				let value = c.value().expect("Can't get value of enum variant").to_string();
				let duplicate_name = generated_values.get(&value).cloned();
				let tpl = if duplicate_name.is_some() {
					&CONST_IGNORED_TPL
				} else {
					&CONST_TPL
				};
				let doc_comment = if duplicate_name.is_some() {
					c.rendered_doc_comment_with_prefix("//", _opencv_version)
				} else {
					c.rendered_doc_comment(_opencv_version)
				};
				generated_values.insert(value.clone(), name.clone());
				tpl.interpolate(&HashMap::from([
					("doc_comment", doc_comment),
					("duplicate_name", duplicate_name.unwrap_or_default()),
					("name", name),
					("value", value),
				]))
			})
			.collect::<Vec<_>>();
		ENUM_TPL.interpolate(&HashMap::from([
			("doc_comment", self.rendered_doc_comment(_opencv_version).into()),
			("debug", self.get_debug().into()),
			("rust_local", self.rust_name(NameStyle::decl())),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("consts", consts.join("").into()),
		]))
	}
}
