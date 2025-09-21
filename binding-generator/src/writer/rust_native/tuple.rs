use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::{disambiguate_single_name, RustNativeGeneratedElement};
use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
use crate::type_ref::{Constness, FishStyle};
use crate::{
	Class, CompiledInterpolation, CppNameStyle, EntityElement, Func, IteratorExt, NameStyle, StrExt, SupportedModule, Tuple,
	TypeRef,
};

impl RustElement for Tuple<'_, '_> {
	fn rust_module(&self) -> SupportedModule {
		DefaultRustNativeElement::rust_module(self.entity())
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str> {
		DefaultRustNativeElement::rust_name(self, self.entity(), style).into()
	}

	fn rust_leafname(&self, fish_style: FishStyle) -> Cow<'_, str> {
		format!(
			"Tuple{fish}<{inner}>",
			fish = fish_style.rust_qual(),
			inner = self.rust_inner()
		)
		.into()
	}
}

impl RustNativeGeneratedElement for Tuple<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module().opencv_name(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/tuple/rust.tpl.rs").compile_interpolation());

		let type_ref = self.type_ref();
		let rust_localalias = self.rust_localalias();
		let elements = self.elements();
		let tuple_desc = tuple_class(&type_ref);

		let mut rets = disambiguate_single_name("arg");

		let getters = elements
			.iter()
			.enumerate()
			.map(|(num, typ)| {
				let ret_name = rets.next().expect("Endless iterator");
				let get_extern = method_get(tuple_desc.clone(), typ.clone(), num).identifier();
				format!(
					"{num} = {ret_name}: {typ}, get_{num} via {get_extern}",
					typ = typ.rust_name(NameStyle::ref_()),
				)
			})
			.join(",\n");
		let new_extern = method_new(type_ref.clone(), &elements).identifier();
		let delete_extern = FuncDesc::method_delete(tuple_desc).identifier();

		RUST_TPL.interpolate(&HashMap::from([
			("rust_localalias", rust_localalias),
			("rust_as_raw_const", type_ref.rust_as_raw_name(Constness::Const).into()),
			("rust_as_raw_mut", type_ref.rust_as_raw_name(Constness::Mut).into()),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("inner_rust_full", self.rust_inner().into()),
			("getters", getters.into()),
			("new_extern", new_extern.into()),
			("delete_extern", delete_extern.into()),
		]))
	}

	fn gen_rust_externs(&self) -> String {
		extern_functions(self).iter().map(Func::gen_rust_externs).join("")
	}

	fn gen_cpp(&self) -> String {
		static CPP_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/tuple/cpp.tpl.cpp").compile_interpolation());

		CPP_TPL.interpolate(&HashMap::from([(
			"methods",
			extern_functions(self).iter().map(Func::gen_cpp).join(""),
		)]))
	}
}

fn extern_functions<'tu, 'ge>(tuple: &Tuple<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let type_ref = tuple.type_ref();
	let elements = tuple.elements();
	let tuple_desc = tuple_class(&type_ref);

	let mut out = Vec::with_capacity(elements.len() + 2);
	out.push(method_new(type_ref, &elements));
	for (i, typ) in elements.into_iter().enumerate() {
		out.push(method_get(tuple_desc.clone(), typ, i));
	}
	out.push(FuncDesc::method_delete(tuple_desc));
	out
}

pub trait TupleExt {
	fn rust_localalias(&self) -> Cow<'_, str>;
	fn rust_inner(&self) -> String;
	fn rust_element_module(&self) -> SupportedModule;
}

impl TupleExt for Tuple<'_, '_> {
	fn rust_localalias(&self) -> Cow<'_, str> {
		const PREFIX: &str = "TupleOf";

		let elems = self.elements();
		let mut out = String::with_capacity(PREFIX.len() + elems.len() * 16);
		out.push_str(PREFIX);
		for (i, elem) in elems.into_iter().enumerate() {
			if i != 0 {
				out.push('_');
			}
			out.push_str(&elem.rust_safe_id(true))
		}
		out.into()
	}

	fn rust_inner(&self) -> String {
		let elems = self.elements();
		let mut out = String::with_capacity(2 + elems.len() * 16);
		out.push('(');
		for (i, elem) in elems.into_iter().enumerate() {
			if i != 0 {
				out.push_str(", ");
			}
			out.push_str(&elem.rust_name(NameStyle::ref_()));
		}
		out.push(')');
		out
	}

	fn rust_element_module(&self) -> SupportedModule {
		self
			.elements()
			.iter()
			.map(|elem_type| elem_type.rust_module())
			.filter(|m| !matches!(m, SupportedModule::Core))
			.reduce(|single_module, elem_module| {
				if single_module != elem_module {
					panic!("Too many element modules: {single_module:?} and at least {elem_module:?}")
				} else {
					single_module
				}
			})
			.unwrap_or_else(|| self.rust_module())
	}
}

fn tuple_class<'tu, 'ge>(typle_type_ref: &TypeRef<'tu, 'ge>) -> Class<'tu, 'ge> {
	Class::new_desc(ClassDesc::boxed(
		typle_type_ref.cpp_name(CppNameStyle::Reference),
		SupportedModule::Core,
	))
}

fn method_new<'tu, 'ge>(tuple_typeref: TypeRef<'tu, 'ge>, elements: &[TypeRef<'tu, 'ge>]) -> Func<'tu, 'ge> {
	let arguments = disambiguate_single_name("arg")
		.zip(elements.iter())
		.map(|(arg_name, type_ref)| Field::new_desc(FieldDesc::new(arg_name, type_ref.clone())))
		.collect::<Vec<_>>();
	Func::new_desc(FuncDesc::new(
		FuncKind::Constructor(tuple_class(&tuple_typeref)),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"new",
		SupportedModule::Core,
		arguments,
		tuple_typeref,
	))
}

fn method_get<'tu, 'ge>(tuple_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>, num: usize) -> Func<'tu, 'ge> {
	Func::new_desc(
		FuncDesc::new(
			FuncKind::InstanceMethod(tuple_class),
			Constness::Const,
			ReturnKind::InfallibleViaArg,
			format!("get_{num}"),
			SupportedModule::Core,
			[],
			element_type.clone(),
		)
		.cpp_body(FuncCppBody::ManualCall(format!("std::get<{num}>(*instance)").into())),
	)
}
