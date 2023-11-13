use crate::type_ref::TypeRef;

pub struct SmartPtrDesc<'tu, 'ge> {
	pub pointee_type_ref: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> SmartPtrDesc<'tu, 'ge> {
	pub fn new(pointee_type_ref: TypeRef<'tu, 'ge>) -> Self {
		Self { pointee_type_ref }
	}
}
