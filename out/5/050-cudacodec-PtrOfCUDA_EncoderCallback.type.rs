ptr_extern! { crate::cudacodec::CUDA_EncoderCallback,
	cv_PtrLcv_cudacodec_EncoderCallbackG_new_null_const, cv_PtrLcv_cudacodec_EncoderCallbackG_delete, cv_PtrLcv_cudacodec_EncoderCallbackG_getInnerPtr_const, cv_PtrLcv_cudacodec_EncoderCallbackG_getInnerPtrMut
}

impl core::Ptr<crate::cudacodec::CUDA_EncoderCallback> {
	#[inline] pub fn as_raw_PtrOfCUDA_EncoderCallback(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_EncoderCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudacodec::CUDA_EncoderCallbackTraitConst for core::Ptr<crate::cudacodec::CUDA_EncoderCallback> {
	#[inline] fn as_raw_CUDA_EncoderCallback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudacodec::CUDA_EncoderCallbackTrait for core::Ptr<crate::cudacodec::CUDA_EncoderCallback> {
	#[inline] fn as_raw_mut_CUDA_EncoderCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::cudacodec::CUDA_EncoderCallback> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_EncoderCallback")
			.finish()
	}
}

