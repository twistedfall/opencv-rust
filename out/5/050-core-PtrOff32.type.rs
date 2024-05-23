ptr_extern! { f32,
	cv_PtrLfloatG_new_null_const, cv_PtrLfloatG_delete, cv_PtrLfloatG_getInnerPtr_const, cv_PtrLfloatG_getInnerPtrMut
}

ptr_extern_ctor! { f32, cv_PtrLfloatG_new_const_float }
impl core::Ptr<f32> {
	#[inline] pub fn as_raw_PtrOff32(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

