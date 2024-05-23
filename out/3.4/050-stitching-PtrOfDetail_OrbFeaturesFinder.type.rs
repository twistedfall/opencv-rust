ptr_extern! { crate::stitching::Detail_OrbFeaturesFinder,
	cv_PtrLcv_detail_OrbFeaturesFinderG_new_null_const, cv_PtrLcv_detail_OrbFeaturesFinderG_delete, cv_PtrLcv_detail_OrbFeaturesFinderG_getInnerPtr_const, cv_PtrLcv_detail_OrbFeaturesFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_OrbFeaturesFinder, cv_PtrLcv_detail_OrbFeaturesFinderG_new_const_OrbFeaturesFinder }
impl core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_OrbFeaturesFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_OrbFeaturesFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_OrbFeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline] fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_OrbFeaturesFinderTrait for core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_OrbFeaturesFinder>, core::Ptr<crate::stitching::Detail_FeaturesFinder>, cv_PtrLcv_detail_OrbFeaturesFinderG_to_PtrOfDetail_FeaturesFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_OrbFeaturesFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_OrbFeaturesFinder")
			.finish()
	}
}

