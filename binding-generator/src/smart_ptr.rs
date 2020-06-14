use std::{
	borrow::Cow,
	fmt,
};

use clang::Entity;

use crate::{
	DefaultElement,
	DefinitionLocation,
	DependentType,
	Element,
	EntityElement,
	GeneratorEnv,
	ReturnTypeWrapper,
	type_ref::TemplateArg,
	TypeRef,
};

#[derive(Clone)]
pub struct SmartPtr<'tu, 'g> {
	entity: Entity<'tu>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> SmartPtr<'tu, 'g> {
	pub fn new(entity: Entity<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.entity.get_type().expect("Can't get smartptr type"), self.gen_env)
	}

	pub fn pointee(&self) -> TypeRef<'tu, 'g> {
		self.type_ref().template_specialization_args().into_iter()
			.find_map(|a| if let TemplateArg::Typename(type_ref) = a {
				Some(type_ref)
			} else {
				None
			}).expect("smart pointer template argument list is empty")
	}

	pub fn dependent_types<D: DependentType<'tu>>(&self) -> Vec<D> {
		let canon = self.type_ref().canonical_clang();
		let mut out = vec![
			D::from_return_type_wrapper(ReturnTypeWrapper::new(canon.clone(), self.gen_env, DefinitionLocation::Module))
		];
		if self.pointee().as_typedef().is_some() {
			out.extend(canon.dependent_types())
		}
		out
	}

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("PtrOf{typ}", typ=self.pointee().rust_safe_id()).into()
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

	fn rust_leafname(&self) -> Cow<str> {
		format!("Ptr::<{typ}>", typ=self.pointee().rust_full()).into()
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
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
