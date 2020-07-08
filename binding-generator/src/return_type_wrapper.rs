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
pub struct ReturnTypeWrapper<'tu> {
	type_ref: TypeRef<'tu>,
	const_hint: Option<bool>,
	definition_location: DefinitionLocation,
	gen_env: &'tu GeneratorEnv<'tu>,
}

impl<'tu> ReturnTypeWrapper<'tu> {
	pub fn new(type_ref: TypeRef<'tu>, definition_location: DefinitionLocation, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self::new_ext(type_ref, None, definition_location, gen_env)
	}

	pub fn new_ext(type_ref: TypeRef<'tu>, const_hint: Option<bool>, definition_location: DefinitionLocation, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self { type_ref, const_hint, definition_location, gen_env }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu> {
		&self.type_ref
	}

	pub fn const_hint(&self) -> Option<bool> {
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
