ptr_extern! { crate::xfeatures2d::BoostDesc,
	cv_PtrLcv_xfeatures2d_BoostDescG_new_null_const, cv_PtrLcv_xfeatures2d_BoostDescG_delete, cv_PtrLcv_xfeatures2d_BoostDescG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_BoostDescG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] pub fn as_raw_PtrOfBoostDesc(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::BoostDescTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BoostDescTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BoostDesc>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_BoostDescG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BoostDesc>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_BoostDescG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::BoostDesc> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBoostDesc")
			.finish()
	}
}

