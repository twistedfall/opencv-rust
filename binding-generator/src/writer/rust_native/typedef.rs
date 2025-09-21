use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;
use crate::debug::NameDebug;
use crate::type_ref::NameStyle;
use crate::writer::rust_native::type_ref::Lifetime;
use crate::{CompiledInterpolation, IteratorExt, StrExt, StringExt, SupportedModule, Typedef};

impl RustElement for Typedef<'_, '_> {
	fn rust_module(&self) -> SupportedModule {
		match self {
			Typedef::Clang { entity, .. } => DefaultRustNativeElement::rust_module(*entity),
			Typedef::Desc(desc) => desc.rust_module,
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str> {
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
}

impl RustNativeGeneratedElement for Typedef<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module().opencv_name(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static TPL: LazyLock<CompiledInterpolation> = LazyLock::new(|| include_str!("tpl/typedef/tpl.rs").compile_interpolation());
		let underlying_type = self.underlying_type_ref();
		let lifetimes = Lifetime::automatic()
			.into_iter()
			.take(underlying_type.rust_lifetime_count())
			.map(|l| l.to_string())
			.join(", ");
		let (generic_args, lifetime) = if lifetimes.is_empty() {
			("".to_string(), Lifetime::Elided)
		} else {
			(format!("<{lifetimes}>"), Lifetime::automatic())
		};

		TPL.interpolate(&HashMap::from([
			("doc_comment", self.rust_doc_comment("///", opencv_version).as_str()),
			("debug", &self.get_debug()),
			("rust_local", &self.rust_name(NameStyle::decl())),
			("generic_args", &generic_args),
			("definition", &underlying_type.rust_name_ext(NameStyle::ref_(), lifetime)),
		]))
	}
}
