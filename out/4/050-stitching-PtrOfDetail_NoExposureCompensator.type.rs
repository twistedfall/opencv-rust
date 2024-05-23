ptr_extern! { crate::stitching::Detail_NoExposureCompensator,
	cv_PtrLcv_detail_NoExposureCompensatorG_new_null_const, cv_PtrLcv_detail_NoExposureCompensatorG_delete, cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_NoExposureCompensator, cv_PtrLcv_detail_NoExposureCompensatorG_new_const_NoExposureCompensator }
impl core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_NoExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_NoExposureCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_NoExposureCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_NoExposureCompensator")
			.finish()
	}
}

