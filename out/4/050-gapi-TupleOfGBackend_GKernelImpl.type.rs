impl core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)> {
	pub fn as_raw_TupleOfGBackend_GKernelImpl(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfGBackend_GKernelImpl(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (crate::gapi::GBackend, crate::gapi::GKernelImpl),
	std_pairLcv_gapi_GBackend__cv_GKernelImplG_new_const_GBackend_GKernelImpl, std_pairLcv_gapi_GBackend__cv_GKernelImplG_delete,
	0 = arg: crate::gapi::GBackend, get_0 via std_pairLcv_gapi_GBackend__cv_GKernelImplG_get_0_const,
	1 = arg_1: crate::gapi::GKernelImpl, get_1 via std_pairLcv_gapi_GBackend__cv_GKernelImplG_get_1_const
}

