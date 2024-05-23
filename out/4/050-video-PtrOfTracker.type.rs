ptr_extern! { crate::video::Tracker,
	cv_PtrLcv_TrackerG_new_null_const, cv_PtrLcv_TrackerG_delete, cv_PtrLcv_TrackerG_getInnerPtr_const, cv_PtrLcv_TrackerG_getInnerPtrMut
}

impl core::Ptr<crate::video::Tracker> {
	#[inline] pub fn as_raw_PtrOfTracker(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTracker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::Tracker> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::Tracker> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::video::Tracker> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTracker")
			.finish()
	}
}

