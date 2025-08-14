mod gen;

use std::borrow::Cow;

use super::element::{DefaultRustNativeElement, RustElement};
use super::RustNativeGeneratedElement;
use crate::class::ClassKind;
use crate::settings::ClassTweak;
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle};
use crate::writer::rust_native::type_ref::Lifetime;
use crate::{Class, Element, Func, IteratorExt, SupportedModule};

impl RustElement for Class<'_, '_> {
	fn rust_module(&self) -> SupportedModule {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_module(entity),
			Self::Desc(desc) => desc.rust_module,
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str> {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_name(self, entity, style).into(),
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

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<'_, str> {
		if self.string_type().is_some() {
			"String".into()
		} else {
			let cpp_declname = self.cpp_name(CppNameStyle::Declaration);
			if cpp_declname == "Vec" {
				"VecN".into()
			} else {
				cpp_declname
			}
		}
	}

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rendered_doc_comment(entity, comment_marker, opencv_version),
			Self::Desc(_) => "".to_string(),
		}
	}
}

impl RustNativeGeneratedElement for Class<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module().opencv_name(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self.kind() {
			ClassKind::Simple => gen::gen_simple_class(self, opencv_version),
			ClassKind::Boxed | ClassKind::BoxedForced => gen::gen_boxed_class(self, opencv_version),
			ClassKind::System | ClassKind::Other => "".to_string(),
		}
	}

	fn gen_rust_externs(&self) -> String {
		gen::extern_functions(self).iter().map(Func::gen_rust_externs).join("")
	}

	fn gen_cpp(&self) -> String {
		gen::extern_functions(self).iter().map(Func::gen_cpp).join("")
	}
}

pub trait ClassExt {
	fn rust_trait_name(&self, style: NameStyle, constness: Constness) -> Cow<'_, str>;
	fn rust_as_raw_name(&self, constness: Constness) -> String;
	fn rust_lifetime(&self) -> Option<Lifetime>;
}

impl ClassExt for Class<'_, '_> {
	fn rust_trait_name(&self, style: NameStyle, constness: Constness) -> Cow<'_, str> {
		let mut out = self.rust_name(style);
		if self.kind().is_trait() {
			if constness.is_const() {
				out.to_mut().push_str("TraitConst");
			} else {
				out.to_mut().push_str("Trait");
			}
		}
		out
	}

	fn rust_as_raw_name(&self, constness: Constness) -> String {
		format!(
			"as_raw{const_qual}_{name}",
			const_qual = constness.rust_function_name_qual(),
			name = self.rust_name(NameStyle::Declaration)
		)
	}

	/// Rust lifetime use by the objects of this class
	fn rust_lifetime(&self) -> Option<Lifetime> {
		match self {
			Class::Clang { gen_env, .. } =>
			{
				#[allow(clippy::bind_instead_of_map)]
				gen_env
					.settings
					.class_tweak
					.get(self.cpp_name(CppNameStyle::Reference).as_ref())
					.and_then(|tweak| match tweak {
						ClassTweak::Lifetime(lt) => Some(*lt),
					})
			}
			Class::Desc(_) => None,
		}
	}
}

pub fn rust_generate_debug_fields<'f>(field_const_methods: impl IntoIterator<Item = Func<'f, 'f>>) -> String {
	field_const_methods
		.into_iter()
		.filter(|f| f.return_type_ref().kind().is_debug())
		.filter_map(|f| {
			f.kind().as_field_accessor().map(|(cls, _)| {
				format!(
					"\n\t.field(\"{name}\", &{trait_name}::{name}(self))",
					trait_name = cls.rust_trait_name(NameStyle::ref_fish(), Constness::Const),
					name = f.rust_leafname(FishStyle::No)
				)
			})
		})
		.join("")
}
