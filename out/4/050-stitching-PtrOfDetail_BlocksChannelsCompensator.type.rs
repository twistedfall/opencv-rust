ptr_extern! { crate::stitching::Detail_BlocksChannelsCompensator,
	cv_PtrLcv_detail_BlocksChannelsCompensatorG_new_null_const, cv_PtrLcv_detail_BlocksChannelsCompensatorG_delete, cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BlocksChannelsCompensator, cv_PtrLcv_detail_BlocksChannelsCompensatorG_new_const_BlocksChannelsCompensator }
impl core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_BlocksChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlocksChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_Detail_BlocksChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlocksChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_mut_Detail_BlocksChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BlocksCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlocksCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator>, core::Ptr<crate::stitching::Detail_BlocksCompensator>, cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_BlocksCompensator }

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BlocksChannelsCompensator")
			.finish()
	}
}

