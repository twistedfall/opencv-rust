ptr_extern! { crate::tracking::TrackerStateEstimatorAdaBoosting,
	cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_new_null_const, cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_delete, cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_getInnerPtr_const, cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerStateEstimatorAdaBoosting, cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_new_const_TrackerStateEstimatorAdaBoosting }
impl core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline] pub fn as_raw_PtrOfTrackerStateEstimatorAdaBoosting(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerStateEstimatorAdaBoosting(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorAdaBoostingTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline] fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorAdaBoostingTrait for core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline] fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting>, core::Ptr<crate::tracking::TrackerStateEstimator>, cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_to_PtrOfTrackerStateEstimator }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerStateEstimatorAdaBoosting> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerStateEstimatorAdaBoosting")
			.finish()
	}
}

