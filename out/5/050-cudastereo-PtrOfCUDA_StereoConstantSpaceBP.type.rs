ptr_extern! { crate::cudastereo::CUDA_StereoConstantSpaceBP,
	cv_PtrLcv_cuda_StereoConstantSpaceBPG_new_null_const, cv_PtrLcv_cuda_StereoConstantSpaceBPG_delete, cv_PtrLcv_cuda_StereoConstantSpaceBPG_getInnerPtr_const, cv_PtrLcv_cuda_StereoConstantSpaceBPG_getInnerPtrMut
}

impl core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] pub fn as_raw_PtrOfCUDA_StereoConstantSpaceBP(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoConstantSpaceBP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudastereo::CUDA_StereoConstantSpaceBPTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_CUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudastereo::CUDA_StereoConstantSpaceBPTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_mut_CUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfAlgorithm }

impl crate::stereo::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stereo::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP>, core::Ptr<crate::stereo::StereoMatcher>, cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfStereoMatcher }

impl crate::cudastereo::CUDA_StereoBeliefPropagationTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudastereo::CUDA_StereoBeliefPropagationTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP>, core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation>, cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfCUDA_StereoBeliefPropagation }

impl std::fmt::Debug for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_StereoConstantSpaceBP")
			.finish()
	}
}

