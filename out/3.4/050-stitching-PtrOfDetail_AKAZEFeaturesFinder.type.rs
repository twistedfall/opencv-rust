ptr_extern! { crate::stitching::Detail_AKAZEFeaturesFinder,
	cv_PtrLcv_detail_AKAZEFeaturesFinderG_new_null_const, cv_PtrLcv_detail_AKAZEFeaturesFinderG_delete, cv_PtrLcv_detail_AKAZEFeaturesFinderG_getInnerPtr_const, cv_PtrLcv_detail_AKAZEFeaturesFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_AKAZEFeaturesFinder, cv_PtrLcv_detail_AKAZEFeaturesFinderG_new_const_AKAZEFeaturesFinder }
impl core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_AKAZEFeaturesFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_AKAZEFeaturesFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_AKAZEFeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline] fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_AKAZEFeaturesFinderTrait for core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder>, core::Ptr<crate::stitching::Detail_FeaturesFinder>, cv_PtrLcv_detail_AKAZEFeaturesFinderG_to_PtrOfDetail_FeaturesFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_AKAZEFeaturesFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_AKAZEFeaturesFinder")
			.finish()
	}
}

