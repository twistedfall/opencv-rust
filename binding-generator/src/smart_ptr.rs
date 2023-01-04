use std::borrow::Cow;
use std::fmt;

use clang::Entity;

use crate::type_ref::{CppNameStyle, TemplateArg};
use crate::{DefaultElement, Element, EntityElement, GeneratedType, GeneratorEnv, TypeRef};

#[derive(Clone)]
pub struct SmartPtr<'tu, 'ge> {
	entity: Entity<'tu>,
	pub(crate) gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> SmartPtr<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.entity.get_type().expect("Can't get smart pointer type"), self.gen_env)
	}

	pub fn pointee(&self) -> TypeRef<'tu, 'ge> {
		self
			.type_ref()
			.template_specialization_args()
			.into_iter()
			.find_map(TemplateArg::into_typename)
			.expect("Smart pointer template argument list is empty")
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		if self.pointee().as_typedef().is_some() {
			self.type_ref().canonical_clang().generated_types()
		} else {
			vec![]
		}
	}
}

impl<'tu> EntityElement<'tu> for SmartPtr<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for SmartPtr<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self) || self.pointee().is_excluded()
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.pointee().is_ignored()
	}

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
		"cv".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}
}

impl fmt::Display for SmartPtr<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for SmartPtr<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("SmartPtr");
		self
			.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("pointee", &self.pointee())
			.finish()
	}
}
