ptr_extern! { crate::rgbd::LineMod_ColorGradient,
	cv_PtrLcv_linemod_ColorGradientG_new_null_const, cv_PtrLcv_linemod_ColorGradientG_delete, cv_PtrLcv_linemod_ColorGradientG_getInnerPtr_const, cv_PtrLcv_linemod_ColorGradientG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::LineMod_ColorGradient, cv_PtrLcv_linemod_ColorGradientG_new_const_ColorGradient }
impl core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline] pub fn as_raw_PtrOfLineMod_ColorGradient(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineMod_ColorGradient(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LineMod_ColorGradientTraitConst for core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline] fn as_raw_LineMod_ColorGradient(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_ColorGradientTrait for core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline] fn as_raw_mut_LineMod_ColorGradient(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::LineMod_ModalityTraitConst for core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::LineMod_ColorGradient>, core::Ptr<crate::rgbd::LineMod_Modality>, cv_PtrLcv_linemod_ColorGradientG_to_PtrOfLineMod_Modality }

impl std::fmt::Debug for core::Ptr<crate::rgbd::LineMod_ColorGradient> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineMod_ColorGradient")
			.field("weak_threshold", &crate::rgbd::LineMod_ColorGradientTraitConst::weak_threshold(self))
			.field("num_features", &crate::rgbd::LineMod_ColorGradientTraitConst::num_features(self))
			.field("strong_threshold", &crate::rgbd::LineMod_ColorGradientTraitConst::strong_threshold(self))
			.finish()
	}
}

