ptr_extern! { crate::videostab::MotionFilterBase,
	cv_PtrLcv_videostab_MotionFilterBaseG_new_null_const, cv_PtrLcv_videostab_MotionFilterBaseG_delete, cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtr_const, cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline] pub fn as_raw_PtrOfMotionFilterBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionFilterBaseTraitConst for core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionFilterBaseTrait for core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MotionFilterBase>, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrLcv_videostab_MotionFilterBaseG_to_PtrOfIMotionStabilizer }

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionFilterBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionFilterBase")
			.finish()
	}
}

