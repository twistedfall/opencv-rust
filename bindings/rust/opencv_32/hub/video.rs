#![allow(unused_parens)]
//! # Video Analysis
//!   # Motion Analysis
//!   # Object Tracking
//!   # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::KalmanFilterTrait, super::DenseOpticalFlow, super::SparseOpticalFlow, super::DualTVL1OpticalFlow, super::FarnebackOpticalFlow, super::SparsePyrLKOpticalFlow, super::BackgroundSubtractor, super::BackgroundSubtractorMOG2, super::BackgroundSubtractorKNN };
}

pub const CV_LKFLOW_GET_MIN_EIGENVALS: i32 = 8;
pub const CV_LKFLOW_INITIAL_GUESSES: i32 = 4;
pub const CV_LKFLOW_PYR_A_READY: i32 = 1;
pub const CV_LKFLOW_PYR_B_READY: i32 = 2;
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
/// The function implements the CAMSHIFT object tracking algorithm [Bradski98](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Bradski98) . First, it finds an
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
pub fn cam_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<core::RotatedRect> {
	input_array_arg!(prob_image);
	unsafe { sys::cv_CamShift_const__InputArrayX_RectX_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.as_raw_TermCriteria()) }.into_result().map(|ptr| core::RotatedRect { ptr })
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
pub fn build_optical_flow_pyramid(img: &dyn core::ToInputArray, pyramid: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> Result<i32> {
	input_array_arg!(img);
	output_array_arg!(pyramid);
	unsafe { sys::cv_buildOpticalFlowPyramid_const__InputArrayX_const__OutputArrayX_Size_int_bool_int_int_bool(img.as_raw__InputArray(), pyramid.as_raw__OutputArray(), &win_size, max_level, with_derivatives, pyr_border, deriv_border, try_reuse_input_image) }.into_result()
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
/// The function finds an optical flow for each prev pixel using the [Farneback2003](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Farneback2003) algorithm so that
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
pub fn calc_optical_flow_farneback(prev: &dyn core::ToInputArray, next: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<()> {
	input_array_arg!(prev);
	input_array_arg!(next);
	input_output_array_arg!(flow);
	unsafe { sys::cv_calcOpticalFlowFarneback_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_double_int_int_int_int_double_int(prev.as_raw__InputArray(), next.as_raw__InputArray(), flow.as_raw__InputOutputArray(), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags) }.into_result()
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
/// optical flow equations (this matrix is called a spatial gradient matrix in [Bouguet00](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Bouguet00)), divided
/// by number of pixels in a window; if this value is less than minEigThreshold, then a corresponding
/// feature is filtered out and its flow is not processed, so it allows to remove bad points and get a
/// performance boost.
/// 
/// The function implements a sparse iterative version of the Lucas-Kanade optical flow in pyramids. See
/// [Bouguet00](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Bouguet00) . The function is parallelized with the TBB library.
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
pub fn calc_optical_flow_pyr_lk(prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, criteria: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	unsafe { sys::cv_calcOpticalFlowPyrLK_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_Size_int_TermCriteria_int_double(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), &win_size, max_level, criteria.as_raw_TermCriteria(), flags, min_eig_threshold) }.into_result()
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
pub fn create_background_subtractor_knn(history: i32, dist2_threshold: f64, detect_shadows: bool) -> Result<types::PtrOfBackgroundSubtractorKNN> {
	unsafe { sys::cv_createBackgroundSubtractorKNN_int_double_bool(history, dist2_threshold, detect_shadows) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorKNN { ptr })
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
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<types::PtrOfBackgroundSubtractorMOG2> {
	unsafe { sys::cv_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorMOG2 { ptr })
}

/// Creates instance of cv::DenseOpticalFlow
pub fn create_opt_flow_dual_tvl1() -> Result<types::PtrOfDualTVL1OpticalFlow> {
	unsafe { sys::cv_createOptFlow_DualTVL1() }.into_result().map(|ptr| types::PtrOfDualTVL1OpticalFlow { ptr })
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
pub fn estimate_rigid_transform(src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, full_affine: bool) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	unsafe { sys::cv_estimateRigidTransform_const__InputArrayX_const__InputArrayX_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), full_affine) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_EP08) .
/// 
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image which should be warped with the final warpMatrix in
/// order to provide an image similar to templateImage, same type as temlateImage.
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
/// 
/// The function estimates the optimum transformation (warpMatrix) with respect to ECC criterion
/// ([EP08](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_EP08)), that is
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BwarpMatrix%7D%20%3D%20%5Ctexttt%7BwarpMatrix%7D%20%3D%20%5Carg%5Cmax%5F%7BW%7D%20%5Ctexttt%7BECC%7D%28%5Ctexttt%7BtemplateImage%7D%28x%2Cy%29%2C%5Ctexttt%7BinputImage%7D%28x%27%2Cy%27%29%29)
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
/// warp (unity matrix) should be given as input. Note that if images undergo strong
/// displacements/rotations, an initial transformation that roughly aligns the images is necessary
/// (e.g., a simple euclidean/similarity transform that allows for the images showing the same image
/// content approximately). Use inverse warping in the second image to take an image close to the first
/// one, i.e. use the flag WARP_INVERSE_MAP with warpAffine or warpPerspective. See also the OpenCV
/// sample image_alignment.cpp that demonstrates the use of the function. Note that the function throws
/// an exception if algorithm does not converges.
/// ## See also
/// estimateAffine2D, estimateAffinePartial2D, findHomography
/// 
/// ## C++ default parameters
/// * motion_type: MOTION_AFFINE
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,50,0.001)
/// * input_mask: noArray()
pub fn find_transform_ecc(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, warp_matrix: &mut dyn core::ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	unsafe { sys::cv_findTransformECC_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_int_TermCriteria_const__InputArrayX(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, criteria.as_raw_TermCriteria(), input_mask.as_raw__InputArray()) }.into_result()
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
/// 
/// 
/// Note:
/// *   A mean-shift tracking sample can be found at opencv_source_code/samples/cpp/camshiftdemo.cpp
pub fn mean_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<i32> {
	input_array_arg!(prob_image);
	unsafe { sys::cv_meanShift_const__InputArrayX_RectX_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Base class for background/foreground segmentation. :
/// 
/// The class is only used to define the common interface for the whole family of background/foreground
/// segmentation algorithms.
pub trait BackgroundSubtractor: core::AlgorithmTrait {
	fn as_raw_BackgroundSubtractor(&self) -> *mut c_void;
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
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		unsafe { sys::cv_BackgroundSubtractor_apply_const__InputArrayX_const__OutputArrayX_double(self.as_raw_BackgroundSubtractor(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
	}
	
	/// Computes a background image.
	/// 
	/// ## Parameters
	/// * backgroundImage: The output background image.
	/// 
	/// 
	/// Note: Sometimes the background image can be very blurry, as it contain the average background
	/// statistics.
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(background_image);
		unsafe { sys::cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayX(self.as_raw_BackgroundSubtractor(), background_image.as_raw__OutputArray()) }.into_result()
	}
	
}

/// K-nearest neigbours - based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the K-nearest neigbours background subtraction described in [Zivkovic2006](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
/// Very efficient if number of foreground pixels is low.
pub trait BackgroundSubtractorKNN: crate::video::BackgroundSubtractor {
	fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void;
	/// Returns the number of last frames that affect the background model
	fn get_history(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getHistory_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the number of last frames that affect the background model
	fn set_history(&mut self, history: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setHistory_int(self.as_raw_BackgroundSubtractorKNN(), history) }.into_result()
	}
	
	/// Returns the number of data samples in the background model
	fn get_n_samples(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getNSamples_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the number of data samples in the background model.
	/// 
	/// The model needs to be reinitalized to reserve memory.
	fn set_n_samples(&mut self, _n_n: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setNSamples_int(self.as_raw_BackgroundSubtractorKNN(), _n_n) }.into_result()
	}
	
	/// Returns the threshold on the squared distance between the pixel and the sample
	/// 
	/// The threshold on the squared distance between the pixel and the sample to decide whether a pixel is
	/// close to a data sample.
	fn get_dist2_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getDist2Threshold_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the threshold on the squared distance
	fn set_dist2_threshold(&mut self, _dist2_threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setDist2Threshold_double(self.as_raw_BackgroundSubtractorKNN(), _dist2_threshold) }.into_result()
	}
	
	/// Returns the number of neighbours, the k in the kNN.
	/// 
	/// K is the number of samples that need to be within dist2Threshold in order to decide that that
	/// pixel is matching the kNN background model.
	fn getk_nn_samples(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getkNNSamples_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the k in the kNN. How many nearest neigbours need to match.
	fn setk_nn_samples(&mut self, _nk_nn: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setkNNSamples_int(self.as_raw_BackgroundSubtractorKNN(), _nk_nn) }.into_result()
	}
	
	/// Returns the shadow detection flag
	/// 
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorKNN for
	/// details.
	fn get_detect_shadows(&self) -> Result<bool> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getDetectShadows_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Enables or disables shadow detection
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setDetectShadows_bool(self.as_raw_BackgroundSubtractorKNN(), detect_shadows) }.into_result()
	}
	
	/// Returns the shadow value
	/// 
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	fn get_shadow_value(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowValue_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the shadow value
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowValue_int(self.as_raw_BackgroundSubtractorKNN(), value) }.into_result()
	}
	
	/// Returns the shadow threshold
	/// 
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiarra,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	fn get_shadow_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowThreshold_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
	}
	
	/// Sets the shadow threshold
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowThreshold_double(self.as_raw_BackgroundSubtractorKNN(), threshold) }.into_result()
	}
	
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the Gaussian mixture model background subtraction described in [Zivkovic2004](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Zivkovic2004)
/// and [Zivkovic2006](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
pub trait BackgroundSubtractorMOG2: crate::video::BackgroundSubtractor {
	fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void;
	/// Returns the number of last frames that affect the background model
	fn get_history(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getHistory_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the number of last frames that affect the background model
	fn set_history(&mut self, history: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setHistory_int(self.as_raw_BackgroundSubtractorMOG2(), history) }.into_result()
	}
	
	/// Returns the number of gaussian components in the background model
	fn get_n_mixtures(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getNMixtures_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the number of gaussian components in the background model.
	/// 
	/// The model needs to be reinitalized to reserve memory.
	fn set_n_mixtures(&mut self, nmixtures: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setNMixtures_int(self.as_raw_BackgroundSubtractorMOG2(), nmixtures) }.into_result()
	}
	
	/// Returns the "background ratio" parameter of the algorithm
	/// 
	/// If a foreground pixel keeps semi-constant value for about backgroundRatio\*history frames, it's
	/// considered background and added to the model as a center of a new component. It corresponds to TB
	/// parameter in the paper.
	fn get_background_ratio(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the "background ratio" parameter of the algorithm
	fn set_background_ratio(&mut self, ratio: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(self.as_raw_BackgroundSubtractorMOG2(), ratio) }.into_result()
	}
	
	/// Returns the variance threshold for the pixel-model match
	/// 
	/// The main threshold on the squared Mahalanobis distance to decide if the sample is well described by
	/// the background model or not. Related to Cthr from the paper.
	fn get_var_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the variance threshold for the pixel-model match
	fn set_var_threshold(&mut self, var_threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThreshold_double(self.as_raw_BackgroundSubtractorMOG2(), var_threshold) }.into_result()
	}
	
	/// Returns the variance threshold for the pixel-model match used for new mixture component generation
	/// 
	/// Threshold for the squared Mahalanobis distance that helps decide when a sample is close to the
	/// existing components (corresponds to Tg in the paper). If a pixel is not close to any component, it
	/// is considered foreground or added as a new component. 3 sigma =\> Tg=3\*3=9 is default. A smaller Tg
	/// value generates more components. A higher Tg value may result in a small number of components but
	/// they can grow too large.
	fn get_var_threshold_gen(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the variance threshold for the pixel-model match used for new mixture component generation
	fn set_var_threshold_gen(&mut self, var_threshold_gen: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(self.as_raw_BackgroundSubtractorMOG2(), var_threshold_gen) }.into_result()
	}
	
	/// Returns the initial variance of each gaussian component
	fn get_var_init(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarInit_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the initial variance of each gaussian component
	fn set_var_init(&mut self, var_init: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarInit_double(self.as_raw_BackgroundSubtractorMOG2(), var_init) }.into_result()
	}
	
	fn get_var_min(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMin_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	fn set_var_min(&mut self, var_min: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMin_double(self.as_raw_BackgroundSubtractorMOG2(), var_min) }.into_result()
	}
	
	fn get_var_max(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMax_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	fn set_var_max(&mut self, var_max: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMax_double(self.as_raw_BackgroundSubtractorMOG2(), var_max) }.into_result()
	}
	
	/// Returns the complexity reduction threshold
	/// 
	/// This parameter defines the number of samples needed to accept to prove the component exists. CT=0.05
	/// is a default value for all the samples. By setting CT=0 you get an algorithm very similar to the
	/// standard Stauffer&Grimson algorithm.
	fn get_complexity_reduction_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the complexity reduction threshold
	fn set_complexity_reduction_threshold(&mut self, ct: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(self.as_raw_BackgroundSubtractorMOG2(), ct) }.into_result()
	}
	
	/// Returns the shadow detection flag
	/// 
	/// If true, the algorithm detects shadows and marks them. See createBackgroundSubtractorMOG2 for
	/// details.
	fn get_detect_shadows(&self) -> Result<bool> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getDetectShadows_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Enables or disables shadow detection
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setDetectShadows_bool(self.as_raw_BackgroundSubtractorMOG2(), detect_shadows) }.into_result()
	}
	
	/// Returns the shadow value
	/// 
	/// Shadow value is the value used to mark shadows in the foreground mask. Default value is 127. Value 0
	/// in the mask always means background, 255 means foreground.
	fn get_shadow_value(&self) -> Result<i32> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowValue_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the shadow value
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowValue_int(self.as_raw_BackgroundSubtractorMOG2(), value) }.into_result()
	}
	
	/// Returns the shadow threshold
	/// 
	/// A shadow is detected if pixel is a darker version of the background. The shadow threshold (Tau in
	/// the paper) is a threshold defining how much darker the shadow can be. Tau= 0.5 means that if a pixel
	/// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiarra,
	/// *Detecting Moving Shadows...*, IEEE PAMI,2003.
	fn get_shadow_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
	}
	
	/// Sets the shadow threshold
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowThreshold_double(self.as_raw_BackgroundSubtractorMOG2(), threshold) }.into_result()
	}
	
}

pub trait DenseOpticalFlow: core::AlgorithmTrait {
	fn as_raw_DenseOpticalFlow(&self) -> *mut c_void;
	/// Calculates an optical flow.
	/// 
	/// ## Parameters
	/// * I0: first 8-bit single-channel input image.
	/// * I1: second input image of the same size and the same type as prev.
	/// * flow: computed flow image that has the same size as prev and type CV_32FC2.
	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		unsafe { sys::cv_DenseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(self.as_raw_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray()) }.into_result()
	}
	
	/// Releases all inner buffers.
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_DenseOpticalFlow_collectGarbage(self.as_raw_DenseOpticalFlow()) }.into_result()
	}
	
}

/// "Dual TV L1" Optical Flow Algorithm.
/// 
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Javier2012) .
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
pub trait DualTVL1OpticalFlow: crate::video::DenseOpticalFlow {
	fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void;
	/// Time step of the numerical scheme
	/// ## See also
	/// setTau
	fn get_tau(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Time step of the numerical scheme
	/// ## See also
	/// setTau getTau
	fn set_tau(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setTau_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda
	fn get_lambda(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda getLambda
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setLambda_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta
	fn get_theta(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta getTheta
	fn set_theta(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setTheta_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma
	fn get_gamma(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma getGamma
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setGamma_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber
	fn get_scales_number(&self) -> Result<i32> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber getScalesNumber
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber
	fn get_warpings_number(&self) -> Result<i32> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon
	fn get_epsilon(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon getEpsilon
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations
	fn get_inner_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations getInnerIterations
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations
	fn get_outer_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations getOuterIterations
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep
	fn get_scale_step(&self) -> Result<f64> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep getScaleStep
	fn set_scale_step(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering
	fn get_median_filtering(&self) -> Result<i32> {
		unsafe { sys::cv_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering getMedianFiltering
	fn set_median_filtering(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
}

impl dyn DualTVL1OpticalFlow + '_ {
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
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> Result<types::PtrOfDualTVL1OpticalFlow> {
		unsafe { sys::cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau, lambda, theta, nscales, warps, epsilon, innner_iterations, outer_iterations, scale_step, gamma, median_filtering, use_initial_flow) }.into_result().map(|ptr| types::PtrOfDualTVL1OpticalFlow { ptr })
	}
	
}
/// Class computing a dense optical flow using the Gunnar Farneback’s algorithm.
pub trait FarnebackOpticalFlow: crate::video::DenseOpticalFlow {
	fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void;
	fn get_num_levels(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_FarnebackOpticalFlow(), num_levels) }.into_result()
	}
	
	fn get_pyr_scale(&self) -> Result<f64> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_FarnebackOpticalFlow(), pyr_scale) }.into_result()
	}
	
	fn get_fast_pyramids(&self) -> Result<bool> {
		unsafe { sys::cv_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_FarnebackOpticalFlow(), fast_pyramids) }.into_result()
	}
	
	fn get_win_size(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getWinSize_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_win_size(&mut self, win_size: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setWinSize_int(self.as_raw_FarnebackOpticalFlow(), win_size) }.into_result()
	}
	
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getNumIters_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setNumIters_int(self.as_raw_FarnebackOpticalFlow(), num_iters) }.into_result()
	}
	
	fn get_poly_n(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPolyN_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPolyN_int(self.as_raw_FarnebackOpticalFlow(), poly_n) }.into_result()
	}
	
	fn get_poly_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_FarnebackOpticalFlow(), poly_sigma) }.into_result()
	}
	
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv_FarnebackOpticalFlow_getFlags_const(self.as_raw_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		unsafe { sys::cv_FarnebackOpticalFlow_setFlags_int(self.as_raw_FarnebackOpticalFlow(), flags) }.into_result()
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
	pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<types::PtrOfFarnebackOpticalFlow> {
		unsafe { sys::cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags) }.into_result().map(|ptr| types::PtrOfFarnebackOpticalFlow { ptr })
	}
	
}
/// Kalman filter class.
/// 
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality. See the OpenCV sample kalman.cpp.
/// 
/// 
/// Note:
/// 
/// *   An example using the standard Kalman filter can be found at
///    opencv_source_code/samples/cpp/kalman.cpp
pub trait KalmanFilterTrait {
	fn as_raw_KalmanFilter(&self) -> *mut c_void;
	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	fn state_pre(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_statePre(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: state_pre")
	}
	
	/// predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
	fn set_state_pre(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setStatePre_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_state_pre")
	}
	
	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	fn state_post(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_statePost(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: state_post")
	}
	
	/// corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
	fn set_state_post(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setStatePost_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_state_post")
	}
	
	/// state transition matrix (A)
	fn transition_matrix(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_transitionMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: transition_matrix")
	}
	
	/// state transition matrix (A)
	fn set_transition_matrix(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTransitionMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_transition_matrix")
	}
	
	/// control matrix (B) (not used if there is no control)
	fn control_matrix(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_controlMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: control_matrix")
	}
	
	/// control matrix (B) (not used if there is no control)
	fn set_control_matrix(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setControlMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_control_matrix")
	}
	
	/// measurement matrix (H)
	fn measurement_matrix(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_measurementMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: measurement_matrix")
	}
	
	/// measurement matrix (H)
	fn set_measurement_matrix(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setMeasurementMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_measurement_matrix")
	}
	
	/// process noise covariance matrix (Q)
	fn process_noise_cov(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_processNoiseCov(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: process_noise_cov")
	}
	
	/// process noise covariance matrix (Q)
	fn set_process_noise_cov(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setProcessNoiseCov_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_process_noise_cov")
	}
	
	/// measurement noise covariance matrix (R)
	fn measurement_noise_cov(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_measurementNoiseCov(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: measurement_noise_cov")
	}
	
	/// measurement noise covariance matrix (R)
	fn set_measurement_noise_cov(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setMeasurementNoiseCov_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_measurement_noise_cov")
	}
	
	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	fn error_cov_pre(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_errorCovPre(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: error_cov_pre")
	}
	
	/// priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)
	fn set_error_cov_pre(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setErrorCovPre_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_error_cov_pre")
	}
	
	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	fn gain(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_gain(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: gain")
	}
	
	/// Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
	fn set_gain(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setGain_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_gain")
	}
	
	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	fn error_cov_post(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_errorCovPost(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: error_cov_post")
	}
	
	/// posteriori error estimate covariance matrix (P(k)): P(k)=(I-K(k)*H)*P'(k)
	fn set_error_cov_post(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setErrorCovPost_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_error_cov_post")
	}
	
	fn temp1(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_temp1(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: temp1")
	}
	
	fn set_temp1(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTemp1_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_temp1")
	}
	
	fn temp2(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_temp2(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: temp2")
	}
	
	fn set_temp2(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTemp2_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_temp2")
	}
	
	fn temp3(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_temp3(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: temp3")
	}
	
	fn set_temp3(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTemp3_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_temp3")
	}
	
	fn temp4(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_temp4(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: temp4")
	}
	
	fn set_temp4(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTemp4_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_temp4")
	}
	
	fn temp5(&mut self) -> core::Mat {
		unsafe { sys::cv_KalmanFilter_temp5(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: temp5")
	}
	
	fn set_temp5(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_KalmanFilter_setTemp5_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_temp5")
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
	fn init(&mut self, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<()> {
		unsafe { sys::cv_KalmanFilter_init_int_int_int_int(self.as_raw_KalmanFilter(), dynam_params, measure_params, control_params, typ) }.into_result()
	}
	
	/// Computes a predicted state.
	/// 
	/// ## Parameters
	/// * control: The optional input control
	/// 
	/// ## C++ default parameters
	/// * control: Mat()
	fn predict(&mut self, control: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_KalmanFilter_predict_const_MatX(self.as_raw_KalmanFilter(), control.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// Updates the predicted state from the measurement.
	/// 
	/// ## Parameters
	/// * measurement: The measured system parameters
	fn correct(&mut self, measurement: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_KalmanFilter_correct_const_MatX(self.as_raw_KalmanFilter(), measurement.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
}

/// Kalman filter class.
/// 
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality. See the OpenCV sample kalman.cpp.
/// 
/// 
/// Note:
/// 
/// *   An example using the standard Kalman filter can be found at
///    opencv_source_code/samples/cpp/kalman.cpp
pub struct KalmanFilter {
	pub(crate) ptr: *mut c_void
}

impl Drop for KalmanFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_KalmanFilter_delete(instance: *mut c_void); }
		unsafe { cv_KalmanFilter_delete(self.as_raw_KalmanFilter()) };
	}
}

impl KalmanFilter {
	pub fn as_raw_KalmanFilter(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for KalmanFilter {}

impl crate::video::KalmanFilterTrait for KalmanFilter {
	fn as_raw_KalmanFilter(&self) -> *mut c_void { self.ptr }
}

impl KalmanFilter {
	/// The constructors.
	/// 
	/// 
	/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
	/// with cvReleaseKalman(&kalmanFilter)
	pub fn default() -> Result<crate::video::KalmanFilter> {
		unsafe { sys::cv_KalmanFilter_KalmanFilter() }.into_result().map(|ptr| crate::video::KalmanFilter { ptr })
	}
	
	/// The constructors.
	/// 
	/// 
	/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
	/// with cvReleaseKalman(&kalmanFilter)
	/// 
	/// ## Overloaded parameters
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
	pub fn new(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<crate::video::KalmanFilter> {
		unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params, measure_params, control_params, typ) }.into_result().map(|ptr| crate::video::KalmanFilter { ptr })
	}
	
}

/// Base interface for sparse optical flow algorithms.
pub trait SparseOpticalFlow: core::AlgorithmTrait {
	fn as_raw_SparseOpticalFlow(&self) -> *mut c_void;
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
	fn calc(&mut self, prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		output_array_arg!(err);
		unsafe { sys::cv_SparseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(self.as_raw_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Class used for calculating a sparse optical flow.
/// 
/// The class can calculate an optical flow for a sparse feature set using the
/// iterative Lucas-Kanade method with pyramids.
/// ## See also
/// calcOpticalFlowPyrLK
pub trait SparsePyrLKOpticalFlow: crate::video::SparseOpticalFlow {
	fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void;
	fn get_win_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_SparsePyrLKOpticalFlow(), &win_size) }.into_result()
	}
	
	fn get_max_level(&self) -> Result<i32> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_SparsePyrLKOpticalFlow(), max_level) }.into_result()
	}
	
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getTermCriteria_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result().map(|ptr| core::TermCriteria { ptr })
	}
	
	fn set_term_criteria(&mut self, crit: &mut core::TermCriteria) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaX(self.as_raw_SparsePyrLKOpticalFlow(), crit.as_raw_TermCriteria()) }.into_result()
	}
	
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getFlags_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setFlags_int(self.as_raw_SparsePyrLKOpticalFlow(), flags) }.into_result()
	}
	
	fn get_min_eig_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_min_eig_threshold(&mut self, min_eig_threshold: f64) -> Result<()> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(self.as_raw_SparsePyrLKOpticalFlow(), min_eig_threshold) }.into_result()
	}
	
}

impl dyn SparsePyrLKOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
	/// * flags: 0
	/// * min_eig_threshold: 1e-4
	pub fn create(win_size: core::Size, max_level: i32, crit: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<types::PtrOfSparsePyrLKOpticalFlow> {
		unsafe { sys::cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(&win_size, max_level, crit.as_raw_TermCriteria(), flags, min_eig_threshold) }.into_result().map(|ptr| types::PtrOfSparsePyrLKOpticalFlow { ptr })
	}
	
}