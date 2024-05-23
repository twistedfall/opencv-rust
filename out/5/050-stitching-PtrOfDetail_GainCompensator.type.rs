ptr_extern! { crate::stitching::Detail_GainCompensator,
	cv_PtrLcv_detail_GainCompensatorG_new_null_const, cv_PtrLcv_detail_GainCompensatorG_delete, cv_PtrLcv_detail_GainCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_GainCompensatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_GainCompensator, cv_PtrLcv_detail_GainCompensatorG_new_const_GainCompensator }
impl core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_GainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_GainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GainCompensatorTrait for core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GainCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_GainCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_GainCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_GainCompensator")
			.finish()
	}
}

