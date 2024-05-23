ptr_extern! { crate::videostab::IMotionStabilizer,
	cv_PtrLcv_videostab_IMotionStabilizerG_new_null_const, cv_PtrLcv_videostab_IMotionStabilizerG_delete, cv_PtrLcv_videostab_IMotionStabilizerG_getInnerPtr_const, cv_PtrLcv_videostab_IMotionStabilizerG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::IMotionStabilizer> {
	#[inline] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::IMotionStabilizer> {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::IMotionStabilizer> {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::IMotionStabilizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIMotionStabilizer")
			.finish()
	}
}

