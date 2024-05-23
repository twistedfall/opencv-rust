impl core::Vector<core::Vector<u8>> {
	pub fn as_raw_VectorOfVectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<u8>,
	std_vectorLstd_vectorLuint8_tGG_new_const, std_vectorLstd_vectorLuint8_tGG_delete,
	std_vectorLstd_vectorLuint8_tGG_len_const, std_vectorLstd_vectorLuint8_tGG_isEmpty_const,
	std_vectorLstd_vectorLuint8_tGG_capacity_const, std_vectorLstd_vectorLuint8_tGG_shrinkToFit,
	std_vectorLstd_vectorLuint8_tGG_reserve_size_t, std_vectorLstd_vectorLuint8_tGG_remove_size_t,
	std_vectorLstd_vectorLuint8_tGG_swap_size_t_size_t, std_vectorLstd_vectorLuint8_tGG_clear,
	std_vectorLstd_vectorLuint8_tGG_get_const_size_t, std_vectorLstd_vectorLuint8_tGG_set_size_t_const_vectorLuint8_tG,
	std_vectorLstd_vectorLuint8_tGG_push_const_vectorLuint8_tG, std_vectorLstd_vectorLuint8_tGG_insert_size_t_const_vectorLuint8_tG,
}

vector_non_copy_or_bool! { clone core::Vector<u8> }

impl ToInputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<uint8_t>>::inputArray() generated
	// ("std::vector<std::vector<uint8_t>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLuint8_tGG_inputArray_const(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<u8>> }

impl ToOutputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<uint8_t>>::outputArray() generated
	// ("std::vector<std::vector<uint8_t>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLuint8_tGG_outputArray(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<uint8_t>>::inputOutputArray() generated
	// ("std::vector<std::vector<uint8_t>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLuint8_tGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<u8>> }

