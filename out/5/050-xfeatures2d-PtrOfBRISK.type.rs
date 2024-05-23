ptr_extern! { crate::xfeatures2d::BRISK,
	cv_PtrLcv_xfeatures2d_BRISKG_new_null_const, cv_PtrLcv_xfeatures2d_BRISKG_delete, cv_PtrLcv_xfeatures2d_BRISKG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_BRISKG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] pub fn as_raw_PtrOfBRISK(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::BRISKTraitConst for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BRISKTrait for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BRISK>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_BRISKG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BRISK>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_BRISKG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::BRISK> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBRISK")
			.finish()
	}
}

