impl core::Vector<core::Vector<i32>> {
	pub fn as_raw_VectorOfVectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<i32>,
	std_vectorLstd_vectorLintGG_new_const, std_vectorLstd_vectorLintGG_delete,
	std_vectorLstd_vectorLintGG_len_const, std_vectorLstd_vectorLintGG_isEmpty_const,
	std_vectorLstd_vectorLintGG_capacity_const, std_vectorLstd_vectorLintGG_shrinkToFit,
	std_vectorLstd_vectorLintGG_reserve_size_t, std_vectorLstd_vectorLintGG_remove_size_t,
	std_vectorLstd_vectorLintGG_swap_size_t_size_t, std_vectorLstd_vectorLintGG_clear,
	std_vectorLstd_vectorLintGG_get_const_size_t, std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG,
	std_vectorLstd_vectorLintGG_push_const_vectorLintG, std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG,
}

vector_non_copy_or_bool! { clone core::Vector<i32> }

impl ToInputArray for core::Vector<core::Vector<i32>> {
	// std::vector<std::vector<int>>::inputArray() generated
	// ("std::vector<std::vector<int>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLintGG_inputArray_const(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

input_array_ref_forward! { core::Vector<core::Vector<i32>> }

impl ToOutputArray for core::Vector<core::Vector<i32>> {
	// std::vector<std::vector<int>>::outputArray() generated
	// ("std::vector<std::vector<int>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLintGG_outputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl ToInputOutputArray for core::Vector<core::Vector<i32>> {
	// std::vector<std::vector<int>>::inputOutputArray() generated
	// ("std::vector<std::vector<int>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLintGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

output_array_ref_forward! { core::Vector<core::Vector<i32>> }

