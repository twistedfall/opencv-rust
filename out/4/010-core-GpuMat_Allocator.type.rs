impl core::GpuMat_AllocatorTraitConst for types::AbstractRefMut<'static, core::GpuMat_Allocator> {
	#[inline] fn as_raw_GpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
}

impl core::GpuMat_AllocatorTrait for types::AbstractRefMut<'static, core::GpuMat_Allocator> {
	#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

