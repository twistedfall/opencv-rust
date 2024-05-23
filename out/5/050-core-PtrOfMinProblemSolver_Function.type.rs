ptr_extern! { core::MinProblemSolver_Function,
	cv_PtrLcv_MinProblemSolver_FunctionG_new_null_const, cv_PtrLcv_MinProblemSolver_FunctionG_delete, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtr_const, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtrMut
}

impl core::Ptr<core::MinProblemSolver_Function> {
	#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::MinProblemSolver_FunctionTraitConst for core::Ptr<core::MinProblemSolver_Function> {
	#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolver_FunctionTrait for core::Ptr<core::MinProblemSolver_Function> {
	#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::MinProblemSolver_Function> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMinProblemSolver_Function")
			.finish()
	}
}

