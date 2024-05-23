impl core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> {
	pub fn as_raw_TupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (crate::dnn::Backend, crate::dnn::Target),
	std_pairLcv_dnn_Backend__cv_dnn_TargetG_new_const_Backend_Target, std_pairLcv_dnn_Backend__cv_dnn_TargetG_delete,
	0 = arg: crate::dnn::Backend, get_0 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_0_const,
	1 = arg_1: crate::dnn::Target, get_1 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_1_const
}

