use std::borrow::Cow;
use std::fmt::Debug;

use clang::{Entity, EntityKind};

use crate::type_ref::FishStyle;
use crate::{
	opencv_module_from_path, reserved_rename, settings, CppNameStyle, Element, GeneratedType, IteratorExt, NameStyle, StrExt,
	StringExt,
};

use super::comment;

pub struct DefaultRustNativeElement;

impl DefaultRustNativeElement {
	pub fn rust_module(entity: Entity) -> Cow<'static, str> {
		let loc = entity
			.get_location()
			.expect("Can't get location")
			.get_spelling_location()
			.file
			.expect("Can't file")
			.get_path();
		opencv_module_from_path(&loc).map_or_else(|| "core".into(), |x| x.to_string().into())
	}

	pub fn rust_module_reference(this: &(impl RustElement + ?Sized)) -> Cow<str> {
		let module = this.rust_module();
		if settings::STATIC_MODULES.contains(module.as_ref()) {
			module
		} else {
			format!("crate::{}", module).into()
		}
	}

	pub fn rust_leafname(this: &(impl Element + ?Sized)) -> Cow<str> {
		reserved_rename(this.cpp_name(CppNameStyle::Declaration).cpp_name_to_rust_case().into())
	}

	pub fn rust_name(this: &(impl RustElement + ?Sized), entity: Entity, name_style: NameStyle) -> String {
		let mut parts = Vec::with_capacity(4);
		parts.push(this.rust_leafname(name_style.turbo_fish_style()).into_owned());
		let mut entity = entity;
		let module = Self::rust_module(entity);
		while let Some(parent) = entity.get_semantic_parent() {
			match parent.get_kind() {
				EntityKind::ClassDecl | EntityKind::StructDecl | EntityKind::ClassTemplate => {
					let parent_name = parent.get_name().expect("Can't get parent name");
					if parts.last().map_or(true, |last| last != &parent_name) {
						parts.push(parent_name);
					}
				}
				EntityKind::EnumDecl => {
					if parent.is_scoped() {
						parts.push(parent.get_name().expect("Can't get parent name"));
					}
				}
				EntityKind::TranslationUnit | EntityKind::UnexposedDecl | EntityKind::FunctionTemplate => {
					break;
				}
				EntityKind::Namespace => {
					let parent_namespace = parent.get_name().expect("Can't get parent name");
					let no_skip_prefix = settings::NO_SKIP_NAMESPACE_IN_LOCALNAME
						.get(module.as_ref())
						.and_then(|module_specific| module_specific.get(parent_namespace.as_str()))
						.or_else(|| {
							settings::NO_SKIP_NAMESPACE_IN_LOCALNAME
								.get("*")
								.and_then(|generic| generic.get(parent_namespace.as_str()))
						});
					if let Some(&prefix) = no_skip_prefix {
						parts.push(prefix.to_string());
					} else {
						break;
					}
				}
				EntityKind::Constructor | EntityKind::FunctionDecl | EntityKind::Method | EntityKind::NotImplemented => {}
				_ => {
					unreachable!("Can't get kind of parent: {parent:#?} for element: {entity:#?}")
				}
			}
			entity = parent;
		}
		let decl_name = parts.into_iter().rev().join("_");
		match name_style {
			NameStyle::Declaration => decl_name,
			NameStyle::Reference(_) => {
				let mut out = this.rust_module_reference().into_owned();
				out.extend_sep("::", &decl_name);
				out
			}
		}
	}

	pub fn rendered_doc_comment_with_prefix(entity: Entity, prefix: &str, opencv_version: &str) -> String {
		let comment = entity.get_comment();
		comment::render_doc_comment(comment.as_deref().unwrap_or(""), prefix, opencv_version)
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

	fn rust_module_reference(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module_reference(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str>;

	/// The very last concrete part of the name in Rust
	///
	/// This might not match `rust_name(NameStyle::Declaration)` because some classes in Rust are prefixed with their namespace. E.g.
	/// `Detail_Blender`, in this case the `rust_leafname()` == `Blender` and `rust_name(NameStyle::Declaration)` == `Detail_Blender`.
	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_leafname(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String;

	fn rendered_doc_comment(&self, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment(self, opencv_version)
	}
}

pub trait DebugRust {
	type DebugType: Debug;

	fn dbg_rust(self) -> Self::DebugType;
}

impl<'ne, 'tu: 'ne, 'ge: 'ne> AsRef<dyn RustNativeGeneratedElement + 'ne> for GeneratedType<'tu, 'ge> {
	fn as_ref(&self) -> &(dyn RustNativeGeneratedElement + 'ne) {
		match self {
			GeneratedType::Vector(vec) => vec,
			GeneratedType::SmartPtr(ptr) => ptr,
			GeneratedType::Tuple(tuple) => tuple,
			GeneratedType::AbstractRefWrapper(aref) => aref,
		}
	}
}
