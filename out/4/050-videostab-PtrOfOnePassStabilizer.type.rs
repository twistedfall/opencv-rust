ptr_extern! { crate::videostab::OnePassStabilizer,
	cv_PtrLcv_videostab_OnePassStabilizerG_new_null_const, cv_PtrLcv_videostab_OnePassStabilizerG_delete, cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtr_const, cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::OnePassStabilizer, cv_PtrLcv_videostab_OnePassStabilizerG_new_const_OnePassStabilizer }
impl core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] pub fn as_raw_PtrOfOnePassStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOnePassStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::OnePassStabilizerTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::OnePassStabilizerTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::OnePassStabilizer>, core::Ptr<crate::videostab::IFrameSource>, cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfIFrameSource }

impl crate::videostab::StabilizerBaseTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::OnePassStabilizer>, core::Ptr<crate::videostab::StabilizerBase>, cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfStabilizerBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::OnePassStabilizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOnePassStabilizer")
			.finish()
	}
}

