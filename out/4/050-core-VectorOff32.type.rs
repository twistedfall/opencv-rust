impl core::Vector<f32> {
	pub fn as_raw_VectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { f32,
	std_vectorLfloatG_new_const, std_vectorLfloatG_delete,
	std_vectorLfloatG_len_const, std_vectorLfloatG_isEmpty_const,
	std_vectorLfloatG_capacity_const, std_vectorLfloatG_shrinkToFit,
	std_vectorLfloatG_reserve_size_t, std_vectorLfloatG_remove_size_t,
	std_vectorLfloatG_swap_size_t_size_t, std_vectorLfloatG_clear,
	std_vectorLfloatG_get_const_size_t, std_vectorLfloatG_set_size_t_const_float,
	std_vectorLfloatG_push_const_float, std_vectorLfloatG_insert_size_t_const_float,
}

vector_copy_non_bool! { f32,
	std_vectorLfloatG_data_const, std_vectorLfloatG_dataMut, cv_fromSlice_const_const_floatX_size_t,
	std_vectorLfloatG_clone_const,
}

impl ToInputArray for core::Vector<f32> {
	// std::vector<float>::inputArray() generated
	// ("std::vector<float>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLfloatG_inputArray_const(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<f32> }

impl ToOutputArray for core::Vector<f32> {
	// std::vector<float>::outputArray() generated
	// ("std::vector<float>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLfloatG_outputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<f32> {
	// std::vector<float>::inputOutputArray() generated
	// ("std::vector<float>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLfloatG_inputOutputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<f32> }

