

impl core::ToInputArray for {{rust_local}} {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_{{rust_local}}_input_array(instance: {{rust_extern}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_local}}_input_array(self.as_raw_{{rust_local}}()) }
			.into_result()
			.map(|ptr| core::_InputArray { ptr })
	}
}

impl core::ToInputArray for &{{rust_local}} {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		(*self).input_array()
	}
}

impl core::ToOutputArray for {{rust_local}} {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_{{rust_local}}_output_array(instance: {{rust_extern}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_local}}_output_array(self.as_raw_{{rust_local}}()) }
			.into_result()
			.map(|ptr| core::_OutputArray { ptr })
	}
}

impl core::ToOutputArray for &mut {{rust_local}} {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		(*self).output_array()
	}
}

impl core::ToInputOutputArray for {{rust_local}} {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_{{rust_local}}_input_output_array(instance: {{rust_extern}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_local}}_input_output_array(self.as_raw_{{rust_local}}()) }
			.into_result()
			.map(|ptr| core::_InputOutputArray { ptr })
	}
}

impl core::ToInputOutputArray for &mut {{rust_local}} {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		(*self).input_output_array()
	}
}
