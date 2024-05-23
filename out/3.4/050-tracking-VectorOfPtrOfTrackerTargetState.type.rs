impl core::Vector<core::Ptr<crate::tracking::TrackerTargetState>> {
	pub fn as_raw_VectorOfPtrOfTrackerTargetState(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfTrackerTargetState(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::tracking::TrackerTargetState>,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_new_const, std_vectorLcv_PtrLcv_TrackerTargetStateGG_delete,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_len_const, std_vectorLcv_PtrLcv_TrackerTargetStateGG_isEmpty_const,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_capacity_const, std_vectorLcv_PtrLcv_TrackerTargetStateGG_shrinkToFit,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_reserve_size_t, std_vectorLcv_PtrLcv_TrackerTargetStateGG_remove_size_t,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_TrackerTargetStateGG_clear,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_get_const_size_t, std_vectorLcv_PtrLcv_TrackerTargetStateGG_set_size_t_const_PtrLTrackerTargetStateG,
	std_vectorLcv_PtrLcv_TrackerTargetStateGG_push_const_PtrLTrackerTargetStateG, std_vectorLcv_PtrLcv_TrackerTargetStateGG_insert_size_t_const_PtrLTrackerTargetStateG,
}

vector_non_copy_or_bool! { core::Ptr<crate::tracking::TrackerTargetState> }


