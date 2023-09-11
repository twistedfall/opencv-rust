use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::{Constness, FishStyle};
use crate::{Class, CompiledInterpolation, CppNameStyle, EntityElement, Func, IteratorExt, NameStyle, StrExt, Tuple, TypeRef};

use super::disambiguate_single_name;
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for Tuple<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		DefaultRustNativeElement::rust_module(self.entity())
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, self.entity(), style).into()
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
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self.entity(), prefix, opencv_version)
	}
}

impl RustNativeGeneratedElement for Tuple<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_element_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/tuple/rust.tpl.rs").compile_interpolation());

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
		let new_extern = method_new(type_ref, &elements).identifier();
		let delete_extern = FuncDesc::method_delete(tuple_desc).identifier();

		RUST_TPL.interpolate(&HashMap::from([
			("rust_localalias", rust_localalias),
			("rust_full", self.rust_name(NameStyle::ref_())),
			("inner_rust_full", self.rust_inner().into()),
			("getters", getters.into()),
			("new_extern", new_extern.into()),
			("delete_extern", delete_extern.into()),
		]))
	}

	fn gen_rust_exports(&self) -> String {
		extern_functions(self).iter().map(Func::gen_rust_exports).join("")
	}

	fn gen_cpp(&self) -> String {
		static CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/tuple/cpp.tpl.cpp").compile_interpolation());

		CPP_TPL.interpolate(&HashMap::from([(
			"methods",
			extern_functions(self).iter().map(Func::gen_cpp).join(""),
		)]))
	}
}

#[inline]
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
				panic!("Too many element modules: {module:?} + {elem_modules:?}")
			}
		} else {
			self.rust_module()
		}
	}
}

fn tuple_class<'tu, 'ge>(typle_type_ref: &TypeRef<'tu, 'ge>) -> Class<'tu, 'ge> {
	Class::new_desc(ClassDesc::boxed(typle_type_ref.cpp_name(CppNameStyle::Reference), "<unused>"))
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
		"<unused>",
		arguments,
		FuncCppBody::Auto,
		FuncRustBody::Auto,
		tuple_typeref,
	))
}

fn method_get<'tu, 'ge>(tuple_class: Class<'tu, 'ge>, element_type: TypeRef<'tu, 'ge>, num: usize) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(tuple_class),
		Constness::Const,
		ReturnKind::InfallibleViaArg,
		format!("get_{num}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualCall(format!("std::get<{num}>(*instance)").into()),
		FuncRustBody::Auto,
		element_type.clone(),
	))
}
