ptr_extern! { crate::video::TrackerGOTURN,
	cv_PtrLcv_TrackerGOTURNG_new_null_const, cv_PtrLcv_TrackerGOTURNG_delete, cv_PtrLcv_TrackerGOTURNG_getInnerPtr_const, cv_PtrLcv_TrackerGOTURNG_getInnerPtrMut
}

impl core::Ptr<crate::video::TrackerGOTURN> {
	#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerGOTURNTraitConst for core::Ptr<crate::video::TrackerGOTURN> {
	#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerGOTURNTrait for core::Ptr<crate::video::TrackerGOTURN> {
	#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerGOTURN> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerGOTURN> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::TrackerGOTURN>, core::Ptr<crate::video::Tracker>, cv_PtrLcv_TrackerGOTURNG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::video::TrackerGOTURN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerGOTURN")
			.finish()
	}
}

