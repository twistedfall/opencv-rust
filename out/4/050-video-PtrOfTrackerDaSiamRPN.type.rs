ptr_extern! { crate::video::TrackerDaSiamRPN,
	cv_PtrLcv_TrackerDaSiamRPNG_new_null_const, cv_PtrLcv_TrackerDaSiamRPNG_delete, cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtr_const, cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtrMut
}

impl core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline] pub fn as_raw_PtrOfTrackerDaSiamRPN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerDaSiamRPN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerDaSiamRPNTraitConst for core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline] fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerDaSiamRPNTrait for core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline] fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::TrackerDaSiamRPN>, core::Ptr<crate::video::Tracker>, cv_PtrLcv_TrackerDaSiamRPNG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::video::TrackerDaSiamRPN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerDaSiamRPN")
			.finish()
	}
}

