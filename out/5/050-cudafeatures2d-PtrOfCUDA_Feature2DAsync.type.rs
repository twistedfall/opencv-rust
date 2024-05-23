ptr_extern! { crate::cudafeatures2d::CUDA_Feature2DAsync,
	cv_PtrLcv_cuda_Feature2DAsyncG_new_null_const, cv_PtrLcv_cuda_Feature2DAsyncG_delete, cv_PtrLcv_cuda_Feature2DAsyncG_getInnerPtr_const, cv_PtrLcv_cuda_Feature2DAsyncG_getInnerPtrMut
}

impl core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] pub fn as_raw_PtrOfCUDA_Feature2DAsync(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_Feature2DAsync(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_Feature2DAsyncG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_cuda_Feature2DAsyncG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_Feature2DAsync")
			.finish()
	}
}

