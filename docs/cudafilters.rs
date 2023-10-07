pub mod cudafilters {
	//! # Image Filtering
	//! 
	//! Functions and classes described in this section are used to perform various linear or non-linear
	//! filtering operations on 2D images.
	//! 
	//! 
	//! Note:
	//!    *   An example containing all basic morphology operators like erode and dilate can be found at
	//!        opencv_source_code/samples/gpu/morphology.cpp
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::FilterTraitConst, super::FilterTrait };
	}
	
	/// Creates a normalized 2D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1, CV_8UC4 and CV_32FC1 are supported for now.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value Point(-1, -1) means that the anchor is at the kernel
	/// center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// boxFilter
	/// 
	/// ## Note
	/// This alternative version of [create_box_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_filter_def(src_type: i32, dst_type: i32, ksize: core::Size) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxFilter_int_int_Size(src_type, dst_type, ksize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a normalized 2D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1, CV_8UC4 and CV_32FC1 are supported for now.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value Point(-1, -1) means that the anchor is at the kernel
	/// center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// boxFilter
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_filter(src_type: i32, dst_type: i32, ksize: core::Size, anchor: core::Point, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxFilter_int_int_Size_Point_int_Scalar(src_type, dst_type, ksize.opencv_as_extern(), anchor.opencv_as_extern(), border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates the maximum filter.
	/// 
	/// ## Parameters
	/// * srcType: Input/output image type. Only CV_8UC1 and CV_8UC4 are supported.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## Note
	/// This alternative version of [create_box_max_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_max_filter_def(src_type: i32, ksize: core::Size) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxMaxFilter_int_Size(src_type, ksize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates the maximum filter.
	/// 
	/// ## Parameters
	/// * srcType: Input/output image type. Only CV_8UC1 and CV_8UC4 are supported.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_max_filter(src_type: i32, ksize: core::Size, anchor: core::Point, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxMaxFilter_int_Size_Point_int_Scalar(src_type, ksize.opencv_as_extern(), anchor.opencv_as_extern(), border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates the minimum filter.
	/// 
	/// ## Parameters
	/// * srcType: Input/output image type. Only CV_8UC1 and CV_8UC4 are supported.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## Note
	/// This alternative version of [create_box_min_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_min_filter_def(src_type: i32, ksize: core::Size) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxMinFilter_int_Size(src_type, ksize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates the minimum filter.
	/// 
	/// ## Parameters
	/// * srcType: Input/output image type. Only CV_8UC1 and CV_8UC4 are supported.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_box_min_filter(src_type: i32, ksize: core::Size, anchor: core::Point, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBoxMinFilter_int_Size_Point_int_Scalar(src_type, ksize.opencv_as_extern(), anchor.opencv_as_extern(), border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a vertical 1D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1 type is supported for now.
	/// * dstType: Output image type. Only CV_32FC1 type is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## Note
	/// This alternative version of [create_column_sum_filter] function uses the following default values for its arguments:
	/// * anchor: -1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_column_sum_filter_def(src_type: i32, dst_type: i32, ksize: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createColumnSumFilter_int_int_int(src_type, dst_type, ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a vertical 1D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1 type is supported for now.
	/// * dstType: Output image type. Only CV_32FC1 type is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## C++ default parameters
	/// * anchor: -1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_column_sum_filter(src_type: i32, dst_type: i32, ksize: i32, anchor: i32, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createColumnSumFilter_int_int_int_int_int_Scalar(src_type, dst_type, ksize, anchor, border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a generalized Deriv operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Derivative order in respect of x.
	/// * dy: Derivative order in respect of y.
	/// * ksize: Aperture size. See getDerivKernels for details.
	/// * normalize: Flag indicating whether to normalize (scale down) the filter coefficients or not.
	/// See getDerivKernels for details.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. For details, see getDerivKernels .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// 
	/// ## Note
	/// This alternative version of [create_deriv_filter] function uses the following default values for its arguments:
	/// * normalize: false
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_deriv_filter_def(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createDerivFilter_int_int_int_int_int(src_type, dst_type, dx, dy, ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a generalized Deriv operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Derivative order in respect of x.
	/// * dy: Derivative order in respect of y.
	/// * ksize: Aperture size. See getDerivKernels for details.
	/// * normalize: Flag indicating whether to normalize (scale down) the filter coefficients or not.
	/// See getDerivKernels for details.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. For details, see getDerivKernels .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// 
	/// ## C++ default parameters
	/// * normalize: false
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_deriv_filter(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32, normalize: bool, scale: f64, row_border_mode: i32, column_border_mode: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createDerivFilter_int_int_int_int_int_bool_double_int_int(src_type, dst_type, dx, dy, ksize, normalize, scale, row_border_mode, column_border_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Gaussian filter.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * ksize: Aperture size. See getGaussianKernel for details.
	/// * sigma1: Gaussian sigma in the horizontal direction. See getGaussianKernel for details.
	/// * sigma2: Gaussian sigma in the vertical direction. If 0, then
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsigma2%7D%5Cleftarrow%5Ctexttt%7Bsigma1%7D) .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// GaussianBlur
	/// 
	/// ## Note
	/// This alternative version of [create_gaussian_filter] function uses the following default values for its arguments:
	/// * sigma2: 0
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_gaussian_filter_def(src_type: i32, dst_type: i32, ksize: core::Size, sigma1: f64) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGaussianFilter_int_int_Size_double(src_type, dst_type, ksize.opencv_as_extern(), sigma1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Gaussian filter.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * ksize: Aperture size. See getGaussianKernel for details.
	/// * sigma1: Gaussian sigma in the horizontal direction. See getGaussianKernel for details.
	/// * sigma2: Gaussian sigma in the vertical direction. If 0, then
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsigma2%7D%5Cleftarrow%5Ctexttt%7Bsigma1%7D) .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// GaussianBlur
	/// 
	/// ## C++ default parameters
	/// * sigma2: 0
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_gaussian_filter(src_type: i32, dst_type: i32, ksize: core::Size, sigma1: f64, sigma2: f64, row_border_mode: i32, column_border_mode: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createGaussianFilter_int_int_Size_double_double_int_int(src_type, dst_type, ksize.opencv_as_extern(), sigma1, sigma2, row_border_mode, column_border_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Laplacian operator.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Supports CV_8U , CV_16U and CV_32F one and four channel image.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * ksize: Aperture size used to compute the second-derivative filters (see getDerivKernels). It
	/// must be positive and odd. Only ksize = 1 and ksize = 3 are supported.
	/// * scale: Optional scale factor for the computed Laplacian values. By default, no scaling is
	/// applied (see getDerivKernels ).
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// Laplacian
	/// 
	/// ## Note
	/// This alternative version of [create_laplacian_filter] function uses the following default values for its arguments:
	/// * ksize: 1
	/// * scale: 1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_laplacian_filter_def(src_type: i32, dst_type: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createLaplacianFilter_int_int(src_type, dst_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Laplacian operator.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Supports CV_8U , CV_16U and CV_32F one and four channel image.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * ksize: Aperture size used to compute the second-derivative filters (see getDerivKernels). It
	/// must be positive and odd. Only ksize = 1 and ksize = 3 are supported.
	/// * scale: Optional scale factor for the computed Laplacian values. By default, no scaling is
	/// applied (see getDerivKernels ).
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// Laplacian
	/// 
	/// ## C++ default parameters
	/// * ksize: 1
	/// * scale: 1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_laplacian_filter(src_type: i32, dst_type: i32, ksize: i32, scale: f64, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createLaplacianFilter_int_int_int_double_int_Scalar(src_type, dst_type, ksize, scale, border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a non-separable linear 2D filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Supports CV_8U , CV_16U and CV_32F one and four channel image.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * kernel: 2D array of filter coefficients.
	/// * anchor: Anchor point. The default value Point(-1, -1) means that the anchor is at the kernel
	/// center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// filter2D
	/// 
	/// ## Note
	/// This alternative version of [create_linear_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_linear_filter_def(src_type: i32, dst_type: i32, kernel: &impl core::ToInputArray) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createLinearFilter_int_int_const__InputArrayR(src_type, dst_type, kernel.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a non-separable linear 2D filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Supports CV_8U , CV_16U and CV_32F one and four channel image.
	/// * dstType: Output image type. Only the same type as src is supported for now.
	/// * kernel: 2D array of filter coefficients.
	/// * anchor: Anchor point. The default value Point(-1, -1) means that the anchor is at the kernel
	/// center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// ## See also
	/// filter2D
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_linear_filter(src_type: i32, dst_type: i32, kernel: &impl core::ToInputArray, anchor: core::Point, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createLinearFilter_int_int_const__InputArrayR_Point_int_Scalar(src_type, dst_type, kernel.as_raw__InputArray(), anchor.opencv_as_extern(), border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs median filtering for each point of the source image.
	/// 
	/// ## Parameters
	/// * srcType: type of of source image. Only CV_8UC1 images are supported for now.
	/// * windowSize: Size of the kernerl used for the filtering. Uses a (windowSize x windowSize) filter.
	/// * partition: Specifies the parallel granularity of the workload. This parameter should be used GPU experts when optimizing performance.
	/// 
	/// Outputs an image that has been filtered using a median-filtering formulation.
	/// 
	/// Details on this algorithm can be found in:
	/// Green, O., 2017. "Efficient scalable median filtering using histogram-based operations",
	///                   IEEE Transactions on Image Processing, 27(5), pp.2217-2228.
	/// 
	/// ## Note
	/// This alternative version of [create_median_filter] function uses the following default values for its arguments:
	/// * partition: 128
	#[inline]
	pub fn create_median_filter_def(src_type: i32, window_size: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMedianFilter_int_int(src_type, window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs median filtering for each point of the source image.
	/// 
	/// ## Parameters
	/// * srcType: type of of source image. Only CV_8UC1 images are supported for now.
	/// * windowSize: Size of the kernerl used for the filtering. Uses a (windowSize x windowSize) filter.
	/// * partition: Specifies the parallel granularity of the workload. This parameter should be used GPU experts when optimizing performance.
	/// 
	/// Outputs an image that has been filtered using a median-filtering formulation.
	/// 
	/// Details on this algorithm can be found in:
	/// Green, O., 2017. "Efficient scalable median filtering using histogram-based operations",
	///                   IEEE Transactions on Image Processing, 27(5), pp.2217-2228.
	/// 
	/// ## C++ default parameters
	/// * partition: 128
	#[inline]
	pub fn create_median_filter(src_type: i32, window_size: i32, partition: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMedianFilter_int_int_int(src_type, window_size, partition, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a 2D morphological filter.
	/// 
	/// ## Parameters
	/// * op: Type of morphological operation. The following types are possible:
	/// *   **MORPH_ERODE** erode
	/// *   **MORPH_DILATE** dilate
	/// *   **MORPH_OPEN** opening
	/// *   **MORPH_CLOSE** closing
	/// *   **MORPH_GRADIENT** morphological gradient
	/// *   **MORPH_TOPHAT** "top hat"
	/// *   **MORPH_BLACKHAT** "black hat"
	/// * srcType: Input/output image type. Only CV_8UC1, CV_8UC4, CV_32FC1 and CV_32FC4 are supported.
	/// * kernel: 2D 8-bit structuring element for the morphological operation.
	/// * anchor: Anchor position within the structuring element. Negative values mean that the anchor
	/// is at the center.
	/// * iterations: Number of times erosion and dilation to be applied.
	/// ## See also
	/// morphologyEx
	/// 
	/// ## Note
	/// This alternative version of [create_morphology_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	#[inline]
	pub fn create_morphology_filter_def(op: i32, src_type: i32, kernel: &impl core::ToInputArray) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMorphologyFilter_int_int_const__InputArrayR(op, src_type, kernel.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a 2D morphological filter.
	/// 
	/// ## Parameters
	/// * op: Type of morphological operation. The following types are possible:
	/// *   **MORPH_ERODE** erode
	/// *   **MORPH_DILATE** dilate
	/// *   **MORPH_OPEN** opening
	/// *   **MORPH_CLOSE** closing
	/// *   **MORPH_GRADIENT** morphological gradient
	/// *   **MORPH_TOPHAT** "top hat"
	/// *   **MORPH_BLACKHAT** "black hat"
	/// * srcType: Input/output image type. Only CV_8UC1, CV_8UC4, CV_32FC1 and CV_32FC4 are supported.
	/// * kernel: 2D 8-bit structuring element for the morphological operation.
	/// * anchor: Anchor position within the structuring element. Negative values mean that the anchor
	/// is at the center.
	/// * iterations: Number of times erosion and dilation to be applied.
	/// ## See also
	/// morphologyEx
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	#[inline]
	pub fn create_morphology_filter(op: i32, src_type: i32, kernel: &impl core::ToInputArray, anchor: core::Point, iterations: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createMorphologyFilter_int_int_const__InputArrayR_Point_int(op, src_type, kernel.as_raw__InputArray(), anchor.opencv_as_extern(), iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a horizontal 1D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1 type is supported for now.
	/// * dstType: Output image type. Only CV_32FC1 type is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## Note
	/// This alternative version of [create_row_sum_filter] function uses the following default values for its arguments:
	/// * anchor: -1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_row_sum_filter_def(src_type: i32, dst_type: i32, ksize: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createRowSumFilter_int_int_int(src_type, dst_type, ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a horizontal 1D box filter.
	/// 
	/// ## Parameters
	/// * srcType: Input image type. Only CV_8UC1 type is supported for now.
	/// * dstType: Output image type. Only CV_32FC1 type is supported for now.
	/// * ksize: Kernel size.
	/// * anchor: Anchor point. The default value (-1) means that the anchor is at the kernel center.
	/// * borderMode: Pixel extrapolation method. For details, see borderInterpolate .
	/// * borderVal: Default border value.
	/// 
	/// ## C++ default parameters
	/// * anchor: -1
	/// * border_mode: BORDER_DEFAULT
	/// * border_val: Scalar::all(0)
	#[inline]
	pub fn create_row_sum_filter(src_type: i32, dst_type: i32, ksize: i32, anchor: i32, border_mode: i32, border_val: core::Scalar) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createRowSumFilter_int_int_int_int_int_Scalar(src_type, dst_type, ksize, anchor, border_mode, border_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a vertical or horizontal Scharr operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Order of the derivative x.
	/// * dy: Order of the derivative y.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. See getDerivKernels for details.
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// Scharr
	/// 
	/// ## Note
	/// This alternative version of [create_scharr_filter] function uses the following default values for its arguments:
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_scharr_filter_def(src_type: i32, dst_type: i32, dx: i32, dy: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createScharrFilter_int_int_int_int(src_type, dst_type, dx, dy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a vertical or horizontal Scharr operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Order of the derivative x.
	/// * dy: Order of the derivative y.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. See getDerivKernels for details.
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// Scharr
	/// 
	/// ## C++ default parameters
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_scharr_filter(src_type: i32, dst_type: i32, dx: i32, dy: i32, scale: f64, row_border_mode: i32, column_border_mode: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createScharrFilter_int_int_int_int_double_int_int(src_type, dst_type, dx, dy, scale, row_border_mode, column_border_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a separable linear filter.
	/// 
	/// ## Parameters
	/// * srcType: Source array type.
	/// * dstType: Destination array type.
	/// * rowKernel: Horizontal filter coefficients. Support kernels with size \<= 32 .
	/// * columnKernel: Vertical filter coefficients. Support kernels with size \<= 32 .
	/// * anchor: Anchor position within the kernel. Negative values mean that anchor is positioned at
	/// the aperture center.
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// sepFilter2D
	/// 
	/// ## Note
	/// This alternative version of [create_separable_linear_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_separable_linear_filter_def(src_type: i32, dst_type: i32, row_kernel: &impl core::ToInputArray, column_kernel: &impl core::ToInputArray) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(row_kernel);
		input_array_arg!(column_kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR(src_type, dst_type, row_kernel.as_raw__InputArray(), column_kernel.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a separable linear filter.
	/// 
	/// ## Parameters
	/// * srcType: Source array type.
	/// * dstType: Destination array type.
	/// * rowKernel: Horizontal filter coefficients. Support kernels with size \<= 32 .
	/// * columnKernel: Vertical filter coefficients. Support kernels with size \<= 32 .
	/// * anchor: Anchor position within the kernel. Negative values mean that anchor is positioned at
	/// the aperture center.
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// sepFilter2D
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_separable_linear_filter(src_type: i32, dst_type: i32, row_kernel: &impl core::ToInputArray, column_kernel: &impl core::ToInputArray, anchor: core::Point, row_border_mode: i32, column_border_mode: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		input_array_arg!(row_kernel);
		input_array_arg!(column_kernel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR_Point_int_int(src_type, dst_type, row_kernel.as_raw__InputArray(), column_kernel.as_raw__InputArray(), anchor.opencv_as_extern(), row_border_mode, column_border_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Sobel operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Derivative order in respect of x.
	/// * dy: Derivative order in respect of y.
	/// * ksize: Size of the extended Sobel kernel. Possible values are 1, 3, 5 or 7.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. For details, see getDerivKernels .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// Sobel
	/// 
	/// ## Note
	/// This alternative version of [create_sobel_filter] function uses the following default values for its arguments:
	/// * ksize: 3
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_sobel_filter_def(src_type: i32, dst_type: i32, dx: i32, dy: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createSobelFilter_int_int_int_int(src_type, dst_type, dx, dy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates a Sobel operator.
	/// 
	/// ## Parameters
	/// * srcType: Source image type.
	/// * dstType: Destination array type.
	/// * dx: Derivative order in respect of x.
	/// * dy: Derivative order in respect of y.
	/// * ksize: Size of the extended Sobel kernel. Possible values are 1, 3, 5 or 7.
	/// * scale: Optional scale factor for the computed derivative values. By default, no scaling is
	/// applied. For details, see getDerivKernels .
	/// * rowBorderMode: Pixel extrapolation method in the vertical direction. For details, see
	/// borderInterpolate.
	/// * columnBorderMode: Pixel extrapolation method in the horizontal direction.
	/// ## See also
	/// Sobel
	/// 
	/// ## C++ default parameters
	/// * ksize: 3
	/// * scale: 1
	/// * row_border_mode: BORDER_DEFAULT
	/// * column_border_mode: -1
	#[inline]
	pub fn create_sobel_filter(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32, scale: f64, row_border_mode: i32, column_border_mode: i32) -> Result<core::Ptr<crate::cudafilters::Filter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createSobelFilter_int_int_int_int_int_double_int_int(src_type, dst_type, dx, dy, ksize, scale, row_border_mode, column_border_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudafilters::Filter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Constant methods for [crate::cudafilters::Filter]
	pub trait FilterTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Filter(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudafilters::Filter]
	pub trait FilterTrait: core::AlgorithmTrait + crate::cudafilters::FilterTraitConst {
		fn as_raw_mut_Filter(&mut self) -> *mut c_void;
	
		/// Applies the specified filter to the image.
		/// 
		/// ## Parameters
		/// * src: Input image.
		/// * dst: Output image.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn apply(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_Filter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Applies the specified filter to the image.
		/// 
		/// ## Parameters
		/// * src: Input image.
		/// * dst: Output image.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn apply_def(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Filter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Common interface for all CUDA filters :
	pub struct Filter {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Filter }
	
	impl Drop for Filter {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_Filter_delete(self.as_raw_mut_Filter()) };
		}
	}
	
	unsafe impl Send for Filter {}
	
	impl core::AlgorithmTraitConst for Filter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Filter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafilters::FilterTraitConst for Filter {
		#[inline] fn as_raw_Filter(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafilters::FilterTrait for Filter {
		#[inline] fn as_raw_mut_Filter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Filter {
	}
	
	boxed_cast_base! { Filter, core::Algorithm, cv_cuda_Filter_to_Algorithm }
	
	impl std::fmt::Debug for Filter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Filter")
				.finish()
		}
	}
}
