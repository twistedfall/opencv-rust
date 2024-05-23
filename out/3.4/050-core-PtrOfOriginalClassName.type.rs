ptr_extern! { core::OriginalClassName,
	cv_PtrLcv_utils_nested_OriginalClassNameG_new_null_const, cv_PtrLcv_utils_nested_OriginalClassNameG_delete, cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtr_const, cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtrMut
}

ptr_extern_ctor! { core::OriginalClassName, cv_PtrLcv_utils_nested_OriginalClassNameG_new_const_OriginalClassName }
impl core::Ptr<core::OriginalClassName> {
	#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::OriginalClassNameTraitConst for core::Ptr<core::OriginalClassName> {
	#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::OriginalClassNameTrait for core::Ptr<core::OriginalClassName> {
	#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::OriginalClassName> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOriginalClassName")
			.finish()
	}
}

