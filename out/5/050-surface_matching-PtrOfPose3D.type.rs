ptr_extern! { crate::surface_matching::Pose3D,
	cv_PtrLcv_ppf_match_3d_Pose3DG_new_null_const, cv_PtrLcv_ppf_match_3d_Pose3DG_delete, cv_PtrLcv_ppf_match_3d_Pose3DG_getInnerPtr_const, cv_PtrLcv_ppf_match_3d_Pose3DG_getInnerPtrMut
}

ptr_extern_ctor! { crate::surface_matching::Pose3D, cv_PtrLcv_ppf_match_3d_Pose3DG_new_const_Pose3D }
impl core::Ptr<crate::surface_matching::Pose3D> {
	#[inline] pub fn as_raw_PtrOfPose3D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPose3D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::surface_matching::Pose3DTraitConst for core::Ptr<crate::surface_matching::Pose3D> {
	#[inline] fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::surface_matching::Pose3DTrait for core::Ptr<crate::surface_matching::Pose3D> {
	#[inline] fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::surface_matching::Pose3D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPose3D")
			.field("alpha", &crate::surface_matching::Pose3DTraitConst::alpha(self))
			.field("residual", &crate::surface_matching::Pose3DTraitConst::residual(self))
			.field("model_index", &crate::surface_matching::Pose3DTraitConst::model_index(self))
			.field("num_votes", &crate::surface_matching::Pose3DTraitConst::num_votes(self))
			.field("pose", &crate::surface_matching::Pose3DTraitConst::pose(self))
			.field("angle", &crate::surface_matching::Pose3DTraitConst::angle(self))
			.field("t", &crate::surface_matching::Pose3DTraitConst::t(self))
			.field("q", &crate::surface_matching::Pose3DTraitConst::q(self))
			.finish()
	}
}

