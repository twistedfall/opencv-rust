ptr_extern! { crate::stitching::Detail_BlocksCompensator,
	cv_PtrLcv_detail_BlocksCompensatorG_new_null_const, cv_PtrLcv_detail_BlocksCompensatorG_delete, cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_BlocksCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlocksCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlocksCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BlocksCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_BlocksCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BlocksCompensator")
			.finish()
	}
}

