impl core::Vector<core::Vector<u8>> {
	pub fn as_raw_VectorOfVectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<u8>,
	std_vectorLstd_vectorLunsigned_charGG_new_const, std_vectorLstd_vectorLunsigned_charGG_delete,
	std_vectorLstd_vectorLunsigned_charGG_len_const, std_vectorLstd_vectorLunsigned_charGG_isEmpty_const,
	std_vectorLstd_vectorLunsigned_charGG_capacity_const, std_vectorLstd_vectorLunsigned_charGG_shrinkToFit,
	std_vectorLstd_vectorLunsigned_charGG_reserve_size_t, std_vectorLstd_vectorLunsigned_charGG_remove_size_t,
	std_vectorLstd_vectorLunsigned_charGG_swap_size_t_size_t, std_vectorLstd_vectorLunsigned_charGG_clear,
	std_vectorLstd_vectorLunsigned_charGG_get_const_size_t, std_vectorLstd_vectorLunsigned_charGG_set_size_t_const_vectorLunsigned_charG,
	std_vectorLstd_vectorLunsigned_charGG_push_const_vectorLunsigned_charG, std_vectorLstd_vectorLunsigned_charGG_insert_size_t_const_vectorLunsigned_charG,
}

vector_non_copy_or_bool! { clone core::Vector<u8> }

impl ToInputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<unsigned char>>::inputArray() generated
	// ("std::vector<std::vector<unsigned char>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLunsigned_charGG_inputArray_const(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<u8>> }

impl ToOutputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<unsigned char>>::outputArray() generated
	// ("std::vector<std::vector<unsigned char>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLunsigned_charGG_outputArray(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<u8>> {
	// std::vector<std::vector<unsigned char>>::inputOutputArray() generated
	// ("std::vector<std::vector<unsigned char>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLunsigned_charGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<u8>> }

