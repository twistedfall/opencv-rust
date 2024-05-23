ptr_extern! { crate::ximgproc::FastGlobalSmootherFilter,
	cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_new_null_const, cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_delete, cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline] pub fn as_raw_PtrOfFastGlobalSmootherFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastGlobalSmootherFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::FastGlobalSmootherFilterTraitConst for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline] fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastGlobalSmootherFilterTrait for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline] fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::FastGlobalSmootherFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFastGlobalSmootherFilter")
			.finish()
	}
}

