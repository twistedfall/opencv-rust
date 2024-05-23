ptr_extern! { crate::cudafeatures2d::CUDA_ORB,
	cv_PtrLcv_cuda_ORBG_new_null_const, cv_PtrLcv_cuda_ORBG_delete, cv_PtrLcv_cuda_ORBG_getInnerPtr_const, cv_PtrLcv_cuda_ORBG_getInnerPtrMut
}

impl core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] pub fn as_raw_PtrOfCUDA_ORB(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_ORB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudafeatures2d::CUDA_ORBTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_CUDA_ORB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudafeatures2d::CUDA_ORBTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_ORB>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_ORBG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_ORB>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_cuda_ORBG_to_PtrOfFeature2D }

impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_ORB>, core::Ptr<crate::cudafeatures2d::CUDA_Feature2DAsync>, cv_PtrLcv_cuda_ORBG_to_PtrOfCUDA_Feature2DAsync }

impl std::fmt::Debug for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_ORB")
			.finish()
	}
}

