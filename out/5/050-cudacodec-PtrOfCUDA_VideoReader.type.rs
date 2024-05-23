ptr_extern! { crate::cudacodec::CUDA_VideoReader,
	cv_PtrLcv_cudacodec_VideoReaderG_new_null_const, cv_PtrLcv_cudacodec_VideoReaderG_delete, cv_PtrLcv_cudacodec_VideoReaderG_getInnerPtr_const, cv_PtrLcv_cudacodec_VideoReaderG_getInnerPtrMut
}

impl core::Ptr<crate::cudacodec::CUDA_VideoReader> {
	#[inline] pub fn as_raw_PtrOfCUDA_VideoReader(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_VideoReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudacodec::CUDA_VideoReaderTraitConst for core::Ptr<crate::cudacodec::CUDA_VideoReader> {
	#[inline] fn as_raw_CUDA_VideoReader(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudacodec::CUDA_VideoReaderTrait for core::Ptr<crate::cudacodec::CUDA_VideoReader> {
	#[inline] fn as_raw_mut_CUDA_VideoReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::cudacodec::CUDA_VideoReader> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_VideoReader")
			.finish()
	}
}

