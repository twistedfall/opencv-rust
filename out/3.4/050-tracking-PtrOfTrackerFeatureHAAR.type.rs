ptr_extern! { crate::tracking::TrackerFeatureHAAR,
	cv_PtrLcv_TrackerFeatureHAARG_new_null_const, cv_PtrLcv_TrackerFeatureHAARG_delete, cv_PtrLcv_TrackerFeatureHAARG_getInnerPtr_const, cv_PtrLcv_TrackerFeatureHAARG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerFeatureHAAR, cv_PtrLcv_TrackerFeatureHAARG_new_const_TrackerFeatureHAAR }
impl core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline] pub fn as_raw_PtrOfTrackerFeatureHAAR(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerFeatureHAAR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureHAARTraitConst for core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline] fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureHAARTrait for core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline] fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerFeatureTraitConst for core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerFeatureHAAR>, core::Ptr<crate::tracking::TrackerFeature>, cv_PtrLcv_TrackerFeatureHAARG_to_PtrOfTrackerFeature }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerFeatureHAAR> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerFeatureHAAR")
			.finish()
	}
}

