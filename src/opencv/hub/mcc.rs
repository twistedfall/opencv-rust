#![allow(unused_parens)]
//! # Macbeth Chart module
//! 
//! Introduction
//! ------------
//! 
//! ColorCharts are a tool for calibrating the color profile of camera, which not
//! only depends on the intrinsic and extrinsic parameters of camera but also on the
//! lighting conditions. This is done by taking the image of a chart, such that the
//! value of its colors present in it known, in the image the color values changes
//! depeding on many variables, this gives us the colors initially present and the
//! colors that are present in the image, based on this information we can apply any
//! suitable algorithm to find the actual color of all the objects present in the
//! image.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::MCC_CChecker, super::MCC_CCheckerDraw, super::MCC_DetectorParametersTrait, super::MCC_CCheckerDetector };
}

/// Standard Macbeth Chart with 24 squares
pub const MCC_MCC24: i32 = 0;
/// DigitalSG with 140 squares
pub const MCC_SG140: i32 = 1;
/// DKK color chart with 12 squares and 6 rectangle
pub const MCC_VINYL18: i32 = 2;
/// TYPECHART
/// 
/// \brief enum to hold the type of the checker
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MCC_TYPECHART {
	/// Standard Macbeth Chart with 24 squares
	MCC24 = 0,
	/// DigitalSG with 140 squares
	SG140 = 1,
	/// DKK color chart with 12 squares and 6 rectangle
	VINYL18 = 2,
}

opencv_type_enum! { crate::mcc::MCC_TYPECHART }

/// CChecker
/// 
/// \brief checker object
/// 
///    This class contains the information about the detected checkers,i.e, their
///    type, the corners of the chart, the color profile, the cost, centers chart,
///    etc.
pub trait MCC_CChecker {
	fn as_raw_MCC_CChecker(&self) -> *const c_void;
	fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void;

	fn set_target(&mut self, _target: crate::mcc::MCC_TYPECHART) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setTarget_TYPECHART(self.as_raw_mut_MCC_CChecker(), _target) }.into_result()
	}
	
	fn set_box(&mut self, mut _box: core::Vector::<core::Point2f>) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setBox_vector_Point2f_(self.as_raw_mut_MCC_CChecker(), _box.as_raw_mut_VectorOfPoint2f()) }.into_result()
	}
	
	fn set_charts_rgb(&mut self, mut _charts_rgb: core::Mat) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setChartsRGB_Mat(self.as_raw_mut_MCC_CChecker(), _charts_rgb.as_raw_mut_Mat()) }.into_result()
	}
	
	fn set_charts_y_cb_cr(&mut self, mut _charts_y_cb_cr: core::Mat) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setChartsYCbCr_Mat(self.as_raw_mut_MCC_CChecker(), _charts_y_cb_cr.as_raw_mut_Mat()) }.into_result()
	}
	
	fn set_cost(&mut self, _cost: f32) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setCost_float(self.as_raw_mut_MCC_CChecker(), _cost) }.into_result()
	}
	
	fn set_center(&mut self, _center: core::Point2f) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setCenter_Point2f(self.as_raw_mut_MCC_CChecker(), _center.opencv_as_extern()) }.into_result()
	}
	
	fn get_target(&mut self) -> Result<crate::mcc::MCC_TYPECHART> {
		unsafe { sys::cv_mcc_CChecker_getTarget(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
	fn get_box(&mut self) -> Result<core::Vector::<core::Point2f>> {
		unsafe { sys::cv_mcc_CChecker_getBox(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } )
	}
	
	fn get_charts_rgb(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_mcc_CChecker_getChartsRGB(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_charts_y_cb_cr(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_mcc_CChecker_getChartsYCbCr(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_cost(&mut self) -> Result<f32> {
		unsafe { sys::cv_mcc_CChecker_getCost(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
	fn get_center(&mut self) -> Result<core::Point2f> {
		unsafe { sys::cv_mcc_CChecker_getCenter(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
}

impl dyn MCC_CChecker + '_ {
	/// \brief Create a new CChecker object.
	/// \return A pointer to the implementation of the CChecker
	pub fn create() -> Result<core::Ptr::<dyn crate::mcc::MCC_CChecker>> {
		unsafe { sys::cv_mcc_CChecker_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CChecker>::opencv_from_extern(r) } )
	}
	
}
/// A class to find the positions of the ColorCharts in the image.
pub trait MCC_CCheckerDetector: core::AlgorithmTrait {
	fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void;
	fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void;

	/// \brief Set the net which will be used to find the approximate
	///        bounding boxes for the color charts.
	/// 
	/// It is not necessary to use this, but this usually results in
	/// better detection rate.
	/// 
	/// \param net the neural network, if the network in empty, then
	///            the function will return false.
	/// \return true if it was able to set the detector's network,
	///        false otherwise.
	fn set_net(&mut self, mut net: crate::dnn::Net) -> Result<bool> {
		unsafe { sys::cv_mcc_CCheckerDetector_setNet_Net(self.as_raw_mut_MCC_CCheckerDetector(), net.as_raw_mut_Net()) }.into_result()
	}
	
	/// \brief Find the ColorCharts in the given image.
	/// 
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param regionsOfInterest regions of image to look for the chart, if
	///                          it is empty, charts are looked for in the
	///                          entire image
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	/// 
	/// ## C++ default parameters
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	fn process_with_roi(&mut self, image: &dyn core::ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: &core::Vector::<core::Rect>, nc: i32, use_net: bool, params: &core::Ptr::<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vector_Rect_R_const_int_bool_const_Ptr_DetectorParameters_R(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, regions_of_interest.as_raw_VectorOfRect(), nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters()) }.into_result()
	}
	
	/// \brief Find the ColorCharts in the given image.
	/// 
	/// Differs from the above one only in the arguments.
	/// 
	/// This version searches for the chart in the full image.
	/// 
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	/// 
	/// ## C++ default parameters
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	fn process(&mut self, image: &dyn core::ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, nc: i32, use_net: bool, params: &core::Ptr::<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_Ptr_DetectorParameters_R(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters()) }.into_result()
	}
	
	/// \brief Get the best color checker. By the best it means the one
	///        detected with the highest confidence.
	/// \return checker A single colorchecker, if atleast one colorchecker
	///                was detected, 'nullptr' otherwise.
	fn get_best_color_checker(&mut self) -> Result<core::Ptr::<dyn crate::mcc::MCC_CChecker>> {
		unsafe { sys::cv_mcc_CCheckerDetector_getBestColorChecker(self.as_raw_mut_MCC_CCheckerDetector()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CChecker>::opencv_from_extern(r) } )
	}
	
	/// \brief Get the list of all detected colorcheckers
	/// \return checkers vector of colorcheckers
	fn get_list_color_checker(&mut self) -> Result<core::Vector::<core::Ptr::<dyn crate::mcc::MCC_CChecker>>> {
		unsafe { sys::cv_mcc_CCheckerDetector_getListColorChecker(self.as_raw_mut_MCC_CCheckerDetector()) }.into_result().map(|r| unsafe { core::Vector::<core::Ptr::<dyn crate::mcc::MCC_CChecker>>::opencv_from_extern(r) } )
	}
	
}

impl dyn MCC_CCheckerDetector + '_ {
	/// \brief Returns the implementation of the CCheckerDetector.
	pub fn create() -> Result<core::Ptr::<dyn crate::mcc::MCC_CCheckerDetector>> {
		unsafe { sys::cv_mcc_CCheckerDetector_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CCheckerDetector>::opencv_from_extern(r) } )
	}
	
}
/// \brief checker draw
/// 
/// This class contains the functions for drawing a detected chart.  This class
/// expects a pointer to the checker which will be drawn by this object in the
/// constructor and then later on whenever the draw function is called the
/// checker will be drawn. Remember that it is not possible to change the
/// checkers which will be draw by a given object, as it is decided in the
/// constructor itself. If you want to draw some other object you can create a
/// new CCheckerDraw instance.
/// 
/// The reason for this type of design is that in some videos we can assume that
/// the checker is always in the same position, even if the image changes, so
/// the drawing will always take place at the same position.
pub trait MCC_CCheckerDraw {
	fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void;
	fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void;

	/// \brief Draws the checker to the given image.
	/// \param img image in color space BGR
	/// \return void
	fn draw(&mut self, img: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(img);
		unsafe { sys::cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(self.as_raw_mut_MCC_CCheckerDraw(), img.as_raw__InputOutputArray()) }.into_result()
	}
	
}

impl dyn MCC_CCheckerDraw + '_ {
	/// \brief Create a new CCheckerDraw object.
	/// \param pChecker The checker which will be drawn by this object.
	/// \param color The color by with which the squares of the checker
	///              will be drawn
	/// \param thickness The thickness with which the sqaures will be
	///                  drawn
	/// \return A pointer to the implementation of the CCheckerDraw
	/// 
	/// ## C++ default parameters
	/// * color: CV_RGB(0,250,0)
	/// * thickness: 2
	pub fn create(mut p_checker: core::Ptr::<dyn crate::mcc::MCC_CChecker>, color: core::Scalar, thickness: i32) -> Result<core::Ptr::<dyn crate::mcc::MCC_CCheckerDraw>> {
		unsafe { sys::cv_mcc_CCheckerDraw_create_Ptr_CChecker__Scalar_int(p_checker.as_raw_mut_PtrOfMCC_CChecker(), color.opencv_as_extern(), thickness) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CCheckerDraw>::opencv_from_extern(r) } )
	}
	
}
/// Parameters for the detectMarker process:
/// - int adaptiveThreshWinSizeMin : minimum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 23).
/// - int adaptiveThreshWinSizeMax : maximum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 153).
/// - int adaptiveThreshWinSizeStep : increments from adaptiveThreshWinSizeMin to
///                                   adaptiveThreshWinSizeMax during the
///                                   thresholding (default 16).
/// - double adaptiveThreshConstant : constant for adaptive thresholding before
///                                   finding contours (default 7)
/// - double minContoursAreaRate : determine minimum area for marker contour to
///                                be detected. This is defined as a rate respect
///                                to the area of the input image. Used only if
///                                neural network is used (default 0.003).
/// - double minContoursArea : determine minimum area for marker contour to be
///                            detected. This is defined as the actual area. Used
///                            only if neural network is not used (default 100).
/// - double confidenceThreshold : minimum confidence for a bounding box detected
///                                by neural network to classify as
///                                detection.(default 0.5)
///                                (0<=confidenceThreshold<=1)
/// - double minContourSolidity : minimum solidity of a contour for it be
///                               detected as a square in the chart. (default
///                               0.9).
/// - double findCandidatesApproxPolyDPEpsMultiplier : multipler to be used in
///                                                    cv::ApproxPolyDP function
///                                                    (default 0.05)
/// - int borderWidth : width of the padding used to pass the inital neural
///                    network detection in the succeeding system.(default 0)
/// - float B0factor : distance between two neighbours squares of the same chart.
///                    Defined as the ratio between distance and large dimension
///                    of square (default 1.25)
/// - float maxError : maximum allowed error in the detection of a chart.
///                    default(0.1)
/// - int minContourPointsAllowed : minium points in a detected contour.
///                                 default(4)
/// - int minContourLengthAllowed : minimum length of a countour. default(100)
/// - int minInterContourDistance : minimum distance between two contours.
///                                 default(100)
/// - int minInterCheckerDistance : minimum distance between two checkers.
///                                 default(10000)
/// - int minImageSize : minimum size of the smaller dimension of the image.
///                      default(1000)
/// - unsigned minGroupSize : minimum number of a squared of a chart that must be
///                           detected. default(4)
pub trait MCC_DetectorParametersTrait {
	fn as_raw_MCC_DetectorParameters(&self) -> *const c_void;
	fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void;

	fn adaptive_thresh_win_size_min(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_min")
	}
	
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_min")
	}
	
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_max")
	}
	
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_max")
	}
	
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_step")
	}
	
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_step")
	}
	
	fn adaptive_thresh_constant(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshConstant_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_constant")
	}
	
	fn set_adaptive_thresh_constant(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshConstant_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_constant")
	}
	
	fn min_contours_area_rate(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContoursAreaRate_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contours_area_rate")
	}
	
	fn set_min_contours_area_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContoursAreaRate_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contours_area_rate")
	}
	
	fn min_contours_area(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContoursArea_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contours_area")
	}
	
	fn set_min_contours_area(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContoursArea_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contours_area")
	}
	
	fn confidence_threshold(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropConfidenceThreshold_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: confidence_threshold")
	}
	
	fn set_confidence_threshold(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropConfidenceThreshold_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_confidence_threshold")
	}
	
	fn min_contour_solidity(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourSolidity_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_solidity")
	}
	
	fn set_min_contour_solidity(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourSolidity_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_solidity")
	}
	
	fn find_candidates_approx_poly_dp_eps_multiplier(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropFindCandidatesApproxPolyDPEpsMultiplier_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: find_candidates_approx_poly_dp_eps_multiplier")
	}
	
	fn set_find_candidates_approx_poly_dp_eps_multiplier(&mut self, val: f64) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropFindCandidatesApproxPolyDPEpsMultiplier_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_find_candidates_approx_poly_dp_eps_multiplier")
	}
	
	fn border_width(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropBorderWidth_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: border_width")
	}
	
	fn set_border_width(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropBorderWidth_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_border_width")
	}
	
	fn b0factor(&self) -> f32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropB0factor_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: b0factor")
	}
	
	fn set_b0factor(&mut self, val: f32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropB0factor_float(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_b0factor")
	}
	
	fn max_error(&self) -> f32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMaxError_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: max_error")
	}
	
	fn set_max_error(&mut self, val: f32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMaxError_float(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_max_error")
	}
	
	fn min_contour_points_allowed(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourPointsAllowed_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_points_allowed")
	}
	
	fn set_min_contour_points_allowed(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourPointsAllowed_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_points_allowed")
	}
	
	fn min_contour_length_allowed(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourLengthAllowed_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_length_allowed")
	}
	
	fn set_min_contour_length_allowed(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourLengthAllowed_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_length_allowed")
	}
	
	fn min_inter_contour_distance(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinInterContourDistance_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_inter_contour_distance")
	}
	
	fn set_min_inter_contour_distance(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinInterContourDistance_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_inter_contour_distance")
	}
	
	fn min_inter_checker_distance(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinInterCheckerDistance_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_inter_checker_distance")
	}
	
	fn set_min_inter_checker_distance(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinInterCheckerDistance_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_inter_checker_distance")
	}
	
	fn min_image_size(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinImageSize_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_image_size")
	}
	
	fn set_min_image_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinImageSize_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_image_size")
	}
	
	fn min_group_size(&self) -> u32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinGroupSize_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_group_size")
	}
	
	fn set_min_group_size(&mut self, val: u32) -> () {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinGroupSize_unsigned_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_group_size")
	}
	
}

/// Parameters for the detectMarker process:
/// - int adaptiveThreshWinSizeMin : minimum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 23).
/// - int adaptiveThreshWinSizeMax : maximum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 153).
/// - int adaptiveThreshWinSizeStep : increments from adaptiveThreshWinSizeMin to
///                                   adaptiveThreshWinSizeMax during the
///                                   thresholding (default 16).
/// - double adaptiveThreshConstant : constant for adaptive thresholding before
///                                   finding contours (default 7)
/// - double minContoursAreaRate : determine minimum area for marker contour to
///                                be detected. This is defined as a rate respect
///                                to the area of the input image. Used only if
///                                neural network is used (default 0.003).
/// - double minContoursArea : determine minimum area for marker contour to be
///                            detected. This is defined as the actual area. Used
///                            only if neural network is not used (default 100).
/// - double confidenceThreshold : minimum confidence for a bounding box detected
///                                by neural network to classify as
///                                detection.(default 0.5)
///                                (0<=confidenceThreshold<=1)
/// - double minContourSolidity : minimum solidity of a contour for it be
///                               detected as a square in the chart. (default
///                               0.9).
/// - double findCandidatesApproxPolyDPEpsMultiplier : multipler to be used in
///                                                    cv::ApproxPolyDP function
///                                                    (default 0.05)
/// - int borderWidth : width of the padding used to pass the inital neural
///                    network detection in the succeeding system.(default 0)
/// - float B0factor : distance between two neighbours squares of the same chart.
///                    Defined as the ratio between distance and large dimension
///                    of square (default 1.25)
/// - float maxError : maximum allowed error in the detection of a chart.
///                    default(0.1)
/// - int minContourPointsAllowed : minium points in a detected contour.
///                                 default(4)
/// - int minContourLengthAllowed : minimum length of a countour. default(100)
/// - int minInterContourDistance : minimum distance between two contours.
///                                 default(100)
/// - int minInterCheckerDistance : minimum distance between two checkers.
///                                 default(10000)
/// - int minImageSize : minimum size of the smaller dimension of the image.
///                      default(1000)
/// - unsigned minGroupSize : minimum number of a squared of a chart that must be
///                           detected. default(4)
pub struct MCC_DetectorParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { MCC_DetectorParameters }

impl Drop for MCC_DetectorParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_MCC_DetectorParameters_delete(instance: *mut c_void); }
		unsafe { cv_MCC_DetectorParameters_delete(self.as_raw_mut_MCC_DetectorParameters()) };
	}
}

impl MCC_DetectorParameters {
	#[inline] pub fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MCC_DetectorParameters {}

impl crate::mcc::MCC_DetectorParametersTrait for MCC_DetectorParameters {
	#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MCC_DetectorParameters {
	pub fn default() -> Result<crate::mcc::MCC_DetectorParameters> {
		unsafe { sys::cv_mcc_DetectorParameters_DetectorParameters() }.into_result().map(|r| unsafe { crate::mcc::MCC_DetectorParameters::opencv_from_extern(r) } )
	}
	
	pub fn create() -> Result<core::Ptr::<crate::mcc::MCC_DetectorParameters>> {
		unsafe { sys::cv_mcc_DetectorParameters_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::mcc::MCC_DetectorParameters>::opencv_from_extern(r) } )
	}
	
}
