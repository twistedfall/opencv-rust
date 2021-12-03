use crate::TypeRef;

#[derive(Debug)]
pub struct AbstractRefWrapper<'tu, 'ge> {
	type_ref: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> AbstractRefWrapper<'tu, 'ge> {
	pub fn new(type_ref: TypeRef<'tu, 'ge>) -> Self {
		Self { type_ref }
	}

	pub fn type_ref(&self) -> &TypeRef<'tu, 'ge> {
		&self.type_ref
	}
}
