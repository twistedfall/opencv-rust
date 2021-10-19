

impl core::ToInputArray for {{rust_localalias}} {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_{{rust_localalias}}_input_array(instance: {{rust_extern_const}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_localalias}}_input_array(self.as_raw_{{rust_localalias}}()) }
			.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for {{rust_localalias}} {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_{{rust_localalias}}_output_array(instance: {{rust_extern_mut}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_localalias}}_output_array(self.as_raw_mut_{{rust_localalias}}()) }
			.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for {{rust_localalias}} {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_{{rust_localalias}}_input_output_array(instance: {{rust_extern_mut}}) -> sys::Result<*mut c_void>; }
		unsafe { cv_{{rust_localalias}}_input_output_array(self.as_raw_mut_{{rust_localalias}}()) }
			.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { {{rust_localalias}} }
