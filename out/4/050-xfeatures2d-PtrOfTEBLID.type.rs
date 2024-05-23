ptr_extern! { crate::xfeatures2d::TEBLID,
	cv_PtrLcv_xfeatures2d_TEBLIDG_new_null_const, cv_PtrLcv_xfeatures2d_TEBLIDG_delete, cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtrMut
}

ptr_extern_ctor! { crate::xfeatures2d::TEBLID, cv_PtrLcv_xfeatures2d_TEBLIDG_new_const_TEBLID }
impl core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] pub fn as_raw_PtrOfTEBLID(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTEBLID(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::TEBLIDTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_TEBLID(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::TEBLIDTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_mut_TEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::TEBLID>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::TEBLID>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::TEBLID> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTEBLID")
			.finish()
	}
}

