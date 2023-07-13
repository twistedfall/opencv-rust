use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle, TypeRefDesc};
use crate::writer::rust_native::RustStringExt;
use crate::{settings, Class, CompiledInterpolation, Func, StrExt, TypeRef, Vector};

use super::element::RustElement;
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for Vector<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		format!(
			"{}::{}",
			self.rust_module_reference(),
			self.rust_leafname(style.turbo_fish_style())
		)
		.as_str()
		.rust_name_from_fullname(style)
		.into_owned()
		.into()
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

	fn rendered_doc_comment_with_prefix(&self, _prefix: &str, _opencv_version: &str) -> String {
		"".to_string()
	}
}

impl RustNativeGeneratedElement for Vector<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module(), self.rust_localalias())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static RUST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/vector/rust.tpl.rs").compile_interpolation());

		static EXTERN_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_extern.tpl.rs").compile_interpolation());

		static ADD_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_copy_non_bool.tpl.rs").compile_interpolation());

		static ADD_NON_COPY_OR_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_non_copy_or_bool.tpl.rs").compile_interpolation());

		static INPUT_OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_input_output_array.tpl.rs").compile_interpolation());

		let vec_type_ref = self.type_ref();

		if vec_type_ref.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let rust_localalias = self.rust_localalias();
		let element_type = self.element_type();
		let vector_class = vector_class(&vec_type_ref, &rust_localalias);

		let extern_new = method_new(&rust_localalias, vector_class.clone(), vec_type_ref.clone()).identifier();
		let extern_delete = FuncDesc::method_delete(&rust_localalias, vector_class.clone()).identifier();
		let extern_len = method_len(&rust_localalias, vector_class.clone()).identifier();
		let extern_is_empty = method_is_empty(&rust_localalias, vector_class.clone()).identifier();
		let extern_capacity = method_capacity(&rust_localalias, vector_class.clone()).identifier();
		let extern_shrink_to_fit = method_shrink_to_fit(&rust_localalias, vector_class.clone()).identifier();
		let extern_reserve = method_reserve(&rust_localalias, vector_class.clone()).identifier();
		let extern_remove = method_remove(&rust_localalias, vector_class.clone()).identifier();
		let extern_swap = method_swap(&rust_localalias, vector_class.clone(), element_type.is_bool()).identifier();
		let extern_clear = method_clear(&rust_localalias, vector_class.clone()).identifier();
		let extern_get = method_get(&rust_localalias, vector_class.clone(), element_type.clone()).identifier();
		let extern_set = method_set(&rust_localalias, vector_class.clone(), element_type.clone()).identifier();
		let extern_push = method_push(&rust_localalias, vector_class.clone(), element_type.clone()).identifier();
		let extern_insert = method_insert(&rust_localalias, vector_class.clone(), element_type.clone()).identifier();

		let mut inter_vars = HashMap::from([
			("rust_localalias", rust_localalias.clone()),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("inner_rust_full", element_type.rust_name(NameStyle::ref_())),
			("extern_new", extern_new.into()),
			("extern_delete", extern_delete.into()),
			("extern_len", extern_len.into()),
			("extern_is_empty", extern_is_empty.into()),
			("extern_capacity", extern_capacity.into()),
			("extern_shrink_to_fit", extern_shrink_to_fit.into()),
			("extern_reserve", extern_reserve.into()),
			("extern_remove", extern_remove.into()),
			("extern_swap", extern_swap.into()),
			("extern_clear", extern_clear.into()),
			("extern_get", extern_get.into()),
			("extern_set", extern_set.into()),
			("extern_push", extern_push.into()),
			("extern_insert", extern_insert.into()),
		]);

		if settings::PREVENT_VECTOR_TYPEDEF_GENERATION.contains(element_type.cpp_name(CppNameStyle::Reference).as_ref()) {
			inter_vars.insert("extern", "".into());
			inter_vars.insert("additional_methods", "".into());
			inter_vars.insert("impls", "".into());
		} else {
			inter_vars.insert("extern", EXTERN_TPL.interpolate(&inter_vars).into());

			let mut impls = String::new();
			let mut additional_methods = String::new();
			if element_type.is_copy() && !element_type.is_bool() {
				let extern_clone = method_clone(&rust_localalias, vector_class.clone(), vec_type_ref.clone()).identifier();
				let extern_from_slice = method_from_slice(&rust_localalias, vec_type_ref, element_type.clone()).identifier();
				inter_vars.extend([
					("extern_clone", extern_clone.into()),
					("extern_from_slice", extern_from_slice.into()),
				]);
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
			if element_type.is_element_data_type() {
				let input_array = method_input_array(&rust_localalias, vector_class.clone()).gen_rust(opencv_version);
				let output_array = method_output_array(&rust_localalias, vector_class.clone()).gen_rust(opencv_version);
				let input_output_array = method_input_output_array(&rust_localalias, vector_class).gen_rust(opencv_version);
				inter_vars.extend([
					("input_array_impl", input_array.into()),
					("output_array_impl", output_array.into()),
					("input_output_array_impl", input_output_array.into()),
				]);
				impls += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
			}

			inter_vars.insert("additional_methods", additional_methods.into());
			inter_vars.insert("impls", impls.into());
		}

		RUST_TPL.interpolate(&inter_vars)
	}

	fn gen_rust_exports(&self) -> String {
		let vec_type_ref = self.type_ref();
		let rust_localalias = self.rust_localalias();
		let element_type = self.element_type();
		let vector_class = vector_class(&vec_type_ref, &rust_localalias);

		let mut out = String::new();
		if element_type.is_element_data_type() {
			out.push_str(&method_input_array(&rust_localalias, vector_class.clone()).gen_rust_exports());
			out.push_str(&method_output_array(&rust_localalias, vector_class.clone()).gen_rust_exports());
			out.push_str(&method_input_output_array(&rust_localalias, vector_class.clone()).gen_rust_exports());
		}
		if element_type.is_copy() && !element_type.is_bool() {
			out.push_str(&method_clone(&rust_localalias, vector_class, vec_type_ref.clone()).gen_rust_exports());
			out.push_str(&method_from_slice(&rust_localalias, vec_type_ref, element_type.clone()).gen_rust_exports());
		}
		out
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp.tpl.cpp").compile_interpolation());

		static METHODS_COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp_methods_copy_non_bool.tpl.cpp").compile_interpolation());

		let vec_type_ref = self.type_ref();
		if vec_type_ref.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		let element_type = self.element_type();
		let element_is_bool = element_type.is_bool();
		let inner_cpp_full = element_type.cpp_name(CppNameStyle::Reference);
		let rust_localalias = self.rust_localalias();
		let vector_class = vector_class(&vec_type_ref, &rust_localalias);
		let mut methods = vec![
			method_new(&rust_localalias, vector_class.clone(), vec_type_ref.clone()).gen_cpp(),
			FuncDesc::method_delete(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_len(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_is_empty(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_capacity(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_shrink_to_fit(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_reserve(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_remove(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_swap(&rust_localalias, vector_class.clone(), element_is_bool).gen_cpp(),
			method_clear(&rust_localalias, vector_class.clone()).gen_cpp(),
			method_push(&rust_localalias, vector_class.clone(), element_type.clone()).gen_cpp(),
			method_insert(&rust_localalias, vector_class.clone(), element_type.clone()).gen_cpp(),
			method_get(&rust_localalias, vector_class.clone(), element_type.clone()).gen_cpp(),
			method_set(&rust_localalias, vector_class.clone(), element_type.clone()).gen_cpp(),
		];
		if element_type.is_copy() && !element_is_bool {
			methods.push(method_clone(&rust_localalias, vector_class.clone(), vec_type_ref.clone()).gen_cpp());
		}
		if element_type.is_element_data_type() {
			methods.push(method_input_array(&rust_localalias, vector_class.clone()).gen_cpp());
			methods.push(method_output_array(&rust_localalias, vector_class.clone()).gen_cpp());
			methods.push(method_input_output_array(&rust_localalias, vector_class).gen_cpp());
		}
		let mut inter_vars = HashMap::from([
			("rust_localalias", rust_localalias.as_ref().into()),
			("cpp_full", vec_type_ref.cpp_name(CppNameStyle::Reference)),
			("cpp_extern_return", vec_type_ref.cpp_extern_return()),
			("inner_cpp_full", inner_cpp_full.as_ref().into()),
			("inner_cpp_extern_return", element_type.cpp_extern_return()),
		]);

		if element_type.is_copy() && !element_is_bool {
			methods.push(METHODS_COPY_NON_BOOL_TPL.interpolate(&inter_vars));
			methods.push(method_from_slice(&rust_localalias, vec_type_ref.clone(), element_type.clone()).gen_cpp());
		}
		inter_vars.insert("methods", methods.join("").into());

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

fn vector_class<'tu, 'ge>(vec_type_ref: &TypeRef, rust_localalias: &str) -> Class<'tu, 'ge> {
	Class::new_desc(ClassDesc::boxed(
		vec_type_ref.cpp_name(CppNameStyle::Reference),
		format!("core::{rust_localalias}"),
	))
}

fn method_new<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>, vec_type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::Constructor(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::new"),
		"core",
		vec![],
		FuncCppBody::Auto,
		vec_type_ref,
	))
}

fn method_len<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::len"),
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->size()".into()),
		TypeRefDesc::size_t(),
	))
}

fn method_is_empty<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::isEmpty"),
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->empty()".into()),
		TypeRefDesc::bool(),
	))
}

fn method_capacity<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::capacity"),
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->capacity()".into()),
		TypeRefDesc::size_t(),
	))
}

fn method_shrink_to_fit<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::shrinkToFit"),
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->shrink_to_fit()".into()),
		TypeRefDesc::void(),
	))
}

fn method_reserve<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::reserve"),
		"core",
		vec![Field::new_desc(FieldDesc::new("additional", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("instance->reserve(instance->size() + {{args}})".into()),
		TypeRefDesc::void(),
	))
}

fn method_remove<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::remove"),
		"core",
		vec![Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("instance->erase(instance->begin() + {{args}})".into()),
		TypeRefDesc::void(),
	))
}

fn method_swap<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>, element_is_bool: bool) -> Func<'tu, 'ge> {
	// https://stackoverflow.com/questions/58660207/why-doesnt-stdswap-work-on-vectorbool-elements-under-clang-win
	let swap_func = if element_is_bool {
		"instance->swap"
	} else {
		"std::swap"
	};
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::swap"),
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index1", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("index2", TypeRefDesc::size_t())),
		],
		FuncCppBody::ManualCall(format!("{swap_func}((*instance)[index1], (*instance)[index2])").into()),
		TypeRefDesc::void(),
	))
}

fn method_clear<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::clear"),
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->clear()".into()),
		TypeRefDesc::void(),
	))
}

fn method_push<'tu, 'ge>(
	rust_localalias: &str,
	vector_class: Class<'tu, 'ge>,
	element_type: TypeRef<'tu, 'ge>,
) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::push"),
		"core",
		vec![Field::new_desc(FieldDesc::new("val", element_type))],
		FuncCppBody::ManualCall("instance->push_back({{args}})".into()),
		TypeRefDesc::void(),
	))
}

fn method_insert<'tu, 'ge>(
	rust_localalias: &str,
	vector_class: Class<'tu, 'ge>,
	element_type: TypeRef<'tu, 'ge>,
) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::insert"),
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("val", element_type)),
		],
		FuncCppBody::ManualCall("instance->insert(instance->begin() + {{args}})".into()),
		TypeRefDesc::void(),
	))
}

fn method_get<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleViaArg,
		format!("cv::{rust_localalias}::get"),
		"core",
		vec![Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("(*instance)[{{args}}]".into()),
		element_type,
	))
}

fn method_set<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::set"),
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("val", element_type.clone())),
		],
		FuncCppBody::ManualCall(format!("(*instance)[index] = {}", element_type.cpp_arg_func_call("val")).into()),
		TypeRefDesc::void(),
	))
}

fn method_clone<'tu, 'ge>(
	rust_localalias: &str,
	vector_class: Class<'tu, 'ge>,
	vec_type_ref: TypeRef<'tu, 'ge>,
) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::clone"),
		"core",
		vec![],
		FuncCppBody::ManualCall("{{ret_type}}(*instance)".into()),
		vec_type_ref,
	))
}

fn method_from_slice<'tu, 'ge>(
	rust_localalias: &str,
	vec_type_ref: TypeRef<'tu, 'ge>,
	element_type: TypeRef<'tu, 'ge>,
) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::Function,
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::fromSlice"),
		"core",
		vec![
			Field::new_desc(FieldDesc::new(
				"data",
				TypeRef::new_pointer(element_type.with_constness(Constness::Const)),
			)),
			Field::new_desc(FieldDesc::new("len", TypeRefDesc::size_t())),
		],
		FuncCppBody::ManualFull("return new {{ret_type}}(data, data + len);".into()),
		vec_type_ref,
	))
}

fn method_input_array<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::Fallible,
		format!("cv::{rust_localalias}::inputArray"),
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_InputArray(*instance)".into()),
		TypeRefDesc::input_array(),
	))
}

fn method_output_array<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::Fallible,
		format!("cv::{rust_localalias}::outputArray"),
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_OutputArray(*instance)".into()),
		TypeRefDesc::output_array(),
	))
}

fn method_input_output_array<'tu, 'ge>(rust_localalias: &str, vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::Fallible,
		format!("cv::{rust_localalias}::inputOutputArray"),
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_InputOutputArray(*instance)".into()),
		TypeRefDesc::input_output_array(),
	))
}
