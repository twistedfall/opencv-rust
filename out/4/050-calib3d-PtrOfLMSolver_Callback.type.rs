ptr_extern! { crate::calib3d::LMSolver_Callback,
	cv_PtrLcv_LMSolver_CallbackG_new_null_const, cv_PtrLcv_LMSolver_CallbackG_delete, cv_PtrLcv_LMSolver_CallbackG_getInnerPtr_const, cv_PtrLcv_LMSolver_CallbackG_getInnerPtrMut
}

impl core::Ptr<crate::calib3d::LMSolver_Callback> {
	#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::calib3d::LMSolver_CallbackTraitConst for core::Ptr<crate::calib3d::LMSolver_Callback> {
	#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::LMSolver_CallbackTrait for core::Ptr<crate::calib3d::LMSolver_Callback> {
	#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::calib3d::LMSolver_Callback> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLMSolver_Callback")
			.finish()
	}
}

