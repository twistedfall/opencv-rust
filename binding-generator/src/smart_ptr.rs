use std::{
	borrow::Cow,
	fmt,
};

use clang::Entity;

use crate::{
	DefaultElement,
	DependentType,
	Element,
	EntityElement,
	GeneratorEnv,
	type_ref::{DependentTypeMode, FishStyle, Lifetime, NameStyle, TemplateArg},
	TypeRef,
};

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
		self.type_ref().template_specialization_args().into_iter()
			.find_map(|a| if let TemplateArg::Typename(type_ref) = a {
				Some(type_ref)
			} else {
				None
			}).expect("Smart pointer template argument list is empty")
	}

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		if self.pointee().as_typedef().is_some() {
			self.type_ref().canonical_clang().dependent_types(DependentTypeMode::None)
		} else {
			vec![]
		}
	}

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("PtrOf{typ}", typ=self.pointee().rust_safe_id_ext(false)).into()
	}

	pub fn rust_fullalias(&self) -> Cow<str> {
		format!("types::{}", self.rust_localalias()).into()
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

	fn cpp_localname(&self) -> Cow<str> {
		"Ptr".into()
	}

	fn rust_module(&self) -> Cow<str> {
		self.pointee().rust_module().into_owned().into()
	}

	fn rust_namespace(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		format!(
			"Ptr{fish}<{typ}>",
			fish=fish_style.rust_qual(),
			typ=self.pointee().rust_name(NameStyle::Reference(FishStyle::Turbo), Lifetime::elided()),
		).into()
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
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
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("pointee", &self.pointee())
			.finish()
	}
}
