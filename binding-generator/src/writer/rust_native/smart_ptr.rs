use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::renderer::CppRenderer;
use crate::type_ref::{Constness, ConstnessOverride, CppNameStyle, FishStyle, NameStyle};
use crate::{CompiledInterpolation, Element, EntityElement, FunctionTypeHint, SmartPtr, StrExt, TypeRef};

use super::class::ClassExt;
use super::element::{DefaultRustNativeElement, RustElement};
use super::func_desc::{ClassDesc, CppFuncDesc, FuncDescCppCall, FuncDescKind};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustElement for SmartPtr<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		self.pointee().rust_module().into_owned().into()
	}

	fn rust_namespace(&self) -> Cow<str> {
		"core".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultRustNativeElement::rust_name(self, style)
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
		DefaultRustNativeElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
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

		static CTOR_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/ctor.tpl.rs").compile_interpolation());

		let pointee_type = self.pointee();

		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_name(NameStyle::ref_()),
			"inner_rust_full" => pointee_type.rust_name(NameStyle::ref_()),
		};

		let mut impls = String::new();
		let mut gen_ctor = pointee_type.is_primitive();
		if let Some(cls) = pointee_type.as_class() {
			gen_ctor |= !cls.is_abstract();
			if cls.is_trait() {
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
				let mut all_bases = cls.all_bases().into_iter().filter(|b| !b.is_excluded()).collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_name(CppNameStyle::Reference).cmp(&b.cpp_name(CppNameStyle::Reference)));
				for base in all_bases {
					inter_vars.insert("base_rust_local", base.rust_name(NameStyle::decl()).into_owned().into());
					inter_vars.insert(
						"base_rust_full",
						base.rust_trait_name(NameStyle::ref_(), Constness::Mut).into_owned().into(),
					);
					inter_vars.insert(
						"base_const_rust_full",
						base.rust_trait_name(NameStyle::ref_(), Constness::Const).into_owned().into(),
					);
					inter_vars.insert(
						"base_rust_full_ref",
						base.type_ref().rust_name(NameStyle::ref_()).into_owned().into(),
					);
					impls += &TRAIT_RAW_TPL.interpolate(&inter_vars);
					if self.gen_env.is_used_in_smart_ptr(base.entity()) {
						impls += &BASE_CAST_TPL.interpolate(&inter_vars);
					}
				}
			}
		};
		if gen_ctor {
			inter_vars.insert("ctor", CTOR_TPL.interpolate(&inter_vars).into());
		} else {
			inter_vars.insert("ctor", "".into());
		}
		inter_vars.insert("impls", impls.into());
		TPL.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation());

		static BASE_CAST_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/smart_ptr/base_cast.tpl.cpp").compile_interpolation());

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inner_cpp_extern = pointee_type.cpp_extern_return(ConstnessOverride::No);
		let mut inner_cpp_extern_const = pointee_type.cpp_extern_return(ConstnessOverride::Const);
		if !pointee_type.is_extern_by_ptr() {
			inner_cpp_extern.to_mut().push('*');
			inner_cpp_extern_const.to_mut().push('*');
		}

		let mut const_renderer = CppRenderer::new(CppNameStyle::Reference, "", true);
		const_renderer.constness_override = ConstnessOverride::Const;
		let cpp_ref_const = type_ref.render(const_renderer);

		let rust_localalias = self.rust_localalias();
		let mut inter_vars = hashmap! {
			"rust_localalias" => rust_localalias.as_ref().into(),
			"cpp_decl" => type_ref.cpp_arg_func_decl("instance").into(),
			"cpp_full_const" => cpp_ref_const,
			"inner_cpp_extern" => inner_cpp_extern,
			"inner_cpp_extern_const" => inner_cpp_extern_const,
		};

		let mut base_cast = String::new();
		let mut gen_ctor = pointee_type.is_copy();
		if let Some(cls) = pointee_type.as_class() {
			gen_ctor |= !cls.is_abstract();
			if cls.is_trait() {
				let mut all_bases = cls
					.all_bases()
					.into_iter()
					.filter(|b| !b.is_excluded() && self.gen_env.is_used_in_smart_ptr(b.entity()))
					.collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_name(CppNameStyle::Reference).cmp(&b.cpp_name(CppNameStyle::Reference)));
				for base in all_bases {
					inter_vars.insert("base_cpp_full", base.cpp_name(CppNameStyle::Reference).into_owned().into());
					inter_vars.insert("base_rust_local", base.rust_name(NameStyle::decl()).into_owned().into());
					let new_base_cast = BASE_CAST_TPL.interpolate(&inter_vars);
					if base_cast.is_empty() {
						base_cast = new_base_cast;
					} else {
						base_cast += &new_base_cast;
					}
				}
			}
		}
		let mut methods = vec![];
		if gen_ctor {
			methods.push(method_new(&rust_localalias, &type_ref, &pointee_type));
		};
		let smartptr_desc = ClassDesc {
			is_boxed: true,
			cpp_name_ref: type_ref.cpp_name(CppNameStyle::Reference).into_owned(),
		};
		methods.push(method_delete(
			&rust_localalias,
			&smartptr_desc,
			&self.gen_env.resolve_typeref("void"),
		));
		inter_vars.insert("methods", methods.join("").into());
		inter_vars.insert("base_cast", base_cast.into());

		TPL.interpolate(&inter_vars)
	}
}

pub trait SmartPtrExt {
	fn rust_localalias(&self) -> Cow<str>;
}

impl SmartPtrExt for SmartPtr<'_, '_> {
	fn rust_localalias(&self) -> Cow<str> {
		format!("PtrOf{typ}", typ = self.pointee().rust_safe_id(false)).into()
	}
}

fn method_new(rust_localalias: &str, smartptr_type: &TypeRef, pointee_type: &TypeRef) -> String {
	let val = if pointee_type.is_copy() {
		if pointee_type.as_simple_class().is_some() {
			panic!("Ptr with simple class is not supported");
		} else {
			format!("new {typ}(val)", typ = pointee_type.cpp_name(CppNameStyle::Reference)).into()
		}
	} else {
		Cow::Borrowed("val")
	};
	CppFuncDesc {
		extern_name: format!("cv_{}_new", rust_localalias).into(),
		constness: Constness::Const,
		is_infallible: true,
		is_naked_return: true,
		return_type: smartptr_type.clone(),
		kind: FuncDescKind::Function,
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual(format!("{{{{ret_type}}}}({val})", val = val).compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![("val".to_string(), pointee_type.clone())],
	}
	.gen_cpp()
}

fn method_delete(rust_localalias: &str, smartptr_desc: &ClassDesc, void: &TypeRef) -> String {
	CppFuncDesc {
		extern_name: format!("cv_{}_delete", rust_localalias).into(),
		constness: Constness::Mut,
		is_infallible: true,
		is_naked_return: true,
		return_type: void.clone(),
		kind: FuncDescKind::InstanceMethod(smartptr_desc.clone()),
		type_hint: FunctionTypeHint::None,
		call: FuncDescCppCall::Manual("delete instance".compile_interpolation()),
		debug: "".to_string(),
		arguments: vec![],
	}
	.gen_cpp()
}
