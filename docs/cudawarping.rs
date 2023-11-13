pub mod cudawarping {
	//! # Image Warping
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use {  };
	}
	
	/// ## Note
	/// This alternative version of [build_warp_affine_maps_2] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps_2_def(mut m: core::Mat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_Mat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps_2(mut m: core::Mat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_Mat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [build_warp_affine_maps_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps_1_def(mut m: core::UMat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_UMat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps_1(mut m: core::UMat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_UMat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Builds transformation maps for affine transformation.
	/// 
	/// ## Parameters
	/// * M: *2x3* Mat or UMat transformation matrix.
	/// * inverse: Flag specifying that M is an inverse transformation ( dst=\>src ).
	/// * dsize: Size of the destination image.
	/// * xmap: X values with CV_32FC1 type.
	/// * ymap: Y values with CV_32FC1 type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpAffine , cuda::remap
	/// 
	/// ## Note
	/// This alternative version of [build_warp_affine_maps] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps_def(m: &impl core::ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(m);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(m.as_raw__InputArray(), inverse, dsize.opencv_as_extern(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Builds transformation maps for affine transformation.
	/// 
	/// ## Parameters
	/// * M: *2x3* Mat or UMat transformation matrix.
	/// * inverse: Flag specifying that M is an inverse transformation ( dst=\>src ).
	/// * dsize: Size of the destination image.
	/// * xmap: X values with CV_32FC1 type.
	/// * ymap: Y values with CV_32FC1 type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpAffine , cuda::remap
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_affine_maps(m: &impl core::ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(m);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(m.as_raw__InputArray(), inverse, dsize.opencv_as_extern(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [build_warp_perspective_maps_2] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps_2_def(mut m: core::Mat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_Mat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps_2(mut m: core::Mat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_Mat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [build_warp_perspective_maps_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps_1_def(mut m: core::UMat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_UMat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps_1(mut m: core::UMat, inverse: bool, dsize: core::Size, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_UMat(), inverse, dsize.opencv_as_extern(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Builds transformation maps for perspective transformation.
	/// 
	/// ## Parameters
	/// * M: *3x3* Mat or UMat transformation matrix.
	/// * inverse: Flag specifying that M is an inverse transformation ( dst=\>src ).
	/// * dsize: Size of the destination image.
	/// * xmap: X values with CV_32FC1 type.
	/// * ymap: Y values with CV_32FC1 type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpPerspective , cuda::remap
	/// 
	/// ## Note
	/// This alternative version of [build_warp_perspective_maps] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps_def(m: &impl core::ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(m);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(m.as_raw__InputArray(), inverse, dsize.opencv_as_extern(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Builds transformation maps for perspective transformation.
	/// 
	/// ## Parameters
	/// * M: *3x3* Mat or UMat transformation matrix.
	/// * inverse: Flag specifying that M is an inverse transformation ( dst=\>src ).
	/// * dsize: Size of the destination image.
	/// * xmap: X values with CV_32FC1 type.
	/// * ymap: Y values with CV_32FC1 type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpPerspective , cuda::remap
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn build_warp_perspective_maps(m: &impl core::ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(m);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(m.as_raw__InputArray(), inverse, dsize.opencv_as_extern(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Smoothes an image and downsamples it.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image. Will have Size((src.cols+1)/2, (src.rows+1)/2) size and the same
	/// type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// pyrDown
	/// 
	/// ## Note
	/// This alternative version of [pyr_down] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn pyr_down_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pyrDown_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Smoothes an image and downsamples it.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image. Will have Size((src.cols+1)/2, (src.rows+1)/2) size and the same
	/// type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// pyrDown
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn pyr_down(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pyrDown_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Upsamples an image and then smoothes it.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image. Will have Size(src.cols\*2, src.rows\*2) size and the same type as
	/// src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [pyr_up] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn pyr_up_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pyrUp_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Upsamples an image and then smoothes it.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image. Will have Size(src.cols\*2, src.rows\*2) size and the same type as
	/// src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn pyr_up(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pyrUp_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a generic geometrical transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image with the size the same as xmap and the type the same as src .
	/// * xmap: X values. Only CV_32FC1 type is supported.
	/// * ymap: Y values. Only CV_32FC1 type is supported.
	/// * interpolation: Interpolation method (see resize ). INTER_NEAREST , INTER_LINEAR and
	/// INTER_CUBIC are supported for now.
	/// * borderMode: Pixel extrapolation method (see borderInterpolate ). BORDER_REFLECT101 ,
	/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
	/// * borderValue: Value used in case of a constant border. By default, it is 0.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function transforms the source image using the specified map:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28xmap%28x%2Cy%29%2C%20ymap%28x%2Cy%29%29)
	/// 
	/// Values of pixels with non-integer coordinates are computed using the bilinear interpolation.
	/// ## See also
	/// remap
	/// 
	/// ## Note
	/// This alternative version of [remap] function uses the following default values for its arguments:
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn remap_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, xmap: &impl core::ToInputArray, ymap: &impl core::ToInputArray, interpolation: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(xmap);
		input_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), xmap.as_raw__InputArray(), ymap.as_raw__InputArray(), interpolation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a generic geometrical transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image with the size the same as xmap and the type the same as src .
	/// * xmap: X values. Only CV_32FC1 type is supported.
	/// * ymap: Y values. Only CV_32FC1 type is supported.
	/// * interpolation: Interpolation method (see resize ). INTER_NEAREST , INTER_LINEAR and
	/// INTER_CUBIC are supported for now.
	/// * borderMode: Pixel extrapolation method (see borderInterpolate ). BORDER_REFLECT101 ,
	/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
	/// * borderValue: Value used in case of a constant border. By default, it is 0.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function transforms the source image using the specified map:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28xmap%28x%2Cy%29%2C%20ymap%28x%2Cy%29%29)
	/// 
	/// Values of pixels with non-integer coordinates are computed using the bilinear interpolation.
	/// ## See also
	/// remap
	/// 
	/// ## C++ default parameters
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn remap(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, xmap: &impl core::ToInputArray, ymap: &impl core::ToInputArray, interpolation: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(xmap);
		input_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), xmap.as_raw__InputArray(), ymap.as_raw__InputArray(), interpolation, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Resizes an image.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image with the same type as src . The size is dsize (when it is non-zero)
	/// or the size is computed from src.size() , fx , and fy .
	/// * dsize: Destination image size. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdsize%20%3D%20Size%28round%28fx%2Asrc%2Ecols%29%2C%20round%28fy%2Asrc%2Erows%29%29%7D)
	/// Either dsize or both fx and fy must be non-zero.
	/// * fx: Scale factor along the horizontal axis. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Ewidth%2Fsrc%2Ecols%7D)
	/// * fy: Scale factor along the vertical axis. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Eheight%2Fsrc%2Erows%7D)
	/// * interpolation: Interpolation method. INTER_NEAREST , INTER_LINEAR and INTER_CUBIC are
	/// supported for now.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// resize
	/// 
	/// ## Note
	/// This alternative version of [resize] function uses the following default values for its arguments:
	/// * fx: 0
	/// * fy: 0
	/// * interpolation: INTER_LINEAR
	/// * stream: Stream::Null()
	#[inline]
	pub fn resize_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Resizes an image.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dst: Destination image with the same type as src . The size is dsize (when it is non-zero)
	/// or the size is computed from src.size() , fx , and fy .
	/// * dsize: Destination image size. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdsize%20%3D%20Size%28round%28fx%2Asrc%2Ecols%29%2C%20round%28fy%2Asrc%2Erows%29%29%7D)
	/// Either dsize or both fx and fy must be non-zero.
	/// * fx: Scale factor along the horizontal axis. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Ewidth%2Fsrc%2Ecols%7D)
	/// * fy: Scale factor along the vertical axis. If it is zero, it is computed as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Eheight%2Fsrc%2Erows%7D)
	/// * interpolation: Interpolation method. INTER_NEAREST , INTER_LINEAR and INTER_CUBIC are
	/// supported for now.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// resize
	/// 
	/// ## C++ default parameters
	/// * fx: 0
	/// * fy: 0
	/// * interpolation: INTER_LINEAR
	/// * stream: Stream::Null()
	#[inline]
	pub fn resize(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dsize: core::Size, fx: f64, fy: f64, interpolation: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dsize.opencv_as_extern(), fx, fy, interpolation, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Rotates an image around the origin (0,0) and then shifts it.
	/// 
	/// ## Parameters
	/// * src: Source image. Supports 1, 3 or 4 channels images with CV_8U , CV_16U or CV_32F
	/// depth.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * dsize: Size of the destination image.
	/// * angle: Angle of rotation in degrees.
	/// * xShift: Shift along the horizontal axis.
	/// * yShift: Shift along the vertical axis.
	/// * interpolation: Interpolation method. Only INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC
	/// are supported.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpAffine
	/// 
	/// ## Note
	/// This alternative version of [rotate] function uses the following default values for its arguments:
	/// * x_shift: 0
	/// * y_shift: 0
	/// * interpolation: INTER_LINEAR
	/// * stream: Stream::Null()
	#[inline]
	pub fn rotate_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dsize: core::Size, angle: f64) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dsize.opencv_as_extern(), angle, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Rotates an image around the origin (0,0) and then shifts it.
	/// 
	/// ## Parameters
	/// * src: Source image. Supports 1, 3 or 4 channels images with CV_8U , CV_16U or CV_32F
	/// depth.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * dsize: Size of the destination image.
	/// * angle: Angle of rotation in degrees.
	/// * xShift: Shift along the horizontal axis.
	/// * yShift: Shift along the vertical axis.
	/// * interpolation: Interpolation method. Only INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC
	/// are supported.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cuda::warpAffine
	/// 
	/// ## C++ default parameters
	/// * x_shift: 0
	/// * y_shift: 0
	/// * interpolation: INTER_LINEAR
	/// * stream: Stream::Null()
	#[inline]
	pub fn rotate(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dsize: core::Size, angle: f64, x_shift: f64, y_shift: f64, interpolation: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double_double_double_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dsize.opencv_as_extern(), angle, x_shift, y_shift, interpolation, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [warp_affine_2] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine_2_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::Mat, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine_2(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [warp_affine_1] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine_1_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::UMat, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine_1(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::UMat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies an affine transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8U , CV_16U , CV_32S , or CV_32F depth and 1, 3, or 4 channels are
	/// supported.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * M: *2x3* Mat or UMat transformation matrix.
	/// * dsize: Size of the destination image.
	/// * flags: Combination of interpolation methods (see resize) and the optional flag
	/// WARP_INVERSE_MAP specifying that M is an inverse transformation ( dst=\>src ). Only
	/// INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC interpolation methods are supported.
	/// * borderMode: 
	/// * borderValue: 
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// warpAffine
	/// 
	/// ## Note
	/// This alternative version of [warp_affine] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, m: &impl core::ToInputArray, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies an affine transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8U , CV_16U , CV_32S , or CV_32F depth and 1, 3, or 4 channels are
	/// supported.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * M: *2x3* Mat or UMat transformation matrix.
	/// * dsize: Size of the destination image.
	/// * flags: Combination of interpolation methods (see resize) and the optional flag
	/// WARP_INVERSE_MAP specifying that M is an inverse transformation ( dst=\>src ). Only
	/// INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC interpolation methods are supported.
	/// * borderMode: 
	/// * borderValue: 
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// warpAffine
	/// 
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_affine(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, m: &impl core::ToInputArray, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [warp_perspective_2] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective_2_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::Mat, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective_2(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [warp_perspective_1] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective_1_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::UMat, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective_1(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mut m: core::UMat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a perspective transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8U , CV_16U , CV_32S , or CV_32F depth and 1, 3, or 4 channels are
	/// supported.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * M: *3x3* Mat or UMat transformation matrix.
	/// * dsize: Size of the destination image.
	/// * flags: Combination of interpolation methods (see resize ) and the optional flag
	/// WARP_INVERSE_MAP specifying that M is the inverse transformation ( dst =\> src ). Only
	/// INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC interpolation methods are supported.
	/// * borderMode: 
	/// * borderValue: 
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// warpPerspective
	/// 
	/// ## Note
	/// This alternative version of [warp_perspective] function uses the following default values for its arguments:
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, m: &impl core::ToInputArray, dsize: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), dsize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a perspective transformation to an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8U , CV_16U , CV_32S , or CV_32F depth and 1, 3, or 4 channels are
	/// supported.
	/// * dst: Destination image with the same type as src . The size is dsize .
	/// * M: *3x3* Mat or UMat transformation matrix.
	/// * dsize: Size of the destination image.
	/// * flags: Combination of interpolation methods (see resize ) and the optional flag
	/// WARP_INVERSE_MAP specifying that M is the inverse transformation ( dst =\> src ). Only
	/// INTER_NEAREST , INTER_LINEAR , and INTER_CUBIC interpolation methods are supported.
	/// * borderMode: 
	/// * borderValue: 
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// warpPerspective
	/// 
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn warp_perspective(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, m: &impl core::ToInputArray, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), dsize.opencv_as_extern(), flags, border_mode, border_value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
}
