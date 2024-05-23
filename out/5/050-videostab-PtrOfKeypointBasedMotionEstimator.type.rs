ptr_extern! { crate::videostab::KeypointBasedMotionEstimator,
	cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_new_null_const, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_delete, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtr_const, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimator, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_new_const_KeypointBasedMotionEstimator }
impl core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::KeypointBasedMotionEstimator>, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_to_PtrOfImageMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKeypointBasedMotionEstimator")
			.finish()
	}
}

