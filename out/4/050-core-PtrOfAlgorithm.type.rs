ptr_extern! { core::Algorithm,
	cv_PtrLcv_AlgorithmG_new_null_const, cv_PtrLcv_AlgorithmG_delete, cv_PtrLcv_AlgorithmG_getInnerPtr_const, cv_PtrLcv_AlgorithmG_getInnerPtrMut
}

ptr_extern_ctor! { core::Algorithm, cv_PtrLcv_AlgorithmG_new_const_Algorithm }
impl core::Ptr<core::Algorithm> {
	#[inline] pub fn as_raw_PtrOfAlgorithm(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAlgorithm(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<core::Algorithm> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<core::Algorithm> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::Algorithm> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAlgorithm")
			.finish()
	}
}

