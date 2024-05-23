ptr_extern! { crate::rgbd::Params,
	cv_PtrLcv_large_kinfu_ParamsG_new_null_const, cv_PtrLcv_large_kinfu_ParamsG_delete, cv_PtrLcv_large_kinfu_ParamsG_getInnerPtr_const, cv_PtrLcv_large_kinfu_ParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::Params, cv_PtrLcv_large_kinfu_ParamsG_new_const_Params }
impl core::Ptr<crate::rgbd::Params> {
	#[inline] pub fn as_raw_PtrOfParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::ParamsTraitConst for core::Ptr<crate::rgbd::Params> {
	#[inline] fn as_raw_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ParamsTrait for core::Ptr<crate::rgbd::Params> {
	#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Params> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfParams")
			.field("frame_size", &crate::rgbd::ParamsTraitConst::frame_size(self))
			.field("intr", &crate::rgbd::ParamsTraitConst::intr(self))
			.field("rgb_intr", &crate::rgbd::ParamsTraitConst::rgb_intr(self))
			.field("depth_factor", &crate::rgbd::ParamsTraitConst::depth_factor(self))
			.field("bilateral_sigma_depth", &crate::rgbd::ParamsTraitConst::bilateral_sigma_depth(self))
			.field("bilateral_sigma_spatial", &crate::rgbd::ParamsTraitConst::bilateral_sigma_spatial(self))
			.field("bilateral_kernel_size", &crate::rgbd::ParamsTraitConst::bilateral_kernel_size(self))
			.field("pyramid_levels", &crate::rgbd::ParamsTraitConst::pyramid_levels(self))
			.field("tsdf_min_camera_movement", &crate::rgbd::ParamsTraitConst::tsdf_min_camera_movement(self))
			.field("light_pose", &crate::rgbd::ParamsTraitConst::light_pose(self))
			.field("icp_dist_thresh", &crate::rgbd::ParamsTraitConst::icp_dist_thresh(self))
			.field("icp_angle_thresh", &crate::rgbd::ParamsTraitConst::icp_angle_thresh(self))
			.field("icp_iterations", &crate::rgbd::ParamsTraitConst::icp_iterations(self))
			.field("truncate_threshold", &crate::rgbd::ParamsTraitConst::truncate_threshold(self))
			.field("volume_params", &crate::rgbd::ParamsTraitConst::volume_params(self))
			.finish()
	}
}

