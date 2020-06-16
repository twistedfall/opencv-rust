use crate::{
	GeneratorEnv,
	TypeRef,
};

#[derive(Debug)]
pub struct AbstractRefWrapper<'tu> {
	type_ref: TypeRef<'tu>,
	gen_env: &'tu GeneratorEnv<'tu>,
}

impl<'tu> AbstractRefWrapper<'tu> {
	pub fn new(type_ref: TypeRef<'tu>, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self { type_ref, gen_env }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu> {
		&self.type_ref
	}
}
