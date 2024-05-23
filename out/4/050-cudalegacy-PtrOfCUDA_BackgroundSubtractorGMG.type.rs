ptr_extern! { crate::cudalegacy::CUDA_BackgroundSubtractorGMG,
	cv_PtrLcv_cuda_BackgroundSubtractorGMGG_new_null_const, cv_PtrLcv_cuda_BackgroundSubtractorGMGG_delete, cv_PtrLcv_cuda_BackgroundSubtractorGMGG_getInnerPtr_const, cv_PtrLcv_cuda_BackgroundSubtractorGMGG_getInnerPtrMut
}

impl core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorGMG(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorGMG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudalegacy::CUDA_BackgroundSubtractorGMGTraitConst for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_CUDA_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudalegacy::CUDA_BackgroundSubtractorGMGTrait for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_BackgroundSubtractorGMGG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_cuda_BackgroundSubtractorGMGG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_BackgroundSubtractorGMG")
			.finish()
	}
}

