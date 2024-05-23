ptr_extern! { crate::rgbd::Kinfu_Params,
	cv_PtrLcv_kinfu_ParamsG_new_null_const, cv_PtrLcv_kinfu_ParamsG_delete, cv_PtrLcv_kinfu_ParamsG_getInnerPtr_const, cv_PtrLcv_kinfu_ParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::Kinfu_Params, cv_PtrLcv_kinfu_ParamsG_new_const_Params }
impl core::Ptr<crate::rgbd::Kinfu_Params> {
	#[inline] pub fn as_raw_PtrOfKinfu_Params(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Params(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_ParamsTraitConst for core::Ptr<crate::rgbd::Kinfu_Params> {
	#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_ParamsTrait for core::Ptr<crate::rgbd::Kinfu_Params> {
	#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Kinfu_Params> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKinfu_Params")
			.field("frame_size", &crate::rgbd::Kinfu_ParamsTraitConst::frame_size(self))
			.field("volume_kind", &crate::rgbd::Kinfu_ParamsTraitConst::volume_kind(self))
			.field("intr", &crate::rgbd::Kinfu_ParamsTraitConst::intr(self))
			.field("rgb_intr", &crate::rgbd::Kinfu_ParamsTraitConst::rgb_intr(self))
			.field("depth_factor", &crate::rgbd::Kinfu_ParamsTraitConst::depth_factor(self))
			.field("bilateral_sigma_depth", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_sigma_depth(self))
			.field("bilateral_sigma_spatial", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_sigma_spatial(self))
			.field("bilateral_kernel_size", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_kernel_size(self))
			.field("pyramid_levels", &crate::rgbd::Kinfu_ParamsTraitConst::pyramid_levels(self))
			.field("volume_dims", &crate::rgbd::Kinfu_ParamsTraitConst::volume_dims(self))
			.field("voxel_size", &crate::rgbd::Kinfu_ParamsTraitConst::voxel_size(self))
			.field("tsdf_min_camera_movement", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_min_camera_movement(self))
			.field("volume_pose", &crate::rgbd::Kinfu_ParamsTraitConst::volume_pose(self))
			.field("tsdf_trunc_dist", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_trunc_dist(self))
			.field("tsdf_max_weight", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_max_weight(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_ParamsTraitConst::raycast_step_factor(self))
			.field("light_pose", &crate::rgbd::Kinfu_ParamsTraitConst::light_pose(self))
			.field("icp_dist_thresh", &crate::rgbd::Kinfu_ParamsTraitConst::icp_dist_thresh(self))
			.field("icp_angle_thresh", &crate::rgbd::Kinfu_ParamsTraitConst::icp_angle_thresh(self))
			.field("icp_iterations", &crate::rgbd::Kinfu_ParamsTraitConst::icp_iterations(self))
			.field("truncate_threshold", &crate::rgbd::Kinfu_ParamsTraitConst::truncate_threshold(self))
			.finish()
	}
}

