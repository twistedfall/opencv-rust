use std::borrow::Cow;

use crate::{
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
pub struct ReturnTypeWrapper<'tu, 'g> {
	type_ref: TypeRef<'tu, 'g>,
	gen_env: &'g GeneratorEnv<'tu>,
	definition_location: DefinitionLocation,
}

impl<'tu, 'g> ReturnTypeWrapper<'tu, 'g> {
	pub fn new(type_ref: TypeRef<'tu, 'g>, gen_env: &'g GeneratorEnv<'tu>, definition_location: DefinitionLocation) -> Self {
		Self { type_ref, gen_env, definition_location }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu, 'g> {
		&self.type_ref
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
