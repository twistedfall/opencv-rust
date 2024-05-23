ptr_extern! { crate::stitching::Detail_ChannelsCompensator,
	cv_PtrLcv_detail_ChannelsCompensatorG_new_null_const, cv_PtrLcv_detail_ChannelsCompensatorG_delete, cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtr_const, cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_ChannelsCompensator, cv_PtrLcv_detail_ChannelsCompensatorG_new_const_ChannelsCompensator }
impl core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline] pub fn as_raw_PtrOfDetail_ChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_ChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_ChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_ChannelsCompensator>, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrLcv_detail_ChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_ChannelsCompensator")
			.finish()
	}
}

