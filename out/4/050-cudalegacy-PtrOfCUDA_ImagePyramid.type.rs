ptr_extern! { crate::cudalegacy::CUDA_ImagePyramid,
	cv_PtrLcv_cuda_ImagePyramidG_new_null_const, cv_PtrLcv_cuda_ImagePyramidG_delete, cv_PtrLcv_cuda_ImagePyramidG_getInnerPtr_const, cv_PtrLcv_cuda_ImagePyramidG_getInnerPtrMut
}

impl core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline] pub fn as_raw_PtrOfCUDA_ImagePyramid(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_ImagePyramid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudalegacy::CUDA_ImagePyramidTraitConst for core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline] fn as_raw_CUDA_ImagePyramid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudalegacy::CUDA_ImagePyramidTrait for core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline] fn as_raw_mut_CUDA_ImagePyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudalegacy::CUDA_ImagePyramid>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_ImagePyramidG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudalegacy::CUDA_ImagePyramid> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_ImagePyramid")
			.finish()
	}
}

