use std::{
	borrow::Cow,
	fmt,
};

use clang::{
	Entity,
	Type,
};

use crate::{
	ConstnessOverride,
	DefaultElement,
	DefinitionLocation,
	DependentType,
	DependentTypeMode,
	Element,
	EntityElement,
	GeneratorEnv,
	ReturnTypeWrapper,
	type_ref::{FishStyle, TemplateArg},
	TypeRef,
};

#[derive(Clone)]
pub struct Vector<'tu, 'ge> {
	type_ref: Type<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
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
		self.type_ref().template_specialization_args().into_iter()
			.find_map(|a| if let TemplateArg::Typename(type_ref) = a {
				Some(type_ref)
			} else {
				None
			}).expect("vector template argument list is empty")
	}

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		let element_type = self.element_type();
		let is_data_type = self.is_data_type(&element_type);
		let mut out = element_type.dependent_types(DependentTypeMode::ForReturn(DefinitionLocation::Type));
		out.reserve(1 + if is_data_type { 3 } else { 0 });
		if element_type.as_string().is_some() {
			out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type(&element_type.cpp_extern_return(ConstnessOverride::No)).expect("Can't resolve string cpp_extern_return()"), self.gen_env),
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
				self.gen_env,
			)));
		} else {
			out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(element_type.canonical_clang(), DefinitionLocation::Module, self.gen_env)));
		}
		if is_data_type {
			out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputArray").expect("Can't resolve _InputArray"), self.gen_env),
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
				self.gen_env,
			)));
			out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_OutputArray").expect("Can't resolve _OutputArray"), self.gen_env),
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
				self.gen_env,
			)));
			out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputOutputArray").expect("Can't resolve _InputOutputArray"), self.gen_env),
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
				self.gen_env,
			)));
		}
		out
	}

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("VectorOf{typ}", typ=self.element_type().rust_safe_id()).into()
	}

	pub fn rust_fullalias(&self) -> Cow<str> {
		format!("types::{}", self.rust_localalias()).into()
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
		"std".into()
	}

	fn cpp_localname(&self) -> Cow<str> {
		"vector".into()
	}

	fn rust_module(&self) -> Cow<str> {
		self.element_type().rust_module().into_owned().into()
	}

	fn rust_namespace(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		let mut inner_typ = self.element_type();
		if let Some(inner) = inner_typ.as_pointer() { // fixme, implement references properly, use MatRef/Mut type
			inner_typ = inner;
		}
		format!(
			"Vector{fish}<{typ}>",
			fish=fish_style.rust_qual(),
			typ=inner_typ.rust_full(),
		).into()
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
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
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity()))
			.field("element_type", &self.element_type())
			.finish()
	}
}
