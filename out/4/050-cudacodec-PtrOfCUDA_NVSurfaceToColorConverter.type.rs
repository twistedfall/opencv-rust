ptr_extern! { crate::cudacodec::CUDA_NVSurfaceToColorConverter,
	cv_PtrLcv_cudacodec_NVSurfaceToColorConverterG_new_null_const, cv_PtrLcv_cudacodec_NVSurfaceToColorConverterG_delete, cv_PtrLcv_cudacodec_NVSurfaceToColorConverterG_getInnerPtr_const, cv_PtrLcv_cudacodec_NVSurfaceToColorConverterG_getInnerPtrMut
}

impl core::Ptr<crate::cudacodec::CUDA_NVSurfaceToColorConverter> {
	#[inline] pub fn as_raw_PtrOfCUDA_NVSurfaceToColorConverter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_NVSurfaceToColorConverter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudacodec::CUDA_NVSurfaceToColorConverterTraitConst for core::Ptr<crate::cudacodec::CUDA_NVSurfaceToColorConverter> {
	#[inline] fn as_raw_CUDA_NVSurfaceToColorConverter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudacodec::CUDA_NVSurfaceToColorConverterTrait for core::Ptr<crate::cudacodec::CUDA_NVSurfaceToColorConverter> {
	#[inline] fn as_raw_mut_CUDA_NVSurfaceToColorConverter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::cudacodec::CUDA_NVSurfaceToColorConverter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_NVSurfaceToColorConverter")
			.finish()
	}
}

