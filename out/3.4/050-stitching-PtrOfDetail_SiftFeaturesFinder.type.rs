ptr_extern! { crate::stitching::Detail_SiftFeaturesFinder,
	cv_PtrLcv_detail_SiftFeaturesFinderG_new_null_const, cv_PtrLcv_detail_SiftFeaturesFinderG_delete, cv_PtrLcv_detail_SiftFeaturesFinderG_getInnerPtr_const, cv_PtrLcv_detail_SiftFeaturesFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_SiftFeaturesFinder, cv_PtrLcv_detail_SiftFeaturesFinderG_new_const_SiftFeaturesFinder }
impl core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_SiftFeaturesFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_SiftFeaturesFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_SiftFeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline] fn as_raw_Detail_SiftFeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SiftFeaturesFinderTrait for core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_SiftFeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_SiftFeaturesFinder>, core::Ptr<crate::stitching::Detail_FeaturesFinder>, cv_PtrLcv_detail_SiftFeaturesFinderG_to_PtrOfDetail_FeaturesFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_SiftFeaturesFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_SiftFeaturesFinder")
			.finish()
	}
}

