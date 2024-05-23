//! # Video Analysis
//!   # Motion Analysis
//!   # Object Tracking
//!   # C API
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{BackgroundSubtractorKNNTrait, BackgroundSubtractorKNNTraitConst, BackgroundSubtractorMOG2Trait, BackgroundSubtractorMOG2TraitConst, BackgroundSubtractorTrait, BackgroundSubtractorTraitConst, DenseOpticalFlowTrait, DenseOpticalFlowTraitConst, DualTVL1OpticalFlowTrait, DualTVL1OpticalFlowTraitConst, FarnebackOpticalFlowTrait, FarnebackOpticalFlowTraitConst, KalmanFilterTrait, KalmanFilterTraitConst, SparseOpticalFlowTrait, SparseOpticalFlowTraitConst, SparsePyrLKOpticalFlowTrait, SparsePyrLKOpticalFlowTraitConst};
}

// CV_LKFLOW_GET_MIN_EIGENVALS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking_c.h:66
pub const CV_LKFLOW_GET_MIN_EIGENVALS: i32 = 8;
// CV_LKFLOW_INITIAL_GUESSES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking_c.h:65
pub const CV_LKFLOW_INITIAL_GUESSES: i32 = 4;
// CV_LKFLOW_PYR_A_READY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking_c.h:63
pub const CV_LKFLOW_PYR_A_READY: i32 = 1;
// CV_LKFLOW_PYR_B_READY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking_c.h:64
pub const CV_LKFLOW_PYR_B_READY: i32 = 2;
// MOTION_AFFINE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:264
pub const MOTION_AFFINE: i32 = 2;
// MOTION_EUCLIDEAN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:263
pub const MOTION_EUCLIDEAN: i32 = 1;
// MOTION_HOMOGRAPHY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:265
pub const MOTION_HOMOGRAPHY: i32 = 3;
// MOTION_TRANSLATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:262
pub const MOTION_TRANSLATION: i32 = 0;
// OPTFLOW_FARNEBACK_GAUSSIAN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:58
pub const OPTFLOW_FARNEBACK_GAUSSIAN: i32 = 256;
// OPTFLOW_LK_GET_MIN_EIGENVALS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:57
pub const OPTFLOW_LK_GET_MIN_EIGENVALS: i32 = 8;
// OPTFLOW_USE_INITIAL_FLOW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:56
pub const OPTFLOW_USE_INITIAL_FLOW: i32 = 4;
/// Finds an object center, size, and orientation.
///
/// ## Parameters
/// * probImage: Back projection of the object histogram. See calcBackProject.
/// * window: Initial search window.
/// * criteria: Stop criteria for the underlying meanShift.
/// returns
/// (in old interfaces) Number of iterations CAMSHIFT took to converge
/// The function implements the CAMSHIFT object tracking algorithm [Bradski98](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bradski98) . First, it finds an
/// object center using meanShift and then adjusts the window size and finds the optimal rotation. The
/// function returns the rotated rectangle structure that includes the object position, size, and
/// orientation. The next position of the search window can be obtained with RotatedRect::boundingRect()
///
/// See the OpenCV sample camshiftdemo.c that tracks colored objects.
///
///
/// Note:
/// *   (Python) A sample explaining the camshift tracking algorithm can be found at
///    opencv_source_code/samples/python/camshift.py
// CamShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:79
// ("cv::CamShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn cam_shift(prob_image: &impl ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<core::RotatedRect> {
	input_array_arg!(prob_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constructs the image pyramid which can be passed to calcOpticalFlowPyrLK.
///
/// ## Parameters
/// * img: 8-bit input image.
/// * pyramid: output pyramid.
/// * winSize: window size of optical flow algorithm. Must be not less than winSize argument of
/// calcOpticalFlowPyrLK. It is needed to calculate required padding for pyramid levels.
/// * maxLevel: 0-based maximal pyramid level number.
/// * withDerivatives: set to precompute gradients for the every pyramid level. If pyramid is
/// constructed without the gradients then calcOpticalFlowPyrLK will calculate them internally.
/// * pyrBorder: the border mode for pyramid layers.
/// * derivBorder: the border mode for gradients.
/// * tryReuseInputImage: put ROI of input image into the pyramid if possible. You can pass false
/// to force data copying.
/// ## Returns
/// number of levels in constructed pyramid. Can be less than maxLevel.
///
/// ## Note
/// This alternative version of [build_optical_flow_pyramid] function uses the following default values for its arguments:
/// * with_derivatives: true
/// * pyr_border: BORDER_REFLECT_101
/// * deriv_border: BORDER_CONSTANT
/// * try_reuse_input_image: true
// cv::buildOpticalFlowPyramid(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int"]), _)]),
#[inline]
pub fn build_optical_flow_pyramid_def(img: &impl ToInputArray, pyramid: &mut impl ToOutputArray, win_size: core::Size, max_level: i32) -> Result<i32> {
	input_array_arg!(img);
	output_array_arg!(pyramid);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int(img.as_raw__InputArray(), pyramid.as_raw__OutputArray(), &win_size, max_level, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constructs the image pyramid which can be passed to calcOpticalFlowPyrLK.
///
/// ## Parameters
/// * img: 8-bit input image.
/// * pyramid: output pyramid.
/// * winSize: window size of optical flow algorithm. Must be not less than winSize argument of
/// calcOpticalFlowPyrLK. It is needed to calculate required padding for pyramid levels.
/// * maxLevel: 0-based maximal pyramid level number.
/// * withDerivatives: set to precompute gradients for the every pyramid level. If pyramid is
/// constructed without the gradients then calcOpticalFlowPyrLK will calculate them internally.
/// * pyrBorder: the border mode for pyramid layers.
/// * derivBorder: the border mode for gradients.
/// * tryReuseInputImage: put ROI of input image into the pyramid if possible. You can pass false
/// to force data copying.
/// ## Returns
/// number of levels in constructed pyramid. Can be less than maxLevel.
///
/// ## C++ default parameters
/// * with_derivatives: true
/// * pyr_border: BORDER_REFLECT_101
/// * deriv_border: BORDER_CONSTANT
/// * try_reuse_input_image: true
// buildOpticalFlowPyramid(InputArray, OutputArrayOfArrays, Size, int, bool, int, int, bool)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel", "withDerivatives", "pyrBorder", "derivBorder", "tryReuseInputImage"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "bool", "int", "int", "bool"]), _)]),
#[inline]
pub fn build_optical_flow_pyramid(img: &impl ToInputArray, pyramid: &mut impl ToOutputArray, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> Result<i32> {
	input_array_arg!(img);
	output_array_arg!(pyramid);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img.as_raw__InputArray(), pyramid.as_raw__OutputArray(), &win_size, max_level, with_derivatives, pyr_border, deriv_border, try_reuse_input_image, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes a dense optical flow using the Gunnar Farneback's algorithm.
///
/// ## Parameters
/// * prev: first 8-bit single-channel input image.
/// * next: second input image of the same size and the same type as prev.
/// * flow: computed flow image that has the same size as prev and type CV_32FC2.
/// * pyr_scale: parameter, specifying the image scale (\<1) to build pyramids for each image;
/// pyr_scale=0.5 means a classical pyramid, where each next layer is twice smaller than the previous
/// one.
/// * levels: number of pyramid layers including the initial image; levels=1 means that no extra
/// layers are created and only the original images are used.
/// * winsize: averaging window size; larger values increase the algorithm robustness to image
/// noise and give more chances for fast motion detection, but yield more blurred motion field.
/// * iterations: number of iterations the algorithm does at each pyramid level.
/// * poly_n: size of the pixel neighborhood used to find polynomial expansion in each pixel;
/// larger values mean that the image will be approximated with smoother surfaces, yielding more
/// robust algorithm and more blurred motion field, typically poly_n =5 or 7.
/// * poly_sigma: standard deviation of the Gaussian that is used to smooth derivatives used as a
/// basis for the polynomial expansion; for poly_n=5, you can set poly_sigma=1.1, for poly_n=7, a
/// good value would be poly_sigma=1.5.
/// * flags: operation flags that can be a combination of the following:
///  *   **OPTFLOW_USE_INITIAL_FLOW** uses the input flow as an initial flow approximation.
///  *   **OPTFLOW_FARNEBACK_GAUSSIAN** uses the Gaussian ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bwinsize%7D%5Ctimes%5Ctexttt%7Bwinsize%7D)
///      filter instead of a box filter of the same size for optical flow estimation; usually, this
///      option gives z more accurate flow than with a box filter, at the cost of lower speed;
///      normally, winsize for a Gaussian window should be set to a larger value to achieve the same
///      level of robustness.
///
/// The function finds an optical flow for each prev pixel using the [Farneback2003](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Farneback2003) algorithm so that
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bprev%7D%20%28y%2Cx%29%20%20%5Csim%20%5Ctexttt%7Bnext%7D%20%28%20y%20%2B%20%5Ctexttt%7Bflow%7D%20%28y%2Cx%29%5B1%5D%2C%20%20x%20%2B%20%5Ctexttt%7Bflow%7D%20%28y%2Cx%29%5B0%5D%29)
///
///
/// Note:
///
/// *   An example using the optical flow algorithm described by Gunnar Farneback can be found at
///    opencv_source_code/samples/cpp/fback.cpp
/// *   (Python) An example using the optical flow algorithm described by Gunnar Farneback can be
///    found at opencv_source_code/samples/python/opt_flow.py
// calcOpticalFlowFarneback(InputArray, InputArray, InputOutputArray, double, int, int, int, int, double, int)(InputArray, InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:223
// ("cv::calcOpticalFlowFarneback", vec![(pred!(mut, ["prev", "next", "flow", "pyr_scale", "levels", "winsize", "iterations", "poly_n", "poly_sigma", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "int", "int", "int", "int", "double", "int"]), _)]),
#[inline]
pub fn calc_optical_flow_farneback(prev: &impl ToInputArray, next: &impl ToInputArray, flow: &mut impl ToInputOutputArray, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<()> {
	input_array_arg!(prev);
	input_array_arg!(next);
	input_output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev.as_raw__InputArray(), next.as_raw__InputArray(), flow.as_raw__InputOutputArray(), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates an optical flow for a sparse feature set using the iterative Lucas-Kanade method with
/// pyramids.
///
/// ## Parameters
/// * prevImg: first 8-bit input image or pyramid constructed by buildOpticalFlowPyramid.
/// * nextImg: second input image or pyramid of the same size and the same type as prevImg.
/// * prevPts: vector of 2D points for which the flow needs to be found; point coordinates must be
/// single-precision floating-point numbers.
/// * nextPts: output vector of 2D points (with single-precision floating-point coordinates)
/// containing the calculated new positions of input features in the second image; when
/// OPTFLOW_USE_INITIAL_FLOW flag is passed, the vector must have the same size as in the input.
/// * status: output status vector (of unsigned chars); each element of the vector is set to 1 if
/// the flow for the corresponding features has been found, otherwise, it is set to 0.
/// * err: output vector of errors; each element of the vector is set to an error for the
/// corresponding feature, type of the error measure can be set in flags parameter; if the flow wasn't
/// found then the error is not defined (use the status parameter to find such cases).
/// * winSize: size of the search window at each pyramid level.
/// * maxLevel: 0-based maximal pyramid level number; if set to 0, pyramids are not used (single
/// level), if set to 1, two levels are used, and so on; if pyramids are passed to input then
/// algorithm will use as many levels as pyramids have but no more than maxLevel.
/// * criteria: parameter, specifying the termination criteria of the iterative search algorithm
/// (after the specified maximum number of iterations criteria.maxCount or when the search window
/// moves by less than criteria.epsilon.
/// * flags: operation flags:
///  *   **OPTFLOW_USE_INITIAL_FLOW** uses initial estimations, stored in nextPts; if the flag is
///      not set, then prevPts is copied to nextPts and is considered the initial estimate.
///  *   **OPTFLOW_LK_GET_MIN_EIGENVALS** use minimum eigen values as an error measure (see
///      minEigThreshold description); if the flag is not set, then L1 distance between patches
///      around the original and a moved point, divided by number of pixels in a window, is used as a
///      error measure.
/// * minEigThreshold: the algorithm calculates the minimum eigen value of a 2x2 normal matrix of
/// optical flow equations (this matrix is called a spatial gradient matrix in [Bouguet00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bouguet00)), divided
/// by number of pixels in a window; if this value is less than minEigThreshold, then a corresponding
/// feature is filtered out and its flow is not processed, so it allows to remove bad points and get a
/// performance boost.
///
/// The function implements a sparse iterative version of the Lucas-Kanade optical flow in pyramids. See
/// [Bouguet00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bouguet00) . The function is parallelized with the TBB library.
///
///
/// Note:
///
/// *   An example using the Lucas-Kanade optical flow algorithm can be found at
///    opencv_source_code/samples/cpp/lkdemo.cpp
/// *   (Python) An example using the Lucas-Kanade optical flow algorithm can be found at
///    opencv_source_code/samples/python/lk_track.py
/// *   (Python) An example using the Lucas-Kanade tracker for homography matching can be found at
///    opencv_source_code/samples/python/lk_homography.py
///
/// ## Note
/// This alternative version of [calc_optical_flow_pyr_lk] function uses the following default values for its arguments:
/// * win_size: Size(21,21)
/// * max_level: 3
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
/// * flags: 0
/// * min_eig_threshold: 1e-4
// cv::calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calc_optical_flow_pyr_lk_def(prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, err: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates an optical flow for a sparse feature set using the iterative Lucas-Kanade method with
/// pyramids.
///
/// ## Parameters
/// * prevImg: first 8-bit input image or pyramid constructed by buildOpticalFlowPyramid.
/// * nextImg: second input image or pyramid of the same size and the same type as prevImg.
/// * prevPts: vector of 2D points for which the flow needs to be found; point coordinates must be
/// single-precision floating-point numbers.
/// * nextPts: output vector of 2D points (with single-precision floating-point coordinates)
/// containing the calculated new positions of input features in the second image; when
/// OPTFLOW_USE_INITIAL_FLOW flag is passed, the vector must have the same size as in the input.
/// * status: output status vector (of unsigned chars); each element of the vector is set to 1 if
/// the flow for the corresponding features has been found, otherwise, it is set to 0.
/// * err: output vector of errors; each element of the vector is set to an error for the
/// corresponding feature, type of the error measure can be set in flags parameter; if the flow wasn't
/// found then the error is not defined (use the status parameter to find such cases).
/// * winSize: size of the search window at each pyramid level.
/// * maxLevel: 0-based maximal pyramid level number; if set to 0, pyramids are not used (single
/// level), if set to 1, two levels are used, and so on; if pyramids are passed to input then
/// algorithm will use as many levels as pyramids have but no more than maxLevel.
/// * criteria: parameter, specifying the termination criteria of the iterative search algorithm
/// (after the specified maximum number of iterations criteria.maxCount or when the search window
/// moves by less than criteria.epsilon.
/// * flags: operation flags:
///  *   **OPTFLOW_USE_INITIAL_FLOW** uses initial estimations, stored in nextPts; if the flag is
///      not set, then prevPts is copied to nextPts and is considered the initial estimate.
///  *   **OPTFLOW_LK_GET_MIN_EIGENVALS** use minimum eigen values as an error measure (see
///      minEigThreshold description); if the flag is not set, then L1 distance between patches
///      around the original and a moved point, divided by number of pixels in a window, is used as a
///      error measure.
/// * minEigThreshold: the algorithm calculates the minimum eigen value of a 2x2 normal matrix of
/// optical flow equations (this matrix is called a spatial gradient matrix in [Bouguet00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bouguet00)), divided
/// by number of pixels in a window; if this value is less than minEigThreshold, then a corresponding
/// feature is filtered out and its flow is not processed, so it allows to remove bad points and get a
/// performance boost.
///
/// The function implements a sparse iterative version of the Lucas-Kanade optical flow in pyramids. See
/// [Bouguet00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bouguet00) . The function is parallelized with the TBB library.
///
///
/// Note:
///
/// *   An example using the Lucas-Kanade optical flow algorithm can be found at
///    opencv_source_code/samples/cpp/lkdemo.cpp
/// *   (Python) An example using the Lucas-Kanade optical flow algorithm can be found at
///    opencv_source_code/samples/python/lk_track.py
/// *   (Python) An example using the Lucas-Kanade tracker for homography matching can be found at
///    opencv_source_code/samples/python/lk_homography.py
///
/// ## C++ default parameters
/// * win_size: Size(21,21)
/// * max_level: 3
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
/// * flags: 0
/// * min_eig_threshold: 1e-4
// calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Size, int, TermCriteria, int, double)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "winSize", "maxLevel", "criteria", "flags", "minEigThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
#[inline]
pub fn calc_optical_flow_pyr_lk(prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, err: &mut impl ToOutputArray, win_size: core::Size, max_level: i32, criteria: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), &win_size, max_level, &criteria, flags, min_eig_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the Enhanced Correlation Coefficient value between two images [EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08) .
///
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image to be warped to provide an image similar to
///  templateImage, same type as templateImage.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// ## See also
/// findTransformECC
///
/// ## Note
/// This alternative version of [compute_ecc] function uses the following default values for its arguments:
/// * input_mask: noArray()
// cv::computeECC(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn compute_ecc_def(template_image: &impl ToInputArray, input_image: &impl ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeECC_const__InputArrayR_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the Enhanced Correlation Coefficient value between two images [EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08) .
///
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image to be warped to provide an image similar to
///  templateImage, same type as templateImage.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// ## See also
/// findTransformECC
///
/// ## C++ default parameters
/// * input_mask: noArray()
// computeECC(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn compute_ecc(template_image: &impl ToInputArray, input_image: &impl ToInputArray, input_mask: &impl ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), input_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates KNN Background Subtractor
///
/// ## Parameters
/// * history: Length of the history.
/// * dist2Threshold: Threshold on the squared distance between the pixel and the sample to decide
/// whether a pixel is close to that sample. This parameter does not affect the background update.
/// * detectShadows: If true, the algorithm will detect shadows and mark them. It decreases the
/// speed a bit, so if you do not need this feature, set the parameter to false.
///
/// ## Note
/// This alternative version of [create_background_subtractor_knn] function uses the following default values for its arguments:
/// * history: 500
/// * dist2_threshold: 400.0
/// * detect_shadows: true
// cv::createBackgroundSubtractorKNN() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_background_subtractor_knn_def() -> Result<core::Ptr<crate::video::BackgroundSubtractorKNN>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorKNN(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::BackgroundSubtractorKNN>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates KNN Background Subtractor
///
/// ## Parameters
/// * history: Length of the history.
/// * dist2Threshold: Threshold on the squared distance between the pixel and the sample to decide
/// whether a pixel is close to that sample. This parameter does not affect the background update.
/// * detectShadows: If true, the algorithm will detect shadows and mark them. It decreases the
/// speed a bit, so if you do not need this feature, set the parameter to false.
///
/// ## C++ default parameters
/// * history: 500
/// * dist2_threshold: 400.0
/// * detect_shadows: true
// createBackgroundSubtractorKNN(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, ["history", "dist2Threshold", "detectShadows"], ["int", "double", "bool"]), _)]),
#[inline]
pub fn create_background_subtractor_knn(history: i32, dist2_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<crate::video::BackgroundSubtractorKNN>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorKNN_int_double_bool(history, dist2_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::BackgroundSubtractorKNN>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates MOG2 Background Subtractor
///
/// ## Parameters
/// * history: Length of the history.
/// * varThreshold: Threshold on the squared Mahalanobis distance between the pixel and the model
/// to decide whether a pixel is well described by the background model. This parameter does not
/// affect the background update.
/// * detectShadows: If true, the algorithm will detect shadows and mark them. It decreases the
/// speed a bit, so if you do not need this feature, set the parameter to false.
///
/// ## Note
/// This alternative version of [create_background_subtractor_mog2] function uses the following default values for its arguments:
/// * history: 500
/// * var_threshold: 16
/// * detect_shadows: true
// cv::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_background_subtractor_mog2_def() -> Result<core::Ptr<crate::video::BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorMOG2(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates MOG2 Background Subtractor
///
/// ## Parameters
/// * history: Length of the history.
/// * varThreshold: Threshold on the squared Mahalanobis distance between the pixel and the model
/// to decide whether a pixel is well described by the background model. This parameter does not
/// affect the background update.
/// * detectShadows: If true, the algorithm will detect shadows and mark them. It decreases the
/// speed a bit, so if you do not need this feature, set the parameter to false.
///
/// ## C++ default parameters
/// * history: 500
/// * var_threshold: 16
/// * detect_shadows: true
// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
#[inline]
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<crate::video::BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates instance of cv::DenseOpticalFlow
// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:573
// ("cv::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr<crate::video::DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createOptFlow_DualTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal affine transformation between two 2D point sets.
///
/// ## Parameters
/// * src: First input 2D point set stored in std::vector or Mat, or an image stored in Mat.
/// * dst: Second input 2D point set of the same size and the same type as A, or another image.
/// * fullAffine: If true, the function finds an optimal affine transformation with no additional
/// restrictions (6 degrees of freedom). Otherwise, the class of transformations to choose from is
/// limited to combinations of translation, rotation, and uniform scaling (4 degrees of freedom).
///
/// The function finds an optimal affine transform *[A|b]* (a 2 x 3 floating-point matrix) that
/// approximates best the affine transformation between:
///
/// *   Two point sets
/// *   Two raster images. In this case, the function first finds some features in the src image and
///    finds the corresponding features in dst image. After that, the problem is reduced to the first
///    case.
/// In case of point sets, the problem is formulated as follows: you need to find a 2x2 matrix *A* and
/// 2x1 vector *b* so that:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5BA%5E%2A%7Cb%5E%2A%5D%20%3D%20arg%20%20%5Cmin%20%5F%7B%5BA%7Cb%5D%7D%20%20%5Csum%20%5Fi%20%20%5C%7C%20%5Ctexttt%7Bdst%7D%5Bi%5D%20%2D%20A%20%7B%20%5Ctexttt%7Bsrc%7D%5Bi%5D%7D%5ET%20%2D%20b%20%20%5C%7C%20%5E2)
/// where src[i] and dst[i] are the i-th points in src and dst, respectively
/// ![inline formula](https://latex.codecogs.com/png.latex?%5BA%7Cb%5D) can be either arbitrary (when fullAffine=true ) or have a form of
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20a%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%20%20%5C%5C%20%2Da%5F%7B12%7D%20%26%20a%5F%7B11%7D%20%26%20b%5F2%20%20%5Cend%7Bbmatrix%7D)
/// when fullAffine=false.
/// ## See also
/// estimateAffine2D, estimateAffinePartial2D, getAffineTransform, getPerspectiveTransform, findHomography
// estimateRigidTransform(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:255
// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
#[inline]
pub fn estimate_rigid_transform(src: &impl ToInputArray, dst: &impl ToInputArray, full_affine: bool) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), full_affine, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// estimateRigidTransform(InputArray, InputArray, bool, int, double, int)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:256
// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine", "ransacMaxIters", "ransacGoodRatio", "ransacSize0"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool", "int", "double", "int"]), _)]),
#[inline]
pub fn estimate_rigid_transform_1(src: &impl ToInputArray, dst: &impl ToInputArray, full_affine: bool, ransac_max_iters: i32, ransac_good_ratio: f64, ransac_size0: i32) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool_int_double_int(src.as_raw__InputArray(), dst.as_raw__InputArray(), full_affine, ransac_max_iters, ransac_good_ratio, ransac_size0, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_transform_ecc_1] function uses the following default values for its arguments:
/// * motion_type: MOTION_AFFINE
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,50,0.001)
/// * input_mask: noArray()
// cv::findTransformECC(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn find_transform_ecc_1_def(template_image: &impl ToInputArray, input_image: &impl ToInputArray, warp_matrix: &mut impl ToInputOutputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08) .
///
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image which should be warped with the final warpMatrix in
/// order to provide an image similar to templateImage, same type as templateImage.
/// * warpMatrix: floating-point ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) or ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) mapping matrix (warp).
/// * motionType: parameter, specifying the type of motion:
///  *   **MOTION_TRANSLATION** sets a translational motion model; warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) with
///      the first ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%202) part being the unity matrix and the rest two parameters being
///      estimated.
///  *   **MOTION_EUCLIDEAN** sets a Euclidean (rigid) transformation as motion model; three
///      parameters are estimated; warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203).
///  *   **MOTION_AFFINE** sets an affine motion model (DEFAULT); six parameters are estimated;
///      warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203).
///  *   **MOTION_HOMOGRAPHY** sets a homography as a motion model; eight parameters are
///      estimated;\`warpMatrix\` is ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203).
/// * criteria: parameter, specifying the termination criteria of the ECC algorithm;
/// criteria.epsilon defines the threshold of the increment in the correlation coefficient between two
/// iterations (a negative criteria.epsilon makes criteria.maxcount the only termination criterion).
/// Default values are shown in the declaration above.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// * gaussFiltSize: An optional value indicating size of gaussian blur filter; (DEFAULT: 5)
///
/// The function estimates the optimum transformation (warpMatrix) with respect to ECC criterion
/// ([EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08)), that is
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BwarpMatrix%7D%20%3D%20%5Carg%5Cmax%5F%7BW%7D%20%5Ctexttt%7BECC%7D%28%5Ctexttt%7BtemplateImage%7D%28x%2Cy%29%2C%5Ctexttt%7BinputImage%7D%28x%27%2Cy%27%29%29)
///
/// where
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20x%27%20%5C%5C%20y%27%20%5Cend%7Bbmatrix%7D%20%3D%20W%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%20%5C%5C%20y%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
///
/// (the equation holds with homogeneous coordinates for homography). It returns the final enhanced
/// correlation coefficient, that is the correlation coefficient between the template image and the
/// final warped input image. When a ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) matrix is given with motionType =0, 1 or 2, the third
/// row is ignored.
///
/// Unlike findHomography and estimateRigidTransform, the function findTransformECC implements an
/// area-based alignment that builds on intensity similarities. In essence, the function updates the
/// initial transformation that roughly aligns the images. If this information is missing, the identity
/// warp (unity matrix) is used as an initialization. Note that if images undergo strong
/// displacements/rotations, an initial transformation that roughly aligns the images is necessary
/// (e.g., a simple euclidean/similarity transform that allows for the images showing the same image
/// content approximately). Use inverse warping in the second image to take an image close to the first
/// one, i.e. use the flag WARP_INVERSE_MAP with warpAffine or warpPerspective. See also the OpenCV
/// sample image_alignment.cpp that demonstrates the use of the function. Note that the function throws
/// an exception if algorithm does not converges.
/// ## See also
/// computeECC, estimateAffine2D, estimateAffinePartial2D, findHomography
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * motion_type: MOTION_AFFINE
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,50,0.001)
/// * input_mask: noArray()
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_transform_ecc_1(template_image: &impl ToInputArray, input_image: &impl ToInputArray, warp_matrix: &mut impl ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &impl ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, &criteria, input_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08) .
///
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image which should be warped with the final warpMatrix in
/// order to provide an image similar to templateImage, same type as templateImage.
/// * warpMatrix: floating-point ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) or ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) mapping matrix (warp).
/// * motionType: parameter, specifying the type of motion:
///  *   **MOTION_TRANSLATION** sets a translational motion model; warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) with
///      the first ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%202) part being the unity matrix and the rest two parameters being
///      estimated.
///  *   **MOTION_EUCLIDEAN** sets a Euclidean (rigid) transformation as motion model; three
///      parameters are estimated; warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203).
///  *   **MOTION_AFFINE** sets an affine motion model (DEFAULT); six parameters are estimated;
///      warpMatrix is ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203).
///  *   **MOTION_HOMOGRAPHY** sets a homography as a motion model; eight parameters are
///      estimated;\`warpMatrix\` is ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203).
/// * criteria: parameter, specifying the termination criteria of the ECC algorithm;
/// criteria.epsilon defines the threshold of the increment in the correlation coefficient between two
/// iterations (a negative criteria.epsilon makes criteria.maxcount the only termination criterion).
/// Default values are shown in the declaration above.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// * gaussFiltSize: An optional value indicating size of gaussian blur filter; (DEFAULT: 5)
///
/// The function estimates the optimum transformation (warpMatrix) with respect to ECC criterion
/// ([EP08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_EP08)), that is
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BwarpMatrix%7D%20%3D%20%5Carg%5Cmax%5F%7BW%7D%20%5Ctexttt%7BECC%7D%28%5Ctexttt%7BtemplateImage%7D%28x%2Cy%29%2C%5Ctexttt%7BinputImage%7D%28x%27%2Cy%27%29%29)
///
/// where
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20x%27%20%5C%5C%20y%27%20%5Cend%7Bbmatrix%7D%20%3D%20W%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%20%5C%5C%20y%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
///
/// (the equation holds with homogeneous coordinates for homography). It returns the final enhanced
/// correlation coefficient, that is the correlation coefficient between the template image and the
/// final warped input image. When a ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) matrix is given with motionType =0, 1 or 2, the third
/// row is ignored.
///
/// Unlike findHomography and estimateRigidTransform, the function findTransformECC implements an
/// area-based alignment that builds on intensity similarities. In essence, the function updates the
/// initial transformation that roughly aligns the images. If this information is missing, the identity
/// warp (unity matrix) is used as an initialization. Note that if images undergo strong
/// displacements/rotations, an initial transformation that roughly aligns the images is necessary
/// (e.g., a simple euclidean/similarity transform that allows for the images showing the same image
/// content approximately). Use inverse warping in the second image to take an image close to the first
/// one, i.e. use the flag WARP_INVERSE_MAP with warpAffine or warpPerspective. See also the OpenCV
/// sample image_alignment.cpp that demonstrates the use of the function. Note that the function throws
/// an exception if algorithm does not converges.
/// ## See also
/// computeECC, estimateAffine2D, estimateAffinePartial2D, findHomography
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray, int)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:336
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask", "gaussFiltSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn find_transform_ecc(template_image: &impl ToInputArray, input_image: &impl ToInputArray, warp_matrix: &mut impl ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &impl ToInputArray, gauss_filt_size: i32) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, &criteria, input_mask.as_raw__InputArray(), gauss_filt_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object on a back projection image.
///
/// ## Parameters
/// * probImage: Back projection of the object histogram. See calcBackProject for details.
/// * window: Initial search window.
/// * criteria: Stop criteria for the iterative search algorithm.
/// returns
/// :   Number of iterations CAMSHIFT took to converge.
/// The function implements the iterative object search algorithm. It takes the input back projection of
/// an object and the initial position. The mass center in window of the back projection image is
/// computed and the search window center shifts to the mass center. The procedure is repeated until the
/// specified number of iterations criteria.maxCount is done or until the window center shifts by less
/// than criteria.epsilon. The algorithm is used inside CamShift and, unlike CamShift , the search
/// window size or orientation do not change during the search. You can simply pass the output of
/// calcBackProject to this function. But better results can be obtained if you pre-filter the back
/// projection and remove the noise. For example, you can do this by retrieving connected components
/// with findContours , throwing away contours with small area ( contourArea ), and rendering the
/// remaining contours with drawContours.
// meanShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:104
// ("cv::meanShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn mean_shift(prob_image: &impl ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<i32> {
	input_array_arg!(prob_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::video::BackgroundSubtractor]
// BackgroundSubtractor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:60
pub trait BackgroundSubtractorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BackgroundSubtractor(&self) -> *const c_void;

	/// Computes a background image.
	///
	/// ## Parameters
	/// * backgroundImage: The output background image.
	///
	///
	/// Note: Sometimes the background image can be very blurry, as it contain the average background
	/// statistics.
	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:81
	// ("cv::BackgroundSubtractor::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_background_image(&self, background_image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(self.as_raw_BackgroundSubtractor(), background_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::BackgroundSubtractor]
pub trait BackgroundSubtractorTrait: core::AlgorithmTrait + crate::video::BackgroundSubtractorTraitConst {
	fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void;

	/// Computes a foreground mask.
	///
	/// ## Parameters
	/// * image: Next video frame.
	/// * fgmask: The output foreground mask as an 8-bit binary image.
	/// * learningRate: The value between 0 and 1 that indicates how fast the background model is
	/// learnt. Negative parameter value makes the algorithm to use some automatically chosen learning
	/// rate. 0 means that the background model is not updated at all, 1 means that the background model
	/// is completely reinitialized from the last frame.
	///
	/// ## C++ default parameters
	/// * learning_rate: -1
	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	#[inline]
	fn apply(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractor(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes a foreground mask.
	///
	/// ## Parameters
	/// * image: Next video frame.
	/// * fgmask: The output foreground mask as an 8-bit binary image.
	/// * learningRate: The value between 0 and 1 that indicates how fast the background model is
	/// learnt. Negative parameter value makes the algorithm to use some automatically chosen learning
	/// rate. 0 means that the background model is not updated at all, 1 means that the background model
	/// is completely reinitialized from the last frame.
	///
	/// ## Note
	/// This alternative version of [BackgroundSubtractorTrait::apply] function uses the following default values for its arguments:
	/// * learning_rate: -1
	// cv::BackgroundSubtractor::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply_def(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_BackgroundSubtractor(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for background/foreground segmentation. :
///
/// The class is only used to define the common interface for the whole family of background/foreground
/// segmentation algorithms.
// BackgroundSubtractor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:60
pub struct BackgroundSubtractor {
	ptr: *mut c_void,
}

opencv_type_boxed! { BackgroundSubtractor }

impl Drop for BackgroundSubtractor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BackgroundSubtractor_delete(self.as_raw_mut_BackgroundSubtractor()) };
	}
}

unsafe impl Send for BackgroundSubtractor {}

impl core::AlgorithmTraitConst for BackgroundSubtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BackgroundSubtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractor, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::BackgroundSubtractorTraitConst for BackgroundSubtractor {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for BackgroundSubtractor {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractor, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

impl BackgroundSubtractor {
}

boxed_cast_descendant! { BackgroundSubtractor, crate::video::BackgroundSubtractorKNN, cv_BackgroundSubtractor_to_BackgroundSubtractorKNN }

boxed_cast_descendant! { BackgroundSubtractor, crate::video::BackgroundSubtractorMOG2, cv_BackgroundSubtractor_to_BackgroundSubtractorMOG2 }

boxed_cast_base! { BackgroundSubtractor, core::Algorithm, cv_BackgroundSubtractor_to_Algorithm }

impl std::fmt::Debug for BackgroundSubtractor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BackgroundSubtractor")
			.finish()
	}
}

/// Constant methods for [crate::video::BackgroundSubtractorKNN]
// BackgroundSubtractorKNN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:229
pub trait BackgroundSubtractorKNNTraitConst: crate::video::BackgroundSubtractorTraitConst {
	fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void;

	/// Returns the number of last frames that affect the background model
	// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:234
	// ("cv::BackgroundSubtractorKNN::getHistory", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getHistory_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of data samples in the background model
	// getNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:241
	// ("cv::BackgroundSubtractorKNN::getNSamples", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_samples(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getNSamples_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the threshold on the squared distance between the pixel and the sample
	///
	/// The threshold on the squared distance between the pixel and the sample to decide whether a pixel is
	/// close to a data sample.
	// getDist2Threshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:253
	// ("cv::BackgroundSubtractorKNN::getDist2Threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_dist2_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getDist2Threshold_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of neighbours, the k in the kNN.
	///
	/// K is the number of samples that need to be within dist2Threshold in order to decide that that
	/// pixel is matching the kNN background model.
	// getkNNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:263
	// ("cv::BackgroundSubtractorKNN::getkNNSamples", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn getk_nn_samples(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getkNNSamples_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow detection flag
	///
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorKNN for
	/// details.
	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:273
	// ("cv::BackgroundSubtractorKNN::getDetectShadows", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getDetectShadows_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow value
	///
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:283
	// ("cv::BackgroundSubtractorKNN::getShadowValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowValue_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow threshold
	///
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:295
	// ("cv::BackgroundSubtractorKNN::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowThreshold_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::BackgroundSubtractorKNN]
pub trait BackgroundSubtractorKNNTrait: crate::video::BackgroundSubtractorKNNTraitConst + crate::video::BackgroundSubtractorTrait {
	fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void;

	/// Sets the number of last frames that affect the background model
	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:237
	// ("cv::BackgroundSubtractorKNN::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setHistory_int(self.as_raw_mut_BackgroundSubtractorKNN(), history, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the number of data samples in the background model.
	///
	/// The model needs to be reinitalized to reserve memory.
	// setNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:246
	// ("cv::BackgroundSubtractorKNN::setNSamples", vec![(pred!(mut, ["_nN"], ["int"]), _)]),
	#[inline]
	fn set_n_samples(&mut self, _n_n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _n_n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the threshold on the squared distance
	// setDist2Threshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:256
	// ("cv::BackgroundSubtractorKNN::setDist2Threshold", vec![(pred!(mut, ["_dist2Threshold"], ["double"]), _)]),
	#[inline]
	fn set_dist2_threshold(&mut self, _dist2_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setDist2Threshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), _dist2_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the k in the kNN. How many nearest neighbours need to match.
	// setkNNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:266
	// ("cv::BackgroundSubtractorKNN::setkNNSamples", vec![(pred!(mut, ["_nkNN"], ["int"]), _)]),
	#[inline]
	fn setk_nn_samples(&mut self, _nk_nn: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setkNNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _nk_nn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Enables or disables shadow detection
	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:276
	// ("cv::BackgroundSubtractorKNN::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorKNN(), detect_shadows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the shadow value
	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:286
	// ("cv::BackgroundSubtractorKNN::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorKNN(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the shadow threshold
	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:298
	// ("cv::BackgroundSubtractorKNN::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// K-nearest neighbours - based Background/Foreground Segmentation Algorithm.
///
/// The class implements the K-nearest neighbours background subtraction described in [Zivkovic2006](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
/// Very efficient if number of foreground pixels is low.
// BackgroundSubtractorKNN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:229
pub struct BackgroundSubtractorKNN {
	ptr: *mut c_void,
}

opencv_type_boxed! { BackgroundSubtractorKNN }

impl Drop for BackgroundSubtractorKNN {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BackgroundSubtractorKNN_delete(self.as_raw_mut_BackgroundSubtractorKNN()) };
	}
}

unsafe impl Send for BackgroundSubtractorKNN {}

impl core::AlgorithmTraitConst for BackgroundSubtractorKNN {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorKNN, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::BackgroundSubtractorTraitConst for BackgroundSubtractorKNN {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for BackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorKNN, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

impl crate::video::BackgroundSubtractorKNNTraitConst for BackgroundSubtractorKNN {
	#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorKNNTrait for BackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorKNN, crate::video::BackgroundSubtractorKNNTraitConst, as_raw_BackgroundSubtractorKNN, crate::video::BackgroundSubtractorKNNTrait, as_raw_mut_BackgroundSubtractorKNN }

impl BackgroundSubtractorKNN {
}

boxed_cast_base! { BackgroundSubtractorKNN, core::Algorithm, cv_BackgroundSubtractorKNN_to_Algorithm }

boxed_cast_base! { BackgroundSubtractorKNN, crate::video::BackgroundSubtractor, cv_BackgroundSubtractorKNN_to_BackgroundSubtractor }

impl std::fmt::Debug for BackgroundSubtractorKNN {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BackgroundSubtractorKNN")
			.finish()
	}
}

/// Constant methods for [crate::video::BackgroundSubtractorMOG2]
// BackgroundSubtractorMOG2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:90
pub trait BackgroundSubtractorMOG2TraitConst: crate::video::BackgroundSubtractorTraitConst {
	fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void;

	/// Returns the number of last frames that affect the background model
	// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:95
	// ("cv::BackgroundSubtractorMOG2::getHistory", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getHistory_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of gaussian components in the background model
	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:102
	// ("cv::BackgroundSubtractorMOG2::getNMixtures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_mixtures(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getNMixtures_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the "background ratio" parameter of the algorithm
	///
	/// If a foreground pixel keeps semi-constant value for about backgroundRatio\*history frames, it's
	/// considered background and added to the model as a center of a new component. It corresponds to TB
	/// parameter in the paper.
	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:115
	// ("cv::BackgroundSubtractorMOG2::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_background_ratio(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the variance threshold for the pixel-model match
	///
	/// The main threshold on the squared Mahalanobis distance to decide if the sample is well described by
	/// the background model or not. Related to Cthr from the paper.
	// getVarThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:125
	// ("cv::BackgroundSubtractorMOG2::getVarThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_var_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the variance threshold for the pixel-model match used for new mixture component generation
	///
	/// Threshold for the squared Mahalanobis distance that helps decide when a sample is close to the
	/// existing components (corresponds to Tg in the paper). If a pixel is not close to any component, it
	/// is considered foreground or added as a new component. 3 sigma =\> Tg=3\*3=9 is default. A smaller Tg
	/// value generates more components. A higher Tg value may result in a small number of components but
	/// they can grow too large.
	// getVarThresholdGen()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:138
	// ("cv::BackgroundSubtractorMOG2::getVarThresholdGen", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_var_threshold_gen(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the initial variance of each gaussian component
	// getVarInit()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:145
	// ("cv::BackgroundSubtractorMOG2::getVarInit", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_var_init(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarInit_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getVarMin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:150
	// ("cv::BackgroundSubtractorMOG2::getVarMin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_var_min(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMin_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getVarMax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:153
	// ("cv::BackgroundSubtractorMOG2::getVarMax", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_var_max(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMax_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the complexity reduction threshold
	///
	/// This parameter defines the number of samples needed to accept to prove the component exists. CT=0.05
	/// is a default value for all the samples. By setting CT=0 you get an algorithm very similar to the
	/// standard Stauffer&Grimson algorithm.
	// getComplexityReductionThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:162
	// ("cv::BackgroundSubtractorMOG2::getComplexityReductionThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_complexity_reduction_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow detection flag
	///
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorMOG2 for
	/// details.
	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:172
	// ("cv::BackgroundSubtractorMOG2::getDetectShadows", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getDetectShadows_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow value
	///
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:182
	// ("cv::BackgroundSubtractorMOG2::getShadowValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowValue_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the shadow threshold
	///
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:194
	// ("cv::BackgroundSubtractorMOG2::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::BackgroundSubtractorMOG2]
pub trait BackgroundSubtractorMOG2Trait: crate::video::BackgroundSubtractorMOG2TraitConst + crate::video::BackgroundSubtractorTrait {
	fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	/// Sets the number of last frames that affect the background model
	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:98
	// ("cv::BackgroundSubtractorMOG2::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setHistory_int(self.as_raw_mut_BackgroundSubtractorMOG2(), history, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the number of gaussian components in the background model.
	///
	/// The model needs to be reinitalized to reserve memory.
	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:107
	// ("cv::BackgroundSubtractorMOG2::setNMixtures", vec![(pred!(mut, ["nmixtures"], ["int"]), _)]),
	#[inline]
	fn set_n_mixtures(&mut self, nmixtures: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setNMixtures_int(self.as_raw_mut_BackgroundSubtractorMOG2(), nmixtures, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the "background ratio" parameter of the algorithm
	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:118
	// ("cv::BackgroundSubtractorMOG2::setBackgroundRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
	#[inline]
	fn set_background_ratio(&mut self, ratio: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the variance threshold for the pixel-model match
	// setVarThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:128
	// ("cv::BackgroundSubtractorMOG2::setVarThreshold", vec![(pred!(mut, ["varThreshold"], ["double"]), _)]),
	#[inline]
	fn set_var_threshold(&mut self, var_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the variance threshold for the pixel-model match used for new mixture component generation
	// setVarThresholdGen(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:141
	// ("cv::BackgroundSubtractorMOG2::setVarThresholdGen", vec![(pred!(mut, ["varThresholdGen"], ["double"]), _)]),
	#[inline]
	fn set_var_threshold_gen(&mut self, var_threshold_gen: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold_gen, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the initial variance of each gaussian component
	// setVarInit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:148
	// ("cv::BackgroundSubtractorMOG2::setVarInit", vec![(pred!(mut, ["varInit"], ["double"]), _)]),
	#[inline]
	fn set_var_init(&mut self, var_init: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarInit_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_init, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setVarMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:151
	// ("cv::BackgroundSubtractorMOG2::setVarMin", vec![(pred!(mut, ["varMin"], ["double"]), _)]),
	#[inline]
	fn set_var_min(&mut self, var_min: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMin_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_min, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setVarMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:154
	// ("cv::BackgroundSubtractorMOG2::setVarMax", vec![(pred!(mut, ["varMax"], ["double"]), _)]),
	#[inline]
	fn set_var_max(&mut self, var_max: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMax_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the complexity reduction threshold
	// setComplexityReductionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:165
	// ("cv::BackgroundSubtractorMOG2::setComplexityReductionThreshold", vec![(pred!(mut, ["ct"], ["double"]), _)]),
	#[inline]
	fn set_complexity_reduction_threshold(&mut self, ct: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ct, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Enables or disables shadow detection
	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:175
	// ("cv::BackgroundSubtractorMOG2::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorMOG2(), detect_shadows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the shadow value
	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:185
	// ("cv::BackgroundSubtractorMOG2::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorMOG2(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the shadow threshold
	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:197
	// ("cv::BackgroundSubtractorMOG2::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes a foreground mask.
	///
	/// ## Parameters
	/// * image: Next video frame. Floating point frame will be used without scaling and should be in range ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C255%5D).
	/// * fgmask: The output foreground mask as an 8-bit binary image.
	/// * learningRate: The value between 0 and 1 that indicates how fast the background model is
	/// learnt. Negative parameter value makes the algorithm to use some automatically chosen learning
	/// rate. 0 means that the background model is not updated at all, 1 means that the background model
	/// is completely reinitialized from the last frame.
	///
	/// ## C++ default parameters
	/// * learning_rate: -1
	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
	// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	#[inline]
	fn apply(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes a foreground mask.
	///
	/// ## Parameters
	/// * image: Next video frame. Floating point frame will be used without scaling and should be in range ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C255%5D).
	/// * fgmask: The output foreground mask as an 8-bit binary image.
	/// * learningRate: The value between 0 and 1 that indicates how fast the background model is
	/// learnt. Negative parameter value makes the algorithm to use some automatically chosen learning
	/// rate. 0 means that the background model is not updated at all, 1 means that the background model
	/// is completely reinitialized from the last frame.
	///
	/// ## Note
	/// This alternative version of [BackgroundSubtractorMOG2Trait::apply] function uses the following default values for its arguments:
	/// * learning_rate: -1
	// cv::BackgroundSubtractorMOG2::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
	// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply_def(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
///
/// The class implements the Gaussian mixture model background subtraction described in [Zivkovic2004](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zivkovic2004)
/// and [Zivkovic2006](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
// BackgroundSubtractorMOG2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:90
pub struct BackgroundSubtractorMOG2 {
	ptr: *mut c_void,
}

opencv_type_boxed! { BackgroundSubtractorMOG2 }

impl Drop for BackgroundSubtractorMOG2 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BackgroundSubtractorMOG2_delete(self.as_raw_mut_BackgroundSubtractorMOG2()) };
	}
}

unsafe impl Send for BackgroundSubtractorMOG2 {}

impl core::AlgorithmTraitConst for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorMOG2, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::BackgroundSubtractorTraitConst for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

impl crate::video::BackgroundSubtractorMOG2TraitConst for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorMOG2Trait for BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorMOG2TraitConst, as_raw_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorMOG2Trait, as_raw_mut_BackgroundSubtractorMOG2 }

impl BackgroundSubtractorMOG2 {
}

boxed_cast_base! { BackgroundSubtractorMOG2, core::Algorithm, cv_BackgroundSubtractorMOG2_to_Algorithm }

boxed_cast_base! { BackgroundSubtractorMOG2, crate::video::BackgroundSubtractor, cv_BackgroundSubtractorMOG2_to_BackgroundSubtractor }

impl std::fmt::Debug for BackgroundSubtractorMOG2 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BackgroundSubtractorMOG2")
			.finish()
	}
}

/// Constant methods for [crate::video::DenseOpticalFlow]
// DenseOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:413
pub trait DenseOpticalFlowTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DenseOpticalFlow(&self) -> *const c_void;

}

/// Mutable methods for [crate::video::DenseOpticalFlow]
pub trait DenseOpticalFlowTrait: core::AlgorithmTrait + crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void;

	/// Calculates an optical flow.
	///
	/// ## Parameters
	/// * I0: first 8-bit single-channel input image.
	/// * I1: second input image of the same size and the same type as prev.
	/// * flow: computed flow image that has the same size as prev and type CV_32FC2.
	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:422
	// ("cv::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn calc(&mut self, i0: &impl ToInputArray, i1: &impl ToInputArray, flow: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Releases all inner buffers.
	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:425
	// ("cv::DenseOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DenseOpticalFlow_collectGarbage(self.as_raw_mut_DenseOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DenseOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:413
pub struct DenseOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { DenseOpticalFlow }

impl Drop for DenseOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_DenseOpticalFlow_delete(self.as_raw_mut_DenseOpticalFlow()) };
	}
}

unsafe impl Send for DenseOpticalFlow {}

impl core::AlgorithmTraitConst for DenseOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DenseOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DenseOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for DenseOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for DenseOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DenseOpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl DenseOpticalFlow {
}

boxed_cast_descendant! { DenseOpticalFlow, crate::video::DualTVL1OpticalFlow, cv_DenseOpticalFlow_to_DualTVL1OpticalFlow }

boxed_cast_descendant! { DenseOpticalFlow, crate::video::FarnebackOpticalFlow, cv_DenseOpticalFlow_to_FarnebackOpticalFlow }

boxed_cast_base! { DenseOpticalFlow, core::Algorithm, cv_DenseOpticalFlow_to_Algorithm }

impl std::fmt::Debug for DenseOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DenseOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::video::DualTVL1OpticalFlow]
// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:491
pub trait DualTVL1OpticalFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau
	// getTau()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:496
	// ("cv::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda
	// getLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:501
	// ("cv::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta
	// getTheta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:506
	// ("cv::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_theta(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma
	// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:511
	// ("cv::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_gamma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber
	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:516
	// ("cv::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scales_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber
	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:521
	// ("cv::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_warpings_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon
	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:526
	// ("cv::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations
	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:531
	// ("cv::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_inner_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations
	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:536
	// ("cv::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_outer_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow
	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:541
	// ("cv::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep
	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:546
	// ("cv::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_step(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering
	// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:551
	// ("cv::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_median_filtering(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::DualTVL1OpticalFlow]
pub trait DualTVL1OpticalFlowTrait: crate::video::DenseOpticalFlowTrait + crate::video::DualTVL1OpticalFlowTraitConst {
	fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau getTau
	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:498
	// ("cv::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda getLambda
	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:503
	// ("cv::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta getTheta
	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:508
	// ("cv::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_theta(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma getGamma
	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:513
	// ("cv::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setGamma_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber getScalesNumber
	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:518
	// ("cv::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:523
	// ("cv::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon getEpsilon
	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:528
	// ("cv::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations getInnerIterations
	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:533
	// ("cv::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations getOuterIterations
	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:538
	// ("cv::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:543
	// ("cv::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep getScaleStep
	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:548
	// ("cv::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_scale_step(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering getMedianFiltering
	// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:553
	// ("cv::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_median_filtering(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// "Dual TV L1" Optical Flow Algorithm.
///
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Javier2012) .
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
///
/// *   member double tau
///    Time step of the numerical scheme.
///
/// *   member double lambda
///    Weight parameter for the data term, attachment parameter. This is the most relevant
///    parameter, which determines the smoothness of the output. The smaller this parameter is,
///    the smoother the solutions we obtain. It depends on the range of motions of the images, so
///    its value should be adapted to each image sequence.
///
/// *   member double theta
///    Weight parameter for (u - v)\^2, tightness parameter. It serves as a link between the
///    attachment and the regularization terms. In theory, it should have a small value in order
///    to maintain both parts in correspondence. The method is stable for a large range of values
///    of this parameter.
///
/// *   member int nscales
///    Number of scales used to create the pyramid of images.
///
/// *   member int warps
///    Number of warpings per scale. Represents the number of times that I1(x+u0) and grad(
///    I1(x+u0) ) are computed per scale. This is a parameter that assures the stability of the
///    method. It also affects the running time, so it is a compromise between speed and
///    accuracy.
///
/// *   member double epsilon
///    Stopping criterion threshold used in the numerical scheme, which is a trade-off between
///    precision and running time. A small value will yield more accurate solutions at the
///    expense of a slower convergence.
///
/// *   member int iterations
///    Stopping criterion iterations number used in the numerical scheme.
///
/// C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:491
pub struct DualTVL1OpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { DualTVL1OpticalFlow }

impl Drop for DualTVL1OpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_DualTVL1OpticalFlow_delete(self.as_raw_mut_DualTVL1OpticalFlow()) };
	}
}

unsafe impl Send for DualTVL1OpticalFlow {}

impl core::AlgorithmTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::video::DualTVL1OpticalFlowTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DualTVL1OpticalFlowTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, crate::video::DualTVL1OpticalFlowTraitConst, as_raw_DualTVL1OpticalFlow, crate::video::DualTVL1OpticalFlowTrait, as_raw_mut_DualTVL1OpticalFlow }

impl DualTVL1OpticalFlow {
	/// Creates instance of cv::DualTVL1OpticalFlow
	///
	/// ## C++ default parameters
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
	// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
	#[inline]
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> Result<core::Ptr<crate::video::DualTVL1OpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau, lambda, theta, nscales, warps, epsilon, innner_iterations, outer_iterations, scale_step, gamma, median_filtering, use_initial_flow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates instance of cv::DualTVL1OpticalFlow
	///
	/// ## Note
	/// This alternative version of [DualTVL1OpticalFlow::create] function uses the following default values for its arguments:
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	// cv::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
	// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::video::DualTVL1OpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DualTVL1OpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DualTVL1OpticalFlow, core::Algorithm, cv_DualTVL1OpticalFlow_to_Algorithm }

boxed_cast_base! { DualTVL1OpticalFlow, crate::video::DenseOpticalFlow, cv_DualTVL1OpticalFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for DualTVL1OpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DualTVL1OpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::video::FarnebackOpticalFlow]
// FarnebackOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:577
pub trait FarnebackOpticalFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void;

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:580
	// ("cv::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:583
	// ("cv::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pyr_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:586
	// ("cv::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fast_pyramids(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:589
	// ("cv::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getWinSize_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:592
	// ("cv::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_iters(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getNumIters_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPolyN()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:595
	// ("cv::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_poly_n(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPolyN_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:598
	// ("cv::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_poly_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:601
	// ("cv::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getFlags_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::FarnebackOpticalFlow]
pub trait FarnebackOpticalFlowTrait: crate::video::DenseOpticalFlowTrait + crate::video::FarnebackOpticalFlowTraitConst {
	fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:581
	// ("cv::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
	#[inline]
	fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_mut_FarnebackOpticalFlow(), num_levels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:584
	// ("cv::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
	#[inline]
	fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_FarnebackOpticalFlow(), pyr_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:587
	// ("cv::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
	#[inline]
	fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_mut_FarnebackOpticalFlow(), fast_pyramids, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:590
	// ("cv::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
	#[inline]
	fn set_win_size(&mut self, win_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setWinSize_int(self.as_raw_mut_FarnebackOpticalFlow(), win_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:593
	// ("cv::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
	#[inline]
	fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setNumIters_int(self.as_raw_mut_FarnebackOpticalFlow(), num_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:596
	// ("cv::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
	#[inline]
	fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_FarnebackOpticalFlow(), poly_n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:599
	// ("cv::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
	#[inline]
	fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_FarnebackOpticalFlow(), poly_sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:602
	// ("cv::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_FarnebackOpticalFlow(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class computing a dense optical flow using the Gunnar Farneback's algorithm.
// FarnebackOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:577
pub struct FarnebackOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { FarnebackOpticalFlow }

impl Drop for FarnebackOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_FarnebackOpticalFlow_delete(self.as_raw_mut_FarnebackOpticalFlow()) };
	}
}

unsafe impl Send for FarnebackOpticalFlow {}

impl core::AlgorithmTraitConst for FarnebackOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FarnebackOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for FarnebackOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FarnebackOpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::video::FarnebackOpticalFlowTraitConst for FarnebackOpticalFlow {
	#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::FarnebackOpticalFlowTrait for FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FarnebackOpticalFlow, crate::video::FarnebackOpticalFlowTraitConst, as_raw_FarnebackOpticalFlow, crate::video::FarnebackOpticalFlowTrait, as_raw_mut_FarnebackOpticalFlow }

impl FarnebackOpticalFlow {
	/// ## C++ default parameters
	/// * num_levels: 5
	/// * pyr_scale: 0.5
	/// * fast_pyramids: false
	/// * win_size: 13
	/// * num_iters: 10
	/// * poly_n: 5
	/// * poly_sigma: 1.1
	/// * flags: 0
	// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
	// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
	#[inline]
	pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<core::Ptr<crate::video::FarnebackOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::FarnebackOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FarnebackOpticalFlow::create] function uses the following default values for its arguments:
	/// * num_levels: 5
	/// * pyr_scale: 0.5
	/// * fast_pyramids: false
	/// * win_size: 13
	/// * num_iters: 10
	/// * poly_n: 5
	/// * poly_sigma: 1.1
	/// * flags: 0
	// cv::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
	// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::video::FarnebackOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::FarnebackOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FarnebackOpticalFlow, core::Algorithm, cv_FarnebackOpticalFlow_to_Algorithm }

boxed_cast_base! { FarnebackOpticalFlow, crate::video::DenseOpticalFlow, cv_FarnebackOpticalFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for FarnebackOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FarnebackOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::video::KalmanFilter]
// KalmanFilter /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:360
pub trait KalmanFilterTraitConst {
	fn as_raw_KalmanFilter(&self) -> *const c_void;

	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	// cv::KalmanFilter::statePre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::statePre", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn state_pre(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propStatePre_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	// cv::KalmanFilter::statePost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::statePost", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn state_post(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propStatePost_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// state transition matrix (A)
	// cv::KalmanFilter::transitionMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::transitionMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn transition_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTransitionMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// control matrix (B) (not used if there is no control)
	// cv::KalmanFilter::controlMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::controlMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn control_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propControlMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// measurement matrix (H)
	// cv::KalmanFilter::measurementMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::measurementMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn measurement_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propMeasurementMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// process noise covariance matrix (Q)
	// cv::KalmanFilter::processNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::processNoiseCov", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn process_noise_cov(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propProcessNoiseCov_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// measurement noise covariance matrix (R)
	// cv::KalmanFilter::measurementNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::measurementNoiseCov", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn measurement_noise_cov(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propMeasurementNoiseCov_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	// cv::KalmanFilter::errorCovPre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::errorCovPre", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn error_cov_pre(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propErrorCovPre_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	// cv::KalmanFilter::gain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::gain", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn gain(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propGain_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	// cv::KalmanFilter::errorCovPost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::errorCovPost", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn error_cov_post(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propErrorCovPost_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::KalmanFilter::temp1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::temp1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn temp1(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp1_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::KalmanFilter::temp2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::temp2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn temp2(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp2_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::KalmanFilter::temp3() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::temp3", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn temp3(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp3_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::KalmanFilter::temp4() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::temp4", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn temp4(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp4_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::KalmanFilter::temp5() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::temp5", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn temp5(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp5_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::video::KalmanFilter]
pub trait KalmanFilterTrait: crate::video::KalmanFilterTraitConst {
	fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void;

	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	// cv::KalmanFilter::setStatePre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::setStatePre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_state_pre(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propStatePre_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	// cv::KalmanFilter::setStatePost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::setStatePost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_state_post(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propStatePost_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// state transition matrix (A)
	// cv::KalmanFilter::setTransitionMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::setTransitionMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_transition_matrix(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTransitionMatrix_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// control matrix (B) (not used if there is no control)
	// cv::KalmanFilter::setControlMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::setControlMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_control_matrix(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propControlMatrix_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// measurement matrix (H)
	// cv::KalmanFilter::setMeasurementMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::setMeasurementMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_measurement_matrix(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propMeasurementMatrix_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// process noise covariance matrix (Q)
	// cv::KalmanFilter::setProcessNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::setProcessNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_process_noise_cov(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propProcessNoiseCov_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// measurement noise covariance matrix (R)
	// cv::KalmanFilter::setMeasurementNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::setMeasurementNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_measurement_noise_cov(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propMeasurementNoiseCov_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	// cv::KalmanFilter::setErrorCovPre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::setErrorCovPre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_error_cov_pre(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propErrorCovPre_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	// cv::KalmanFilter::setGain(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::setGain", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_gain(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propGain_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	// cv::KalmanFilter::setErrorCovPost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::setErrorCovPost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_error_cov_post(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propErrorCovPost_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	// cv::KalmanFilter::setTemp1(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::setTemp1", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_temp1(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp1_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	// cv::KalmanFilter::setTemp2(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::setTemp2", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_temp2(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp2_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	// cv::KalmanFilter::setTemp3(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::setTemp3", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_temp3(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp3_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	// cv::KalmanFilter::setTemp4(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::setTemp4", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_temp4(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp4_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	// cv::KalmanFilter::setTemp5(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::setTemp5", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_temp5(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_propTemp5_const_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_Mat()) };
		ret
	}

	/// Re-initializes Kalman filter. The previous content is destroyed.
	///
	/// ## Parameters
	/// * dynamParams: Dimensionality of the state.
	/// * measureParams: Dimensionality of the measurement.
	/// * controlParams: Dimensionality of the control vector.
	/// * type: Type of the created matrices that should be CV_32F or CV_64F.
	///
	/// ## C++ default parameters
	/// * control_params: 0
	/// * typ: CV_32F
	// init(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	#[inline]
	fn init(&mut self, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_init_int_int_int_int(self.as_raw_mut_KalmanFilter(), dynam_params, measure_params, control_params, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Re-initializes Kalman filter. The previous content is destroyed.
	///
	/// ## Parameters
	/// * dynamParams: Dimensionality of the state.
	/// * measureParams: Dimensionality of the measurement.
	/// * controlParams: Dimensionality of the control vector.
	/// * type: Type of the created matrices that should be CV_32F or CV_64F.
	///
	/// ## Note
	/// This alternative version of [KalmanFilterTrait::init] function uses the following default values for its arguments:
	/// * control_params: 0
	/// * typ: CV_32F
	// cv::KalmanFilter::init(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	#[inline]
	fn init_def(&mut self, dynam_params: i32, measure_params: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_init_int_int(self.as_raw_mut_KalmanFilter(), dynam_params, measure_params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes a predicted state.
	///
	/// ## Parameters
	/// * control: The optional input control
	///
	/// ## C++ default parameters
	/// * control: Mat()
	// predict(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, ["control"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn predict(&mut self, control: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_predict_const_MatR(self.as_raw_mut_KalmanFilter(), control.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes a predicted state.
	///
	/// ## Parameters
	/// * control: The optional input control
	///
	/// ## Note
	/// This alternative version of [KalmanFilterTrait::predict] function uses the following default values for its arguments:
	/// * control: Mat()
	// cv::KalmanFilter::predict() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn predict_def(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_predict(self.as_raw_mut_KalmanFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Updates the predicted state from the measurement.
	///
	/// ## Parameters
	/// * measurement: The measured system parameters
	// correct(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:391
	// ("cv::KalmanFilter::correct", vec![(pred!(mut, ["measurement"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn correct(&mut self, measurement: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_correct_const_MatR(self.as_raw_mut_KalmanFilter(), measurement.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Kalman filter class.
///
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality.
///
/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
/// with cvReleaseKalman(&kalmanFilter)
// KalmanFilter /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:360
pub struct KalmanFilter {
	ptr: *mut c_void,
}

opencv_type_boxed! { KalmanFilter }

impl Drop for KalmanFilter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_KalmanFilter_delete(self.as_raw_mut_KalmanFilter()) };
	}
}

unsafe impl Send for KalmanFilter {}

impl crate::video::KalmanFilterTraitConst for KalmanFilter {
	#[inline] fn as_raw_KalmanFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::KalmanFilterTrait for KalmanFilter {
	#[inline] fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KalmanFilter, crate::video::KalmanFilterTraitConst, as_raw_KalmanFilter, crate::video::KalmanFilterTrait, as_raw_mut_KalmanFilter }

impl KalmanFilter {
	// KalmanFilter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:363
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::video::KalmanFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_KalmanFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::KalmanFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// ## Parameters
	/// * dynamParams: Dimensionality of the state.
	/// * measureParams: Dimensionality of the measurement.
	/// * controlParams: Dimensionality of the control vector.
	/// * type: Type of the created matrices that should be CV_32F or CV_64F.
	///
	/// ## C++ default parameters
	/// * control_params: 0
	/// * typ: CV_32F
	// KalmanFilter(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn new(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<crate::video::KalmanFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params, measure_params, control_params, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::KalmanFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * dynamParams: Dimensionality of the state.
	/// * measureParams: Dimensionality of the measurement.
	/// * controlParams: Dimensionality of the control vector.
	/// * type: Type of the created matrices that should be CV_32F or CV_64F.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * control_params: 0
	/// * typ: CV_32F
	// cv::KalmanFilter::KalmanFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	#[inline]
	pub fn new_def(dynam_params: i32, measure_params: i32) -> Result<crate::video::KalmanFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int(dynam_params, measure_params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::KalmanFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for KalmanFilter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KalmanFilter")
			.field("state_pre", &crate::video::KalmanFilterTraitConst::state_pre(self))
			.field("state_post", &crate::video::KalmanFilterTraitConst::state_post(self))
			.field("transition_matrix", &crate::video::KalmanFilterTraitConst::transition_matrix(self))
			.field("control_matrix", &crate::video::KalmanFilterTraitConst::control_matrix(self))
			.field("measurement_matrix", &crate::video::KalmanFilterTraitConst::measurement_matrix(self))
			.field("process_noise_cov", &crate::video::KalmanFilterTraitConst::process_noise_cov(self))
			.field("measurement_noise_cov", &crate::video::KalmanFilterTraitConst::measurement_noise_cov(self))
			.field("error_cov_pre", &crate::video::KalmanFilterTraitConst::error_cov_pre(self))
			.field("gain", &crate::video::KalmanFilterTraitConst::gain(self))
			.field("error_cov_post", &crate::video::KalmanFilterTraitConst::error_cov_post(self))
			.field("temp1", &crate::video::KalmanFilterTraitConst::temp1(self))
			.field("temp2", &crate::video::KalmanFilterTraitConst::temp2(self))
			.field("temp3", &crate::video::KalmanFilterTraitConst::temp3(self))
			.field("temp4", &crate::video::KalmanFilterTraitConst::temp4(self))
			.field("temp5", &crate::video::KalmanFilterTraitConst::temp5(self))
			.finish()
	}
}

/// Constant methods for [crate::video::SparseOpticalFlow]
// SparseOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:430
pub trait SparseOpticalFlowTraitConst: core::AlgorithmTraitConst {
	fn as_raw_SparseOpticalFlow(&self) -> *const c_void;

}

/// Mutable methods for [crate::video::SparseOpticalFlow]
pub trait SparseOpticalFlowTrait: core::AlgorithmTrait + crate::video::SparseOpticalFlowTraitConst {
	fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void;

	/// Calculates a sparse optical flow.
	///
	/// ## Parameters
	/// * prevImg: First input image.
	/// * nextImg: Second input image of the same size and the same type as prevImg.
	/// * prevPts: Vector of 2D points for which the flow needs to be found.
	/// * nextPts: Output vector of 2D points containing the calculated new positions of input features in the second image.
	/// * status: Output status vector. Each element of the vector is set to 1 if the
	///               flow for the corresponding features has been found. Otherwise, it is set to 0.
	/// * err: Optional output vector that contains error response for each point (inverse confidence).
	///
	/// ## C++ default parameters
	/// * err: cv::noArray()
	// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
	// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn calc(&mut self, prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, err: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		output_array_arg!(err);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates a sparse optical flow.
	///
	/// ## Parameters
	/// * prevImg: First input image.
	/// * nextImg: Second input image of the same size and the same type as prevImg.
	/// * prevPts: Vector of 2D points for which the flow needs to be found.
	/// * nextPts: Output vector of 2D points containing the calculated new positions of input features in the second image.
	/// * status: Output status vector. Each element of the vector is set to 1 if the
	///               flow for the corresponding features has been found. Otherwise, it is set to 0.
	/// * err: Optional output vector that contains error response for each point (inverse confidence).
	///
	/// ## Note
	/// This alternative version of [SparseOpticalFlowTrait::calc] function uses the following default values for its arguments:
	/// * err: cv::noArray()
	// cv::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
	// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn calc_def(&mut self, prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(self.as_raw_mut_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base interface for sparse optical flow algorithms.
// SparseOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:430
pub struct SparseOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SparseOpticalFlow }

impl Drop for SparseOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SparseOpticalFlow_delete(self.as_raw_mut_SparseOpticalFlow()) };
	}
}

unsafe impl Send for SparseOpticalFlow {}

impl core::AlgorithmTraitConst for SparseOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SparseOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparseOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::SparseOpticalFlowTraitConst for SparseOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for SparseOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparseOpticalFlow, crate::video::SparseOpticalFlowTraitConst, as_raw_SparseOpticalFlow, crate::video::SparseOpticalFlowTrait, as_raw_mut_SparseOpticalFlow }

impl SparseOpticalFlow {
}

boxed_cast_descendant! { SparseOpticalFlow, crate::video::SparsePyrLKOpticalFlow, cv_SparseOpticalFlow_to_SparsePyrLKOpticalFlow }

boxed_cast_base! { SparseOpticalFlow, core::Algorithm, cv_SparseOpticalFlow_to_Algorithm }

impl std::fmt::Debug for SparseOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SparseOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::video::SparsePyrLKOpticalFlow]
// SparsePyrLKOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:624
pub trait SparsePyrLKOpticalFlowTraitConst: crate::video::SparseOpticalFlowTraitConst {
	fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void;

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:627
	// ("cv::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_win_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:630
	// ("cv::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:633
	// ("cv::SparsePyrLKOpticalFlow::getTermCriteria", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getTermCriteria_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:636
	// ("cv::SparsePyrLKOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getFlags_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinEigThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:639
	// ("cv::SparsePyrLKOpticalFlow::getMinEigThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_eig_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::video::SparsePyrLKOpticalFlow]
pub trait SparsePyrLKOpticalFlowTrait: crate::video::SparseOpticalFlowTrait + crate::video::SparsePyrLKOpticalFlowTraitConst {
	fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void;

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:628
	// ("cv::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
	#[inline]
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_SparsePyrLKOpticalFlow(), &win_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:631
	// ("cv::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
	#[inline]
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), max_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTermCriteria(TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:634
	// ("cv::SparsePyrLKOpticalFlow::setTermCriteria", vec![(pred!(mut, ["crit"], ["cv::TermCriteria*"]), _)]),
	#[inline]
	fn set_term_criteria(&mut self, crit: &mut core::TermCriteria) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(self.as_raw_mut_SparsePyrLKOpticalFlow(), crit, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:637
	// ("cv::SparsePyrLKOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setFlags_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinEigThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:640
	// ("cv::SparsePyrLKOpticalFlow::setMinEigThreshold", vec![(pred!(mut, ["minEigThreshold"], ["double"]), _)]),
	#[inline]
	fn set_min_eig_threshold(&mut self, min_eig_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(self.as_raw_mut_SparsePyrLKOpticalFlow(), min_eig_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class used for calculating a sparse optical flow.
///
/// The class can calculate an optical flow for a sparse feature set using the
/// iterative Lucas-Kanade method with pyramids.
/// ## See also
/// calcOpticalFlowPyrLK
// SparsePyrLKOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:624
pub struct SparsePyrLKOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SparsePyrLKOpticalFlow }

impl Drop for SparsePyrLKOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_delete(self.as_raw_mut_SparsePyrLKOpticalFlow()) };
	}
}

unsafe impl Send for SparsePyrLKOpticalFlow {}

impl core::AlgorithmTraitConst for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLKOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::SparseOpticalFlowTraitConst for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLKOpticalFlow, crate::video::SparseOpticalFlowTraitConst, as_raw_SparseOpticalFlow, crate::video::SparseOpticalFlowTrait, as_raw_mut_SparseOpticalFlow }

impl crate::video::SparsePyrLKOpticalFlowTraitConst for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::SparsePyrLKOpticalFlowTrait for SparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLKOpticalFlow, crate::video::SparsePyrLKOpticalFlowTraitConst, as_raw_SparsePyrLKOpticalFlow, crate::video::SparsePyrLKOpticalFlowTrait, as_raw_mut_SparsePyrLKOpticalFlow }

impl SparsePyrLKOpticalFlow {
	/// ## C++ default parameters
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
	/// * flags: 0
	/// * min_eig_threshold: 1e-4
	// create(Size, int, TermCriteria, int, double)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
	// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "crit", "flags", "minEigThreshold"], ["cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
	#[inline]
	pub fn create(win_size: core::Size, max_level: i32, crit: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<core::Ptr<crate::video::SparsePyrLKOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(&win_size, max_level, &crit, flags, min_eig_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::SparsePyrLKOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SparsePyrLKOpticalFlow::create] function uses the following default values for its arguments:
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
	/// * flags: 0
	/// * min_eig_threshold: 1e-4
	// cv::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
	// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::video::SparsePyrLKOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::SparsePyrLKOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SparsePyrLKOpticalFlow, core::Algorithm, cv_SparsePyrLKOpticalFlow_to_Algorithm }

boxed_cast_base! { SparsePyrLKOpticalFlow, crate::video::SparseOpticalFlow, cv_SparsePyrLKOpticalFlow_to_SparseOpticalFlow }

impl std::fmt::Debug for SparsePyrLKOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SparsePyrLKOpticalFlow")
			.finish()
	}
}
