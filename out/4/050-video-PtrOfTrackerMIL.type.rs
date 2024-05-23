ptr_extern! { crate::video::TrackerMIL,
	cv_PtrLcv_TrackerMILG_new_null_const, cv_PtrLcv_TrackerMILG_delete, cv_PtrLcv_TrackerMILG_getInnerPtr_const, cv_PtrLcv_TrackerMILG_getInnerPtrMut
}

impl core::Ptr<crate::video::TrackerMIL> {
	#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerMILTraitConst for core::Ptr<crate::video::TrackerMIL> {
	#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerMILTrait for core::Ptr<crate::video::TrackerMIL> {
	#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerMIL> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerMIL> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::TrackerMIL>, core::Ptr<crate::video::Tracker>, cv_PtrLcv_TrackerMILG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::video::TrackerMIL> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerMIL")
			.finish()
	}
}

