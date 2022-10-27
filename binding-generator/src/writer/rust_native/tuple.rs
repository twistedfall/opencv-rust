use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::type_ref::{Constness, ConstnessOverride, ExternDir};
use crate::writer::rust_native::func::disambiguate_single_name;
use crate::writer::rust_native::func_desc::{ClassDesc, CppFuncDesc, FuncDescCppCall, FuncDescKind};
use crate::{CompiledInterpolation, CppNameStyle, Element, FunctionTypeHint, IteratorExt, NameStyle, StrExt, Tuple, TypeRef};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for Tuple<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/tuple/rust.tpl.rs").compile_interpolation());

		let tuple_type = self.type_ref();

		let rust_localalias = self.rust_localalias();

		let mut rets = disambiguate_single_name("arg");

		let getters = self
			.elements()
			.into_iter()
			.enumerate()
			.map(|(i, typ)| {
				let ret_name = rets.next().expect("Endless iterator");
				format!(
					"{ret_name}: {extern_type_mut} = {num}, cv_{alias}_get_{num}: {extern_return} => get_{num}: {typ}",
					ret_name = ret_name,
					extern_type_mut = typ.rust_extern(ExternDir::ToCpp(ConstnessOverride::Mut)),
					alias = rust_localalias,
					num = i,
					typ = typ.rust_name(NameStyle::ref_()),
					extern_return = typ.rust_extern(ExternDir::FromCpp)
				)
			})
			.join(",\n");

		RUST_TPL.interpolate(&hashmap! {
			"rust_localalias" => rust_localalias,
			"rust_full" => self.rust_name(NameStyle::ref_()),
			"rust_extern_const" => tuple_type.rust_extern(ExternDir::ToCpp(ConstnessOverride::Const)),
			"rust_extern_mut" => tuple_type.rust_extern(ExternDir::ToCpp(ConstnessOverride::Mut)),
			"inner_rust_full" => self.rust_inner().into(),
			"getters" => getters.into(),
		})
	}

	fn gen_cpp(&self) -> String {
		static CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/tuple/cpp.tpl.cpp").compile_interpolation());

		let type_ref = self.type_ref();

		let rust_localalias = self.rust_localalias();
		let elements = self.elements();
		let tuple_desc = ClassDesc {
			is_by_ptr: true,
			cpp_name_ref: type_ref.cpp_name(CppNameStyle::Reference).into_owned(),
		};
		let void = self.gen_env.resolve_typeref("void");
		let mut methods = vec![
			method_new(&rust_localalias, &type_ref, self.tuple_type, &elements),
			method_delete(&rust_localalias, &tuple_desc, &void),
		];

		for (i, typ) in elements.into_iter().enumerate() {
			methods.push(method_get(&rust_localalias, &tuple_desc, &typ, i));
		}

		CPP_TPL.interpolate(&hashmap! {
			"methods" => methods.join(""),
		})
	}
}

fn method_new(rust_localalias: &str, tuple_typeref: &TypeRef, tuple_type: &str, elements: &[TypeRef]) -> String {
	let arguments = disambiguate_single_name("arg")
		.zip(elements.iter())
		.map(|(arg_name, type_ref)| (arg_name, type_ref.clone()))
		.collect();
	CppFuncDesc {
		extern_name: format!("cv_{}_new", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: tuple_typeref.clone(),
		kind: FuncDescKind::Function,
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual(
			format!("std::make_{tuple_type}({{{{args}}}})", tuple_type = tuple_type).compile_interpolation(),
		),
		debug: "".to_string(),
		arguments,
	}
	.gen_cpp()
}

fn method_delete(rust_localalias: &str, tuple_desc: &ClassDesc, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_delete", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(tuple_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("delete instance".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}

fn method_get(rust_localalias: &str, tuple_desc: &ClassDesc, element_type: &TypeRef, num: usize) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{alias}_get_{num}", alias = rust_localalias, num = num).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: false,
		return_type: element_type.clone(),
		kind: FuncDescKind::InstanceMethod(tuple_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual(format!("std::get<{num}>(*instance)", num = num).compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}
