ptr_extern! { crate::cudaobjdetect::CUDA_HOG,
	cv_PtrLcv_cuda_HOGG_new_null_const, cv_PtrLcv_cuda_HOGG_delete, cv_PtrLcv_cuda_HOGG_getInnerPtr_const, cv_PtrLcv_cuda_HOGG_getInnerPtrMut
}

impl core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline] pub fn as_raw_PtrOfCUDA_HOG(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_HOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaobjdetect::CUDA_HOGTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline] fn as_raw_CUDA_HOG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaobjdetect::CUDA_HOGTrait for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline] fn as_raw_mut_CUDA_HOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaobjdetect::CUDA_HOG>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_HOGG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_HOG")
			.finish()
	}
}

