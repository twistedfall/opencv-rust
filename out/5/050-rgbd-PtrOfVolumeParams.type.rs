ptr_extern! { crate::rgbd::VolumeParams,
	cv_PtrLcv_large_kinfu_VolumeParamsG_new_null_const, cv_PtrLcv_large_kinfu_VolumeParamsG_delete, cv_PtrLcv_large_kinfu_VolumeParamsG_getInnerPtr_const, cv_PtrLcv_large_kinfu_VolumeParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::VolumeParams, cv_PtrLcv_large_kinfu_VolumeParamsG_new_const_VolumeParams }
impl core::Ptr<crate::rgbd::VolumeParams> {
	#[inline] pub fn as_raw_PtrOfVolumeParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVolumeParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::VolumeParamsTraitConst for core::Ptr<crate::rgbd::VolumeParams> {
	#[inline] fn as_raw_VolumeParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::VolumeParamsTrait for core::Ptr<crate::rgbd::VolumeParams> {
	#[inline] fn as_raw_mut_VolumeParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::VolumeParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfVolumeParams")
			.field("kind", &crate::rgbd::VolumeParamsTraitConst::kind(self))
			.field("resolution_x", &crate::rgbd::VolumeParamsTraitConst::resolution_x(self))
			.field("resolution_y", &crate::rgbd::VolumeParamsTraitConst::resolution_y(self))
			.field("resolution_z", &crate::rgbd::VolumeParamsTraitConst::resolution_z(self))
			.field("unit_resolution", &crate::rgbd::VolumeParamsTraitConst::unit_resolution(self))
			.field("volum_size", &crate::rgbd::VolumeParamsTraitConst::volum_size(self))
			.field("pose", &crate::rgbd::VolumeParamsTraitConst::pose(self))
			.field("voxel_size", &crate::rgbd::VolumeParamsTraitConst::voxel_size(self))
			.field("tsdf_trunc_dist", &crate::rgbd::VolumeParamsTraitConst::tsdf_trunc_dist(self))
			.field("max_weight", &crate::rgbd::VolumeParamsTraitConst::max_weight(self))
			.field("depth_trunc_threshold", &crate::rgbd::VolumeParamsTraitConst::depth_trunc_threshold(self))
			.field("raycast_step_factor", &crate::rgbd::VolumeParamsTraitConst::raycast_step_factor(self))
			.finish()
	}
}

