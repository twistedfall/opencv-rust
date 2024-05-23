ptr_extern! { crate::rapid::Rapid_OLSTracker,
	cv_PtrLcv_rapid_OLSTrackerG_new_null_const, cv_PtrLcv_rapid_OLSTrackerG_delete, cv_PtrLcv_rapid_OLSTrackerG_getInnerPtr_const, cv_PtrLcv_rapid_OLSTrackerG_getInnerPtrMut
}

impl core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] pub fn as_raw_PtrOfRapid_OLSTracker(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRapid_OLSTracker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rapid::Rapid_OLSTrackerTraitConst for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_Rapid_OLSTracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Rapid_OLSTrackerTrait for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_mut_Rapid_OLSTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rapid::Rapid_OLSTracker>, core::Ptr<core::Algorithm>, cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfAlgorithm }

impl crate::rapid::Rapid_TrackerTraitConst for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rapid::Rapid_OLSTracker>, core::Ptr<crate::rapid::Rapid_Tracker>, cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfRapid_Tracker }

impl std::fmt::Debug for core::Ptr<crate::rapid::Rapid_OLSTracker> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRapid_OLSTracker")
			.finish()
	}
}

