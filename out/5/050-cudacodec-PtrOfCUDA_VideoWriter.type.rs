ptr_extern! { crate::cudacodec::CUDA_VideoWriter,
	cv_PtrLcv_cudacodec_VideoWriterG_new_null_const, cv_PtrLcv_cudacodec_VideoWriterG_delete, cv_PtrLcv_cudacodec_VideoWriterG_getInnerPtr_const, cv_PtrLcv_cudacodec_VideoWriterG_getInnerPtrMut
}

impl core::Ptr<crate::cudacodec::CUDA_VideoWriter> {
	#[inline] pub fn as_raw_PtrOfCUDA_VideoWriter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_VideoWriter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudacodec::CUDA_VideoWriterTraitConst for core::Ptr<crate::cudacodec::CUDA_VideoWriter> {
	#[inline] fn as_raw_CUDA_VideoWriter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudacodec::CUDA_VideoWriterTrait for core::Ptr<crate::cudacodec::CUDA_VideoWriter> {
	#[inline] fn as_raw_mut_CUDA_VideoWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::cudacodec::CUDA_VideoWriter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_VideoWriter")
			.finish()
	}
}

