use std::borrow::Cow;
use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::entity::WalkAction;
use crate::type_ref::CppNameStyle;
use crate::{Const, DefaultElement, Element, EntityElement, EntityExt, StrExt};

#[derive(Clone)]
pub struct Enum<'tu> {
	entity: Entity<'tu>,
	custom_fullname: Option<String>,
}

impl<'tu> Enum<'tu> {
	pub fn new(entity: Entity<'tu>) -> Self {
		Self {
			entity,
			custom_fullname: None,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: String) -> Self {
		Self {
			entity,
			custom_fullname: Some(custom_fullname),
		}
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
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.namespace().into()
		} else {
			DefaultElement::cpp_namespace(self).into()
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			match style {
				CppNameStyle::Declaration => custom_fullname.localname().into(),
				CppNameStyle::Reference => custom_fullname.into(),
			}
		} else {
			DefaultElement::cpp_name(self, style)
		}
	}
}

impl fmt::Display for Enum<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			self.entity.get_display_name().unwrap_or_else(|| "unnamed".to_string())
		)
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
