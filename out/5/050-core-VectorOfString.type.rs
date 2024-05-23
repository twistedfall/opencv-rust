impl core::Vector<String> {
	pub fn as_raw_VectorOfString(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfString(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { String,
	std_vectorLcv_StringG_new_const, std_vectorLcv_StringG_delete,
	std_vectorLcv_StringG_len_const, std_vectorLcv_StringG_isEmpty_const,
	std_vectorLcv_StringG_capacity_const, std_vectorLcv_StringG_shrinkToFit,
	std_vectorLcv_StringG_reserve_size_t, std_vectorLcv_StringG_remove_size_t,
	std_vectorLcv_StringG_swap_size_t_size_t, std_vectorLcv_StringG_clear,
	std_vectorLcv_StringG_get_const_size_t, std_vectorLcv_StringG_set_size_t_const_String,
	std_vectorLcv_StringG_push_const_String, std_vectorLcv_StringG_insert_size_t_const_String,
}

vector_non_copy_or_bool! { String }


