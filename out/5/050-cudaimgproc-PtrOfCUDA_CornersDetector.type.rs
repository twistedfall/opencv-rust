ptr_extern! { crate::cudaimgproc::CUDA_CornersDetector,
	cv_PtrLcv_cuda_CornersDetectorG_new_null_const, cv_PtrLcv_cuda_CornersDetectorG_delete, cv_PtrLcv_cuda_CornersDetectorG_getInnerPtr_const, cv_PtrLcv_cuda_CornersDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline] pub fn as_raw_PtrOfCUDA_CornersDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_CornersDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaimgproc::CUDA_CornersDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline] fn as_raw_CUDA_CornersDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaimgproc::CUDA_CornersDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline] fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaimgproc::CUDA_CornersDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_CornersDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_CornersDetector")
			.finish()
	}
}

