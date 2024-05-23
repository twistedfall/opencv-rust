ptr_extern! { crate::stitching::Detail_FeaturesMatcher,
	cv_PtrLcv_detail_FeaturesMatcherG_new_null_const, cv_PtrLcv_detail_FeaturesMatcherG_delete, cv_PtrLcv_detail_FeaturesMatcherG_getInnerPtr_const, cv_PtrLcv_detail_FeaturesMatcherG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
	#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_FeaturesMatcher")
			.finish()
	}
}

