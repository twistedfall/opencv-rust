use std::borrow::Cow;

use clang::{Entity, EntityKind, EntityVisitResult, StorageClass};

use crate::{
	element::{DefaultElement, EntityElement},
	Element,
	type_ref::FishStyle,
};

impl<'tu> EntityElement<'tu> for Entity<'tu> {
	fn entity(&self) -> Entity<'tu> {
		*self
	}
}

impl Element for Entity<'_> {
	fn is_system(&self) -> bool {
		DefaultElement::is_system(self)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self)
	}

	fn usr(&self) -> Cow<str> {
		DefaultElement::usr(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self).into()
	}

	fn cpp_localname(&self) -> Cow<str> {
		DefaultElement::cpp_localname(self)
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
	}
}

pub trait EntityExt<'tu> {
	fn walk_children_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_bases_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_enums_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_classes_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_typedefs_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_fields_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_consts_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_methods_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
}

impl<'tu> EntityExt<'tu> for Entity<'tu> {
	/// # Arguments
	/// * `predicate`: returns true to continue iteration or false to break
	/// returns: true if predicate returned false, false otherwise
	fn walk_children_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.visit_children(|child, _| if predicate(child) {
			EntityVisitResult::Continue
		} else {
			EntityVisitResult::Break
		})
	}

	fn walk_bases_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::BaseSpecifier => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}

	fn walk_enums_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::EnumDecl => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}

	fn walk_classes_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}

	fn walk_typedefs_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}

	fn walk_fields_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::FieldDecl => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}

	fn walk_consts_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::VarDecl => {
				if let Some(StorageClass::Static) = child.get_storage_class() {
					if child.evaluate().is_some() {
						predicate(child)
					} else {
						true
						// panic!("Non-evaluatable constant: {:#?}", child)
					}
				} else {
					panic!("Non-static constant: {:#?}", child)
				}
			}
			_ => {
				true
			}
		})
	}

	fn walk_methods_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
			| EntityKind::ConversionFunction => {
				predicate(child)
			}
			_ => {
				true
			}
		})
	}
}

