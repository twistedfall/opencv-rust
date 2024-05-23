impl core::Vector<core::Vector<core::Vec2d>> {
	pub fn as_raw_VectorOfVectorOfVec2d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfVec2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Vec2d>,
	std_vectorLstd_vectorLcv_Vec2dGG_new_const, std_vectorLstd_vectorLcv_Vec2dGG_delete,
	std_vectorLstd_vectorLcv_Vec2dGG_len_const, std_vectorLstd_vectorLcv_Vec2dGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Vec2dGG_capacity_const, std_vectorLstd_vectorLcv_Vec2dGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Vec2dGG_reserve_size_t, std_vectorLstd_vectorLcv_Vec2dGG_remove_size_t,
	std_vectorLstd_vectorLcv_Vec2dGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Vec2dGG_clear,
	std_vectorLstd_vectorLcv_Vec2dGG_get_const_size_t, std_vectorLstd_vectorLcv_Vec2dGG_set_size_t_const_vectorLVec2dG,
	std_vectorLstd_vectorLcv_Vec2dGG_push_const_vectorLVec2dG, std_vectorLstd_vectorLcv_Vec2dGG_insert_size_t_const_vectorLVec2dG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Vec2d> }

impl ToInputArray for core::Vector<core::Vector<core::Vec2d>> {
	// std::vector<std::vector<cv::Vec2d>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2dGG_inputArray_const(self.as_raw_VectorOfVectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<core::Vec2d>> }

impl ToOutputArray for core::Vector<core::Vector<core::Vec2d>> {
	// std::vector<std::vector<cv::Vec2d>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2dGG_outputArray(self.as_raw_mut_VectorOfVectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<core::Vec2d>> {
	// std::vector<std::vector<cv::Vec2d>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec2d>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2dGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<core::Vec2d>> }

