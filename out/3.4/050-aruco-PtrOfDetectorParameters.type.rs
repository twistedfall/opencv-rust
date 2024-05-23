ptr_extern! { crate::aruco::DetectorParameters,
	cv_PtrLcv_aruco_DetectorParametersG_new_null_const, cv_PtrLcv_aruco_DetectorParametersG_delete, cv_PtrLcv_aruco_DetectorParametersG_getInnerPtr_const, cv_PtrLcv_aruco_DetectorParametersG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::DetectorParameters, cv_PtrLcv_aruco_DetectorParametersG_new_const_DetectorParameters }
impl core::Ptr<crate::aruco::DetectorParameters> {
	#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::DetectorParametersTraitConst for core::Ptr<crate::aruco::DetectorParameters> {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::DetectorParametersTrait for core::Ptr<crate::aruco::DetectorParameters> {
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::aruco::DetectorParameters> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetectorParameters")
			.field("adaptive_thresh_win_size_min", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
			.field("adaptive_thresh_win_size_max", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
			.field("adaptive_thresh_win_size_step", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
			.field("adaptive_thresh_constant", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_constant(self))
			.field("min_marker_perimeter_rate", &crate::aruco::DetectorParametersTraitConst::min_marker_perimeter_rate(self))
			.field("max_marker_perimeter_rate", &crate::aruco::DetectorParametersTraitConst::max_marker_perimeter_rate(self))
			.field("polygonal_approx_accuracy_rate", &crate::aruco::DetectorParametersTraitConst::polygonal_approx_accuracy_rate(self))
			.field("min_corner_distance_rate", &crate::aruco::DetectorParametersTraitConst::min_corner_distance_rate(self))
			.field("min_distance_to_border", &crate::aruco::DetectorParametersTraitConst::min_distance_to_border(self))
			.field("min_marker_distance_rate", &crate::aruco::DetectorParametersTraitConst::min_marker_distance_rate(self))
			.field("corner_refinement_method", &crate::aruco::DetectorParametersTraitConst::corner_refinement_method(self))
			.field("corner_refinement_win_size", &crate::aruco::DetectorParametersTraitConst::corner_refinement_win_size(self))
			.field("corner_refinement_max_iterations", &crate::aruco::DetectorParametersTraitConst::corner_refinement_max_iterations(self))
			.field("corner_refinement_min_accuracy", &crate::aruco::DetectorParametersTraitConst::corner_refinement_min_accuracy(self))
			.field("marker_border_bits", &crate::aruco::DetectorParametersTraitConst::marker_border_bits(self))
			.field("perspective_remove_pixel_per_cell", &crate::aruco::DetectorParametersTraitConst::perspective_remove_pixel_per_cell(self))
			.field("perspective_remove_ignored_margin_per_cell", &crate::aruco::DetectorParametersTraitConst::perspective_remove_ignored_margin_per_cell(self))
			.field("max_erroneous_bits_in_border_rate", &crate::aruco::DetectorParametersTraitConst::max_erroneous_bits_in_border_rate(self))
			.field("min_otsu_std_dev", &crate::aruco::DetectorParametersTraitConst::min_otsu_std_dev(self))
			.field("error_correction_rate", &crate::aruco::DetectorParametersTraitConst::error_correction_rate(self))
			.field("april_tag_quad_decimate", &crate::aruco::DetectorParametersTraitConst::april_tag_quad_decimate(self))
			.field("april_tag_quad_sigma", &crate::aruco::DetectorParametersTraitConst::april_tag_quad_sigma(self))
			.field("april_tag_min_cluster_pixels", &crate::aruco::DetectorParametersTraitConst::april_tag_min_cluster_pixels(self))
			.field("april_tag_max_nmaxima", &crate::aruco::DetectorParametersTraitConst::april_tag_max_nmaxima(self))
			.field("april_tag_critical_rad", &crate::aruco::DetectorParametersTraitConst::april_tag_critical_rad(self))
			.field("april_tag_max_line_fit_mse", &crate::aruco::DetectorParametersTraitConst::april_tag_max_line_fit_mse(self))
			.field("april_tag_min_white_black_diff", &crate::aruco::DetectorParametersTraitConst::april_tag_min_white_black_diff(self))
			.field("april_tag_deglitch", &crate::aruco::DetectorParametersTraitConst::april_tag_deglitch(self))
			.field("detect_inverted_marker", &crate::aruco::DetectorParametersTraitConst::detect_inverted_marker(self))
			.finish()
	}
}

