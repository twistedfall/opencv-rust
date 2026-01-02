pub mod cudaimgproc {
	//! # Image Processing
	//!    # Color space processing
	//!    # Histogram Calculation
	//!    # Structural Analysis and Shape Descriptors
	//!    # Hough Transform
	//!    # Feature Detection
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{CUDA_CLAHETrait, CUDA_CLAHETraitConst, CUDA_CannyEdgeDetectorTrait, CUDA_CannyEdgeDetectorTraitConst, CUDA_CornernessCriteriaTrait, CUDA_CornernessCriteriaTraitConst, CUDA_CornersDetectorTrait, CUDA_CornersDetectorTraitConst, CUDA_HoughCirclesDetectorTrait, CUDA_HoughCirclesDetectorTraitConst, CUDA_HoughLinesDetectorTrait, CUDA_HoughLinesDetectorTraitConst, CUDA_HoughSegmentDetectorTrait, CUDA_HoughSegmentDetectorTraitConst, CUDA_TemplateMatchingTrait, CUDA_TemplateMatchingTraitConst};
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
	/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) algorithm for 8-way connectivity.
	pub const CUDA_CCL_BKE: i32 = 0;
	/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) algorithm for 8-way connectivity.
	pub const CUDA_CCL_DEFAULT: i32 = -1;
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
	pub const CUDA_FIRST_ORDER_MOMENTS: i32 = 1;
	pub const CUDA_SECOND_ORDER_MOMENTS: i32 = 2;
	pub const CUDA_THIRD_ORDER_MOMENTS: i32 = 3;
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

	opencv_type_enum! { crate::cudaimgproc::CUDA_AlphaCompTypes { ALPHA_OVER, ALPHA_IN, ALPHA_OUT, ALPHA_ATOP, ALPHA_XOR, ALPHA_PLUS, ALPHA_OVER_PREMUL, ALPHA_IN_PREMUL, ALPHA_OUT_PREMUL, ALPHA_ATOP_PREMUL, ALPHA_XOR_PREMUL, ALPHA_PLUS_PREMUL, ALPHA_PREMUL } }

	/// Connected Components Algorithm
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_ConnectedComponentsAlgorithmsTypes {
		/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) algorithm for 8-way connectivity.
		CCL_DEFAULT = -1,
		/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) algorithm for 8-way connectivity.
		CCL_BKE = 0,
	}

	opencv_type_enum! { crate::cudaimgproc::CUDA_ConnectedComponentsAlgorithmsTypes { CCL_DEFAULT, CCL_BKE } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
		// Duplicate, use COLOR_BayerRG2BGR_MHT instead
		// COLOR_BayerBG2RGB_MHT = 258,
		// Bayer Demosaicing (Malvar, He, and Cutler)
		// Duplicate, use COLOR_BayerGR2BGR_MHT instead
		// COLOR_BayerGB2RGB_MHT = 259,
		// Bayer Demosaicing (Malvar, He, and Cutler)
		// Duplicate, use COLOR_BayerBG2BGR_MHT instead
		// COLOR_BayerRG2RGB_MHT = 256,
		// Bayer Demosaicing (Malvar, He, and Cutler)
		// Duplicate, use COLOR_BayerGB2BGR_MHT instead
		// COLOR_BayerGR2RGB_MHT = 257,
		/// Bayer Demosaicing (Malvar, He, and Cutler)
		COLOR_BayerBG2GRAY_MHT = 260,
		/// Bayer Demosaicing (Malvar, He, and Cutler)
		COLOR_BayerGB2GRAY_MHT = 261,
		/// Bayer Demosaicing (Malvar, He, and Cutler)
		COLOR_BayerRG2GRAY_MHT = 262,
		/// Bayer Demosaicing (Malvar, He, and Cutler)
		COLOR_BayerGR2GRAY_MHT = 263,
	}

	opencv_type_enum! { crate::cudaimgproc::CUDA_DemosaicTypes { COLOR_BayerBG2BGR_MHT, COLOR_BayerGB2BGR_MHT, COLOR_BayerRG2BGR_MHT, COLOR_BayerGR2BGR_MHT, COLOR_BayerBG2GRAY_MHT, COLOR_BayerGB2GRAY_MHT, COLOR_BayerRG2GRAY_MHT, COLOR_BayerGR2GRAY_MHT } }

	/// Order of image moments.
	/// ## Parameters
	/// * FIRST_ORDER_MOMENTS: First order moments
	/// * SECOND_ORDER_MOMENTS: Second order moments.
	/// * THIRD_ORDER_MOMENTS: Third order moments.
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_MomentsOrder {
		FIRST_ORDER_MOMENTS = 1,
		SECOND_ORDER_MOMENTS = 2,
		THIRD_ORDER_MOMENTS = 3,
	}

	opencv_type_enum! { crate::cudaimgproc::CUDA_MomentsOrder { FIRST_ORDER_MOMENTS, SECOND_ORDER_MOMENTS, THIRD_ORDER_MOMENTS } }

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
	/// ## Note
	/// This alternative version of [alpha_comp] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn alpha_comp_def(img1: &impl ToInputArray, img2: &impl ToInputArray, dst: &mut impl ToOutputArray, alpha_op: i32) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(img1.as_raw__InputArray(), img2.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha_op, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

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
	#[inline]
	pub fn alpha_comp(img1: &impl ToInputArray, img2: &impl ToInputArray, dst: &mut impl ToOutputArray, alpha_op: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(img1.as_raw__InputArray(), img2.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha_op, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [bilateral_filter] function uses the following default values for its arguments:
	/// * border_mode: BORDER_DEFAULT
	/// * stream: Stream::Null()
	#[inline]
	pub fn bilateral_filter_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, kernel_size: i32, sigma_color: f32, sigma_spatial: f32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float(src.as_raw__InputArray(), dst.as_raw__OutputArray(), kernel_size, sigma_color, sigma_spatial, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn bilateral_filter(src: &impl ToInputArray, dst: &mut impl ToOutputArray, kernel_size: i32, sigma_color: f32, sigma_spatial: f32, border_mode: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), kernel_size, sigma_color, sigma_spatial, border_mode, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [blend_linear] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn blend_linear_def(img1: &impl ToInputArray, img2: &impl ToInputArray, weights1: &impl ToInputArray, weights2: &impl ToInputArray, result: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_array_arg!(weights1);
		input_array_arg!(weights2);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(img1.as_raw__InputArray(), img2.as_raw__InputArray(), weights1.as_raw__InputArray(), weights2.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn blend_linear(img1: &impl ToInputArray, img2: &impl ToInputArray, weights1: &impl ToInputArray, weights2: &impl ToInputArray, result: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_array_arg!(weights1);
		input_array_arg!(weights2);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(img1.as_raw__InputArray(), img2.as_raw__InputArray(), weights1.as_raw__InputArray(), weights2.as_raw__InputArray(), result.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates histogram for one channel 8-bit image confined in given mask.
	///
	/// ## Parameters
	/// * src: Source image with CV_8UC1 type.
	/// * hist: Destination histogram with one row, 256 columns, and the CV_32SC1 type.
	/// * mask: A mask image same size as src and of type CV_8UC1.
	/// * stream: Stream for the asynchronous version.
	///
	/// ## Note
	/// This alternative version of [calc_hist_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_hist_1_def(src: &impl ToInputArray, mask: &impl ToInputArray, hist: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(mask);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray(), hist.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn calc_hist_1(src: &impl ToInputArray, mask: &impl ToInputArray, hist: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(mask);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), mask.as_raw__InputArray(), hist.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates histogram for one channel 8-bit image.
	///
	/// ## Parameters
	/// * src: Source image with CV_8UC1 type.
	/// * hist: Destination histogram with one row, 256 columns, and the CV_32SC1 type.
	/// * stream: Stream for the asynchronous version.
	///
	/// ## Note
	/// This alternative version of [calc_hist] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_hist_def(src: &impl ToInputArray, hist: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn calc_hist(src: &impl ToInputArray, hist: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes the Connected Components Labeled image of a binary image.
	///
	/// The function takes as input a binary image and performs Connected Components Labeling. The output
	/// is an image where each Connected Component is assigned a unique label (integer value).
	/// ltype specifies the output label image type, an important consideration based on the total
	/// number of labels or alternatively the total number of pixels in the source image.
	/// ccltype specifies the connected components labeling algorithm to use, currently
	/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) is supported, see the [connected_components_algorithms_types]
	/// for details. Note that labels in the output are not required to be sequential.
	///
	/// ## Parameters
	/// * image: The 8-bit single-channel image to be labeled.
	/// * labels: Destination labeled image.
	/// * connectivity: Connectivity to use for the labeling procedure. 8 for 8-way connectivity is supported.
	/// * ltype: Output image label type. Currently CV_32S is supported.
	/// * ccltype: Connected components algorithm type (see the #ConnectedComponentsAlgorithmsTypes).
	///
	///
	/// Note: A sample program demonstrating Connected Components Labeling in CUDA can be found at
	///
	/// opencv_contrib_source_code/modules/cudaimgproc/samples/connected_components.cpp
	///
	/// ## Overloaded parameters
	///
	///
	/// * image: The 8-bit single-channel image to be labeled.
	/// * labels: Destination labeled image.
	/// * connectivity: Connectivity to use for the labeling procedure. 8 for 8-way connectivity is supported.
	/// * ltype: Output image label type. Currently CV_32S is supported.
	///
	/// ## Note
	/// This alternative version of [connected_components] function uses the following default values for its arguments:
	/// * connectivity: 8
	/// * ltype: CV_32S
	#[inline]
	pub fn connected_components_def(image: &impl ToInputArray, labels: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), labels.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes the Connected Components Labeled image of a binary image.
	///
	/// The function takes as input a binary image and performs Connected Components Labeling. The output
	/// is an image where each Connected Component is assigned a unique label (integer value).
	/// ltype specifies the output label image type, an important consideration based on the total
	/// number of labels or alternatively the total number of pixels in the source image.
	/// ccltype specifies the connected components labeling algorithm to use, currently
	/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) is supported, see the [connected_components_algorithms_types]
	/// for details. Note that labels in the output are not required to be sequential.
	///
	/// ## Parameters
	/// * image: The 8-bit single-channel image to be labeled.
	/// * labels: Destination labeled image.
	/// * connectivity: Connectivity to use for the labeling procedure. 8 for 8-way connectivity is supported.
	/// * ltype: Output image label type. Currently CV_32S is supported.
	/// * ccltype: Connected components algorithm type (see the #ConnectedComponentsAlgorithmsTypes).
	///
	///
	/// Note: A sample program demonstrating Connected Components Labeling in CUDA can be found at
	///
	/// opencv_contrib_source_code/modules/cudaimgproc/samples/connected_components.cpp
	///
	/// ## Overloaded parameters
	///
	///
	/// * image: The 8-bit single-channel image to be labeled.
	/// * labels: Destination labeled image.
	/// * connectivity: Connectivity to use for the labeling procedure. 8 for 8-way connectivity is supported.
	/// * ltype: Output image label type. Currently CV_32S is supported.
	///
	/// ## C++ default parameters
	/// * connectivity: 8
	/// * ltype: CV_32S
	#[inline]
	pub fn connected_components(image: &impl ToInputArray, labels: &mut impl ToOutputArray, connectivity: i32, ltype: i32) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(image.as_raw__InputArray(), labels.as_raw__OutputArray(), connectivity, ltype, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes the Connected Components Labeled image of a binary image.
	///
	/// The function takes as input a binary image and performs Connected Components Labeling. The output
	/// is an image where each Connected Component is assigned a unique label (integer value).
	/// ltype specifies the output label image type, an important consideration based on the total
	/// number of labels or alternatively the total number of pixels in the source image.
	/// ccltype specifies the connected components labeling algorithm to use, currently
	/// BKE [Allegretti2019](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Allegretti2019) is supported, see the [connected_components_algorithms_types]
	/// for details. Note that labels in the output are not required to be sequential.
	///
	/// ## Parameters
	/// * image: The 8-bit single-channel image to be labeled.
	/// * labels: Destination labeled image.
	/// * connectivity: Connectivity to use for the labeling procedure. 8 for 8-way connectivity is supported.
	/// * ltype: Output image label type. Currently CV_32S is supported.
	/// * ccltype: Connected components algorithm type (see the #ConnectedComponentsAlgorithmsTypes).
	///
	///
	/// Note: A sample program demonstrating Connected Components Labeling in CUDA can be found at
	///
	/// opencv_contrib_source_code/modules/cudaimgproc/samples/connected_components.cpp
	#[inline]
	pub fn connected_components_with_algorithm(image: &impl ToInputArray, labels: &mut impl ToOutputArray, connectivity: i32, ltype: i32, ccltype: crate::cudaimgproc::CUDA_ConnectedComponentsAlgorithmsTypes) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_ConnectedComponentsAlgorithmsTypes(image.as_raw__InputArray(), labels.as_raw__OutputArray(), connectivity, ltype, ccltype, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts the spatial image moments returned from cuda::spatialMoments to cv::Moments.
	/// ## Parameters
	/// * spatialMoments: Spatial moments returned from cuda::spatialMoments.
	/// * order: Order used when calculating image moments with cuda::spatialMoments.
	/// * momentsType: Precision used when calculating image moments with cuda::spatialMoments.
	///
	/// ## Returns
	/// cv::Moments.
	/// ## See also
	/// cuda::spatialMoments, cuda::moments, cuda::convertSpatialMoments, cuda::numMoments, cuda::MomentsOrder
	#[inline]
	pub fn convert_spatial_moments(mut spatial_moments: impl core::MatTrait, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32) -> Result<core::Moments> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_convertSpatialMoments_Mat_const_MomentsOrder_const_int(spatial_moments.as_raw_mut_Mat(), order, moments_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Creates implementation for cuda::CLAHE .
	///
	/// ## Parameters
	/// * clipLimit: Threshold for contrast limiting.
	/// * tileGridSize: Size of grid for histogram equalization. Input image will be divided into
	/// equally sized rectangular tiles. tileGridSize defines the number of tiles in row and column.
	///
	/// ## Note
	/// This alternative version of [create_clahe] function uses the following default values for its arguments:
	/// * clip_limit: 40.0
	/// * tile_grid_size: Size(8,8)
	#[inline]
	pub fn create_clahe_def() -> Result<core::Ptr<crate::cudaimgproc::CUDA_CLAHE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createCLAHE(ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CLAHE>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_clahe(clip_limit: f64, tile_grid_size: core::Size) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CLAHE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createCLAHE_double_Size(clip_limit, &tile_grid_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CLAHE>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_canny_edge_detector] function uses the following default values for its arguments:
	/// * apperture_size: 3
	/// * l2gradient: false
	#[inline]
	pub fn create_canny_edge_detector_def(low_thresh: f64, high_thresh: f64) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createCannyEdgeDetector_double_double(low_thresh, high_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CannyEdgeDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_canny_edge_detector(low_thresh: f64, high_thresh: f64, apperture_size: i32, l2gradient: bool) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createCannyEdgeDetector_double_double_int_bool(low_thresh, high_thresh, apperture_size, l2gradient, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CannyEdgeDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates implementation for generalized hough transform from [Ballard1981](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Ballard1981) .
	#[inline]
	pub fn create_generalized_hough_ballard() -> Result<core::Ptr<crate::imgproc::GeneralizedHoughBallard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGeneralizedHoughBallard(ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::imgproc::GeneralizedHoughBallard>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates implementation for generalized hough transform from [Guil1999](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Guil1999) .
	#[inline]
	pub fn create_generalized_hough_guil() -> Result<core::Ptr<crate::imgproc::GeneralizedHoughGuil>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGeneralizedHoughGuil(ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::imgproc::GeneralizedHoughGuil>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_good_features_to_track_detector] function uses the following default values for its arguments:
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 0.0
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * harris_k: 0.04
	#[inline]
	pub fn create_good_features_to_track_detector_def(src_type: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornersDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGoodFeaturesToTrackDetector_int(src_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornersDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_good_features_to_track_detector(src_type: i32, max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, harris_k: f64) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornersDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGoodFeaturesToTrackDetector_int_int_double_double_int_bool_double(src_type, max_corners, quality_level, min_distance, block_size, use_harris_detector, harris_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornersDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_harris_corner] function uses the following default values for its arguments:
	/// * border_type: BORDER_REFLECT101
	#[inline]
	pub fn create_harris_corner_def(src_type: i32, block_size: i32, ksize: i32, k: f64) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHarrisCorner_int_int_int_double(src_type, block_size, ksize, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_harris_corner(src_type: i32, block_size: i32, ksize: i32, k: f64, border_type: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHarrisCorner_int_int_int_double_int(src_type, block_size, ksize, k, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_hough_circles_detector] function uses the following default values for its arguments:
	/// * max_circles: 4096
	#[inline]
	pub fn create_hough_circles_detector_def(dp: f32, min_dist: f32, canny_threshold: i32, votes_threshold: i32, min_radius: i32, max_radius: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int(dp, min_dist, canny_threshold, votes_threshold, min_radius, max_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughCirclesDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_hough_circles_detector(dp: f32, min_dist: f32, canny_threshold: i32, votes_threshold: i32, min_radius: i32, max_radius: i32, max_circles: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int_int(dp, min_dist, canny_threshold, votes_threshold, min_radius, max_radius, max_circles, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughCirclesDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_hough_lines_detector] function uses the following default values for its arguments:
	/// * do_sort: false
	/// * max_lines: 4096
	#[inline]
	pub fn create_hough_lines_detector_def(rho: f32, theta: f32, threshold: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughLinesDetector_float_float_int(rho, theta, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughLinesDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_hough_lines_detector(rho: f32, theta: f32, threshold: i32, do_sort: bool, max_lines: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughLinesDetector_float_float_int_bool_int(rho, theta, threshold, do_sort, max_lines, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughLinesDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates implementation for cuda::HoughSegmentDetector .
	///
	/// ## Parameters
	/// * rho: Distance resolution of the accumulator in pixels.
	/// * theta: Angle resolution of the accumulator in radians.
	/// * minLineLength: Minimum line length. Line segments shorter than that are rejected.
	/// * maxLineGap: Maximum allowed gap between points on the same line to link them.
	/// * maxLines: Maximum number of output lines.
	/// * threshold: %Accumulator threshold parameter. Only those lines are returned that get enough
	/// votes ( ![inline formula](https://latex.codecogs.com/png.latex?%3E%5Ctexttt%7Bthreshold%7D) ).
	///
	/// ## Note
	/// This alternative version of [create_hough_segment_detector] function uses the following default values for its arguments:
	/// * max_lines: 4096
	/// * threshold: -1
	#[inline]
	pub fn create_hough_segment_detector_def(rho: f32, theta: f32, min_line_length: i32, max_line_gap: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughSegmentDetector_float_float_int_int(rho, theta, min_line_length, max_line_gap, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughSegmentDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates implementation for cuda::HoughSegmentDetector .
	///
	/// ## Parameters
	/// * rho: Distance resolution of the accumulator in pixels.
	/// * theta: Angle resolution of the accumulator in radians.
	/// * minLineLength: Minimum line length. Line segments shorter than that are rejected.
	/// * maxLineGap: Maximum allowed gap between points on the same line to link them.
	/// * maxLines: Maximum number of output lines.
	/// * threshold: %Accumulator threshold parameter. Only those lines are returned that get enough
	/// votes ( ![inline formula](https://latex.codecogs.com/png.latex?%3E%5Ctexttt%7Bthreshold%7D) ).
	///
	/// ## C++ default parameters
	/// * max_lines: 4096
	/// * threshold: -1
	#[inline]
	pub fn create_hough_segment_detector(rho: f32, theta: f32, min_line_length: i32, max_line_gap: i32, max_lines: i32, threshold: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createHoughSegmentDetector_float_float_int_int_int_int(rho, theta, min_line_length, max_line_gap, max_lines, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_HoughSegmentDetector>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_min_eigen_val_corner] function uses the following default values for its arguments:
	/// * border_type: BORDER_REFLECT101
	#[inline]
	pub fn create_min_eigen_val_corner_def(src_type: i32, block_size: i32, ksize: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMinEigenValCorner_int_int_int(src_type, block_size, ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_min_eigen_val_corner(src_type: i32, block_size: i32, ksize: i32, border_type: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMinEigenValCorner_int_int_int_int(src_type, block_size, ksize, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_CornernessCriteria>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [create_template_matching] function uses the following default values for its arguments:
	/// * user_block_size: Size()
	#[inline]
	pub fn create_template_matching_def(src_type: i32, method: i32) -> Result<core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createTemplateMatching_int_int(src_type, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_TemplateMatching>::opencv_from_extern(ret) };
		Ok(ret)
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
	#[inline]
	pub fn create_template_matching(src_type: i32, method: i32, user_block_size: core::Size) -> Result<core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createTemplateMatching_int_int_Size(src_type, method, &user_block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaimgproc::CUDA_TemplateMatching>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [cvt_color] function uses the following default values for its arguments:
	/// * dcn: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn cvt_color_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, code: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn cvt_color(src: &impl ToInputArray, dst: &mut impl ToOutputArray, code: i32, dcn: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, dcn, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// *   Demosaicing using Malvar-He-Cutler algorithm ([MHT2011](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_MHT2011))
	///
	///    > -   COLOR_BayerBG2GRAY_MHT , COLOR_BayerGB2GRAY_MHT , COLOR_BayerRG2GRAY_MHT ,
	///    >     COLOR_BayerGR2GRAY_MHT
	///    > -   COLOR_BayerBG2BGR_MHT , COLOR_BayerGB2BGR_MHT , COLOR_BayerRG2BGR_MHT ,
	///    >     COLOR_BayerGR2BGR_MHT
	/// ## See also
	/// cvtColor
	///
	/// ## Note
	/// This alternative version of [demosaicing] function uses the following default values for its arguments:
	/// * dcn: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn demosaicing_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, code: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// *   Demosaicing using Malvar-He-Cutler algorithm ([MHT2011](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_MHT2011))
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
	#[inline]
	pub fn demosaicing(src: &impl ToInputArray, dst: &mut impl ToOutputArray, code: i32, dcn: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), code, dcn, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [equalize_hist] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn equalize_hist_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn equalize_hist(src: &impl ToInputArray, dst: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [even_levels] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn even_levels_def(levels: &mut impl ToOutputArray, n_levels: i32, lower_level: i32, upper_level: i32) -> Result<()> {
		output_array_arg!(levels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_evenLevels_const__OutputArrayR_int_int_int(levels.as_raw__OutputArray(), n_levels, lower_level, upper_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn even_levels(levels: &mut impl ToOutputArray, n_levels: i32, lower_level: i32, upper_level: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		output_array_arg!(levels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_evenLevels_const__OutputArrayR_int_int_int_StreamR(levels.as_raw__OutputArray(), n_levels, lower_level, upper_level, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Routines for correcting image color gamma.
	///
	/// ## Parameters
	/// * src: Source image (3- or 4-channel 8 bit).
	/// * dst: Destination image.
	/// * forward: true for forward gamma correction or false for inverse gamma correction.
	/// * stream: Stream for the asynchronous version.
	///
	/// Gamma correction is conformant to BT.709 [BT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_BT).709 with &gamma;=0.45.
	///
	/// For the forward transform, RGB values are normalised to fit in the range L=[0..1], then:
	/// - For L < 0.018
	///   + V = 4.5*L
	/// - For L >= 0.018
	///   + V = 1.099 * L^0.45 - 0.099
	///
	/// With V then being scaled back to [0..255].
	///
	/// ![image](https://docs.opencv.org/4.13.0/gammacorrection.png)
	///
	/// ## Note
	/// This alternative version of [gamma_correction] function uses the following default values for its arguments:
	/// * forward: true
	/// * stream: Stream::Null()
	#[inline]
	pub fn gamma_correction_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Routines for correcting image color gamma.
	///
	/// ## Parameters
	/// * src: Source image (3- or 4-channel 8 bit).
	/// * dst: Destination image.
	/// * forward: true for forward gamma correction or false for inverse gamma correction.
	/// * stream: Stream for the asynchronous version.
	///
	/// Gamma correction is conformant to BT.709 [BT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_BT).709 with &gamma;=0.45.
	///
	/// For the forward transform, RGB values are normalised to fit in the range L=[0..1], then:
	/// - For L < 0.018
	///   + V = 4.5*L
	/// - For L >= 0.018
	///   + V = 1.099 * L^0.45 - 0.099
	///
	/// With V then being scaled back to [0..255].
	///
	/// ![image](https://docs.opencv.org/4.13.0/gammacorrection.png)
	///
	/// ## C++ default parameters
	/// * forward: true
	/// * stream: Stream::Null()
	#[inline]
	pub fn gamma_correction(src: &impl ToInputArray, dst: &mut impl ToOutputArray, forward: bool, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR_bool_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), forward, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [hist_even] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn hist_even_def(src: &impl ToInputArray, hist: &mut impl ToOutputArray, hist_size: i32, lower_level: i32, upper_level: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int(src.as_raw__InputArray(), hist.as_raw__OutputArray(), hist_size, lower_level, upper_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn hist_even(src: &impl ToInputArray, hist: &mut impl ToOutputArray, hist_size: i32, lower_level: i32, upper_level: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), hist_size, lower_level, upper_level, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [hist_range] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn hist_range_def(src: &impl ToInputArray, hist: &mut impl ToOutputArray, levels: &impl ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		input_array_arg!(levels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), levels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn hist_range(src: &impl ToInputArray, hist: &mut impl ToOutputArray, levels: &impl ToInputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(hist);
		input_array_arg!(levels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), hist.as_raw__OutputArray(), levels.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [mean_shift_filtering] function uses the following default values for its arguments:
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_shift_filtering_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, sp: i32, sr: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn mean_shift_filtering(src: &impl ToInputArray, dst: &mut impl ToOutputArray, sp: i32, sr: i32, criteria: core::TermCriteria, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, &criteria, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [mean_shift_proc] function uses the following default values for its arguments:
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_shift_proc_def(src: &impl ToInputArray, dstr: &mut impl ToOutputArray, dstsp: &mut impl ToOutputArray, sp: i32, sr: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dstr);
		output_array_arg!(dstsp);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dstr.as_raw__OutputArray(), dstsp.as_raw__OutputArray(), sp, sr, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn mean_shift_proc(src: &impl ToInputArray, dstr: &mut impl ToOutputArray, dstsp: &mut impl ToOutputArray, sp: i32, sr: i32, criteria: core::TermCriteria, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dstr);
		output_array_arg!(dstsp);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dstr.as_raw__OutputArray(), dstsp.as_raw__OutputArray(), sp, sr, &criteria, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [mean_shift_segmentation] function uses the following default values for its arguments:
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_shift_segmentation_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, sp: i32, sr: i32, minsize: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, minsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn mean_shift_segmentation(src: &impl ToInputArray, dst: &mut impl ToOutputArray, sp: i32, sr: i32, minsize: i32, criteria: core::TermCriteria, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int_TermCriteria_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sp, sr, minsize, &criteria, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the moments up to the 3rd order of a rasterized shape.
	///
	/// The function computes moments, up to the 3rd order, of a rasterized shape. The
	/// results are returned in the structure cv::Moments.
	///
	/// ## Parameters
	/// * src: Raster image (single-channel 2D array).
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's.
	/// * order: Order of largest moments to calculate with lower order moments requiring less computation.
	/// * momentsType: Precision to use when calculating moments. Available types are \ref CV_32F and \ref CV_64F with the performance of \ref CV_32F an order of magnitude greater than \ref CV_64F. If the image is small the accuracy from \ref CV_32F can be equal or very close to \ref CV_64F.
	///
	///
	/// Note: For maximum performance use the asynchronous version cuda::spatialMoments() as this version interally allocates and deallocates both GpuMat and HostMem to respectively perform the calculation on the device and download the result to the host.
	/// The costly HostMem allocation cannot be avoided however the GpuMat device allocation can be by using BufferPool, e.g.
	/// ```
	///    setBufferPoolUsage(true);
	///    setBufferPoolConfig(getDevice(), numMoments(order) * ((momentsType == CV_64F) ? sizeof(double) : sizeof(float)), 1);
	/// ```
	/// see the \a CUDA_TEST_P(Moments, Accuracy) test inside opencv_contrib_source_code/modules/cudaimgproc/test/test_moments.cpp for an example.
	/// ## Returns
	/// cv::Moments.
	/// ## See also
	/// cuda::spatialMoments, cuda::convertSpatialMoments, cuda::numMoments, cuda::MomentsOrder
	///
	/// ## Note
	/// This alternative version of [moments] function uses the following default values for its arguments:
	/// * binary_image: false
	/// * order: MomentsOrder::THIRD_ORDER_MOMENTS
	/// * moments_type: CV_64F
	#[inline]
	pub fn moments_def(src: &impl ToInputArray) -> Result<core::Moments> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_moments_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the moments up to the 3rd order of a rasterized shape.
	///
	/// The function computes moments, up to the 3rd order, of a rasterized shape. The
	/// results are returned in the structure cv::Moments.
	///
	/// ## Parameters
	/// * src: Raster image (single-channel 2D array).
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's.
	/// * order: Order of largest moments to calculate with lower order moments requiring less computation.
	/// * momentsType: Precision to use when calculating moments. Available types are \ref CV_32F and \ref CV_64F with the performance of \ref CV_32F an order of magnitude greater than \ref CV_64F. If the image is small the accuracy from \ref CV_32F can be equal or very close to \ref CV_64F.
	///
	///
	/// Note: For maximum performance use the asynchronous version cuda::spatialMoments() as this version interally allocates and deallocates both GpuMat and HostMem to respectively perform the calculation on the device and download the result to the host.
	/// The costly HostMem allocation cannot be avoided however the GpuMat device allocation can be by using BufferPool, e.g.
	/// ```
	///    setBufferPoolUsage(true);
	///    setBufferPoolConfig(getDevice(), numMoments(order) * ((momentsType == CV_64F) ? sizeof(double) : sizeof(float)), 1);
	/// ```
	/// see the \a CUDA_TEST_P(Moments, Accuracy) test inside opencv_contrib_source_code/modules/cudaimgproc/test/test_moments.cpp for an example.
	/// ## Returns
	/// cv::Moments.
	/// ## See also
	/// cuda::spatialMoments, cuda::convertSpatialMoments, cuda::numMoments, cuda::MomentsOrder
	///
	/// ## C++ default parameters
	/// * binary_image: false
	/// * order: MomentsOrder::THIRD_ORDER_MOMENTS
	/// * moments_type: CV_64F
	#[inline]
	pub fn moments(src: &impl ToInputArray, binary_image: bool, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32) -> Result<core::Moments> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_moments_const__InputArrayR_const_bool_const_MomentsOrder_const_int(src.as_raw__InputArray(), binary_image, order, moments_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of image moments less than or equal to the largest image moments \a order.
	/// ## Parameters
	/// * order: Order of largest moments to calculate with lower order moments requiring less computation.
	/// ## Returns
	/// number of image moments.
	/// ## See also
	/// cuda::spatialMoments, cuda::moments, cuda::MomentsOrder
	#[inline]
	pub fn num_moments(order: crate::cudaimgproc::CUDA_MomentsOrder) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_numMoments_const_MomentsOrder(order, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the spatial moments up to the 3rd order of a rasterized shape.
	///
	/// Asynchronous version of cuda::moments() which only calculates the spatial (not centralized or normalized) moments, up to the 3rd order, of a rasterized shape.
	/// Each moment is returned as a column entry in the 1D \a moments array.
	///
	/// ## Parameters
	/// * src: Raster image (single-channel 2D array).
	/// * moments:[out] 1D array with each column entry containing a spatial image moment.
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's.
	/// * order: Order of largest moments to calculate with lower order moments requiring less computation.
	/// * momentsType: Precision to use when calculating moments. Available types are \ref CV_32F and \ref CV_64F with the performance of \ref CV_32F an order of magnitude greater than \ref CV_64F. If the image is small the accuracy from \ref CV_32F can be equal or very close to \ref CV_64F.
	/// * stream: Stream for the asynchronous version.
	///
	///
	/// Note: For maximum performance pre-allocate a 1D GpuMat for \a moments of the correct type and size large enough to store the all the image moments of up to the desired \a order. e.g. With \a order === MomentsOrder::SECOND_ORDER_MOMENTS and \a momentsType == \ref CV_32F \a moments can be allocated as
	/// ```
	/// GpuMat momentsDevice(1,numMoments(MomentsOrder::SECOND_ORDER_MOMENTS),CV_32F)
	/// ```
	/// The central and normalized moments can easily be calculated on the host by downloading the \a moments array and using the cuda::convertSpatialMoments helper function. e.g.
	/// ```
	/// HostMem spatialMomentsHostMem(1, numMoments(MomentsOrder::SECOND_ORDER_MOMENTS), CV_32F);
	/// spatialMomentsDevice.download(spatialMomentsHostMem, stream);
	/// stream.waitForCompletion();
	/// Mat spatialMoments = spatialMomentsHostMem.createMatHeader();
	/// cv::Moments cvMoments = convertSpatialMoments<float>(spatialMoments, order);
	/// ```
	///
	/// see the \a CUDA_TEST_P(Moments, Async) test inside opencv_contrib_source_code/modules/cudaimgproc/test/test_moments.cpp for an example.
	/// ## See also
	/// cuda::moments, cuda::convertSpatialMoments, cuda::numMoments, cuda::MomentsOrder
	///
	/// ## Note
	/// This alternative version of [spatial_moments] function uses the following default values for its arguments:
	/// * binary_image: false
	/// * order: MomentsOrder::THIRD_ORDER_MOMENTS
	/// * moments_type: CV_64F
	/// * stream: Stream::Null()
	#[inline]
	pub fn spatial_moments_def(src: &impl ToInputArray, moments: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(moments);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), moments.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the spatial moments up to the 3rd order of a rasterized shape.
	///
	/// Asynchronous version of cuda::moments() which only calculates the spatial (not centralized or normalized) moments, up to the 3rd order, of a rasterized shape.
	/// Each moment is returned as a column entry in the 1D \a moments array.
	///
	/// ## Parameters
	/// * src: Raster image (single-channel 2D array).
	/// * moments:[out] 1D array with each column entry containing a spatial image moment.
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's.
	/// * order: Order of largest moments to calculate with lower order moments requiring less computation.
	/// * momentsType: Precision to use when calculating moments. Available types are \ref CV_32F and \ref CV_64F with the performance of \ref CV_32F an order of magnitude greater than \ref CV_64F. If the image is small the accuracy from \ref CV_32F can be equal or very close to \ref CV_64F.
	/// * stream: Stream for the asynchronous version.
	///
	///
	/// Note: For maximum performance pre-allocate a 1D GpuMat for \a moments of the correct type and size large enough to store the all the image moments of up to the desired \a order. e.g. With \a order === MomentsOrder::SECOND_ORDER_MOMENTS and \a momentsType == \ref CV_32F \a moments can be allocated as
	/// ```
	/// GpuMat momentsDevice(1,numMoments(MomentsOrder::SECOND_ORDER_MOMENTS),CV_32F)
	/// ```
	/// The central and normalized moments can easily be calculated on the host by downloading the \a moments array and using the cuda::convertSpatialMoments helper function. e.g.
	/// ```
	/// HostMem spatialMomentsHostMem(1, numMoments(MomentsOrder::SECOND_ORDER_MOMENTS), CV_32F);
	/// spatialMomentsDevice.download(spatialMomentsHostMem, stream);
	/// stream.waitForCompletion();
	/// Mat spatialMoments = spatialMomentsHostMem.createMatHeader();
	/// cv::Moments cvMoments = convertSpatialMoments<float>(spatialMoments, order);
	/// ```
	///
	/// see the \a CUDA_TEST_P(Moments, Async) test inside opencv_contrib_source_code/modules/cudaimgproc/test/test_moments.cpp for an example.
	/// ## See also
	/// cuda::moments, cuda::convertSpatialMoments, cuda::numMoments, cuda::MomentsOrder
	///
	/// ## C++ default parameters
	/// * binary_image: false
	/// * order: MomentsOrder::THIRD_ORDER_MOMENTS
	/// * moments_type: CV_64F
	/// * stream: Stream::Null()
	#[inline]
	pub fn spatial_moments(src: &impl ToInputArray, moments: &mut impl ToOutputArray, binary_image: bool, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(moments);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR_const_bool_const_MomentsOrder_const_int_StreamR(src.as_raw__InputArray(), moments.as_raw__OutputArray(), binary_image, order, moments_type, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [swap_channels] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn swap_channels_def(image: &mut impl ToInputOutputArray, dst_order: &[i32; 4]) -> Result<()> {
		input_output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX(image.as_raw__InputOutputArray(), dst_order, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn swap_channels(image: &mut impl ToInputOutputArray, dst_order: &[i32; 4], stream: &mut impl core::StreamTrait) -> Result<()> {
		input_output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX_StreamR(image.as_raw__InputOutputArray(), dst_order, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Base class for Contrast Limited Adaptive Histogram Equalization. :
	pub struct CUDA_CLAHE {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_CLAHE }

	impl Drop for CUDA_CLAHE {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_CLAHE_delete(self.as_raw_mut_CUDA_CLAHE()) };
		}
	}

	unsafe impl Send for CUDA_CLAHE {}

	/// Constant methods for [crate::cudaimgproc::CUDA_CLAHE]
	pub trait CUDA_CLAHETraitConst: crate::imgproc::CLAHETraitConst {
		fn as_raw_CUDA_CLAHE(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_CLAHE]
	pub trait CUDA_CLAHETrait: crate::cudaimgproc::CUDA_CLAHETraitConst + crate::imgproc::CLAHETrait {
		fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void;

		/// Equalizes the histogram of a grayscale image using Contrast Limited Adaptive Histogram Equalization.
		///
		/// ## Parameters
		/// * src: Source image with CV_8UC1 type.
		/// * dst: Destination image.
		/// * stream: Stream for the asynchronous version.
		#[inline]
		fn apply(&mut self, src: &impl ToInputArray, dst: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CLAHE_apply_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CLAHE(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_CLAHE {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_CLAHE")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_CLAHE, core::Algorithm, cv_cuda_CLAHE_to_Algorithm }

	boxed_cast_base! { CUDA_CLAHE, crate::imgproc::CLAHE, cv_cuda_CLAHE_to_CLAHE }

	impl core::AlgorithmTraitConst for CUDA_CLAHE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_CLAHE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CLAHE, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::imgproc::CLAHETraitConst for CUDA_CLAHE {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::imgproc::CLAHETrait for CUDA_CLAHE {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CLAHE, crate::imgproc::CLAHETraitConst, as_raw_CLAHE, crate::imgproc::CLAHETrait, as_raw_mut_CLAHE }

	impl crate::cudaimgproc::CUDA_CLAHETraitConst for CUDA_CLAHE {
		#[inline] fn as_raw_CUDA_CLAHE(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_CLAHETrait for CUDA_CLAHE {
		#[inline] fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CLAHE, crate::cudaimgproc::CUDA_CLAHETraitConst, as_raw_CUDA_CLAHE, crate::cudaimgproc::CUDA_CLAHETrait, as_raw_mut_CUDA_CLAHE }

	/// Base class for Canny Edge Detector. :
	pub struct CUDA_CannyEdgeDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_CannyEdgeDetector }

	impl Drop for CUDA_CannyEdgeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_CannyEdgeDetector_delete(self.as_raw_mut_CUDA_CannyEdgeDetector()) };
		}
	}

	unsafe impl Send for CUDA_CannyEdgeDetector {}

	/// Constant methods for [crate::cudaimgproc::CUDA_CannyEdgeDetector]
	pub trait CUDA_CannyEdgeDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void;

		#[inline]
		fn get_low_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_getLowThreshold_const(self.as_raw_CUDA_CannyEdgeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_high_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_getHighThreshold_const(self.as_raw_CUDA_CannyEdgeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_apperture_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_getAppertureSize_const(self.as_raw_CUDA_CannyEdgeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_l2_gradient(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_getL2Gradient_const(self.as_raw_CUDA_CannyEdgeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_CannyEdgeDetector]
	pub trait CUDA_CannyEdgeDetectorTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_CannyEdgeDetectorTraitConst {
		fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void;

		/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
		///
		/// ## Parameters
		/// * image: Single-channel 8-bit input image.
		/// * edges: Output edge map. It has the same size and type as image.
		/// * stream: Stream for the asynchronous version.
		///
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn detect(&mut self, image: &impl ToInputArray, edges: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(edges);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CannyEdgeDetector(), image.as_raw__InputArray(), edges.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
		///
		/// ## Parameters
		/// * image: Single-channel 8-bit input image.
		/// * edges: Output edge map. It has the same size and type as image.
		/// * stream: Stream for the asynchronous version.
		///
		/// ## Note
		/// This alternative version of [CUDA_CannyEdgeDetectorTrait::detect] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def(&mut self, image: &impl ToInputArray, edges: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(edges);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_CannyEdgeDetector(), image.as_raw__InputArray(), edges.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
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
		#[inline]
		fn detect_1(&mut self, dx: &impl ToInputArray, dy: &impl ToInputArray, edges: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(dx);
			input_array_arg!(dy);
			output_array_arg!(edges);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CannyEdgeDetector(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), edges.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Finds edges in an image using the [Canny86](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Canny86) algorithm.
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
		/// ## Note
		/// This alternative version of [CUDA_CannyEdgeDetectorTrait::detect] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def_1(&mut self, dx: &impl ToInputArray, dy: &impl ToInputArray, edges: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(dx);
			input_array_arg!(dy);
			output_array_arg!(edges);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_CannyEdgeDetector(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), edges.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_low_threshold(&mut self, low_thresh: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_setLowThreshold_double(self.as_raw_mut_CUDA_CannyEdgeDetector(), low_thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_high_threshold(&mut self, high_thresh: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_setHighThreshold_double(self.as_raw_mut_CUDA_CannyEdgeDetector(), high_thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_apperture_size(&mut self, apperture_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_setAppertureSize_int(self.as_raw_mut_CUDA_CannyEdgeDetector(), apperture_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_l2_gradient(&mut self, l2gradient: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CannyEdgeDetector_setL2Gradient_bool(self.as_raw_mut_CUDA_CannyEdgeDetector(), l2gradient, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_CannyEdgeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_CannyEdgeDetector")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_CannyEdgeDetector, core::Algorithm, cv_cuda_CannyEdgeDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_CannyEdgeDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_CannyEdgeDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CannyEdgeDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTraitConst for CUDA_CannyEdgeDetector {
		#[inline] fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTrait for CUDA_CannyEdgeDetector {
		#[inline] fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CannyEdgeDetector, crate::cudaimgproc::CUDA_CannyEdgeDetectorTraitConst, as_raw_CUDA_CannyEdgeDetector, crate::cudaimgproc::CUDA_CannyEdgeDetectorTrait, as_raw_mut_CUDA_CannyEdgeDetector }

	/// Base class for Cornerness Criteria computation. :
	pub struct CUDA_CornernessCriteria {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_CornernessCriteria }

	impl Drop for CUDA_CornernessCriteria {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_CornernessCriteria_delete(self.as_raw_mut_CUDA_CornernessCriteria()) };
		}
	}

	unsafe impl Send for CUDA_CornernessCriteria {}

	/// Constant methods for [crate::cudaimgproc::CUDA_CornernessCriteria]
	pub trait CUDA_CornernessCriteriaTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_CornernessCriteria]
	pub trait CUDA_CornernessCriteriaTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_CornernessCriteriaTraitConst {
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
		#[inline]
		fn compute(&mut self, src: &impl ToInputArray, dst: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CornernessCriteria(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Computes the cornerness criteria at each image pixel.
		///
		/// ## Parameters
		/// * src: Source image.
		/// * dst: Destination image containing cornerness values. It will have the same size as src and
		/// CV_32FC1 type.
		/// * stream: Stream for the asynchronous version.
		///
		/// ## Note
		/// This alternative version of [CUDA_CornernessCriteriaTrait::compute] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn compute_def(&mut self, src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_CornernessCriteria(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_CornernessCriteria {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_CornernessCriteria")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_CornernessCriteria, core::Algorithm, cv_cuda_CornernessCriteria_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_CornernessCriteria {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_CornernessCriteria {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CornernessCriteria, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_CornernessCriteriaTraitConst for CUDA_CornernessCriteria {
		#[inline] fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_CornernessCriteriaTrait for CUDA_CornernessCriteria {
		#[inline] fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CornernessCriteria, crate::cudaimgproc::CUDA_CornernessCriteriaTraitConst, as_raw_CUDA_CornernessCriteria, crate::cudaimgproc::CUDA_CornernessCriteriaTrait, as_raw_mut_CUDA_CornernessCriteria }

	/// Base class for Corners Detector. :
	pub struct CUDA_CornersDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_CornersDetector }

	impl Drop for CUDA_CornersDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_CornersDetector_delete(self.as_raw_mut_CUDA_CornersDetector()) };
		}
	}

	unsafe impl Send for CUDA_CornersDetector {}

	/// Constant methods for [crate::cudaimgproc::CUDA_CornersDetector]
	pub trait CUDA_CornersDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_CornersDetector(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_CornersDetector]
	pub trait CUDA_CornersDetectorTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_CornersDetectorTraitConst {
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
		#[inline]
		fn detect(&mut self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, mask: &impl ToInputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_CornersDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

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
		/// ## Note
		/// This alternative version of [CUDA_CornersDetectorTrait::detect] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def(&mut self, image: &impl ToInputArray, corners: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_CornersDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_corners(&mut self, max_corners: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornersDetector_setMaxCorners_int(self.as_raw_mut_CUDA_CornersDetector(), max_corners, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CornersDetector_setMinDistance_double(self.as_raw_mut_CUDA_CornersDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_CornersDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_CornersDetector")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_CornersDetector, core::Algorithm, cv_cuda_CornersDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_CornersDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_CornersDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CornersDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_CornersDetectorTraitConst for CUDA_CornersDetector {
		#[inline] fn as_raw_CUDA_CornersDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_CornersDetectorTrait for CUDA_CornersDetector {
		#[inline] fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_CornersDetector, crate::cudaimgproc::CUDA_CornersDetectorTraitConst, as_raw_CUDA_CornersDetector, crate::cudaimgproc::CUDA_CornersDetectorTrait, as_raw_mut_CUDA_CornersDetector }

	/// Base class for circles detector algorithm. :
	pub struct CUDA_HoughCirclesDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_HoughCirclesDetector }

	impl Drop for CUDA_HoughCirclesDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_HoughCirclesDetector_delete(self.as_raw_mut_CUDA_HoughCirclesDetector()) };
		}
	}

	unsafe impl Send for CUDA_HoughCirclesDetector {}

	/// Constant methods for [crate::cudaimgproc::CUDA_HoughCirclesDetector]
	pub trait CUDA_HoughCirclesDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void;

		#[inline]
		fn get_dp(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getDp_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_dist(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getMinDist_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_canny_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getCannyThreshold_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_votes_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getVotesThreshold_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_radius(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getMinRadius_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_radius(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getMaxRadius_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_circles(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_getMaxCircles_const(self.as_raw_CUDA_HoughCirclesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_HoughCirclesDetector]
	pub trait CUDA_HoughCirclesDetectorTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_HoughCirclesDetectorTraitConst {
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
		#[inline]
		fn detect(&mut self, src: &impl ToInputArray, circles: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(circles);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughCirclesDetector(), src.as_raw__InputArray(), circles.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

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
		/// ## Note
		/// This alternative version of [CUDA_HoughCirclesDetectorTrait::detect] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def(&mut self, src: &impl ToInputArray, circles: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(circles);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_HoughCirclesDetector(), src.as_raw__InputArray(), circles.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_dp(&mut self, dp: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setDp_float(self.as_raw_mut_CUDA_HoughCirclesDetector(), dp, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_dist(&mut self, min_dist: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setMinDist_float(self.as_raw_mut_CUDA_HoughCirclesDetector(), min_dist, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_canny_threshold(&mut self, canny_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setCannyThreshold_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), canny_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_votes_threshold(&mut self, votes_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setVotesThreshold_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), votes_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_radius(&mut self, min_radius: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setMinRadius_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), min_radius, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_radius(&mut self, max_radius: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setMaxRadius_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), max_radius, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_circles(&mut self, max_circles: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughCirclesDetector_setMaxCircles_int(self.as_raw_mut_CUDA_HoughCirclesDetector(), max_circles, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_HoughCirclesDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_HoughCirclesDetector")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_HoughCirclesDetector, core::Algorithm, cv_cuda_HoughCirclesDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_HoughCirclesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_HoughCirclesDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughCirclesDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTraitConst for CUDA_HoughCirclesDetector {
		#[inline] fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTrait for CUDA_HoughCirclesDetector {
		#[inline] fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughCirclesDetector, crate::cudaimgproc::CUDA_HoughCirclesDetectorTraitConst, as_raw_CUDA_HoughCirclesDetector, crate::cudaimgproc::CUDA_HoughCirclesDetectorTrait, as_raw_mut_CUDA_HoughCirclesDetector }

	/// Base class for lines detector algorithm. :
	pub struct CUDA_HoughLinesDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_HoughLinesDetector }

	impl Drop for CUDA_HoughLinesDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_HoughLinesDetector_delete(self.as_raw_mut_CUDA_HoughLinesDetector()) };
		}
	}

	unsafe impl Send for CUDA_HoughLinesDetector {}

	/// Constant methods for [crate::cudaimgproc::CUDA_HoughLinesDetector]
	pub trait CUDA_HoughLinesDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void;

		#[inline]
		fn get_rho(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_getRho_const(self.as_raw_CUDA_HoughLinesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_theta(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_getTheta_const(self.as_raw_CUDA_HoughLinesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_getThreshold_const(self.as_raw_CUDA_HoughLinesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_do_sort(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_getDoSort_const(self.as_raw_CUDA_HoughLinesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_lines(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_getMaxLines_const(self.as_raw_CUDA_HoughLinesDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_HoughLinesDetector]
	pub trait CUDA_HoughLinesDetectorTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_HoughLinesDetectorTraitConst {
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
		#[inline]
		fn detect(&mut self, src: &impl ToInputArray, lines: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(lines);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughLinesDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

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
		/// ## Note
		/// This alternative version of [CUDA_HoughLinesDetectorTrait::detect] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def(&mut self, src: &impl ToInputArray, lines: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(lines);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_HoughLinesDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn download_results(&mut self, d_lines: &impl ToInputArray, h_lines: &mut impl ToOutputArray, h_votes: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(d_lines);
			output_array_arg!(h_lines);
			output_array_arg!(h_votes);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughLinesDetector(), d_lines.as_raw__InputArray(), h_lines.as_raw__OutputArray(), h_votes.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Downloads results from cuda::HoughLinesDetector::detect to host memory.
		///
		/// ## Parameters
		/// * d_lines: Result of cuda::HoughLinesDetector::detect .
		/// * h_lines: Output host array.
		/// * h_votes: Optional output array for line's votes.
		/// * stream: Stream for the asynchronous version.
		///
		/// ## Note
		/// This alternative version of [CUDA_HoughLinesDetectorTrait::download_results] function uses the following default values for its arguments:
		/// * h_votes: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn download_results_def(&mut self, d_lines: &impl ToInputArray, h_lines: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(d_lines);
			output_array_arg!(h_lines);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_HoughLinesDetector(), d_lines.as_raw__InputArray(), h_lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_rho(&mut self, rho: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_setRho_float(self.as_raw_mut_CUDA_HoughLinesDetector(), rho, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_theta(&mut self, theta: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_setTheta_float(self.as_raw_mut_CUDA_HoughLinesDetector(), theta, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_setThreshold_int(self.as_raw_mut_CUDA_HoughLinesDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_do_sort(&mut self, do_sort: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_setDoSort_bool(self.as_raw_mut_CUDA_HoughLinesDetector(), do_sort, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_lines(&mut self, max_lines: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughLinesDetector_setMaxLines_int(self.as_raw_mut_CUDA_HoughLinesDetector(), max_lines, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_HoughLinesDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_HoughLinesDetector")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_HoughLinesDetector, core::Algorithm, cv_cuda_HoughLinesDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_HoughLinesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_HoughLinesDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughLinesDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_HoughLinesDetectorTraitConst for CUDA_HoughLinesDetector {
		#[inline] fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_HoughLinesDetectorTrait for CUDA_HoughLinesDetector {
		#[inline] fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughLinesDetector, crate::cudaimgproc::CUDA_HoughLinesDetectorTraitConst, as_raw_CUDA_HoughLinesDetector, crate::cudaimgproc::CUDA_HoughLinesDetectorTrait, as_raw_mut_CUDA_HoughLinesDetector }

	/// Base class for line segments detector algorithm. :
	pub struct CUDA_HoughSegmentDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_HoughSegmentDetector }

	impl Drop for CUDA_HoughSegmentDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_HoughSegmentDetector_delete(self.as_raw_mut_CUDA_HoughSegmentDetector()) };
		}
	}

	unsafe impl Send for CUDA_HoughSegmentDetector {}

	/// Constant methods for [crate::cudaimgproc::CUDA_HoughSegmentDetector]
	pub trait CUDA_HoughSegmentDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void;

		#[inline]
		fn get_rho(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getRho_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_theta(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getTheta_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_line_length(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getMinLineLength_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_line_gap(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getMaxLineGap_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_lines(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getMaxLines_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_getThreshold_const(self.as_raw_CUDA_HoughSegmentDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_HoughSegmentDetector]
	pub trait CUDA_HoughSegmentDetectorTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_HoughSegmentDetectorTraitConst {
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
		#[inline]
		fn detect(&mut self, src: &impl ToInputArray, lines: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(lines);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HoughSegmentDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

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
		/// ## Note
		/// This alternative version of [CUDA_HoughSegmentDetectorTrait::detect] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_def(&mut self, src: &impl ToInputArray, lines: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(lines);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_HoughSegmentDetector(), src.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_rho(&mut self, rho: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setRho_float(self.as_raw_mut_CUDA_HoughSegmentDetector(), rho, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_theta(&mut self, theta: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setTheta_float(self.as_raw_mut_CUDA_HoughSegmentDetector(), theta, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_line_length(&mut self, min_line_length: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setMinLineLength_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), min_line_length, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_line_gap(&mut self, max_line_gap: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setMaxLineGap_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), max_line_gap, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_lines(&mut self, max_lines: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setMaxLines_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), max_lines, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HoughSegmentDetector_setThreshold_int(self.as_raw_mut_CUDA_HoughSegmentDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_HoughSegmentDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_HoughSegmentDetector")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_HoughSegmentDetector, core::Algorithm, cv_cuda_HoughSegmentDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_HoughSegmentDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_HoughSegmentDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughSegmentDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTraitConst for CUDA_HoughSegmentDetector {
		#[inline] fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTrait for CUDA_HoughSegmentDetector {
		#[inline] fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_HoughSegmentDetector, crate::cudaimgproc::CUDA_HoughSegmentDetectorTraitConst, as_raw_CUDA_HoughSegmentDetector, crate::cudaimgproc::CUDA_HoughSegmentDetectorTrait, as_raw_mut_CUDA_HoughSegmentDetector }

	/// Base class for Template Matching. :
	pub struct CUDA_TemplateMatching {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_TemplateMatching }

	impl Drop for CUDA_TemplateMatching {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_TemplateMatching_delete(self.as_raw_mut_CUDA_TemplateMatching()) };
		}
	}

	unsafe impl Send for CUDA_TemplateMatching {}

	/// Constant methods for [crate::cudaimgproc::CUDA_TemplateMatching]
	pub trait CUDA_TemplateMatchingTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudaimgproc::CUDA_TemplateMatching]
	pub trait CUDA_TemplateMatchingTrait: core::AlgorithmTrait + crate::cudaimgproc::CUDA_TemplateMatchingTraitConst {
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
		#[inline]
		fn match_(&mut self, image: &impl ToInputArray, templ: &impl ToInputArray, result: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(templ);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_TemplateMatching(), image.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Computes a proximity map for a raster template and an image where the template is searched for.
		///
		/// ## Parameters
		/// * image: Source image.
		/// * templ: Template image with the size and type the same as image .
		/// * result: Map containing comparison results ( CV_32FC1 ). If image is *W x H* and templ is *w
		/// x h*, then result must be *W-w+1 x H-h+1*.
		/// * stream: Stream for the asynchronous version.
		///
		/// ## Note
		/// This alternative version of [CUDA_TemplateMatchingTrait::match_] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn match__def(&mut self, image: &impl ToInputArray, templ: &impl ToInputArray, result: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(templ);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_TemplateMatching(), image.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_TemplateMatching {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_TemplateMatching")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_TemplateMatching, core::Algorithm, cv_cuda_TemplateMatching_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_TemplateMatching {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_TemplateMatching {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_TemplateMatching, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudaimgproc::CUDA_TemplateMatchingTraitConst for CUDA_TemplateMatching {
		#[inline] fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudaimgproc::CUDA_TemplateMatchingTrait for CUDA_TemplateMatching {
		#[inline] fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_TemplateMatching, crate::cudaimgproc::CUDA_TemplateMatchingTraitConst, as_raw_CUDA_TemplateMatching, crate::cudaimgproc::CUDA_TemplateMatchingTrait, as_raw_mut_CUDA_TemplateMatching }

}
