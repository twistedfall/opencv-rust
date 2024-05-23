impl core::Vector<core::Tuple<(core::UMat, u8)>> {
	pub fn as_raw_VectorOfTupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfTupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Tuple<(core::UMat, u8)>,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_new_const, std_vectorLstd_pairLcv_UMat__unsigned_charGG_delete,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_len_const, std_vectorLstd_pairLcv_UMat__unsigned_charGG_isEmpty_const,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_capacity_const, std_vectorLstd_pairLcv_UMat__unsigned_charGG_shrinkToFit,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_reserve_size_t, std_vectorLstd_pairLcv_UMat__unsigned_charGG_remove_size_t,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_swap_size_t_size_t, std_vectorLstd_pairLcv_UMat__unsigned_charGG_clear,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_get_const_size_t, std_vectorLstd_pairLcv_UMat__unsigned_charGG_set_size_t_const_pairLcv_UMat__unsigned_charG,
	std_vectorLstd_pairLcv_UMat__unsigned_charGG_push_const_pairLcv_UMat__unsigned_charG, std_vectorLstd_pairLcv_UMat__unsigned_charGG_insert_size_t_const_pairLcv_UMat__unsigned_charG,
}

vector_non_copy_or_bool! { core::Tuple<(core::UMat, u8)> }


