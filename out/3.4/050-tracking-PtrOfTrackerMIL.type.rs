ptr_extern! { crate::tracking::TrackerMIL,
	cv_PtrLcv_TrackerMILG_new_null_const, cv_PtrLcv_TrackerMILG_delete, cv_PtrLcv_TrackerMILG_getInnerPtr_const, cv_PtrLcv_TrackerMILG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerMILTraitConst for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerMILTrait for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerMIL>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerMILG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerMIL>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerMILG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerMIL> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerMIL")
			.finish()
	}
}

