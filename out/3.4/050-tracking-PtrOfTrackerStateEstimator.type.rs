ptr_extern! { crate::tracking::TrackerStateEstimator,
	cv_PtrLcv_TrackerStateEstimatorG_new_null_const, cv_PtrLcv_TrackerStateEstimatorG_delete, cv_PtrLcv_TrackerStateEstimatorG_getInnerPtr_const, cv_PtrLcv_TrackerStateEstimatorG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerStateEstimator> {
	#[inline] pub fn as_raw_PtrOfTrackerStateEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerStateEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorTraitConst for core::Ptr<crate::tracking::TrackerStateEstimator> {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for core::Ptr<crate::tracking::TrackerStateEstimator> {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerStateEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerStateEstimator")
			.finish()
	}
}

