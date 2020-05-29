use std::{
	borrow::Cow,
	fmt,
};

use clang::{
	Entity,
	Type,
};
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Constness,
	DefaultElement,
	DefinitionLocation,
	DependentTypeMode,
	Element,
	EntityElement,
	GeneratedElement,
	GeneratorEnv,
	ReturnTypeWrapper,
	settings,
	StrExt,
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

	fn is_data_type(&self, type_ref: &TypeRef) -> bool {
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

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		let element_type = self.element_type();
		let is_data_type = self.is_data_type(&element_type);
		let mut out = element_type.dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Type));
		out.reserve(2 + if is_data_type { 3 } else { 0 });
		if element_type.is_string() {
			out.push(Box::new(ReturnTypeWrapper::new(
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
				out.push(Box::new(ReturnTypeWrapper::new(
					TypeRef::new(vec_std_string, self.gen_env),
					self.gen_env,
					DefinitionLocation::Module,
				)));
			} else {
				out.push(Box::new(ReturnTypeWrapper::new(self.type_ref(), self.gen_env, DefinitionLocation::Module)));
			}
		} else {
			out.push(Box::new(ReturnTypeWrapper::new(element_type.canonical_clang(), self.gen_env, DefinitionLocation::Type)));
			out.push(Box::new(ReturnTypeWrapper::new(self.type_ref().canonical_clang(), self.gen_env, DefinitionLocation::Module)));
		}
		if is_data_type {
			out.push(Box::new(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputArray").expect("Can't resolve _InputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
			out.push(Box::new(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_OutputArray").expect("Can't resolve _OutputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
			out.push(Box::new(ReturnTypeWrapper::new(
				TypeRef::new(self.gen_env.resolve_type("cv::_InputOutputArray").expect("Can't resolve _InputOutputArray"), self.gen_env),
				self.gen_env,
				DefinitionLocation::Custom(element_type.rust_module().into_owned()),
			)));
		}
		out
	}

	pub fn rust_localalias(&self) -> Cow<str> {
		format!("VectorOf{typ}", typ = self.element_type().rust_safe_id()).into()
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

impl GeneratedElement for Vector<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TYPE_ALIAS_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_type_alias.tpl.rs").compile_interpolation()
		);

		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust.tpl.rs").compile_interpolation()
		);

		static ADD_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_copy_non_bool.tpl.rs").compile_interpolation()
		);

		static ADD_NON_COPY_OR_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_non_copy_or_bool.tpl.rs").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_input_output_array.tpl.rs").compile_interpolation()
		);

		let vec_type = self.type_ref();
		let element_type = self.element_type();
		let inner_rust_full_arg = if element_type.is_string() {
			"&'i str".into()
		} else {
			element_type.rust_full()
		};
		let mut pre_call = element_type.rust_arg_pre_call("val", false);
		if !pre_call.is_empty() {
			pre_call.push_str(";");
		}
		let mut pre_call_infallible = element_type.rust_arg_pre_call("val", true);
		if !pre_call_infallible.is_empty() {
			pre_call_infallible.push_str(";");
		}
		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_fullname(),
			"rust_extern_const" => vec_type.rust_extern_with_const(Constness::Const),
			"rust_extern_mut" => vec_type.rust_extern_with_const(Constness::Mut),
			"inner_rust_func_decl" => element_type.rust_arg_func_decl("val").into(),
			"inner_rust_extern_func_decl" => element_type.rust_extern_arg_func_decl("val", Constness::Auto).into(),
			"inner_rust_func_call" => element_type.rust_arg_func_call("val", false).into(),
			"inner_rust_full" => element_type.rust_full(),
			"inner_rust_full_arg" => inner_rust_full_arg.into(),
			"pre_call" => pre_call.into(),
			"pre_call_infallible" => pre_call_infallible.into(),
			"inner_rust_extern_return" => element_type.rust_extern_return(),
			"inner_rust_extern_return_wrapper" => element_type.rust_extern_return_wrapper_full(),
			"ret_map" => element_type.rust_return_map(false),
		};
		if element_type.as_typedef().is_some()
			&& !element_type.is_data_type()
			&& !element_type.is_string()
			&& !settings::FORCE_VECTOR_TYPEDEF_GENERATION.contains(element_type.cpp_full().as_ref())
		{
			&TYPE_ALIAS_TPL
		} else {
			inter_vars.insert("alias", TYPE_ALIAS_TPL.interpolate(&inter_vars).into());
			let mut impls = String::new();
			let mut additional_methods = String::new();
			if element_type.is_copy() && !element_type.is_bool() {
				additional_methods += &ADD_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
			} else {
				additional_methods += &ADD_NON_COPY_OR_BOOL_TPL.interpolate(&inter_vars);
			}
			if self.is_data_type(&element_type) {
				impls += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
			}

			inter_vars.insert("additional_methods", additional_methods.into());
			inter_vars.insert("impls", impls.into());
			&COMMON_TPL
		}.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp.tpl.cpp").compile_interpolation()
		);

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_copy_non_bool.tpl.cpp").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_input_output_array.tpl.cpp").compile_interpolation()
		);

		let vec_type = self.type_ref();
		let element_type = self.element_type();
		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"cpp_full" => vec_type.cpp_full(),
			"cpp_extern_return" => vec_type.cpp_extern_return(),
			"inner_cpp_full" => element_type.cpp_full(),
			"inner_cpp_func_decl" => element_type.cpp_arg_func_decl("val").into(),
			"inner_cpp_func_call" => element_type.cpp_arg_func_call("val").into(),
			"inner_cpp_extern_return" => element_type.cpp_extern_return(),
			"inner_cpp_extern_return_wrapper" => element_type.cpp_extern_return_wrapper_full(),
		};

		let mut prefix = Cow::Borrowed("");
		let mut suffix = Cow::Borrowed("");
		if element_type.is_by_ptr() {
			prefix = format!("new {inner_cpp_full}(", inner_cpp_full=element_type.cpp_full()).into();
			suffix = ")".into();
		} else if element_type.is_string() {
			prefix = "ocvrs_create_string(".into();
			suffix = ".c_str())".into();
		}
		inter_vars.insert("prefix", prefix);
		inter_vars.insert("suffix", suffix);
		let mut exports = String::new();
		if element_type.is_copy() && !element_type.is_bool() {
			exports += &METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
		}
		if self.is_data_type(&element_type) {
			exports += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
		}
		inter_vars.insert("exports", exports.into());

		COMMON_TPL.interpolate(&inter_vars)
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
