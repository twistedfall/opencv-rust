ptr_extern! { crate::cudacodec::CUDA_RawVideoSource,
	cv_PtrLcv_cudacodec_RawVideoSourceG_new_null_const, cv_PtrLcv_cudacodec_RawVideoSourceG_delete, cv_PtrLcv_cudacodec_RawVideoSourceG_getInnerPtr_const, cv_PtrLcv_cudacodec_RawVideoSourceG_getInnerPtrMut
}

impl core::Ptr<crate::cudacodec::CUDA_RawVideoSource> {
	#[inline] pub fn as_raw_PtrOfCUDA_RawVideoSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_RawVideoSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudacodec::CUDA_RawVideoSourceTraitConst for core::Ptr<crate::cudacodec::CUDA_RawVideoSource> {
	#[inline] fn as_raw_CUDA_RawVideoSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudacodec::CUDA_RawVideoSourceTrait for core::Ptr<crate::cudacodec::CUDA_RawVideoSource> {
	#[inline] fn as_raw_mut_CUDA_RawVideoSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::cudacodec::CUDA_RawVideoSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_RawVideoSource")
			.finish()
	}
}

