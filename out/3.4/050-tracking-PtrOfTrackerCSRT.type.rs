ptr_extern! { crate::tracking::TrackerCSRT,
	cv_PtrLcv_TrackerCSRTG_new_null_const, cv_PtrLcv_TrackerCSRTG_delete, cv_PtrLcv_TrackerCSRTG_getInnerPtr_const, cv_PtrLcv_TrackerCSRTG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] pub fn as_raw_PtrOfTrackerCSRT(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerCSRT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerCSRTTraitConst for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerCSRTTrait for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerCSRT>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerCSRTG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerCSRT>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerCSRTG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerCSRT> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerCSRT")
			.finish()
	}
}

