ptr_extern! { crate::videostab::MotionEstimatorRansacL2,
	cv_PtrLcv_videostab_MotionEstimatorRansacL2G_new_null_const, cv_PtrLcv_videostab_MotionEstimatorRansacL2G_delete, cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtr_const, cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MotionEstimatorRansacL2, cv_PtrLcv_videostab_MotionEstimatorRansacL2G_new_const_MotionEstimatorRansacL2 }
impl core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorRansacL2(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorRansacL2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorRansacL2TraitConst for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorRansacL2Trait for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MotionEstimatorRansacL2>, core::Ptr<crate::videostab::MotionEstimatorBase>, cv_PtrLcv_videostab_MotionEstimatorRansacL2G_to_PtrOfMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionEstimatorRansacL2")
			.finish()
	}
}

