

extern "C" {
	fn cv_{{rust_localalias}}_input_array(instance: extern_send!({{rust_full}}), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
	fn cv_{{rust_localalias}}_output_array(instance: extern_send!(mut {{rust_full}}), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
	fn cv_{{rust_localalias}}_input_output_array(instance: extern_send!(mut {{rust_full}}), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
}

impl core::ToInputArray for {{rust_full}} {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { cv_{{rust_localalias}}_input_array(self.as_raw_{{rust_localalias}}(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for {{rust_full}} {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { cv_{{rust_localalias}}_output_array(self.as_raw_mut_{{rust_localalias}}(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for {{rust_full}} {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { cv_{{rust_localalias}}_input_output_array(self.as_raw_mut_{{rust_localalias}}(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { {{rust_full}} }
