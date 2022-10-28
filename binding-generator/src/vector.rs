use std::borrow::Cow;
use std::fmt;

use clang::{Entity, Type};

use crate::type_ref::{CppNameStyle, FishStyle, NameStyle, TemplateArg};
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

	pub fn rust_element_module(&self) -> Cow<str> {
		self.element_type().rust_module().into_owned().into()
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

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("VectorOf{typ}", typ = self.element_type().rust_safe_id(true)).into()
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		// force this to be std because on some systems the actual namespace for vector is something like "std::__1"
		"std".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultElement::rust_name(self, style)
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		let mut inner_typ = self.element_type();
		if let Some(inner) = inner_typ.as_pointer() {
			// fixme, implement references properly, use MatRef/Mut type
			inner_typ = inner;
		}
		format!(
			"Vector{fish}<{typ}>",
			fish = fish_style.rust_qual(),
			typ = inner_typ.rust_name(NameStyle::ref_()),
		)
		.into()
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
