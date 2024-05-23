ptr_extern! { crate::videostab::TwoPassStabilizer,
	cv_PtrLcv_videostab_TwoPassStabilizerG_new_null_const, cv_PtrLcv_videostab_TwoPassStabilizerG_delete, cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtr_const, cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::TwoPassStabilizer, cv_PtrLcv_videostab_TwoPassStabilizerG_new_const_TwoPassStabilizer }
impl core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] pub fn as_raw_PtrOfTwoPassStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTwoPassStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::TwoPassStabilizerTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::TwoPassStabilizerTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::TwoPassStabilizer>, core::Ptr<crate::videostab::IFrameSource>, cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfIFrameSource }

impl crate::videostab::StabilizerBaseTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::TwoPassStabilizer>, core::Ptr<crate::videostab::StabilizerBase>, cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfStabilizerBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::TwoPassStabilizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTwoPassStabilizer")
			.finish()
	}
}

