#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Video Analysis
//!   # Motion Analysis
//!   # Object Tracking
//!   # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::KalmanFilterTraitConst, super::KalmanFilterTrait, super::DenseOpticalFlowConst, super::DenseOpticalFlow, super::SparseOpticalFlowConst, super::SparseOpticalFlow, super::FarnebackOpticalFlowConst, super::FarnebackOpticalFlow, super::VariationalRefinementConst, super::VariationalRefinement, super::DISOpticalFlowConst, super::DISOpticalFlow, super::SparsePyrLKOpticalFlowConst, super::SparsePyrLKOpticalFlow, super::TrackerConst, super::Tracker, super::TrackerMILConst, super::TrackerMIL, super::TrackerGOTURN_ParamsTraitConst, super::TrackerGOTURN_ParamsTrait, super::TrackerGOTURNConst, super::TrackerGOTURN, super::TrackerDaSiamRPN_ParamsTraitConst, super::TrackerDaSiamRPN_ParamsTrait, super::TrackerDaSiamRPNConst, super::TrackerDaSiamRPN, super::BackgroundSubtractorConst, super::BackgroundSubtractor, super::BackgroundSubtractorMOG2Const, super::BackgroundSubtractorMOG2, super::BackgroundSubtractorKNNConst, super::BackgroundSubtractorKNN };
}

pub const DISOpticalFlow_PRESET_FAST: i32 = 1;
pub const DISOpticalFlow_PRESET_MEDIUM: i32 = 2;
pub const DISOpticalFlow_PRESET_ULTRAFAST: i32 = 0;
pub const MOTION_AFFINE: i32 = 2;
pub const MOTION_EUCLIDEAN: i32 = 1;
pub const MOTION_HOMOGRAPHY: i32 = 3;
pub const MOTION_TRANSLATION: i32 = 0;
pub const OPTFLOW_FARNEBACK_GAUSSIAN: i32 = 256;
pub const OPTFLOW_LK_GET_MIN_EIGENVALS: i32 = 8;
pub const OPTFLOW_USE_INITIAL_FLOW: i32 = 4;
/// Finds an object center, size, and orientation.
/// 
/// ## Parameters
/// * probImage: Back projection of the object histogram. See calcBackProject.
/// * window: Initial search window.
/// * criteria: Stop criteria for the underlying meanShift.
/// returns
/// (in old interfaces) Number of iterations CAMSHIFT took to converge
/// The function implements the CAMSHIFT object tracking algorithm [Bradski98](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bradski98) . First, it finds an
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
#[inline]
pub fn cam_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<core::RotatedRect> {
	input_array_arg!(prob_image);
	unsafe { sys::cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.opencv_as_extern()) }.into_result().map(|r| unsafe { core::RotatedRect::opencv_from_extern(r) } )
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
#[inline]
pub fn build_optical_flow_pyramid(img: &dyn core::ToInputArray, pyramid: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> Result<i32> {
	input_array_arg!(img);
	output_array_arg!(pyramid);
	unsafe { sys::cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img.as_raw__InputArray(), pyramid.as_raw__OutputArray(), win_size.opencv_as_extern(), max_level, with_derivatives, pyr_border, deriv_border, try_reuse_input_image) }.into_result()
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
/// The function finds an optical flow for each prev pixel using the [Farneback2003](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Farneback2003) algorithm so that
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
#[inline]
pub fn calc_optical_flow_farneback(prev: &dyn core::ToInputArray, next: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<()> {
	input_array_arg!(prev);
	input_array_arg!(next);
	input_output_array_arg!(flow);
	unsafe { sys::cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev.as_raw__InputArray(), next.as_raw__InputArray(), flow.as_raw__InputOutputArray(), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags) }.into_result()
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
/// optical flow equations (this matrix is called a spatial gradient matrix in [Bouguet00](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bouguet00)), divided
/// by number of pixels in a window; if this value is less than minEigThreshold, then a corresponding
/// feature is filtered out and its flow is not processed, so it allows to remove bad points and get a
/// performance boost.
/// 
/// The function implements a sparse iterative version of the Lucas-Kanade optical flow in pyramids. See
/// [Bouguet00](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bouguet00) . The function is parallelized with the TBB library.
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
#[inline]
pub fn calc_optical_flow_pyr_lk(prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, criteria: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	unsafe { sys::cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), win_size.opencv_as_extern(), max_level, criteria.opencv_as_extern(), flags, min_eig_threshold) }.into_result()
}

/// Computes the Enhanced Correlation Coefficient value between two images [EP08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_EP08) .
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
#[inline]
pub fn compute_ecc(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, input_mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_array_arg!(input_mask);
	unsafe { sys::cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), input_mask.as_raw__InputArray()) }.into_result()
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
#[inline]
pub fn create_background_subtractor_knn(history: i32, dist2_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<dyn crate::video::BackgroundSubtractorKNN>> {
	unsafe { sys::cv_createBackgroundSubtractorKNN_int_double_bool(history, dist2_threshold, detect_shadows) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::BackgroundSubtractorKNN>::opencv_from_extern(r) } )
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
#[inline]
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<dyn crate::video::BackgroundSubtractorMOG2>> {
	unsafe { sys::cv_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::BackgroundSubtractorMOG2>::opencv_from_extern(r) } )
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
/// 
/// 
/// **Deprecated**: Use cv::estimateAffine2D, cv::estimateAffinePartial2D instead. If you are using this function
/// with images, extract points using cv::calcOpticalFlowPyrLK and then use the estimation functions.
/// ## See also
/// estimateAffine2D, estimateAffinePartial2D, getAffineTransform, getPerspectiveTransform, findHomography
#[deprecated = "Use cv::estimateAffine2D, cv::estimateAffinePartial2D instead. If you are using this function"]
#[inline]
pub fn estimate_rigid_transform(src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, full_affine: bool) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	unsafe { sys::cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), full_affine) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_EP08) .
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
/// ([EP08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_EP08)), that is
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
#[inline]
pub fn find_transform_ecc_1(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, warp_matrix: &mut dyn core::ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, criteria.opencv_as_extern(), input_mask.as_raw__InputArray()) }.into_result()
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_EP08) .
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
/// ([EP08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_EP08)), that is
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
#[inline]
pub fn find_transform_ecc(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, warp_matrix: &mut dyn core::ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &dyn core::ToInputArray, gauss_filt_size: i32) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, criteria.opencv_as_extern(), input_mask.as_raw__InputArray(), gauss_filt_size) }.into_result()
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
#[inline]
pub fn mean_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<i32> {
	input_array_arg!(prob_image);
	unsafe { sys::cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.opencv_as_extern()) }.into_result()
}

/// Read a .flo file
/// 
/// ## Parameters
/// * path: Path to the file to be loaded
/// 
/// The function readOpticalFlow loads a flow field from a file and returns it as a single matrix.
/// Resulting Mat has a type CV_32FC2 - floating-point, 2-channel. First channel corresponds to the
/// flow in the horizontal direction (u), second - vertical (v).
#[inline]
pub fn read_optical_flow(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	unsafe { sys::cv_readOpticalFlow_const_StringR(path.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Write a .flo to disk
/// 
/// ## Parameters
/// * path: Path to the file to be written
/// * flow: Flow field to be stored
/// 
/// The function stores a flow field in a file, returns true on success, false otherwise.
/// The flow field must be a 2-channel, floating-point matrix (CV_32FC2). First channel corresponds
/// to the flow in the horizontal direction (u), second - vertical (v).
#[inline]
pub fn write_optical_flow(path: &str, flow: &dyn core::ToInputArray) -> Result<bool> {
	extern_container_arg!(path);
	input_array_arg!(flow);
	unsafe { sys::cv_writeOpticalFlow_const_StringR_const__InputArrayR(path.opencv_as_extern(), flow.as_raw__InputArray()) }.into_result()
}

/// Base class for background/foreground segmentation. :
/// 
/// The class is only used to define the common interface for the whole family of background/foreground
/// segmentation algorithms.
pub trait BackgroundSubtractorConst: core::AlgorithmTraitConst {
	fn as_raw_BackgroundSubtractor(&self) -> *const c_void;

	/// Computes a background image.
	/// 
	/// ## Parameters
	/// * backgroundImage: The output background image.
	/// 
	/// 
	/// Note: Sometimes the background image can be very blurry, as it contain the average background
	/// statistics.
	#[inline]
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(background_image);
		unsafe { sys::cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(self.as_raw_BackgroundSubtractor(), background_image.as_raw__OutputArray()) }.into_result()
	}
	
}

pub trait BackgroundSubtractor: core::AlgorithmTrait + crate::video::BackgroundSubtractorConst {
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
	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		unsafe { sys::cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractor(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
	}
	
}

/// K-nearest neighbours - based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the K-nearest neighbours background subtraction described in [Zivkovic2006](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
/// Very efficient if number of foreground pixels is low.
pub trait BackgroundSubtractorKNNConst: crate::video::BackgroundSubtractorConst {
	fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void;

	/// Returns the number of last frames that affect the background model
	#[inline]
	fn get_history(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getHistory_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the number of data samples in the background model
	#[inline]
	fn get_n_samples(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getNSamples_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the threshold on the squared distance between the pixel and the sample
	/// 
	/// The threshold on the squared distance between the pixel and the sample to decide whether a pixel is
	/// close to a data sample.
	#[inline]
	fn get_dist2_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getDist2Threshold_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the number of neighbours, the k in the kNN.
	/// 
	/// K is the number of samples that need to be within dist2Threshold in order to decide that that
	/// pixel is matching the kNN background model.
	#[inline]
	fn getk_nn_samples(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getkNNSamples_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the shadow detection flag
	/// 
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorKNN for
	/// details.
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getDetectShadows_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the shadow value
	/// 
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowValue_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Returns the shadow threshold
	/// 
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowThreshold_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
}

pub trait BackgroundSubtractorKNN: crate::video::BackgroundSubtractor + crate::video::BackgroundSubtractorKNNConst {
	fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void;

	/// Sets the number of last frames that affect the background model
	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setHistory_int(self.as_raw_mut_BackgroundSubtractorKNN(), history) }.into_result()
	}
	
	/// Sets the number of data samples in the background model.
	/// 
	/// The model needs to be reinitalized to reserve memory.
	#[inline]
	fn set_n_samples(&mut self, _n_n: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _n_n) }.into_result()
	}
	
	/// Sets the threshold on the squared distance
	#[inline]
	fn set_dist2_threshold(&mut self, _dist2_threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setDist2Threshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), _dist2_threshold) }.into_result()
	}
	
	/// Sets the k in the kNN. How many nearest neighbours need to match.
	#[inline]
	fn setk_nn_samples(&mut self, _nk_nn: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setkNNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _nk_nn) }.into_result()
	}
	
	/// Enables or disables shadow detection
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorKNN(), detect_shadows) }.into_result()
	}
	
	/// Sets the shadow value
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorKNN(), value) }.into_result()
	}
	
	/// Sets the shadow threshold
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), threshold) }.into_result()
	}
	
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the Gaussian mixture model background subtraction described in [Zivkovic2004](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Zivkovic2004)
/// and [Zivkovic2006](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
pub trait BackgroundSubtractorMOG2Const: crate::video::BackgroundSubtractorConst {
	fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void;

	/// Returns the number of last frames that affect the background model
	#[inline]
	fn get_history(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getHistory_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the number of gaussian components in the background model
	#[inline]
	fn get_n_mixtures(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getNMixtures_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the "background ratio" parameter of the algorithm
	/// 
	/// If a foreground pixel keeps semi-constant value for about backgroundRatio\*history frames, it's
	/// considered background and added to the model as a center of a new component. It corresponds to TB
	/// parameter in the paper.
	#[inline]
	fn get_background_ratio(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the variance threshold for the pixel-model match
	/// 
	/// The main threshold on the squared Mahalanobis distance to decide if the sample is well described by
	/// the background model or not. Related to Cthr from the paper.
	#[inline]
	fn get_var_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the variance threshold for the pixel-model match used for new mixture component generation
	/// 
	/// Threshold for the squared Mahalanobis distance that helps decide when a sample is close to the
	/// existing components (corresponds to Tg in the paper). If a pixel is not close to any component, it
	/// is considered foreground or added as a new component. 3 sigma =\> Tg=3\*3=9 is default. A smaller Tg
	/// value generates more components. A higher Tg value may result in a small number of components but
	/// they can grow too large.
	#[inline]
	fn get_var_threshold_gen(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the initial variance of each gaussian component
	#[inline]
	fn get_var_init(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarInit_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	#[inline]
	fn get_var_min(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMin_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	#[inline]
	fn get_var_max(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMax_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the complexity reduction threshold
	/// 
	/// This parameter defines the number of samples needed to accept to prove the component exists. CT=0.05
	/// is a default value for all the samples. By setting CT=0 you get an algorithm very similar to the
	/// standard Stauffer&Grimson algorithm.
	#[inline]
	fn get_complexity_reduction_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the shadow detection flag
	/// 
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorMOG2 for
	/// details.
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getDetectShadows_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the shadow value
	/// 
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowValue_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Returns the shadow threshold
	/// 
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
}

pub trait BackgroundSubtractorMOG2: crate::video::BackgroundSubtractor + crate::video::BackgroundSubtractorMOG2Const {
	fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	/// Sets the number of last frames that affect the background model
	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setHistory_int(self.as_raw_mut_BackgroundSubtractorMOG2(), history) }.into_result()
	}
	
	/// Sets the number of gaussian components in the background model.
	/// 
	/// The model needs to be reinitalized to reserve memory.
	#[inline]
	fn set_n_mixtures(&mut self, nmixtures: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setNMixtures_int(self.as_raw_mut_BackgroundSubtractorMOG2(), nmixtures) }.into_result()
	}
	
	/// Sets the "background ratio" parameter of the algorithm
	#[inline]
	fn set_background_ratio(&mut self, ratio: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ratio) }.into_result()
	}
	
	/// Sets the variance threshold for the pixel-model match
	#[inline]
	fn set_var_threshold(&mut self, var_threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold) }.into_result()
	}
	
	/// Sets the variance threshold for the pixel-model match used for new mixture component generation
	#[inline]
	fn set_var_threshold_gen(&mut self, var_threshold_gen: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold_gen) }.into_result()
	}
	
	/// Sets the initial variance of each gaussian component
	#[inline]
	fn set_var_init(&mut self, var_init: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarInit_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_init) }.into_result()
	}
	
	#[inline]
	fn set_var_min(&mut self, var_min: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMin_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_min) }.into_result()
	}
	
	#[inline]
	fn set_var_max(&mut self, var_max: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMax_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_max) }.into_result()
	}
	
	/// Sets the complexity reduction threshold
	#[inline]
	fn set_complexity_reduction_threshold(&mut self, ct: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ct) }.into_result()
	}
	
	/// Enables or disables shadow detection
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorMOG2(), detect_shadows) }.into_result()
	}
	
	/// Sets the shadow value
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorMOG2(), value) }.into_result()
	}
	
	/// Sets the shadow threshold
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), threshold) }.into_result()
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
	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		unsafe { sys::cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
	}
	
}

/// DIS optical flow algorithm.
/// 
/// This class implements the Dense Inverse Search (DIS) optical flow algorithm. More
/// details about the algorithm can be found at [Kroeger2016](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Kroeger2016) . Includes three presets with preselected
/// parameters to provide reasonable trade-off between speed and quality. However, even the slowest preset is
/// still relatively fast, use DeepFlow if you need better quality and don't care about speed.
/// 
/// This implementation includes several additional features compared to the algorithm described in the paper,
/// including spatial propagation of flow vectors (@ref getUseSpatialPropagation), as well as an option to
/// utilize an initial flow approximation passed to @ref calc (which is, essentially, temporal propagation,
/// if the previous frame's flow field is passed).
pub trait DISOpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_DISOpticalFlow(&self) -> *const c_void;

	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale
	#[inline]
	fn get_finest_scale(&self) -> Result<i32> {
		unsafe { sys::cv_DISOpticalFlow_getFinestScale_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		unsafe { sys::cv_DISOpticalFlow_getPatchSize_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride
	#[inline]
	fn get_patch_stride(&self) -> Result<i32> {
		unsafe { sys::cv_DISOpticalFlow_getPatchStride_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations
	#[inline]
	fn get_gradient_descent_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_DISOpticalFlow_getGradientDescentIterations_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Number of fixed point iterations of variational refinement per scale. Set to zero to
	///    disable variational refinement completely. Higher values will typically result in more smooth and
	///    high-quality flow.
	/// ## See also
	/// setGradientDescentIterations
	#[inline]
	fn get_variational_refinement_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementIterations_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha
	#[inline]
	fn get_variational_refinement_alpha(&self) -> Result<f32> {
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementAlpha_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta
	#[inline]
	fn get_variational_refinement_delta(&self) -> Result<f32> {
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementDelta_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma
	#[inline]
	fn get_variational_refinement_gamma(&self) -> Result<f32> {
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementGamma_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization
	#[inline]
	fn get_use_mean_normalization(&self) -> Result<bool> {
		unsafe { sys::cv_DISOpticalFlow_getUseMeanNormalization_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation
	#[inline]
	fn get_use_spatial_propagation(&self) -> Result<bool> {
		unsafe { sys::cv_DISOpticalFlow_getUseSpatialPropagation_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
}

pub trait DISOpticalFlow: crate::video::DISOpticalFlowConst + crate::video::DenseOpticalFlow {
	fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void;

	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale getFinestScale
	#[inline]
	fn set_finest_scale(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setFinestScale_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize getPatchSize
	#[inline]
	fn set_patch_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setPatchSize_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride getPatchStride
	#[inline]
	fn set_patch_stride(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setPatchStride_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	#[inline]
	fn set_gradient_descent_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setGradientDescentIterations_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	#[inline]
	fn set_variational_refinement_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementIterations_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha getVariationalRefinementAlpha
	#[inline]
	fn set_variational_refinement_alpha(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementAlpha_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta getVariationalRefinementDelta
	#[inline]
	fn set_variational_refinement_delta(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementDelta_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma getVariationalRefinementGamma
	#[inline]
	fn set_variational_refinement_gamma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementGamma_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization getUseMeanNormalization
	#[inline]
	fn set_use_mean_normalization(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setUseMeanNormalization_bool(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation getUseSpatialPropagation
	#[inline]
	fn set_use_spatial_propagation(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_DISOpticalFlow_setUseSpatialPropagation_bool(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
}

impl dyn DISOpticalFlow + '_ {
	/// Creates an instance of DISOpticalFlow
	/// 
	/// ## Parameters
	/// * preset: one of PRESET_ULTRAFAST, PRESET_FAST and PRESET_MEDIUM
	/// 
	/// ## C++ default parameters
	/// * preset: DISOpticalFlow::PRESET_FAST
	#[inline]
	pub fn create(preset: i32) -> Result<core::Ptr<dyn crate::video::DISOpticalFlow>> {
		unsafe { sys::cv_DISOpticalFlow_create_int(preset) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DISOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Base class for dense optical flow algorithms
pub trait DenseOpticalFlowConst: core::AlgorithmTraitConst {
	fn as_raw_DenseOpticalFlow(&self) -> *const c_void;

}

pub trait DenseOpticalFlow: core::AlgorithmTrait + crate::video::DenseOpticalFlowConst {
	fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void;

	/// Calculates an optical flow.
	/// 
	/// ## Parameters
	/// * I0: first 8-bit single-channel input image.
	/// * I1: second input image of the same size and the same type as prev.
	/// * flow: computed flow image that has the same size as prev and type CV_32FC2.
	#[inline]
	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		unsafe { sys::cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray()) }.into_result()
	}
	
	/// Releases all inner buffers.
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_DenseOpticalFlow_collectGarbage(self.as_raw_mut_DenseOpticalFlow()) }.into_result()
	}
	
}

/// Class computing a dense optical flow using the Gunnar Farneback's algorithm.
pub trait FarnebackOpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_num_levels(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_pyr_scale(&self) -> Result<f64> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_fast_pyramids(&self) -> Result<bool> {
		unsafe { sys::cv_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_win_size(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getWinSize_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getNumIters_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_poly_n(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPolyN_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_poly_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getFlags_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
}

pub trait FarnebackOpticalFlow: crate::video::DenseOpticalFlow + crate::video::FarnebackOpticalFlowConst {
	fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_mut_FarnebackOpticalFlow(), num_levels) }.into_result()
	}
	
	#[inline]
	fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_FarnebackOpticalFlow(), pyr_scale) }.into_result()
	}
	
	#[inline]
	fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_mut_FarnebackOpticalFlow(), fast_pyramids) }.into_result()
	}
	
	#[inline]
	fn set_win_size(&mut self, win_size: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setWinSize_int(self.as_raw_mut_FarnebackOpticalFlow(), win_size) }.into_result()
	}
	
	#[inline]
	fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setNumIters_int(self.as_raw_mut_FarnebackOpticalFlow(), num_iters) }.into_result()
	}
	
	#[inline]
	fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_FarnebackOpticalFlow(), poly_n) }.into_result()
	}
	
	#[inline]
	fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_FarnebackOpticalFlow(), poly_sigma) }.into_result()
	}
	
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_FarnebackOpticalFlow(), flags) }.into_result()
	}
	
}

impl dyn FarnebackOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * num_levels: 5
	/// * pyr_scale: 0.5
	/// * fast_pyramids: false
	/// * win_size: 13
	/// * num_iters: 10
	/// * poly_n: 5
	/// * poly_sigma: 1.1
	/// * flags: 0
	#[inline]
	pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<core::Ptr<dyn crate::video::FarnebackOpticalFlow>> {
		unsafe { sys::cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::FarnebackOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Kalman filter class.
/// 
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality.
/// 
/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
/// with cvReleaseKalman(&kalmanFilter)
pub trait KalmanFilterTraitConst {
	fn as_raw_KalmanFilter(&self) -> *const c_void;

	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	#[inline]
	fn state_pre(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropStatePre_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: state_pre")
	}
	
	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	#[inline]
	fn state_post(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropStatePost_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: state_post")
	}
	
	/// state transition matrix (A)
	#[inline]
	fn transition_matrix(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTransitionMatrix_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: transition_matrix")
	}
	
	/// control matrix (B) (not used if there is no control)
	#[inline]
	fn control_matrix(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropControlMatrix_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: control_matrix")
	}
	
	/// measurement matrix (H)
	#[inline]
	fn measurement_matrix(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropMeasurementMatrix_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: measurement_matrix")
	}
	
	/// process noise covariance matrix (Q)
	#[inline]
	fn process_noise_cov(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropProcessNoiseCov_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: process_noise_cov")
	}
	
	/// measurement noise covariance matrix (R)
	#[inline]
	fn measurement_noise_cov(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropMeasurementNoiseCov_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: measurement_noise_cov")
	}
	
	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	#[inline]
	fn error_cov_pre(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropErrorCovPre_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: error_cov_pre")
	}
	
	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	#[inline]
	fn gain(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropGain_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: gain")
	}
	
	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	#[inline]
	fn error_cov_post(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropErrorCovPost_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: error_cov_post")
	}
	
	#[inline]
	fn temp1(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTemp1_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: temp1")
	}
	
	#[inline]
	fn temp2(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTemp2_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: temp2")
	}
	
	#[inline]
	fn temp3(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTemp3_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: temp3")
	}
	
	#[inline]
	fn temp4(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTemp4_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: temp4")
	}
	
	#[inline]
	fn temp5(&self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_getPropTemp5_const(self.as_raw_KalmanFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: temp5")
	}
	
}

pub trait KalmanFilterTrait: crate::video::KalmanFilterTraitConst {
	fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void;

	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	#[inline]
	fn set_state_pre(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropStatePre_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_state_pre")
	}
	
	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	#[inline]
	fn set_state_post(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropStatePost_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_state_post")
	}
	
	/// state transition matrix (A)
	#[inline]
	fn set_transition_matrix(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTransitionMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_transition_matrix")
	}
	
	/// control matrix (B) (not used if there is no control)
	#[inline]
	fn set_control_matrix(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropControlMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_control_matrix")
	}
	
	/// measurement matrix (H)
	#[inline]
	fn set_measurement_matrix(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropMeasurementMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_measurement_matrix")
	}
	
	/// process noise covariance matrix (Q)
	#[inline]
	fn set_process_noise_cov(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropProcessNoiseCov_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_process_noise_cov")
	}
	
	/// measurement noise covariance matrix (R)
	#[inline]
	fn set_measurement_noise_cov(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropMeasurementNoiseCov_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_measurement_noise_cov")
	}
	
	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	#[inline]
	fn set_error_cov_pre(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropErrorCovPre_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_error_cov_pre")
	}
	
	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	#[inline]
	fn set_gain(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropGain_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_gain")
	}
	
	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	#[inline]
	fn set_error_cov_post(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropErrorCovPost_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_error_cov_post")
	}
	
	#[inline]
	fn set_temp1(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTemp1_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_temp1")
	}
	
	#[inline]
	fn set_temp2(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTemp2_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_temp2")
	}
	
	#[inline]
	fn set_temp3(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTemp3_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_temp3")
	}
	
	#[inline]
	fn set_temp4(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTemp4_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_temp4")
	}
	
	#[inline]
	fn set_temp5(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_KalmanFilter_setPropTemp5_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_temp5")
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
	#[inline]
	fn init(&mut self, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<()> {
		unsafe { sys::cv_KalmanFilter_init_int_int_int_int(self.as_raw_mut_KalmanFilter(), dynam_params, measure_params, control_params, typ) }.into_result()
	}
	
	/// Computes a predicted state.
	/// 
	/// ## Parameters
	/// * control: The optional input control
	/// 
	/// ## C++ default parameters
	/// * control: Mat()
	#[inline]
	fn predict(&mut self, control: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_KalmanFilter_predict_const_MatR(self.as_raw_mut_KalmanFilter(), control.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Updates the predicted state from the measurement.
	/// 
	/// ## Parameters
	/// * measurement: The measured system parameters
	#[inline]
	fn correct(&mut self, measurement: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_KalmanFilter_correct_const_MatR(self.as_raw_mut_KalmanFilter(), measurement.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Kalman filter class.
/// 
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality.
/// 
/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
/// with cvReleaseKalman(&kalmanFilter)
pub struct KalmanFilter {
	ptr: *mut c_void
}

opencv_type_boxed! { KalmanFilter }

impl Drop for KalmanFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_KalmanFilter_delete(instance: *mut c_void); }
		unsafe { cv_KalmanFilter_delete(self.as_raw_mut_KalmanFilter()) };
	}
}

unsafe impl Send for KalmanFilter {}

impl crate::video::KalmanFilterTraitConst for KalmanFilter {
	#[inline] fn as_raw_KalmanFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::KalmanFilterTrait for KalmanFilter {
	#[inline] fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KalmanFilter {
	#[inline]
	pub fn default() -> Result<crate::video::KalmanFilter> {
		unsafe { sys::cv_KalmanFilter_KalmanFilter() }.into_result().map(|r| unsafe { crate::video::KalmanFilter::opencv_from_extern(r) } )
	}
	
	/// ## Parameters
	/// * dynamParams: Dimensionality of the state.
	/// * measureParams: Dimensionality of the measurement.
	/// * controlParams: Dimensionality of the control vector.
	/// * type: Type of the created matrices that should be CV_32F or CV_64F.
	/// 
	/// ## C++ default parameters
	/// * control_params: 0
	/// * typ: CV_32F
	#[inline]
	pub fn new(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<crate::video::KalmanFilter> {
		unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params, measure_params, control_params, typ) }.into_result().map(|r| unsafe { crate::video::KalmanFilter::opencv_from_extern(r) } )
	}
	
}

/// Base interface for sparse optical flow algorithms.
pub trait SparseOpticalFlowConst: core::AlgorithmTraitConst {
	fn as_raw_SparseOpticalFlow(&self) -> *const c_void;

}

pub trait SparseOpticalFlow: core::AlgorithmTrait + crate::video::SparseOpticalFlowConst {
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
	#[inline]
	fn calc(&mut self, prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		output_array_arg!(err);
		unsafe { sys::cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Class used for calculating a sparse optical flow.
/// 
/// The class can calculate an optical flow for a sparse feature set using the
/// iterative Lucas-Kanade method with pyramids.
/// ## See also
/// calcOpticalFlowPyrLK
pub trait SparsePyrLKOpticalFlowConst: crate::video::SparseOpticalFlowConst {
	fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_win_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getTermCriteria_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getFlags_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	#[inline]
	fn get_min_eig_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
}

pub trait SparsePyrLKOpticalFlow: crate::video::SparseOpticalFlow + crate::video::SparsePyrLKOpticalFlowConst {
	fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_SparsePyrLKOpticalFlow(), win_size.opencv_as_extern()) }.into_result()
	}
	
	#[inline]
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), max_level) }.into_result()
	}
	
	#[inline]
	fn set_term_criteria(&mut self, crit: &mut core::TermCriteria) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(self.as_raw_mut_SparsePyrLKOpticalFlow(), crit) }.into_result()
	}
	
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setFlags_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), flags) }.into_result()
	}
	
	#[inline]
	fn set_min_eig_threshold(&mut self, min_eig_threshold: f64) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(self.as_raw_mut_SparsePyrLKOpticalFlow(), min_eig_threshold) }.into_result()
	}
	
}

impl dyn SparsePyrLKOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
	/// * flags: 0
	/// * min_eig_threshold: 1e-4
	#[inline]
	pub fn create(win_size: core::Size, max_level: i32, crit: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow>> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size.opencv_as_extern(), max_level, crit.opencv_as_extern(), flags, min_eig_threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::SparsePyrLKOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Base abstract class for the long-term tracker
pub trait TrackerConst {
	fn as_raw_Tracker(&self) -> *const c_void;

}

pub trait Tracker: crate::video::TrackerConst {
	fn as_raw_mut_Tracker(&mut self) -> *mut c_void;

	/// Initialize the tracker with a known bounding box that surrounded the target
	/// ## Parameters
	/// * image: The initial frame
	/// * boundingBox: The initial bounding box
	#[inline]
	fn init(&mut self, image: &dyn core::ToInputArray, bounding_box: core::Rect) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_Tracker_init_const__InputArrayR_const_RectR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), &bounding_box) }.into_result()
	}
	
	/// Update the tracker, find the new most likely bounding box for the target
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box that represent the new target location, if true was returned, not
	/// modified otherwise
	/// 
	/// ## Returns
	/// True means that target was located and false means that tracker cannot locate target in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	#[inline]
	fn update(&mut self, image: &dyn core::ToInputArray, bounding_box: &mut core::Rect) -> Result<bool> {
		input_array_arg!(image);
		unsafe { sys::cv_Tracker_update_const__InputArrayR_RectR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), bounding_box) }.into_result()
	}
	
}

pub trait TrackerDaSiamRPNConst: crate::video::TrackerConst {
	fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void;

}

pub trait TrackerDaSiamRPN: crate::video::Tracker + crate::video::TrackerDaSiamRPNConst {
	fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void;

	/// Return tracking score
	#[inline]
	fn get_tracking_score(&mut self) -> Result<f32> {
		unsafe { sys::cv_TrackerDaSiamRPN_getTrackingScore(self.as_raw_mut_TrackerDaSiamRPN()) }.into_result()
	}
	
}

impl dyn TrackerDaSiamRPN + '_ {
	/// Constructor
	/// ## Parameters
	/// * parameters: DaSiamRPN parameters TrackerDaSiamRPN::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerDaSiamRPN::Params()
	#[inline]
	pub fn create(parameters: &crate::video::TrackerDaSiamRPN_Params) -> Result<core::Ptr<dyn crate::video::TrackerDaSiamRPN>> {
		unsafe { sys::cv_TrackerDaSiamRPN_create_const_ParamsR(parameters.as_raw_TrackerDaSiamRPN_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::TrackerDaSiamRPN>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerDaSiamRPN_ParamsTraitConst {
	fn as_raw_TrackerDaSiamRPN_Params(&self) -> *const c_void;

	#[inline]
	fn model(&self) -> String {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropModel_const(self.as_raw_TrackerDaSiamRPN_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: model")
	}
	
	#[inline]
	fn kernel_cls1(&self) -> String {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropKernel_cls1_const(self.as_raw_TrackerDaSiamRPN_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: kernel_cls1")
	}
	
	#[inline]
	fn kernel_r1(&self) -> String {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropKernel_r1_const(self.as_raw_TrackerDaSiamRPN_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: kernel_r1")
	}
	
	#[inline]
	fn backend(&self) -> i32 {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropBackend_const(self.as_raw_TrackerDaSiamRPN_Params()) }.into_result().expect("Infallible function failed: backend")
	}
	
	#[inline]
	fn target(&self) -> i32 {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropTarget_const(self.as_raw_TrackerDaSiamRPN_Params()) }.into_result().expect("Infallible function failed: target")
	}
	
}

pub trait TrackerDaSiamRPN_ParamsTrait: crate::video::TrackerDaSiamRPN_ParamsTraitConst {
	fn as_raw_mut_TrackerDaSiamRPN_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_model(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropModel_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_model")
	}
	
	#[inline]
	fn set_kernel_cls1(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropKernel_cls1_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_kernel_cls1")
	}
	
	#[inline]
	fn set_kernel_r1(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropKernel_r1_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_kernel_r1")
	}
	
	#[inline]
	fn set_backend(&mut self, val: i32) {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropBackend_int(self.as_raw_mut_TrackerDaSiamRPN_Params(), val) }.into_result().expect("Infallible function failed: set_backend")
	}
	
	#[inline]
	fn set_target(&mut self, val: i32) {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropTarget_int(self.as_raw_mut_TrackerDaSiamRPN_Params(), val) }.into_result().expect("Infallible function failed: set_target")
	}
	
}

pub struct TrackerDaSiamRPN_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerDaSiamRPN_Params }

impl Drop for TrackerDaSiamRPN_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerDaSiamRPN_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerDaSiamRPN_Params_delete(self.as_raw_mut_TrackerDaSiamRPN_Params()) };
	}
}

unsafe impl Send for TrackerDaSiamRPN_Params {}

impl crate::video::TrackerDaSiamRPN_ParamsTraitConst for TrackerDaSiamRPN_Params {
	#[inline] fn as_raw_TrackerDaSiamRPN_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerDaSiamRPN_ParamsTrait for TrackerDaSiamRPN_Params {
	#[inline] fn as_raw_mut_TrackerDaSiamRPN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerDaSiamRPN_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerDaSiamRPN_Params> {
		unsafe { sys::cv_TrackerDaSiamRPN_Params_Params() }.into_result().map(|r| unsafe { crate::video::TrackerDaSiamRPN_Params::opencv_from_extern(r) } )
	}
	
}

/// the GOTURN (Generic Object Tracking Using Regression Networks) tracker
/// 
/// GOTURN ([GOTURN](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_GOTURN)) is kind of trackers based on Convolutional Neural Networks (CNN). While taking all advantages of CNN trackers,
/// GOTURN is much faster due to offline training without online fine-tuning nature.
/// GOTURN tracker addresses the problem of single target tracking: given a bounding box label of an object in the first frame of the video,
/// we track that object through the rest of the video. NOTE: Current method of GOTURN does not handle occlusions; however, it is fairly
/// robust to viewpoint changes, lighting changes, and deformations.
/// Inputs of GOTURN are two RGB patches representing Target and Search patches resized to 227x227.
/// Outputs of GOTURN are predicted bounding box coordinates, relative to Search patch coordinate system, in format X1,Y1,X2,Y2.
/// Original paper is here: <http://davheld.github.io/GOTURN/GOTURN.pdf>
/// As long as original authors implementation: <https://github.com/davheld/GOTURN#train-the-tracker>
/// Implementation of training algorithm is placed in separately here due to 3d-party dependencies:
/// <https://github.com/Auron-X/GOTURN_Training_Toolkit>
/// GOTURN architecture goturn.prototxt and trained model goturn.caffemodel are accessible on opencv_extra GitHub repository.
pub trait TrackerGOTURNConst: crate::video::TrackerConst {
	fn as_raw_TrackerGOTURN(&self) -> *const c_void;

}

pub trait TrackerGOTURN: crate::video::Tracker + crate::video::TrackerGOTURNConst {
	fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void;

}

impl dyn TrackerGOTURN + '_ {
	/// Constructor
	/// ## Parameters
	/// * parameters: GOTURN parameters TrackerGOTURN::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerGOTURN::Params()
	#[inline]
	pub fn create(parameters: &crate::video::TrackerGOTURN_Params) -> Result<core::Ptr<dyn crate::video::TrackerGOTURN>> {
		unsafe { sys::cv_TrackerGOTURN_create_const_ParamsR(parameters.as_raw_TrackerGOTURN_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::TrackerGOTURN>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerGOTURN_ParamsTraitConst {
	fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void;

	#[inline]
	fn model_txt(&self) -> String {
		unsafe { sys::cv_TrackerGOTURN_Params_getPropModelTxt_const(self.as_raw_TrackerGOTURN_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: model_txt")
	}
	
	#[inline]
	fn model_bin(&self) -> String {
		unsafe { sys::cv_TrackerGOTURN_Params_getPropModelBin_const(self.as_raw_TrackerGOTURN_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: model_bin")
	}
	
}

pub trait TrackerGOTURN_ParamsTrait: crate::video::TrackerGOTURN_ParamsTraitConst {
	fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_model_txt(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_TrackerGOTURN_Params_setPropModelTxt_string(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_model_txt")
	}
	
	#[inline]
	fn set_model_bin(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_TrackerGOTURN_Params_setPropModelBin_string(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_model_bin")
	}
	
}

pub struct TrackerGOTURN_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerGOTURN_Params }

impl Drop for TrackerGOTURN_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerGOTURN_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerGOTURN_Params_delete(self.as_raw_mut_TrackerGOTURN_Params()) };
	}
}

unsafe impl Send for TrackerGOTURN_Params {}

impl crate::video::TrackerGOTURN_ParamsTraitConst for TrackerGOTURN_Params {
	#[inline] fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerGOTURN_ParamsTrait for TrackerGOTURN_Params {
	#[inline] fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerGOTURN_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerGOTURN_Params> {
		unsafe { sys::cv_TrackerGOTURN_Params_Params() }.into_result().map(|r| unsafe { crate::video::TrackerGOTURN_Params::opencv_from_extern(r) } )
	}
	
}

/// The MIL algorithm trains a classifier in an online manner to separate the object from the
/// background.
/// 
/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
/// based on [MIL](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_MIL) .
/// 
/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
pub trait TrackerMILConst: crate::video::TrackerConst {
	fn as_raw_TrackerMIL(&self) -> *const c_void;

}

pub trait TrackerMIL: crate::video::Tracker + crate::video::TrackerMILConst {
	fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void;

}

impl dyn TrackerMIL + '_ {
	/// Create MIL tracker instance
	/// ## Parameters
	/// * parameters: MIL parameters TrackerMIL::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerMIL::Params()
	#[inline]
	pub fn create(parameters: crate::video::TrackerMIL_Params) -> Result<core::Ptr<dyn crate::video::TrackerMIL>> {
		unsafe { sys::cv_TrackerMIL_create_const_ParamsR(&parameters) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::TrackerMIL>::opencv_from_extern(r) } )
	}
	
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TrackerMIL_Params {
	/// radius for gathering positive instances during init
	pub sampler_init_in_radius: f32,
	/// # negative samples to use during init
	pub sampler_init_max_neg_num: i32,
	/// size of search window
	pub sampler_search_win_size: f32,
	/// radius for gathering positive instances during tracking
	pub sampler_track_in_radius: f32,
	/// # positive samples to use during tracking
	pub sampler_track_max_pos_num: i32,
	/// # negative samples to use during tracking
	pub sampler_track_max_neg_num: i32,
	/// # features
	pub feature_set_num_features: i32,
}

opencv_type_simple! { crate::video::TrackerMIL_Params }

impl TrackerMIL_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerMIL_Params> {
		unsafe { sys::cv_TrackerMIL_Params_Params() }.into_result()
	}
	
}

/// Variational optical flow refinement
/// 
/// This class implements variational refinement of the input flow field, i.e.
/// it uses input flow to initialize the minimization of the following functional:
/// ![inline formula](https://latex.codecogs.com/png.latex?E%28U%29%20%3D%20%5Cint%5F%7B%5COmega%7D%20%5Cdelta%20%5CPsi%28E%5FI%29%20%2B%20%5Cgamma%20%5CPsi%28E%5FG%29%20%2B%20%5Calpha%20%5CPsi%28E%5FS%29%20),
/// where ![inline formula](https://latex.codecogs.com/png.latex?E%5FI%2CE%5FG%2CE%5FS) are color constancy, gradient constancy and smoothness terms
/// respectively. ![inline formula](https://latex.codecogs.com/png.latex?%5CPsi%28s%5E2%29%3D%5Csqrt%7Bs%5E2%2B%5Cepsilon%5E2%7D) is a robust penalizer to limit the
/// influence of outliers. A complete formulation and a description of the minimization
/// procedure can be found in [Brox2004](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Brox2004)
pub trait VariationalRefinementConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_VariationalRefinement(&self) -> *const c_void;

	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations
	#[inline]
	fn get_fixed_point_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_VariationalRefinement_getFixedPointIterations_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations
	#[inline]
	fn get_sor_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_VariationalRefinement_getSorIterations_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega
	#[inline]
	fn get_omega(&self) -> Result<f32> {
		unsafe { sys::cv_VariationalRefinement_getOmega_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha
	#[inline]
	fn get_alpha(&self) -> Result<f32> {
		unsafe { sys::cv_VariationalRefinement_getAlpha_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setDelta
	#[inline]
	fn get_delta(&self) -> Result<f32> {
		unsafe { sys::cv_VariationalRefinement_getDelta_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma
	#[inline]
	fn get_gamma(&self) -> Result<f32> {
		unsafe { sys::cv_VariationalRefinement_getGamma_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
}

pub trait VariationalRefinement: crate::video::DenseOpticalFlow + crate::video::VariationalRefinementConst {
	fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void;

	/// @ref calc function overload to handle separate horizontal (u) and vertical (v) flow components
	/// (to avoid extra splits/merges)
	#[inline]
	fn calc_uv(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow_u: &mut dyn core::ToInputOutputArray, flow_v: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow_u);
		input_output_array_arg!(flow_v);
		unsafe { sys::cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_VariationalRefinement(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow_u.as_raw__InputOutputArray(), flow_v.as_raw__InputOutputArray()) }.into_result()
	}
	
	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations getFixedPointIterations
	#[inline]
	fn set_fixed_point_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setFixedPointIterations_int(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations getSorIterations
	#[inline]
	fn set_sor_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setSorIterations_int(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega getOmega
	#[inline]
	fn set_omega(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setOmega_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha getAlpha
	#[inline]
	fn set_alpha(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setAlpha_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setDelta getDelta
	#[inline]
	fn set_delta(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setDelta_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma getGamma
	#[inline]
	fn set_gamma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_VariationalRefinement_setGamma_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
}

impl dyn VariationalRefinement + '_ {
	/// Creates an instance of VariationalRefinement
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::video::VariationalRefinement>> {
		unsafe { sys::cv_VariationalRefinement_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::VariationalRefinement>::opencv_from_extern(r) } )
	}
	
}