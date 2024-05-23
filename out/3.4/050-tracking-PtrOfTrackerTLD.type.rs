ptr_extern! { crate::tracking::TrackerTLD,
	cv_PtrLcv_TrackerTLDG_new_null_const, cv_PtrLcv_TrackerTLDG_delete, cv_PtrLcv_TrackerTLDG_getInnerPtr_const, cv_PtrLcv_TrackerTLDG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] pub fn as_raw_PtrOfTrackerTLD(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerTLD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerTLDTraitConst for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_TrackerTLD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTLDTrait for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_mut_TrackerTLD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerTLD>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerTLDG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerTLD>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerTLDG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerTLD> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerTLD")
			.finish()
	}
}

