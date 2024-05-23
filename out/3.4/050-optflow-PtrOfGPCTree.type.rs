ptr_extern! { crate::optflow::GPCTree,
	cv_PtrLcv_optflow_GPCTreeG_new_null_const, cv_PtrLcv_optflow_GPCTreeG_delete, cv_PtrLcv_optflow_GPCTreeG_getInnerPtr_const, cv_PtrLcv_optflow_GPCTreeG_getInnerPtrMut
}

ptr_extern_ctor! { crate::optflow::GPCTree, cv_PtrLcv_optflow_GPCTreeG_new_const_GPCTree }
impl core::Ptr<crate::optflow::GPCTree> {
	#[inline] pub fn as_raw_PtrOfGPCTree(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGPCTree(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::GPCTreeTraitConst for core::Ptr<crate::optflow::GPCTree> {
	#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::GPCTreeTrait for core::Ptr<crate::optflow::GPCTree> {
	#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::GPCTree> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::GPCTree> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::GPCTree>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_GPCTreeG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::optflow::GPCTree> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGPCTree")
			.finish()
	}
}

