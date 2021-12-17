use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Constness,
	ConstnessOverride,
	Element,
	EntityElement,
	SmartPtr,
	StrExt,
	type_ref::{self, FishStyle, NameStyle},
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for SmartPtr<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/rust.tpl.rs").compile_interpolation()
		);

		static TRAIT_RAW_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/trait_raw.tpl.rs").compile_interpolation()
		);

		static BASE_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/base_cast.tpl.rs").compile_interpolation()
		);

		static CTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/ctor.tpl.rs").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_fullname(FishStyle::No),
			"rust_extern_const" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Mut)),
			"inner_rust_full" => pointee_type.rust_full(),
		};

		let mut impls = String::new();
		let mut gen_ctor = pointee_type.is_primitive();
		if let Some(cls) = pointee_type.as_class() {
			gen_ctor |= !cls.is_abstract();
			if cls.is_trait() {
				inter_vars.insert("base_rust_local", cls.rust_localname(FishStyle::No).into_owned().into());
				inter_vars.insert("base_rust_full", cls.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Mut).into_owned().into());
				inter_vars.insert("base_const_rust_full", cls.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Const).into_owned().into());
				impls += &TRAIT_RAW_TPL.interpolate(&inter_vars);
				let mut all_bases = cls.all_bases().into_iter()
					.filter(|b| !b.is_excluded())
					.collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_fullname().cmp(&b.cpp_fullname()));
				for base in all_bases {
					inter_vars.insert("base_rust_local", base.rust_localname(FishStyle::No).into_owned().into());
					inter_vars.insert("base_rust_full", base.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Mut).into_owned().into());
					inter_vars.insert("base_const_rust_full", base.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Const).into_owned().into());
					inter_vars.insert("base_rust_full_ref", base.type_ref().rust_full().into_owned().into());
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
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation()
		);

		static BASE_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/base_cast.tpl.cpp").compile_interpolation()
		);

		static CTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/ctor.tpl.cpp").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inner_cpp_extern = pointee_type.cpp_extern_return(ConstnessOverride::No);
		let mut inner_cpp_extern_const = pointee_type.cpp_extern_return(ConstnessOverride::Yes(Constness::Const));
		if !pointee_type.is_by_ptr() {
			inner_cpp_extern.to_mut().push('*');
			inner_cpp_extern_const.to_mut().push('*');
		}

		let mut const_renderer = type_ref::CppRenderer::new(NameStyle::Reference(FishStyle::Turbo), "", true);
		const_renderer.constness_override = ConstnessOverride::Yes(Constness::Const);
		let cpp_full_const = type_ref.render(const_renderer);

		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"cpp_extern" => type_ref.cpp_extern(),
			"cpp_decl" => type_ref.cpp_arg_func_decl("instance").into(),
			"cpp_full" => type_ref.cpp_full(),
			"cpp_full_const" => cpp_full_const,
			"inner_cpp_full" => pointee_type.cpp_full(),
			"inner_cpp_extern" => inner_cpp_extern,
			"inner_cpp_extern_const" => inner_cpp_extern_const,
		};

		let mut base_cast = String::new();
		let pointee_primitive = pointee_type.is_primitive();
		let mut gen_ctor = pointee_primitive;
		if let Some(cls) = pointee_type.as_class() {
			gen_ctor |= !cls.is_abstract();
			if cls.is_trait() {
				let mut all_bases = cls.all_bases().into_iter()
					.filter(|b| !b.is_excluded() && self.gen_env.is_used_in_smart_ptr(b.entity()))
					.collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_fullname().cmp(&b.cpp_fullname()));
				for base in all_bases {
					inter_vars.insert("base_cpp_full", base.cpp_fullname().into_owned().into());
					inter_vars.insert("base_rust_local", base.rust_localname(FishStyle::No).into_owned().into());
					let new_base_cast = BASE_CAST_TPL.interpolate(&inter_vars);
					if base_cast.is_empty() {
						base_cast = new_base_cast;
					} else {
						base_cast += &new_base_cast;
					}
				}
			}
		}
		if gen_ctor {
			let inner_cpp_func_call = if pointee_primitive {
				format!("new {typ}(val)", typ=pointee_type.cpp_full()).into()
			} else {
				let mut out = pointee_type.cpp_arg_func_call("val");
				if out.starts_with('*') {
					out.to_mut().drain(..1);
				}
				out
			};
			inter_vars.insert("inner_cpp_func_decl", pointee_type.cpp_arg_func_decl("val").into());
			inter_vars.insert("inner_cpp_func_call", inner_cpp_func_call);
			inter_vars.insert("ctor", CTOR_TPL.interpolate(&inter_vars).into());
		} else {
			inter_vars.insert("ctor", "".into());
		};
		inter_vars.insert("base_cast", base_cast.into());

		TPL.interpolate(&inter_vars)
	}
}
