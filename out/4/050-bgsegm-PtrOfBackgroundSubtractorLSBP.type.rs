ptr_extern! { crate::bgsegm::BackgroundSubtractorLSBP,
	cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_new_null_const, cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_delete, cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_getInnerPtr_const, cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_getInnerPtrMut
}

impl core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorLSBP(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorLSBP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorLSBPTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_BackgroundSubtractorLSBP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorLSBPTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_mut_BackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP>, core::Ptr<core::Algorithm>, cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackgroundSubtractorLSBP")
			.finish()
	}
}

