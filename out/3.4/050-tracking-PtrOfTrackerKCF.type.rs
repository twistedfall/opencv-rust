ptr_extern! { crate::tracking::TrackerKCF,
	cv_PtrLcv_TrackerKCFG_new_null_const, cv_PtrLcv_TrackerKCFG_delete, cv_PtrLcv_TrackerKCFG_getInnerPtr_const, cv_PtrLcv_TrackerKCFG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] pub fn as_raw_PtrOfTrackerKCF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerKCF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerKCFTraitConst for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerKCFTrait for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerKCF>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerKCFG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerKCF>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerKCFG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerKCF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerKCF")
			.finish()
	}
}

