impl core::Vector<crate::surface_matching::Pose3DPtr> {
	pub fn as_raw_VectorOfPose3DPtr(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPose3DPtr(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::surface_matching::Pose3DPtr,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_new_const, std_vectorLcv_ppf_match_3d_Pose3DPtrG_delete,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_len_const, std_vectorLcv_ppf_match_3d_Pose3DPtrG_isEmpty_const,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_capacity_const, std_vectorLcv_ppf_match_3d_Pose3DPtrG_shrinkToFit,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_reserve_size_t, std_vectorLcv_ppf_match_3d_Pose3DPtrG_remove_size_t,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_swap_size_t_size_t, std_vectorLcv_ppf_match_3d_Pose3DPtrG_clear,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_get_const_size_t, std_vectorLcv_ppf_match_3d_Pose3DPtrG_set_size_t_const_Pose3DPtr,
	std_vectorLcv_ppf_match_3d_Pose3DPtrG_push_const_Pose3DPtr, std_vectorLcv_ppf_match_3d_Pose3DPtrG_insert_size_t_const_Pose3DPtr,
}

vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }


