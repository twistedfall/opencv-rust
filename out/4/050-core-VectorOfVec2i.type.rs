impl core::Vector<core::Vec2i> {
	pub fn as_raw_VectorOfVec2i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec2i,
	std_vectorLcv_Vec2iG_new_const, std_vectorLcv_Vec2iG_delete,
	std_vectorLcv_Vec2iG_len_const, std_vectorLcv_Vec2iG_isEmpty_const,
	std_vectorLcv_Vec2iG_capacity_const, std_vectorLcv_Vec2iG_shrinkToFit,
	std_vectorLcv_Vec2iG_reserve_size_t, std_vectorLcv_Vec2iG_remove_size_t,
	std_vectorLcv_Vec2iG_swap_size_t_size_t, std_vectorLcv_Vec2iG_clear,
	std_vectorLcv_Vec2iG_get_const_size_t, std_vectorLcv_Vec2iG_set_size_t_const_Vec2i,
	std_vectorLcv_Vec2iG_push_const_Vec2i, std_vectorLcv_Vec2iG_insert_size_t_const_Vec2i,
}

vector_copy_non_bool! { core::Vec2i,
	std_vectorLcv_Vec2iG_data_const, std_vectorLcv_Vec2iG_dataMut, cv_fromSlice_const_const_Vec2iX_size_t,
	std_vectorLcv_Vec2iG_clone_const,
}

impl ToInputArray for core::Vector<core::Vec2i> {
	// std::vector<cv::Vec2i>::inputArray() generated
	// ("std::vector<cv::Vec2i>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2iG_inputArray_const(self.as_raw_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vec2i> }

impl ToOutputArray for core::Vector<core::Vec2i> {
	// std::vector<cv::Vec2i>::outputArray() generated
	// ("std::vector<cv::Vec2i>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2iG_outputArray(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vec2i> {
	// std::vector<cv::Vec2i>::inputOutputArray() generated
	// ("std::vector<cv::Vec2i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2iG_inputOutputArray(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vec2i> }

