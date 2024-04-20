use crate::type_ref::{TypeRef, TypeRefDesc};
use crate::vector::Vector;

#[derive(Clone)]
pub struct VectorDesc<'tu, 'ge> {
	pub element_type: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> VectorDesc<'tu, 'ge> {
	pub fn new(element_type: TypeRef<'tu, 'ge>) -> Self {
		Self { element_type }
	}

	pub fn vector_of_cv_string() -> Vector<'static, 'static> {
		Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_string()))
	}
}
