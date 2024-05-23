ptr_extern! { crate::videostab::NullLog,
	cv_PtrLcv_videostab_NullLogG_new_null_const, cv_PtrLcv_videostab_NullLogG_delete, cv_PtrLcv_videostab_NullLogG_getInnerPtr_const, cv_PtrLcv_videostab_NullLogG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullLog, cv_PtrLcv_videostab_NullLogG_new_const_NullLog }
impl core::Ptr<crate::videostab::NullLog> {
	#[inline] pub fn as_raw_PtrOfNullLog(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullLog(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullLogTraitConst for core::Ptr<crate::videostab::NullLog> {
	#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullLogTrait for core::Ptr<crate::videostab::NullLog> {
	#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::NullLog> {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::NullLog> {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullLog>, core::Ptr<crate::videostab::ILog>, cv_PtrLcv_videostab_NullLogG_to_PtrOfILog }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullLog> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullLog")
			.finish()
	}
}

