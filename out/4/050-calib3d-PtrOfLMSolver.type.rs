ptr_extern! { crate::calib3d::LMSolver,
	cv_PtrLcv_LMSolverG_new_null_const, cv_PtrLcv_LMSolverG_delete, cv_PtrLcv_LMSolverG_getInnerPtr_const, cv_PtrLcv_LMSolverG_getInnerPtrMut
}

impl core::Ptr<crate::calib3d::LMSolver> {
	#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::calib3d::LMSolverTraitConst for core::Ptr<crate::calib3d::LMSolver> {
	#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::LMSolverTrait for core::Ptr<crate::calib3d::LMSolver> {
	#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::LMSolver> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::calib3d::LMSolver> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::calib3d::LMSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_LMSolverG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::calib3d::LMSolver> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLMSolver")
			.finish()
	}
}

