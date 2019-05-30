//! # Video Analysis
//! # Motion Analysis
//! # Object Tracking
//! # C API
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

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
/// The function implements the CAMSHIFT object tracking algorithm [Bradski98](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Bradski98) . First, it finds an
/// object center using meanShift and then adjusts the window size and finds the optimal rotation. The
/// function returns the rotated rectangle structure that includes the object position, size, and
/// orientation. The next position of the search window can be obtained with RotatedRect::boundingRect()
/// 
/// See the OpenCV sample camshiftdemo.c that tracks colored objects.
/// 
/// 
/// Note:
/// *   (Python) A sample explaining the camshift tracking algorithm can be found at
/// opencv_source_code/samples/python/camshift.py
pub fn cam_shift(prob_image: &core::Mat, window: &mut core::Rect, criteria: &core::TermCriteria) -> Result<core::RotatedRect> {
    unsafe { sys::cv_CamShift_Mat_Rect_TermCriteria(prob_image.as_raw_Mat(), window, criteria.as_raw_TermCriteria()) }.into_result().map(|ptr| core::RotatedRect { ptr })
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
pub fn build_optical_flow_pyramid(img: &core::Mat, pyramid: &mut types::VectorOfMat, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> Result<i32> {
    unsafe { sys::cv_buildOpticalFlowPyramid_Mat_VectorOfMat_Size_int_bool_int_int_bool(img.as_raw_Mat(), pyramid.as_raw_VectorOfMat(), win_size, max_level, with_derivatives, pyr_border, deriv_border, try_reuse_input_image) }.into_result()
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
/// *   **OPTFLOW_USE_INITIAL_FLOW** uses the input flow as an initial flow approximation.
/// *   **OPTFLOW_FARNEBACK_GAUSSIAN** uses the Gaussian <span lang='latex'>\texttt{winsize}\times\texttt{winsize}</span>
/// filter instead of a box filter of the same size for optical flow estimation; usually, this
/// option gives z more accurate flow than with a box filter, at the cost of lower speed;
/// normally, winsize for a Gaussian window should be set to a larger value to achieve the same
/// level of robustness.
/// 
/// The function finds an optical flow for each prev pixel using the [Farneback2003](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Farneback2003) algorithm so that
/// 
/// <div lang='latex'>\texttt{prev} (y,x)  \sim \texttt{next} ( y + \texttt{flow} (y,x)[1],  x + \texttt{flow} (y,x)[0])</div>
/// 
/// 
/// Note:
/// 
/// *   An example using the optical flow algorithm described by Gunnar Farneback can be found at
/// opencv_source_code/samples/cpp/fback.cpp
/// *   (Python) An example using the optical flow algorithm described by Gunnar Farneback can be
/// found at opencv_source_code/samples/python/opt_flow.py
pub fn calc_optical_flow_farneback(prev: &core::Mat, next: &core::Mat, flow: &mut core::Mat, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<()> {
    unsafe { sys::cv_calcOpticalFlowFarneback_Mat_Mat_Mat_double_int_int_int_int_double_int(prev.as_raw_Mat(), next.as_raw_Mat(), flow.as_raw_Mat(), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags) }.into_result()
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
/// *   **OPTFLOW_USE_INITIAL_FLOW** uses initial estimations, stored in nextPts; if the flag is
/// not set, then prevPts is copied to nextPts and is considered the initial estimate.
/// *   **OPTFLOW_LK_GET_MIN_EIGENVALS** use minimum eigen values as an error measure (see
/// minEigThreshold description); if the flag is not set, then L1 distance between patches
/// around the original and a moved point, divided by number of pixels in a window, is used as a
/// error measure.
/// * minEigThreshold: the algorithm calculates the minimum eigen value of a 2x2 normal matrix of
/// optical flow equations (this matrix is called a spatial gradient matrix in [Bouguet00](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Bouguet00)), divided
/// by number of pixels in a window; if this value is less than minEigThreshold, then a corresponding
/// feature is filtered out and its flow is not processed, so it allows to remove bad points and get a
/// performance boost.
/// 
/// The function implements a sparse iterative version of the Lucas-Kanade optical flow in pyramids. See
/// [Bouguet00](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Bouguet00) . The function is parallelized with the TBB library.
/// 
/// 
/// Note:
/// 
/// *   An example using the Lucas-Kanade optical flow algorithm can be found at
/// opencv_source_code/samples/cpp/lkdemo.cpp
/// *   (Python) An example using the Lucas-Kanade optical flow algorithm can be found at
/// opencv_source_code/samples/python/lk_track.py
/// *   (Python) An example using the Lucas-Kanade tracker for homography matching can be found at
/// opencv_source_code/samples/python/lk_homography.py
///
/// ## C++ default parameters
/// * win_size: Size(21,21)
/// * max_level: 3
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS, 30, 0.01)
/// * flags: 0
/// * min_eig_threshold: 1e-4
pub fn calc_optical_flow_pyr_lk(prev_img: &core::Mat, next_img: &core::Mat, prev_pts: &core::Mat, next_pts: &mut core::Mat, status: &mut core::Mat, err: &mut core::Mat, win_size: core::Size, max_level: i32, criteria: &core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<()> {
    unsafe { sys::cv_calcOpticalFlowPyrLK_Mat_Mat_Mat_Mat_Mat_Mat_Size_int_TermCriteria_int_double(prev_img.as_raw_Mat(), next_img.as_raw_Mat(), prev_pts.as_raw_Mat(), next_pts.as_raw_Mat(), status.as_raw_Mat(), err.as_raw_Mat(), win_size, max_level, criteria.as_raw_TermCriteria(), flags, min_eig_threshold) }.into_result()
}

/// Computes the Enhanced Correlation Coefficient value between two images [EP08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EP08) .
/// 
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image to be warped to provide an image similar to
/// templateImage, same type as templateImage.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// 
/// ## See also
/// findTransformECC
///
/// ## C++ default parameters
/// * input_mask: noArray()
pub fn compute_ecc(template_image: &core::Mat, input_image: &core::Mat, input_mask: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_computeECC_Mat_Mat_Mat(template_image.as_raw_Mat(), input_image.as_raw_Mat(), input_mask.as_raw_Mat()) }.into_result()
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
pub fn create_opt_flow__dual_tvl1() -> Result<types::PtrOfDualTVL1OpticalFlow> {
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
///   Two point sets
///   Two raster images. In this case, the function first finds some features in the src image and
/// finds the corresponding features in dst image. After that, the problem is reduced to the first
/// case.
/// In case of point sets, the problem is formulated as follows: you need to find a 2x2 matrix *A* and
/// 2x1 vector *b* so that:
/// 
/// <div lang='latex'>[A^*|b^*] = arg  \min _{[A|b]}  \sum _i  \| \texttt{dst}[i] - A { \texttt{src}[i]}^T - b  \| ^2</div>
/// where src[i] and dst[i] are the i-th points in src and dst, respectively
/// <span lang='latex'>[A|b]</span> can be either arbitrary (when fullAffine=true ) or have a form of
/// <div lang='latex'>\begin{bmatrix} a_{11} & a_{12} & b_1  \\ -a_{12} & a_{11} & b_2  \end{bmatrix}</div>
/// when fullAffine=false.
/// 
/// ## See also
/// estimateAffine2D, estimateAffinePartial2D, getAffineTransform, getPerspectiveTransform, findHomography
pub fn estimate_rigid_transform(src: &core::Mat, dst: &core::Mat, full_affine: bool) -> Result<core::Mat> {
    unsafe { sys::cv_estimateRigidTransform_Mat_Mat_bool(src.as_raw_Mat(), dst.as_raw_Mat(), full_affine) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn estimate_rigid_transform_1(src: &core::Mat, dst: &core::Mat, full_affine: bool, ransac_max_iters: i32, ransac_good_ratio: f64, ransac_size0: i32) -> Result<core::Mat> {
    unsafe { sys::cv_estimateRigidTransform_Mat_Mat_bool_int_double_int(src.as_raw_Mat(), dst.as_raw_Mat(), full_affine, ransac_max_iters, ransac_good_ratio, ransac_size0) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EP08) .
/// 
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image which should be warped with the final warpMatrix in
/// order to provide an image similar to templateImage, same type as templateImage.
/// * warpMatrix: floating-point <span lang='latex'>2\times 3</span> or <span lang='latex'>3\times 3</span> mapping matrix (warp).
/// * motionType: parameter, specifying the type of motion:
/// *   **MOTION_TRANSLATION** sets a translational motion model; warpMatrix is <span lang='latex'>2\times 3</span> with
/// the first <span lang='latex'>2\times 2</span> part being the unity matrix and the rest two parameters being
/// estimated.
/// *   **MOTION_EUCLIDEAN** sets a Euclidean (rigid) transformation as motion model; three
/// parameters are estimated; warpMatrix is <span lang='latex'>2\times 3</span>.
/// *   **MOTION_AFFINE** sets an affine motion model (DEFAULT); six parameters are estimated;
/// warpMatrix is <span lang='latex'>2\times 3</span>.
/// *   **MOTION_HOMOGRAPHY** sets a homography as a motion model; eight parameters are
/// estimated;\`warpMatrix\` is <span lang='latex'>3\times 3</span>.
/// * criteria: parameter, specifying the termination criteria of the ECC algorithm;
/// criteria.epsilon defines the threshold of the increment in the correlation coefficient between two
/// iterations (a negative criteria.epsilon makes criteria.maxcount the only termination criterion).
/// Default values are shown in the declaration above.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// * gaussFiltSize: An optional value indicating size of gaussian blur filter; (DEFAULT: 5)
/// 
/// The function estimates the optimum transformation (warpMatrix) with respect to ECC criterion
/// ([EP08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EP08)), that is
/// 
/// <div lang='latex'>\texttt{warpMatrix} = \texttt{warpMatrix} = \arg\max_{W} \texttt{ECC}(\texttt{templateImage}(x,y),\texttt{inputImage}(x',y'))</div>
/// 
/// where
/// 
/// <div lang='latex'>\begin{bmatrix} x' \\ y' \end{bmatrix} = W \cdot \begin{bmatrix} x \\ y \\ 1 \end{bmatrix}</div>
/// 
/// (the equation holds with homogeneous coordinates for homography). It returns the final enhanced
/// correlation coefficient, that is the correlation coefficient between the template image and the
/// final warped input image. When a <span lang='latex'>3\times 3</span> matrix is given with motionType =0, 1 or 2, the third
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
/// 
/// ## See also
/// computeECC, estimateAffine2D, estimateAffinePartial2D, findHomography
/// 
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * motion_type: MOTION_AFFINE
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS, 50, 0.001)
/// * input_mask: noArray()
pub fn find_transform_ecc(template_image: &core::Mat, input_image: &core::Mat, warp_matrix: &mut core::Mat, motion_type: i32, criteria: &core::TermCriteria, input_mask: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_findTransformECC_Mat_Mat_Mat_int_TermCriteria_Mat(template_image.as_raw_Mat(), input_image.as_raw_Mat(), warp_matrix.as_raw_Mat(), motion_type, criteria.as_raw_TermCriteria(), input_mask.as_raw_Mat()) }.into_result()
}

/// Finds the geometric transform (warp) between two images in terms of the ECC criterion [EP08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EP08) .
/// 
/// ## Parameters
/// * templateImage: single-channel template image; CV_8U or CV_32F array.
/// * inputImage: single-channel input image which should be warped with the final warpMatrix in
/// order to provide an image similar to templateImage, same type as templateImage.
/// * warpMatrix: floating-point <span lang='latex'>2\times 3</span> or <span lang='latex'>3\times 3</span> mapping matrix (warp).
/// * motionType: parameter, specifying the type of motion:
/// *   **MOTION_TRANSLATION** sets a translational motion model; warpMatrix is <span lang='latex'>2\times 3</span> with
/// the first <span lang='latex'>2\times 2</span> part being the unity matrix and the rest two parameters being
/// estimated.
/// *   **MOTION_EUCLIDEAN** sets a Euclidean (rigid) transformation as motion model; three
/// parameters are estimated; warpMatrix is <span lang='latex'>2\times 3</span>.
/// *   **MOTION_AFFINE** sets an affine motion model (DEFAULT); six parameters are estimated;
/// warpMatrix is <span lang='latex'>2\times 3</span>.
/// *   **MOTION_HOMOGRAPHY** sets a homography as a motion model; eight parameters are
/// estimated;\`warpMatrix\` is <span lang='latex'>3\times 3</span>.
/// * criteria: parameter, specifying the termination criteria of the ECC algorithm;
/// criteria.epsilon defines the threshold of the increment in the correlation coefficient between two
/// iterations (a negative criteria.epsilon makes criteria.maxcount the only termination criterion).
/// Default values are shown in the declaration above.
/// * inputMask: An optional mask to indicate valid values of inputImage.
/// * gaussFiltSize: An optional value indicating size of gaussian blur filter; (DEFAULT: 5)
/// 
/// The function estimates the optimum transformation (warpMatrix) with respect to ECC criterion
/// ([EP08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EP08)), that is
/// 
/// <div lang='latex'>\texttt{warpMatrix} = \texttt{warpMatrix} = \arg\max_{W} \texttt{ECC}(\texttt{templateImage}(x,y),\texttt{inputImage}(x',y'))</div>
/// 
/// where
/// 
/// <div lang='latex'>\begin{bmatrix} x' \\ y' \end{bmatrix} = W \cdot \begin{bmatrix} x \\ y \\ 1 \end{bmatrix}</div>
/// 
/// (the equation holds with homogeneous coordinates for homography). It returns the final enhanced
/// correlation coefficient, that is the correlation coefficient between the template image and the
/// final warped input image. When a <span lang='latex'>3\times 3</span> matrix is given with motionType =0, 1 or 2, the third
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
/// 
/// ## See also
/// computeECC, estimateAffine2D, estimateAffinePartial2D, findHomography
pub fn find_transform_ecc_1(template_image: &core::Mat, input_image: &core::Mat, warp_matrix: &mut core::Mat, motion_type: i32, criteria: &core::TermCriteria, input_mask: &core::Mat, gauss_filt_size: i32) -> Result<f64> {
    unsafe { sys::cv_findTransformECC_Mat_Mat_Mat_int_TermCriteria_Mat_int(template_image.as_raw_Mat(), input_image.as_raw_Mat(), warp_matrix.as_raw_Mat(), motion_type, criteria.as_raw_TermCriteria(), input_mask.as_raw_Mat(), gauss_filt_size) }.into_result()
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
pub fn mean_shift(prob_image: &core::Mat, window: &mut core::Rect, criteria: &core::TermCriteria) -> Result<i32> {
    unsafe { sys::cv_meanShift_Mat_Rect_TermCriteria(prob_image.as_raw_Mat(), window, criteria.as_raw_TermCriteria()) }.into_result()
}

// Generating impl for trait cv::BackgroundSubtractor (trait)
/// Base class for background/foreground segmentation. :
/// 
/// The class is only used to define the common interface for the whole family of background/foreground
/// segmentation algorithms.
pub trait BackgroundSubtractor: core::Algorithm {
    #[inline(always)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void;
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
    fn apply(&mut self, image: &core::Mat, fgmask: &mut core::Mat, learning_rate: f64) -> Result<()> {
        unsafe { sys::cv_BackgroundSubtractor_apply_Mat_Mat_double(self.as_raw_BackgroundSubtractor(), image.as_raw_Mat(), fgmask.as_raw_Mat(), learning_rate) }.into_result()
    }
    
    /// Computes a background image.
    /// 
    /// ## Parameters
    /// * backgroundImage: The output background image.
    /// 
    /// 
    /// Note: Sometimes the background image can be very blurry, as it contain the average background
    /// statistics.
    fn get_background_image(&self, background_image: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_BackgroundSubtractor_getBackgroundImage_const_Mat(self.as_raw_BackgroundSubtractor(), background_image.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> BackgroundSubtractor + 'a {

}

// Generating impl for trait cv::BackgroundSubtractorKNN (trait)
/// K-nearest neighbours - based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the K-nearest neighbours background subtraction described in [Zivkovic2006](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
/// Very efficient if number of foreground pixels is low.
pub trait BackgroundSubtractorKNN: crate::video::BackgroundSubtractor {
    #[inline(always)] fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void;
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
    
    /// Sets the k in the kNN. How many nearest neighbours need to match.
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
    /// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
    /// *Detecting Moving Shadows...*, IEEE PAMI,2003.
    fn get_shadow_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_BackgroundSubtractorKNN_getShadowThreshold_const(self.as_raw_BackgroundSubtractorKNN()) }.into_result()
    }
    
    /// Sets the shadow threshold
    fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
        unsafe { sys::cv_BackgroundSubtractorKNN_setShadowThreshold_double(self.as_raw_BackgroundSubtractorKNN(), threshold) }.into_result()
    }
    
}

impl<'a> BackgroundSubtractorKNN + 'a {

}

// Generating impl for trait cv::BackgroundSubtractorMOG2 (trait)
/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the Gaussian mixture model background subtraction described in [Zivkovic2004](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zivkovic2004)
/// and [Zivkovic2006](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zivkovic2006) .
pub trait BackgroundSubtractorMOG2: crate::video::BackgroundSubtractor {
    #[inline(always)] fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void;
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
    /// is more than twice darker then it is not shadow. See Prati, Mikic, Trivedi and Cucchiara,
    /// *Detecting Moving Shadows...*, IEEE PAMI,2003.
    fn get_shadow_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowThreshold_const(self.as_raw_BackgroundSubtractorMOG2()) }.into_result()
    }
    
    /// Sets the shadow threshold
    fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
        unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowThreshold_double(self.as_raw_BackgroundSubtractorMOG2(), threshold) }.into_result()
    }
    
    /// Computes a foreground mask.
    /// 
    /// ## Parameters
    /// * image: Next video frame. Floating point frame will be used without scaling and should be in range <span lang='latex'>[0,255]</span>.
    /// * fgmask: The output foreground mask as an 8-bit binary image.
    /// * learningRate: The value between 0 and 1 that indicates how fast the background model is
    /// learnt. Negative parameter value makes the algorithm to use some automatically chosen learning
    /// rate. 0 means that the background model is not updated at all, 1 means that the background model
    /// is completely reinitialized from the last frame.
    ///
    /// ## C++ default parameters
    /// * learning_rate: -1
    fn apply(&mut self, image: &core::Mat, fgmask: &mut core::Mat, learning_rate: f64) -> Result<()> {
        unsafe { sys::cv_BackgroundSubtractorMOG2_apply_Mat_Mat_double(self.as_raw_BackgroundSubtractorMOG2(), image.as_raw_Mat(), fgmask.as_raw_Mat(), learning_rate) }.into_result()
    }
    
}

impl<'a> BackgroundSubtractorMOG2 + 'a {

}

// Generating impl for trait cv::DenseOpticalFlow (trait)
pub trait DenseOpticalFlow: core::Algorithm {
    #[inline(always)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void;
    /// Calculates an optical flow.
    /// 
    /// ## Parameters
    /// * I0: first 8-bit single-channel input image.
    /// * I1: second input image of the same size and the same type as prev.
    /// * flow: computed flow image that has the same size as prev and type CV_32FC2.
    fn calc(&mut self, i0: &core::Mat, i1: &core::Mat, flow: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_DenseOpticalFlow_calc_Mat_Mat_Mat(self.as_raw_DenseOpticalFlow(), i0.as_raw_Mat(), i1.as_raw_Mat(), flow.as_raw_Mat()) }.into_result()
    }
    
    /// Releases all inner buffers.
    fn collect_garbage(&mut self) -> Result<()> {
        unsafe { sys::cv_DenseOpticalFlow_collectGarbage(self.as_raw_DenseOpticalFlow()) }.into_result()
    }
    
}

impl<'a> DenseOpticalFlow + 'a {

}

// Generating impl for trait cv::DualTVL1OpticalFlow (trait)
/// "Dual TV L1" Optical Flow Algorithm.
/// 
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Javier2012) .
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
/// 
/// *   member double tau
/// Time step of the numerical scheme.
/// 
/// *   member double lambda
/// Weight parameter for the data term, attachment parameter. This is the most relevant
/// parameter, which determines the smoothness of the output. The smaller this parameter is,
/// the smoother the solutions we obtain. It depends on the range of motions of the images, so
/// its value should be adapted to each image sequence.
/// 
/// *   member double theta
/// Weight parameter for (u - v)\^2, tightness parameter. It serves as a link between the
/// attachment and the regularization terms. In theory, it should have a small value in order
/// to maintain both parts in correspondence. The method is stable for a large range of values
/// of this parameter.
/// 
/// *   member int nscales
/// Number of scales used to create the pyramid of images.
/// 
/// *   member int warps
/// Number of warpings per scale. Represents the number of times that I1(x+u0) and grad(
/// I1(x+u0) ) are computed per scale. This is a parameter that assures the stability of the
/// method. It also affects the running time, so it is a compromise between speed and
/// accuracy.
/// 
/// *   member double epsilon
/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between
/// precision and running time. A small value will yield more accurate solutions at the
/// expense of a slower convergence.
/// 
/// *   member int iterations
/// Stopping criterion iterations number used in the numerical scheme.
/// 
/// C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
pub trait DualTVL1OpticalFlow: crate::video::DenseOpticalFlow {
    #[inline(always)] fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void;
    /// @see setTau
    fn get_tau(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getTau @see getTau
    fn set_tau(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setTau_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setLambda
    fn get_lambda(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getLambda @see getLambda
    fn set_lambda(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setLambda_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setTheta
    fn get_theta(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getTheta @see getTheta
    fn set_theta(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setTheta_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setGamma
    fn get_gamma(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getGamma @see getGamma
    fn set_gamma(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setGamma_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setScalesNumber
    fn get_scales_number(&self) -> Result<i32> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getScalesNumber @see getScalesNumber
    fn set_scales_number(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setWarpingsNumber
    fn get_warpings_number(&self) -> Result<i32> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getWarpingsNumber @see getWarpingsNumber
    fn set_warpings_number(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setEpsilon
    fn get_epsilon(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getEpsilon @see getEpsilon
    fn set_epsilon(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setInnerIterations
    fn get_inner_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getInnerIterations @see getInnerIterations
    fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setOuterIterations
    fn get_outer_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getOuterIterations @see getOuterIterations
    fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setUseInitialFlow
    fn get_use_initial_flow(&self) -> Result<bool> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getUseInitialFlow @see getUseInitialFlow
    fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setScaleStep
    fn get_scale_step(&self) -> Result<f64> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getScaleStep @see getScaleStep
    fn set_scale_step(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
    /// @see setMedianFiltering
    fn get_median_filtering(&self) -> Result<i32> {
        unsafe { sys::cv_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
    }
    
    /// @copybrief getMedianFiltering @see getMedianFiltering
    fn set_median_filtering(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_DualTVL1OpticalFlow(), val) }.into_result()
    }
    
}

impl<'a> DualTVL1OpticalFlow + 'a {

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

// Generating impl for trait cv::FarnebackOpticalFlow (trait)
/// Class computing a dense optical flow using the Gunnar Farneback's algorithm.
pub trait FarnebackOpticalFlow: crate::video::DenseOpticalFlow {
    #[inline(always)] fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void;
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

impl<'a> FarnebackOpticalFlow + 'a {

    ///
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

// boxed class cv::KalmanFilter
/// Kalman filter class.
/// 
/// The class implements a standard Kalman filter <http://en.wikipedia.org/wiki/Kalman_filter>,
/// [Welch95](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Welch95) . However, you can modify transitionMatrix, controlMatrix, and measurementMatrix to get
/// an extended Kalman filter functionality.
/// 
/// Note: In C API when CvKalman\* kalmanFilter structure is not needed anymore, it should be released
/// with cvReleaseKalman(&kalmanFilter)
#[allow(dead_code)]
pub struct KalmanFilter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::video::KalmanFilter {
    fn drop(&mut self) {
        unsafe { sys::cv_KalmanFilter_delete(self.ptr) };
    }
}
impl crate::video::KalmanFilter {
    #[inline(always)] pub fn as_raw_KalmanFilter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl KalmanFilter {

    pub fn default() -> Result<crate::video::KalmanFilter> {
        unsafe { sys::cv_KalmanFilter_KalmanFilter() }.into_result().map(|ptr| crate::video::KalmanFilter { ptr })
    }
    
    /// ## Parameters
    /// * dynamParams: Dimensionality of the state.
    /// * measureParams: Dimensionality of the measurement.
    /// * controlParams: Dimensionality of the control vector.
    /// * type: Type of the created matrices that should be CV_32F or CV_64F.
    ///
    /// ## C++ default parameters
    /// * control_params: 0
    /// * _type: CV_32F
    pub fn new(dynam_params: i32, measure_params: i32, control_params: i32, _type: i32) -> Result<crate::video::KalmanFilter> {
        unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params, measure_params, control_params, _type) }.into_result().map(|ptr| crate::video::KalmanFilter { ptr })
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
    /// * _type: CV_32F
    pub fn init(&mut self, dynam_params: i32, measure_params: i32, control_params: i32, _type: i32) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_init_int_int_int_int(self.as_raw_KalmanFilter(), dynam_params, measure_params, control_params, _type) }.into_result()
    }
    
    /// Computes a predicted state.
    /// 
    /// ## Parameters
    /// * control: The optional input control
    ///
    /// ## C++ default parameters
    /// * control: Mat()
    pub fn predict(&mut self, control: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_predict_Mat(self.as_raw_KalmanFilter(), control.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Updates the predicted state from the measurement.
    /// 
    /// ## Parameters
    /// * measurement: The measured system parameters
    pub fn correct(&mut self, measurement: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_correct_Mat(self.as_raw_KalmanFilter(), measurement.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn state_pre(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_statePre(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn set_state_pre(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_statePre_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
    pub fn state_post(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_statePost(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < predicted state (x'(k)): x(k)=A*x(k-1)+B*u(k)
    pub fn set_state_post(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_statePost_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
    pub fn transition_matrix(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_transitionMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < corrected state (x(k)): x(k)=x'(k)+K(k)*(z(k)-H*x'(k))
    pub fn set_transition_matrix(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_transitionMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < state transition matrix (A)
    pub fn control_matrix(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_controlMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < state transition matrix (A)
    pub fn set_control_matrix(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_controlMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < control matrix (B) (not used if there is no control)
    pub fn measurement_matrix(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_measurementMatrix(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < control matrix (B) (not used if there is no control)
    pub fn set_measurement_matrix(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_measurementMatrix_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < measurement matrix (H)
    pub fn process_noise_cov(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_processNoiseCov(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < measurement matrix (H)
    pub fn set_process_noise_cov(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_processNoiseCov_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < process noise covariance matrix (Q)
    pub fn measurement_noise_cov(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_measurementNoiseCov(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < process noise covariance matrix (Q)
    pub fn set_measurement_noise_cov(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_measurementNoiseCov_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < measurement noise covariance matrix (R)
    pub fn error_cov_pre(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_errorCovPre(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < measurement noise covariance matrix (R)
    pub fn set_error_cov_pre(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_errorCovPre_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)*/
    pub fn gain(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_gain(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < priori error estimate covariance matrix (P'(k)): P'(k)=A*P(k-1)*At + Q)*/
    pub fn set_gain(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_gain_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
    /// < Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
    pub fn error_cov_post(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_KalmanFilter_errorCovPost(self.as_raw_KalmanFilter()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// < Kalman gain matrix (K(k)): K(k)=P'(k)*Ht*inv(H*P'(k)*Ht+R)
    pub fn set_error_cov_post(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_KalmanFilter_set_errorCovPost_Mat(self.as_raw_KalmanFilter(), val.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::SparseOpticalFlow (trait)
/// Base interface for sparse optical flow algorithms.
pub trait SparseOpticalFlow: core::Algorithm {
    #[inline(always)] fn as_raw_SparseOpticalFlow(&self) -> *mut c_void;
    /// Calculates a sparse optical flow.
    /// 
    /// ## Parameters
    /// * prevImg: First input image.
    /// * nextImg: Second input image of the same size and the same type as prevImg.
    /// * prevPts: Vector of 2D points for which the flow needs to be found.
    /// * nextPts: Output vector of 2D points containing the calculated new positions of input features in the second image.
    /// * status: Output status vector. Each element of the vector is set to 1 if the
    /// flow for the corresponding features has been found. Otherwise, it is set to 0.
    /// * err: Optional output vector that contains error response for each point (inverse confidence).
    ///
    /// ## C++ default parameters
    /// * err: cv::noArray()
    fn calc(&mut self, prev_img: &core::Mat, next_img: &core::Mat, prev_pts: &core::Mat, next_pts: &mut core::Mat, status: &mut core::Mat, err: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_SparseOpticalFlow_calc_Mat_Mat_Mat_Mat_Mat_Mat(self.as_raw_SparseOpticalFlow(), prev_img.as_raw_Mat(), next_img.as_raw_Mat(), prev_pts.as_raw_Mat(), next_pts.as_raw_Mat(), status.as_raw_Mat(), err.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> SparseOpticalFlow + 'a {

}

// Generating impl for trait cv::SparsePyrLKOpticalFlow (trait)
/// Class used for calculating a sparse optical flow.
/// 
/// The class can calculate an optical flow for a sparse feature set using the
/// iterative Lucas-Kanade method with pyramids.
/// 
/// ## See also
/// calcOpticalFlowPyrLK
pub trait SparsePyrLKOpticalFlow: crate::video::SparseOpticalFlow {
    #[inline(always)] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void;
    fn get_win_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_SparsePyrLKOpticalFlow()) }.into_result()
    }
    
    fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
        unsafe { sys::cv_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_SparsePyrLKOpticalFlow(), win_size) }.into_result()
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
        unsafe { sys::cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteria(self.as_raw_SparsePyrLKOpticalFlow(), crit.as_raw_TermCriteria()) }.into_result()
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

impl<'a> SparsePyrLKOpticalFlow + 'a {

    ///
    /// ## C++ default parameters
    /// * win_size: Size(21, 21)
    /// * max_level: 3
    /// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS, 30, 0.01)
    /// * flags: 0
    /// * min_eig_threshold: 1e-4
    pub fn create(win_size: core::Size, max_level: i32, crit: &core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<types::PtrOfSparsePyrLKOpticalFlow> {
        unsafe { sys::cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size, max_level, crit.as_raw_TermCriteria(), flags, min_eig_threshold) }.into_result().map(|ptr| types::PtrOfSparsePyrLKOpticalFlow { ptr })
    }
    
}

