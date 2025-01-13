use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::element::RustElement;
use super::type_ref::{Lifetime, TypeRefExt};
use super::RustNativeGeneratedElement;
use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
use crate::settings::{ARG_OVERRIDE_SELF, CFG_ATTR_ONLY_OPENCV_5};
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle, TypeRefDesc, TypeRefTypeHint};
use crate::{settings, Class, CompiledInterpolation, Func, IteratorExt, StrExt, StringExt, TypeRef, Vector};

impl RustElement for Vector<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		let decl_name = self.rust_leafname(style.turbo_fish_style());
		match style {
			NameStyle::Declaration => decl_name,
			NameStyle::Reference(_) => {
				let mut out = self.rust_module_reference().into_owned();
				out.extend_sep("::", &decl_name);
				out.into()
			}
		}
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		let mut inner_typ = self.element_type();
		if let Some(inner) = inner_typ.kind().as_pointer() {
			// fixme, implement references properly, use VectorRef/Mut type
			inner_typ = inner.into_owned();
		}
		format!(
			"Vector{fish}<{typ}>",
			fish = fish_style.rust_qual(),
			typ = inner_typ.rust_name(NameStyle::ref_()),
		)
		.into()
	}

	fn rendered_doc_comment(&self, _comment_marker: &str, _opencv_version: &str) -> String {
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

		static COPY_NON_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_copy_non_bool.tpl.rs").compile_interpolation());

		static NON_COPY_OR_BOOL_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_non_copy_or_bool.tpl.rs").compile_interpolation());

		static BOXED_REF_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_boxed_ref.tpl.rs").compile_interpolation());

		static INPUT_ARRAY_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_input_array.tpl.rs").compile_interpolation());

		static OUTPUT_ARRAY_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/rust_output_array.tpl.rs").compile_interpolation());

		let vec_type_ref = self.type_ref();
		if vec_type_ref.constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}

		let rust_localalias = self.rust_localalias();
		let element_type = self.element_type();

		let mut inter_vars = HashMap::from([
			("rust_localalias", rust_localalias.clone()),
			("rust_as_raw_const", vec_type_ref.rust_as_raw_name(Constness::Const).into()),
			("rust_as_raw_mut", vec_type_ref.rust_as_raw_name(Constness::Mut).into()),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("inner_rust_full", element_type.rust_name(NameStyle::ref_())),
		]);

		let mut impls = String::with_capacity(1024);
		// Generate only the basic type alias and as_raw* methods for char, the rest will be handled by the generated Vector<u8> and
		// Vector<i8> to handle the dualistic nature of C++ char on different platforms, see also `TypeRef::generated_types()`
		// in binding-generator/src/type_ref.rs
		if !element_type.base().kind().is_char() {
			let element_kind = element_type.kind();
			let element_is_bool = element_kind.is_bool();
			let vector_class = vector_class(&vec_type_ref);

			let extern_new = method_new(vector_class.clone(), vec_type_ref.clone()).identifier();
			let extern_delete = FuncDesc::method_delete(vector_class.clone()).identifier();
			let extern_len = method_len(vector_class.clone()).identifier();
			let extern_is_empty = method_is_empty(vector_class.clone()).identifier();
			let extern_capacity = method_capacity(vector_class.clone()).identifier();
			let extern_shrink_to_fit = method_shrink_to_fit(vector_class.clone()).identifier();
			let extern_reserve = method_reserve(vector_class.clone()).identifier();
			let extern_remove = method_remove(vector_class.clone()).identifier();
			let extern_swap = method_swap(vector_class.clone(), element_is_bool).identifier();
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
				("cfg_attr", "".into()),
			]);

			if settings::PREVENT_VECTOR_TYPEDEF_GENERATION.contains(element_type.cpp_name(CppNameStyle::Reference).as_ref()) {
				inter_vars.extend([("extern", "".into()), ("additional_methods", "".into())]);
			} else {
				EXTERN_TPL.interpolate_into(&mut impls, &inter_vars);

				if element_kind.is_copy(element_type.type_hint()) && !element_is_bool {
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
					COPY_NON_BOOL_TPL.interpolate_into(&mut impls, &inter_vars);
				} else {
					inter_vars.insert(
						"clone",
						if element_kind.is_clone(element_type.type_hint()) {
							"clone "
						} else {
							""
						}
						.into(),
					);
					NON_COPY_OR_BOOL_TPL.interpolate_into(&mut impls, &inter_vars);
				}
				let is_data_type_or_vec_of_data_type_opencv_4 = is_data_type_or_vec_of_data_type(&element_type, false);
				let is_data_type_or_vec_of_data_type_opencv_5 = is_data_type_or_vec_of_data_type(&element_type, true);
				if element_is_bool || is_data_type_or_vec_of_data_type_opencv_4 {
					let input_array = method_input_array(vector_class.clone(), None).gen_rust(opencv_version);
					inter_vars.insert("input_array_impl", input_array.into());
					INPUT_ARRAY_TPL.interpolate_into(&mut impls, &inter_vars);
					if !element_is_bool {
						let output_array = method_output_array(vector_class.clone(), None).gen_rust(opencv_version);
						let input_output_array = method_input_output_array(vector_class.clone(), None).gen_rust(opencv_version);
						inter_vars.extend([
							("output_array_impl", output_array.into()),
							("input_output_array_impl", input_output_array.into()),
						]);
						OUTPUT_ARRAY_TPL.interpolate_into(&mut impls, &inter_vars);
					}
				}
				// string is a custom type in 3.4 so exclude it explicitly
				if element_kind
					.as_class()
					.is_some_and(|cls| cls.kind().is_boxed() && cls.string_type().is_none())
				{
					BOXED_REF_TPL.interpolate_into(&mut impls, &inter_vars);
					inter_vars.insert(
						"inner_rust_full",
						format!("BoxedRef<'t, {}>", inter_vars["inner_rust_full"]).into(),
					);
					EXTERN_TPL.interpolate_into(&mut impls, &inter_vars);
					if is_data_type_or_vec_of_data_type_opencv_4 || is_data_type_or_vec_of_data_type_opencv_5 {
						inter_vars.insert("rust_full", format!("BoxedRef<'_, {}>", rust_localalias).into());
						INPUT_ARRAY_TPL.interpolate_into(&mut impls, &inter_vars);
					}
				}
				if !element_is_bool && is_data_type_or_vec_of_data_type_opencv_5 {
					let input_array = method_input_array(vector_class.clone(), None).gen_rust(opencv_version);
					inter_vars.insert("cfg_attr", format!("#[cfg({})]", CFG_ATTR_ONLY_OPENCV_5.0).into());
					inter_vars.insert("input_array_impl", input_array.into());
					INPUT_ARRAY_TPL.interpolate_into(&mut impls, &inter_vars);
					let output_array = method_output_array(vector_class.clone(), None).gen_rust(opencv_version);
					let input_output_array = method_input_output_array(vector_class, None).gen_rust(opencv_version);
					inter_vars.extend([
						("output_array_impl", output_array.into()),
						("input_output_array_impl", input_output_array.into()),
					]);
					OUTPUT_ARRAY_TPL.interpolate_into(&mut impls, &inter_vars);
				}
			}
		}
		inter_vars.insert("impls", impls.into());

		RUST_TPL.interpolate(&inter_vars)
	}

	fn gen_rust_externs(&self) -> String {
		if self.type_ref().constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}
		extern_functions(self).iter().map(Func::gen_rust_externs).join("")
	}

	fn gen_cpp(&self) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/vector/cpp.tpl.cpp").compile_interpolation());

		if self.type_ref().constness().is_const() {
			// todo we should generate smth like VectorRef in this case
			return "".to_string();
		}

		COMMON_TPL.interpolate(&HashMap::from([(
			"methods",
			extern_functions(self).iter().map(Func::gen_cpp).join(""),
		)]))
	}
}

fn extern_functions<'tu, 'ge>(vec: &Vector<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let element_type = vec.element_type();
	let mut out = Vec::with_capacity(7);
	if !element_type.base().kind().is_char() {
		let element_kind = element_type.kind();
		let element_is_bool = element_kind.is_bool();
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
		if element_kind.is_copy(element_type.type_hint()) && !element_is_bool {
			out.push(method_clone(vector_class.clone(), vec_type_ref.clone()));
			out.push(method_data(vector_class.clone(), element_type.clone()));
			out.push(method_data_mut(vector_class.clone(), element_type.clone()));
			out.push(method_from_slice(vec_type_ref, element_type.clone()));
		}
		if element_is_bool || is_data_type_or_vec_of_data_type(&element_type, false) {
			out.push(method_input_array(vector_class.clone(), None));
			if !element_is_bool {
				out.push(method_output_array(vector_class.clone(), None));
				out.push(method_input_output_array(vector_class.clone(), None));
			}
		}
		if !element_is_bool && is_data_type_or_vec_of_data_type(&element_type, true) {
			let only_opencv_5 = Some((CFG_ATTR_ONLY_OPENCV_5.0.to_string(), CFG_ATTR_ONLY_OPENCV_5.1.to_string()));
			out.push(method_input_array(vector_class.clone(), only_opencv_5.clone()));
			out.push(method_output_array(vector_class.clone(), only_opencv_5.clone()));
			out.push(method_input_output_array(vector_class, only_opencv_5));
		}
	}
	out
}

/// Returns true if self is a data type or a vector with the element being a data type
fn is_data_type_or_vec_of_data_type(type_ref: &TypeRef, for_opencv_5: bool) -> bool {
	type_ref.kind().as_vector().map_or_else(
		|| {
			if for_opencv_5 {
				type_ref.is_data_type_in_opencv_5()
			} else {
				type_ref.is_data_type()
			}
		},
		|vec| {
			if for_opencv_5 {
				vec.element_type().is_data_type_in_opencv_5()
			} else {
				vec.element_type().is_data_type()
			}
		},
	)
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
		[],
		vec_type_ref,
	))
}

fn method_len<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::InfallibleNaked,
			"cv::len",
			"core",
			[],
			TypeRefDesc::size_t(),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->size()".into())),
	)
}

fn method_is_empty<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::InfallibleNaked,
			"cv::isEmpty",
			"core",
			[],
			TypeRefDesc::bool(),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->empty()".into())),
	)
}

fn method_capacity<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::InfallibleNaked,
			"cv::capacity",
			"core",
			[],
			TypeRefDesc::size_t(),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->capacity()".into())),
	)
}

fn method_shrink_to_fit<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::shrinkToFit",
			"core",
			[],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->shrink_to_fit()".into())),
	)
}

fn method_reserve<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::reserve",
			"core",
			[Field::new_desc(FieldDesc::new("additional", TypeRefDesc::size_t()))],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall(
			"instance->reserve(instance->size() + {{args}})".into(),
		)),
	)
}

fn method_remove<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::remove",
			"core",
			[Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall(
			"instance->erase(instance->begin() + {{args}})".into(),
		)),
	)
}

fn method_swap<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_is_bool: bool) -> Func<'tu, 'ge> {
	// https://stackoverflow.com/questions/58660207/why-doesnt-stdswap-work-on-vectorbool-elements-under-clang-win
	let swap_func = if element_is_bool {
		"instance->swap"
	} else {
		"std::swap"
	};
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::swap",
			"core",
			[
				Field::new_desc(FieldDesc::new("index1", TypeRefDesc::size_t())),
				Field::new_desc(FieldDesc::new("index2", TypeRefDesc::size_t())),
			],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall(
			format!("{swap_func}((*instance)[index1], (*instance)[index2])").into(),
		)),
	)
}

fn method_clear<'tu, 'ge>(vector_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		"cv::clear",
		"core",
		[],
		TypeRefDesc::void(),
	))
}

fn method_push<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::push",
			"core",
			[Field::new_desc(FieldDesc::new(
				"val",
				element_type.with_inherent_constness(Constness::Const),
			))],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->push_back({{args}})".into())),
	)
}

fn method_insert<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::insert",
			"core",
			[
				Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
				Field::new_desc(FieldDesc::new("val", element_type.with_inherent_constness(Constness::Const))),
			],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall(
			"instance->insert(instance->begin() + {{args}})".into(),
		)),
	)
}

fn method_get<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::InfallibleViaArg,
			"cv::get",
			"core",
			[Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t()))],
			element_type,
		)
		.cpp_body(FuncCppBody::ManualCall("(*instance)[{{args}}]".into())),
	)
}

fn method_set<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::set",
			"core",
			[
				Field::new_desc(FieldDesc::new("index", TypeRefDesc::size_t())),
				Field::new_desc(FieldDesc::new(
					"val",
					element_type.clone().with_inherent_constness(Constness::Const),
				)),
			],
			TypeRefDesc::void(),
		)
		.cpp_body(FuncCppBody::ManualCall(
			format!(
				"(*instance)[index] = {}",
				element_type.render_lane().to_dyn().cpp_arg_func_call("val")
			)
			.into(),
		)),
	)
}

fn method_clone<'tu, 'ge>(vector_class: Class<'tu, 'ge>, vec_type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::InfallibleNaked,
			"cv::clone",
			"core",
			[],
			vec_type_ref,
		)
		.cpp_body(FuncCppBody::ManualCall("{{ret_type}}(*instance)".into())),
	)
}

fn method_data<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(vector_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"cv::data",
		"core",
		[],
		TypeRef::new_pointer(element_type.with_inherent_constness(Constness::Const))
			.with_type_hint(TypeRefTypeHint::CharPtrSingleChar),
	))
}

fn method_data_mut<'tu, 'ge>(vector_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::InfallibleNaked,
			"cv::dataMut",
			"core",
			[],
			TypeRef::new_pointer(element_type.with_inherent_constness(Constness::Mut))
				.with_type_hint(TypeRefTypeHint::CharPtrSingleChar),
		)
		.cpp_body(FuncCppBody::ManualCall("instance->data()".into())),
	)
}

fn method_from_slice<'tu, 'ge>(vec_type_ref: TypeRef<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::Function,
			Constness::Const,
			ReturnKind::InfallibleNaked,
			"cv::fromSlice",
			"core",
			[
				Field::new_desc(FieldDesc::new(
					"data",
					TypeRef::new_pointer(element_type.with_inherent_constness(Constness::Const)),
				)),
				Field::new_desc(FieldDesc::new("len", TypeRefDesc::size_t())),
			],
			vec_type_ref,
		)
		.cpp_body(FuncCppBody::ManualCallReturn(
			"return new {{ret_type}}(data, data + len);".into(),
		)),
	)
}

fn method_input_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>, cfg_attr: Option<(String, String)>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Const,
			ReturnKind::Fallible,
			"cv::inputArray",
			"core",
			[],
			TypeRefDesc::cv_input_array()
				.with_inherent_constness(Constness::Const)
				.with_type_hint(TypeRefTypeHint::BoxedAsRef(
					Constness::Mut,
					&[ARG_OVERRIDE_SELF],
					Lifetime::Elided,
				)),
		)
		.cpp_body(FuncCppBody::ManualCall("cv::_InputArray(*instance)".into()))
		.cfg_attr(cfg_attr),
	)
}

fn method_output_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>, cfg_attr: Option<(String, String)>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::Fallible,
			"cv::outputArray",
			"core",
			[],
			TypeRefDesc::cv_output_array()
				.with_inherent_constness(Constness::Mut)
				.with_type_hint(TypeRefTypeHint::BoxedAsRef(
					Constness::Mut,
					&[ARG_OVERRIDE_SELF],
					Lifetime::Elided,
				)),
		)
		.cpp_body(FuncCppBody::ManualCall("cv::_OutputArray(*instance)".into()))
		.cfg_attr(cfg_attr),
	)
}

fn method_input_output_array<'tu, 'ge>(vector_class: Class<'tu, 'ge>, cfg_attr: Option<(String, String)>) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(vector_class),
			Constness::Mut,
			ReturnKind::Fallible,
			"cv::inputOutputArray",
			"core",
			[],
			TypeRefDesc::cv_input_output_array()
				.with_inherent_constness(Constness::Mut)
				.with_type_hint(TypeRefTypeHint::BoxedAsRef(
					Constness::Mut,
					&[ARG_OVERRIDE_SELF],
					Lifetime::Elided,
				)),
		)
		.cpp_body(FuncCppBody::ManualCall("cv::_InputOutputArray(*instance)".into()))
		.cfg_attr(cfg_attr),
	)
}
