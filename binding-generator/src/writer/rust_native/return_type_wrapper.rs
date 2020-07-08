use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	ReturnTypeWrapper,
	StrExt,
	StringExt,
	TypeRef,
};

use super::RustNativeGeneratedElement;

fn cpp_extern_return<'t>(type_ref: &'t TypeRef, const_hint: Option<bool>) -> Cow<'t, str> {
	let mut out = type_ref.cpp_extern_return();
	if const_hint.unwrap_or(false) && !type_ref.is_const() {
		out.to_mut().insert_str(0, "const ");
	}
	out
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
