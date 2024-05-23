impl core::Vector<core::MatShape> {
	pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::MatShape,
	std_vectorLcv_MatShapeG_new_const, std_vectorLcv_MatShapeG_delete,
	std_vectorLcv_MatShapeG_len_const, std_vectorLcv_MatShapeG_isEmpty_const,
	std_vectorLcv_MatShapeG_capacity_const, std_vectorLcv_MatShapeG_shrinkToFit,
	std_vectorLcv_MatShapeG_reserve_size_t, std_vectorLcv_MatShapeG_remove_size_t,
	std_vectorLcv_MatShapeG_swap_size_t_size_t, std_vectorLcv_MatShapeG_clear,
	std_vectorLcv_MatShapeG_get_const_size_t, std_vectorLcv_MatShapeG_set_size_t_const_MatShape,
	std_vectorLcv_MatShapeG_push_const_MatShape, std_vectorLcv_MatShapeG_insert_size_t_const_MatShape,
}

vector_non_copy_or_bool! { clone core::MatShape }

vector_boxed_ref! { core::MatShape }

vector_extern! { BoxedRef<'t, core::MatShape>,
	std_vectorLcv_MatShapeG_new_const, std_vectorLcv_MatShapeG_delete,
	std_vectorLcv_MatShapeG_len_const, std_vectorLcv_MatShapeG_isEmpty_const,
	std_vectorLcv_MatShapeG_capacity_const, std_vectorLcv_MatShapeG_shrinkToFit,
	std_vectorLcv_MatShapeG_reserve_size_t, std_vectorLcv_MatShapeG_remove_size_t,
	std_vectorLcv_MatShapeG_swap_size_t_size_t, std_vectorLcv_MatShapeG_clear,
	std_vectorLcv_MatShapeG_get_const_size_t, std_vectorLcv_MatShapeG_set_size_t_const_MatShape,
	std_vectorLcv_MatShapeG_push_const_MatShape, std_vectorLcv_MatShapeG_insert_size_t_const_MatShape,
}


