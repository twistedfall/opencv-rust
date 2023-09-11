use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::smart_ptr::SmartPtrDesc;
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle, TypeRef, TypeRefKind};
use crate::writer::rust_native::class::rust_generate_debug_fields;
use crate::writer::rust_native::RustStringExt;
use crate::{Class, CompiledInterpolation, Element, Func, IteratorExt, SmartPtr, StrExt};

use super::class::ClassExt;
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for SmartPtr<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		self.pointee().rust_module().into_owned().into()
	}

	// fixme, we shouldn't override the rust_module_reference and rely on rust_module to provide the correct module
	fn rust_module_reference(&self) -> Cow<str> {
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
		format!(
			"Ptr{fish}<{typ}>",
			fish = fish_style.rust_qual(),
			typ = self.pointee().rust_name(NameStyle::ref_()),
		)
		.into()
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		match self {
			&Self::Clang { entity, .. } => {
				DefaultRustNativeElement::rendered_doc_comment_with_prefix(entity, prefix, opencv_version)
			}
			Self::Desc(_) => "".to_string(),
		}
	}
}

impl RustNativeGeneratedElement for SmartPtr<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/smart_ptr/rust.tpl.rs").compile_interpolation());

		static TRAIT_RAW_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/trait_raw.tpl.rs").compile_interpolation());

		static BASE_CAST_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/base_cast.tpl.rs").compile_interpolation());

		static IMPL_DEBUG_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/impl_debug.rs").compile_interpolation());

		static CTOR_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/ctor.tpl.rs").compile_interpolation());

		let rust_localalias = self.rust_localalias();
		let rust_full = self.rust_name(NameStyle::ref_());
		let pointee_type = self.pointee();
		let type_ref = self.type_ref();
		let smartptr_class = smartptr_class(&type_ref);

		let extern_delete = FuncDesc::method_delete(smartptr_class.clone()).identifier();
		let extern_get_inner_ptr =
			method_get_inner_ptr(smartptr_class.clone(), pointee_type.clone(), Constness::Const).identifier();
		let extern_get_inner_ptr_mut =
			method_get_inner_ptr(smartptr_class.clone(), pointee_type.clone(), Constness::Mut).identifier();

		let mut inter_vars = HashMap::from([
			("rust_localalias", rust_localalias.clone()),
			("rust_full", rust_full.clone()),
			(
				"inner_rust_full",
				pointee_type.rust_name(NameStyle::ref_()).into_owned().into(),
			),
			("extern_delete", extern_delete.into()),
			("extern_get_inner_ptr", extern_get_inner_ptr.into()),
			("extern_get_inner_ptr_mut", extern_get_inner_ptr_mut.into()),
		]);

		let mut impls = String::new();
		if let Some(cls) = pointee_type.as_class().filter(Class::is_trait) {
			inter_vars.insert("base_rust_local", cls.rust_name(NameStyle::decl()).into_owned().into());
			inter_vars.insert(
				"base_rust_full",
				cls.rust_trait_name(NameStyle::ref_(), Constness::Mut).into_owned().into(),
			);
			inter_vars.insert(
				"base_const_rust_full",
				cls.rust_trait_name(NameStyle::ref_(), Constness::Const).into_owned().into(),
			);
			impls += &TRAIT_RAW_TPL.interpolate(&inter_vars);
			let fields = cls.fields();
			let mut field_const_methods = cls.field_methods(
				fields.iter().filter(|f| f.exclude_kind().is_included()),
				Some(Constness::Const),
			);
			for base in all_bases(&cls) {
				let base_rust_local = base.rust_name(NameStyle::decl());
				inter_vars.insert("base_rust_local", base_rust_local.clone().into_owned().into());
				inter_vars.insert(
					"base_rust_full",
					base.rust_trait_name(NameStyle::ref_(), Constness::Mut).into_owned().into(),
				);
				inter_vars.insert(
					"base_const_rust_full",
					base.rust_trait_name(NameStyle::ref_(), Constness::Const).into_owned().into(),
				);
				inter_vars.insert("base_rust_full_ref", base.rust_name(NameStyle::ref_()).into_owned().into());
				impls += &TRAIT_RAW_TPL.interpolate(&inter_vars);
				let extern_cast_to_base = method_cast_to_base(smartptr_class.clone(), base.type_ref(), &base_rust_local).identifier();
				inter_vars.insert("extern_cast_to_base", extern_cast_to_base.into());
				impls += &BASE_CAST_TPL.interpolate(&inter_vars);
				let base_fields = base.fields();
				let base_field_const_methods = base.field_methods(
					base_fields.iter().filter(|f| f.exclude_kind().is_included()),
					Some(Constness::Const),
				);
				field_const_methods.extend(base_field_const_methods);
			}
			let debug_fields = rust_generate_debug_fields(field_const_methods);
			impls += &IMPL_DEBUG_TPL.interpolate(&HashMap::from([
				("rust_full", rust_full.as_ref()),
				("rust_localalias", rust_localalias.as_ref()),
				("debug_fields", &debug_fields),
			]));
		};
		if gen_ctor(&pointee_type) {
			let extern_new = method_new(smartptr_class, type_ref, pointee_type).identifier();
			inter_vars.insert("extern_new", extern_new.into());
			inter_vars.insert("ctor", CTOR_TPL.interpolate(&inter_vars).into());
		} else {
			inter_vars.insert("ctor", "".into());
		}
		inter_vars.insert("impls", impls.into());
		TPL.interpolate(&inter_vars)
	}

	fn gen_rust_exports(&self) -> String {
		extern_functions(self).iter().map(Func::gen_rust_exports).join("")
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation());

		TPL.interpolate(&[("methods", extern_functions(self).iter().map(Func::gen_cpp).join(""))].into())
	}
}

#[inline]
fn extern_functions<'tu, 'ge>(ptr: &SmartPtr<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let type_ref = ptr.type_ref();
	let pointee_type = ptr.pointee();
	let smartptr_class = smartptr_class(&type_ref);

	let mut out = Vec::with_capacity(6);
	out.push(method_get_inner_ptr(
		smartptr_class.clone(),
		pointee_type.clone(),
		Constness::Const,
	));
	out.push(method_get_inner_ptr(
		smartptr_class.clone(),
		pointee_type.clone(),
		Constness::Mut,
	));
	out.push(FuncDesc::method_delete(smartptr_class.clone()));
	if let Some(cls) = pointee_type.as_class().filter(Class::is_trait) {
		for base in all_bases(&cls) {
			out.push(method_cast_to_base(
				smartptr_class.clone(),
				base.type_ref(),
				&base.rust_name(NameStyle::decl()),
			));
		}
	}
	if gen_ctor(&pointee_type) {
		out.push(method_new(smartptr_class, type_ref, pointee_type));
	}
	out
}

fn gen_ctor(pointee_type: &TypeRef) -> bool {
	match pointee_type.canonical().kind().as_ref() {
		TypeRefKind::Primitive(_, _) => true,
		TypeRefKind::Class(cls) => !cls.is_abstract(),
		_ => false,
	}
}

fn all_bases<'tu, 'ge>(cls: &Class<'tu, 'ge>) -> Vec<Class<'tu, 'ge>> {
	let mut out = cls
		.all_bases()
		.into_iter()
		.filter(|b| b.exclude_kind().is_included())
		.collect::<Vec<_>>();
	out.sort_unstable_by(|a, b| a.cpp_name(CppNameStyle::Reference).cmp(&b.cpp_name(CppNameStyle::Reference)));
	out
}

pub trait SmartPtrExt {
	fn rust_localalias(&self) -> Cow<str>;
}

impl SmartPtrExt for SmartPtr<'_, '_> {
	fn rust_localalias(&self) -> Cow<str> {
		/*
		let pointee = self.pointee();
		let pointee_alias = pointee.rust_safe_id(true);
		let pointee_alias = if let Some(rem) = pointee_alias.strip_prefix("const_") {
			format!("Const{rem}").into()
		} else {
			pointee_alias
		};
		format!("PtrOf{pointee_alias}").into()
		*/
		// fixme: Not adding const here in rust_safe_id() leads to some smart pointers losing the const qualifier on the internal
		// type (e.g. cv::Ptr<const cv::optflow::PCAPrior>). If we add it (see commented code above) it leads to problems with casting
		// because casting doesn't take constness into account. This might not be a problem per se (e.g. if we own Ptr<PCAPrior> there is
		// no problem to pass it as Ptr<const PCAPrior>), otherwise fix it so that it works with add_const = true in rust_safe_id().
		format!("PtrOf{typ}", typ = self.pointee().rust_safe_id(false)).into()
	}
}

fn smartptr_class<'tu, 'ge>(smart_ptr_type_ref: &TypeRef<'tu, 'ge>) -> Class<'tu, 'ge> {
	Class::new_desc(ClassDesc::boxed(
		smart_ptr_type_ref.cpp_name(CppNameStyle::Reference),
		"<unused>",
	))
}

fn method_new<'tu, 'ge>(
	smartptr_class: Class<'tu, 'ge>,
	smartptr_type_ref: TypeRef<'tu, 'ge>,
	pointee_type: TypeRef<'tu, 'ge>,
) -> Func<'tu, 'ge> {
	let val = if pointee_type.is_copy() {
		if pointee_type.as_simple_class().is_some() {
			panic!("Ptr with simple class is not supported");
		} else {
			format!("new {typ}(val)", typ = pointee_type.cpp_name(CppNameStyle::Reference)).into()
		}
	} else {
		Cow::Borrowed("val")
	};
	Func::new_desc(FuncDesc::new(
		FuncKind::Constructor(smartptr_class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"new",
		"<unused>",
		vec![Field::new_desc(FieldDesc::new("val", pointee_type))],
		FuncCppBody::ManualFull(format!("return new {{{{ret_type}}}}({val});").into()),
		FuncRustBody::Auto,
		smartptr_type_ref,
	))
}

fn method_cast_to_base<'tu, 'ge>(
	smartptr_class: Class<'tu, 'ge>,
	base_type_ref: TypeRef<'tu, 'ge>,
	base_rust_local: &str,
) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(smartptr_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("to_PtrOf{base_rust_local}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualFull(
			format!(
				"return new {{{{ret_type}}}}(instance->dynamicCast<{base_type}>());",
				base_type = base_type_ref.cpp_name(CppNameStyle::Reference)
			)
			.into(),
		),
		FuncRustBody::Auto,
		TypeRef::new_smartptr(SmartPtr::new_desc(SmartPtrDesc::new(base_type_ref))),
	))
}

fn method_get_inner_ptr<'tu, 'ge>(
	smartptr_class: Class<'tu, 'ge>,
	pointee_type: TypeRef<'tu, 'ge>,
	constness: Constness,
) -> Func<'tu, 'ge> {
	let pointee_type = if constness.is_const() {
		pointee_type.with_constness(constness)
	} else {
		pointee_type
	};
	// if the pointee type is actually const make sure to also generate a const function, needed for
	// Ptr<const cv::optflow::PCAPrior*>
	let pointee_type_constness = pointee_type.constness();
	let return_type_ref = if pointee_type.extern_pass_kind().is_by_ptr() {
		pointee_type
	} else {
		TypeRef::new_pointer(pointee_type)
	};
	let name_suffix = if constness.is_mut() {
		"Mut"
	} else {
		""
	};
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(smartptr_class),
		pointee_type_constness,
		ReturnKind::InfallibleNaked,
		format!("getInnerPtr{name_suffix}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualFull("return instance->get();".into()),
		FuncRustBody::Auto,
		return_type_ref,
	))
}
