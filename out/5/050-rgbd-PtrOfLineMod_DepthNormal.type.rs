ptr_extern! { crate::rgbd::LineMod_DepthNormal,
	cv_PtrLcv_linemod_DepthNormalG_new_null_const, cv_PtrLcv_linemod_DepthNormalG_delete, cv_PtrLcv_linemod_DepthNormalG_getInnerPtr_const, cv_PtrLcv_linemod_DepthNormalG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::LineMod_DepthNormal, cv_PtrLcv_linemod_DepthNormalG_new_const_DepthNormal }
impl core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline] pub fn as_raw_PtrOfLineMod_DepthNormal(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineMod_DepthNormal(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LineMod_DepthNormalTraitConst for core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline] fn as_raw_LineMod_DepthNormal(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_DepthNormalTrait for core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline] fn as_raw_mut_LineMod_DepthNormal(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::LineMod_ModalityTraitConst for core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::LineMod_DepthNormal>, core::Ptr<crate::rgbd::LineMod_Modality>, cv_PtrLcv_linemod_DepthNormalG_to_PtrOfLineMod_Modality }

impl std::fmt::Debug for core::Ptr<crate::rgbd::LineMod_DepthNormal> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineMod_DepthNormal")
			.field("distance_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::distance_threshold(self))
			.field("difference_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::difference_threshold(self))
			.field("num_features", &crate::rgbd::LineMod_DepthNormalTraitConst::num_features(self))
			.field("extract_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::extract_threshold(self))
			.finish()
	}
}

