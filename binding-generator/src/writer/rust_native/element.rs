use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::ControlFlow;

use clang::{Entity, EntityKind};

use super::comment::RenderComment;
use crate::type_ref::FishStyle;
use crate::{
	opencv_module_from_path, reserved_rename, settings, CppNameStyle, Element, EntityExt, GeneratedType, IteratorExt, NameStyle,
	StringExt, SupportedModule,
};

pub struct DefaultRustNativeElement;

impl DefaultRustNativeElement {
	pub fn rust_module(entity: Entity) -> SupportedModule {
		entity
			.get_location()
			.expect("Can't get location")
			.get_spelling_location()
			.file
			.and_then(|file| opencv_module_from_path(&file.get_path()))
			.map_or(SupportedModule::Core, |m| rust_module_hack(entity, m))
	}

	pub fn rust_module_reference(this: &(impl RustElement + ?Sized)) -> Cow<'_, str> {
		let module = this.rust_module();
		let module_rust_safe_name = module.rust_safe_name();
		if settings::STATIC_RUST_MODULES.contains(module_rust_safe_name) {
			module_rust_safe_name.into()
		} else {
			format!("crate::{module_rust_safe_name}").into()
		}
	}

	pub fn rust_leafname(this: &(impl Element + ?Sized)) -> Cow<'_, str> {
		reserved_rename(this.cpp_name(CppNameStyle::Declaration))
	}

	pub fn rust_name(this: &(impl RustElement + ?Sized), entity: Entity, name_style: NameStyle) -> String {
		let mut parts = Vec::with_capacity(4);
		parts.push(this.rust_leafname(name_style.turbo_fish_style()));
		let mut entity = entity;
		let module = Self::rust_module(entity);
		while let Some(parent) = entity.get_semantic_parent() {
			match parent.get_kind() {
				EntityKind::ClassDecl | EntityKind::StructDecl | EntityKind::ClassTemplate => {
					let parent_name = parent.get_name().expect("Can't get parent name");
					if parts.last().is_none_or(|last| last != &parent_name) {
						parts.push(parent_name.into());
					}
				}
				EntityKind::EnumDecl => {
					if parent.is_scoped() {
						parts.push(parent.get_name().expect("Can't get parent name").into());
					}
				}
				EntityKind::TranslationUnit | EntityKind::UnexposedDecl | EntityKind::FunctionTemplate => break,
				EntityKind::Namespace => {
					let parent_namespace = parent.get_name().expect("Can't get parent name");
					let no_skip_prefix = settings::NO_SKIP_NAMESPACE_IN_LOCALNAME
						.get(&Some(module))
						.and_then(|module_specific| module_specific.get(parent_namespace.as_str()))
						.or_else(|| {
							settings::NO_SKIP_NAMESPACE_IN_LOCALNAME
								.get(&None)
								.and_then(|generic| generic.get(parent_namespace.as_str()))
						});
					if let Some(&prefix) = no_skip_prefix {
						parts.push(prefix.into());
					}
				}
				EntityKind::Constructor | EntityKind::FunctionDecl | EntityKind::Method | EntityKind::NotImplemented => {}
				_ => unreachable!("Can't get kind of parent: {parent:#?} for element: {entity:#?}"),
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

	fn gen_rust_externs(&self) -> String {
		"".to_string()
	}

	fn gen_cpp(&self) -> String {
		"".to_string()
	}
}

pub trait RustElement: Element {
	fn rust_module(&self) -> SupportedModule;

	fn rust_module_reference(&self) -> Cow<'_, str> {
		DefaultRustNativeElement::rust_module_reference(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str>;

	/// The very last concrete part of the name in Rust
	///
	/// This might not match `rust_name(NameStyle::Declaration)` because some classes in Rust are prefixed with their namespace. E.g.
	/// `Detail_Blender`, in this case the `rust_leafname()` == `Blender` and `rust_name(NameStyle::Declaration)` == `Detail_Blender`.
	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<'_, str> {
		DefaultRustNativeElement::rust_leafname(self)
	}

	fn rust_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		RenderComment::new(self.doc_comment().into_owned(), opencv_version)
			.render_with_comment_marker(comment_marker)
			.into_owned()
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

/// Adjusts the module for certain entities that are technically in one module but actually implemented in another.
///
/// One (and for now the only) example is that some class declarations are technically in `hpp` files in the `video` module, but
/// their code is actually in the `tracking` module of `opencv_contrib`. This mismatch leads to linking errors if only the
/// `opencv_video` module is linked.
fn rust_module_hack(entity: Entity, module_from_path: SupportedModule) -> SupportedModule {
	match module_from_path {
		SupportedModule::Video => {
			let break_res = entity.walk_parents(|parent| match parent.get_kind() {
				EntityKind::Namespace if parent.is_inline_namespace() && parent.get_name().is_some_and(|n| n == "tracking") => {
					ControlFlow::Break(SupportedModule::Tracking)
				}
				_ => ControlFlow::Continue(()),
			});
			// todo: MSRV 1.83 use `break_value()`
			match break_res {
				ControlFlow::Break(m) => Some(m),
				ControlFlow::Continue(_) => None,
			}
			.unwrap_or(module_from_path)
		}
		_ => module_from_path,
	}
}
