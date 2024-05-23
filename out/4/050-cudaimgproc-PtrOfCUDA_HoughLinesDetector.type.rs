ptr_extern! { crate::cudaimgproc::CUDA_HoughLinesDetector,
	cv_PtrLcv_cuda_HoughLinesDetectorG_new_null_const, cv_PtrLcv_cuda_HoughLinesDetectorG_delete, cv_PtrLcv_cuda_HoughLinesDetectorG_getInnerPtr_const, cv_PtrLcv_cuda_HoughLinesDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline] pub fn as_raw_PtrOfCUDA_HoughLinesDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughLinesDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_HoughLinesDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline] fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_HoughLinesDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline] fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_HoughLinesDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_HoughLinesDetector")
			.finish()
	}
}

