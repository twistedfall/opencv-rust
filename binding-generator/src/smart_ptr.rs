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
	type_ref::TemplateArg,
	TypeRef,
};

#[derive(Clone)]
pub struct SmartPtr<'tu> {
	entity: Entity<'tu>,
	gen_env: &'tu GeneratorEnv<'tu>,
}

impl<'tu> SmartPtr<'tu> {
	pub fn new(entity: Entity<'tu>, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self { entity, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu> {
		TypeRef::new(self.entity.get_type().expect("Can't get smartptr type"), self.gen_env)
	}

	pub fn pointee(&self) -> TypeRef<'tu> {
		self.type_ref().template_specialization_args().into_iter()
			.find_map(|a| if let TemplateArg::Typename(type_ref) = a {
				Some(type_ref)
			} else {
				None
			}).expect("smart pointer template argument list is empty")
	}

	pub fn dependent_types<D: DependentType<'tu>>(&self) -> Vec<D> {
		if self.pointee().as_typedef().is_some() {
			self.type_ref().canonical_clang().dependent_types()
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

impl<'tu> EntityElement<'tu> for SmartPtr<'tu> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for SmartPtr<'_> {
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

	fn rust_leafname(&self) -> Cow<str> {
		format!("Ptr::<{typ}>", typ=self.pointee().rust_full()).into()
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

impl fmt::Display for SmartPtr<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for SmartPtr<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("SmartPtr");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("pointee", &self.pointee())
			.finish()
	}
}
