ptr_extern! { crate::optflow::PCAPrior,
	cv_PtrLcv_optflow_PCAPriorG_new_null_const, cv_PtrLcv_optflow_PCAPriorG_delete, cv_PtrLcv_optflow_PCAPriorG_getInnerPtr_const, cv_PtrLcv_optflow_PCAPriorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::optflow::PCAPrior, cv_PtrLcv_optflow_PCAPriorG_new_const_PCAPrior }
impl core::Ptr<crate::optflow::PCAPrior> {
	#[inline] pub fn as_raw_PtrOfPCAPrior(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCAPrior(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::PCAPriorTraitConst for core::Ptr<crate::optflow::PCAPrior> {
	#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::PCAPriorTrait for core::Ptr<crate::optflow::PCAPrior> {
	#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::optflow::PCAPrior> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPCAPrior")
			.finish()
	}
}

