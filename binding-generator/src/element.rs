use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Component, Path};

use clang::{Accessibility, Entity, EntityKind};

use crate::type_ref::CppNameStyle;
use crate::{settings, IteratorExt, StringExt};

pub struct DefaultElement;

impl DefaultElement {
	pub fn is_excluded(this: &(impl Element + ?Sized)) -> bool {
		this.is_ignored() || this.is_system() || settings::ELEMENT_EXCLUDE.contains(this.cpp_name(CppNameStyle::Reference).as_ref())
	}

	pub fn is_ignored(this: &(impl Element + ?Sized)) -> bool {
		!this.is_public() || settings::ELEMENT_IGNORE.contains(this.cpp_name(CppNameStyle::Reference).as_ref())
	}

	pub fn is_system<'tu>(this: &impl EntityElement<'tu>) -> bool {
		this.entity().is_in_system_header()
			&& !is_opencv_path(
				&this
					.entity()
					.get_location()
					.expect("Can't get entity location")
					.get_spelling_location()
					.file
					.expect("Can't get location path")
					.get_path(),
			)
	}

	pub fn is_public<'tu>(this: &impl EntityElement<'tu>) -> bool {
		this.entity().get_accessibility().map_or(true, |a| Accessibility::Public == a)
	}

	pub fn usr<'tu>(this: &impl EntityElement<'tu>) -> Cow<str> {
		this.entity().get_usr().expect("Can't get USR").0.into()
	}

	pub fn cpp_namespace<'tu>(this: &impl EntityElement<'tu>) -> String {
		let mut parts = vec![];
		let mut e = this.entity();
		while let Some(parent) = e.get_semantic_parent() {
			match parent.get_kind() {
				EntityKind::ClassDecl
				| EntityKind::Namespace
				| EntityKind::StructDecl
				| EntityKind::EnumDecl
				| EntityKind::UnionDecl
				| EntityKind::ClassTemplate
				| EntityKind::ClassTemplatePartialSpecialization => {
					// handle anonymous enums inside classes and anonymous namespaces
					if let Some(parent_name) = parent.get_name() {
						parts.push(parent_name);
					}
				}
				EntityKind::TranslationUnit | EntityKind::UnexposedDecl | EntityKind::NotImplemented => {}
				_ => {
					unreachable!("Can't get kind of parent for cpp namespace: {:#?}", parent)
				}
			}
			e = parent;
		}
		parts.into_iter().rev().join("::")
	}

	pub fn cpp_decl_name_with_namespace<'t>(this: &'t (impl Element + ?Sized), decl_name: &str) -> Cow<'t, str> {
		let mut out = this.cpp_namespace();
		out.to_mut().extend_sep("::", decl_name);
		out
	}

	pub fn cpp_name<'tu>(this: &(impl EntityElement<'tu> + Element + ?Sized), style: CppNameStyle) -> Cow<str> {
		let decl_name = this.entity().get_name().unwrap_or_else(|| "unnamed".to_string());
		match style {
			CppNameStyle::Declaration => decl_name.into(),
			CppNameStyle::Reference => DefaultElement::cpp_decl_name_with_namespace(this, &decl_name),
		}
	}
}

pub trait Element: fmt::Debug {
	fn update_debug_struct<'dref, 'a, 'b>(
		&self,
		struct_debug: &'dref mut fmt::DebugStruct<'a, 'b>,
	) -> &'dref mut fmt::DebugStruct<'a, 'b> {
		struct_debug
			.field("cpp_fullname", &self.cpp_name(CppNameStyle::Reference))
			.field("is_excluded", &self.is_excluded())
			.field("is_ignored", &self.is_ignored())
			.field("is_system", &self.is_system())
			.field("is_public", &self.is_public())
	}

	/// true if an element shouldn't be generated
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
	}

	/// true if there shouldn't be any references to that element
	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self)
	}

	fn is_system(&self) -> bool;

	fn is_public(&self) -> bool;

	fn usr(&self) -> Cow<str>;

	fn cpp_namespace(&self) -> Cow<str>;

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str>;
}

pub trait EntityElement<'tu> {
	fn entity(&self) -> Entity<'tu>;
}

pub fn is_opencv_path(path: &Path) -> bool {
	path
		.components()
		.rfind(|c| {
			if let Component::Normal(c) = c {
				if *c == "opencv2" || *c == "Headers" {
					return true;
				}
			}
			false
		})
		.is_some()
}

/// Returns path component that corresponds to OpenCV module name. It's either a directory
/// (e.g. "calib3d") or a header file (e.g. "dnn.hpp")
fn opencv_module_component(path: &Path) -> Option<&OsStr> {
	let mut module_comp = path
		.components()
		.rev()
		.filter_map(|c| {
			if let Component::Normal(c) = c {
				Some(c)
			} else {
				None
			}
		})
		.peekable();
	let mut module = None;
	while let Some(cur) = module_comp.next() {
		if let Some(&parent) = module_comp.peek() {
			if parent == "opencv2" || parent == "src_cpp" || parent == "Headers" {
				module = Some(cur);
				break;
			}
		}
	}
	module
}

/// Return OpenCV module name if the path points to the main header file, e.g. "opencv2/dnn.hpp".
pub fn main_opencv_module_from_path(path: &Path) -> Option<&str> {
	opencv_module_component(path)
		.and_then(|m| m.to_str())
		.and_then(|m| m.strip_suffix(".hpp"))
}

/// Return OpenCV module from the given path
pub fn opencv_module_from_path(path: &Path) -> Option<&str> {
	opencv_module_component(path)
		.and_then(|m| m.to_str())
		.and_then(|m| m.strip_suffix(".hpp").or(Some(m)))
}
