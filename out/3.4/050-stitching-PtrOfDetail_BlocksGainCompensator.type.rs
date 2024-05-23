ptr_extern! { crate::stitching::Detail_BlocksGainCompensator,
	cv_PtrLcv_detail_BlocksGainCompensatorG_new_null_const, cv_PtrLcv_detail_BlocksGainCompensatorG_delete, cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BlocksGainCompensator, cv_PtrLcv_detail_BlocksGainCompensatorG_new_const_BlocksGainCompensator }
impl core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_BlocksGainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksGainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlocksGainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlocksGainCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BlocksGainCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_BlocksGainCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BlocksGainCompensator")
			.finish()
	}
}

