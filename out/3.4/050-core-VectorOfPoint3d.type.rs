impl core::Vector<core::Point3d> {
	pub fn as_raw_VectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Point3d,
	std_vectorLcv_Point3dG_new_const, std_vectorLcv_Point3dG_delete,
	std_vectorLcv_Point3dG_len_const, std_vectorLcv_Point3dG_isEmpty_const,
	std_vectorLcv_Point3dG_capacity_const, std_vectorLcv_Point3dG_shrinkToFit,
	std_vectorLcv_Point3dG_reserve_size_t, std_vectorLcv_Point3dG_remove_size_t,
	std_vectorLcv_Point3dG_swap_size_t_size_t, std_vectorLcv_Point3dG_clear,
	std_vectorLcv_Point3dG_get_const_size_t, std_vectorLcv_Point3dG_set_size_t_const_Point3d,
	std_vectorLcv_Point3dG_push_const_Point3d, std_vectorLcv_Point3dG_insert_size_t_const_Point3d,
}

vector_copy_non_bool! { core::Point3d,
	std_vectorLcv_Point3dG_data_const, std_vectorLcv_Point3dG_dataMut, cv_fromSlice_const_const_Point3dX_size_t,
	std_vectorLcv_Point3dG_clone_const,
}

impl ToInputArray for core::Vector<core::Point3d> {
	// std::vector<cv::Point3d>::inputArray() generated
	// ("std::vector<cv::Point3d>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3dG_inputArray_const(self.as_raw_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Point3d> }

impl ToOutputArray for core::Vector<core::Point3d> {
	// std::vector<cv::Point3d>::outputArray() generated
	// ("std::vector<cv::Point3d>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3dG_outputArray(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Point3d> {
	// std::vector<cv::Point3d>::inputOutputArray() generated
	// ("std::vector<cv::Point3d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3dG_inputOutputArray(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Point3d> }

