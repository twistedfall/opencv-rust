ptr_extern! { crate::ximgproc::FastBilateralSolverFilter,
	cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_new_null_const, cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_delete, cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline] pub fn as_raw_PtrOfFastBilateralSolverFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastBilateralSolverFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::FastBilateralSolverFilterTraitConst for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline] fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastBilateralSolverFilterTrait for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline] fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::FastBilateralSolverFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFastBilateralSolverFilter")
			.finish()
	}
}

