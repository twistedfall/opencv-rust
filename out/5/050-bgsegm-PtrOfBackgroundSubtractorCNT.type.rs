ptr_extern! { crate::bgsegm::BackgroundSubtractorCNT,
	cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_new_null_const, cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_delete, cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_getInnerPtr_const, cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_getInnerPtrMut
}

impl core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorCNT(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorCNT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorCNTTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_BackgroundSubtractorCNT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorCNTTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_mut_BackgroundSubtractorCNT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorCNT>, core::Ptr<core::Algorithm>, cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::BackgroundSubtractorCNT>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackgroundSubtractorCNT")
			.finish()
	}
}

