ptr_extern! { crate::tracking::TrackerStateEstimatorMILBoosting,
	cv_PtrLcv_TrackerStateEstimatorMILBoostingG_new_null_const, cv_PtrLcv_TrackerStateEstimatorMILBoostingG_delete, cv_PtrLcv_TrackerStateEstimatorMILBoostingG_getInnerPtr_const, cv_PtrLcv_TrackerStateEstimatorMILBoostingG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerStateEstimatorMILBoosting, cv_PtrLcv_TrackerStateEstimatorMILBoostingG_new_const_TrackerStateEstimatorMILBoosting }
impl core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline] pub fn as_raw_PtrOfTrackerStateEstimatorMILBoosting(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerStateEstimatorMILBoosting(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorMILBoostingTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline] fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorMILBoostingTrait for core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline] fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting>, core::Ptr<crate::tracking::TrackerStateEstimator>, cv_PtrLcv_TrackerStateEstimatorMILBoostingG_to_PtrOfTrackerStateEstimator }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerStateEstimatorMILBoosting> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerStateEstimatorMILBoosting")
			.finish()
	}
}

