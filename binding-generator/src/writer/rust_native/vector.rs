use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Constness,
	ConstnessOverride,
	Element,
	settings,
	StrExt,
	type_ref::FishStyle,
	Vector,
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for Vector<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TYPE_ALIAS_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/rust_type_alias.tpl.rs").compile_interpolation()
		);

		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/rust.tpl.rs").compile_interpolation()
		);

		static ADD_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/rust_copy_non_bool.tpl.rs").compile_interpolation()
		);

		static ADD_NON_COPY_OR_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/rust_non_copy_or_bool.tpl.rs").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/rust_input_output_array.tpl.rs").compile_interpolation()
		);

		let vec_type = self.type_ref();
		if vec_type.constness().is_const() { // todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let element_type = self.element_type();
		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_fullname(FishStyle::No),
			"rust_extern_const" => vec_type.rust_extern(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => vec_type.rust_extern(ConstnessOverride::Yes(Constness::Mut)),
			"inner_rust_full" => element_type.rust_full(),
			"inner_rust_extern_return" => element_type.rust_extern_return(),
		};
		if element_type.as_typedef().is_some()
			&& !element_type.is_data_type()
			&& !element_type.as_string().is_some()
			&& !settings::PREVENT_VECTOR_TYPEDEF_GENERATION.contains(element_type.cpp_full().as_ref())
		{
			&TYPE_ALIAS_TPL
		} else {
			inter_vars.insert("alias", TYPE_ALIAS_TPL.interpolate(&inter_vars).into());
			let mut impls = String::new();
			let mut additional_methods = String::new();
			if element_type.is_copy() && !element_type.is_bool() {
				additional_methods += &ADD_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
			} else {
				inter_vars.insert("clone", if vec_type.is_clone() {
					"clone "
				} else {
					""
				}.into());
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
			|| include_str!("tpl/vector/cpp.tpl.cpp").compile_interpolation()
		);

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/cpp_methods_copy_non_bool.tpl.cpp").compile_interpolation()
		);

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/vector/cpp_input_output_array.tpl.cpp").compile_interpolation()
		);

		let vec_type = self.type_ref();
		if vec_type.constness().is_const() { // todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let element_type = self.element_type();
		let element_is_bool = element_type.is_bool();
		// https://stackoverflow.com/questions/58660207/why-doesnt-stdswap-work-on-vectorbool-elements-under-clang-win
		let swap_func = if element_is_bool {
			"instance->swap"
		} else {
			"std::swap"
		};
		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"cpp_full" => vec_type.cpp_full(),
			"cpp_extern_return" => vec_type.cpp_extern_return(ConstnessOverride::No),
			"inner_cpp_full" => element_type.cpp_full(),
			"inner_cpp_func_decl" => element_type.cpp_arg_func_decl("val").into(),
			"inner_cpp_func_call" => element_type.cpp_arg_func_call("val"),
			"inner_cpp_extern_return" => element_type.cpp_extern_return(ConstnessOverride::No),
			"inner_cpp_extern_return_wrapper" => element_type.cpp_extern_return_wrapper_full(ConstnessOverride::No),
			"swap_func" => swap_func.into(),
		};

		let mut prefix = Cow::Borrowed("");
		let mut suffix = Cow::Borrowed("");
		if element_type.is_by_ptr() {
			prefix = format!("new {inner_cpp_full}(", inner_cpp_full = element_type.cpp_full()).into();
			suffix = ")".into();
		} else if element_type.as_string().is_some() {
			prefix = "ocvrs_create_string(".into();
			suffix = ".c_str())".into();
		}
		inter_vars.insert("prefix", prefix);
		inter_vars.insert("suffix", suffix);
		let mut exports = String::new();
		if element_type.is_copy() && !element_is_bool {
			exports += &METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
		}
		if self.is_data_type(&element_type) {
			exports += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
		}
		inter_vars.insert("exports", exports.into());

		COMMON_TPL.interpolate(&inter_vars)
	}
}
