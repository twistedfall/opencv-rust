impl core::Tuple<(String, core::Ptr<crate::tracking::TrackerFeature>)> {
	pub fn as_raw_TupleOfString_PtrOfTrackerFeature(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfString_PtrOfTrackerFeature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (String, core::Ptr<crate::tracking::TrackerFeature>),
	std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_new_const_String_PtrLTrackerFeatureG, std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_delete,
	0 = arg: String, get_0 via std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_get_0_const,
	1 = arg_1: core::Ptr<crate::tracking::TrackerFeature>, get_1 via std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_get_1_const
}

