impl core::Vector<core::Tuple<(core::Point2i, core::Point2i)>> {
	pub fn as_raw_VectorOfTupleOfPoint2i_Point2i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfTupleOfPoint2i_Point2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Tuple<(core::Point2i, core::Point2i)>,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_new_const, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_delete,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_len_const, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_isEmpty_const,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_capacity_const, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_shrinkToFit,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_reserve_size_t, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_remove_size_t,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_swap_size_t_size_t, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_clear,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_get_const_size_t, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_set_size_t_const_pairLcv_Point2i__cv_Point2iG,
	std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_push_const_pairLcv_Point2i__cv_Point2iG, std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_insert_size_t_const_pairLcv_Point2i__cv_Point2iG,
}

vector_non_copy_or_bool! { core::Tuple<(core::Point2i, core::Point2i)> }


