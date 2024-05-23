ptr_extern! { crate::video::TrackerNano,
	cv_PtrLcv_TrackerNanoG_new_null_const, cv_PtrLcv_TrackerNanoG_delete, cv_PtrLcv_TrackerNanoG_getInnerPtr_const, cv_PtrLcv_TrackerNanoG_getInnerPtrMut
}

impl core::Ptr<crate::video::TrackerNano> {
	#[inline] pub fn as_raw_PtrOfTrackerNano(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerNano(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerNanoTraitConst for core::Ptr<crate::video::TrackerNano> {
	#[inline] fn as_raw_TrackerNano(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerNanoTrait for core::Ptr<crate::video::TrackerNano> {
	#[inline] fn as_raw_mut_TrackerNano(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerNano> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerNano> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::TrackerNano>, core::Ptr<crate::video::Tracker>, cv_PtrLcv_TrackerNanoG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::video::TrackerNano> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerNano")
			.finish()
	}
}

