ptr_extern! { crate::cudaimgproc::CUDA_CornernessCriteria,
	cv_PtrLcv_cuda_CornernessCriteriaG_new_null_const, cv_PtrLcv_cuda_CornernessCriteriaG_delete, cv_PtrLcv_cuda_CornernessCriteriaG_getInnerPtr_const, cv_PtrLcv_cuda_CornernessCriteriaG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline] pub fn as_raw_PtrOfCUDA_CornernessCriteria(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_CornernessCriteria(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_CornernessCriteriaTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline] fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_CornernessCriteriaTrait for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline] fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_CornernessCriteriaG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_CornernessCriteria")
			.finish()
	}
}

