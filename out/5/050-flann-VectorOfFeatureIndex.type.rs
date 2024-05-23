impl core::Vector<crate::flann::FeatureIndex> {
	pub fn as_raw_VectorOfFeatureIndex(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfFeatureIndex(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::flann::FeatureIndex,
	std_vectorLcvflann_lsh_FeatureIndexG_new_const, std_vectorLcvflann_lsh_FeatureIndexG_delete,
	std_vectorLcvflann_lsh_FeatureIndexG_len_const, std_vectorLcvflann_lsh_FeatureIndexG_isEmpty_const,
	std_vectorLcvflann_lsh_FeatureIndexG_capacity_const, std_vectorLcvflann_lsh_FeatureIndexG_shrinkToFit,
	std_vectorLcvflann_lsh_FeatureIndexG_reserve_size_t, std_vectorLcvflann_lsh_FeatureIndexG_remove_size_t,
	std_vectorLcvflann_lsh_FeatureIndexG_swap_size_t_size_t, std_vectorLcvflann_lsh_FeatureIndexG_clear,
	std_vectorLcvflann_lsh_FeatureIndexG_get_const_size_t, std_vectorLcvflann_lsh_FeatureIndexG_set_size_t_const_FeatureIndex,
	std_vectorLcvflann_lsh_FeatureIndexG_push_const_FeatureIndex, std_vectorLcvflann_lsh_FeatureIndexG_insert_size_t_const_FeatureIndex,
}

vector_copy_non_bool! { crate::flann::FeatureIndex,
	std_vectorLcvflann_lsh_FeatureIndexG_data_const, std_vectorLcvflann_lsh_FeatureIndexG_dataMut, cv_fromSlice_const_const_FeatureIndexX_size_t,
	std_vectorLcvflann_lsh_FeatureIndexG_clone_const,
}

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputArray for core::Vector<crate::flann::FeatureIndex> {
	// std::vector<cvflann::lsh::FeatureIndex>::inputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcvflann_lsh_FeatureIndexG_inputArray_const(self.as_raw_VectorOfFeatureIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
input_array_ref_forward! { core::Vector<crate::flann::FeatureIndex> }

#[cfg(ocvrs_opencv_branch_5)]
impl ToOutputArray for core::Vector<crate::flann::FeatureIndex> {
	// std::vector<cvflann::lsh::FeatureIndex>::outputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcvflann_lsh_FeatureIndexG_outputArray(self.as_raw_mut_VectorOfFeatureIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputOutputArray for core::Vector<crate::flann::FeatureIndex> {
	// std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray() generated
	// ("std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcvflann_lsh_FeatureIndexG_inputOutputArray(self.as_raw_mut_VectorOfFeatureIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
output_array_ref_forward! { core::Vector<crate::flann::FeatureIndex> }

