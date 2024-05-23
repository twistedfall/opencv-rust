ptr_extern! { crate::cudafilters::Filter,
	cv_PtrLcv_cuda_FilterG_new_null_const, cv_PtrLcv_cuda_FilterG_delete, cv_PtrLcv_cuda_FilterG_getInnerPtr_const, cv_PtrLcv_cuda_FilterG_getInnerPtrMut
}

impl core::Ptr<crate::cudafilters::Filter> {
	#[inline] pub fn as_raw_PtrOfFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudafilters::FilterTraitConst for core::Ptr<crate::cudafilters::Filter> {
	#[inline] fn as_raw_Filter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudafilters::FilterTrait for core::Ptr<crate::cudafilters::Filter> {
	#[inline] fn as_raw_mut_Filter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudafilters::Filter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudafilters::Filter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafilters::Filter>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_FilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudafilters::Filter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFilter")
			.finish()
	}
}

