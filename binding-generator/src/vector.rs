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
	DefaultElement,
	DefinitionLocation,
	DependentTypeMode,
	Element,
	EntityElement,
	GeneratedElement,
	GeneratorEnv,
	StrExt,
	TypeRef,
};

#[derive(Clone)]
pub struct Vector<'tu, 'g> {
	type_ref: Type<'tu>,
	element: TypeRef<'tu, 'g>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> Vector<'tu, 'g> {
	pub fn new(type_ref: Type<'tu>, element: TypeRef<'tu, 'g>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { type_ref, element, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.type_ref, self.gen_env)
	}

	pub fn element_type(&self) -> TypeRef<'tu, 'g> {
		self.element.clone()
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.element.dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Type))
	}
}

impl<'tu> EntityElement<'tu> for Vector<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.type_ref.get_declaration().expect("Can't get declaration")
	}
}

impl Element for Vector<'_, '_> {
	fn is_ignored(&self) -> bool {
		// workaround for race when type with std::string gets generated first
		// we only want vector<cv::String> because it's more compatible across OpenCV versions
		DefaultElement::is_ignored(self) || self.element.is_ignored()
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
		DefaultElement::cpp_namespace(self)
	}

	fn cpp_localname(&self) -> Cow<str> {
		DefaultElement::cpp_localname(self)
	}

	fn rust_module(&self) -> Cow<str> {
		self.element.rust_module()
	}

	fn rust_leafname(&self) -> Cow<str> {
		format!("VectorOf{typ}", typ=self.element.rust_safe_id()).into()
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}

	fn rust_fullname(&self) -> Cow<str> {
		format!("types::{}", self.rust_localname()).into()
	}
}

impl GeneratedElement for Vector<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_common.tpl.rs").compile_interpolation()
		);

		static METHODS_BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_boxed.tpl.rs").compile_interpolation()
		);

		static METHODS_STRING_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_string.tpl.rs").compile_interpolation()
		);

		static METHODS_NON_BOXED_COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_non_boxed_common.tpl.rs").compile_interpolation()
		);

		static METHODS_SIMPLE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_simple.tpl.rs").compile_interpolation()
		);

		static METHODS_NON_BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_non_boxed.tpl.rs").compile_interpolation()
		);

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_methods_copy_non_bool.tpl.rs").compile_interpolation()
		);

		static INHERENT_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_inherent_copy_non_bool.tpl.rs").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/rust_input_output_array.tpl.rs").compile_interpolation()
		);

		let vec_type = self.type_ref();
		let mut inter_vars = hashmap! {
			"rust_local" => vec_type.rust_local(),
			"rust_extern" => vec_type.rust_extern(),
			"cpp_full" => vec_type.cpp_full(),
			"cpp_extern" => vec_type.cpp_extern(),
			"inner_cpp_full" => self.element.cpp_full(),
			"inner_cpp_extern" => self.element.cpp_extern(),
			"inner_rust_extern" => self.element.rust_extern(),
			"inner_rust_local" => self.element.rust_local(),
			"inner_canonical_rust_local" => self.element.canonical().rust_local().into_owned().into(),
			"inner_rust_full" => self.element.rust_full(),
			"rust_return_wrapper_type" => self.element.rust_extern_return_wrapper_full(),
			"cpp_return_wrapper_type" => self.element.cpp_extern_return_wrapper_full(),
		};
		let mut vector_methods = String::new();
		let mut inherent_methods = String::new();
		let mut impls = String::new();
		if self.element.is_by_ptr() {
			vector_methods += &METHODS_BOXED_TPL.interpolate(&inter_vars);
		} else if self.element.is_string() {
			vector_methods += &METHODS_STRING_TPL.interpolate(&inter_vars);
		} else {
			vector_methods += &METHODS_NON_BOXED_COMMON_TPL.interpolate(&inter_vars);
			if self.element.as_simple_class().is_some() {
				vector_methods += &METHODS_SIMPLE_TPL.interpolate(&inter_vars);
			} else {
				vector_methods += &METHODS_NON_BOXED_TPL.interpolate(&inter_vars);
			}
			if self.element.is_copy() && !self.element.is_bool() {
				vector_methods += &METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
				inherent_methods += &INHERENT_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
			}
		}
		if self.element.is_data_type() || self.element.as_vector().map_or(false, |v| v.element_type().is_data_type()) {
			impls += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
		}

		inter_vars.insert("vector_methods", vector_methods.into());
		inter_vars.insert("inherent_methods", inherent_methods.into());
		inter_vars.insert("impls", impls.into());
		COMMON_TPL.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_common.tpl.cpp").compile_interpolation()
		);

		static METHODS_BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_boxed.tpl.cpp").compile_interpolation()
		);

		static METHODS_STRING_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_string.tpl.cpp").compile_interpolation()
		);

		static METHODS_NON_BOXED_COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_non_boxed_common.tpl.cpp").compile_interpolation()
		);

		static METHODS_SIMPLE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_simple.tpl.cpp").compile_interpolation()
		);

		static METHODS_NON_BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_non_boxed.tpl.cpp").compile_interpolation()
		);

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_methods_copy_non_bool.tpl.cpp").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/vector/cpp_input_output_aray.tpl.cpp").compile_interpolation()
		);

		let vec_type = self.type_ref();
		let mut inter_vars = hashmap! {
			"rust_local" => vec_type.rust_local(),
			"cpp_full" => vec_type.cpp_full(),
			"cpp_extern" => vec_type.cpp_extern(),
			"inner_cpp_full" => self.element.cpp_full(),
			"inner_cpp_extern" => self.element.cpp_extern(),
			"cpp_return_wrapper_type" => self.element.cpp_extern_return_wrapper_full(),
			"call_arg" => self.element.cpp_arg_func_call("val").into(),
		};

		let mut exports = String::new();
		if self.element.is_by_ptr() {
			exports += &METHODS_BOXED_TPL.interpolate(&inter_vars);
		} else if self.element.is_string() {
			exports += &METHODS_STRING_TPL.interpolate(&inter_vars);
		} else {
			exports += &METHODS_NON_BOXED_COMMON_TPL.interpolate(&inter_vars);
			if self.element.as_simple_class().is_some() {
				exports += &METHODS_SIMPLE_TPL.interpolate(&inter_vars);
			} else {
				exports += &METHODS_NON_BOXED_TPL.interpolate(&inter_vars);
			}
			if self.element.is_copy() && !self.element.is_bool() {
				exports += &METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
			}
		}
		if self.element.is_data_type() || self.element.as_vector().map_or(false, |v| v.element_type().is_data_type()) {
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
