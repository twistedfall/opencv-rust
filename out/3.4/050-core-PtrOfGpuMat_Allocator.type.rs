ptr_extern! { core::GpuMat_Allocator,
	cv_PtrLcv_cuda_GpuMat_AllocatorG_new_null_const, cv_PtrLcv_cuda_GpuMat_AllocatorG_delete, cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtr_const, cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtrMut
}

impl core::Ptr<core::GpuMat_Allocator> {
	#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::GpuMat_AllocatorTraitConst for core::Ptr<core::GpuMat_Allocator> {
	#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::GpuMat_AllocatorTrait for core::Ptr<core::GpuMat_Allocator> {
	#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::GpuMat_Allocator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGpuMat_Allocator")
			.finish()
	}
}

