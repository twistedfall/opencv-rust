ptr_extern! { crate::rgbd::Kinfu_VolumeParams,
	cv_PtrLcv_kinfu_VolumeParamsG_new_null_const, cv_PtrLcv_kinfu_VolumeParamsG_delete, cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtr_const, cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::Kinfu_VolumeParams, cv_PtrLcv_kinfu_VolumeParamsG_new_const_VolumeParams }
impl core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
	#[inline] pub fn as_raw_PtrOfKinfu_VolumeParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_VolumeParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_VolumeParamsTraitConst for core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
	#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_VolumeParamsTrait for core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
	#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKinfu_VolumeParams")
			.field("typ", &crate::rgbd::Kinfu_VolumeParamsTraitConst::typ(self))
			.field("resolution", &crate::rgbd::Kinfu_VolumeParamsTraitConst::resolution(self))
			.field("unit_resolution", &crate::rgbd::Kinfu_VolumeParamsTraitConst::unit_resolution(self))
			.field("pose", &crate::rgbd::Kinfu_VolumeParamsTraitConst::pose(self))
			.field("voxel_size", &crate::rgbd::Kinfu_VolumeParamsTraitConst::voxel_size(self))
			.field("tsdf_trunc_dist", &crate::rgbd::Kinfu_VolumeParamsTraitConst::tsdf_trunc_dist(self))
			.field("max_weight", &crate::rgbd::Kinfu_VolumeParamsTraitConst::max_weight(self))
			.field("depth_trunc_threshold", &crate::rgbd::Kinfu_VolumeParamsTraitConst::depth_trunc_threshold(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_VolumeParamsTraitConst::raycast_step_factor(self))
			.finish()
	}
}

