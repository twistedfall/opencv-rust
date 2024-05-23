ptr_extern! { crate::features::AffineFeature,
	cv_PtrLcv_AffineFeatureG_new_null_const, cv_PtrLcv_AffineFeatureG_delete, cv_PtrLcv_AffineFeatureG_getInnerPtr_const, cv_PtrLcv_AffineFeatureG_getInnerPtrMut
}

impl core::Ptr<crate::features::AffineFeature> {
	#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::AffineFeatureTraitConst for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::AffineFeatureTrait for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::AffineFeature>, core::Ptr<core::Algorithm>, cv_PtrLcv_AffineFeatureG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::features::AffineFeature> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::AffineFeature>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_AffineFeatureG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features::AffineFeature> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAffineFeature")
			.finish()
	}
}

