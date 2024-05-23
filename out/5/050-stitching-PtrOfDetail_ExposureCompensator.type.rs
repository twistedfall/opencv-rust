ptr_extern! { crate::stitching::Detail_ExposureCompensator,
	cv_PtrLcv_detail_ExposureCompensatorG_new_null_const, cv_PtrLcv_detail_ExposureCompensatorG_delete, cv_PtrLcv_detail_ExposureCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_ExposureCompensatorG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_ExposureCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ExposureCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_ExposureCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_ExposureCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_ExposureCompensator")
			.finish()
	}
}

