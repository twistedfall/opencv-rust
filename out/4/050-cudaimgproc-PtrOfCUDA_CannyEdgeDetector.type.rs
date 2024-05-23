ptr_extern! { crate::cudaimgproc::CUDA_CannyEdgeDetector,
	cv_PtrLcv_cuda_CannyEdgeDetectorG_new_null_const, cv_PtrLcv_cuda_CannyEdgeDetectorG_delete, cv_PtrLcv_cuda_CannyEdgeDetectorG_getInnerPtr_const, cv_PtrLcv_cuda_CannyEdgeDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline] pub fn as_raw_PtrOfCUDA_CannyEdgeDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_CannyEdgeDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline] fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline] fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_CannyEdgeDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_CannyEdgeDetector")
			.finish()
	}
}

