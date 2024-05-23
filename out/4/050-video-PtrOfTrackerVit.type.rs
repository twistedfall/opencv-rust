ptr_extern! { crate::video::TrackerVit,
	cv_PtrLcv_TrackerVitG_new_null_const, cv_PtrLcv_TrackerVitG_delete, cv_PtrLcv_TrackerVitG_getInnerPtr_const, cv_PtrLcv_TrackerVitG_getInnerPtrMut
}

impl core::Ptr<crate::video::TrackerVit> {
	#[inline] pub fn as_raw_PtrOfTrackerVit(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerVit(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::TrackerVitTraitConst for core::Ptr<crate::video::TrackerVit> {
	#[inline] fn as_raw_TrackerVit(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerVitTrait for core::Ptr<crate::video::TrackerVit> {
	#[inline] fn as_raw_mut_TrackerVit(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerVit> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerVit> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::TrackerVit>, core::Ptr<crate::video::Tracker>, cv_PtrLcv_TrackerVitG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::video::TrackerVit> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerVit")
			.finish()
	}
}

