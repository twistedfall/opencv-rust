use std::{
	borrow::Cow,
	fmt,
};

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::{
	Const,
	DefaultElement,
	Element,
	EntityElement,
	EntityExt,
	StrExt,
	type_ref::FishStyle,
};

#[derive(Clone)]
pub struct Enum<'tu> {
	entity: Entity<'tu>,
	custom_fullname: Option<String>,
}

impl<'tu> Enum<'tu> {
	pub fn new(entity: Entity<'tu>) -> Self {
		Self { entity, custom_fullname: None }
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: String) -> Self {
		Self { entity, custom_fullname: Some(custom_fullname) }
	}

	pub fn as_typedefed(&self) -> Option<Entity> {
		if matches!(self.entity.get_kind(), EntityKind::TypedefDecl | EntityKind::TypeAliasDecl) {
			let mut child = None;
			self.entity.walk_children_while(|c| {
				child = Some(c);
				false
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		if self.custom_fullname.is_some() {
			self.cpp_fullname().namespace().to_string().into()
		} else {
			DefaultElement::cpp_namespace(self).into()
		}
	}

	fn cpp_localname(&self) -> Cow<str> {
		if self.custom_fullname.is_some() {
			self.cpp_fullname().localname().to_string().into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn cpp_fullname(&self) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.into()
		} else {
			DefaultElement::cpp_fullname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		self.cpp_localname()
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
	}
}

impl fmt::Display for Enum<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().unwrap_or_else(|| "unnamed".to_string()))
	}
}

impl fmt::Debug for Enum<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Enum");
		self.update_debug_struct(&mut debug_struct)
			.field("consts", &self.consts())
			.finish()
	}
}
