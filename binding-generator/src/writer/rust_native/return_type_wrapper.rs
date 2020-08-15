use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	ConstnessOverride,
	ReturnTypeWrapper,
	StrExt,
	StringExt,
	type_ref::CppExternReturnRenderer,
	TypeRef,
};

use super::RustNativeGeneratedElement;

fn cpp_extern_return<'t>(type_ref: &'t TypeRef, constness: ConstnessOverride) -> Cow<'t, str> {
	let mut renderer = CppExternReturnRenderer::new();
	renderer.constness_override = constness;
	type_ref.render(renderer)
}

impl RustNativeGeneratedElement for ReturnTypeWrapper<'_> {
	fn element_order(&self) -> u8 {
		10
	}

	fn element_safe_id(&self) -> String {
		let mut name = cpp_extern_return(self.type_ref(), self.const_hint()).into_owned();
		name.cleanup_name();
		format!("{}-{}", self.definition_location(), name)
	}

	fn gen_cpp(&self) -> String {
		// this explicit instantiation is needed for visual studio compiler, without it it produces error:
		// C linkage function cannot return C++ class
		static CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/return_type_wrapper/cpp.tpl.cpp").compile_interpolation()
		);

		CPP_TPL.interpolate(&hashmap! {
			"cpp_full" => cpp_extern_return(self.type_ref(), self.const_hint()),
		})
	}
}
