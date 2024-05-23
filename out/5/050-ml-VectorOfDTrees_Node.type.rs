impl core::Vector<crate::ml::DTrees_Node> {
	pub fn as_raw_VectorOfDTrees_Node(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::ml::DTrees_Node,
	std_vectorLcv_ml_DTrees_NodeG_new_const, std_vectorLcv_ml_DTrees_NodeG_delete,
	std_vectorLcv_ml_DTrees_NodeG_len_const, std_vectorLcv_ml_DTrees_NodeG_isEmpty_const,
	std_vectorLcv_ml_DTrees_NodeG_capacity_const, std_vectorLcv_ml_DTrees_NodeG_shrinkToFit,
	std_vectorLcv_ml_DTrees_NodeG_reserve_size_t, std_vectorLcv_ml_DTrees_NodeG_remove_size_t,
	std_vectorLcv_ml_DTrees_NodeG_swap_size_t_size_t, std_vectorLcv_ml_DTrees_NodeG_clear,
	std_vectorLcv_ml_DTrees_NodeG_get_const_size_t, std_vectorLcv_ml_DTrees_NodeG_set_size_t_const_Node,
	std_vectorLcv_ml_DTrees_NodeG_push_const_Node, std_vectorLcv_ml_DTrees_NodeG_insert_size_t_const_Node,
}

vector_non_copy_or_bool! { crate::ml::DTrees_Node }

vector_boxed_ref! { crate::ml::DTrees_Node }

vector_extern! { BoxedRef<'t, crate::ml::DTrees_Node>,
	std_vectorLcv_ml_DTrees_NodeG_new_const, std_vectorLcv_ml_DTrees_NodeG_delete,
	std_vectorLcv_ml_DTrees_NodeG_len_const, std_vectorLcv_ml_DTrees_NodeG_isEmpty_const,
	std_vectorLcv_ml_DTrees_NodeG_capacity_const, std_vectorLcv_ml_DTrees_NodeG_shrinkToFit,
	std_vectorLcv_ml_DTrees_NodeG_reserve_size_t, std_vectorLcv_ml_DTrees_NodeG_remove_size_t,
	std_vectorLcv_ml_DTrees_NodeG_swap_size_t_size_t, std_vectorLcv_ml_DTrees_NodeG_clear,
	std_vectorLcv_ml_DTrees_NodeG_get_const_size_t, std_vectorLcv_ml_DTrees_NodeG_set_size_t_const_Node,
	std_vectorLcv_ml_DTrees_NodeG_push_const_Node, std_vectorLcv_ml_DTrees_NodeG_insert_size_t_const_Node,
}


