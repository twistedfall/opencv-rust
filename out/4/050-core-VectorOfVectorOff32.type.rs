impl core::Vector<core::Vector<f32>> {
	pub fn as_raw_VectorOfVectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<f32>,
	std_vectorLstd_vectorLfloatGG_new_const, std_vectorLstd_vectorLfloatGG_delete,
	std_vectorLstd_vectorLfloatGG_len_const, std_vectorLstd_vectorLfloatGG_isEmpty_const,
	std_vectorLstd_vectorLfloatGG_capacity_const, std_vectorLstd_vectorLfloatGG_shrinkToFit,
	std_vectorLstd_vectorLfloatGG_reserve_size_t, std_vectorLstd_vectorLfloatGG_remove_size_t,
	std_vectorLstd_vectorLfloatGG_swap_size_t_size_t, std_vectorLstd_vectorLfloatGG_clear,
	std_vectorLstd_vectorLfloatGG_get_const_size_t, std_vectorLstd_vectorLfloatGG_set_size_t_const_vectorLfloatG,
	std_vectorLstd_vectorLfloatGG_push_const_vectorLfloatG, std_vectorLstd_vectorLfloatGG_insert_size_t_const_vectorLfloatG,
}

vector_non_copy_or_bool! { clone core::Vector<f32> }

impl ToInputArray for core::Vector<core::Vector<f32>> {
	// std::vector<std::vector<float>>::inputArray() generated
	// ("std::vector<std::vector<float>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLfloatGG_inputArray_const(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<f32>> }

impl ToOutputArray for core::Vector<core::Vector<f32>> {
	// std::vector<std::vector<float>>::outputArray() generated
	// ("std::vector<std::vector<float>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLfloatGG_outputArray(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<f32>> {
	// std::vector<std::vector<float>>::inputOutputArray() generated
	// ("std::vector<std::vector<float>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLfloatGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<f32>> }

