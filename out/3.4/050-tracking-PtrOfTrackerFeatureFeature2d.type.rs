ptr_extern! { crate::tracking::TrackerFeatureFeature2d,
	cv_PtrLcv_TrackerFeatureFeature2dG_new_null_const, cv_PtrLcv_TrackerFeatureFeature2dG_delete, cv_PtrLcv_TrackerFeatureFeature2dG_getInnerPtr_const, cv_PtrLcv_TrackerFeatureFeature2dG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerFeatureFeature2d, cv_PtrLcv_TrackerFeatureFeature2dG_new_const_TrackerFeatureFeature2d }
impl core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline] pub fn as_raw_PtrOfTrackerFeatureFeature2d(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerFeatureFeature2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureFeature2dTraitConst for core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline] fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureFeature2dTrait for core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline] fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerFeatureTraitConst for core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerFeatureFeature2d>, core::Ptr<crate::tracking::TrackerFeature>, cv_PtrLcv_TrackerFeatureFeature2dG_to_PtrOfTrackerFeature }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerFeatureFeature2d> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerFeatureFeature2d")
			.finish()
	}
}

