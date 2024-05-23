ptr_extern! { crate::optflow::RLOFOpticalFlowParameter,
	cv_PtrLcv_optflow_RLOFOpticalFlowParameterG_new_null_const, cv_PtrLcv_optflow_RLOFOpticalFlowParameterG_delete, cv_PtrLcv_optflow_RLOFOpticalFlowParameterG_getInnerPtr_const, cv_PtrLcv_optflow_RLOFOpticalFlowParameterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::optflow::RLOFOpticalFlowParameter, cv_PtrLcv_optflow_RLOFOpticalFlowParameterG_new_const_RLOFOpticalFlowParameter }
impl core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
	#[inline] pub fn as_raw_PtrOfRLOFOpticalFlowParameter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRLOFOpticalFlowParameter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::RLOFOpticalFlowParameterTraitConst for core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
	#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::RLOFOpticalFlowParameterTrait for core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
	#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRLOFOpticalFlowParameter")
			.field("solver_type", &crate::optflow::RLOFOpticalFlowParameterTraitConst::solver_type(self))
			.field("support_region_type", &crate::optflow::RLOFOpticalFlowParameterTraitConst::support_region_type(self))
			.field("norm_sigma0", &crate::optflow::RLOFOpticalFlowParameterTraitConst::norm_sigma0(self))
			.field("norm_sigma1", &crate::optflow::RLOFOpticalFlowParameterTraitConst::norm_sigma1(self))
			.field("small_win_size", &crate::optflow::RLOFOpticalFlowParameterTraitConst::small_win_size(self))
			.field("large_win_size", &crate::optflow::RLOFOpticalFlowParameterTraitConst::large_win_size(self))
			.field("cross_segmentation_threshold", &crate::optflow::RLOFOpticalFlowParameterTraitConst::cross_segmentation_threshold(self))
			.field("max_level", &crate::optflow::RLOFOpticalFlowParameterTraitConst::max_level(self))
			.field("use_initial_flow", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_initial_flow(self))
			.field("use_illumination_model", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_illumination_model(self))
			.field("use_global_motion_prior", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_global_motion_prior(self))
			.field("max_iteration", &crate::optflow::RLOFOpticalFlowParameterTraitConst::max_iteration(self))
			.field("min_eigen_value", &crate::optflow::RLOFOpticalFlowParameterTraitConst::min_eigen_value(self))
			.field("global_motion_ransac_threshold", &crate::optflow::RLOFOpticalFlowParameterTraitConst::global_motion_ransac_threshold(self))
			.finish()
	}
}

