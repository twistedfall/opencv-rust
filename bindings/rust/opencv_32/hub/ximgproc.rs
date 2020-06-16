#![allow(unused_parens)]
//! # Extended Image Processing
//!    # Structured forests for fast edge detection
//! 
//! This module contains implementations of modern structured edge detection algorithms,
//! i.e. algorithms which somehow takes into account pixel affinities in natural images.
//! 
//!    # Filters
//! 
//!    # Superpixels
//! 
//!    # Image segmentation
//! 
//!    # Fast line detector
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DTFilter, super::GuidedFilter, super::AdaptiveManifoldFilter, super::FastGlobalSmootherFilter, super::DisparityFilter, super::DisparityWLSFilter, super::SparseMatchInterpolator, super::EdgeAwareInterpolator, super::RFFeatureGetter, super::StructuredEdgeDetection, super::SuperpixelSEEDS, super::GraphSegmentation, super::SelectiveSearchSegmentationStrategy, super::SelectiveSearchSegmentationStrategyColor, super::SelectiveSearchSegmentationStrategySize, super::SelectiveSearchSegmentationStrategyTexture, super::SelectiveSearchSegmentationStrategyFill, super::SelectiveSearchSegmentationStrategyMultiple, super::SelectiveSearchSegmentation, super::SuperpixelSLIC, super::SuperpixelLSC, super::FastLineDetector };
}

pub const AM_FILTER: i32 = 4;
pub const ARO_0_45: i32 = 0;
pub const ARO_315_0: i32 = 3;
pub const ARO_315_135: i32 = 6;
pub const ARO_315_45: i32 = 4;
pub const ARO_45_135: i32 = 5;
pub const ARO_45_90: i32 = 1;
pub const ARO_90_135: i32 = 2;
pub const ARO_CTR_HOR: i32 = 7;
pub const ARO_CTR_VER: i32 = 8;
pub const DTF_IC: i32 = 1;
pub const DTF_NC: i32 = 0;
pub const DTF_RF: i32 = 2;
pub const FHT_ADD: i32 = 2;
pub const FHT_AVE: i32 = 3;
pub const FHT_MAX: i32 = 1;
pub const FHT_MIN: i32 = 0;
pub const GUIDED_FILTER: i32 = 3;
pub const HDO_DESKEW: i32 = 1;
pub const HDO_RAW: i32 = 0;
/// Skip validations of image borders.
pub const RO_IGNORE_BORDERS: i32 = 1;
/// Validate each rule in a proper way.
pub const RO_STRICT: i32 = 0;
pub const SLIC: i32 = 100;
pub const SLICO: i32 = 101;
pub const THINNING_GUOHALL: i32 = 1;
pub const THINNING_ZHANGSUEN: i32 = 0;
/// ![inline formula](https://latex.codecogs.com/png.latex?dot%28I1%2CI2%29%2F%28%7CI1%7C%2A%7CI2%7C%29)
pub const WMF_COS: i32 = 3;
/// ![inline formula](https://latex.codecogs.com/png.latex?exp%28%2D%7CI1%2DI2%7C%5E2%2F%282%2Asigma%5E2%29%29)
pub const WMF_EXP: i32 = 0;
/// ![inline formula](https://latex.codecogs.com/png.latex?%28%7CI1%2DI2%7C%2Bsigma%29%5E%2D1)
pub const WMF_IV1: i32 = 1;
/// ![inline formula](https://latex.codecogs.com/png.latex?%28%7CI1%2DI2%7C%5E2%2Bsigma%5E2%29%5E%2D1)
pub const WMF_IV2: i32 = 2;
/// ![inline formula](https://latex.codecogs.com/png.latex?%28min%28r1%2Cr2%29%2Bmin%28g1%2Cg2%29%2Bmin%28b1%2Cb2%29%29%2F%28max%28r1%2Cr2%29%2Bmax%28g1%2Cg2%29%2Bmax%28b1%2Cb2%29%29)
pub const WMF_JAC: i32 = 4;
/// unweighted
pub const WMF_OFF: i32 = 5;
///   Specifies the part of Hough space to calculate
/// @details The enum specifies the part of Hough space to calculate. Each
/// member specifies primarily direction of lines (horizontal or vertical)
/// and the direction of angle changes.
/// Direction of angle changes is from multiples of 90 to odd multiples of 45.
/// The image considered to be written top-down and left-to-right.
/// Angles are started from vertical line and go clockwise.
/// Separate quarters and halves are written in orientation they should be in
/// full Hough space.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AngleRangeOption {
	ARO_0_45 = 0,
	ARO_45_90 = 1,
	ARO_90_135 = 2,
	ARO_315_0 = 3,
	ARO_315_45 = 4,
	ARO_45_135 = 5,
	ARO_315_135 = 6,
	ARO_CTR_HOR = 7,
	ARO_CTR_VER = 8,
}

opencv_type_enum! { crate::ximgproc::AngleRangeOption }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EdgeAwareFiltersList {
	DTF_NC = 0,
	DTF_IC = 1,
	DTF_RF = 2,
	GUIDED_FILTER = 3,
	AM_FILTER = 4,
}

opencv_type_enum! { crate::ximgproc::EdgeAwareFiltersList }

///   Specifies to do or not to do skewing of Hough transform image
/// @details The enum specifies to do or not to do skewing of Hough transform image
/// so it would be no cycling in Hough transform image through borders of image.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HoughDeskewOption {
	HDO_RAW = 0,
	HDO_DESKEW = 1,
}

opencv_type_enum! { crate::ximgproc::HoughDeskewOption }

///   Specifies binary operations.
/// @details The enum specifies binary operations, that is such ones which involve
///          two operands. Formally, a binary operation @f$ f @f$ on a set @f$ S @f$
///          is a binary relation that maps elements of the Cartesian product
///          @f$ S \times S @f$ to @f$ S @f$:
///           @f[ f: S \times S \to S @f]
/// @ingroup MinUtils_MathOper
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HoughOp {
	FHT_MIN = 0,
	FHT_MAX = 1,
	FHT_ADD = 2,
	FHT_AVE = 3,
}

opencv_type_enum! { crate::ximgproc::HoughOp }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RulesOption {
	/// Validate each rule in a proper way.
	RO_STRICT = 0,
	/// Skip validations of image borders.
	RO_IGNORE_BORDERS = 1,
}

opencv_type_enum! { crate::ximgproc::RulesOption }

/// Class implementing the SLIC (Simple Linear Iterative Clustering) superpixels
/// 
/// ## Parameters
/// * image: Image to segment
/// * algorithm: Chooses the algorithm variant to use:
/// SLIC segments image using a desired region_size, and in addition
/// SLICO will choose an adaptive compactness factor.
/// * region_size: Chooses an average superpixel size measured in pixels
/// * ruler: Chooses the enforcement of superpixel smoothness factor of superpixel
/// 
/// The function initializes a SuperpixelSLIC object for the input image. It sets the parameters of choosed
/// superpixel algorithm, which are: region_size and ruler. It preallocate some buffers for future
/// computing iterations over the given image. An example of SLIC versus SLICO is ilustrated in the
/// following picture.
/// 
/// ![image](https://docs.opencv.org/3.2.0/superpixels_slic.png)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SLIC {
	SLIC = 100,
	SLICO = 101,
}

opencv_type_enum! { crate::ximgproc::SLIC }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ThinningTypes {
	THINNING_ZHANGSUEN = 0,
	THINNING_GUOHALL = 1,
}

opencv_type_enum! { crate::ximgproc::ThinningTypes }

/// Specifies weight types of weighted median filter.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WMFWeightType {
	/// ![inline formula](https://latex.codecogs.com/png.latex?exp%28%2D%7CI1%2DI2%7C%5E2%2F%282%2Asigma%5E2%29%29)
	WMF_EXP = 0,
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28%7CI1%2DI2%7C%2Bsigma%29%5E%2D1)
	WMF_IV1 = 1,
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28%7CI1%2DI2%7C%5E2%2Bsigma%5E2%29%5E%2D1)
	WMF_IV2 = 2,
	/// ![inline formula](https://latex.codecogs.com/png.latex?dot%28I1%2CI2%29%2F%28%7CI1%7C%2A%7CI2%7C%29)
	WMF_COS = 3,
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28min%28r1%2Cr2%29%2Bmin%28g1%2Cg2%29%2Bmin%28b1%2Cb2%29%29%2F%28max%28r1%2Cr2%29%2Bmax%28g1%2Cg2%29%2Bmax%28b1%2Cb2%29%29)
	WMF_JAC = 4,
	/// unweighted
	WMF_OFF = 5,
}

opencv_type_enum! { crate::ximgproc::WMFWeightType }

///   Calculates 2D Fast Hough transform of an image.
/// ## Parameters
/// * dst: The destination image, result of transformation.
/// * src: The source (input) image.
/// * dstMatDepth: The depth of destination image
/// * op: The operation to be applied, see cv::HoughOp
/// * angleRange: The part of Hough space to calculate, see cv::AngleRangeOption
/// * makeSkew: Specifies to do or not to do image skewing, see cv::HoughDeskewOption
/// 
/// The function calculates the fast Hough transform for full, half or quarter
/// range of angles.
/// 
/// ## C++ default parameters
/// * angle_range: ARO_315_135
/// * op: FHT_ADD
/// * make_skew: HDO_DESKEW
pub fn fast_hough_transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dst_mat_depth: i32, angle_range: i32, op: i32, make_skew: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dst_mat_depth, angle_range, op, make_skew) }.into_result()
}

///   Applies X Deriche filter to an image.
/// 
/// For more details about this implementation, please see http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.476.5736&rep=rep1&type=pdf
/// 
/// ## Parameters
/// * _op: Source 8-bit or 16bit image, 1-channel or 3-channel image.
/// * _dst: result CV_32FC image with same number of channel than _op.
/// * alphaDerive: double see paper
/// * alphaMean: double see paper
pub fn gradient_deriche_x(_op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha_derive: f64, alpha_mean: f64) -> Result<()> {
	input_array_arg!(_op);
	output_array_arg!(_dst);
	unsafe { sys::cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(_op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha_derive, alpha_mean) }.into_result()
}

///   Applies Y Deriche filter to an image.
/// 
/// For more details about this implementation, please see http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.476.5736&rep=rep1&type=pdf
/// 
/// ## Parameters
/// * _op: Source 8-bit or 16bit image, 1-channel or 3-channel image.
/// * _dst: result CV_32FC image with same number of channel than _op.
/// * alphaDerive: double see paper
/// * alphaMean: double see paper
pub fn gradient_deriche_y(_op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha_derive: f64, alpha_mean: f64) -> Result<()> {
	input_array_arg!(_op);
	output_array_arg!(_dst);
	unsafe { sys::cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(_op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha_derive, alpha_mean) }.into_result()
}

pub fn gradient_paillou_x(op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(_dst);
	unsafe { sys::cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha, omega) }.into_result()
}

///   Applies Paillou filter to an image.
/// 
/// For more details about this implementation, please see [paillou1997detecting](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_paillou1997detecting)
/// 
/// ## Parameters
/// * op: Source 8-bit or 16bit image, 1-channel or 3-channel image.
/// * _dst: result CV_32F image with same numeber of channel than op.
/// * omega: double see paper
/// * alpha: double see paper
/// ## See also
/// GradientPaillouX, GradientPaillouY
pub fn gradient_paillou_y(op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(_dst);
	unsafe { sys::cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha, omega) }.into_result()
}

///   Calculates coordinates of line segment corresponded by point in Hough space.
/// ## Parameters
/// * houghPoint: Point in Hough space.
/// * srcImgInfo: The source (input) image of Hough transform.
/// * angleRange: The part of Hough space where point is situated, see cv::AngleRangeOption
/// * makeSkew: Specifies to do or not to do image skewing, see cv::HoughDeskewOption
/// * rules: Specifies strictness of line segment calculating, see cv::RulesOption
/// @retval  [Vec4i]     Coordinates of line segment corresponded by point in Hough space.
/// @remarks If rules parameter set to RO_STRICT
///        then returned line cut along the border of source image.
/// @remarks If rules parameter set to RO_WEAK then in case of point, which belongs
///        the incorrect part of Hough image, returned line will not intersect source image.
/// 
/// The function calculates coordinates of line segment corresponded by point in Hough space.
/// 
/// ## C++ default parameters
/// * angle_range: ARO_315_135
/// * make_skew: HDO_DESKEW
/// * rules: RO_IGNORE_BORDERS
pub fn hough_point2_line(hough_point: core::Point, src_img_info: &dyn core::ToInputArray, angle_range: i32, make_skew: i32, rules: i32) -> Result<core::Vec4i> {
	input_array_arg!(src_img_info);
	unsafe { sys::cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(&hough_point, src_img_info.as_raw__InputArray(), angle_range, make_skew, rules) }.into_result()
}

/// Simple one-line Adaptive Manifold Filter call.
/// 
/// ## Parameters
/// * joint: joint (also called as guided) image or array of images with any numbers of channels.
/// 
/// * src: filtering image with any numbers of channels.
/// 
/// * dst: output image.
/// 
/// * sigma_s: spatial standard deviation.
/// 
/// * sigma_r: color space standard deviation, it is similar to the sigma in the color space into
/// bilateralFilter.
/// 
/// * adjust_outliers: optional, specify perform outliers adjust operation or not, (Eq. 9) in the
/// original paper.
/// 
/// 
/// Note: Joint images with CV_8U and CV_16U depth converted to images with CV_32F depth and [0; 1]
/// color range before processing. Hence color space sigma sigma_r must be in [0; 1] range, unlike same
/// sigmas in bilateralFilter and dtFilter functions. see also: bilateralFilter, dtFilter, guidedFilter
/// 
/// ## C++ default parameters
/// * adjust_outliers: false
pub fn am_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_s: f64, sigma_r: f64, adjust_outliers: bool) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_s, sigma_r, adjust_outliers) }.into_result()
}

/// Applies the bilateral texture filter to an image. It performs structure-preserving texture filter.
/// For more details about this filter see [Cho2014](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Cho2014).
/// 
/// ## Parameters
/// * src: Source image whose depth is 8-bit UINT or 32-bit FLOAT
/// 
/// * dst: Destination image of the same size and type as src.
/// 
/// * fr: Radius of kernel to be used for filtering. It should be positive integer
/// 
/// * numIter: Number of iterations of algorithm, It should be positive integer
/// 
/// * sigmaAlpha: Controls the sharpness of the weight transition from edges to smooth/texture regions, where
/// a bigger value means sharper transition. When the value is negative, it is automatically calculated.
/// 
/// * sigmaAvg: Range blur parameter for texture blurring. Larger value makes result to be more blurred. When the
/// value is negative, it is automatically calculated as described in the paper.
/// ## See also
/// rollingGuidanceFilter, bilateralFilter
/// 
/// ## C++ default parameters
/// * fr: 3
/// * num_iter: 1
/// * sigma_alpha: -1.
/// * sigma_avg: -1.
pub fn bilateral_texture_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, fr: i32, num_iter: i32, sigma_alpha: f64, sigma_avg: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), fr, num_iter, sigma_alpha, sigma_avg) }.into_result()
}

/// Function for computing the percent of "bad" pixels in the disparity map
/// (pixels where error is higher than a specified threshold)
/// 
/// ## Parameters
/// * GT: ground truth disparity map
/// 
/// * src: disparity map to evaluate
/// 
/// * ROI: region of interest
/// 
/// * thresh: threshold used to determine "bad" pixels
/// 
/// @result returns mean square error between GT and src
/// 
/// ## C++ default parameters
/// * thresh: 24
pub fn compute_bad_pixel_percent(gt: &dyn core::ToInputArray, src: &dyn core::ToInputArray, roi: core::Rect, thresh: i32) -> Result<f64> {
	input_array_arg!(gt);
	input_array_arg!(src);
	unsafe { sys::cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(gt.as_raw__InputArray(), src.as_raw__InputArray(), roi.opencv_to_extern(), thresh) }.into_result()
}

/// Function for computing mean square error for disparity maps
/// 
/// ## Parameters
/// * GT: ground truth disparity map
/// 
/// * src: disparity map to evaluate
/// 
/// * ROI: region of interest
/// 
/// @result returns mean square error between GT and src
pub fn compute_mse(gt: &dyn core::ToInputArray, src: &dyn core::ToInputArray, roi: core::Rect) -> Result<f64> {
	input_array_arg!(gt);
	input_array_arg!(src);
	unsafe { sys::cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(gt.as_raw__InputArray(), src.as_raw__InputArray(), roi.opencv_to_extern()) }.into_result()
}

/// Computes the estimated covariance matrix of an image using the sliding
/// window forumlation.
/// 
/// ## Parameters
/// * src: The source image. Input image must be of a complex type.
/// * dst: The destination estimated covariance matrix. Output matrix will be size (windowRows*windowCols, windowRows*windowCols).
/// * windowRows: The number of rows in the window.
/// * windowCols: The number of cols in the window.
/// The window size parameters control the accuracy of the estimation.
/// The sliding window moves over the entire image from the top-left corner
/// to the bottom right corner. Each location of the window represents a sample.
/// If the window is the size of the image, then this gives the exact covariance matrix.
/// For all other cases, the sizes of the window will impact the number of samples
/// and the number of elements in the estimated covariance matrix.
pub fn covariance_estimation(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, window_rows: i32, window_cols: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), window_rows, window_cols) }.into_result()
}

/// Factory method, create instance of AdaptiveManifoldFilter and produce some initialization routines.
/// 
/// ## Parameters
/// * sigma_s: spatial standard deviation.
/// 
/// * sigma_r: color space standard deviation, it is similar to the sigma in the color space into
/// bilateralFilter.
/// 
/// * adjust_outliers: optional, specify perform outliers adjust operation or not, (Eq. 9) in the
/// original paper.
/// 
/// For more details about Adaptive Manifold Filter parameters, see the original article [Gastal12](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Gastal12) .
/// 
/// 
/// Note: Joint images with CV_8U and CV_16U depth converted to images with CV_32F depth and [0; 1]
/// color range before processing. Hence color space sigma sigma_r must be in [0; 1] range, unlike same
/// sigmas in bilateralFilter and dtFilter functions.
/// 
/// ## C++ default parameters
/// * adjust_outliers: false
pub fn create_am_filter(sigma_s: f64, sigma_r: f64, adjust_outliers: bool) -> Result<core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>> {
	unsafe { sys::cv_ximgproc_createAMFilter_double_double_bool(sigma_s, sigma_r, adjust_outliers) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>::opencv_from_extern(r) } )
}

/// Factory method, create instance of DTFilter and produce initialization routines.
/// 
/// ## Parameters
/// * guide: guided image (used to build transformed distance, which describes edge structure of
/// guided image).
/// 
/// * sigmaSpatial: ![inline formula](https://latex.codecogs.com/png.latex?%7B%5Csigma%7D%5FH) parameter in the original article, it's similar to the sigma in the
/// coordinate space into bilateralFilter.
/// 
/// * sigmaColor: ![inline formula](https://latex.codecogs.com/png.latex?%7B%5Csigma%7D%5Fr) parameter in the original article, it's similar to the sigma in the
/// color space into bilateralFilter.
/// 
/// * mode: one form three modes DTF_NC, DTF_RF and DTF_IC which corresponds to three modes for
/// filtering 2D signals in the article.
/// 
/// * numIters: optional number of iterations used for filtering, 3 is quite enough.
/// 
/// For more details about Domain Transform filter parameters, see the original article [Gastal11](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Gastal11) and
/// [Domain Transform filter homepage](http://www.inf.ufrgs.br/~eslgastal/DomainTransform/).
/// 
/// ## C++ default parameters
/// * mode: DTF_NC
/// * num_iters: 3
pub fn create_dt_filter(guide: &dyn core::ToInputArray, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32) -> Result<core::Ptr::<dyn crate::ximgproc::DTFilter>> {
	input_array_arg!(guide);
	unsafe { sys::cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(guide.as_raw__InputArray(), sigma_spatial, sigma_color, mode, num_iters) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::DTFilter>::opencv_from_extern(r) } )
}

/// More generic factory method, create instance of DisparityWLSFilter and execute basic
/// initialization routines. When using this method you will need to set-up the ROI, matchers and
/// other parameters by yourself.
/// 
/// ## Parameters
/// * use_confidence: filtering with confidence requires two disparity maps (for the left and right views) and is
/// approximately two times slower. However, quality is typically significantly better.
pub fn create_disparity_wls_filter_generic(use_confidence: bool) -> Result<core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>> {
	unsafe { sys::cv_ximgproc_createDisparityWLSFilterGeneric_bool(use_confidence) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>::opencv_from_extern(r) } )
}

/// Convenience factory method that creates an instance of DisparityWLSFilter and sets up all the relevant
/// filter parameters automatically based on the matcher instance. Currently supports only StereoBM and StereoSGBM.
/// 
/// ## Parameters
/// * matcher_left: stereo matcher instance that will be used with the filter
pub fn create_disparity_wls_filter(mut matcher_left: core::Ptr::<dyn crate::calib3d::StereoMatcher>) -> Result<core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>> {
	unsafe { sys::cv_ximgproc_createDisparityWLSFilter_Ptr_StereoMatcher_(matcher_left.as_raw_mut_PtrOfStereoMatcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>::opencv_from_extern(r) } )
}

/// Factory method that creates an instance of the
/// EdgeAwareInterpolator.
pub fn create_edge_aware_interpolator() -> Result<core::Ptr::<dyn crate::ximgproc::EdgeAwareInterpolator>> {
	unsafe { sys::cv_ximgproc_createEdgeAwareInterpolator() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::EdgeAwareInterpolator>::opencv_from_extern(r) } )
}

/// Factory method, create instance of FastGlobalSmootherFilter and execute the initialization routines.
/// 
/// ## Parameters
/// * guide: image serving as guide for filtering. It should have 8-bit depth and either 1 or 3 channels.
/// 
/// * lambda: parameter defining the amount of regularization
/// 
/// * sigma_color: parameter, that is similar to color space sigma in bilateralFilter.
/// 
/// * lambda_attenuation: internal parameter, defining how much lambda decreases after each iteration. Normally,
/// it should be 0.25. Setting it to 1.0 may lead to streaking artifacts.
/// 
/// * num_iter: number of iterations used for filtering, 3 is usually enough.
/// 
/// For more details about Fast Global Smoother parameters, see the original paper [Min2014](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Min2014). However, please note that
/// there are several differences. Lambda attenuation described in the paper is implemented a bit differently so do not
/// expect the results to be identical to those from the paper; sigma_color values from the paper should be multiplied by 255.0 to
/// achieve the same effect. Also, in case of image filtering where source and guide image are the same, authors
/// propose to dynamically update the guide image after each iteration. To maximize the performance this feature
/// was not implemented here.
/// 
/// ## C++ default parameters
/// * lambda_attenuation: 0.25
/// * num_iter: 3
pub fn create_fast_global_smoother_filter(guide: &dyn core::ToInputArray, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32) -> Result<core::Ptr::<dyn crate::ximgproc::FastGlobalSmootherFilter>> {
	input_array_arg!(guide);
	unsafe { sys::cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(guide.as_raw__InputArray(), lambda, sigma_color, lambda_attenuation, num_iter) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::FastGlobalSmootherFilter>::opencv_from_extern(r) } )
}

/// Creates a smart pointer to a FastLineDetector object and initializes it
/// 
/// ## Parameters
/// * _length_threshold: 10         - Segment shorter than this will be discarded
/// * _distance_threshold: 1.41421356 - A point placed from a hypothesis line
///                                          segment farther than this will be
///                                          regarded as an outlier
/// * _canny_th1: 50         - First threshold for
///                                          hysteresis procedure in Canny()
/// * _canny_th2: 50         - Second threshold for
///                                          hysteresis procedure in Canny()
/// * _canny_aperture_size: 3          - Aperturesize for the sobel
///                                          operator in Canny()
/// * _do_merge: false      - If true, incremental merging of segments
///                                          will be perfomred
/// 
/// ## C++ default parameters
/// * _length_threshold: 10
/// * _distance_threshold: 1.414213562f
/// * _canny_th1: 50.0
/// * _canny_th2: 50.0
/// * _canny_aperture_size: 3
/// * _do_merge: false
pub fn create_fast_line_detector(_length_threshold: i32, _distance_threshold: f32, _canny_th1: f64, _canny_th2: f64, _canny_aperture_size: i32, _do_merge: bool) -> Result<core::Ptr::<dyn crate::ximgproc::FastLineDetector>> {
	unsafe { sys::cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(_length_threshold, _distance_threshold, _canny_th1, _canny_th2, _canny_aperture_size, _do_merge) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::FastLineDetector>::opencv_from_extern(r) } )
}

/// Factory method, create instance of GuidedFilter and produce initialization routines.
/// 
/// ## Parameters
/// * guide: guided image (or array of images) with up to 3 channels, if it have more then 3
/// channels then only first 3 channels will be used.
/// 
/// * radius: radius of Guided Filter.
/// 
/// * eps: regularization term of Guided Filter. ![inline formula](https://latex.codecogs.com/png.latex?%7Beps%7D%5E2) is similar to the sigma in the color
/// space into bilateralFilter.
/// 
/// For more details about Guided Filter parameters, see the original article [Kaiming10](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Kaiming10) .
pub fn create_guided_filter(guide: &dyn core::ToInputArray, radius: i32, eps: f64) -> Result<core::Ptr::<dyn crate::ximgproc::GuidedFilter>> {
	input_array_arg!(guide);
	unsafe { sys::cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(guide.as_raw__InputArray(), radius, eps) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::GuidedFilter>::opencv_from_extern(r) } )
}

pub fn create_rf_feature_getter() -> Result<core::Ptr::<dyn crate::ximgproc::RFFeatureGetter>> {
	unsafe { sys::cv_ximgproc_createRFFeatureGetter() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::RFFeatureGetter>::opencv_from_extern(r) } )
}

/// Convenience method to set up the matcher for computing the right-view disparity map
/// that is required in case of filtering with confidence.
/// 
/// ## Parameters
/// * matcher_left: main stereo matcher instance that will be used with the filter
pub fn create_right_matcher(mut matcher_left: core::Ptr::<dyn crate::calib3d::StereoMatcher>) -> Result<core::Ptr::<dyn crate::calib3d::StereoMatcher>> {
	unsafe { sys::cv_ximgproc_createRightMatcher_Ptr_StereoMatcher_(matcher_left.as_raw_mut_PtrOfStereoMatcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::calib3d::StereoMatcher>::opencv_from_extern(r) } )
}

/// !
/// * The only constructor
/// *
/// * \param model : name of the file where the model is stored
/// * \param howToGetFeatures : optional object inheriting from RFFeatureGetter.
/// *                           You need it only if you would like to train your
/// *                           own forest, pass NULL otherwise
/// 
/// ## C++ default parameters
/// * how_to_get_features: Ptr<RFFeatureGetter>()
pub fn create_structured_edge_detection(model: &str, how_to_get_features: core::Ptr::<dyn crate::ximgproc::RFFeatureGetter>) -> Result<core::Ptr::<dyn crate::ximgproc::StructuredEdgeDetection>> {
	extern_container_arg!(model);
	unsafe { sys::cv_ximgproc_createStructuredEdgeDetection_const_StringR_Ptr_RFFeatureGetter_(model.opencv_to_extern(), how_to_get_features.as_raw_PtrOfRFFeatureGetter()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::StructuredEdgeDetection>::opencv_from_extern(r) } )
}

/// Class implementing the LSC (Linear Spectral Clustering) superpixels
/// 
/// ## Parameters
/// * image: Image to segment
/// * region_size: Chooses an average superpixel size measured in pixels
/// * ratio: Chooses the enforcement of superpixel compactness factor of superpixel
/// 
/// The function initializes a SuperpixelLSC object for the input image. It sets the parameters of
/// superpixel algorithm, which are: region_size and ruler. It preallocate some buffers for future
/// computing iterations over the given image. An example of LSC is ilustrated in the following picture.
/// For enanched results it is recommended for color images to preprocess image with little gaussian blur
/// with a small 3 x 3 kernel and additional conversion into CieLAB color space.
/// 
/// ![image](https://docs.opencv.org/3.2.0/superpixels_lsc.png)
/// 
/// ## C++ default parameters
/// * region_size: 10
/// * ratio: 0.075f
pub fn create_superpixel_lsc(image: &dyn core::ToInputArray, region_size: i32, ratio: f32) -> Result<core::Ptr::<dyn crate::ximgproc::SuperpixelLSC>> {
	input_array_arg!(image);
	unsafe { sys::cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(image.as_raw__InputArray(), region_size, ratio) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelLSC>::opencv_from_extern(r) } )
}

/// Initializes a SuperpixelSEEDS object.
/// 
/// ## Parameters
/// * image_width: Image width.
/// * image_height: Image height.
/// * image_channels: Number of channels of the image.
/// * num_superpixels: Desired number of superpixels. Note that the actual number may be smaller
/// due to restrictions (depending on the image size and num_levels). Use getNumberOfSuperpixels() to
/// get the actual number.
/// * num_levels: Number of block levels. The more levels, the more accurate is the segmentation,
/// but needs more memory and CPU time.
/// * prior: enable 3x3 shape smoothing term if \>0. A larger value leads to smoother shapes. prior
/// must be in the range [0, 5].
/// * histogram_bins: Number of histogram bins.
/// * double_step: If true, iterate each block level twice for higher accuracy.
/// 
/// The function initializes a SuperpixelSEEDS object for the input image. It stores the parameters of
/// the image: image_width, image_height and image_channels. It also sets the parameters of the SEEDS
/// superpixel algorithm, which are: num_superpixels, num_levels, use_prior, histogram_bins and
/// double_step.
/// 
/// The number of levels in num_levels defines the amount of block levels that the algorithm use in the
/// optimization. The initialization is a grid, in which the superpixels are equally distributed through
/// the width and the height of the image. The larger blocks correspond to the superpixel size, and the
/// levels with smaller blocks are formed by dividing the larger blocks into 2 x 2 blocks of pixels,
/// recursively until the smaller block level. An example of initialization of 4 block levels is
/// illustrated in the following figure.
/// 
/// ![image](https://docs.opencv.org/3.2.0/superpixels_blocks.png)
/// 
/// ## C++ default parameters
/// * prior: 2
/// * histogram_bins: 5
/// * double_step: false
pub fn create_superpixel_seeds(image_width: i32, image_height: i32, image_channels: i32, num_superpixels: i32, num_levels: i32, prior: i32, histogram_bins: i32, double_step: bool) -> Result<core::Ptr::<dyn crate::ximgproc::SuperpixelSEEDS>> {
	unsafe { sys::cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(image_width, image_height, image_channels, num_superpixels, num_levels, prior, histogram_bins, double_step) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelSEEDS>::opencv_from_extern(r) } )
}

/// ## C++ default parameters
/// * algorithm: SLICO
/// * region_size: 10
/// * ruler: 10.0f
pub fn create_superpixel_slic(image: &dyn core::ToInputArray, algorithm: i32, region_size: i32, ruler: f32) -> Result<core::Ptr::<dyn crate::ximgproc::SuperpixelSLIC>> {
	input_array_arg!(image);
	unsafe { sys::cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(image.as_raw__InputArray(), algorithm, region_size, ruler) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelSLIC>::opencv_from_extern(r) } )
}

/// Simple one-line Domain Transform filter call. If you have multiple images to filter with the same
/// guided image then use DTFilter interface to avoid extra computations on initialization stage.
/// 
/// ## Parameters
/// * guide: guided image (also called as joint image) with unsigned 8-bit or floating-point 32-bit
/// depth and up to 4 channels.
/// * src: filtering image with unsigned 8-bit or floating-point 32-bit depth and up to 4 channels.
/// * dst: 
/// * sigmaSpatial: ![inline formula](https://latex.codecogs.com/png.latex?%7B%5Csigma%7D%5FH) parameter in the original article, it's similar to the sigma in the
/// coordinate space into bilateralFilter.
/// * sigmaColor: ![inline formula](https://latex.codecogs.com/png.latex?%7B%5Csigma%7D%5Fr) parameter in the original article, it's similar to the sigma in the
/// color space into bilateralFilter.
/// * mode: one form three modes DTF_NC, DTF_RF and DTF_IC which corresponds to three modes for
/// filtering 2D signals in the article.
/// * numIters: optional number of iterations used for filtering, 3 is quite enough.
/// ## See also
/// bilateralFilter, guidedFilter, amFilter
/// 
/// ## C++ default parameters
/// * mode: DTF_NC
/// * num_iters: 3
pub fn dt_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_spatial, sigma_color, mode, num_iters) }.into_result()
}

/// Simple one-line Fast Global Smoother filter call. If you have multiple images to filter with the same
/// guide then use FastGlobalSmootherFilter interface to avoid extra computations.
/// 
/// ## Parameters
/// * guide: image serving as guide for filtering. It should have 8-bit depth and either 1 or 3 channels.
/// 
/// * src: source image for filtering with unsigned 8-bit or signed 16-bit or floating-point 32-bit depth and up to 4 channels.
/// 
/// * dst: destination image.
/// 
/// * lambda: parameter defining the amount of regularization
/// 
/// * sigma_color: parameter, that is similar to color space sigma in bilateralFilter.
/// 
/// * lambda_attenuation: internal parameter, defining how much lambda decreases after each iteration. Normally,
/// it should be 0.25. Setting it to 1.0 may lead to streaking artifacts.
/// 
/// * num_iter: number of iterations used for filtering, 3 is usually enough.
/// 
/// ## C++ default parameters
/// * lambda_attenuation: 0.25
/// * num_iter: 3
pub fn fast_global_smoother_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), lambda, sigma_color, lambda_attenuation, num_iter) }.into_result()
}

/// Function for creating a disparity map visualization (clamped CV_8U image)
/// 
/// ## Parameters
/// * src: input disparity map (CV_16S depth)
/// 
/// * dst: output visualization
/// 
/// * scale: disparity map will be multiplied by this value for visualization
/// 
/// ## C++ default parameters
/// * scale: 1.0
pub fn get_disparity_vis(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), scale) }.into_result()
}

/// Simple one-line Guided Filter call.
/// 
/// If you have multiple images to filter with the same guided image then use GuidedFilter interface to
/// avoid extra computations on initialization stage.
/// 
/// ## Parameters
/// * guide: guided image (or array of images) with up to 3 channels, if it have more then 3
/// channels then only first 3 channels will be used.
/// 
/// * src: filtering image with any numbers of channels.
/// 
/// * dst: output image.
/// 
/// * radius: radius of Guided Filter.
/// 
/// * eps: regularization term of Guided Filter. ![inline formula](https://latex.codecogs.com/png.latex?%7Beps%7D%5E2) is similar to the sigma in the color
/// space into bilateralFilter.
/// 
/// * dDepth: optional depth of the output image.
/// ## See also
/// bilateralFilter, dtFilter, amFilter
/// 
/// ## C++ default parameters
/// * d_depth: -1
pub fn guided_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, radius: i32, eps: f64, d_depth: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), radius, eps, d_depth) }.into_result()
}

/// Applies the joint bilateral filter to an image.
/// 
/// ## Parameters
/// * joint: Joint 8-bit or floating-point, 1-channel or 3-channel image.
/// 
/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image with the same depth as joint
/// image.
/// 
/// * dst: Destination image of the same size and type as src .
/// 
/// * d: Diameter of each pixel neighborhood that is used during filtering. If it is non-positive,
/// it is computed from sigmaSpace .
/// 
/// * sigmaColor: Filter sigma in the color space. A larger value of the parameter means that
/// farther colors within the pixel neighborhood (see sigmaSpace ) will be mixed together, resulting in
/// larger areas of semi-equal color.
/// 
/// * sigmaSpace: Filter sigma in the coordinate space. A larger value of the parameter means that
/// farther pixels will influence each other as long as their colors are close enough (see sigmaColor ).
/// When d\>0 , it specifies the neighborhood size regardless of sigmaSpace . Otherwise, d is
/// proportional to sigmaSpace .
/// 
/// * borderType: 
/// 
/// 
/// Note: bilateralFilter and jointBilateralFilter use L1 norm to compute difference between colors.
/// ## See also
/// bilateralFilter, amFilter
/// 
/// ## C++ default parameters
/// * border_type: BORDER_DEFAULT
pub fn joint_bilateral_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d, sigma_color, sigma_space, border_type) }.into_result()
}

/// Global image smoothing via L0 gradient minimization.
/// 
/// ## Parameters
/// * src: source image for filtering with unsigned 8-bit or signed 16-bit or floating-point depth.
/// 
/// * dst: destination image.
/// 
/// * lambda: parameter defining the smooth term weight.
/// 
/// * kappa: parameter defining the increasing factor of the weight of the gradient data term.
/// 
/// For more details about L0 Smoother, see the original paper [xu2011image](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_xu2011image).
/// 
/// ## C++ default parameters
/// * lambda: 0.02
/// * kappa: 2.0
pub fn l0_smooth(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, lambda: f64, kappa: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), lambda, kappa) }.into_result()
}

/// Applies Niblack thresholding to input image.
/// 
/// The function transforms a grayscale image to a binary image according to the formulae:
/// *   **THRESH_BINARY**
///    ![block formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7BmaxValue%7D%7D%7Bif%20%5C%28src%28x%2Cy%29%20%3E%20T%28x%2Cy%29%5C%29%7D%7B0%7D%7Botherwise%7D)
/// *   **THRESH_BINARY_INV**
///    ![block formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28src%28x%2Cy%29%20%3E%20T%28x%2Cy%29%5C%29%7D%7B%5Ctexttt%7BmaxValue%7D%7D%7Botherwise%7D)
/// where ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2Cy%29) is a threshold calculated individually for each pixel.
/// 
/// The threshold value ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2C%20y%29) is the mean minus ![inline formula](https://latex.codecogs.com/png.latex?%20delta%20) times standard deviation
/// of ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BblockSize%7D%20%5Ctimes%5Ctexttt%7BblockSize%7D) neighborhood of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29).
/// 
/// The function can't process the image in-place.
/// 
/// ## Parameters
/// * _src: Source 8-bit single-channel image.
/// * _dst: Destination image of the same size and the same type as src.
/// * maxValue: Non-zero value assigned to the pixels for which the condition is satisfied,
/// used with the THRESH_BINARY and THRESH_BINARY_INV thresholding types.
/// * type: Thresholding type, see cv::ThresholdTypes.
/// * blockSize: Size of a pixel neighborhood that is used to calculate a threshold value
/// for the pixel: 3, 5, 7, and so on.
/// * delta: Constant multiplied with the standard deviation and subtracted from the mean.
/// Normally, it is taken to be a real number between 0 and 1.
/// ## See also
/// threshold, adaptiveThreshold
pub fn ni_black_threshold(_src: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, max_value: f64, typ: i32, block_size: i32, delta: f64) -> Result<()> {
	input_array_arg!(_src);
	output_array_arg!(_dst);
	unsafe { sys::cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double(_src.as_raw__InputArray(), _dst.as_raw__OutputArray(), max_value, typ, block_size, delta) }.into_result()
}

/// Function for reading ground truth disparity maps. Supports basic Middlebury
/// and MPI-Sintel formats. Note that the resulting disparity map is scaled by 16.
/// 
/// ## Parameters
/// * src_path: path to the image, containing ground-truth disparity map
/// 
/// * dst: output disparity map, CV_16S depth
/// 
/// @result returns zero if successfully read the ground truth
pub fn read_gt(src_path: &str, dst: &mut dyn core::ToOutputArray) -> Result<i32> {
	extern_container_arg!(mut src_path);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_readGT_String_const__OutputArrayR(src_path.opencv_to_extern_mut(), dst.as_raw__OutputArray()) }.into_result()
}

/// Applies the rolling guidance filter to an image.
/// 
/// For more details, please see [zhang2014rolling](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_zhang2014rolling)
/// 
/// ## Parameters
/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image.
/// 
/// * dst: Destination image of the same size and type as src.
/// 
/// * d: Diameter of each pixel neighborhood that is used during filtering. If it is non-positive,
/// it is computed from sigmaSpace .
/// 
/// * sigmaColor: Filter sigma in the color space. A larger value of the parameter means that
/// farther colors within the pixel neighborhood (see sigmaSpace ) will be mixed together, resulting in
/// larger areas of semi-equal color.
/// 
/// * sigmaSpace: Filter sigma in the coordinate space. A larger value of the parameter means that
/// farther pixels will influence each other as long as their colors are close enough (see sigmaColor ).
/// When d\>0 , it specifies the neighborhood size regardless of sigmaSpace . Otherwise, d is
/// proportional to sigmaSpace .
/// 
/// * numOfIter: Number of iterations of joint edge-preserving filtering applied on the source image.
/// 
/// * borderType: 
/// 
/// 
/// Note:  rollingGuidanceFilter uses jointBilateralFilter as the edge-preserving filter.
/// ## See also
/// jointBilateralFilter, bilateralFilter, amFilter
/// 
/// ## C++ default parameters
/// * d: -1
/// * sigma_color: 25
/// * sigma_space: 3
/// * num_of_iter: 4
/// * border_type: BORDER_DEFAULT
pub fn rolling_guidance_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d: i32, sigma_color: f64, sigma_space: f64, num_of_iter: i32, border_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), d, sigma_color, sigma_space, num_of_iter, border_type) }.into_result()
}

/// Creates a graph based segmentor
/// ## Parameters
/// * sigma: The sigma parameter, used to smooth image
/// * k: The k parameter of the algorythm
/// * min_size: The minimum size of segments
/// 
/// ## C++ default parameters
/// * sigma: 0.5
/// * k: 300
/// * min_size: 100
pub fn create_graph_segmentation(sigma: f64, k: f32, min_size: i32) -> Result<core::Ptr::<dyn crate::ximgproc::GraphSegmentation>> {
	unsafe { sys::cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(sigma, k, min_size) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::GraphSegmentation>::opencv_from_extern(r) } )
}

/// Create a new SelectiveSearchSegmentation class.
pub fn create_selective_search_segmentation() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentation>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentation() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentation>::opencv_from_extern(r) } )
}

/// Create a new color-based strategy
pub fn create_selective_search_segmentation_strategy_color() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>::opencv_from_extern(r) } )
}

/// Create a new fill-based strategy
pub fn create_selective_search_segmentation_strategy_fill() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>::opencv_from_extern(r) } )
}

/// Create a new multiple strategy
pub fn create_selective_search_segmentation_strategy_multiple() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(r) } )
}

/// Create a new multiple strategy and set one subtrategy
/// ## Parameters
/// * s1: The first strategy
pub fn create_selective_search_segmentation_strategy_multiple_1(mut s1: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(r) } )
}

/// Create a new multiple strategy and set two subtrategies, with equal weights
/// ## Parameters
/// * s1: The first strategy
/// * s2: The second strategy
pub fn create_selective_search_segmentation_strategy_multiple_2(mut s1: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(r) } )
}

/// Create a new multiple strategy and set three subtrategies, with equal weights
/// ## Parameters
/// * s1: The first strategy
/// * s2: The second strategy
/// * s3: The third strategy
pub fn create_selective_search_segmentation_strategy_multiple_3(mut s1: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s3: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s3.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(r) } )
}

/// Create a new multiple strategy and set four subtrategies, with equal weights
/// ## Parameters
/// * s1: The first strategy
/// * s2: The second strategy
/// * s3: The third strategy
/// * s4: The forth strategy
pub fn create_selective_search_segmentation_strategy_multiple_4(mut s1: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s3: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s4: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s3.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s4.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(r) } )
}

/// Create a new size-based strategy
pub fn create_selective_search_segmentation_strategy_size() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>::opencv_from_extern(r) } )
}

/// Create a new size-based strategy
pub fn create_selective_search_segmentation_strategy_texture() -> Result<core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>> {
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>::opencv_from_extern(r) } )
}

/// Applies a binary blob thinning operation, to achieve a skeletization of the input image.
/// 
/// The function transforms a binary blob image into a skeletized form using the technique of Zhang-Suen.
/// 
/// ## Parameters
/// * src: Source 8-bit single-channel image, containing binary blobs, with blobs having 255 pixel values.
/// * dst: Destination image of the same size and the same type as src. The function can work in-place.
/// * thinningType: Value that defines which thinning algorithm should be used. See cv::ThinningTypes
/// 
/// ## C++ default parameters
/// * thinning_type: THINNING_ZHANGSUEN
pub fn thinning(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, thinning_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), thinning_type) }.into_result()
}

///   Applies weighted median filter to an image.
/// 
/// For more details about this implementation, please see [zhang2014100](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_zhang2014100)+
/// 
/// ## Parameters
/// * joint: Joint 8-bit, 1-channel or 3-channel image.
/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image.
/// * dst: Destination image.
/// * r: Radius of filtering kernel, should be a positive integer.
/// * sigma: Filter range standard deviation for the joint image.
/// * weightType: weightType The type of weight definition, see WMFWeightType
/// * mask: A 0-1 mask that has the same size with I. This mask is used to ignore the effect of some pixels. If the pixel value on mask is 0,
///                           the pixel will be ignored when maintaining the joint-histogram. This is useful for applications like optical flow occlusion handling.
/// ## See also
/// medianBlur, jointBilateralFilter
/// 
/// ## C++ default parameters
/// * sigma: 25.5
/// * weight_type: WMF_EXP
/// * mask: Mat()
pub fn weighted_median_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, r: i32, sigma: f64, weight_type: crate::ximgproc::WMFWeightType, mut mask: core::Mat) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_WMFWeightType_Mat(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), r, sigma, weight_type, mask.as_raw_mut_Mat()) }.into_result()
}

/// Interface for Adaptive Manifold Filter realizations.
/// 
/// For more details about this filter see [Gastal12](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Gastal12) and References_.
/// 
/// Below listed optional parameters which may be set up with Algorithm::set function.
/// *   member double sigma_s = 16.0
/// Spatial standard deviation.
/// *   member double sigma_r = 0.2
/// Color space standard deviation.
/// *   member int tree_height = -1
/// Height of the manifold tree (default = -1 : automatically computed).
/// *   member int num_pca_iterations = 1
/// Number of iterations to computed the eigenvector.
/// *   member bool adjust_outliers = false
/// Specify adjust outliers using Eq. 9 or not.
/// *   member bool use_RNG = true
/// Specify use random number generator to compute eigenvector or not.
pub trait AdaptiveManifoldFilter: core::AlgorithmTrait {
	fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void;
	fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void;

	/// Apply high-dimensional filtering using adaptive manifolds.
	/// 
	/// ## Parameters
	/// * src: filtering image with any numbers of channels.
	/// 
	/// * dst: output image.
	/// 
	/// * joint: optional joint (also called as guided) image with any numbers of channels.
	/// 
	/// ## C++ default parameters
	/// * joint: noArray()
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, joint: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(joint);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_AdaptiveManifoldFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), joint.as_raw__InputArray()) }.into_result()
	}
	
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(self.as_raw_mut_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setSigmaS
	fn get_sigma_s(&self) -> Result<f64> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setSigmaS getSigmaS
	fn set_sigma_s(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
	/// ## See also
	/// setSigmaR
	fn get_sigma_r(&self) -> Result<f64> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setSigmaR getSigmaR
	fn set_sigma_r(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
	/// ## See also
	/// setTreeHeight
	fn get_tree_height(&self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setTreeHeight getTreeHeight
	fn set_tree_height(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
	/// ## See also
	/// setPCAIterations
	fn get_pca_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setPCAIterations getPCAIterations
	fn set_pca_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
	/// ## See also
	/// setAdjustOutliers
	fn get_adjust_outliers(&self) -> Result<bool> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setAdjustOutliers getAdjustOutliers
	fn set_adjust_outliers(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
	/// ## See also
	/// setUseRNG
	fn get_use_rng(&self) -> Result<bool> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(self.as_raw_AdaptiveManifoldFilter()) }.into_result()
	}
	
	/// ## See also
	/// setUseRNG getUseRNG
	fn set_use_rng(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(self.as_raw_mut_AdaptiveManifoldFilter(), val) }.into_result()
	}
	
}

impl dyn AdaptiveManifoldFilter + '_ {
	pub fn create() -> Result<core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>> {
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>::opencv_from_extern(r) } )
	}
	
}
/// Interface for realizations of Domain Transform filter.
/// 
/// For more details about this filter see [Gastal11](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Gastal11) .
pub trait DTFilter: core::AlgorithmTrait {
	fn as_raw_DTFilter(&self) -> *const c_void;
	fn as_raw_mut_DTFilter(&mut self) -> *mut c_void;

	/// Produce domain transform filtering operation on source image.
	/// 
	/// ## Parameters
	/// * src: filtering image with unsigned 8-bit or floating-point 32-bit depth and up to 4 channels.
	/// 
	/// * dst: destination image.
	/// 
	/// * dDepth: optional depth of the output image. dDepth can be set to -1, which will be equivalent
	/// to src.depth().
	/// 
	/// ## C++ default parameters
	/// * d_depth: -1
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d_depth: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_DTFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d_depth) }.into_result()
	}
	
}

/// Main interface for all disparity map filters.
pub trait DisparityFilter: core::AlgorithmTrait {
	fn as_raw_DisparityFilter(&self) -> *const c_void;
	fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void;

	/// Apply filtering to the disparity map.
	/// 
	/// ## Parameters
	/// * disparity_map_left: disparity map of the left view, 1 channel, CV_16S type. Implicitly assumes that disparity
	/// values are scaled by 16 (one-pixel disparity corresponds to the value of 16 in the disparity map). Disparity map
	/// can have any resolution, it will be automatically resized to fit left_view resolution.
	/// 
	/// * left_view: left view of the original stereo-pair to guide the filtering process, 8-bit single-channel
	/// or three-channel image.
	/// 
	/// * filtered_disparity_map: output disparity map.
	/// 
	/// * disparity_map_right: optional argument, some implementations might also use the disparity map
	/// of the right view to compute confidence maps, for instance.
	/// 
	/// * ROI: region of the disparity map to filter. Optional, usually it should be set automatically.
	/// 
	/// * right_view: optional argument, some implementations might also use the right view of the original
	/// stereo-pair.
	/// 
	/// ## C++ default parameters
	/// * disparity_map_right: Mat()
	/// * roi: Rect()
	/// * right_view: Mat()
	fn filter(&mut self, disparity_map_left: &dyn core::ToInputArray, left_view: &dyn core::ToInputArray, filtered_disparity_map: &mut dyn core::ToOutputArray, disparity_map_right: &dyn core::ToInputArray, roi: core::Rect, right_view: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(disparity_map_left);
		input_array_arg!(left_view);
		output_array_arg!(filtered_disparity_map);
		input_array_arg!(disparity_map_right);
		input_array_arg!(right_view);
		unsafe { sys::cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(self.as_raw_mut_DisparityFilter(), disparity_map_left.as_raw__InputArray(), left_view.as_raw__InputArray(), filtered_disparity_map.as_raw__OutputArray(), disparity_map_right.as_raw__InputArray(), roi.opencv_to_extern(), right_view.as_raw__InputArray()) }.into_result()
	}
	
}

/// Disparity map filter based on Weighted Least Squares filter (in form of Fast Global Smoother that
/// is a lot faster than traditional Weighted Least Squares filter implementations) and optional use of
/// left-right-consistency-based confidence to refine the results in half-occlusions and uniform areas.
pub trait DisparityWLSFilter: crate::ximgproc::DisparityFilter {
	fn as_raw_DisparityWLSFilter(&self) -> *const c_void;
	fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void;

	/// Lambda is a parameter defining the amount of regularization during filtering. Larger values force
	/// filtered disparity map edges to adhere more to source image edges. Typical value is 8000.
	fn get_lambda(&mut self) -> Result<f64> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getLambda(self.as_raw_mut_DisparityWLSFilter()) }.into_result()
	}
	
	/// ## See also
	/// getLambda
	fn set_lambda(&mut self, _lambda: f64) -> Result<()> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setLambda_double(self.as_raw_mut_DisparityWLSFilter(), _lambda) }.into_result()
	}
	
	/// SigmaColor is a parameter defining how sensitive the filtering process is to source image edges.
	/// Large values can lead to disparity leakage through low-contrast edges. Small values can make the filter too
	/// sensitive to noise and textures in the source image. Typical values range from 0.8 to 2.0.
	fn get_sigma_color(&mut self) -> Result<f64> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getSigmaColor(self.as_raw_mut_DisparityWLSFilter()) }.into_result()
	}
	
	/// ## See also
	/// getSigmaColor
	fn set_sigma_color(&mut self, _sigma_color: f64) -> Result<()> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(self.as_raw_mut_DisparityWLSFilter(), _sigma_color) }.into_result()
	}
	
	/// LRCthresh is a threshold of disparity difference used in left-right-consistency check during
	/// confidence map computation. The default value of 24 (1.5 pixels) is virtually always good enough.
	fn get_lr_cthresh(&mut self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getLRCthresh(self.as_raw_mut_DisparityWLSFilter()) }.into_result()
	}
	
	/// ## See also
	/// getLRCthresh
	fn set_lr_cthresh(&mut self, _lrc_thresh: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(self.as_raw_mut_DisparityWLSFilter(), _lrc_thresh) }.into_result()
	}
	
	/// DepthDiscontinuityRadius is a parameter used in confidence computation. It defines the size of
	/// low-confidence regions around depth discontinuities.
	fn get_depth_discontinuity_radius(&mut self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(self.as_raw_mut_DisparityWLSFilter()) }.into_result()
	}
	
	/// ## See also
	/// getDepthDiscontinuityRadius
	fn set_depth_discontinuity_radius(&mut self, _disc_radius: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(self.as_raw_mut_DisparityWLSFilter(), _disc_radius) }.into_result()
	}
	
	/// Get the confidence map that was used in the last filter call. It is a CV_32F one-channel image
	/// with values ranging from 0.0 (totally untrusted regions of the raw disparity map) to 255.0 (regions containing
	/// correct disparity values with a high degree of confidence).
	fn get_confidence_map(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getConfidenceMap(self.as_raw_mut_DisparityWLSFilter()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Get the ROI used in the last filter call
	fn get_roi(&mut self) -> Result<core::Rect> {
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getROI(self.as_raw_mut_DisparityWLSFilter()) }.into_result()
	}
	
}

/// Sparse match interpolation algorithm based on modified locally-weighted affine
/// estimator from [Revaud2015](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Revaud2015) and Fast Global Smoother as post-processing filter.
pub trait EdgeAwareInterpolator: crate::ximgproc::SparseMatchInterpolator {
	fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void;
	fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void;

	/// K is a number of nearest-neighbor matches considered, when fitting a locally affine
	/// model. Usually it should be around 128. However, lower values would make the interpolation
	/// noticeably faster.
	fn set_k(&mut self, _k: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setK_int(self.as_raw_mut_EdgeAwareInterpolator(), _k) }.into_result()
	}
	
	/// ## See also
	/// setK
	fn get_k(&mut self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getK(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
	/// Sigma is a parameter defining how fast the weights decrease in the locally-weighted affine
	/// fitting. Higher values can help preserve fine details, lower values can help to get rid of noise in the
	/// output flow.
	fn set_sigma(&mut self, _sigma: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setSigma_float(self.as_raw_mut_EdgeAwareInterpolator(), _sigma) }.into_result()
	}
	
	/// ## See also
	/// setSigma
	fn get_sigma(&mut self) -> Result<f32> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getSigma(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
	/// Lambda is a parameter defining the weight of the edge-aware term in geodesic distance,
	/// should be in the range of 0 to 1000.
	fn set_lambda(&mut self, _lambda: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setLambda_float(self.as_raw_mut_EdgeAwareInterpolator(), _lambda) }.into_result()
	}
	
	/// ## See also
	/// setLambda
	fn get_lambda(&mut self) -> Result<f32> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getLambda(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
	/// Sets whether the fastGlobalSmootherFilter() post-processing is employed. It is turned on by
	/// default.
	fn set_use_post_processing(&mut self, _use_post_proc: bool) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(self.as_raw_mut_EdgeAwareInterpolator(), _use_post_proc) }.into_result()
	}
	
	/// ## See also
	/// setUsePostProcessing
	fn get_use_post_processing(&mut self) -> Result<bool> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	fn set_fgs_lambda(&mut self, _lambda: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(self.as_raw_mut_EdgeAwareInterpolator(), _lambda) }.into_result()
	}
	
	/// ## See also
	/// setFGSLambda
	fn get_fgs_lambda(&mut self) -> Result<f32> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
	/// ## See also
	/// setFGSLambda
	fn set_fgs_sigma(&mut self, _sigma: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(self.as_raw_mut_EdgeAwareInterpolator(), _sigma) }.into_result()
	}
	
	/// ## See also
	/// setFGSLambda
	fn get_fgs_sigma(&mut self) -> Result<f32> {
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(self.as_raw_mut_EdgeAwareInterpolator()) }.into_result()
	}
	
}

/// Interface for implementations of Fast Global Smoother filter.
/// 
/// For more details about this filter see [Min2014](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Min2014) and [Farbman2008](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Farbman2008) .
pub trait FastGlobalSmootherFilter: core::AlgorithmTrait {
	fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void;
	fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void;

	/// Apply smoothing operation to the source image.
	/// 
	/// ## Parameters
	/// * src: source image for filtering with unsigned 8-bit or signed 16-bit or floating-point 32-bit depth and up to 4 channels.
	/// 
	/// * dst: destination image.
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FastGlobalSmootherFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
}

/// @include samples/fld_lines.cpp
pub trait FastLineDetector: core::AlgorithmTrait {
	fn as_raw_FastLineDetector(&self) -> *const c_void;
	fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void;

	/// @example fld_lines.cpp
	///       An example using the FastLineDetector
	/// 
	/// Finds lines in the input image.
	///       This is the output of the default parameters of the algorithm on the above
	///       shown image.
	/// 
	///       ![image](https://docs.opencv.org/3.2.0/corridor_fld.jpg)
	/// 
	/// ## Parameters
	/// * _image: A grayscale (CV_8UC1) input image. If only a roi needs to be
	///       selected, use: `fld_ptr-\>detect(image(roi), lines, ...);
	///       lines += Scalar(roi.x, roi.y, roi.x, roi.y);`
	/// * _lines: A vector of Vec4f elements specifying the beginning
	///       and ending point of a line.  Where Vec4f is (x1, y1, x2, y2), point
	///       1 is the start, point 2 - end. Returned lines are directed so that the
	///       brighter side is on their left.
	fn detect(&mut self, _image: &dyn core::ToInputArray, _lines: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(_image);
		output_array_arg!(_lines);
		unsafe { sys::cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FastLineDetector(), _image.as_raw__InputArray(), _lines.as_raw__OutputArray()) }.into_result()
	}
	
	/// Draws the line segments on a given image.
	/// ## Parameters
	/// * _image: The image, where the lines will be drawn. Should be bigger
	/// or equal to the image, where the lines were found.
	/// * lines: A vector of the lines that needed to be drawn.
	/// * draw_arrow: If true, arrow heads will be drawn.
	/// 
	/// ## C++ default parameters
	/// * draw_arrow: false
	fn draw_segments(&mut self, _image: &mut dyn core::ToInputOutputArray, lines: &dyn core::ToInputArray, draw_arrow: bool) -> Result<()> {
		input_output_array_arg!(_image);
		input_array_arg!(lines);
		unsafe { sys::cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool(self.as_raw_mut_FastLineDetector(), _image.as_raw__InputOutputArray(), lines.as_raw__InputArray(), draw_arrow) }.into_result()
	}
	
}

/// Interface for realizations of Guided Filter.
/// 
/// For more details about this filter see [Kaiming10](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Kaiming10) .
pub trait GuidedFilter: core::AlgorithmTrait {
	fn as_raw_GuidedFilter(&self) -> *const c_void;
	fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void;

	/// Apply Guided Filter to the filtering image.
	/// 
	/// ## Parameters
	/// * src: filtering image with any numbers of channels.
	/// 
	/// * dst: output image.
	/// 
	/// * dDepth: optional depth of the output image. dDepth can be set to -1, which will be equivalent
	/// to src.depth().
	/// 
	/// ## C++ default parameters
	/// * d_depth: -1
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d_depth: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_GuidedFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d_depth) }.into_result()
	}
	
}

/// !
/// Helper class for training part of [P. Dollar and C. L. Zitnick. Structured Forests for Fast Edge Detection, 2013].
pub trait RFFeatureGetter: core::AlgorithmTrait {
	fn as_raw_RFFeatureGetter(&self) -> *const c_void;
	fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void;

	/// !
	/// * This functions extracts feature channels from src.
	/// * Than StructureEdgeDetection uses this feature space
	/// * to detect edges.
	/// *
	/// * \param src : source image to extract features
	/// * \param features : output n-channel floating point feature matrix.
	/// *
	/// * \param gnrmRad : __rf.options.gradientNormalizationRadius
	/// * \param gsmthRad : __rf.options.gradientSmoothingRadius
	/// * \param shrink : __rf.options.shrinkNumber
	/// * \param outNum : __rf.options.numberOfOutputChannels
	/// * \param gradNum : __rf.options.numberOfGradientOrientations
	fn get_features(&self, src: &core::Mat, features: &mut core::Mat, gnrm_rad: i32, gsmth_rad: i32, shrink: i32, out_num: i32, grad_num: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_int_int_int_int_int(self.as_raw_RFFeatureGetter(), src.as_raw_Mat(), features.as_raw_mut_Mat(), gnrm_rad, gsmth_rad, shrink, out_num, grad_num) }.into_result()
	}
	
}

/// Main interface for all filters, that take sparse matches as an
/// input and produce a dense per-pixel matching (optical flow) as an output.
pub trait SparseMatchInterpolator: core::AlgorithmTrait {
	fn as_raw_SparseMatchInterpolator(&self) -> *const c_void;
	fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void;

	/// Interpolate input sparse matches.
	/// 
	/// ## Parameters
	/// * from_image: first of the two matched images, 8-bit single-channel or three-channel.
	/// 
	/// * from_points: points of the from_image for which there are correspondences in the
	/// to_image (Point2f vector, size shouldn't exceed 32767)
	/// 
	/// * to_image: second of the two matched images, 8-bit single-channel or three-channel.
	/// 
	/// * to_points: points in the to_image corresponding to from_points
	/// (Point2f vector, size shouldn't exceed 32767)
	/// 
	/// * dense_flow: output dense matching (two-channel CV_32F image)
	fn interpolate(&mut self, from_image: &dyn core::ToInputArray, from_points: &dyn core::ToInputArray, to_image: &dyn core::ToInputArray, to_points: &dyn core::ToInputArray, dense_flow: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(from_image);
		input_array_arg!(from_points);
		input_array_arg!(to_image);
		input_array_arg!(to_points);
		output_array_arg!(dense_flow);
		unsafe { sys::cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SparseMatchInterpolator(), from_image.as_raw__InputArray(), from_points.as_raw__InputArray(), to_image.as_raw__InputArray(), to_points.as_raw__InputArray(), dense_flow.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Class implementing edge detection algorithm from [Dollar2013](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Dollar2013) :
pub trait StructuredEdgeDetection: core::AlgorithmTrait {
	fn as_raw_StructuredEdgeDetection(&self) -> *const c_void;
	fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void;

	/// The function detects edges in src and draw them to dst.
	/// 
	/// The algorithm underlies this function is much more robust to texture presence, than common
	/// approaches, e.g. Sobel
	/// ## Parameters
	/// * src: source image (RGB, float, in [0;1]) to detect edges
	/// * dst: destination image (grayscale, float, in [0;1]) where edges are drawn
	/// ## See also
	/// Sobel, Canny
	fn detect_edges(&self, src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const_MatR_MatR(self.as_raw_StructuredEdgeDetection(), src.as_raw_Mat(), dst.as_raw_mut_Mat()) }.into_result()
	}
	
}

/// Class implementing the LSC (Linear Spectral Clustering) superpixels
/// algorithm described in [LiCVPR2015LSC](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_LiCVPR2015LSC).
/// 
/// LSC (Linear Spectral Clustering) produces compact and uniform superpixels with low
/// computational costs. Basically, a normalized cuts formulation of the superpixel
/// segmentation is adopted based on a similarity metric that measures the color
/// similarity and space proximity between image pixels. LSC is of linear computational
/// complexity and high memory efficiency and is able to preserve global properties of images
pub trait SuperpixelLSC: core::AlgorithmTrait {
	fn as_raw_SuperpixelLSC(&self) -> *const c_void;
	fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void;

	/// Calculates the actual amount of superpixels on a given segmentation computed
	/// and stored in SuperpixelLSC object.
	fn get_number_of_superpixels(&self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(self.as_raw_SuperpixelLSC()) }.into_result()
	}
	
	/// Calculates the superpixel segmentation on a given image with the initialized
	/// parameters in the SuperpixelLSC object.
	/// 
	/// This function can be called again without the need of initializing the algorithm with
	/// createSuperpixelLSC(). This save the computational cost of allocating memory for all the
	/// structures of the algorithm.
	/// 
	/// ## Parameters
	/// * num_iterations: Number of iterations. Higher number improves the result.
	/// 
	/// The function computes the superpixels segmentation of an image with the parameters initialized
	/// with the function createSuperpixelLSC(). The algorithms starts from a grid of superpixels and
	/// then refines the boundaries by proposing updates of edges boundaries.
	/// 
	/// ## C++ default parameters
	/// * num_iterations: 10
	fn iterate(&mut self, num_iterations: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_SuperpixelLSC_iterate_int(self.as_raw_mut_SuperpixelLSC(), num_iterations) }.into_result()
	}
	
	/// Returns the segmentation labeling of the image.
	/// 
	/// Each label represents a superpixel, and each pixel is assigned to one superpixel label.
	/// 
	/// ## Parameters
	/// * labels_out: Return: A CV_32SC1 integer array containing the labels of the superpixel
	/// segmentation. The labels are in the range [0, getNumberOfSuperpixels()].
	/// 
	/// The function returns an image with the labels of the superpixel segmentation. The labels are in
	/// the range [0, getNumberOfSuperpixels()].
	fn get_labels(&self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(self.as_raw_SuperpixelLSC(), labels_out.as_raw__OutputArray()) }.into_result()
	}
	
	/// Returns the mask of the superpixel segmentation stored in SuperpixelLSC object.
	/// 
	/// ## Parameters
	/// * image: Return: CV_8U1 image mask where -1 indicates that the pixel is a superpixel border,
	/// and 0 otherwise.
	/// 
	/// * thick_line: If false, the border is only one pixel wide, otherwise all pixels at the border
	/// are masked.
	/// 
	/// The function return the boundaries of the superpixel segmentation.
	/// 
	/// ## C++ default parameters
	/// * thick_line: true
	fn get_label_contour_mask(&self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(self.as_raw_SuperpixelLSC(), image.as_raw__OutputArray(), thick_line) }.into_result()
	}
	
	/// Enforce label connectivity.
	/// 
	/// ## Parameters
	/// * min_element_size: The minimum element size in percents that should be absorbed into a bigger
	/// superpixel. Given resulted average superpixel size valid value should be in 0-100 range, 25 means
	/// that less then a quarter sized superpixel should be absorbed, this is default.
	/// 
	/// The function merge component that is too small, assigning the previously found adjacent label
	/// to this component. Calling this function may change the final number of superpixels.
	/// 
	/// ## C++ default parameters
	/// * min_element_size: 20
	fn enforce_label_connectivity(&mut self, min_element_size: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(self.as_raw_mut_SuperpixelLSC(), min_element_size) }.into_result()
	}
	
}

/// Class implementing the SEEDS (Superpixels Extracted via Energy-Driven Sampling) superpixels
/// algorithm described in [VBRV14](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_VBRV14) .
/// 
/// The algorithm uses an efficient hill-climbing algorithm to optimize the superpixels' energy
/// function that is based on color histograms and a boundary term, which is optional. The energy
/// function encourages superpixels to be of the same color, and if the boundary term is activated, the
/// superpixels have smooth boundaries and are of similar shape. In practice it starts from a regular
/// grid of superpixels and moves the pixels or blocks of pixels at the boundaries to refine the
/// solution. The algorithm runs in real-time using a single CPU.
pub trait SuperpixelSEEDS: core::AlgorithmTrait {
	fn as_raw_SuperpixelSEEDS(&self) -> *const c_void;
	fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void;

	/// Calculates the superpixel segmentation on a given image stored in SuperpixelSEEDS object.
	/// 
	/// The function computes the superpixels segmentation of an image with the parameters initialized
	/// with the function createSuperpixelSEEDS().
	fn get_number_of_superpixels(&mut self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(self.as_raw_mut_SuperpixelSEEDS()) }.into_result()
	}
	
	/// Calculates the superpixel segmentation on a given image with the initialized
	/// parameters in the SuperpixelSEEDS object.
	/// 
	/// This function can be called again for other images without the need of initializing the
	/// algorithm with createSuperpixelSEEDS(). This save the computational cost of allocating memory
	/// for all the structures of the algorithm.
	/// 
	/// ## Parameters
	/// * img: Input image. Supported formats: CV_8U, CV_16U, CV_32F. Image size & number of
	/// channels must match with the initialized image size & channels with the function
	/// createSuperpixelSEEDS(). It should be in HSV or Lab color space. Lab is a bit better, but also
	/// slower.
	/// 
	/// * num_iterations: Number of pixel level iterations. Higher number improves the result.
	/// 
	/// The function computes the superpixels segmentation of an image with the parameters initialized
	/// with the function createSuperpixelSEEDS(). The algorithms starts from a grid of superpixels and
	/// then refines the boundaries by proposing updates of blocks of pixels that lie at the boundaries
	/// from large to smaller size, finalizing with proposing pixel updates. An illustrative example
	/// can be seen below.
	/// 
	/// ![image](https://docs.opencv.org/3.2.0/superpixels_blocks2.png)
	/// 
	/// ## C++ default parameters
	/// * num_iterations: 4
	fn iterate(&mut self, img: &dyn core::ToInputArray, num_iterations: i32) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(self.as_raw_mut_SuperpixelSEEDS(), img.as_raw__InputArray(), num_iterations) }.into_result()
	}
	
	/// Returns the segmentation labeling of the image.
	/// 
	/// Each label represents a superpixel, and each pixel is assigned to one superpixel label.
	/// 
	/// ## Parameters
	/// * labels_out: Return: A CV_32UC1 integer array containing the labels of the superpixel
	/// segmentation. The labels are in the range [0, getNumberOfSuperpixels()].
	/// 
	/// The function returns an image with ssthe labels of the superpixel segmentation. The labels are in
	/// the range [0, getNumberOfSuperpixels()].
	fn get_labels(&mut self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(self.as_raw_mut_SuperpixelSEEDS(), labels_out.as_raw__OutputArray()) }.into_result()
	}
	
	/// Returns the mask of the superpixel segmentation stored in SuperpixelSEEDS object.
	/// 
	/// ## Parameters
	/// * image: Return: CV_8UC1 image mask where -1 indicates that the pixel is a superpixel border,
	/// and 0 otherwise.
	/// 
	/// * thick_line: If false, the border is only one pixel wide, otherwise all pixels at the border
	/// are masked.
	/// 
	/// The function return the boundaries of the superpixel segmentation.
	/// 
	/// 
	/// Note:
	///    *   (Python) A demo on how to generate superpixels in images from the webcam can be found at
	///        opencv_source_code/samples/python2/seeds.py
	///    *   (cpp) A demo on how to generate superpixels in images from the webcam can be found at
	///        opencv_source_code/modules/ximgproc/samples/seeds.cpp. By adding a file image as a command
	///        line argument, the static image will be used instead of the webcam.
	///    *   It will show a window with the video from the webcam with the superpixel boundaries marked
	///        in red (see below). Use Space to switch between different output modes. At the top of the
	///        window there are 4 sliders, from which the user can change on-the-fly the number of
	///        superpixels, the number of block levels, the strength of the boundary prior term to modify
	///        the shape, and the number of iterations at pixel level. This is useful to play with the
	///        parameters and set them to the user convenience. In the console the frame-rate of the
	///        algorithm is indicated.
	/// 
	/// ![image](https://docs.opencv.org/3.2.0/superpixels_demo.png)
	/// 
	/// ## C++ default parameters
	/// * thick_line: false
	fn get_label_contour_mask(&mut self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(self.as_raw_mut_SuperpixelSEEDS(), image.as_raw__OutputArray(), thick_line) }.into_result()
	}
	
}

/// Class implementing the SLIC (Simple Linear Iterative Clustering) superpixels
/// algorithm described in [Achanta2012](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Achanta2012).
/// 
/// SLIC (Simple Linear Iterative Clustering) clusters pixels using pixel channels and image plane space
/// to efficiently generate compact, nearly uniform superpixels. The simplicity of approach makes it
/// extremely easy to use a lone parameter specifies the number of superpixels and the efficiency of
/// the algorithm makes it very practical.
pub trait SuperpixelSLIC: core::AlgorithmTrait {
	fn as_raw_SuperpixelSLIC(&self) -> *const c_void;
	fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void;

	/// Calculates the actual amount of superpixels on a given segmentation computed
	/// and stored in SuperpixelSLIC object.
	fn get_number_of_superpixels(&self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(self.as_raw_SuperpixelSLIC()) }.into_result()
	}
	
	/// Calculates the superpixel segmentation on a given image with the initialized
	/// parameters in the SuperpixelSLIC object.
	/// 
	/// This function can be called again without the need of initializing the algorithm with
	/// createSuperpixelSLIC(). This save the computational cost of allocating memory for all the
	/// structures of the algorithm.
	/// 
	/// ## Parameters
	/// * num_iterations: Number of iterations. Higher number improves the result.
	/// 
	/// The function computes the superpixels segmentation of an image with the parameters initialized
	/// with the function createSuperpixelSLIC(). The algorithms starts from a grid of superpixels and
	/// then refines the boundaries by proposing updates of edges boundaries.
	/// 
	/// ## C++ default parameters
	/// * num_iterations: 10
	fn iterate(&mut self, num_iterations: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_iterate_int(self.as_raw_mut_SuperpixelSLIC(), num_iterations) }.into_result()
	}
	
	/// Returns the segmentation labeling of the image.
	/// 
	/// Each label represents a superpixel, and each pixel is assigned to one superpixel label.
	/// 
	/// ## Parameters
	/// * labels_out: Return: A CV_32SC1 integer array containing the labels of the superpixel
	/// segmentation. The labels are in the range [0, getNumberOfSuperpixels()].
	/// 
	/// The function returns an image with the labels of the superpixel segmentation. The labels are in
	/// the range [0, getNumberOfSuperpixels()].
	fn get_labels(&self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(self.as_raw_SuperpixelSLIC(), labels_out.as_raw__OutputArray()) }.into_result()
	}
	
	/// Returns the mask of the superpixel segmentation stored in SuperpixelSLIC object.
	/// 
	/// ## Parameters
	/// * image: Return: CV_8U1 image mask where -1 indicates that the pixel is a superpixel border,
	/// and 0 otherwise.
	/// 
	/// * thick_line: If false, the border is only one pixel wide, otherwise all pixels at the border
	/// are masked.
	/// 
	/// The function return the boundaries of the superpixel segmentation.
	/// 
	/// ## C++ default parameters
	/// * thick_line: true
	fn get_label_contour_mask(&self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(self.as_raw_SuperpixelSLIC(), image.as_raw__OutputArray(), thick_line) }.into_result()
	}
	
	/// Enforce label connectivity.
	/// 
	/// ## Parameters
	/// * min_element_size: The minimum element size in percents that should be absorbed into a bigger
	/// superpixel. Given resulted average superpixel size valid value should be in 0-100 range, 25 means
	/// that less then a quarter sized superpixel should be absorbed, this is default.
	/// 
	/// The function merge component that is too small, assigning the previously found adjacent label
	/// to this component. Calling this function may change the final number of superpixels.
	/// 
	/// ## C++ default parameters
	/// * min_element_size: 25
	fn enforce_label_connectivity(&mut self, min_element_size: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(self.as_raw_mut_SuperpixelSLIC(), min_element_size) }.into_result()
	}
	
}

/// Graph Based Segmentation Algorithm.
/// The class implements the algorithm described in [PFF2004](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_PFF2004) .
pub trait GraphSegmentation: core::AlgorithmTrait {
	fn as_raw_GraphSegmentation(&self) -> *const c_void;
	fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void;

	/// Segment an image and store output in dst
	/// ## Parameters
	/// * src: The input image. Any number of channel (1 (Eg: Gray), 3 (Eg: RGB), 4 (Eg: RGB-D)) can be provided
	/// * dst: The output segmentation. It's a CV_32SC1 Mat with the same number of cols and rows as input image, with an unique, sequential, id for each pixel.
	fn process_image(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_GraphSegmentation(), src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn set_sigma(&mut self, sigma: f64) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(self.as_raw_mut_GraphSegmentation(), sigma) }.into_result()
	}
	
	fn get_sigma(&mut self) -> Result<f64> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getSigma(self.as_raw_mut_GraphSegmentation()) }.into_result()
	}
	
	fn set_k(&mut self, k: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setK_float(self.as_raw_mut_GraphSegmentation(), k) }.into_result()
	}
	
	fn get_k(&mut self) -> Result<f32> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getK(self.as_raw_mut_GraphSegmentation()) }.into_result()
	}
	
	fn set_min_size(&mut self, min_size: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(self.as_raw_mut_GraphSegmentation(), min_size) }.into_result()
	}
	
	fn get_min_size(&mut self) -> Result<i32> {
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getMinSize(self.as_raw_mut_GraphSegmentation()) }.into_result()
	}
	
}

/// Selective search segmentation algorithm
/// The class implements the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentation: core::AlgorithmTrait {
	fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void;

	/// Set a image used by switch* functions to initialize the class
	/// ## Parameters
	/// * img: The image
	fn set_base_image(&mut self, img: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(self.as_raw_mut_SelectiveSearchSegmentation(), img.as_raw__InputArray()) }.into_result()
	}
	
	/// Initialize the class with the 'Single stragegy' parameters describled in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
	/// ## Parameters
	/// * k: The k parameter for the graph segmentation
	/// * sigma: The sigma parameter for the graph segmentation
	/// 
	/// ## C++ default parameters
	/// * k: 200
	/// * sigma: 0.8f
	fn switch_to_single_strategy(&mut self, k: i32, sigma: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), k, sigma) }.into_result()
	}
	
	/// Initialize the class with the 'Selective search fast' parameters describled in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
	/// ## Parameters
	/// * base_k: The k parameter for the first graph segmentation
	/// * inc_k: The increment of the k parameter for all graph segmentations
	/// * sigma: The sigma parameter for the graph segmentation
	/// 
	/// ## C++ default parameters
	/// * base_k: 150
	/// * inc_k: 150
	/// * sigma: 0.8f
	fn switch_to_selective_search_fast(&mut self, base_k: i32, inc_k: i32, sigma: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), base_k, inc_k, sigma) }.into_result()
	}
	
	/// Initialize the class with the 'Selective search fast' parameters describled in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
	/// ## Parameters
	/// * base_k: The k parameter for the first graph segmentation
	/// * inc_k: The increment of the k parameter for all graph segmentations
	/// * sigma: The sigma parameter for the graph segmentation
	/// 
	/// ## C++ default parameters
	/// * base_k: 150
	/// * inc_k: 150
	/// * sigma: 0.8f
	fn switch_to_selective_search_quality(&mut self, base_k: i32, inc_k: i32, sigma: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), base_k, inc_k, sigma) }.into_result()
	}
	
	/// Add a new image in the list of images to process.
	/// ## Parameters
	/// * img: The image
	fn add_image(&mut self, img: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(self.as_raw_mut_SelectiveSearchSegmentation(), img.as_raw__InputArray()) }.into_result()
	}
	
	/// Clear the list of images to process
	fn clear_images(&mut self) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(self.as_raw_mut_SelectiveSearchSegmentation()) }.into_result()
	}
	
	/// Add a new graph segmentation in the list of graph segementations to process.
	/// ## Parameters
	/// * g: The graph segmentation
	fn add_graph_segmentation(&mut self, mut g: core::Ptr::<dyn crate::ximgproc::GraphSegmentation>) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_Ptr_GraphSegmentation_(self.as_raw_mut_SelectiveSearchSegmentation(), g.as_raw_mut_PtrOfGraphSegmentation()) }.into_result()
	}
	
	/// Clear the list of graph segmentations to process;
	fn clear_graph_segmentations(&mut self) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(self.as_raw_mut_SelectiveSearchSegmentation()) }.into_result()
	}
	
	/// Add a new strategy in the list of strategy to process.
	/// ## Parameters
	/// * s: The strategy
	fn add_strategy(&mut self, mut s: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_Ptr_SelectiveSearchSegmentationStrategy_(self.as_raw_mut_SelectiveSearchSegmentation(), s.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy()) }.into_result()
	}
	
	/// Clear the list of strategy to process;
	fn clear_strategies(&mut self) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(self.as_raw_mut_SelectiveSearchSegmentation()) }.into_result()
	}
	
	/// Based on all images, graph segmentations and stragies, computes all possible rects and return them
	/// ## Parameters
	/// * rects: The list of rects. The first ones are more relevents than the lasts ones.
	fn process(&mut self, rects: &mut core::Vector::<core::Rect>) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vector_Rect_R(self.as_raw_mut_SelectiveSearchSegmentation(), rects.as_raw_mut_VectorOfRect()) }.into_result()
	}
	
}

/// Strategie for the selective search segmentation algorithm
/// The class implements a generic stragery for the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentationStrategy: core::AlgorithmTrait {
	fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void;

	/// Set a initial image, with a segementation.
	/// ## Parameters
	/// * img: The input image. Any number of channel can be provided
	/// * regions: A segementation of the image. The parameter must be the same size of img.
	/// * sizes: The sizes of different regions
	/// * image_id: If not set to -1, try to cache pre-computations. If the same set og (img, regions, size) is used, the image_id need to be the same.
	/// 
	/// ## C++ default parameters
	/// * image_id: -1
	fn set_image(&mut self, img: &dyn core::ToInputArray, regions: &dyn core::ToInputArray, sizes: &dyn core::ToInputArray, image_id: i32) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(regions);
		input_array_arg!(sizes);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), img.as_raw__InputArray(), regions.as_raw__InputArray(), sizes.as_raw__InputArray(), image_id) }.into_result()
	}
	
	/// Return the score between two regions (between 0 and 1)
	/// ## Parameters
	/// * r1: The first region
	/// * r2: The second region
	fn get(&mut self, r1: i32, r2: i32) -> Result<f32> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), r1, r2) }.into_result()
	}
	
	/// Inform the strategy that two regions will be merged
	/// ## Parameters
	/// * r1: The first region
	/// * r2: The second region
	fn merge(&mut self, r1: i32, r2: i32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), r1, r2) }.into_result()
	}
	
}

/// Color-based strategy for the selective search segmentation algorithm
/// The class is implemented from the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentationStrategyColor: crate::ximgproc::SelectiveSearchSegmentationStrategy {
	fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void;

}

/// Fill-based strategy for the selective search segmentation algorithm
/// The class is implemented from the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentationStrategyFill: crate::ximgproc::SelectiveSearchSegmentationStrategy {
	fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void;

}

/// Regroup multiple strategies for the selective search segmentation algorithm
pub trait SelectiveSearchSegmentationStrategyMultiple: crate::ximgproc::SelectiveSearchSegmentationStrategy {
	fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void;

	/// Add a new sub-strategy
	/// ## Parameters
	/// * g: The strategy
	/// * weight: The weight of the strategy
	fn add_strategy(&mut self, mut g: core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, weight: f32) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_Ptr_SelectiveSearchSegmentationStrategy__float(self.as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(), g.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), weight) }.into_result()
	}
	
	/// Remove all sub-strategies
	fn clear_strategies(&mut self) -> Result<()> {
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(self.as_raw_mut_SelectiveSearchSegmentationStrategyMultiple()) }.into_result()
	}
	
}

/// Size-based strategy for the selective search segmentation algorithm
/// The class is implemented from the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentationStrategySize: crate::ximgproc::SelectiveSearchSegmentationStrategy {
	fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void;

}

/// Texture-based strategy for the selective search segmentation algorithm
/// The class is implemented from the algorithm described in [uijlings2013selective](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_uijlings2013selective).
pub trait SelectiveSearchSegmentationStrategyTexture: crate::ximgproc::SelectiveSearchSegmentationStrategy {
	fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void;
	fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void;

}
