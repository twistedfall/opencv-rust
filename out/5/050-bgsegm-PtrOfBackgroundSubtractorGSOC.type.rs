ptr_extern! { crate::bgsegm::BackgroundSubtractorGSOC,
	cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_new_null_const, cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_delete, cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_getInnerPtr_const, cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_getInnerPtrMut
}

impl core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGSOC(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGSOC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorGSOCTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_BackgroundSubtractorGSOC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorGSOCTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_mut_BackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC>, core::Ptr<core::Algorithm>, cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackgroundSubtractorGSOC")
			.finish()
	}
}

