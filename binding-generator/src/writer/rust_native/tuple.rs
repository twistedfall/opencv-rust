use std::borrow::Cow;
use std::collections::HashSet;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::type_ref::{Constness, FishStyle};
use crate::{CompiledInterpolation, CppNameStyle, FunctionTypeHint, IteratorExt, NameStyle, StrExt, Tuple, TypeRef};

use super::element::{DefaultRustNativeElement, RustElement};
use super::func::disambiguate_single_name;
use super::func_desc::{ClassDesc, CppFuncDesc, FuncDescCppCall, FuncDescKind};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for Tuple<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, style)
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<str> {
		format!(
			"Tuple{fish}<{inner}>",
			fish = fish_style.rust_qual(),
			inner = self.rust_inner()
		)
		.into()
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Tuple<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/tuple/rust.tpl.rs").compile_interpolation());

		let rust_localalias = self.rust_localalias();

		let mut rets = disambiguate_single_name("arg");

		let getters = self
			.elements()
			.into_iter()
			.enumerate()
			.map(|(i, typ)| {
				let ret_name = rets.next().expect("Endless iterator");
				format!(
					"{num} = {ret_name}: {typ}, get_{num} via cv_{alias}_get_{num}",
					num = i,
					ret_name = ret_name,
					typ = typ.rust_name(NameStyle::ref_()),
					alias = rust_localalias,
				)
			})
			.join(",\n");

		RUST_TPL.interpolate(&hashmap! {
			"rust_localalias" => rust_localalias,
			"rust_full" => self.rust_name(NameStyle::ref_()),
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
			is_boxed: true,
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

pub trait TupleExt {
	fn rust_localalias(&self) -> Cow<str>;
	fn rust_inner(&self) -> String;
	fn rust_element_module(&self) -> Cow<str>;
}

impl TupleExt for Tuple<'_, '_> {
	fn rust_localalias(&self) -> Cow<str> {
		format!(
			"TupleOf{typ}",
			typ = self
				.elements()
				.into_iter()
				.map(|e| e.rust_safe_id(true).into_owned())
				.join("_")
		)
		.into()
	}

	fn rust_inner(&self) -> String {
		let mut out = "(".to_string();
		out.push_str(
			&self
				.elements()
				.into_iter()
				.map(|e| e.rust_name(NameStyle::ref_()).into_owned())
				.join(", "),
		);
		out.push(')');
		out
	}

	fn rust_element_module(&self) -> Cow<str> {
		let mut elem_modules = self
			.elements()
			.into_iter()
			.map(|elem_type| elem_type.rust_module().into_owned())
			.collect::<HashSet<_>>()
			.into_iter()
			.filter(|m| m != "core")
			.collect::<Vec<_>>();
		if let Some(module) = elem_modules.pop() {
			if elem_modules.is_empty() {
				module.into()
			} else {
				panic!("Too many element modules: {:?} + {:?}", module, elem_modules)
			}
		} else {
			self.rust_module()
		}
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
