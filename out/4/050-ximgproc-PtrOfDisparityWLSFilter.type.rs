ptr_extern! { crate::ximgproc::DisparityWLSFilter,
	cv_PtrLcv_ximgproc_DisparityWLSFilterG_new_null_const, cv_PtrLcv_ximgproc_DisparityWLSFilterG_delete, cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] pub fn as_raw_PtrOfDisparityWLSFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDisparityWLSFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::DisparityWLSFilterTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_DisparityWLSFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DisparityWLSFilterTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::DisparityWLSFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfAlgorithm }

impl crate::ximgproc::DisparityFilterTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DisparityFilterTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::DisparityWLSFilter>, core::Ptr<crate::ximgproc::DisparityFilter>, cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfDisparityFilter }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDisparityWLSFilter")
			.finish()
	}
}

