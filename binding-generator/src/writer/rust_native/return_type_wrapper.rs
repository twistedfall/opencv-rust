use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	ReturnTypeWrapper,
	StrExt,
	StringExt,
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for ReturnTypeWrapper<'_, '_> {
	fn element_order(&self) -> u8 {
		10
	}

	fn element_safe_id(&self) -> String {
		let mut name = self.type_ref().cpp_extern_return(self.const_hint()).into_owned();
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
			"cpp_full" => self.type_ref().cpp_extern_return(self.const_hint()),
		})
	}
}
