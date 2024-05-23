impl core::Vector<u64> {
	pub fn as_raw_VectorOfu64(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfu64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { u64,
	std_vectorLuint64_tG_new_const, std_vectorLuint64_tG_delete,
	std_vectorLuint64_tG_len_const, std_vectorLuint64_tG_isEmpty_const,
	std_vectorLuint64_tG_capacity_const, std_vectorLuint64_tG_shrinkToFit,
	std_vectorLuint64_tG_reserve_size_t, std_vectorLuint64_tG_remove_size_t,
	std_vectorLuint64_tG_swap_size_t_size_t, std_vectorLuint64_tG_clear,
	std_vectorLuint64_tG_get_const_size_t, std_vectorLuint64_tG_set_size_t_const_uint64_t,
	std_vectorLuint64_tG_push_const_uint64_t, std_vectorLuint64_tG_insert_size_t_const_uint64_t,
}

vector_copy_non_bool! { u64,
	std_vectorLuint64_tG_data_const, std_vectorLuint64_tG_dataMut, cv_fromSlice_const_const_uint64_tX_size_t,
	std_vectorLuint64_tG_clone_const,
}

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputArray for core::Vector<u64> {
	// std::vector<uint64_t>::inputArray() generated
	// ("std::vector<uint64_t>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLuint64_tG_inputArray_const(self.as_raw_VectorOfu64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
input_array_ref_forward! { core::Vector<u64> }

#[cfg(ocvrs_opencv_branch_5)]
impl ToOutputArray for core::Vector<u64> {
	// std::vector<uint64_t>::outputArray() generated
	// ("std::vector<uint64_t>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLuint64_tG_outputArray(self.as_raw_mut_VectorOfu64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputOutputArray for core::Vector<u64> {
	// std::vector<uint64_t>::inputOutputArray() generated
	// ("std::vector<uint64_t>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLuint64_tG_inputOutputArray(self.as_raw_mut_VectorOfu64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
output_array_ref_forward! { core::Vector<u64> }

