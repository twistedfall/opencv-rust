ptr_extern! { crate::videostab::MotionEstimatorL1,
	cv_PtrLcv_videostab_MotionEstimatorL1G_new_null_const, cv_PtrLcv_videostab_MotionEstimatorL1G_delete, cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtr_const, cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MotionEstimatorL1, cv_PtrLcv_videostab_MotionEstimatorL1G_new_const_MotionEstimatorL1 }
impl core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorL1(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorL1(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorL1TraitConst for core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorL1Trait for core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MotionEstimatorL1>, core::Ptr<crate::videostab::MotionEstimatorBase>, cv_PtrLcv_videostab_MotionEstimatorL1G_to_PtrOfMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionEstimatorL1> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionEstimatorL1")
			.finish()
	}
}

