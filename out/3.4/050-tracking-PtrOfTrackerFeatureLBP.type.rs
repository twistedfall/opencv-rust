ptr_extern! { crate::tracking::TrackerFeatureLBP,
	cv_PtrLcv_TrackerFeatureLBPG_new_null_const, cv_PtrLcv_TrackerFeatureLBPG_delete, cv_PtrLcv_TrackerFeatureLBPG_getInnerPtr_const, cv_PtrLcv_TrackerFeatureLBPG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerFeatureLBP, cv_PtrLcv_TrackerFeatureLBPG_new_const_TrackerFeatureLBP }
impl core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline] pub fn as_raw_PtrOfTrackerFeatureLBP(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerFeatureLBP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureLBPTraitConst for core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline] fn as_raw_TrackerFeatureLBP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureLBPTrait for core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline] fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerFeatureTraitConst for core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerFeatureLBP>, core::Ptr<crate::tracking::TrackerFeature>, cv_PtrLcv_TrackerFeatureLBPG_to_PtrOfTrackerFeature }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerFeatureLBP> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerFeatureLBP")
			.finish()
	}
}

