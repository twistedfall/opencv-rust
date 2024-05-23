impl core::Vector<core::Vector<core::Point3d>> {
	pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point3d>,
	std_vectorLstd_vectorLcv_Point3dGG_new_const, std_vectorLstd_vectorLcv_Point3dGG_delete,
	std_vectorLstd_vectorLcv_Point3dGG_len_const, std_vectorLstd_vectorLcv_Point3dGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Point3dGG_capacity_const, std_vectorLstd_vectorLcv_Point3dGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Point3dGG_reserve_size_t, std_vectorLstd_vectorLcv_Point3dGG_remove_size_t,
	std_vectorLstd_vectorLcv_Point3dGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Point3dGG_clear,
	std_vectorLstd_vectorLcv_Point3dGG_get_const_size_t, std_vectorLstd_vectorLcv_Point3dGG_set_size_t_const_vectorLPoint3dG,
	std_vectorLstd_vectorLcv_Point3dGG_push_const_vectorLPoint3dG, std_vectorLstd_vectorLcv_Point3dGG_insert_size_t_const_vectorLPoint3dG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Point3d> }

impl ToInputArray for core::Vector<core::Vector<core::Point3d>> {
	// std::vector<std::vector<cv::Point3d>>::inputArray() generated
	// ("std::vector<std::vector<cv::Point3d>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3dGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<core::Point3d>> }

impl ToOutputArray for core::Vector<core::Vector<core::Point3d>> {
	// std::vector<std::vector<cv::Point3d>>::outputArray() generated
	// ("std::vector<std::vector<cv::Point3d>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3dGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<core::Point3d>> {
	// std::vector<std::vector<cv::Point3d>>::inputOutputArray() generated
	// ("std::vector<std::vector<cv::Point3d>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3dGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<core::Point3d>> }

