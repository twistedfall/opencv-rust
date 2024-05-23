ptr_extern! { crate::tracking::TrackerGOTURN,
	cv_PtrLcv_TrackerGOTURNG_new_null_const, cv_PtrLcv_TrackerGOTURNG_delete, cv_PtrLcv_TrackerGOTURNG_getInnerPtr_const, cv_PtrLcv_TrackerGOTURNG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerGOTURNTraitConst for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerGOTURNTrait for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerGOTURN>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerGOTURNG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerGOTURN>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerGOTURNG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerGOTURN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerGOTURN")
			.finish()
	}
}

