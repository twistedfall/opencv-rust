ptr_extern! { crate::tracking::TrackerBoosting,
	cv_PtrLcv_TrackerBoostingG_new_null_const, cv_PtrLcv_TrackerBoostingG_delete, cv_PtrLcv_TrackerBoostingG_getInnerPtr_const, cv_PtrLcv_TrackerBoostingG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] pub fn as_raw_PtrOfTrackerBoosting(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerBoosting(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerBoostingTraitConst for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_TrackerBoosting(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerBoostingTrait for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_mut_TrackerBoosting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerBoosting>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerBoostingG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerBoosting>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerBoostingG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerBoosting> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerBoosting")
			.finish()
	}
}

