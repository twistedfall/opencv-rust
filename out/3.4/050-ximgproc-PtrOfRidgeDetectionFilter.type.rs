ptr_extern! { crate::ximgproc::RidgeDetectionFilter,
	cv_PtrLcv_ximgproc_RidgeDetectionFilterG_new_null_const, cv_PtrLcv_ximgproc_RidgeDetectionFilterG_delete, cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline] pub fn as_raw_PtrOfRidgeDetectionFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRidgeDetectionFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::RidgeDetectionFilterTraitConst for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline] fn as_raw_RidgeDetectionFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RidgeDetectionFilterTrait for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline] fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::RidgeDetectionFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_RidgeDetectionFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRidgeDetectionFilter")
			.finish()
	}
}

