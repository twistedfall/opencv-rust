ptr_extern! { core::MinProblemSolver,
	cv_PtrLcv_MinProblemSolverG_new_null_const, cv_PtrLcv_MinProblemSolverG_delete, cv_PtrLcv_MinProblemSolverG_getInnerPtr_const, cv_PtrLcv_MinProblemSolverG_getInnerPtrMut
}

impl core::Ptr<core::MinProblemSolver> {
	#[inline] pub fn as_raw_PtrOfMinProblemSolver(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::MinProblemSolverTraitConst for core::Ptr<core::MinProblemSolver> {
	#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolverTrait for core::Ptr<core::MinProblemSolver> {
	#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<core::MinProblemSolver> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<core::MinProblemSolver> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<core::MinProblemSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_MinProblemSolverG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<core::MinProblemSolver> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMinProblemSolver")
			.finish()
	}
}

