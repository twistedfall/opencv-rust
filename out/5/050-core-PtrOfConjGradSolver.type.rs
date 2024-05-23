ptr_extern! { core::ConjGradSolver,
	cv_PtrLcv_ConjGradSolverG_new_null_const, cv_PtrLcv_ConjGradSolverG_delete, cv_PtrLcv_ConjGradSolverG_getInnerPtr_const, cv_PtrLcv_ConjGradSolverG_getInnerPtrMut
}

impl core::Ptr<core::ConjGradSolver> {
	#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::ConjGradSolverTraitConst for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::ConjGradSolverTrait for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_ConjGradSolverG_to_PtrOfAlgorithm }

impl core::MinProblemSolverTraitConst for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolverTrait for core::Ptr<core::ConjGradSolver> {
	#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_ConjGradSolverG_to_PtrOfMinProblemSolver }

impl std::fmt::Debug for core::Ptr<core::ConjGradSolver> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConjGradSolver")
			.finish()
	}
}

