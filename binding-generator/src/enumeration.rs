use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::debug::LocationName;
use crate::element::{ExcludeKind, UNNAMED};
use crate::entity::WalkAction;
use crate::type_ref::CppNameStyle;
use crate::{Const, DefaultElement, Element, EntityElement, EntityExt, NameDebug, StrExt};

#[derive(Clone, PartialEq)]
pub struct Enum<'tu> {
	entity: Entity<'tu>,
	custom_fullname: Option<Rc<str>>,
}

impl<'tu> Enum<'tu> {
	pub fn new(entity: Entity<'tu>) -> Self {
		Self {
			entity,
			custom_fullname: None,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: impl Into<Rc<str>>) -> Self {
		Self {
			entity,
			custom_fullname: Some(custom_fullname.into()),
		}
	}

	pub fn is_anonymous(&self) -> bool {
		self.entity.is_anonymous() || /* for clang-6 quirk */ self.cpp_name(CppNameStyle::Declaration) == UNNAMED
	}

	pub fn as_typedefed(&self) -> Option<Entity> {
		if matches!(self.entity.get_kind(), EntityKind::TypedefDecl | EntityKind::TypeAliasDecl) {
			let mut child = None;
			self.entity.walk_children_while(|c| {
				child = Some(c);
				WalkAction::Interrupt
			});
			Some(child.expect("Invalid anonymous typedefed enum"))
		} else {
			None
		}
	}

	pub fn consts(&self) -> Vec<Const> {
		let mut out = vec![];
		self.as_typedefed().unwrap_or(self.entity).visit_children(|const_decl, _| {
			if const_decl.get_kind() == EntityKind::EnumConstantDecl {
				out.push(Const::new(const_decl));
			}
			EntityVisitResult::Continue
		});
		out
	}
}

impl<'tu> EntityElement<'tu> for Enum<'tu> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Enum<'_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_is_excluded(|| self.as_typedefed().is_some())
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity)
	}

	fn doc_comment(&self) -> Cow<str> {
		self.entity.get_comment().unwrap_or_default().into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.namespace().into()
		} else {
			DefaultElement::cpp_namespace(self.entity).into()
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		if let Some(custom_fullname) = self.custom_fullname.as_deref() {
			custom_fullname.cpp_name_from_fullname(style).into()
		} else {
			DefaultElement::cpp_name(self, self.entity(), style)
		}
	}
}

impl<'me> NameDebug<'me> for &'me Enum<'_> {
	fn file_line_name(self) -> LocationName<'me> {
		self.entity.file_line_name()
	}
}

impl fmt::Debug for Enum<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Enum");
		self
			.update_debug_struct(&mut debug_struct)
			.field("consts", &self.consts())
			.finish()
	}
}
