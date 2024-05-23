ptr_extern! { crate::cudaarithm::DFT,
	cv_PtrLcv_cuda_DFTG_new_null_const, cv_PtrLcv_cuda_DFTG_delete, cv_PtrLcv_cuda_DFTG_getInnerPtr_const, cv_PtrLcv_cuda_DFTG_getInnerPtrMut
}

impl core::Ptr<crate::cudaarithm::DFT> {
	#[inline] pub fn as_raw_PtrOfDFT(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDFT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaarithm::DFTTraitConst for core::Ptr<crate::cudaarithm::DFT> {
	#[inline] fn as_raw_DFT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaarithm::DFTTrait for core::Ptr<crate::cudaarithm::DFT> {
	#[inline] fn as_raw_mut_DFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::DFT> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::DFT> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaarithm::DFT>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_DFTG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaarithm::DFT> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDFT")
			.finish()
	}
}

