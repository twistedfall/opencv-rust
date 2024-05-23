impl core::Vector<core::Vector<core::MatShape>> {
	pub fn as_raw_VectorOfVectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::MatShape>,
	std_vectorLstd_vectorLcv_MatShapeGG_new_const, std_vectorLstd_vectorLcv_MatShapeGG_delete,
	std_vectorLstd_vectorLcv_MatShapeGG_len_const, std_vectorLstd_vectorLcv_MatShapeGG_isEmpty_const,
	std_vectorLstd_vectorLcv_MatShapeGG_capacity_const, std_vectorLstd_vectorLcv_MatShapeGG_shrinkToFit,
	std_vectorLstd_vectorLcv_MatShapeGG_reserve_size_t, std_vectorLstd_vectorLcv_MatShapeGG_remove_size_t,
	std_vectorLstd_vectorLcv_MatShapeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_MatShapeGG_clear,
	std_vectorLstd_vectorLcv_MatShapeGG_get_const_size_t, std_vectorLstd_vectorLcv_MatShapeGG_set_size_t_const_vectorLMatShapeG,
	std_vectorLstd_vectorLcv_MatShapeGG_push_const_vectorLMatShapeG, std_vectorLstd_vectorLcv_MatShapeGG_insert_size_t_const_vectorLMatShapeG,
}

vector_non_copy_or_bool! { clone core::Vector<core::MatShape> }


