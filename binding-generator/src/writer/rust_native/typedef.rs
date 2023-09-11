use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::debug::NameDebug;
use crate::type_ref::{FishStyle, NameStyle, TypeRefKind};
use crate::{CompiledInterpolation, CppNameStyle, DefaultElement, EntityElement, IteratorExt, StrExt, Typedef};

use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::{Lifetime, TypeRefExt};
use super::RustNativeGeneratedElement;

impl RustElement for Typedef<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self.entity())
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, self.entity(), style).into()
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		match self.underlying_type_ref().source().kind().as_ref() {
			TypeRefKind::Class(..)
			| TypeRefKind::Function(..)
			| TypeRefKind::StdVector(..)
			| TypeRefKind::SmartPtr(..)
			| TypeRefKind::StdTuple(..) => DefaultElement::cpp_name(self, self.entity(), CppNameStyle::Declaration),
			_ => DefaultRustNativeElement::rust_leafname(self),
		}
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self.entity(), prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Typedef<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/typedef/tpl.rs").compile_interpolation());
		let underlying_type = self.underlying_type_ref();
		let lifetimes = Lifetime::explicit()
			.into_iter()
			.take(underlying_type.rust_lifetime_count())
			.map(|l| l.to_string())
			.join(", ");
		let generic_args = if lifetimes.is_empty() {
			"".to_string()
		} else {
			format!("<{lifetimes}>")
		};

		TPL.interpolate(&HashMap::from([
			("doc_comment", Cow::Owned(self.rendered_doc_comment(_opencv_version))),
			("debug", self.get_debug().into()),
			("rust_local", self.rust_name(NameStyle::decl())),
			("generic_args", generic_args.into()),
			(
				"definition",
				underlying_type.rust_name_ext(NameStyle::ref_(), Lifetime::explicit()),
			),
		]))
	}
}
