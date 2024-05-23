impl core::Vector<core::Vector<bool>> {
	pub fn as_raw_VectorOfVectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<bool>,
	std_vectorLstd_vectorLboolGG_new_const, std_vectorLstd_vectorLboolGG_delete,
	std_vectorLstd_vectorLboolGG_len_const, std_vectorLstd_vectorLboolGG_isEmpty_const,
	std_vectorLstd_vectorLboolGG_capacity_const, std_vectorLstd_vectorLboolGG_shrinkToFit,
	std_vectorLstd_vectorLboolGG_reserve_size_t, std_vectorLstd_vectorLboolGG_remove_size_t,
	std_vectorLstd_vectorLboolGG_swap_size_t_size_t, std_vectorLstd_vectorLboolGG_clear,
	std_vectorLstd_vectorLboolGG_get_const_size_t, std_vectorLstd_vectorLboolGG_set_size_t_const_vectorLboolG,
	std_vectorLstd_vectorLboolGG_push_const_vectorLboolG, std_vectorLstd_vectorLboolGG_insert_size_t_const_vectorLboolG,
}

vector_non_copy_or_bool! { clone core::Vector<bool> }

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputArray for core::Vector<core::Vector<bool>> {
	// std::vector<std::vector<bool>>::inputArray() generated
	// ("std::vector<std::vector<bool>>::inputArray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLboolGG_inputArray_const(self.as_raw_VectorOfVectorOfbool(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
input_array_ref_forward! { core::Vector<core::Vector<bool>> }

#[cfg(ocvrs_opencv_branch_5)]
impl ToOutputArray for core::Vector<core::Vector<bool>> {
	// std::vector<std::vector<bool>>::outputArray() generated
	// ("std::vector<std::vector<bool>>::outputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLboolGG_outputArray(self.as_raw_mut_VectorOfVectorOfbool(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
impl ToInputOutputArray for core::Vector<core::Vector<bool>> {
	// std::vector<std::vector<bool>>::inputOutputArray() generated
	// ("std::vector<std::vector<bool>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLboolGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfbool(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

#[cfg(ocvrs_opencv_branch_5)]
output_array_ref_forward! { core::Vector<core::Vector<bool>> }

