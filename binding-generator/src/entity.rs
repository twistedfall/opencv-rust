use std::borrow::Cow;

use clang::{Entity, EntityKind, EntityVisitResult, StorageClass};

use crate::type_ref::CppNameStyle;
use crate::{DefaultElement, Element, EntityElement};

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

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}
}

#[derive(Copy, Clone)]
pub enum WalkAction {
	Continue,
	Interrupt,
}

impl WalkAction {
	pub fn continue_until(condition: bool) -> Self {
		if condition {
			WalkAction::Interrupt
		} else {
			WalkAction::Continue
		}
	}
}

impl From<WalkAction> for EntityVisitResult {
	fn from(value: WalkAction) -> Self {
		match value {
			WalkAction::Continue => Self::Continue,
			WalkAction::Interrupt => Self::Break,
		}
	}
}

#[derive(Copy, Clone)]
pub enum WalkResult {
	Completed,
	Interrupted,
}

impl WalkResult {
	pub fn is_interrupted(self) -> bool {
		matches!(self, WalkResult::Interrupted)
	}
}

pub trait EntityExt<'tu> {
	fn walk_children_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_bases_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_enums_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_classes_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_typedefs_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_fields_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_consts_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_methods_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
}

impl<'tu> EntityExt<'tu> for Entity<'tu> {
	/// # Arguments
	/// * `predicate`: returns true to continue iteration or false to stop
	/// returns: true if predicate stopped iteration, false otherwise
	fn walk_children_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		let res = self.visit_children(|child, _| match predicate(child) {
			WalkAction::Continue => EntityVisitResult::Continue,
			WalkAction::Interrupt => EntityVisitResult::Break,
		});
		if res {
			WalkResult::Interrupted
		} else {
			WalkResult::Completed
		}
	}

	fn walk_bases_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::BaseSpecifier => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_enums_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::EnumDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_classes_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_typedefs_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_fields_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::FieldDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_consts_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::VarDecl => {
				if let Some(StorageClass::Static) = child.get_storage_class() {
					if child.evaluate().is_some() {
						predicate(child)
					} else {
						WalkAction::Continue
					}
				} else {
					panic!("Non-static constant: {:#?}", child)
				}
			}
			_ => WalkAction::Continue,
		})
	}

	fn walk_methods_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate | EntityKind::ConversionFunction => {
				predicate(child)
			}
			_ => WalkAction::Continue,
		})
	}
}
