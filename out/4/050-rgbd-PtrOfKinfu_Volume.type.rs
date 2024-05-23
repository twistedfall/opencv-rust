ptr_extern! { crate::rgbd::Kinfu_Volume,
	cv_PtrLcv_kinfu_VolumeG_new_null_const, cv_PtrLcv_kinfu_VolumeG_delete, cv_PtrLcv_kinfu_VolumeG_getInnerPtr_const, cv_PtrLcv_kinfu_VolumeG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::Kinfu_Volume> {
	#[inline] pub fn as_raw_PtrOfKinfu_Volume(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Volume(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_VolumeTraitConst for core::Ptr<crate::rgbd::Kinfu_Volume> {
	#[inline] fn as_raw_Kinfu_Volume(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_VolumeTrait for core::Ptr<crate::rgbd::Kinfu_Volume> {
	#[inline] fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Kinfu_Volume> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKinfu_Volume")
			.field("voxel_size", &crate::rgbd::Kinfu_VolumeTraitConst::voxel_size(self))
			.field("voxel_size_inv", &crate::rgbd::Kinfu_VolumeTraitConst::voxel_size_inv(self))
			.field("pose", &crate::rgbd::Kinfu_VolumeTraitConst::pose(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_VolumeTraitConst::raycast_step_factor(self))
			.finish()
	}
}

