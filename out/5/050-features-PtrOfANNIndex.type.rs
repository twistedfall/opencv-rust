ptr_extern! { crate::features::ANNIndex,
	cv_PtrLcv_ANNIndexG_new_null_const, cv_PtrLcv_ANNIndexG_delete, cv_PtrLcv_ANNIndexG_getInnerPtr_const, cv_PtrLcv_ANNIndexG_getInnerPtrMut
}

impl core::Ptr<crate::features::ANNIndex> {
	#[inline] pub fn as_raw_PtrOfANNIndex(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfANNIndex(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::ANNIndexTraitConst for core::Ptr<crate::features::ANNIndex> {
	#[inline] fn as_raw_ANNIndex(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::ANNIndexTrait for core::Ptr<crate::features::ANNIndex> {
	#[inline] fn as_raw_mut_ANNIndex(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::features::ANNIndex> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfANNIndex")
			.finish()
	}
}

