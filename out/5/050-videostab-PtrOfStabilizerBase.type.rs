ptr_extern! { crate::videostab::StabilizerBase,
	cv_PtrLcv_videostab_StabilizerBaseG_new_null_const, cv_PtrLcv_videostab_StabilizerBaseG_delete, cv_PtrLcv_videostab_StabilizerBaseG_getInnerPtr_const, cv_PtrLcv_videostab_StabilizerBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::StabilizerBase> {
	#[inline] pub fn as_raw_PtrOfStabilizerBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStabilizerBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::StabilizerBaseTraitConst for core::Ptr<crate::videostab::StabilizerBase> {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for core::Ptr<crate::videostab::StabilizerBase> {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::StabilizerBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStabilizerBase")
			.finish()
	}
}

