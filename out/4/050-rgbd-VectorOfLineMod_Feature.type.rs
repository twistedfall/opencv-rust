impl core::Vector<crate::rgbd::LineMod_Feature> {
	pub fn as_raw_VectorOfLineMod_Feature(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfLineMod_Feature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::rgbd::LineMod_Feature,
	std_vectorLcv_linemod_FeatureG_new_const, std_vectorLcv_linemod_FeatureG_delete,
	std_vectorLcv_linemod_FeatureG_len_const, std_vectorLcv_linemod_FeatureG_isEmpty_const,
	std_vectorLcv_linemod_FeatureG_capacity_const, std_vectorLcv_linemod_FeatureG_shrinkToFit,
	std_vectorLcv_linemod_FeatureG_reserve_size_t, std_vectorLcv_linemod_FeatureG_remove_size_t,
	std_vectorLcv_linemod_FeatureG_swap_size_t_size_t, std_vectorLcv_linemod_FeatureG_clear,
	std_vectorLcv_linemod_FeatureG_get_const_size_t, std_vectorLcv_linemod_FeatureG_set_size_t_const_Feature,
	std_vectorLcv_linemod_FeatureG_push_const_Feature, std_vectorLcv_linemod_FeatureG_insert_size_t_const_Feature,
}

vector_copy_non_bool! { crate::rgbd::LineMod_Feature,
	std_vectorLcv_linemod_FeatureG_data_const, std_vectorLcv_linemod_FeatureG_dataMut, cv_fromSlice_const_const_FeatureX_size_t,
	std_vectorLcv_linemod_FeatureG_clone_const,
}


