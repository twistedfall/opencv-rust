use std::borrow::Cow;
use std::fmt;

use clang::{Entity, Type};

use crate::type_ref::{CppNameStyle, TemplateArg};
use crate::{DefaultElement, Element, EntityElement, GeneratedType, GeneratorEnv, TypeRef};

#[derive(Clone)]
pub struct Vector<'tu, 'ge> {
	type_ref: Type<'tu>,
	pub(crate) gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Vector<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { type_ref, gen_env }
	}

	pub fn is_data_type(&self, type_ref: &TypeRef) -> bool {
		type_ref.is_data_type() || type_ref.as_vector().map_or(false, |v| v.element_type().is_data_type())
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.type_ref, self.gen_env)
	}

	pub fn element_type(&self) -> TypeRef<'tu, 'ge> {
		self
			.type_ref()
			.template_specialization_args()
			.into_iter()
			.find_map(TemplateArg::into_typename)
			.expect("vector template argument list is empty")
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self.element_type().generated_types()
	}
}

impl<'tu> EntityElement<'tu> for Vector<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.type_ref.get_declaration().expect("Can't get declaration")
	}
}

impl Element for Vector<'_, '_> {
	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.element_type().is_ignored()
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
		// force this to be std because on some systems the actual namespace for vector is something like "std::__1"
		"std".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}
}

impl fmt::Display for Vector<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity().get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Vector<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Vector");
		self
			.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity()))
			.field("element_type", &self.element_type())
			.finish()
	}
}
