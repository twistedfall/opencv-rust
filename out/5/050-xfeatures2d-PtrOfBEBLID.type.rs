ptr_extern! { crate::xfeatures2d::BEBLID,
	cv_PtrLcv_xfeatures2d_BEBLIDG_new_null_const, cv_PtrLcv_xfeatures2d_BEBLIDG_delete, cv_PtrLcv_xfeatures2d_BEBLIDG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_BEBLIDG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] pub fn as_raw_PtrOfBEBLID(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBEBLID(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::BEBLIDTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BEBLIDTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BEBLID>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_BEBLIDG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BEBLID>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_BEBLIDG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::BEBLID> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBEBLID")
			.finish()
	}
}

