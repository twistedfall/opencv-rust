ptr_extern! { crate::ximgproc::GuidedFilter,
	cv_PtrLcv_ximgproc_GuidedFilterG_new_null_const, cv_PtrLcv_ximgproc_GuidedFilterG_delete, cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline] pub fn as_raw_PtrOfGuidedFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGuidedFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::GuidedFilterTraitConst for core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline] fn as_raw_GuidedFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::GuidedFilterTrait for core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline] fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::GuidedFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_GuidedFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::GuidedFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGuidedFilter")
			.finish()
	}
}

