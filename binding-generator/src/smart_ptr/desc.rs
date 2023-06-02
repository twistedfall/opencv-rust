use crate::type_ref::TypeRef;
use crate::GeneratorEnv;

pub struct SmartPtrDesc<'tu, 'ge> {
	pub pointee_type_ref: TypeRef<'tu, 'ge>,
	pub gen_env: &'ge GeneratorEnv<'tu>,
}
