ptr_extern! { crate::videostab::DeblurerBase,
	cv_PtrLcv_videostab_DeblurerBaseG_new_null_const, cv_PtrLcv_videostab_DeblurerBaseG_delete, cv_PtrLcv_videostab_DeblurerBaseG_getInnerPtr_const, cv_PtrLcv_videostab_DeblurerBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::DeblurerBase> {
	#[inline] pub fn as_raw_PtrOfDeblurerBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::DeblurerBase> {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::DeblurerBase> {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::DeblurerBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDeblurerBase")
			.finish()
	}
}

