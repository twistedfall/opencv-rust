ptr_extern! { crate::surface_matching::PoseCluster3D,
	cv_PtrLcv_ppf_match_3d_PoseCluster3DG_new_null_const, cv_PtrLcv_ppf_match_3d_PoseCluster3DG_delete, cv_PtrLcv_ppf_match_3d_PoseCluster3DG_getInnerPtr_const, cv_PtrLcv_ppf_match_3d_PoseCluster3DG_getInnerPtrMut
}

ptr_extern_ctor! { crate::surface_matching::PoseCluster3D, cv_PtrLcv_ppf_match_3d_PoseCluster3DG_new_const_PoseCluster3D }
impl core::Ptr<crate::surface_matching::PoseCluster3D> {
	#[inline] pub fn as_raw_PtrOfPoseCluster3D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::surface_matching::PoseCluster3DTraitConst for core::Ptr<crate::surface_matching::PoseCluster3D> {
	#[inline] fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::surface_matching::PoseCluster3DTrait for core::Ptr<crate::surface_matching::PoseCluster3D> {
	#[inline] fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::surface_matching::PoseCluster3D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPoseCluster3D")
			.field("pose_list", &crate::surface_matching::PoseCluster3DTraitConst::pose_list(self))
			.field("num_votes", &crate::surface_matching::PoseCluster3DTraitConst::num_votes(self))
			.field("id", &crate::surface_matching::PoseCluster3DTraitConst::id(self))
			.finish()
	}
}

