impl core::Vector<core::Scalar> {
	pub fn as_raw_VectorOfScalar(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Scalar,
	std_vectorLcv_ScalarG_new_const, std_vectorLcv_ScalarG_delete,
	std_vectorLcv_ScalarG_len_const, std_vectorLcv_ScalarG_isEmpty_const,
	std_vectorLcv_ScalarG_capacity_const, std_vectorLcv_ScalarG_shrinkToFit,
	std_vectorLcv_ScalarG_reserve_size_t, std_vectorLcv_ScalarG_remove_size_t,
	std_vectorLcv_ScalarG_swap_size_t_size_t, std_vectorLcv_ScalarG_clear,
	std_vectorLcv_ScalarG_get_const_size_t, std_vectorLcv_ScalarG_set_size_t_const_Scalar,
	std_vectorLcv_ScalarG_push_const_Scalar, std_vectorLcv_ScalarG_insert_size_t_const_Scalar,
}

vector_copy_non_bool! { core::Scalar,
	std_vectorLcv_ScalarG_data_const, std_vectorLcv_ScalarG_dataMut, cv_fromSlice_const_const_ScalarX_size_t,
	std_vectorLcv_ScalarG_clone_const,
}

impl ToInputArray for core::Vector<core::Scalar> {
	// std::vector<cv::Scalar>::inputArray() generated
	// ("std::vector<cv::Scalar>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_ScalarG_inputArray_const(self.as_raw_VectorOfScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Scalar> }

impl ToOutputArray for core::Vector<core::Scalar> {
	// std::vector<cv::Scalar>::outputArray() generated
	// ("std::vector<cv::Scalar>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_ScalarG_outputArray(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Scalar> {
	// std::vector<cv::Scalar>::inputOutputArray() generated
	// ("std::vector<cv::Scalar>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_ScalarG_inputOutputArray(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Scalar> }

