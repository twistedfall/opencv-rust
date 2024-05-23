ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
	cv_PtrLcv_detail_BestOf2NearestMatcherG_new_null_const, cv_PtrLcv_detail_BestOf2NearestMatcherG_delete, cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtr_const, cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrLcv_detail_BestOf2NearestMatcherG_new_const_BestOf2NearestMatcher }
impl core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>, core::Ptr<crate::stitching::Detail_FeaturesMatcher>, cv_PtrLcv_detail_BestOf2NearestMatcherG_to_PtrOfDetail_FeaturesMatcher }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BestOf2NearestMatcher")
			.finish()
	}
}

