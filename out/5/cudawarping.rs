//! # Image Warping
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
}

/// ## Note
/// This alternative version of [build_warp_affine_maps_2] function uses the following default values for its arguments:
/// * stream: Stream::Null()
// cv::cuda::buildWarpAffineMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:162
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
#[inline]
pub fn build_warp_affine_maps_2_def(mut m: impl core::MatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_Mat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * stream: Stream::Null()
// buildWarpAffineMaps(Mat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:162
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_affine_maps_2(mut m: impl core::MatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_Mat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [build_warp_affine_maps_1] function uses the following default values for its arguments:
/// * stream: Stream::Null()
// cv::cuda::buildWarpAffineMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:158
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
#[inline]
pub fn build_warp_affine_maps_1_def(mut m: impl core::UMatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_UMat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * stream: Stream::Null()
// buildWarpAffineMaps(UMat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:158
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_affine_maps_1(mut m: impl core::UMatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_UMat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::buildWarpAffineMaps(InputArray, Primitive, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:156
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn build_warp_affine_maps_def(m: &impl ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(xmap);
	output_array_arg!(ymap);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(m.as_raw__InputArray(), inverse, &dsize, xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
// buildWarpAffineMaps(InputArray, bool, Size, OutputArray, OutputArray, Stream &)(InputArray, Primitive, SimpleClass, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:156
// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_affine_maps(m: &impl ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(xmap);
	output_array_arg!(ymap);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(m.as_raw__InputArray(), inverse, &dsize, xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [build_warp_perspective_maps_2] function uses the following default values for its arguments:
/// * stream: Stream::Null()
// cv::cuda::buildWarpPerspectiveMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:212
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps_2_def(mut m: impl core::MatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_Mat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * stream: Stream::Null()
// buildWarpPerspectiveMaps(Mat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:212
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps_2(mut m: impl core::MatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_Mat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [build_warp_perspective_maps_1] function uses the following default values for its arguments:
/// * stream: Stream::Null()
// cv::cuda::buildWarpPerspectiveMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:208
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps_1_def(mut m: impl core::UMatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR(m.as_raw_mut_UMat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * stream: Stream::Null()
// buildWarpPerspectiveMaps(UMat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:208
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps_1(mut m: impl core::UMatTrait, inverse: bool, dsize: core::Size, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(m.as_raw_mut_UMat(), inverse, &dsize, xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::buildWarpPerspectiveMaps(InputArray, Primitive, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:206
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps_def(m: &impl ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(xmap);
	output_array_arg!(ymap);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(m.as_raw__InputArray(), inverse, &dsize, xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
// buildWarpPerspectiveMaps(InputArray, bool, Size, OutputArray, OutputArray, Stream &)(InputArray, Primitive, SimpleClass, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:206
// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn build_warp_perspective_maps(m: &impl ToInputArray, inverse: bool, dsize: core::Size, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(xmap);
	output_array_arg!(ymap);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(m.as_raw__InputArray(), inverse, &dsize, xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::pyrDown(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:243
// ("cv::cuda::pyrDown", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn pyr_down_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
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
// pyrDown(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:243
// ("cv::cuda::pyrDown", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn pyr_down(src: &impl ToInputArray, dst: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
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
// cv::cuda::pyrUp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:252
// ("cv::cuda::pyrUp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn pyr_up_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
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
// pyrUp(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:252
// ("cv::cuda::pyrUp", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn pyr_up(src: &impl ToInputArray, dst: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
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
/// The extra flag WARP_RELATIVE_MAP can be ORed to the interpolation method
/// (e.g. INTER_LINEAR | WARP_RELATIVE_MAP)
/// * borderMode: Pixel extrapolation method (see borderInterpolate ). BORDER_REFLECT101 ,
/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
/// * borderValue: Value used in case of a constant border. By default, it is 0.
/// * stream: Stream for the asynchronous version.
///
/// The function transforms the source image using the specified map:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28xmap%28x%2Cy%29%2C%20ymap%28x%2Cy%29%29)
///
/// with the WARP_RELATIVE_MAP flag :
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28x%2Bmap%5Fx%28x%2Cy%29%2Cy%2Bmap%5Fy%28x%2Cy%29%29)
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
// cv::cuda::remap(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:92
// ("cv::cuda::remap", vec![(pred!(mut, ["src", "dst", "xmap", "ymap", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn remap_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, xmap: &impl ToInputArray, ymap: &impl ToInputArray, interpolation: i32) -> Result<()> {
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
/// The extra flag WARP_RELATIVE_MAP can be ORed to the interpolation method
/// (e.g. INTER_LINEAR | WARP_RELATIVE_MAP)
/// * borderMode: Pixel extrapolation method (see borderInterpolate ). BORDER_REFLECT101 ,
/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
/// * borderValue: Value used in case of a constant border. By default, it is 0.
/// * stream: Stream for the asynchronous version.
///
/// The function transforms the source image using the specified map:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28xmap%28x%2Cy%29%2C%20ymap%28x%2Cy%29%29)
///
/// with the WARP_RELATIVE_MAP flag :
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28x%2Bmap%5Fx%28x%2Cy%29%2Cy%2Bmap%5Fy%28x%2Cy%29%29)
///
/// Values of pixels with non-integer coordinates are computed using the bilinear interpolation.
/// ## See also
/// remap
///
/// ## C++ default parameters
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
/// * stream: Stream::Null()
// remap(InputArray, OutputArray, InputArray, InputArray, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:92
// ("cv::cuda::remap", vec![(pred!(mut, ["src", "dst", "xmap", "ymap", "interpolation", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn remap(src: &impl ToInputArray, dst: &mut impl ToOutputArray, xmap: &impl ToInputArray, ymap: &impl ToInputArray, interpolation: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(xmap);
	input_array_arg!(ymap);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), xmap.as_raw__InputArray(), ymap.as_raw__InputArray(), interpolation, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::resize(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:114
// ("cv::cuda::resize", vec![(pred!(mut, ["src", "dst", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
#[inline]
pub fn resize_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), &dsize, ocvrs_return.as_mut_ptr()) };
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
// resize(InputArray, OutputArray, Size, double, double, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:114
// ("cv::cuda::resize", vec![(pred!(mut, ["src", "dst", "dsize", "fx", "fy", "interpolation", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn resize(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dsize: core::Size, fx: f64, fy: f64, interpolation: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), &dsize, fx, fy, interpolation, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::rotate(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:231
// ("cv::cuda::rotate", vec![(pred!(mut, ["src", "dst", "dsize", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double"]), _)]),
#[inline]
pub fn rotate_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dsize: core::Size, angle: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), &dsize, angle, ocvrs_return.as_mut_ptr()) };
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
// rotate(InputArray, OutputArray, Size, double, double, double, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:231
// ("cv::cuda::rotate", vec![(pred!(mut, ["src", "dst", "dsize", "angle", "xShift", "yShift", "interpolation", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn rotate(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dsize: core::Size, angle: f64, x_shift: f64, y_shift: f64, interpolation: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double_double_double_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), &dsize, angle, x_shift, y_shift, interpolation, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpAffine(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:140
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size"]), _)]),
#[inline]
pub fn warp_affine_2_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::MatTrait, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), &dsize, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
/// * stream: Stream::Null()
// warpAffine(InputArray, OutputArray, Mat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:140
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_affine_2(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::MatTrait, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpAffine(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:135
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size"]), _)]),
#[inline]
pub fn warp_affine_1_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::UMatTrait, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), &dsize, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
/// * stream: Stream::Null()
// warpAffine(InputArray, OutputArray, UMat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:135
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_affine_1(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::UMatTrait, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpAffine(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:132
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
#[inline]
pub fn warp_affine_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, m: &impl ToInputArray, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), &dsize, ocvrs_return.as_mut_ptr()) };
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
// warpAffine(InputArray, OutputArray, InputArray, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:132
// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_affine(src: &impl ToInputArray, dst: &mut impl ToOutputArray, m: &impl ToInputArray, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpPerspective(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:190
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size"]), _)]),
#[inline]
pub fn warp_perspective_2_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::MatTrait, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), &dsize, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
/// * stream: Stream::Null()
// warpPerspective(InputArray, OutputArray, Mat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:190
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_perspective_2(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::MatTrait, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_Mat(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpPerspective(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:185
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size"]), _)]),
#[inline]
pub fn warp_perspective_1_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::UMatTrait, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), &dsize, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
/// * stream: Stream::Null()
// warpPerspective(InputArray, OutputArray, UMat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:185
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_perspective_1(src: &impl ToInputArray, dst: &mut impl ToOutputArray, mut m: impl core::UMatTrait, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw_mut_UMat(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
// cv::cuda::warpPerspective(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:182
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
#[inline]
pub fn warp_perspective_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, m: &impl ToInputArray, dsize: core::Size) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), &dsize, ocvrs_return.as_mut_ptr()) };
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
// warpPerspective(InputArray, OutputArray, InputArray, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:182
// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
#[inline]
pub fn warp_perspective(src: &impl ToInputArray, dst: &mut impl ToOutputArray, m: &impl ToInputArray, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray(), &dsize, flags, border_mode, &border_value, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}
