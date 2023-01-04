use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::type_ref::{Constness, ConstnessOverride, CppNameStyle, FishStyle, NameStyle};
use crate::{settings, CompiledInterpolation, FunctionTypeHint, StrExt, TypeRef, Vector};

use super::element::{DefaultRustNativeElement, RustElement};
use super::func_desc::{ClassDesc, CppFuncDesc, FuncDescCppCall, FuncDescKind};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for Vector<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, style)
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Vector<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/vector/rust.tpl.rs").compile_interpolation());

		static EXTERN_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_extern.tpl.rs").compile_interpolation());

		static ADD_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_copy_non_bool.tpl.rs").compile_interpolation());

		static ADD_NON_COPY_OR_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_non_copy_or_bool.tpl.rs").compile_interpolation());

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_input_output_array.tpl.rs").compile_interpolation());

		let vec_type = self.type_ref();
		if vec_type.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let element_type = self.element_type();
		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_name(NameStyle::ref_()),
			"inner_rust_full" => element_type.rust_name(NameStyle::ref_()),
		};

		if settings::PREVENT_VECTOR_TYPEDEF_GENERATION.contains(element_type.cpp_name(CppNameStyle::Reference).as_ref()) {
			inter_vars.insert("extern", "".into());
			inter_vars.insert("additional_methods", "".into());
			inter_vars.insert("impls", "".into());
		} else {
			inter_vars.insert("extern", EXTERN_TPL.interpolate(&inter_vars).into());

			let mut impls = String::new();
			let mut additional_methods = String::new();
			if element_type.is_copy() && !element_type.is_bool() {
				additional_methods += &ADD_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
			} else {
				inter_vars.insert(
					"clone",
					if element_type.is_clone() {
						"clone "
					} else {
						""
					}
					.into(),
				);
				additional_methods += &ADD_NON_COPY_OR_BOOL_TPL.interpolate(&inter_vars);
			}
			if self.is_data_type(&element_type) {
				impls += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
			}

			inter_vars.insert("additional_methods", additional_methods.into());
			inter_vars.insert("impls", impls.into());
		}

		RUST_TPL.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp.tpl.cpp").compile_interpolation());

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp_methods_copy_non_bool.tpl.cpp").compile_interpolation());

		let vec_type = self.type_ref();
		if vec_type.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let element_type = self.element_type();
		let element_is_bool = element_type.is_bool();
		let inner_cpp_full = element_type.cpp_name(CppNameStyle::Reference);
		let rust_localalias = self.rust_localalias();
		let vector_class_desc = ClassDesc {
			is_boxed: true,
			cpp_name_ref: vec_type.cpp_name(CppNameStyle::Reference).into_owned(),
		};
		let size_t = self.gen_env.resolve_typeref("size_t");
		let void = self.gen_env.resolve_typeref("void");
		let boolean = self.gen_env.resolve_typeref("bool");
		let mut methods = vec![
			method_new(&rust_localalias, &vector_class_desc, &vec_type),
			method_delete(&rust_localalias, &vector_class_desc, &void),
			method_len(&rust_localalias, &vector_class_desc, &size_t),
			method_is_empty(&rust_localalias, &vector_class_desc, &boolean),
			method_capacity(&rust_localalias, &vector_class_desc, &size_t),
			method_shrink_to_fit(&rust_localalias, &vector_class_desc, &void),
			method_reserve(&rust_localalias, &vector_class_desc, &void, &size_t),
			method_remove(&rust_localalias, &vector_class_desc, &void, &size_t),
			method_swap(&rust_localalias, &vector_class_desc, element_is_bool, &void, &size_t),
			method_clear(&rust_localalias, &vector_class_desc, &void),
			method_push(&rust_localalias, &vector_class_desc, &element_type, &void),
			method_insert(&rust_localalias, &vector_class_desc, &element_type, &void, &size_t),
			method_get(&rust_localalias, &vector_class_desc, &element_type, &size_t),
			method_set(&rust_localalias, &vector_class_desc, &element_type, &void, &size_t),
		];
		if element_type.is_copy() && !element_is_bool {
			methods.push(method_clone(&rust_localalias, &vector_class_desc, &vec_type));
		}
		if self.is_data_type(&element_type) {
			methods.push(method_input_array(
				&rust_localalias,
				&vector_class_desc,
				self.gen_env.resolve_typeref("cv::_InputArray"),
			));
			methods.push(method_output_array(
				&rust_localalias,
				&vector_class_desc,
				self.gen_env.resolve_typeref("cv::_OutputArray"),
			));
			methods.push(method_input_output_array(
				&rust_localalias,
				&vector_class_desc,
				self.gen_env.resolve_typeref("cv::_InputOutputArray"),
			));
		}
		let mut inter_vars = hashmap! {
			"rust_localalias" => rust_localalias.as_ref().into(),
			"cpp_full" => vec_type.cpp_name(CppNameStyle::Reference),
			"cpp_extern_return" => vec_type.cpp_extern_return(ConstnessOverride::No),
			"inner_cpp_full" => inner_cpp_full.as_ref().into(),
			"inner_cpp_extern_return" => element_type.cpp_extern_return(ConstnessOverride::No),
			"methods" => methods.join("").into(),
		};

		let mut exports = String::new();
		if element_type.is_copy() && !element_is_bool {
			exports += &METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
		}
		inter_vars.insert("exports", exports.into());

		COMMON_TPL.interpolate(&inter_vars)
	}
}

pub trait VectorExt {
	fn rust_element_module(&self) -> Cow<str>;
	fn rust_localalias(&self) -> Cow<str>;
}

impl VectorExt for Vector<'_, '_> {
	fn rust_element_module(&self) -> Cow<str> {
		self.element_type().rust_module().into_owned().into()
	}

	fn rust_localalias(&self) -> Cow<str> {
		format!("VectorOf{typ}", typ = self.element_type().rust_safe_id(true)).into()
	}
}

fn method_new(rust_localalias: &str, vector_class_desc: &ClassDesc, vec_type: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_new", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: vec_type.clone(),
		kind: FuncDescKind::Constructor(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Auto {
			name_decl: "<unsued>".into(),
			name_ref: "<unused>".into(),
		},
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_delete(rust_localalias: &str, vector_class_desc: &ClassDesc, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_delete", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("delete instance".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_len(rust_localalias: &str, vector_class_desc: &ClassDesc, size_t: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_len", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: size_t.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->size()".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_is_empty(rust_localalias: &str, vector_class_desc: &ClassDesc, boolean: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_is_empty", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: boolean.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->empty()".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_capacity(rust_localalias: &str, vector_class_desc: &ClassDesc, size_t: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_capacity", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: size_t.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->capacity()".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_shrink_to_fit(rust_localalias: &str, vector_class_desc: &ClassDesc, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_shrink_to_fit", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->shrink_to_fit()".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_reserve(rust_localalias: &str, vector_class_desc: &ClassDesc, void: &TypeRef, size_t: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_reserve", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->reserve(instance->size() + {{args}})".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![("additional".to_string(), size_t.clone())],
	}
	.gen_cpp()
}

fn method_remove(rust_localalias: &str, vector_class_desc: &ClassDesc, void: &TypeRef, size_t: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_remove", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->erase(instance->begin() + {{args}})".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![("index".to_string(), size_t.clone())],
	}
	.gen_cpp()
}

fn method_swap(
	rust_localalias: &str,
	vector_class_desc: &ClassDesc,
	element_is_bool: bool,
	void: &TypeRef,
	size_t: &TypeRef,
) -> String {
	// https://stackoverflow.com/questions/58660207/why-doesnt-stdswap-work-on-vectorbool-elements-under-clang-win
	let swap_func = if element_is_bool {
		"instance->swap"
	} else {
		"std::swap"
	};
	CppFuncDesc {
		extern_name: format!("cv_{}_swap", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual(
			format!("{swap_func}((*instance)[index1], (*instance)[index2])", swap_func = swap_func).compile_interpolation(),
		),
		debug: "".to_string(),
		arguments: vec![("index1".to_string(), size_t.clone()), ("index2".to_string(), size_t.clone())],
	}
	.gen_cpp()
}

fn method_clear(rust_localalias: &str, vector_class_desc: &ClassDesc, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_clear", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->clear()".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_push(rust_localalias: &str, vector_class_desc: &ClassDesc, element_type: &TypeRef, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_push", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->push_back({{args}})".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![("val".to_string(), element_type.clone())],
	}
	.gen_cpp()
}

fn method_insert(
	rust_localalias: &str,
	vector_class_desc: &ClassDesc,
	element_type: &TypeRef,
	void: &TypeRef,
	size_t: &TypeRef,
) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_insert", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("instance->insert(instance->begin() + {{args}})".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![
			("index".to_string(), size_t.clone()),
			("val".to_string(), element_type.clone()),
		],
	}
	.gen_cpp()
}

fn method_get(rust_localalias: &str, vector_class_desc: &ClassDesc, element_type: &TypeRef, size_t: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_get", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: false,
		return_type: element_type.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("(*instance)[{{args}}]".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![("index".to_string(), size_t.clone())],
	}
	.gen_cpp()
}

fn method_set(
	rust_localalias: &str,
	vector_class_desc: &ClassDesc,
	element_type: &TypeRef,
	void: &TypeRef,
	size_t: &TypeRef,
) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_set", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual(
			format!("(*instance)[index] = {}", element_type.cpp_arg_func_call("val")).compile_interpolation(),
		),
		debug: "".to_string(),
		arguments: vec![
			("index".to_string(), size_t.clone()),
			("val".to_string(), element_type.clone()),
		],
	}
	.gen_cpp()
}

fn method_clone(rust_localalias: &str, vector_class_desc: &ClassDesc, vec_type: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_clone", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: vec_type.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("{{ret_type}}(*instance)".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_input_array(rust_localalias: &str, vector_class_desc: &ClassDesc, input_array: TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_input_array", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: false,
		is_naked_return: false,
		return_type: input_array,
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("cv::_InputArray(*instance)".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_output_array(rust_localalias: &str, vector_class_desc: &ClassDesc, output_array: TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_output_array", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: false,
		is_naked_return: false,
		return_type: output_array.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("cv::_OutputArray(*instance)".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_input_output_array(rust_localalias: &str, vector_class_desc: &ClassDesc, input_output_array: TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_input_output_array", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: false,
		is_naked_return: false,
		return_type: input_output_array.clone(),
		kind: FuncDescKind::InstanceMethod(vector_class_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("cv::_InputOutputArray(*instance)".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}
