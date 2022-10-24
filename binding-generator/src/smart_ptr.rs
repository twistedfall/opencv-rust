use std::borrow::Cow;
use std::fmt;

use clang::Entity;

use crate::type_ref::{CppNameStyle, DependentTypeMode, FishStyle, NameStyle, TemplateArg};
use crate::{DefaultElement, DependentType, Element, EntityElement, GeneratorEnv, TypeRef};

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

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		if self.pointee().as_typedef().is_some() {
			self.type_ref().canonical_clang().dependent_types(DependentTypeMode::None)
		} else {
			vec![]
		}
	}

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("PtrOf{typ}", typ = self.pointee().rust_safe_id(false)).into()
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		"cv".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}

	fn rust_module(&self) -> Cow<str> {
		self.pointee().rust_module().into_owned().into()
	}

	fn rust_namespace(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultElement::rust_name(self, style)
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		format!(
			"Ptr{fish}<{typ}>",
			fish = fish_style.rust_qual(),
			typ = self.pointee().rust_name(NameStyle::ref_()),
		)
		.into()
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
