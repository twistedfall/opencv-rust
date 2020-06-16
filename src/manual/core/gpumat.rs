use crate::{
	core::{
		_InputArray,
		_InputOutputArray,
		_OutputArray,
		GpuMat,
		HostMem,
		ToInputArray,
		ToInputOutputArray,
		ToOutputArray,
	},
	Result,
};

impl GpuMat {
	#[inline]
	pub fn default() -> Result<Self> {
		unsafe { Self::new(&mut Self::default_allocator()?) }
	}
}

impl ToInputArray for GpuMat {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		_InputArray::from_gpumat(self)
	}
}

impl ToInputArray for &GpuMat {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		(*self).input_array()
	}
}

impl ToOutputArray for GpuMat {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		_OutputArray::from_gpumat_mut(self)
	}
}

impl ToOutputArray for &mut GpuMat {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		(*self).output_array()
	}
}

impl ToInputOutputArray for GpuMat {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		_InputOutputArray::from_gpumat_mut(self)
	}
}

impl ToInputOutputArray for &mut GpuMat {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		(*self).input_output_array()
	}
}

impl ToInputArray for HostMem {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		_InputArray::from_hostmem(self)
	}
}

impl ToInputArray for &HostMem {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		(*self).input_array()
	}
}

impl ToOutputArray for HostMem {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		_OutputArray::from_hostmem_mut(self)
	}
}

impl ToOutputArray for &mut HostMem {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		(*self).output_array()
	}
}

impl ToInputOutputArray for HostMem {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		_InputOutputArray::from_hostmem_mut(self)
	}
}

impl ToInputOutputArray for &mut HostMem {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		(*self).input_output_array()
	}
}
