impl core::Vector<core::Vector<core::Vec2f>> {
	pub fn as_raw_VectorOfVectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Vec2f>,
	std_vectorLstd_vectorLcv_Vec2fGG_new_const, std_vectorLstd_vectorLcv_Vec2fGG_delete,
	std_vectorLstd_vectorLcv_Vec2fGG_len_const, std_vectorLstd_vectorLcv_Vec2fGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Vec2fGG_capacity_const, std_vectorLstd_vectorLcv_Vec2fGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Vec2fGG_reserve_size_t, std_vectorLstd_vectorLcv_Vec2fGG_remove_size_t,
	std_vectorLstd_vectorLcv_Vec2fGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Vec2fGG_clear,
	std_vectorLstd_vectorLcv_Vec2fGG_get_const_size_t, std_vectorLstd_vectorLcv_Vec2fGG_set_size_t_const_vectorLVec2fG,
	std_vectorLstd_vectorLcv_Vec2fGG_push_const_vectorLVec2fG, std_vectorLstd_vectorLcv_Vec2fGG_insert_size_t_const_vectorLVec2fG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Vec2f> }

impl ToInputArray for core::Vector<core::Vector<core::Vec2f>> {
	// std::vector<std::vector<cv::Vec2f>>::inputArray() generated
	// ("std::vector<std::vector<cv::Vec2f>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2fGG_inputArray_const(self.as_raw_VectorOfVectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<core::Vec2f>> }

impl ToOutputArray for core::Vector<core::Vector<core::Vec2f>> {
	// std::vector<std::vector<cv::Vec2f>>::outputArray() generated
	// ("std::vector<std::vector<cv::Vec2f>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2fGG_outputArray(self.as_raw_mut_VectorOfVectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<core::Vec2f>> {
	// std::vector<std::vector<cv::Vec2f>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Vec2f>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Vec2fGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<core::Vec2f>> }

