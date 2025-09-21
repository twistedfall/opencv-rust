use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::collections::HashMap;
use std::sync::LazyLock;

use clang::EntityKind;

use super::element::{DefaultRustNativeElement, RustElement};
use super::RustNativeGeneratedElement;
use crate::constant::{Value, ValueKind};
use crate::debug::NameDebug;
use crate::type_ref::{FishStyle, NameStyle};
use crate::{settings, CompiledInterpolation, Const, StrExt, SupportedModule};

impl RustElement for Const<'_> {
	fn rust_module(&self) -> SupportedModule {
		match self {
			&Self::Clang { entity } => DefaultRustNativeElement::rust_module(entity),
			Self::Desc(desc) => desc.rust_module,
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str> {
		match self {
			&Self::Clang { entity } => {
				let mut out = DefaultRustNativeElement::rust_name(self, entity, style);
				if let Some(without_suffix) = out.strip_suffix("_OCVRS_OVERRIDE") {
					let new_len = without_suffix.len();
					out.truncate(new_len);
				}
				out.into()
			}
			Self::Desc(_) => match style {
				NameStyle::Declaration => self.rust_leafname(FishStyle::No),
				NameStyle::Reference(fish_style) => format!(
					"{}::{}",
					DefaultRustNativeElement::rust_module_reference(self),
					self.rust_leafname(fish_style)
				)
				.into(),
			},
		}
	}
}

impl RustNativeGeneratedElement for Const<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module().opencv_name(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static RUST_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/const/rust.tpl.rs").compile_interpolation());

		let parent_is_class = match self {
			&Self::Clang { entity } => entity
				.get_lexical_parent()
				.is_some_and(|p| matches!(p.get_kind(), EntityKind::ClassDecl | EntityKind::StructDecl)),
			Self::Desc(_) => false,
		};
		let name = if parent_is_class {
			self.rust_leafname(FishStyle::No)
		} else {
			self.rust_name(NameStyle::decl())
		};

		if let Some(value) = self.value() {
			let typ = settings::CONST_TYPE_OVERRIDE
				.get(name.as_ref())
				.unwrap_or(&value.kind)
				.rust_type();
			RUST_TPL.interpolate(&HashMap::from([
				("doc_comment", Owned(self.rust_doc_comment("///", opencv_version))),
				("debug", self.get_debug().into()),
				("name", name),
				("type", typ.into()),
				("value", value.rust_render()),
			]))
		} else {
			"".to_string()
		}
	}
}

pub trait ValueKindExt {
	fn rust_type(self) -> &'static str;
}

impl ValueKindExt for ValueKind {
	fn rust_type(self) -> &'static str {
		match self {
			ValueKind::Integer => "i32",
			ValueKind::UnsignedInteger => "u32",
			ValueKind::Usize => "usize",
			ValueKind::Float => "f32",
			ValueKind::Double => "f64",
			ValueKind::String => "&str",
		}
	}
}

pub trait ValueExt {
	fn rust_render(&self) -> Cow<'_, str>;
}

impl ValueExt for Value {
	fn rust_render(&self) -> Cow<'_, str> {
		match self.kind {
			ValueKind::Float | ValueKind::Double if !self.value.contains('.') => Owned(format!("{}.", self.value)),
			ValueKind::Integer => {
				if let Some(no_prefix) = self.value.strip_prefix("0x") {
					// todo: use let chain when MSRV is 1.88
					if i32::from_str_radix(no_prefix, 16).is_err() {
						Owned(format!("{}u32 as i32", self.value))
					} else {
						Borrowed(&self.value)
					}
				} else {
					Borrowed(&self.value)
				}
			}
			ValueKind::UnsignedInteger | ValueKind::Usize | ValueKind::Float | ValueKind::Double | ValueKind::String => {
				Borrowed(&self.value)
			}
		}
	}
}
