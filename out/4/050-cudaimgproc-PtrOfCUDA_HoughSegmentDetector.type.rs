ptr_extern! { crate::cudaimgproc::CUDA_HoughSegmentDetector,
	cv_PtrLcv_cuda_HoughSegmentDetectorG_new_null_const, cv_PtrLcv_cuda_HoughSegmentDetectorG_delete, cv_PtrLcv_cuda_HoughSegmentDetectorG_getInnerPtr_const, cv_PtrLcv_cuda_HoughSegmentDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline] pub fn as_raw_PtrOfCUDA_HoughSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline] fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline] fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_HoughSegmentDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_HoughSegmentDetector")
			.finish()
	}
}

