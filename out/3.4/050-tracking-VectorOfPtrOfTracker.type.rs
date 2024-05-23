impl core::Vector<core::Ptr<crate::tracking::Tracker>> {
	pub fn as_raw_VectorOfPtrOfTracker(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfTracker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::tracking::Tracker>,
	std_vectorLcv_PtrLcv_TrackerGG_new_const, std_vectorLcv_PtrLcv_TrackerGG_delete,
	std_vectorLcv_PtrLcv_TrackerGG_len_const, std_vectorLcv_PtrLcv_TrackerGG_isEmpty_const,
	std_vectorLcv_PtrLcv_TrackerGG_capacity_const, std_vectorLcv_PtrLcv_TrackerGG_shrinkToFit,
	std_vectorLcv_PtrLcv_TrackerGG_reserve_size_t, std_vectorLcv_PtrLcv_TrackerGG_remove_size_t,
	std_vectorLcv_PtrLcv_TrackerGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_TrackerGG_clear,
	std_vectorLcv_PtrLcv_TrackerGG_get_const_size_t, std_vectorLcv_PtrLcv_TrackerGG_set_size_t_const_PtrLTrackerG,
	std_vectorLcv_PtrLcv_TrackerGG_push_const_PtrLTrackerG, std_vectorLcv_PtrLcv_TrackerGG_insert_size_t_const_PtrLTrackerG,
}

vector_non_copy_or_bool! { core::Ptr<crate::tracking::Tracker> }


