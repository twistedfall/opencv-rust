use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::settings::ArgOverride;
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::RustStringExt;
use crate::{settings, Class, CompiledInterpolation, Func, IteratorExt, StrExt, TypeRef, Vector};

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

		let rust_localalias = self.rust_localalias();
		let element_type = self.element_type();

		let mut inter_vars = HashMap::from([
			("rust_localalias", rust_localalias.clone()),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("inner_rust_full", element_type.rust_name(NameStyle::ref_())),
		]);

		let mut impls = String::new();
		// Generate only the basic type alias and as_raw* methods for char, the rest will be handled by the generated Vector<u8> and
		// Vector<i8> to handle the dualistic nature of C++ char on different platforms, see also `TypeRef::generated_types()`
		// in binding-generator/src/type_ref.rs
		if !element_type.base().is_char() {
			let vec_type_ref = self.type_ref();

			if vec_type_ref.constness().is_const() {
				// todo we should generate smth like VectorRef in this case
				return "".to_string();
			}
			let vector_class = vector_class(&vec_type_ref);

			let extern_new = method_new(vector_class.clone(), vec_type_ref.clone()).identifier();
			let extern_delete = FuncDesc::method_delete(vector_class.clone()).identifier();
			let extern_len = method_len(vector_class.clone()).identifier();
			let extern_is_empty = method_is_empty(vector_class.clone()).identifier();
			let extern_capacity = method_capacity(vector_class.clone()).identifier();
			let extern_shrink_to_fit = method_shrink_to_fit(vector_class.clone()).identifier();
			let extern_reserve = method_reserve(vector_class.clone()).identifier();
			let extern_remove = method_remove(vector_class.clone()).identifier();
			let extern_swap = method_swap(vector_class.clone(), element_type.is_bool()).identifier();
			let extern_clear = method_clear(vector_class.clone()).identifier();
			let extern_get = method_get(vector_class.clone(), element_type.clone()).identifier();
			let extern_set = method_set(vector_class.clone(), element_type.clone()).identifier();
			let extern_push = method_push(vector_class.clone(), element_type.clone()).identifier();
			let extern_insert = method_insert(vector_class.clone(), element_type.clone()).identifier();

			inter_vars.extend([
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
			} else {
				impls += &EXTERN_TPL.interpolate(&inter_vars);

				if element_type.is_copy() && !element_type.is_bool() {
					let extern_clone = method_clone(vector_class.clone(), vec_type_ref.clone()).identifier();
					let extern_data = method_data(vector_class.clone(), element_type.clone()).identifier();
					let extern_data_mut = method_data_mut(vector_class.clone(), element_type.clone()).identifier();
					let extern_from_slice = method_from_slice(vec_type_ref, element_type.clone()).identifier();
					inter_vars.extend([
						("extern_clone", extern_clone.into()),
						("extern_data", extern_data.into()),
						("extern_data_mut", extern_data_mut.into()),
						("extern_from_slice", extern_from_slice.into()),
					]);
					impls += &ADD_COPY_NON_BOOL_TPL.interpolate(&inter_vars);
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
					impls += &ADD_NON_COPY_OR_BOOL_TPL.interpolate(&inter_vars);
				}
				if element_type.is_element_data_type() {
					let input_array = method_input_array(vector_class.clone()).gen_rust(_opencv_version);
					let output_array = method_output_array(vector_class.clone()).gen_rust(_opencv_version);
					let input_output_array = method_input_output_array(vector_class).gen_rust(_opencv_version);
					inter_vars.extend([
						("input_array_impl", input_array.into()),
						("output_array_impl", output_array.into()),
						("input_output_array_impl", input_output_array.into()),
					]);
					impls += &INPUT_OUTPUT_ARRAY_TPL.interpolate(&inter_vars);
				}
			}
		}
		inter_vars.insert("impls", impls.into());

		RUST_TPL.interpolate(&inter_vars)
	}

	fn gen_rust_exports(&self) -> String {
		extern_functions(self).iter().map(Func::gen_rust_exports).join("")
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp.tpl.cpp").compile_interpolation());

		let vec_type_ref = self.type_ref();
		if vec_type_ref.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}

		COMMON_TPL.interpolate(&HashMap::from([(
			"methods",
			extern_functions(self).iter().map(Func::gen_cpp).join(""),
		)]))
	}
}

#[inline]
fn extern_functions<'tu, 'ge>(vec: &Vector<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let element_type = vec.element_type();
	let mut out = Vec::with_capacity(7);
	if !element_type.base().is_char() {
		let element_is_bool = element_type.is_bool();
		let vec_type_ref = vec.type_ref();
		let vector_class = vector_class(&vec_type_ref);
		out.extend([
			method_new(vector_class.clone(), vec_type_ref.clone()),
			FuncDesc::method_delete(vector_class.clone()),
			method_len(vector_class.clone()),
			method_is_empty(vector_class.clone()),
			method_capacity(vector_class.clone()),
			method_shrink_to_fit(vector_class.clone()),
			method_reserve(vector_class.clone()),
			method_remove(vector_class.clone()),
			method_swap(vector_class.clone(), element_is_bool),
			method_clear(vector_class.clone()),
			method_push(vector_class.clone(), element_type.clone()),
			method_insert(vector_class.clone(), element_type.clone()),
			method_get(vector_class.clone(), element_type.clone()),
			method_set(vector_class.clone(), element_type.clone()),
		]);
		if element_type.is_copy() && !element_is_bool {
			out.push(method_clone(vector_class.clone(), vec_type_ref.clone()));
			out.push(method_data(vector_class.clone(), element_type.clone()));
			out.push(method_data_mut(vector_class.clone(), element_type.clone()));
			out.push(method_from_slice(vec_type_ref, element_type.clone()));
		}
		if element_type.is_element_data_type() {
			out.push(method_input_array(vector_class.clone()));
			out.push(method_output_array(vector_class.clone()));
			out.push(method_input_output_array(vector_class));
		}
	}
	out
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

fn vector_class<'tu, 'ge>(vec_type_ref: &TypeRef) -> Class<'tu, 'ge> {
	Class::new_desc(ClassDesc::boxed(vec_type_ref.cpp_name(CppNameStyle::Reference), "core"))
}

fn method_new<'tu, 'ge>(vector_class: Class<'tu, 'ge>, vec_type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::Constructor(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::new",
		"core",
		vec![],
		FuncCppBody::Auto,
		FuncRustBody::Auto,
		vec_type_ref,
	))
}

fn method_len<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::len",
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->size()".into()),
		FuncRustBody::Auto,
		TypeRefDesc::size_t(),
	))
}

fn method_is_empty<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::isEmpty",
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->empty()".into()),
		FuncRustBody::Auto,
		TypeRefDesc::bool(),
	))
}

fn method_capacity<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::capacity",
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->capacity()".into()),
		FuncRustBody::Auto,
		TypeRefDesc::size_t(),
	))
}

fn method_shrink_to_fit<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::shrinkToFit",
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->shrink_to_fit()".into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_reserve<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::reserve",
		"core",
		vec![Field::new_desc(FieldDesc::new("additional", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("instance->reserve(instance->size() + {{args}})".into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_remove<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::remove",
		"core",
		vec![Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("instance->erase(instance->begin() + {{args}})".into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_swap<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_is_bool: bool) -> Func<'tu, 'ge> {
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
		"cv::swap",
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index1", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("index2", TypeRefDesc::size_t())),
		],
		FuncCppBody::ManualCall(format!("{swap_func}((*instance)[index1], (*instance)[index2])").into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_clear<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::clear",
		"core",
		vec![],
		FuncCppBody::Auto,
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_push<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::push",
		"core",
		vec![Field::new_desc(FieldDesc::new(
			"val",
			element_type.with_constness(Constness::Const),
		))],
		FuncCppBody::ManualCall("instance->push_back({{args}})".into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_insert<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::insert",
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("val", element_type.with_constness(Constness::Const))),
		],
		FuncCppBody::ManualCall("instance->insert(instance->begin() + {{args}})".into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_get<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleViaArg,
		"cv::get",
		"core",
		vec![Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
		FuncCppBody::ManualCall("(*instance)[{{args}}]".into()),
		FuncRustBody::Auto,
		element_type,
	))
}

fn method_set<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::set",
		"core",
		vec![
			Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
			Field::new_desc(FieldDesc::new("val", element_type.clone().with_constness(Constness::Const))),
		],
		FuncCppBody::ManualCall(format!("(*instance)[index] = {}", element_type.cpp_arg_func_call("val")).into()),
		FuncRustBody::Auto,
		TypeRefDesc::void(),
	))
}

fn method_clone<'tu, 'ge>(vector_class: Class<'tu, 'ge>, vec_type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::clone",
		"core",
		vec![],
		FuncCppBody::ManualCall("{{ret_type}}(*instance)".into()),
		FuncRustBody::Auto,
		vec_type_ref,
	))
}

fn method_data<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::data",
		"core",
		vec![],
		FuncCppBody::Auto,
		FuncRustBody::Auto,
		TypeRef::new_pointer(element_type.with_constness(Constness::Const))
			.with_type_hint(TypeRefTypeHint::ArgOverride(ArgOverride::CharPtrNotString)),
	))
}

fn method_data_mut<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::dataMut",
		"core",
		vec![],
		FuncCppBody::ManualCall("instance->data()".into()),
		FuncRustBody::Auto,
		TypeRef::new_pointer(element_type.with_constness(Constness::Mut))
			.with_type_hint(TypeRefTypeHint::ArgOverride(ArgOverride::CharPtrNotString)),
	))
}

fn method_from_slice<'tu, 'ge>(vec_type_ref: TypeRef<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::Function,
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::fromSlice",
		"core",
		vec![
			Field::new_desc(FieldDesc::new(
				"data",
				TypeRef::new_pointer(element_type.with_constness(Constness::Const)),
			)),
			Field::new_desc(FieldDesc::new("len", TypeRefDesc::size_t())),
		],
		FuncCppBody::ManualFull("return new {{ret_type}}(data, data + len);".into()),
		FuncRustBody::Auto,
		vec_type_ref,
	))
}

fn method_input_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::Fallible,
		"cv::inputArray",
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_InputArray(*instance)".into()),
		FuncRustBody::Auto,
		TypeRefDesc::cv_input_array(),
	))
}

fn method_output_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::Fallible,
		"cv::outputArray",
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_OutputArray(*instance)".into()),
		FuncRustBody::Auto,
		TypeRefDesc::cv_output_array(),
	))
}

fn method_input_output_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::Fallible,
		"cv::inputOutputArray",
		"core",
		vec![],
		FuncCppBody::ManualCall("cv::_InputOutputArray(*instance)".into()),
		FuncRustBody::Auto,
		TypeRefDesc::cv_input_output_array(),
	))
}
