ptr_extern! { crate::tracking::TrackerFeature,
	cv_PtrLcv_TrackerFeatureG_new_null_const, cv_PtrLcv_TrackerFeatureG_delete, cv_PtrLcv_TrackerFeatureG_getInnerPtr_const, cv_PtrLcv_TrackerFeatureG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerFeature> {
	#[inline] pub fn as_raw_PtrOfTrackerFeature(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerFeature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureTraitConst for core::Ptr<crate::tracking::TrackerFeature> {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for core::Ptr<crate::tracking::TrackerFeature> {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerFeature> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerFeature")
			.finish()
	}
}

