ptr_extern! { crate::cudastereo::CUDA_StereoBeliefPropagation,
	cv_PtrLcv_cuda_StereoBeliefPropagationG_new_null_const, cv_PtrLcv_cuda_StereoBeliefPropagationG_delete, cv_PtrLcv_cuda_StereoBeliefPropagationG_getInnerPtr_const, cv_PtrLcv_cuda_StereoBeliefPropagationG_getInnerPtrMut
}

impl core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] pub fn as_raw_PtrOfCUDA_StereoBeliefPropagation(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBeliefPropagation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudastereo::CUDA_StereoBeliefPropagationTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudastereo::CUDA_StereoBeliefPropagationTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_StereoBeliefPropagationG_to_PtrOfAlgorithm }

impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation>, core::Ptr<crate::calib3d::StereoMatcher>, cv_PtrLcv_cuda_StereoBeliefPropagationG_to_PtrOfStereoMatcher }

impl std::fmt::Debug for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_StereoBeliefPropagation")
			.finish()
	}
}

