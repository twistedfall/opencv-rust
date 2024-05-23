ptr_extern! { core::Formatter,
	cv_PtrLcv_FormatterG_new_null_const, cv_PtrLcv_FormatterG_delete, cv_PtrLcv_FormatterG_getInnerPtr_const, cv_PtrLcv_FormatterG_getInnerPtrMut
}

impl core::Ptr<core::Formatter> {
	#[inline] pub fn as_raw_PtrOfFormatter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::FormatterTraitConst for core::Ptr<core::Formatter> {
	#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::FormatterTrait for core::Ptr<core::Formatter> {
	#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::Formatter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFormatter")
			.finish()
	}
}

