use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Component, Path};

use clang::{Accessibility, Entity, EntityKind};

use crate::type_ref::CppNameStyle;
use crate::{settings, IteratorExt, StrExt, StringExt};

pub const UNNAMED: &str = "unnamed";

pub struct DefaultElement;

impl DefaultElement {
	pub fn exclude_kind(this: &(impl Element + ?Sized)) -> ExcludeKind {
		let cpp_refname = this.cpp_name(CppNameStyle::Reference);
		ExcludeKind::Included
			.with_is_ignored(|| {
				!this.is_public()
					|| settings::ELEMENT_EXCLUDE_KIND
						.get(cpp_refname.as_ref())
						.map_or(false, |ek| ek.is_ignored())
			})
			.with_is_excluded(|| {
				(this.is_system() && !settings::IMPLEMENTED_SYSTEM_CLASSES.contains(cpp_refname.as_ref()))
					|| settings::ELEMENT_EXCLUDE_KIND
						.get(cpp_refname.as_ref())
						.map_or(false, |ek| ek.is_excluded())
			})
	}

	pub fn is_system(entity: Entity) -> bool {
		entity.is_in_system_header()
			&& !is_opencv_path(
				&entity
					.get_location()
					.expect("Can't get entity location")
					.get_spelling_location()
					.file
					.expect("Can't get location path")
					.get_path(),
			)
	}

	pub fn is_public(entity: Entity) -> bool {
		entity.get_accessibility().map_or(true, |a| Accessibility::Public == a)
	}

	pub fn cpp_namespace(entity: Entity) -> String {
		let mut entity = entity;
		let mut parts = vec![];
		while let Some(parent) = entity.get_semantic_parent() {
			match parent.get_kind() {
				EntityKind::ClassDecl
				| EntityKind::Namespace
				| EntityKind::StructDecl
				| EntityKind::EnumDecl
				| EntityKind::UnionDecl
				| EntityKind::ClassTemplate
				| EntityKind::ClassTemplatePartialSpecialization
				| EntityKind::FunctionTemplate
				| EntityKind::Method => {
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
			entity = parent;
		}
		parts.into_iter().rev().join("::")
	}

	pub fn cpp_decl_name_with_namespace<'t>(this: &'t (impl Element + ?Sized), decl_name: &str) -> Cow<'t, str> {
		let mut out = this.cpp_namespace();
		out.to_mut().extend_sep("::", decl_name);
		out
	}

	pub fn cpp_name<'t>(this: &'t (impl Element + ?Sized), entity: Entity, style: CppNameStyle) -> Cow<'t, str> {
		let decl_name = entity
			.get_name()
			.or_else(|| {
				if matches!(
					entity.get_kind(),
					EntityKind::StructDecl | EntityKind::ClassDecl | EntityKind::EnumDecl
				) {
					// for <clang-16 the classes and enums defined with `typedef struct` will have no name
					// and the only way to get the name from the typedef is to rely on the type's `get_display_name`
					entity.get_type().map(|typ| {
						typ.get_display_name()
							.cpp_name_from_fullname(CppNameStyle::Declaration)
							.to_string()
					})
				} else {
					None
				}
			})
			.map_or(Cow::Borrowed(UNNAMED), Cow::Owned);
		match style {
			CppNameStyle::Declaration => decl_name,
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
			.field("exclude_kind", &self.exclude_kind())
			.field("is_system", &self.is_system())
			.field("is_public", &self.is_public())
	}

	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self)
	}

	fn is_system(&self) -> bool;

	fn is_public(&self) -> bool;

	fn doc_comment(&self) -> Cow<str>;

	fn cpp_namespace(&self) -> Cow<str>;

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExcludeKind {
	Included,
	/// The bindings will not be generated for this element
	Excluded,
	/// Like [ExcludeKind::Excluded], but any element that references this element will also be excluded
	Ignored,
}

impl ExcludeKind {
	pub fn is_excluded(self) -> bool {
		match self {
			Self::Excluded | Self::Ignored => true,
			Self::Included => false,
		}
	}

	pub fn is_ignored(self) -> bool {
		match self {
			Self::Ignored => true,
			Self::Included | Self::Excluded => false,
		}
	}

	pub fn is_included(self) -> bool {
		match self {
			Self::Included => true,
			Self::Ignored | Self::Excluded => false,
		}
	}

	/// Return [ExcludeKind::Excluded] if `is_excluded` is true and `self` is [ExcludeKind::Included],
	/// `self` otherwise
	pub fn with_is_excluded(self, is_excluded: impl FnOnce() -> bool) -> ExcludeKind {
		match self {
			Self::Included => {
				if is_excluded() {
					Self::Excluded
				} else {
					self
				}
			}
			Self::Excluded | Self::Ignored => self,
		}
	}

	/// Return [ExcludeKind::Ignored] if `is_ignored` is true, `self` otherwise
	pub fn with_is_ignored(self, is_ignored: impl FnOnce() -> bool) -> ExcludeKind {
		match self {
			Self::Included | Self::Excluded => {
				if is_ignored() {
					Self::Ignored
				} else {
					self
				}
			}
			Self::Ignored => self,
		}
	}

	/// Return the most ignored kind between `self` and `new()`
	pub fn with_exclude_kind(self, new: impl FnOnce() -> ExcludeKind) -> ExcludeKind {
		match self {
			Self::Included => new(),
			Self::Excluded => match new() {
				Self::Ignored => Self::Ignored,
				Self::Included | Self::Excluded => self,
			},
			Self::Ignored => self,
		}
	}

	/// Like [ExcludeKind::with_exclude_kind], but will return [ExcludeKind::Excluded] if `new()`
	/// returns [ExcludeKind::Ignored], useful for container types like [crate::Vector]
	pub fn with_reference_exclude_kind(self, new: impl FnOnce() -> ExcludeKind) -> ExcludeKind {
		match self {
			Self::Included => match new() {
				Self::Ignored => Self::Excluded,
				Self::Included | Self::Excluded => self,
			},
			Self::Excluded | Self::Ignored => self,
		}
	}
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

/// Return OpenCV module from the given path
pub fn opencv_module_from_path(path: &Path) -> Option<&str> {
	opencv_module_component(path)
		.and_then(|m| m.to_str())
		.and_then(|m| m.strip_suffix(".hpp").or(Some(m)))
}
