ptr_extern! { crate::videostab::NullDeblurer,
	cv_PtrLcv_videostab_NullDeblurerG_new_null_const, cv_PtrLcv_videostab_NullDeblurerG_delete, cv_PtrLcv_videostab_NullDeblurerG_getInnerPtr_const, cv_PtrLcv_videostab_NullDeblurerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullDeblurer, cv_PtrLcv_videostab_NullDeblurerG_new_const_NullDeblurer }
impl core::Ptr<crate::videostab::NullDeblurer> {
	#[inline] pub fn as_raw_PtrOfNullDeblurer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullDeblurer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullDeblurerTraitConst for core::Ptr<crate::videostab::NullDeblurer> {
	#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullDeblurerTrait for core::Ptr<crate::videostab::NullDeblurer> {
	#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::NullDeblurer> {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::NullDeblurer> {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullDeblurer>, core::Ptr<crate::videostab::DeblurerBase>, cv_PtrLcv_videostab_NullDeblurerG_to_PtrOfDeblurerBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullDeblurer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullDeblurer")
			.finish()
	}
}

