ptr_extern! { crate::videostab::MotionEstimatorBase,
	cv_PtrLcv_videostab_MotionEstimatorBaseG_new_null_const, cv_PtrLcv_videostab_MotionEstimatorBaseG_delete, cv_PtrLcv_videostab_MotionEstimatorBaseG_getInnerPtr_const, cv_PtrLcv_videostab_MotionEstimatorBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::MotionEstimatorBase> {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorBase> {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorBase> {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionEstimatorBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionEstimatorBase")
			.finish()
	}
}

