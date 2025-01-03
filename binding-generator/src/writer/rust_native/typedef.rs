use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;
use crate::debug::NameDebug;
use crate::type_ref::NameStyle;
use crate::writer::rust_native::type_ref::Lifetime;
use crate::{CompiledInterpolation, IteratorExt, StrExt, StringExt, Typedef};

impl RustElement for Typedef<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		match self {
			Typedef::Clang { entity, .. } => DefaultRustNativeElement::rust_module(*entity),
			Typedef::Desc(desc) => desc.rust_module.as_ref().into(),
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		match self {
			Typedef::Clang { entity, .. } => DefaultRustNativeElement::rust_name(self, *entity, style).into(),
			Typedef::Desc(_) => {
				let decl_name = self.rust_leafname(style.turbo_fish_style());
				match style {
					NameStyle::Declaration => decl_name,
					NameStyle::Reference(_) => {
						let mut out = self.rust_module_reference().into_owned();
						out.extend_sep("::", &decl_name);
						out.into()
					}
				}
			}
		}
	}

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		match self {
			Typedef::Clang { entity, .. } => DefaultRustNativeElement::rendered_doc_comment(*entity, comment_marker, opencv_version),
			Typedef::Desc(_) => "".to_string(),
		}
	}
}

impl RustNativeGeneratedElement for Typedef<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/typedef/tpl.rs").compile_interpolation());
		let underlying_type = self.underlying_type_ref();
		let lifetimes = Lifetime::automatic()
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
			("doc_comment", Cow::Owned(self.rendered_doc_comment("///", opencv_version))),
			("debug", self.get_debug().into()),
			("rust_local", self.rust_name(NameStyle::decl())),
			("generic_args", generic_args.into()),
			(
				"definition",
				underlying_type.rust_name_ext(NameStyle::ref_(), Lifetime::automatic()),
			),
		]))
	}
}
