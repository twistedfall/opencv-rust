ptr_extern! { crate::mcc::MCC_DetectorParameters,
	cv_PtrLcv_mcc_DetectorParametersG_new_null_const, cv_PtrLcv_mcc_DetectorParametersG_delete, cv_PtrLcv_mcc_DetectorParametersG_getInnerPtr_const, cv_PtrLcv_mcc_DetectorParametersG_getInnerPtrMut
}

ptr_extern_ctor! { crate::mcc::MCC_DetectorParameters, cv_PtrLcv_mcc_DetectorParametersG_new_const_DetectorParameters }
impl core::Ptr<crate::mcc::MCC_DetectorParameters> {
	#[inline] pub fn as_raw_PtrOfMCC_DetectorParameters(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_DetectorParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mcc::MCC_DetectorParametersTraitConst for core::Ptr<crate::mcc::MCC_DetectorParameters> {
	#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_DetectorParametersTrait for core::Ptr<crate::mcc::MCC_DetectorParameters> {
	#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mcc::MCC_DetectorParameters> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMCC_DetectorParameters")
			.field("adaptive_thresh_win_size_min", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
			.field("adaptive_thresh_win_size_max", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
			.field("adaptive_thresh_win_size_step", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
			.field("adaptive_thresh_constant", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_constant(self))
			.field("min_contours_area_rate", &crate::mcc::MCC_DetectorParametersTraitConst::min_contours_area_rate(self))
			.field("min_contours_area", &crate::mcc::MCC_DetectorParametersTraitConst::min_contours_area(self))
			.field("confidence_threshold", &crate::mcc::MCC_DetectorParametersTraitConst::confidence_threshold(self))
			.field("min_contour_solidity", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_solidity(self))
			.field("find_candidates_approx_poly_dp_eps_multiplier", &crate::mcc::MCC_DetectorParametersTraitConst::find_candidates_approx_poly_dp_eps_multiplier(self))
			.field("border_width", &crate::mcc::MCC_DetectorParametersTraitConst::border_width(self))
			.field("b0factor", &crate::mcc::MCC_DetectorParametersTraitConst::b0factor(self))
			.field("max_error", &crate::mcc::MCC_DetectorParametersTraitConst::max_error(self))
			.field("min_contour_points_allowed", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_points_allowed(self))
			.field("min_contour_length_allowed", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_length_allowed(self))
			.field("min_inter_contour_distance", &crate::mcc::MCC_DetectorParametersTraitConst::min_inter_contour_distance(self))
			.field("min_inter_checker_distance", &crate::mcc::MCC_DetectorParametersTraitConst::min_inter_checker_distance(self))
			.field("min_image_size", &crate::mcc::MCC_DetectorParametersTraitConst::min_image_size(self))
			.field("min_group_size", &crate::mcc::MCC_DetectorParametersTraitConst::min_group_size(self))
			.finish()
	}
}

