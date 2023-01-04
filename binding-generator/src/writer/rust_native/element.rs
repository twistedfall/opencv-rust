use std::borrow::Cow;

use clang::EntityKind;

use crate::type_ref::FishStyle;
use crate::{
	opencv_module_from_path, reserved_rename, settings, CppNameStyle, Element, EntityElement, GeneratedType, IteratorExt,
	NameStyle, StrExt, StringExt,
};

use super::comment;

pub struct DefaultRustNativeElement;

impl DefaultRustNativeElement {
	pub fn rust_module<'tu>(this: &(impl EntityElement<'tu> + ?Sized)) -> Cow<str> {
		let loc = this
			.entity()
			.get_location()
			.expect("Can't get location")
			.get_spelling_location()
			.file
			.expect("Can't file")
			.get_path();
		opencv_module_from_path(&loc).map_or_else(|| "core".into(), |x| x.to_string().into())
	}

	pub fn rust_namespace(this: &(impl RustElement + ?Sized)) -> Cow<str> {
		let module = this.rust_module();
		if settings::STATIC_MODULES.contains(module.as_ref()) {
			module
		} else {
			format!("crate::{}", this.rust_module()).into()
		}
	}

	pub fn rust_leafname(this: &(impl Element + ?Sized)) -> Cow<str> {
		reserved_rename(this.cpp_name(CppNameStyle::Declaration).to_snake_case().into())
	}

	pub fn rust_name<'tu>(this: &(impl EntityElement<'tu> + RustElement + ?Sized), name_style: NameStyle) -> Cow<str> {
		let mut parts = Vec::with_capacity(4);
		parts.push(this.rust_leafname(name_style.turbo_fish_style()).into_owned());
		let module = Self::rust_module(this);
		let mut e = this.entity();
		while let Some(parent) = e.get_semantic_parent() {
			match parent.get_kind() {
				EntityKind::ClassDecl | EntityKind::StructDecl => {
					parts.push(parent.get_name().expect("Can't get parent name"));
				}
				EntityKind::EnumDecl => {
					if parent.is_scoped() {
						parts.push(parent.get_name().expect("Can't get parent name"));
					}
				}
				EntityKind::TranslationUnit | EntityKind::UnexposedDecl => {
					break;
				}
				EntityKind::Namespace => {
					let no_skip_prefix = settings::NO_SKIP_NAMESPACE_IN_LOCALNAME
						.get(module.as_ref())
						.or_else(|| settings::NO_SKIP_NAMESPACE_IN_LOCALNAME.get("*"))
						.and_then(|config| config.get(parent.get_name().expect("Can't get parent name").as_str()));
					if let Some(&prefix) = no_skip_prefix {
						parts.push(prefix.to_string());
					} else {
						break;
					}
				}
				EntityKind::Constructor
				| EntityKind::FunctionTemplate
				| EntityKind::FunctionDecl
				| EntityKind::Method
				| EntityKind::NotImplemented => {}
				_ => {
					unreachable!("Can't get kind of parent: {:#?} for element: {:#?}", parent, e)
				}
			}
			e = parent;
		}
		let decl_name = parts.into_iter().rev().join("_");
		match name_style {
			NameStyle::Declaration => decl_name.into(),
			NameStyle::Reference(_) => {
				let mut out = this.rust_namespace();
				out.to_mut().extend_sep("::", &decl_name);
				out
			}
		}
	}

	pub fn rendered_doc_comment_with_prefix<'tu>(this: &impl EntityElement<'tu>, prefix: &str, opencv_version: &str) -> String {
		comment::render_doc_comment(&this.entity().get_comment().unwrap_or_default(), prefix, opencv_version)
	}

	pub fn rendered_doc_comment(this: &(impl RustElement + ?Sized), opencv_version: &str) -> String {
		this.rendered_doc_comment_with_prefix("///", opencv_version)
	}
}

pub trait RustNativeGeneratedElement {
	/// Element order in the output file, lower means earlier
	fn element_order(&self) -> u8 {
		50
	}

	fn element_safe_id(&self) -> String;

	fn gen_rust(&self, _opencv_version: &str) -> String {
		"".to_string()
	}

	fn gen_rust_exports(&self) -> String {
		"".to_string()
	}

	fn gen_cpp(&self) -> String {
		"".to_string()
	}
}

pub trait RustElement: Element {
	fn rust_module(&self) -> Cow<str>;

	fn rust_namespace(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_namespace(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str>;

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_leafname(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String;

	fn rendered_doc_comment(&self, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment(self, opencv_version)
	}
}

impl<'ne, 'tu: 'ne, 'ge: 'ne> AsRef<dyn RustNativeGeneratedElement + 'ne> for GeneratedType<'tu, 'ge> {
	fn as_ref(&self) -> &(dyn RustNativeGeneratedElement + 'ne) {
		match self {
			GeneratedType::AbstractRefWrapper(r) => r,
			GeneratedType::Vector(vec) => vec,
			GeneratedType::SmartPtr(ptr) => ptr,
			GeneratedType::Tuple(tuple) => tuple,
		}
	}
}
