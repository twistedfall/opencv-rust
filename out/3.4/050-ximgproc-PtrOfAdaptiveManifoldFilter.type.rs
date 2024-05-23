ptr_extern! { crate::ximgproc::AdaptiveManifoldFilter,
	cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_new_null_const, cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_delete, cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline] pub fn as_raw_PtrOfAdaptiveManifoldFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAdaptiveManifoldFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::AdaptiveManifoldFilterTraitConst for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline] fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::AdaptiveManifoldFilterTrait for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline] fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::AdaptiveManifoldFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAdaptiveManifoldFilter")
			.finish()
	}
}

