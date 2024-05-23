ptr_extern! { crate::rgbd::Dynafu_DynaFu,
	cv_PtrLcv_dynafu_DynaFuG_new_null_const, cv_PtrLcv_dynafu_DynaFuG_delete, cv_PtrLcv_dynafu_DynaFuG_getInnerPtr_const, cv_PtrLcv_dynafu_DynaFuG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::Dynafu_DynaFu> {
	#[inline] pub fn as_raw_PtrOfDynafu_DynaFu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDynafu_DynaFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Dynafu_DynaFuTraitConst for core::Ptr<crate::rgbd::Dynafu_DynaFu> {
	#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Dynafu_DynaFuTrait for core::Ptr<crate::rgbd::Dynafu_DynaFu> {
	#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Dynafu_DynaFu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDynafu_DynaFu")
			.finish()
	}
}

