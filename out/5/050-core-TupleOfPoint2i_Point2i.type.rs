impl core::Tuple<(core::Point2i, core::Point2i)> {
	pub fn as_raw_TupleOfPoint2i_Point2i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfPoint2i_Point2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (core::Point2i, core::Point2i),
	std_pairLcv_Point2i__cv_Point2iG_new_const_Point2i_Point2i, std_pairLcv_Point2i__cv_Point2iG_delete,
	0 = arg: core::Point2i, get_0 via std_pairLcv_Point2i__cv_Point2iG_get_0_const,
	1 = arg_1: core::Point2i, get_1 via std_pairLcv_Point2i__cv_Point2iG_get_1_const
}

