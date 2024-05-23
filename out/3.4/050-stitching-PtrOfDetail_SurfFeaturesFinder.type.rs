ptr_extern! { crate::stitching::Detail_SurfFeaturesFinder,
	cv_PtrLcv_detail_SurfFeaturesFinderG_new_null_const, cv_PtrLcv_detail_SurfFeaturesFinderG_delete, cv_PtrLcv_detail_SurfFeaturesFinderG_getInnerPtr_const, cv_PtrLcv_detail_SurfFeaturesFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_SurfFeaturesFinder, cv_PtrLcv_detail_SurfFeaturesFinderG_new_const_SurfFeaturesFinder }
impl core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_SurfFeaturesFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_SurfFeaturesFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_SurfFeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline] fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SurfFeaturesFinderTrait for core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_SurfFeaturesFinder>, core::Ptr<crate::stitching::Detail_FeaturesFinder>, cv_PtrLcv_detail_SurfFeaturesFinderG_to_PtrOfDetail_FeaturesFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_SurfFeaturesFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_SurfFeaturesFinder")
			.finish()
	}
}

