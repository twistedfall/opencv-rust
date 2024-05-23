ptr_extern! { crate::ximgproc::DisparityFilter,
	cv_PtrLcv_ximgproc_DisparityFilterG_new_null_const, cv_PtrLcv_ximgproc_DisparityFilterG_delete, cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline] pub fn as_raw_PtrOfDisparityFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDisparityFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::DisparityFilterTraitConst for core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DisparityFilterTrait for core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::DisparityFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_DisparityFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::DisparityFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDisparityFilter")
			.finish()
	}
}

