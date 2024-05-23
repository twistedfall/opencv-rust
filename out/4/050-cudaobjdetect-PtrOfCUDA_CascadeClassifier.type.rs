ptr_extern! { crate::cudaobjdetect::CUDA_CascadeClassifier,
	cv_PtrLcv_cuda_CascadeClassifierG_new_null_const, cv_PtrLcv_cuda_CascadeClassifierG_delete, cv_PtrLcv_cuda_CascadeClassifierG_getInnerPtr_const, cv_PtrLcv_cuda_CascadeClassifierG_getInnerPtrMut
}

impl core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline] pub fn as_raw_PtrOfCUDA_CascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_CascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaobjdetect::CUDA_CascadeClassifierTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline] fn as_raw_CUDA_CascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaobjdetect::CUDA_CascadeClassifierTrait for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline] fn as_raw_mut_CUDA_CascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_CascadeClassifierG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_CascadeClassifier")
			.finish()
	}
}

