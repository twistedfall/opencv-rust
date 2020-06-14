use std::{
	borrow::Cow,
	fmt,
};

use clang::{
	Entity,
	Type,
};

use crate::{
	DefaultElement,
	DefinitionLocation,
	DependentType,
	DependentTypeMode,
	Element,
	EntityElement,
	GeneratorEnv,
	ReturnTypeWrapper,
	type_ref::TemplateArg,
	TypeRef,
};

#[derive(Clone)]
pub struct Vector<'tu, 'g> {
	type_ref: Type<'tu>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> Vector<'tu, 'g> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { type_ref, gen_env }
	}

	pub fn is_data_type(&self, type_ref: &TypeRef) -> bool {
		type_ref.is_data_type() || type_ref.as_vector().map_or(false, |v| v.element_type().is_data_type())
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.type_ref, self.gen_env)
	}

	pub fn element_type(&self) -> TypeRef<'tu, 'g> {
		self.type_ref().template_specialization_args().into_iter()
			.find_map(|a| if let TemplateArg::Typename(type_ref) = a {
				Some(type_ref)
			} else {
				None
			}).expect("vector template argument list is empty")
	}

	pub fn dependent_types<D: DependentType<'tu>>(&self) -> Vec<D> {
		let element_type = self.element_type();
		let is_data_type = self.is_data_type(&element_type);
		let mut out = element_type.dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Type));
		out.reserve(2 + if is_data_type { 3 } else { 0 });
		if element_type.is_string() {
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type(&element_type.cpp_extern_return()).expect("Can't resolve string cpp_extern_return()"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
			// We need to generate return wrappers for std::vector<cv::String>, but it has several issues:
			// * we can't use get_canonical_type() because it resolves into compiler dependent inner type like
			//   std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>
			// * we can't generate both vector<cv::String> and vector<std::string> because for OpenCV 4
			//   cv::String is an typedef to std::string and it would lead to duplicate definition error
			// That's why we try to resolve both types and check if they are the same, if they are we only generate
			// vector<std::string> if not - both.
			let vec_cv_string = self.gen_env.resolve_type("std::vector<cv::String>").expect("Can't resolve std::vector<cv::String>");
			let vec_std_string = self.gen_env.resolve_type("std::vector<std::string>").expect("Can't resolve std::vector<std::string>");
			if vec_cv_string.get_canonical_type() == vec_std_string.get_canonical_type() {
				out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(
					TypeRef::new(vec_std_string, self.gen_env),
					self.gen_env,
					DefinitionLocation::Module,
				)));
			} else {
				out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(self.type_ref(), self.gen_env, DefinitionLocation::Module)));
			}
		} else {
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(element_type.canonical_clang(), self.gen_env, DefinitionLocation::Type)));
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(self.type_ref().canonical_clang(), self.gen_env, DefinitionLocation::Module)));
		}
		if is_data_type {
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputArray").expect("Can't resolve _InputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_OutputArray").expect("Can't resolve _OutputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
			out.push(D::from_return_type_wrapper(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputOutputArray").expect("Can't resolve _InputOutputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
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

	fn rust_leafname(&self) -> Cow<str> {
		let mut inner_typ = self.element_type();
		if let Some(inner) = inner_typ.as_pointer() { // fixme, implement references properly, use MatRef/Mut type
			inner_typ = inner;
		}
		format!("Vector::<{typ}>", typ=inner_typ.rust_full()).into()
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
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
