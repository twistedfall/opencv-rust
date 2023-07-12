use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
use crate::smart_ptr::SmartPtrDesc;
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle, TypeRef, TypeRefDesc, TypeRefKind};
use crate::writer::rust_native::class::rust_generate_debug_fields;
use crate::writer::rust_native::RustStringExt;
use crate::{Class, CompiledInterpolation, Element, Func, GeneratorEnv, SmartPtr, StrExt};

use super::class::ClassExt;
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for SmartPtr<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		self.pointee().rust_module().into_owned().into()
	}

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

		let extern_delete = FuncDesc::method_delete(&rust_localalias, smartptr_class.clone()).identifier();
		let extern_get_inner_ptr = method_get_inner_ptr(
			smartptr_class.clone(),
			&rust_localalias,
			pointee_type.clone(),
			Constness::Const,
		)
		.identifier();
		let extern_get_inner_ptr_mut =
			method_get_inner_ptr(smartptr_class, &rust_localalias, pointee_type.clone(), Constness::Mut).identifier();

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
			let gen_env = self.gen_env();
			let fields = cls.fields();
			let mut field_const_methods = cls.field_methods(
				fields.iter().filter(|f| f.exclude_kind().is_included()),
				Some(Constness::Const),
			);
			for base in all_bases(&cls) {
				inter_vars.insert("base_rust_local", base.rust_name(NameStyle::decl()).into_owned().into());
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
				if gen_env.is_used_in_smart_ptr(base.cpp_name(CppNameStyle::Reference).as_ref()) {
					impls += &BASE_CAST_TPL.interpolate(&inter_vars);
				}
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
			let extern_new = method_new(&rust_localalias, type_ref, pointee_type).identifier();
			inter_vars.insert("extern_new", extern_new.into());
			inter_vars.insert("ctor", CTOR_TPL.interpolate(&inter_vars).into());
		} else {
			inter_vars.insert("ctor", "".into());
		}
		inter_vars.insert("impls", impls.into());
		TPL.interpolate(&inter_vars)
	}

	fn gen_rust_exports(&self) -> String {
		let type_ref = self.type_ref();
		let pointee_type = self.pointee();
		let rust_localalias = self.rust_localalias();
		let smartptr_class = smartptr_class(&type_ref);

		let mut out = String::new();
		out.push_str(
			&method_get_inner_ptr(
				smartptr_class.clone(),
				&rust_localalias,
				pointee_type.clone(),
				Constness::Const,
			)
			.gen_rust_exports(),
		);
		out.push_str(
			&method_get_inner_ptr(smartptr_class.clone(), &rust_localalias, pointee_type.clone(), Constness::Mut).gen_rust_exports(),
		);
		out.push_str(&FuncDesc::method_delete(&rust_localalias, smartptr_class.clone()).gen_rust_exports());
		if let Some(cls) = pointee_type.as_class().filter(Class::is_trait) {
			let gen_env = self.gen_env();
			for base in all_bases(&cls)
				.into_iter()
				.filter(|base| gen_env.is_used_in_smart_ptr(base.cpp_name(CppNameStyle::Reference).as_ref()))
			{
				out.push_str(
					&method_cast_to_base(
						smartptr_class.clone(),
						base.type_ref(),
						&rust_localalias,
						&base.rust_name(NameStyle::decl()),
						self.gen_env(),
					)
					.gen_rust_exports(),
				);
			}
		}
		if gen_ctor(&pointee_type) {
			out.push_str(&method_new(&rust_localalias, type_ref, pointee_type).gen_rust_exports());
		}
		out
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation());

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();
		let smartptr_class = smartptr_class(&type_ref);
		let rust_localalias = self.rust_localalias();

		let mut methods = Vec::with_capacity(8);
		methods.push(
			method_get_inner_ptr(
				smartptr_class.clone(),
				&rust_localalias,
				pointee_type.clone(),
				Constness::Const,
			)
			.gen_cpp(),
		);
		methods
			.push(method_get_inner_ptr(smartptr_class.clone(), &rust_localalias, pointee_type.clone(), Constness::Mut).gen_cpp());
		methods.push(FuncDesc::method_delete(&rust_localalias, smartptr_class.clone()).gen_cpp());
		if let Some(cls) = pointee_type.as_class().filter(Class::is_trait) {
			let gen_env = self.gen_env();
			for base in all_bases(&cls)
				.into_iter()
				.filter(|base| gen_env.is_used_in_smart_ptr(base.cpp_name(CppNameStyle::Reference).as_ref()))
			{
				methods.push(
					method_cast_to_base(
						smartptr_class.clone(),
						base.type_ref(),
						&rust_localalias,
						&base.rust_name(NameStyle::decl()),
						self.gen_env(),
					)
					.gen_cpp(),
				);
			}
		}

		if gen_ctor(&pointee_type) {
			methods.push(method_new(&rust_localalias, type_ref, pointee_type).gen_cpp());
		};

		TPL.interpolate(&[("methods", methods.join(""))].into())
	}
}

fn gen_ctor(pointee_type: &TypeRef) -> bool {
	pointee_type.is_primitive() || pointee_type.as_class().as_ref().map_or(false, |cls| !cls.is_abstract())
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
	rust_localalias: &str,
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
		FuncKind::Function,
		Constness::Const,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::new"),
		"<unused>",
		vec![Field::new_desc(FieldDesc::new("val", pointee_type))],
		FuncCppBody::ManualCall(format!("{{{{ret_type}}}}({val})").into()),
		smartptr_type_ref,
	))
}

fn method_cast_to_base<'tu, 'ge>(
	smartptr_class: Class<'tu, 'ge>,
	base_type_ref: TypeRef<'tu, 'ge>,
	rust_localalias: &str,
	base_rust_local: &str,
	gen_env: &'ge GeneratorEnv<'tu>,
) -> Func<'tu, 'ge> {
	let cpp_body = FuncCppBody::ManualFull(
		format!(
			"return new {{{{ret_type}}}}(instance->dynamicCast<{base_type}>());",
			base_type = base_type_ref.cpp_name(CppNameStyle::Reference)
		)
		.into(),
	);
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(smartptr_class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("cv::{rust_localalias}::to_PtrOf{base_rust_local}"),
		"<unused>",
		vec![],
		cpp_body,
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::SmartPtr(SmartPtr::new_desc(SmartPtrDesc {
			pointee_type_ref: base_type_ref,
			gen_env,
		})))),
	))
}

fn method_get_inner_ptr<'tu, 'ge>(
	smartptr_class: Class<'tu, 'ge>,
	rust_localalias: &str,
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
		format!("cv::{rust_localalias}::getInnerPtr{name_suffix}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualFull("return instance->get();".into()),
		return_type_ref,
	))
}
