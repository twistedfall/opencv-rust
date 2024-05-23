ptr_extern! { crate::xfeatures2d::AffineFeature2D,
	cv_PtrLcv_xfeatures2d_AffineFeature2DG_new_null_const, cv_PtrLcv_xfeatures2d_AffineFeature2DG_delete, cv_PtrLcv_xfeatures2d_AffineFeature2DG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_AffineFeature2DG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] pub fn as_raw_PtrOfAffineFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::AffineFeature2DTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::AffineFeature2DTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::AffineFeature2D>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_AffineFeature2DG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::AffineFeature2D>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_AffineFeature2DG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAffineFeature2D")
			.finish()
	}
}

