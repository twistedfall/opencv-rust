ptr_extern! { crate::objdetect::DetectorParameters,
	cv_PtrLcv_aruco_DetectorParametersG_new_null_const, cv_PtrLcv_aruco_DetectorParametersG_delete, cv_PtrLcv_aruco_DetectorParametersG_getInnerPtr_const, cv_PtrLcv_aruco_DetectorParametersG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::DetectorParameters, cv_PtrLcv_aruco_DetectorParametersG_new_const_DetectorParameters }
impl core::Ptr<crate::objdetect::DetectorParameters> {
	#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::DetectorParametersTraitConst for core::Ptr<crate::objdetect::DetectorParameters> {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::DetectorParametersTrait for core::Ptr<crate::objdetect::DetectorParameters> {
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::DetectorParameters> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetectorParameters")
			.field("adaptive_thresh_win_size_min", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
			.field("adaptive_thresh_win_size_max", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
			.field("adaptive_thresh_win_size_step", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
			.field("adaptive_thresh_constant", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_constant(self))
			.field("min_marker_perimeter_rate", &crate::objdetect::DetectorParametersTraitConst::min_marker_perimeter_rate(self))
			.field("max_marker_perimeter_rate", &crate::objdetect::DetectorParametersTraitConst::max_marker_perimeter_rate(self))
			.field("polygonal_approx_accuracy_rate", &crate::objdetect::DetectorParametersTraitConst::polygonal_approx_accuracy_rate(self))
			.field("min_corner_distance_rate", &crate::objdetect::DetectorParametersTraitConst::min_corner_distance_rate(self))
			.field("min_distance_to_border", &crate::objdetect::DetectorParametersTraitConst::min_distance_to_border(self))
			.field("min_marker_distance_rate", &crate::objdetect::DetectorParametersTraitConst::min_marker_distance_rate(self))
			.field("min_group_distance", &crate::objdetect::DetectorParametersTraitConst::min_group_distance(self))
			.field("corner_refinement_method", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_method(self))
			.field("corner_refinement_win_size", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_win_size(self))
			.field("relative_corner_refinment_win_size", &crate::objdetect::DetectorParametersTraitConst::relative_corner_refinment_win_size(self))
			.field("corner_refinement_max_iterations", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_max_iterations(self))
			.field("corner_refinement_min_accuracy", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_min_accuracy(self))
			.field("marker_border_bits", &crate::objdetect::DetectorParametersTraitConst::marker_border_bits(self))
			.field("perspective_remove_pixel_per_cell", &crate::objdetect::DetectorParametersTraitConst::perspective_remove_pixel_per_cell(self))
			.field("perspective_remove_ignored_margin_per_cell", &crate::objdetect::DetectorParametersTraitConst::perspective_remove_ignored_margin_per_cell(self))
			.field("max_erroneous_bits_in_border_rate", &crate::objdetect::DetectorParametersTraitConst::max_erroneous_bits_in_border_rate(self))
			.field("min_otsu_std_dev", &crate::objdetect::DetectorParametersTraitConst::min_otsu_std_dev(self))
			.field("error_correction_rate", &crate::objdetect::DetectorParametersTraitConst::error_correction_rate(self))
			.field("april_tag_quad_decimate", &crate::objdetect::DetectorParametersTraitConst::april_tag_quad_decimate(self))
			.field("april_tag_quad_sigma", &crate::objdetect::DetectorParametersTraitConst::april_tag_quad_sigma(self))
			.field("april_tag_min_cluster_pixels", &crate::objdetect::DetectorParametersTraitConst::april_tag_min_cluster_pixels(self))
			.field("april_tag_max_nmaxima", &crate::objdetect::DetectorParametersTraitConst::april_tag_max_nmaxima(self))
			.field("april_tag_critical_rad", &crate::objdetect::DetectorParametersTraitConst::april_tag_critical_rad(self))
			.field("april_tag_max_line_fit_mse", &crate::objdetect::DetectorParametersTraitConst::april_tag_max_line_fit_mse(self))
			.field("april_tag_min_white_black_diff", &crate::objdetect::DetectorParametersTraitConst::april_tag_min_white_black_diff(self))
			.field("april_tag_deglitch", &crate::objdetect::DetectorParametersTraitConst::april_tag_deglitch(self))
			.field("detect_inverted_marker", &crate::objdetect::DetectorParametersTraitConst::detect_inverted_marker(self))
			.field("use_aruco3_detection", &crate::objdetect::DetectorParametersTraitConst::use_aruco3_detection(self))
			.field("min_side_length_canonical_img", &crate::objdetect::DetectorParametersTraitConst::min_side_length_canonical_img(self))
			.field("min_marker_length_ratio_original_img", &crate::objdetect::DetectorParametersTraitConst::min_marker_length_ratio_original_img(self))
			.finish()
	}
}

