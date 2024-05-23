ptr_extern! { crate::cudaarithm::LookUpTable,
	cv_PtrLcv_cuda_LookUpTableG_new_null_const, cv_PtrLcv_cuda_LookUpTableG_delete, cv_PtrLcv_cuda_LookUpTableG_getInnerPtr_const, cv_PtrLcv_cuda_LookUpTableG_getInnerPtrMut
}

impl core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline] pub fn as_raw_PtrOfLookUpTable(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLookUpTable(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaarithm::LookUpTableTraitConst for core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline] fn as_raw_LookUpTable(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaarithm::LookUpTableTrait for core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline] fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaarithm::LookUpTable>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_LookUpTableG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaarithm::LookUpTable> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLookUpTable")
			.finish()
	}
}

