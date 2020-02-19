use std::borrow::Cow;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::{
	element::{DefaultElement, EntityElement},
	Element,
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
		DefaultElement::cpp_namespace(self)
	}

	fn cpp_localname(&self) -> Cow<str> {
		DefaultElement::cpp_localname(self)
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

pub trait EntityExt<'tu> {
	fn walk_children_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_bases_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_enums_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_classes_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_typedefs_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_fields_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
	fn walk_methods_while(&self, predicate: impl FnMut(Entity<'tu>) -> bool) -> bool;
}

impl<'tu> EntityExt<'tu> for Entity<'tu> {
	fn walk_children_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.visit_children(|child, _| {
			if predicate(child) {
				EntityVisitResult::Continue
			} else {
				EntityVisitResult::Break
			}
		})
	}

	fn walk_bases_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::BaseSpecifier => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}

	fn walk_enums_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::EnumDecl => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}

	fn walk_classes_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::ClassDecl | EntityKind::StructDecl => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}

	fn walk_typedefs_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::TypedefDecl => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}

	fn walk_fields_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::FieldDecl | EntityKind::VarDecl => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}

	fn walk_methods_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> bool) -> bool {
		self.walk_children_while(|child| {
			match child.get_kind() {
				EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
				| EntityKind::ConversionFunction => {
					predicate(child)
				}
				_ => {
					true
				}
			}
		})
	}
}

