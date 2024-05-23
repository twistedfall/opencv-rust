impl core::Vector<f64> {
	pub fn as_raw_VectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { f64,
	std_vectorLdoubleG_new_const, std_vectorLdoubleG_delete,
	std_vectorLdoubleG_len_const, std_vectorLdoubleG_isEmpty_const,
	std_vectorLdoubleG_capacity_const, std_vectorLdoubleG_shrinkToFit,
	std_vectorLdoubleG_reserve_size_t, std_vectorLdoubleG_remove_size_t,
	std_vectorLdoubleG_swap_size_t_size_t, std_vectorLdoubleG_clear,
	std_vectorLdoubleG_get_const_size_t, std_vectorLdoubleG_set_size_t_const_double,
	std_vectorLdoubleG_push_const_double, std_vectorLdoubleG_insert_size_t_const_double,
}

vector_copy_non_bool! { f64,
	std_vectorLdoubleG_data_const, std_vectorLdoubleG_dataMut, cv_fromSlice_const_const_doubleX_size_t,
	std_vectorLdoubleG_clone_const,
}

impl ToInputArray for core::Vector<f64> {
	// std::vector<double>::inputArray() generated
	// ("std::vector<double>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLdoubleG_inputArray_const(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<f64> }

impl ToOutputArray for core::Vector<f64> {
	// std::vector<double>::outputArray() generated
	// ("std::vector<double>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLdoubleG_outputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<f64> {
	// std::vector<double>::inputOutputArray() generated
	// ("std::vector<double>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLdoubleG_inputOutputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<f64> }

