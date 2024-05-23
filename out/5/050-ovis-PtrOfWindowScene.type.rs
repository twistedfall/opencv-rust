ptr_extern! { crate::ovis::WindowScene,
	cv_PtrLcv_ovis_WindowSceneG_new_null_const, cv_PtrLcv_ovis_WindowSceneG_delete, cv_PtrLcv_ovis_WindowSceneG_getInnerPtr_const, cv_PtrLcv_ovis_WindowSceneG_getInnerPtrMut
}

impl core::Ptr<crate::ovis::WindowScene> {
	#[inline] pub fn as_raw_PtrOfWindowScene(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWindowScene(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ovis::WindowSceneTraitConst for core::Ptr<crate::ovis::WindowScene> {
	#[inline] fn as_raw_WindowScene(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ovis::WindowSceneTrait for core::Ptr<crate::ovis::WindowScene> {
	#[inline] fn as_raw_mut_WindowScene(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::ovis::WindowScene> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfWindowScene")
			.finish()
	}
}

