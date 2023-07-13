use crate::type_ref::TypeRef;

#[derive(Clone)]
pub struct VectorDesc<'tu, 'ge> {
	pub element_type: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> VectorDesc<'tu, 'ge> {
	pub fn new(element_type: TypeRef<'tu, 'ge>) -> Self {
		Self { element_type }
	}
}
