impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)> {
	pub fn as_raw_TupleOfGMat_GMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfGMat_GMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat),
	std_tupleLcv_GMat__cv_GMat__cv_GMatG_new_const_GMat_GMat_GMat, std_tupleLcv_GMat__cv_GMat__cv_GMatG_delete,
	0 = arg: crate::gapi::GMat, get_0 via std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_0_const,
	1 = arg_1: crate::gapi::GMat, get_1 via std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_1_const,
	2 = arg_2: crate::gapi::GMat, get_2 via std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_2_const
}

