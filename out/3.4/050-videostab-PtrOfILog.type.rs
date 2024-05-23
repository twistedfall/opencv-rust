ptr_extern! { crate::videostab::ILog,
	cv_PtrLcv_videostab_ILogG_new_null_const, cv_PtrLcv_videostab_ILogG_delete, cv_PtrLcv_videostab_ILogG_getInnerPtr_const, cv_PtrLcv_videostab_ILogG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::ILog> {
	#[inline] pub fn as_raw_PtrOfILog(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfILog(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::ILog> {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::ILog> {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::ILog> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfILog")
			.finish()
	}
}

