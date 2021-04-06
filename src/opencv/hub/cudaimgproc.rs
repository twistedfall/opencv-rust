#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Image Processing
//!    # Color space processing
//!    # Histogram Calculation
//!    # Hough Transform
//!    # Feature Detection
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CUDA_CLAHE, super::CUDA_CannyEdgeDetector, super::CUDA_HoughLinesDetector, super::CUDA_HoughSegmentDetector, super::CUDA_HoughCirclesDetector, super::CUDA_CornernessCriteria, super::CUDA_CornersDetector, super::CUDA_TemplateMatching };
}

pub const CUDA_ALPHA_ATOP: i32 = 3;
pub const CUDA_ALPHA_ATOP_PREMUL: i32 = 9;
pub const CUDA_ALPHA_IN: i32 = 1;
pub const CUDA_ALPHA_IN_PREMUL: i32 = 7;
pub const CUDA_ALPHA_OUT: i32 = 2;
pub const CUDA_ALPHA_OUT_PREMUL: i32 = 8;
pub const CUDA_ALPHA_OVER: i32 = 0;
pub const CUDA_ALPHA_OVER_PREMUL: i32 = 6;
pub const CUDA_ALPHA_PLUS: i32 = 5;
pub const CUDA_ALPHA_PLUS_PREMUL: i32 = 11;
pub const CUDA_ALPHA_PREMUL: i32 = 12;
pub const CUDA_ALPHA_XOR: i32 = 4;
pub const CUDA_ALPHA_XOR_PREMUL: i32 = 10;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerBG2BGR_MHT: i32 = 256;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerBG2GRAY_MHT: i32 = 260;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerBG2RGB_MHT: i32 = 258;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGB2BGR_MHT: i32 = 257;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGB2GRAY_MHT: i32 = 261;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGB2RGB_MHT: i32 = 259;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGR2BGR_MHT: i32 = 259;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGR2GRAY_MHT: i32 = 263;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerGR2RGB_MHT: i32 = 257;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerRG2BGR_MHT: i32 = 258;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerRG2GRAY_MHT: i32 = 262;
/// Bayer Demosaicing (Malvar, He, and Cutler)
pub const CUDA_COLOR_BayerRG2RGB_MHT: i32 = 256;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CUDA_AlphaCompTypes {
	ALPHA_OVER = 0,
	ALPHA_IN = 1,
	ALPHA_OUT = 2,
	ALPHA_ATOP = 3,
	ALPHA_XOR = 4,
	ALPHA_PLUS = 5,
	ALPHA_OVER_PREMUL = 6,
	ALPHA_IN_PREMUL = 7,
	ALPHA_OUT_PREMUL = 8,
	ALPHA_ATOP_PREMUL = 9,
	ALPHA_XOR_PREMUL = 10,
	ALPHA_PLUS_PREMUL = 11,
	ALPHA_PREMUL = 12,
}

opencv_type_enum! { crate::cudaimgproc::CUDA_AlphaCompTypes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CUDA_DemosaicTypes {
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerBG2BGR_MHT = 256,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerGB2BGR_MHT = 257,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerRG2BGR_MHT = 258,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerGR2BGR_MHT = 259,
	// Bayer Demosaicing (Malvar, He, and Cutler)
	// COLOR_BayerBG2RGB_MHT = 258 as isize, // duplicate discriminant
	// Bayer Demosaicing (Malvar, He, and Cutler)
	// COLOR_BayerGB2RGB_MHT = 259 as isize, // duplicate discriminant
	// Bayer Demosaicing (Malvar, He, and Cutler)
	// COLOR_BayerRG2RGB_MHT = 256 as isize, // duplicate discriminant
	// Bayer Demosaicing (Malvar, He, and Cutler)
	// COLOR_BayerGR2RGB_MHT = 257 as isize, // duplicate discriminant
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerBG2GRAY_MHT = 260,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerGB2GRAY_MHT = 261,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerRG2GRAY_MHT = 262,
	/// Bayer Demosaicing (Malvar, He, and Cutler)
	COLOR_BayerGR2GRAY_MHT = 263,
}

opencv_type_enum! { crate::cudaimgproc::CUDA_DemosaicTypes }

/// Composites two images using alpha opacity values contained in each image.
/// 
/// ## Parameters
/// * img1: First image. Supports CV_8UC4 , CV_16UC4 , CV_32SC4 and CV_32FC4 types.
/// * img2: Second image. Must have the same size and the same type as img1 .
/// * dst: Destination image.
/// * alpha_op: Flag specifying the alpha-blending operation:
/// *   **ALPHA_OVER**
/// *   **ALPHA_IN**
/// *   **ALPHA_OUT**
/// *   **ALPHA_ATOP**
/// *   **ALPHA_XOR**
/// *   **ALPHA_PLUS**
/// *   **ALPHA_OVER_PREMUL**
/// *   **ALPHA_IN_PREMUL**
/// *   **ALPHA_OUT_PREMUL**
/// *   **ALPHA_ATOP_PREMUL**
/// *   **ALPHA_XOR_PREMUL**
/// *   **ALPHA_PLUS_PREMUL**
/// *   **ALPHA_PREMUL**
/// * stream: Stream for the asynchronous version.
/// 
/// 
/// Note:
///    *   An example demonstrating the use of alphaComp can be found at
///        opencv_source_code/samples/gpu/alpha_comp.cpp
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn alpha_comp(img1: &dyn core::ToInputArray, img2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha_op: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(img1.as_raw__InputArray(), img2.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha_op, stream.as_raw_mut_Stream()) }.into_result()
}

/// Performs bilateral filtering of passed image
/// 
/// ## Parameters
/// * src: Source image. Supports only (channels != 2 && depth() != CV_8S && depth() != CV_32S
/// && depth() != CV_64F).
/// * dst: Destination imagwe.
/// * kernel_size: Kernel window size.
/// * sigma_color: Filter sigma in the color space.
/// * sigma_spatial: Filter sigma in the coordinate space.
/// * borderMode: Border type. See borderInterpolate for details. BORDER_REFLECT101 ,
/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
/// * stream: Stream for the asynchronous version.
/// ## See also
/// bilateralFilter
/// 
/// ## C++ default parameters
/// * border_mode: BORDER_DEFAULT
/// * stream: Stream::Null()
pub fn bilateral_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, kernel_size: i32, sigma_color: f32, sigma_spatial: f32, border_mode: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), kernel_size, sigma_color, sigma_spatial, border_mode, stream.as_raw_mut_Stream()) }.into_result()
}

/// Performs linear blending of two images.
/// 
/// ## Parameters
/// * img1: First image. Supports only CV_8U and CV_32F depth.
/// * img2: Second image. Must have the same size and the same type as img1 .
/// * weights1: Weights for first image. Must have tha same size as img1 . Supports only CV_32F
/// type.
/// * weights2: Weights for second image. Must have tha same size as img2 . Supports only CV_32F
/// type.
/// * result: Destination image.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn blend_linear(img1: &dyn core::ToInputArray, img2: &dyn core::ToInputArray, weights1: &dyn core::ToInputArray, weights2: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_array_arg!(weights1);
	input_array_arg!(weights2);
	output_array_arg!(result);
	unsafe { sys::cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(img1.as_raw__InputArray(), img2.as_raw__InputArray(), weights1.as_raw__InputArray(), weights2.as_raw__InputArray(), result.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Calculates histogram for one channel 8-bit image confined in given mask.
/// 
/// ## Parameters
/// * src: Source image with CV_8UC1 type.
/// * hist: Destination histogram with one row, 256 columns, and the CV_32SC1 type.
/// * mask: A mask image same size as src and of type CV_8UC1.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn calc_hist_1(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, hist: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	output_array_arg!(hist);
	unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), mask.as_raw__InputArray(), hist.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Calculates histogram for one channel 8-bit image.
/// 
/// ## Parameters
/// * src: Source image with CV_8UC1 type.
/// * hist: Destination histogram with one row, 256 columns, and the CV_32SC1 type.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn calc_hist(src: &dyn core::ToInputArray, hist: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(hist);
	unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Creates implementation for cuda::CLAHE .
/// 
/// ## Parameters
/// * clipLimit: Threshold for contrast limiting.
/// * tileGridSize: Size of grid for histogram equalization. Input image will be divided into
/// equally sized rectangular tiles. tileGridSize defines the number of tiles in row and column.
/// 
/// ## C++ default parameters
/// * clip_limit: 40.0
/// * tile_grid_size: Size(8,8)
pub fn create_clahe(clip_limit: f64, tile_grid_size: core::Size) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_CLAHE>> {
	unsafe { sys::cv_cuda_createCLAHE_double_Size(clip_limit, tile_grid_size.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_CLAHE>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::CannyEdgeDetector .
/// 
/// ## Parameters
/// * low_thresh: First threshold for the hysteresis procedure.
/// * high_thresh: Second threshold for the hysteresis procedure.
/// * apperture_size: Aperture size for the Sobel operator.
/// * L2gradient: Flag indicating whether a more accurate ![inline formula](https://latex.codecogs.com/png.latex?L%5F2) norm
/// ![inline formula](https://latex.codecogs.com/png.latex?%3D%5Csqrt%7B%28dI%2Fdx%29%5E2%20%2B%20%28dI%2Fdy%29%5E2%7D) should be used to compute the image gradient magnitude (
/// L2gradient=true ), or a faster default ![inline formula](https://latex.codecogs.com/png.latex?L%5F1) norm ![inline formula](https://latex.codecogs.com/png.latex?%3D%7CdI%2Fdx%7C%2B%7CdI%2Fdy%7C) is enough ( L2gradient=false
/// ).
/// 
/// ## C++ default parameters
/// * apperture_size: 3
/// * l2gradient: false
pub fn create_canny_edge_detector(low_thresh: f64, high_thresh: f64, apperture_size: i32, l2gradient: bool) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_CannyEdgeDetector>> {
	unsafe { sys::cv_cuda_createCannyEdgeDetector_double_double_int_bool(low_thresh, high_thresh, apperture_size, l2gradient) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_CannyEdgeDetector>::opencv_from_extern(r) } )
}

/// Creates implementation for generalized hough transform from [Ballard1981](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_Ballard1981) .
pub fn create_generalized_hough_ballard() -> Result<core::Ptr::<dyn crate::imgproc::GeneralizedHoughBallard>> {
	unsafe { sys::cv_cuda_createGeneralizedHoughBallard() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::imgproc::GeneralizedHoughBallard>::opencv_from_extern(r) } )
}

/// Creates implementation for generalized hough transform from [Guil1999](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_Guil1999) .
pub fn create_generalized_hough_guil() -> Result<core::Ptr::<dyn crate::imgproc::GeneralizedHoughGuil>> {
	unsafe { sys::cv_cuda_createGeneralizedHoughGuil() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::imgproc::GeneralizedHoughGuil>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::CornersDetector .
/// 
/// ## Parameters
/// * srcType: Input source type. Only CV_8UC1 and CV_32FC1 are supported for now.
/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
/// the strongest of them is returned.
/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
/// (see cornerMinEigenVal ) or the Harris function response (see cornerHarris ). The corners with the
/// quality measure less than the product are rejected. For example, if the best corner has the
/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
/// less than 15 are rejected.
/// * minDistance: Minimum possible Euclidean distance between the returned corners.
/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
/// pixel neighborhood. See cornerEigenValsAndVecs .
/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see cornerHarris)
/// or cornerMinEigenVal.
/// * harrisK: Free parameter of the Harris detector.
/// 
/// ## C++ default parameters
/// * max_corners: 1000
/// * quality_level: 0.01
/// * min_distance: 0.0
/// * block_size: 3
/// * use_harris_detector: false
/// * harris_k: 0.04
pub fn create_good_features_to_track_detector(src_type: i32, max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, harris_k: f64) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_CornersDetector>> {
	unsafe { sys::cv_cuda_createGoodFeaturesToTrackDetector_int_int_double_double_int_bool_double(src_type, max_corners, quality_level, min_distance, block_size, use_harris_detector, harris_k) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_CornersDetector>::opencv_from_extern(r) } )
}

/// Creates implementation for Harris cornerness criteria.
/// 
/// ## Parameters
/// * srcType: Input source type. Only CV_8UC1 and CV_32FC1 are supported for now.
/// * blockSize: Neighborhood size.
/// * ksize: Aperture parameter for the Sobel operator.
/// * k: Harris detector free parameter.
/// * borderType: Pixel extrapolation method. Only BORDER_REFLECT101 and BORDER_REPLICATE are
/// supported for now.
/// ## See also
/// cornerHarris
/// 
/// ## C++ default parameters
/// * border_type: BORDER_REFLECT101
pub fn create_harris_corner(src_type: i32, block_size: i32, ksize: i32, k: f64, border_type: i32) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_CornernessCriteria>> {
	unsafe { sys::cv_cuda_createHarrisCorner_int_int_int_double_int(src_type, block_size, ksize, k, border_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::HoughCirclesDetector .
/// 
/// ## Parameters
/// * dp: Inverse ratio of the accumulator resolution to the image resolution. For example, if
/// dp=1 , the accumulator has the same resolution as the input image. If dp=2 , the accumulator has
/// half as big width and height.
/// * minDist: Minimum distance between the centers of the detected circles. If the parameter is
/// too small, multiple neighbor circles may be falsely detected in addition to a true one. If it is
/// too large, some circles may be missed.
/// * cannyThreshold: The higher threshold of the two passed to Canny edge detector (the lower one
/// is twice smaller).
/// * votesThreshold: The accumulator threshold for the circle centers at the detection stage. The
/// smaller it is, the more false circles may be detected.
/// * minRadius: Minimum circle radius.
/// * maxRadius: Maximum circle radius.
/// * maxCircles: Maximum number of output circles.
/// 
/// ## C++ default parameters
/// * max_circles: 4096
pub fn create_hough_circles_detector(dp: f32, min_dist: f32, canny_threshold: i32, votes_threshold: i32, min_radius: i32, max_radius: i32, max_circles: i32) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughCirclesDetector>> {
	unsafe { sys::cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int_int(dp, min_dist, canny_threshold, votes_threshold, min_radius, max_radius, max_circles) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughCirclesDetector>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::HoughLinesDetector .
/// 
/// ## Parameters
/// * rho: Distance resolution of the accumulator in pixels.
/// * theta: Angle resolution of the accumulator in radians.
/// * threshold: Accumulator threshold parameter. Only those lines are returned that get enough
/// votes ( ![inline formula](https://latex.codecogs.com/png.latex?%3E%5Ctexttt%7Bthreshold%7D) ).
/// * doSort: Performs lines sort by votes.
/// * maxLines: Maximum number of output lines.
/// 
/// ## C++ default parameters
/// * do_sort: false
/// * max_lines: 4096
pub fn create_hough_lines_detector(rho: f32, theta: f32, threshold: i32, do_sort: bool, max_lines: i32) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughLinesDetector>> {
	unsafe { sys::cv_cuda_createHoughLinesDetector_float_float_int_bool_int(rho, theta, threshold, do_sort, max_lines) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughLinesDetector>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::HoughSegmentDetector .
/// 
/// ## Parameters
/// * rho: Distance resolution of the accumulator in pixels.
/// * theta: Angle resolution of the accumulator in radians.
/// * minLineLength: Minimum line length. Line segments shorter than that are rejected.
/// * maxLineGap: Maximum allowed gap between points on the same line to link them.
/// * maxLines: Maximum number of output lines.
/// 
/// ## C++ default parameters
/// * max_lines: 4096
pub fn create_hough_segment_detector(rho: f32, theta: f32, min_line_length: i32, max_line_gap: i32, max_lines: i32) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughSegmentDetector>> {
	unsafe { sys::cv_cuda_createHoughSegmentDetector_float_float_int_int_int(rho, theta, min_line_length, max_line_gap, max_lines) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_HoughSegmentDetector>::opencv_from_extern(r) } )
}

/// Creates implementation for the minimum eigen value of a 2x2 derivative covariation matrix (the
/// cornerness criteria).
/// 
/// ## Parameters
/// * srcType: Input source type. Only CV_8UC1 and CV_32FC1 are supported for now.
/// * blockSize: Neighborhood size.
/// * ksize: Aperture parameter for the Sobel operator.
/// * borderType: Pixel extrapolation method. Only BORDER_REFLECT101 and BORDER_REPLICATE are
/// supported for now.
/// ## See also
/// cornerMinEigenVal
/// 
/// ## C++ default parameters
/// * border_type: BORDER_REFLECT101
pub fn create_min_eigen_val_corner(src_type: i32, block_size: i32, ksize: i32, border_type: i32) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_CornernessCriteria>> {
	unsafe { sys::cv_cuda_createMinEigenValCorner_int_int_int_int(src_type, block_size, ksize, border_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(r) } )
}

/// Creates implementation for cuda::TemplateMatching .
/// 
/// ## Parameters
/// * srcType: Input source type. CV_32F and CV_8U depth images (1..4 channels) are supported
/// for now.
/// * method: Specifies the way to compare the template with the image.
/// * user_block_size: You can use field user_block_size to set specific block size. If you
/// leave its default value Size(0,0) then automatic estimation of block size will be used (which is
/// optimized for speed). By varying user_block_size you can reduce memory requirements at the cost
/// of speed.
/// 
/// The following methods are supported for the CV_8U depth images for now:
/// 
/// *   CV_TM_SQDIFF
/// *   CV_TM_SQDIFF_NORMED
/// *   CV_TM_CCORR
/// *   CV_TM_CCORR_NORMED
/// *   CV_TM_CCOEFF
/// *   CV_TM_CCOEFF_NORMED
/// 
/// The following methods are supported for the CV_32F images for now:
/// 
/// *   CV_TM_SQDIFF
/// *   CV_TM_CCORR
/// ## See also
/// matchTemplate
/// 
/// ## C++ default parameters
/// * user_block_size: Size()
pub fn create_template_matching(src_type: i32, method: i32, user_block_size: core::Size) -> Result<core::Ptr::<dyn crate::cudaimgproc::CUDA_TemplateMatching>> {
	unsafe { sys::cv_cuda_createTemplateMatching_int_int_Size(src_type, method, user_block_size.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaimgproc::CUDA_TemplateMatching>::opencv_from_extern(r) } )
}

/// Converts an image from one color space to another.
/// 
/// ## Parameters
/// * src: Source image with CV_8U , CV_16U , or CV_32F depth and 1, 3, or 4 channels.
/// * dst: Destination image.
/// * code: Color space conversion code. For details, see cvtColor .
/// * dcn: Number of channels in the destination image. If the parameter is 0, the number of the
/// channels is derived automatically from src and the code .
/// * stream: Stream for the asynchronous version.
/// 
/// 3-channel color spaces (like HSV, XYZ, and so on) can be stored in a 4-channel image for better
/// performance.
/// ## See also
/// cvtColor
/// 
/// ## C++ default parameters
/// * dcn: 0
/// * stream: Stream::Null()
pub fn cvt_color(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, code: i32, dcn: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, dcn, stream.as_raw_mut_Stream()) }.into_result()
}

/// Converts an image from Bayer pattern to RGB or grayscale.
/// 
/// ## Parameters
/// * src: Source image (8-bit or 16-bit single channel).
/// * dst: Destination image.
/// * code: Color space conversion code (see the description below).
/// * dcn: Number of channels in the destination image. If the parameter is 0, the number of the
/// channels is derived automatically from src and the code .
/// * stream: Stream for the asynchronous version.
/// 
/// The function can do the following transformations:
/// 
/// *   Demosaicing using bilinear interpolation
/// 
///    > -   COLOR_BayerBG2GRAY , COLOR_BayerGB2GRAY , COLOR_BayerRG2GRAY , COLOR_BayerGR2GRAY
///    > -   COLOR_BayerBG2BGR , COLOR_BayerGB2BGR , COLOR_BayerRG2BGR , COLOR_BayerGR2BGR
/// 
/// *   Demosaicing using Malvar-He-Cutler algorithm ([MHT2011](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_MHT2011))
/// 
///    > -   COLOR_BayerBG2GRAY_MHT , COLOR_BayerGB2GRAY_MHT , COLOR_BayerRG2GRAY_MHT ,
///    >     COLOR_BayerGR2GRAY_MHT
///    > -   COLOR_BayerBG2BGR_MHT , COLOR_BayerGB2BGR_MHT , COLOR_BayerRG2BGR_MHT ,
///    >     COLOR_BayerGR2BGR_MHT
/// ## See also
/// cvtColor
/// 
/// ## C++ default parameters
/// * dcn: -1
/// * stream: Stream::Null()
pub fn demosaicing(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, code: i32, dcn: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, dcn, stream.as_raw_mut_Stream()) }.into_result()
}

/// Equalizes the histogram of a grayscale image.
/// 
/// ## Parameters
/// * src: Source image with CV_8UC1 type.
/// * dst: Destination image.
/// * stream: Stream for the asynchronous version.
/// ## See also
/// equalizeHist
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn equalize_hist(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Computes levels with even distribution.
/// 
/// ## Parameters
/// * levels: Destination array. levels has 1 row, nLevels columns, and the CV_32SC1 type.
/// * nLevels: Number of computed levels. nLevels must be at least 2.
/// * lowerLevel: Lower boundary value of the lowest level.
/// * upperLevel: Upper boundary value of the greatest level.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn even_levels(levels: &mut dyn core::ToOutputArray, n_levels: i32, lower_level: i32, upper_level: i32, stream: &mut core::Stream) -> Result<()> {
	output_array_arg!(levels);
	unsafe { sys::cv_cuda_evenLevels_const__OutputArrayR_int_int_int_StreamR(levels.as_raw__OutputArray(), n_levels, lower_level, upper_level, stream.as_raw_mut_Stream()) }.into_result()
}

/// Routines for correcting image color gamma.
/// 
/// ## Parameters
/// * src: Source image (3- or 4-channel 8 bit).
/// * dst: Destination image.
/// * forward: true for forward gamma correction or false for inverse gamma correction.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * forward: true
/// * stream: Stream::Null()
pub fn gamma_correction(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, forward: bool, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR_bool_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), forward, stream.as_raw_mut_Stream()) }.into_result()
}

/// Calculates a histogram with evenly distributed bins.
/// 
/// ## Parameters
/// * src: Source image. CV_8U, CV_16U, or CV_16S depth and 1 or 4 channels are supported. For
/// a four-channel image, all channels are processed separately.
/// * hist: Destination histogram with one row, histSize columns, and the CV_32S type.
/// * histSize: Size of the histogram.
/// * lowerLevel: Lower boundary of lowest-level bin.
/// * upperLevel: Upper boundary of highest-level bin.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn hist_even(src: &dyn core::ToInputArray, hist: &mut dyn core::ToOutputArray, hist_size: i32, lower_level: i32, upper_level: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(hist);
	unsafe { sys::cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), hist_size, lower_level, upper_level, stream.as_raw_mut_Stream()) }.into_result()
}

/// Calculates a histogram with bins determined by the levels array.
/// 
/// ## Parameters
/// * src: Source image. CV_8U , CV_16U , or CV_16S depth and 1 or 4 channels are supported.
/// For a four-channel image, all channels are processed separately.
/// * hist: Destination histogram with one row, (levels.cols-1) columns, and the CV_32SC1 type.
/// * levels: Number of levels in the histogram.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn hist_range(src: &dyn core::ToInputArray, hist: &mut dyn core::ToOutputArray, levels: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(hist);
	input_array_arg!(levels);
	unsafe { sys::cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), levels.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Performs mean-shift filtering for each point of the source image.
/// 
/// ## Parameters
/// * src: Source image. Only CV_8UC4 images are supported for now.
/// * dst: Destination image containing the color of mapped points. It has the same size and type
/// as src .
/// * sp: Spatial window radius.
/// * sr: Color window radius.
/// * criteria: Termination criteria. See TermCriteria.
/// * stream: Stream for the asynchronous version.
/// 
/// It maps each point of the source image into another point. As a result, you have a new color and new
/// position of each point.
/// 
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
/// * stream: Stream::Null()
pub fn mean_shift_filtering(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sp: i32, sr: i32, criteria: core::TermCriteria, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, criteria.opencv_as_extern(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Performs a mean-shift procedure and stores information about processed points (their colors and
/// positions) in two images.
/// 
/// ## Parameters
/// * src: Source image. Only CV_8UC4 images are supported for now.
/// * dstr: Destination image containing the color of mapped points. The size and type is the same
/// as src .
/// * dstsp: Destination image containing the position of mapped points. The size is the same as
/// src size. The type is CV_16SC2 .
/// * sp: Spatial window radius.
/// * sr: Color window radius.
/// * criteria: Termination criteria. See TermCriteria.
/// * stream: Stream for the asynchronous version.
/// ## See also
/// cuda::meanShiftFiltering
/// 
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
/// * stream: Stream::Null()
pub fn mean_shift_proc(src: &dyn core::ToInputArray, dstr: &mut dyn core::ToOutputArray, dstsp: &mut dyn core::ToOutputArray, sp: i32, sr: i32, criteria: core::TermCriteria, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dstr);
	output_array_arg!(dstsp);
	unsafe { sys::cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dstr.as_raw__OutputArray(), dstsp.as_raw__OutputArray(), sp, sr, criteria.opencv_as_extern(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Performs a mean-shift segmentation of the source image and eliminates small segments.
/// 
/// ## Parameters
/// * src: Source image. Only CV_8UC4 images are supported for now.
/// * dst: Segmented image with the same size and type as src (host or gpu memory).
/// * sp: Spatial window radius.
/// * sr: Color window radius.
/// * minsize: Minimum segment size. Smaller segments are merged.
/// * criteria: Termination criteria. See TermCriteria.
/// * stream: Stream for the asynchronous version.
/// 
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
/// * stream: Stream::Null()
pub fn mean_shift_segmentation(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sp: i32, sr: i32, minsize: i32, criteria: core::TermCriteria, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, minsize, criteria.opencv_as_extern(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Exchanges the color channels of an image in-place.
/// 
/// ## Parameters
/// * image: Source image. Supports only CV_8UC4 type.
/// * dstOrder: Integer array describing how channel values are permutated. The n-th entry of the
/// array contains the number of the channel that is stored in the n-th channel of the output image.
/// E.g. Given an RGBA image, aDstOrder = [3,2,1,0] converts this to ABGR channel order.
/// * stream: Stream for the asynchronous version.
/// 
/// The methods support arbitrary permutations of the original channels, including replication.
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn swap_channels(image: &mut dyn core::ToInputOutputArray, dst_order: &[i32; 4], stream: &mut core::Stream) -> Result<()> {
	input_output_array_arg!(image);
	unsafe { sys::cv_cuda_swapChannels_const__InputOutputArrayR_const_int_X__4__StreamR(image.as_raw__InputOutputArray(), dst_order, stream.as_raw_mut_Stream()) }.into_result()
}

/// Base class for Contrast Limited Adaptive Histogram Equalization. :
pub trait CUDA_CLAHE: crate::imgproc::CLAHE {
	fn as_raw_CUDA_CLAHE(&self) -> *const c_void;
	fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void;

	/// Equalizes the histogram of a grayscale image using Contrast Limited Adaptive Histogram Equalization.
	/// 
	/// ## Parameters
	/// * src: Source image with CV_8UC1 type.
	/// * dst: Destination image.
	/// * stream: Stream for the asynchronous version.
	fn apply(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_CLAHE_apply_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CLAHE(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}

/// Base class for Canny Edge Detector. :
pub trait CUDA_CannyEdgeDetector: core::AlgorithmTrait {
	fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void;

	/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
	/// 
	/// ## Parameters
	/// * image: Single-channel 8-bit input image.
	/// * edges: Output edge map. It has the same size and type as image.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn detect(&mut self, image: &dyn core::ToInputArray, edges: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(edges);
		unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CannyEdgeDetector(), image.as_raw__InputArray(), edges.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
	/// 
	/// ## Parameters
	/// * image: Single-channel 8-bit input image.
	/// * edges: Output edge map. It has the same size and type as image.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * dx: First derivative of image in the vertical direction. Support only CV_32S type.
	/// * dy: First derivative of image in the horizontal direction. Support only CV_32S type.
	/// * edges: Output edge map. It has the same size and type as image.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn detect_1(&mut self, dx: &dyn core::ToInputArray, dy: &dyn core::ToInputArray, edges: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(dx);
		input_array_arg!(dy);
		output_array_arg!(edges);
		unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CannyEdgeDetector(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), edges.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn set_low_threshold(&mut self, low_thresh: f64) -> Result<()> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_setLowThreshold_double(self.as_raw_mut_CUDA_CannyEdgeDetector(), low_thresh) }.into_result()
	}
	
	fn get_low_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_getLowThreshold_const(self.as_raw_CUDA_CannyEdgeDetector()) }.into_result()
	}
	
	fn set_high_threshold(&mut self, high_thresh: f64) -> Result<()> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_setHighThreshold_double(self.as_raw_mut_CUDA_CannyEdgeDetector(), high_thresh) }.into_result()
	}
	
	fn get_high_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_getHighThreshold_const(self.as_raw_CUDA_CannyEdgeDetector()) }.into_result()
	}
	
	fn set_apperture_size(&mut self, apperture_size: i32) -> Result<()> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_setAppertureSize_int(self.as_raw_mut_CUDA_CannyEdgeDetector(), apperture_size) }.into_result()
	}
	
	fn get_apperture_size(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_getAppertureSize_const(self.as_raw_CUDA_CannyEdgeDetector()) }.into_result()
	}
	
	fn set_l2_gradient(&mut self, l2gradient: bool) -> Result<()> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_setL2Gradient_bool(self.as_raw_mut_CUDA_CannyEdgeDetector(), l2gradient) }.into_result()
	}
	
	fn get_l2_gradient(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_CannyEdgeDetector_getL2Gradient_const(self.as_raw_CUDA_CannyEdgeDetector()) }.into_result()
	}
	
}

/// Base class for Cornerness Criteria computation. :
pub trait CUDA_CornernessCriteria: core::AlgorithmTrait {
	fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void;
	fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void;

	/// Computes the cornerness criteria at each image pixel.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image containing cornerness values. It will have the same size as src and
	/// CV_32FC1 type.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn compute(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CornernessCriteria(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}

/// Base class for Corners Detector. :
pub trait CUDA_CornersDetector: core::AlgorithmTrait {
	fn as_raw_CUDA_CornersDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void;

	/// Determines strong corners on an image.
	/// 
	/// ## Parameters
	/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
	/// * corners: Output vector of detected corners (1-row matrix with CV_32FC2 type with corners
	/// positions).
	/// * mask: Optional region of interest. If the image is not empty (it needs to have the type
	/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	fn detect(&mut self, image: &dyn core::ToInputArray, corners: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_CornersDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}

/// Base class for circles detector algorithm. :
pub trait CUDA_HoughCirclesDetector: core::AlgorithmTrait {
	fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void;

	/// Finds circles in a grayscale image using the Hough transform.
	/// 
	/// ## Parameters
	/// * src: 8-bit, single-channel grayscale input image.
	/// * circles: Output vector of found circles. Each vector is encoded as a 3-element
	/// floating-point vector ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%2C%20radius%29) .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// HoughCircles
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn detect(&mut self, src: &dyn core::ToInputArray, circles: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(circles);
		unsafe { sys::cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughCirclesDetector(), src.as_raw__InputArray(), circles.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn set_dp(&mut self, dp: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setDp_float(self.as_raw_mut_CUDA_HoughCirclesDetector(), dp) }.into_result()
	}
	
	fn get_dp(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getDp_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_min_dist(&mut self, min_dist: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setMinDist_float(self.as_raw_mut_CUDA_HoughCirclesDetector(), min_dist) }.into_result()
	}
	
	fn get_min_dist(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getMinDist_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_canny_threshold(&mut self, canny_threshold: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setCannyThreshold_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), canny_threshold) }.into_result()
	}
	
	fn get_canny_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getCannyThreshold_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_votes_threshold(&mut self, votes_threshold: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setVotesThreshold_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), votes_threshold) }.into_result()
	}
	
	fn get_votes_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getVotesThreshold_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_min_radius(&mut self, min_radius: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setMinRadius_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), min_radius) }.into_result()
	}
	
	fn get_min_radius(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getMinRadius_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_max_radius(&mut self, max_radius: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setMaxRadius_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), max_radius) }.into_result()
	}
	
	fn get_max_radius(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getMaxRadius_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
	fn set_max_circles(&mut self, max_circles: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_setMaxCircles_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), max_circles) }.into_result()
	}
	
	fn get_max_circles(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughCirclesDetector_getMaxCircles_const(self.as_raw_CUDA_HoughCirclesDetector()) }.into_result()
	}
	
}

/// Base class for lines detector algorithm. :
pub trait CUDA_HoughLinesDetector: core::AlgorithmTrait {
	fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void;

	/// Finds lines in a binary image using the classical Hough transform.
	/// 
	/// ## Parameters
	/// * src: 8-bit, single-channel binary source image.
	/// * lines: Output vector of lines. Each line is represented by a two-element vector
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28%5Crho%2C%20%5Ctheta%29) . ![inline formula](https://latex.codecogs.com/png.latex?%5Crho) is the distance from the coordinate origin ![inline formula](https://latex.codecogs.com/png.latex?%280%2C0%29) (top-left corner of
	/// the image). ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta) is the line rotation angle in radians (
	/// ![inline formula](https://latex.codecogs.com/png.latex?0%20%5Csim%20%5Ctextrm%7Bvertical%20line%7D%2C%20%5Cpi%2F2%20%5Csim%20%5Ctextrm%7Bhorizontal%20line%7D) ).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// HoughLines
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn detect(&mut self, src: &dyn core::ToInputArray, lines: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(lines);
		unsafe { sys::cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughLinesDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Downloads results from cuda::HoughLinesDetector::detect to host memory.
	/// 
	/// ## Parameters
	/// * d_lines: Result of cuda::HoughLinesDetector::detect .
	/// * h_lines: Output host array.
	/// * h_votes: Optional output array for line's votes.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * h_votes: noArray()
	/// * stream: Stream::Null()
	fn download_results(&mut self, d_lines: &dyn core::ToInputArray, h_lines: &mut dyn core::ToOutputArray, h_votes: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(d_lines);
		output_array_arg!(h_lines);
		output_array_arg!(h_votes);
		unsafe { sys::cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughLinesDetector(), d_lines.as_raw__InputArray(), h_lines.as_raw__OutputArray(), h_votes.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn set_rho(&mut self, rho: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughLinesDetector_setRho_float(self.as_raw_mut_CUDA_HoughLinesDetector(), rho) }.into_result()
	}
	
	fn get_rho(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughLinesDetector_getRho_const(self.as_raw_CUDA_HoughLinesDetector()) }.into_result()
	}
	
	fn set_theta(&mut self, theta: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughLinesDetector_setTheta_float(self.as_raw_mut_CUDA_HoughLinesDetector(), theta) }.into_result()
	}
	
	fn get_theta(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughLinesDetector_getTheta_const(self.as_raw_CUDA_HoughLinesDetector()) }.into_result()
	}
	
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughLinesDetector_setThreshold_int(self.as_raw_mut_CUDA_HoughLinesDetector(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughLinesDetector_getThreshold_const(self.as_raw_CUDA_HoughLinesDetector()) }.into_result()
	}
	
	fn set_do_sort(&mut self, do_sort: bool) -> Result<()> {
		unsafe { sys::cv_cuda_HoughLinesDetector_setDoSort_bool(self.as_raw_mut_CUDA_HoughLinesDetector(), do_sort) }.into_result()
	}
	
	fn get_do_sort(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_HoughLinesDetector_getDoSort_const(self.as_raw_CUDA_HoughLinesDetector()) }.into_result()
	}
	
	fn set_max_lines(&mut self, max_lines: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughLinesDetector_setMaxLines_int(self.as_raw_mut_CUDA_HoughLinesDetector(), max_lines) }.into_result()
	}
	
	fn get_max_lines(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughLinesDetector_getMaxLines_const(self.as_raw_CUDA_HoughLinesDetector()) }.into_result()
	}
	
}

/// Base class for line segments detector algorithm. :
pub trait CUDA_HoughSegmentDetector: core::AlgorithmTrait {
	fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void;

	/// Finds line segments in a binary image using the probabilistic Hough transform.
	/// 
	/// ## Parameters
	/// * src: 8-bit, single-channel binary source image.
	/// * lines: Output vector of lines. Each line is represented by a 4-element vector
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28x%5F1%2C%20y%5F1%2C%20x%5F2%2C%20y%5F2%29) , where ![inline formula](https://latex.codecogs.com/png.latex?%28x%5F1%2Cy%5F1%29) and ![inline formula](https://latex.codecogs.com/png.latex?%28x%5F2%2C%20y%5F2%29) are the ending points of each detected
	/// line segment.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// HoughLinesP
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn detect(&mut self, src: &dyn core::ToInputArray, lines: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(lines);
		unsafe { sys::cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughSegmentDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn set_rho(&mut self, rho: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_setRho_float(self.as_raw_mut_CUDA_HoughSegmentDetector(), rho) }.into_result()
	}
	
	fn get_rho(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_getRho_const(self.as_raw_CUDA_HoughSegmentDetector()) }.into_result()
	}
	
	fn set_theta(&mut self, theta: f32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_setTheta_float(self.as_raw_mut_CUDA_HoughSegmentDetector(), theta) }.into_result()
	}
	
	fn get_theta(&self) -> Result<f32> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_getTheta_const(self.as_raw_CUDA_HoughSegmentDetector()) }.into_result()
	}
	
	fn set_min_line_length(&mut self, min_line_length: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_setMinLineLength_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), min_line_length) }.into_result()
	}
	
	fn get_min_line_length(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_getMinLineLength_const(self.as_raw_CUDA_HoughSegmentDetector()) }.into_result()
	}
	
	fn set_max_line_gap(&mut self, max_line_gap: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_setMaxLineGap_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), max_line_gap) }.into_result()
	}
	
	fn get_max_line_gap(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_getMaxLineGap_const(self.as_raw_CUDA_HoughSegmentDetector()) }.into_result()
	}
	
	fn set_max_lines(&mut self, max_lines: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_setMaxLines_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), max_lines) }.into_result()
	}
	
	fn get_max_lines(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HoughSegmentDetector_getMaxLines_const(self.as_raw_CUDA_HoughSegmentDetector()) }.into_result()
	}
	
}

/// Base class for Template Matching. :
pub trait CUDA_TemplateMatching: core::AlgorithmTrait {
	fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void;
	fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void;

	/// Computes a proximity map for a raster template and an image where the template is searched for.
	/// 
	/// ## Parameters
	/// * image: Source image.
	/// * templ: Template image with the size and type the same as image .
	/// * result: Map containing comparison results ( CV_32FC1 ). If image is *W x H* and templ is *w
	/// x h*, then result must be *W-w+1 x H-h+1*.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn match_(&mut self, image: &dyn core::ToInputArray, templ: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(templ);
		output_array_arg!(result);
		unsafe { sys::cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_TemplateMatching(), image.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}
