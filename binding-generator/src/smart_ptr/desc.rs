use crate::type_ref::TypeRef;

pub struct SmartPtrDesc<'tu, 'ge> {
	pub pointee_type_ref: TypeRef<'tu, 'ge>,
}
