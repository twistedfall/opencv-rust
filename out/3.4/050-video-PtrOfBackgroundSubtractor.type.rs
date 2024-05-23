ptr_extern! { crate::video::BackgroundSubtractor,
	cv_PtrLcv_BackgroundSubtractorG_new_null_const, cv_PtrLcv_BackgroundSubtractorG_delete, cv_PtrLcv_BackgroundSubtractorG_getInnerPtr_const, cv_PtrLcv_BackgroundSubtractorG_getInnerPtrMut
}

impl core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::BackgroundSubtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_BackgroundSubtractorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::video::BackgroundSubtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackgroundSubtractor")
			.finish()
	}
}

