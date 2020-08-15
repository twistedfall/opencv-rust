use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Constness,
	ConstnessOverride,
	Element,
	SmartPtr,
	StrExt,
	type_ref,
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for SmartPtr<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localalias())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/rust.tpl.rs").compile_interpolation()
		);

		static TRAIT_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/trait_cast.tpl.rs").compile_interpolation()
		);

		static CTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/ctor.tpl.rs").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"rust_full" => self.rust_fullname(),
			"rust_extern_const" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Mut)),
			"inner_rust_full" => pointee_type.rust_full(),
		};

		let mut impls = String::new();
		if pointee_type.is_primitive() || pointee_type.as_class().map_or(false, |c| !c.is_abstract()) {
			inter_vars.insert("ctor", CTOR_TPL.interpolate(&inter_vars).into());
		} else {
			inter_vars.insert("ctor", "".into());
		}
		if let Some(cls) = pointee_type.as_class() {
			if cls.is_trait() {
				let mut all_bases = cls.all_bases();
				all_bases.insert(cls);
				let mut all_bases = all_bases.into_iter()
					.filter(|b| !b.is_excluded())
					.collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_fullname().cmp(&b.cpp_fullname()));
				for base in all_bases {
					inter_vars.insert("base_rust_local", base.rust_localname().into_owned().into());
					inter_vars.insert("base_rust_full", base.rust_trait_fullname().into_owned().into());
					impls += &TRAIT_CAST_TPL.interpolate(&inter_vars);
				}
			}
		};
		inter_vars.insert("impls", impls.into());
		TPL.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation()
		);

		static CTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/smart_ptr/ctor.tpl.cpp").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inner_cpp_extern = pointee_type.cpp_extern_return();
		if !pointee_type.is_by_ptr() {
			inner_cpp_extern.to_mut().push('*');
		}

		let mut inter_vars = hashmap! {
			"rust_localalias" => self.rust_localalias(),
			"cpp_extern" => type_ref.cpp_extern(),
			"cpp_full" => type_ref.cpp_full(),
			"inner_cpp_full" => pointee_type.cpp_full(),
			"inner_cpp_extern" => inner_cpp_extern,
		};

		let pointee_primitive = pointee_type.is_primitive();
		if pointee_primitive || pointee_type.as_class().map_or(false, |c| !c.is_abstract()) {
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

		TPL.interpolate(&inter_vars)
	}
}
