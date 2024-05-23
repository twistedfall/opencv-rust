impl core::Vector<core::Rect2d> {
	pub fn as_raw_VectorOfRect2d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfRect2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Rect2d,
	std_vectorLcv_Rect2dG_new_const, std_vectorLcv_Rect2dG_delete,
	std_vectorLcv_Rect2dG_len_const, std_vectorLcv_Rect2dG_isEmpty_const,
	std_vectorLcv_Rect2dG_capacity_const, std_vectorLcv_Rect2dG_shrinkToFit,
	std_vectorLcv_Rect2dG_reserve_size_t, std_vectorLcv_Rect2dG_remove_size_t,
	std_vectorLcv_Rect2dG_swap_size_t_size_t, std_vectorLcv_Rect2dG_clear,
	std_vectorLcv_Rect2dG_get_const_size_t, std_vectorLcv_Rect2dG_set_size_t_const_Rect2d,
	std_vectorLcv_Rect2dG_push_const_Rect2d, std_vectorLcv_Rect2dG_insert_size_t_const_Rect2d,
}

vector_copy_non_bool! { core::Rect2d,
	std_vectorLcv_Rect2dG_data_const, std_vectorLcv_Rect2dG_dataMut, cv_fromSlice_const_const_Rect2dX_size_t,
	std_vectorLcv_Rect2dG_clone_const,
}

impl ToInputArray for core::Vector<core::Rect2d> {
	// std::vector<cv::Rect2d>::inputArray() generated
	// ("std::vector<cv::Rect2d>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Rect2dG_inputArray_const(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Rect2d> }

impl ToOutputArray for core::Vector<core::Rect2d> {
	// std::vector<cv::Rect2d>::outputArray() generated
	// ("std::vector<cv::Rect2d>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Rect2dG_outputArray(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Rect2d> {
	// std::vector<cv::Rect2d>::inputOutputArray() generated
	// ("std::vector<cv::Rect2d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Rect2dG_inputOutputArray(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Rect2d> }

