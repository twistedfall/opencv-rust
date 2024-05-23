ptr_extern! { crate::xfeatures2d::SURF,
	cv_PtrLcv_xfeatures2d_SURFG_new_null_const, cv_PtrLcv_xfeatures2d_SURFG_delete, cv_PtrLcv_xfeatures2d_SURFG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_SURFG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] pub fn as_raw_PtrOfSURF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSURF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::SURFTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::SURFTrait for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::SURF>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_SURFG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::SURF>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_SURFG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::SURF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSURF")
			.finish()
	}
}

