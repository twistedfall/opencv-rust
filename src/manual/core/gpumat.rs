use crate::{
	core::{GpuMat, HostMem},
	input_output_array, Result,
};

impl GpuMat {
	#[inline]
	#[allow(clippy::should_implement_trait)]
	pub fn default() -> Result<Self> {
		unsafe { Self::new(&mut Self::default_allocator()?) }
	}
}

input_output_array! { GpuMat, from_gpumat, from_gpumat_mut }
input_output_array! { HostMem, from_hostmem, from_hostmem_mut }
