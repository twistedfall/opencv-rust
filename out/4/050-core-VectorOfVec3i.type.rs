impl core::Vector<core::Vec3i> {
	pub fn as_raw_VectorOfVec3i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec3i,
	std_vectorLcv_Vec3iG_new_const, std_vectorLcv_Vec3iG_delete,
	std_vectorLcv_Vec3iG_len_const, std_vectorLcv_Vec3iG_isEmpty_const,
	std_vectorLcv_Vec3iG_capacity_const, std_vectorLcv_Vec3iG_shrinkToFit,
	std_vectorLcv_Vec3iG_reserve_size_t, std_vectorLcv_Vec3iG_remove_size_t,
	std_vectorLcv_Vec3iG_swap_size_t_size_t, std_vectorLcv_Vec3iG_clear,
	std_vectorLcv_Vec3iG_get_const_size_t, std_vectorLcv_Vec3iG_set_size_t_const_Vec3i,
	std_vectorLcv_Vec3iG_push_const_Vec3i, std_vectorLcv_Vec3iG_insert_size_t_const_Vec3i,
}

vector_copy_non_bool! { core::Vec3i,
	std_vectorLcv_Vec3iG_data_const, std_vectorLcv_Vec3iG_dataMut, cv_fromSlice_const_const_Vec3iX_size_t,
	std_vectorLcv_Vec3iG_clone_const,
}

impl ToInputArray for core::Vector<core::Vec3i> {
	// std::vector<cv::Vec3i>::inputArray() generated
	// ("std::vector<cv::Vec3i>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3iG_inputArray_const(self.as_raw_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vec3i> }

impl ToOutputArray for core::Vector<core::Vec3i> {
	// std::vector<cv::Vec3i>::outputArray() generated
	// ("std::vector<cv::Vec3i>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3iG_outputArray(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vec3i> {
	// std::vector<cv::Vec3i>::inputOutputArray() generated
	// ("std::vector<cv::Vec3i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3iG_inputOutputArray(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vec3i> }

