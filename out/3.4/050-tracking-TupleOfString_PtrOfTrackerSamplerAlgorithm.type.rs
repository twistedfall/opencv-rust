impl core::Tuple<(String, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>)> {
	pub fn as_raw_TupleOfString_PtrOfTrackerSamplerAlgorithm(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfString_PtrOfTrackerSamplerAlgorithm(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (String, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>),
	std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_new_const_String_PtrLTrackerSamplerAlgorithmG, std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_delete,
	0 = arg: String, get_0 via std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_get_0_const,
	1 = arg_1: core::Ptr<crate::tracking::TrackerSamplerAlgorithm>, get_1 via std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_get_1_const
}

