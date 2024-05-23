ptr_extern! { crate::videostab::LpMotionStabilizer,
	cv_PtrLcv_videostab_LpMotionStabilizerG_new_null_const, cv_PtrLcv_videostab_LpMotionStabilizerG_delete, cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtr_const, cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::LpMotionStabilizer, cv_PtrLcv_videostab_LpMotionStabilizerG_new_const_LpMotionStabilizer }
impl core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline] pub fn as_raw_PtrOfLpMotionStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLpMotionStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::LpMotionStabilizerTraitConst for core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::LpMotionStabilizerTrait for core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::LpMotionStabilizer>, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrLcv_videostab_LpMotionStabilizerG_to_PtrOfIMotionStabilizer }

impl std::fmt::Debug for core::Ptr<crate::videostab::LpMotionStabilizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLpMotionStabilizer")
			.finish()
	}
}

