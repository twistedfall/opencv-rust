ptr_extern! { crate::videostab::ImageMotionEstimatorBase,
	cv_PtrLcv_videostab_ImageMotionEstimatorBaseG_new_null_const, cv_PtrLcv_videostab_ImageMotionEstimatorBaseG_delete, cv_PtrLcv_videostab_ImageMotionEstimatorBaseG_getInnerPtr_const, cv_PtrLcv_videostab_ImageMotionEstimatorBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
	#[inline] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfImageMotionEstimatorBase")
			.finish()
	}
}

