ptr_extern! { crate::stitching::Detail_FeaturesFinder,
	cv_PtrLcv_detail_FeaturesFinderG_new_null_const, cv_PtrLcv_detail_FeaturesFinderG_delete, cv_PtrLcv_detail_FeaturesFinderG_getInnerPtr_const, cv_PtrLcv_detail_FeaturesFinderG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_FeaturesFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_FeaturesFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_FeaturesFinder> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_FeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_FeaturesFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_FeaturesFinder")
			.finish()
	}
}

