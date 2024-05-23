impl core::Vector<crate::line_descriptor::KeyLine> {
	pub fn as_raw_VectorOfKeyLine(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfKeyLine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::line_descriptor::KeyLine,
	std_vectorLcv_line_descriptor_KeyLineG_new_const, std_vectorLcv_line_descriptor_KeyLineG_delete,
	std_vectorLcv_line_descriptor_KeyLineG_len_const, std_vectorLcv_line_descriptor_KeyLineG_isEmpty_const,
	std_vectorLcv_line_descriptor_KeyLineG_capacity_const, std_vectorLcv_line_descriptor_KeyLineG_shrinkToFit,
	std_vectorLcv_line_descriptor_KeyLineG_reserve_size_t, std_vectorLcv_line_descriptor_KeyLineG_remove_size_t,
	std_vectorLcv_line_descriptor_KeyLineG_swap_size_t_size_t, std_vectorLcv_line_descriptor_KeyLineG_clear,
	std_vectorLcv_line_descriptor_KeyLineG_get_const_size_t, std_vectorLcv_line_descriptor_KeyLineG_set_size_t_const_KeyLine,
	std_vectorLcv_line_descriptor_KeyLineG_push_const_KeyLine, std_vectorLcv_line_descriptor_KeyLineG_insert_size_t_const_KeyLine,
}

vector_copy_non_bool! { crate::line_descriptor::KeyLine,
	std_vectorLcv_line_descriptor_KeyLineG_data_const, std_vectorLcv_line_descriptor_KeyLineG_dataMut, cv_fromSlice_const_const_KeyLineX_size_t,
	std_vectorLcv_line_descriptor_KeyLineG_clone_const,
}


