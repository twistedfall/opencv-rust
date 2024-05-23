impl core::Vector<core::Vector<core::Rect>> {
	pub fn as_raw_VectorOfVectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Rect>,
	std_vectorLstd_vectorLcv_RectGG_new_const, std_vectorLstd_vectorLcv_RectGG_delete,
	std_vectorLstd_vectorLcv_RectGG_len_const, std_vectorLstd_vectorLcv_RectGG_isEmpty_const,
	std_vectorLstd_vectorLcv_RectGG_capacity_const, std_vectorLstd_vectorLcv_RectGG_shrinkToFit,
	std_vectorLstd_vectorLcv_RectGG_reserve_size_t, std_vectorLstd_vectorLcv_RectGG_remove_size_t,
	std_vectorLstd_vectorLcv_RectGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_RectGG_clear,
	std_vectorLstd_vectorLcv_RectGG_get_const_size_t, std_vectorLstd_vectorLcv_RectGG_set_size_t_const_vectorLRectG,
	std_vectorLstd_vectorLcv_RectGG_push_const_vectorLRectG, std_vectorLstd_vectorLcv_RectGG_insert_size_t_const_vectorLRectG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Rect> }

impl ToInputArray for core::Vector<core::Vector<core::Rect>> {
	// std::vector<std::vector<cv::Rect>>::inputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_RectGG_inputArray_const(self.as_raw_VectorOfVectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<core::Rect>> }

impl ToOutputArray for core::Vector<core::Vector<core::Rect>> {
	// std::vector<std::vector<cv::Rect>>::outputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_RectGG_outputArray(self.as_raw_mut_VectorOfVectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<core::Rect>> {
	// std::vector<std::vector<cv::Rect>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Rect>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_RectGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<core::Rect>> }

