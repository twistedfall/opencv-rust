ptr_extern! { crate::xfeatures2d::TBMR,
	cv_PtrLcv_xfeatures2d_TBMRG_new_null_const, cv_PtrLcv_xfeatures2d_TBMRG_delete, cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] pub fn as_raw_PtrOfTBMR(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTBMR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::TBMRTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_TBMR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::TBMRTrait for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_mut_TBMR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::TBMR>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::TBMR>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfFeature2D }

impl crate::xfeatures2d::AffineFeature2DTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::AffineFeature2DTrait for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::TBMR>, core::Ptr<crate::xfeatures2d::AffineFeature2D>, cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAffineFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::TBMR> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTBMR")
			.finish()
	}
}

