use std::borrow::Cow;

use crate::{
	ConstnessOverride,
	GeneratorEnv,
	TypeRef,
};

#[derive(Clone, Debug, PartialEq)]
pub enum DefinitionLocation {
	Module,
	Type,
	Custom(String),
}

#[derive(Debug)]
pub struct ReturnTypeWrapper<'tu, 'ge> {
	type_ref: TypeRef<'tu, 'ge>,
	const_hint: ConstnessOverride,
	definition_location: DefinitionLocation,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> ReturnTypeWrapper<'tu, 'ge> {
	pub fn new(type_ref: TypeRef<'tu, 'ge>, definition_location: DefinitionLocation, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(type_ref, ConstnessOverride::No, definition_location, gen_env)
	}

	pub fn new_ext(type_ref: TypeRef<'tu, 'ge>, const_hint: ConstnessOverride, definition_location: DefinitionLocation, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { type_ref, const_hint, definition_location, gen_env }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu, 'ge> {
		&self.type_ref
	}

	pub fn const_hint(&self) -> ConstnessOverride {
		self.const_hint
	}

	pub fn definition_location(&self) -> Cow<str> {
		match &self.definition_location {
			DefinitionLocation::Module => {
				self.gen_env.module().into()
			}
			DefinitionLocation::Type => {
				self.type_ref.rust_module()
			}
			DefinitionLocation::Custom(module) => {
				module.into()
			}
		}
	}
}
