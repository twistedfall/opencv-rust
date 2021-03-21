use crate::{
	GeneratorEnv,
	TypeRef,
};

#[derive(Debug)]
pub struct AbstractRefWrapper<'tu, 'ge> {
	type_ref: TypeRef<'tu, 'ge>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> AbstractRefWrapper<'tu, 'ge> {
	pub fn new(type_ref: TypeRef<'tu, 'ge>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { type_ref, gen_env }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu, 'ge> {
		&self.type_ref
	}
}
