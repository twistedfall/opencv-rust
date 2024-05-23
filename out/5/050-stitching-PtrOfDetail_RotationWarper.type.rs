ptr_extern! { crate::stitching::Detail_RotationWarper,
	cv_PtrLcv_detail_RotationWarperG_new_null_const, cv_PtrLcv_detail_RotationWarperG_delete, cv_PtrLcv_detail_RotationWarperG_getInnerPtr_const, cv_PtrLcv_detail_RotationWarperG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_RotationWarper> {
	#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarperTraitConst for core::Ptr<crate::stitching::Detail_RotationWarper> {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for core::Ptr<crate::stitching::Detail_RotationWarper> {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_RotationWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_RotationWarper")
			.finish()
	}
}

