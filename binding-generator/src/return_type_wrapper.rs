use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	GeneratedElement,
	GeneratorEnv,
	StrExt,
	StringExt,
	TypeRef,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DefinitionLocation {
	Module,
	Type,
}

pub struct ReturnTypeWrapper<'tu, 'g> {
	type_ref: TypeRef<'tu, 'g>,
	gen_env: &'g GeneratorEnv<'tu>,
	definition_location: DefinitionLocation,
}

impl<'tu, 'g> ReturnTypeWrapper<'tu, 'g> {
	pub fn new(type_ref: TypeRef<'tu, 'g>, gen_env: &'g GeneratorEnv<'tu>, definition_location: DefinitionLocation) -> Self {
		Self { type_ref, gen_env, definition_location }
	}
}

impl GeneratedElement for ReturnTypeWrapper<'_, '_> {
	fn element_order(&self) -> u8 {
		10
	}

	fn element_safe_id(&self) -> String {
		let mut name = self.type_ref.cpp_extern().into_owned();
		name.cleanup_name();
		let module = match self.definition_location {
			DefinitionLocation::Module => {
				self.gen_env.module().into()
			},
			DefinitionLocation::Type => {
				self.type_ref.rust_module()
			},
		};
		format!("{}-{}", module, name)
	}

	fn gen_cpp(&self) -> String {
		// this explicit instantiation is needed for visual studio compiler, without it it produces error:
		// C linkage function cannot return C++ class
		static CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/return_type_wrapper/cpp.tpl.cpp").compile_interpolation()
		);

		CPP_TPL.interpolate(&hashmap! {
			"cpp_full" => self.type_ref.cpp_extern(),
		})
	}
}
