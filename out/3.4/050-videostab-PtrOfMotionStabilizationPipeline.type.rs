ptr_extern! { crate::videostab::MotionStabilizationPipeline,
	cv_PtrLcv_videostab_MotionStabilizationPipelineG_new_null_const, cv_PtrLcv_videostab_MotionStabilizationPipelineG_delete, cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtr_const, cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MotionStabilizationPipeline, cv_PtrLcv_videostab_MotionStabilizationPipelineG_new_const_MotionStabilizationPipeline }
impl core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline] pub fn as_raw_PtrOfMotionStabilizationPipeline(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionStabilizationPipeline(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionStabilizationPipelineTraitConst for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionStabilizationPipelineTrait for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MotionStabilizationPipeline>, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrLcv_videostab_MotionStabilizationPipelineG_to_PtrOfIMotionStabilizer }

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionStabilizationPipeline")
			.finish()
	}
}

