ptr_extern! { crate::videostab::LogToStdout,
	cv_PtrLcv_videostab_LogToStdoutG_new_null_const, cv_PtrLcv_videostab_LogToStdoutG_delete, cv_PtrLcv_videostab_LogToStdoutG_getInnerPtr_const, cv_PtrLcv_videostab_LogToStdoutG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::LogToStdout, cv_PtrLcv_videostab_LogToStdoutG_new_const_LogToStdout }
impl core::Ptr<crate::videostab::LogToStdout> {
	#[inline] pub fn as_raw_PtrOfLogToStdout(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLogToStdout(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::LogToStdoutTraitConst for core::Ptr<crate::videostab::LogToStdout> {
	#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::LogToStdoutTrait for core::Ptr<crate::videostab::LogToStdout> {
	#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::LogToStdout> {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::LogToStdout> {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::LogToStdout>, core::Ptr<crate::videostab::ILog>, cv_PtrLcv_videostab_LogToStdoutG_to_PtrOfILog }

impl std::fmt::Debug for core::Ptr<crate::videostab::LogToStdout> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLogToStdout")
			.finish()
	}
}

