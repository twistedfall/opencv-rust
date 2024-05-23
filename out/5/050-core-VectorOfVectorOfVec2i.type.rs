impl core::Vector<core::Vector<core::Vec2i>> {
	pub fn as_raw_VectorOfVectorOfVec2i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Vec2i>,
	std_vectorLstd_vectorLcv_Vec2iGG_new_const, std_vectorLstd_vectorLcv_Vec2iGG_delete,
	std_vectorLstd_vectorLcv_Vec2iGG_len_const, std_vectorLstd_vectorLcv_Vec2iGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Vec2iGG_capacity_const, std_vectorLstd_vectorLcv_Vec2iGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Vec2iGG_reserve_size_t, std_vectorLstd_vectorLcv_Vec2iGG_remove_size_t,
	std_vectorLstd_vectorLcv_Vec2iGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Vec2iGG_clear,
	std_vectorLstd_vectorLcv_Vec2iGG_get_const_size_t, std_vectorLstd_vectorLcv_Vec2iGG_set_size_t_const_vectorLVec2iG,
	std_vectorLstd_vectorLcv_Vec2iGG_push_const_vectorLVec2iG, std_vectorLstd_vectorLcv_Vec2iGG_insert_size_t_const_vectorLVec2iG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Vec2i> }

impl ToInputArray for core::Vector<core::Vector<core::Vec2i>> {
	// std::vector<std::vector<cv::Vec2i>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2iGG_inputArray_const(self.as_raw_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<core::Vec2i>> }

impl ToOutputArray for core::Vector<core::Vector<core::Vec2i>> {
	// std::vector<std::vector<cv::Vec2i>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2iGG_outputArray(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<core::Vec2i>> {
	// std::vector<std::vector<cv::Vec2i>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec2i>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2iGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<core::Vec2i>> }

