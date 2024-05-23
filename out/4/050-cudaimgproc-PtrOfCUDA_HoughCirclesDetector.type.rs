ptr_extern! { crate::cudaimgproc::CUDA_HoughCirclesDetector,
	cv_PtrLcv_cuda_HoughCirclesDetectorG_new_null_const, cv_PtrLcv_cuda_HoughCirclesDetectorG_delete, cv_PtrLcv_cuda_HoughCirclesDetectorG_getInnerPtr_const, cv_PtrLcv_cuda_HoughCirclesDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline] pub fn as_raw_PtrOfCUDA_HoughCirclesDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughCirclesDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline] fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline] fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_HoughCirclesDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_HoughCirclesDetector")
			.finish()
	}
}

