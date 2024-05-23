impl core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)> {
	pub fn as_raw_TupleOfGMat_GScalar(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfGMat_GScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (crate::gapi::GMat, crate::gapi::GScalar),
	std_tupleLcv_GMat__cv_GScalarG_new_const_GMat_GScalar, std_tupleLcv_GMat__cv_GScalarG_delete,
	0 = arg: crate::gapi::GMat, get_0 via std_tupleLcv_GMat__cv_GScalarG_get_0_const,
	1 = arg_1: crate::gapi::GScalar, get_1 via std_tupleLcv_GMat__cv_GScalarG_get_1_const
}

