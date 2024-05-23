ptr_extern! { core::DownhillSolver,
	cv_PtrLcv_DownhillSolverG_new_null_const, cv_PtrLcv_DownhillSolverG_delete, cv_PtrLcv_DownhillSolverG_getInnerPtr_const, cv_PtrLcv_DownhillSolverG_getInnerPtrMut
}

impl core::Ptr<core::DownhillSolver> {
	#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::DownhillSolverTraitConst for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::DownhillSolverTrait for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_DownhillSolverG_to_PtrOfAlgorithm }

impl core::MinProblemSolverTraitConst for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolverTrait for core::Ptr<core::DownhillSolver> {
	#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_DownhillSolverG_to_PtrOfMinProblemSolver }

impl std::fmt::Debug for core::Ptr<core::DownhillSolver> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDownhillSolver")
			.finish()
	}
}

