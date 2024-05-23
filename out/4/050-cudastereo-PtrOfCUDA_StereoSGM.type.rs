ptr_extern! { crate::cudastereo::CUDA_StereoSGM,
	cv_PtrLcv_cuda_StereoSGMG_new_null_const, cv_PtrLcv_cuda_StereoSGMG_delete, cv_PtrLcv_cuda_StereoSGMG_getInnerPtr_const, cv_PtrLcv_cuda_StereoSGMG_getInnerPtrMut
}

impl core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] pub fn as_raw_PtrOfCUDA_StereoSGM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoSGM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudastereo::CUDA_StereoSGMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_CUDA_StereoSGM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudastereo::CUDA_StereoSGMTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_mut_CUDA_StereoSGM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoSGM>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_StereoSGMG_to_PtrOfAlgorithm }

impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoSGM>, core::Ptr<crate::calib3d::StereoMatcher>, cv_PtrLcv_cuda_StereoSGMG_to_PtrOfStereoMatcher }

impl crate::calib3d::StereoSGBMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoSGBMTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoSGM>, core::Ptr<crate::calib3d::StereoSGBM>, cv_PtrLcv_cuda_StereoSGMG_to_PtrOfStereoSGBM }

impl std::fmt::Debug for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_StereoSGM")
			.finish()
	}
}

