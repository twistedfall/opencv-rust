ptr_extern! { crate::cudaimgproc::CUDA_TemplateMatching,
	cv_PtrLcv_cuda_TemplateMatchingG_new_null_const, cv_PtrLcv_cuda_TemplateMatchingG_delete, cv_PtrLcv_cuda_TemplateMatchingG_getInnerPtr_const, cv_PtrLcv_cuda_TemplateMatchingG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline] pub fn as_raw_PtrOfCUDA_TemplateMatching(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_TemplateMatching(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_TemplateMatchingTraitConst for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline] fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_TemplateMatchingTrait for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline] fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_TemplateMatchingG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_TemplateMatching")
			.finish()
	}
}

