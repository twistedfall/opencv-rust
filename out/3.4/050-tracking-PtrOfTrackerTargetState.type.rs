ptr_extern! { crate::tracking::TrackerTargetState,
	cv_PtrLcv_TrackerTargetStateG_new_null_const, cv_PtrLcv_TrackerTargetStateG_delete, cv_PtrLcv_TrackerTargetStateG_getInnerPtr_const, cv_PtrLcv_TrackerTargetStateG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerTargetState, cv_PtrLcv_TrackerTargetStateG_new_const_TrackerTargetState }
impl core::Ptr<crate::tracking::TrackerTargetState> {
	#[inline] pub fn as_raw_PtrOfTrackerTargetState(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerTargetState(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerTargetStateTraitConst for core::Ptr<crate::tracking::TrackerTargetState> {
	#[inline] fn as_raw_TrackerTargetState(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTargetStateTrait for core::Ptr<crate::tracking::TrackerTargetState> {
	#[inline] fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerTargetState> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerTargetState")
			.finish()
	}
}

