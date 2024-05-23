impl core::Tuple<(core::Ptr<crate::tracking::TrackerTargetState>, f32)> {
	pub fn as_raw_TupleOfPtrOfTrackerTargetState_f32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfPtrOfTrackerTargetState_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (core::Ptr<crate::tracking::TrackerTargetState>, f32),
	std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_new_const_PtrLTrackerTargetStateG_float, std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_delete,
	0 = arg: core::Ptr<crate::tracking::TrackerTargetState>, get_0 via std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_get_0_const,
	1 = arg_1: f32, get_1 via std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_get_1_const
}

