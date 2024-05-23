ptr_extern! { crate::tracking::TrackerStateEstimatorSVM,
	cv_PtrLcv_TrackerStateEstimatorSVMG_new_null_const, cv_PtrLcv_TrackerStateEstimatorSVMG_delete, cv_PtrLcv_TrackerStateEstimatorSVMG_getInnerPtr_const, cv_PtrLcv_TrackerStateEstimatorSVMG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerStateEstimatorSVM, cv_PtrLcv_TrackerStateEstimatorSVMG_new_const_TrackerStateEstimatorSVM }
impl core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline] pub fn as_raw_PtrOfTrackerStateEstimatorSVM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerStateEstimatorSVM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorSVMTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline] fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorSVMTrait for core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline] fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorTraitConst for core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerStateEstimatorSVM>, core::Ptr<crate::tracking::TrackerStateEstimator>, cv_PtrLcv_TrackerStateEstimatorSVMG_to_PtrOfTrackerStateEstimator }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerStateEstimatorSVM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerStateEstimatorSVM")
			.finish()
	}
}

