//! # Stereo Correspondence
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{StereoBMTrait, StereoBMTraitConst, StereoMatcherTrait, StereoMatcherTraitConst, StereoSGBMTrait, StereoSGBMTraitConst};
}

// STEREO_ZERO_DISPARITY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:19
pub const STEREO_ZERO_DISPARITY: i32 = 1024;
// PREFILTER_NORMALIZED_RESPONSE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:285
pub const StereoBM_PREFILTER_NORMALIZED_RESPONSE: i32 = 0;
// PREFILTER_XSOBEL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:286
pub const StereoBM_PREFILTER_XSOBEL: i32 = 1;
// DISP_SCALE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:245
pub const StereoMatcher_DISP_SCALE: i32 = 16;
// DISP_SHIFT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:244
pub const StereoMatcher_DISP_SHIFT: i32 = 4;
// MODE_HH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:353
pub const StereoSGBM_MODE_HH: i32 = 1;
// MODE_HH4 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:355
pub const StereoSGBM_MODE_HH4: i32 = 3;
// MODE_SGBM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:352
pub const StereoSGBM_MODE_SGBM: i32 = 0;
// MODE_SGBM_3WAY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:354
pub const StereoSGBM_MODE_SGBM_3WAY: i32 = 2;
/// Filters off small noise blobs (speckles) in the disparity map
///
/// ## Parameters
/// * img: The input 16-bit signed disparity image
/// * newVal: The disparity value used to paint-off the speckles
/// * maxSpeckleSize: The maximum speckle size to consider it a speckle. Larger blobs are not
/// affected by the algorithm
/// * maxDiff: Maximum difference between neighbor disparity pixels to put them into the same
/// blob. Note that since StereoBM, StereoSGBM and may be other algorithms return a fixed-point
/// disparity map, where disparity values are multiplied by 16, this scale factor should be taken into
/// account when specifying this parameter value.
/// * buf: The optional temporary buffer to avoid memory allocation within the function.
///
/// ## Note
/// This alternative version of [filter_speckles] function uses the following default values for its arguments:
/// * buf: noArray()
// cv::filterSpeckles(InputOutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff"], ["const cv::_InputOutputArray*", "double", "int", "double"]), _)]),
#[inline]
pub fn filter_speckles_def(img: &mut impl ToInputOutputArray, new_val: f64, max_speckle_size: i32, max_diff: f64) -> Result<()> {
	input_output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterSpeckles_const__InputOutputArrayR_double_int_double(img.as_raw__InputOutputArray(), new_val, max_speckle_size, max_diff, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Filters off small noise blobs (speckles) in the disparity map
///
/// ## Parameters
/// * img: The input 16-bit signed disparity image
/// * newVal: The disparity value used to paint-off the speckles
/// * maxSpeckleSize: The maximum speckle size to consider it a speckle. Larger blobs are not
/// affected by the algorithm
/// * maxDiff: Maximum difference between neighbor disparity pixels to put them into the same
/// blob. Note that since StereoBM, StereoSGBM and may be other algorithms return a fixed-point
/// disparity map, where disparity values are multiplied by 16, this scale factor should be taken into
/// account when specifying this parameter value.
/// * buf: The optional temporary buffer to avoid memory allocation within the function.
///
/// ## C++ default parameters
/// * buf: noArray()
// filterSpeckles(InputOutputArray, double, int, double, InputOutputArray)(InputOutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff", "buf"], ["const cv::_InputOutputArray*", "double", "int", "double", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn filter_speckles(img: &mut impl ToInputOutputArray, new_val: f64, max_speckle_size: i32, max_diff: f64, buf: &mut impl ToInputOutputArray) -> Result<()> {
	input_output_array_arg!(img);
	input_output_array_arg!(buf);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(img.as_raw__InputOutputArray(), new_val, max_speckle_size, max_diff, buf.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo rectification for fisheye camera model
///
/// ## Parameters
/// * K1: First camera intrinsic matrix.
/// * D1: First camera distortion parameters.
/// * K2: Second camera intrinsic matrix.
/// * D2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second
/// cameras.
/// * tvec: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see reprojectImageTo3D ).
/// * flags: Operation flags that may be zero or [cv::CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * fov_scale: Divisor for new focal length.
///
/// ## Note
/// This alternative version of [stereo_rectify_1] function uses the following default values for its arguments:
/// * new_image_size: Size()
/// * balance: 0.0
/// * fov_scale: 1.0
// cv::fisheye::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn stereo_rectify_1_def(k1: &impl ToInputArray, d1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, tvec: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(k1);
	input_array_arg!(d1);
	input_array_arg!(k2);
	input_array_arg!(d2);
	input_array_arg!(r);
	input_array_arg!(tvec);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(k1.as_raw__InputArray(), d1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), tvec.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo rectification for fisheye camera model
///
/// ## Parameters
/// * K1: First camera intrinsic matrix.
/// * D1: First camera distortion parameters.
/// * K2: Second camera intrinsic matrix.
/// * D2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second
/// cameras.
/// * tvec: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see reprojectImageTo3D ).
/// * flags: Operation flags that may be zero or [cv::CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * fov_scale: Divisor for new focal length.
///
/// ## C++ default parameters
/// * new_image_size: Size()
/// * balance: 0.0
/// * fov_scale: 1.0
// stereoRectify(InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, const Size &, double, double)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags", "newImageSize", "balance", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::Size*", "double", "double"]), _)]),
#[inline]
pub fn stereo_rectify_1(k1: &impl ToInputArray, d1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, tvec: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32, new_image_size: core::Size, balance: f64, fov_scale: f64) -> Result<()> {
	input_array_arg!(k1);
	input_array_arg!(d1);
	input_array_arg!(k2);
	input_array_arg!(d2);
	input_array_arg!(r);
	input_array_arg!(tvec);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(k1.as_raw__InputArray(), d1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), tvec.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, &new_image_size, balance, fov_scale, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// computes valid disparity ROI from the valid ROIs of the rectified images (that are returned by #stereoRectify)
// getValidDisparityROI(Rect, Rect, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:434
// ("cv::getValidDisparityROI", vec![(pred!(mut, ["roi1", "roi2", "minDisparity", "numberOfDisparities", "blockSize"], ["cv::Rect", "cv::Rect", "int", "int", "int"]), _)]),
#[inline]
pub fn get_valid_disparity_roi(roi1: core::Rect, roi2: core::Rect, min_disparity: i32, number_of_disparities: i32, block_size: i32) -> Result<core::Rect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getValidDisparityROI_Rect_Rect_int_int_int(&roi1, &roi2, min_disparity, number_of_disparities, block_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// rectify3Collinear(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArrayOfArrays, InputArrayOfArrays, Size, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double, Size, Rect *, Rect *, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:188
// ("cv::rectify3Collinear", vec![(pred!(mut, ["_cameraMatrix1", "_distCoeffs1", "_cameraMatrix2", "_distCoeffs2", "_cameraMatrix3", "_distCoeffs3", "_imgpt1", "_imgpt3", "imageSize", "_Rmat12", "_Tmat12", "_Rmat13", "_Tmat13", "_Rmat1", "_Rmat2", "_Rmat3", "_Pmat1", "_Pmat2", "_Pmat3", "_Qmat", "alpha", "newImgSize", "roi1", "roi2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Size", "cv::Rect*", "cv::Rect*", "int"]), _)]),
#[inline]
pub fn rectify3_collinear(_camera_matrix1: &impl ToInputArray, _dist_coeffs1: &impl ToInputArray, _camera_matrix2: &impl ToInputArray, _dist_coeffs2: &impl ToInputArray, _camera_matrix3: &impl ToInputArray, _dist_coeffs3: &impl ToInputArray, _imgpt1: &impl ToInputArray, _imgpt3: &impl ToInputArray, image_size: core::Size, _rmat12: &impl ToInputArray, _tmat12: &impl ToInputArray, _rmat13: &impl ToInputArray, _tmat13: &impl ToInputArray, _rmat1: &mut impl ToOutputArray, _rmat2: &mut impl ToOutputArray, _rmat3: &mut impl ToOutputArray, _pmat1: &mut impl ToOutputArray, _pmat2: &mut impl ToOutputArray, _pmat3: &mut impl ToOutputArray, _qmat: &mut impl ToOutputArray, alpha: f64, new_img_size: core::Size, roi1: &mut core::Rect, roi2: &mut core::Rect, flags: i32) -> Result<f32> {
	input_array_arg!(_camera_matrix1);
	input_array_arg!(_dist_coeffs1);
	input_array_arg!(_camera_matrix2);
	input_array_arg!(_dist_coeffs2);
	input_array_arg!(_camera_matrix3);
	input_array_arg!(_dist_coeffs3);
	input_array_arg!(_imgpt1);
	input_array_arg!(_imgpt3);
	input_array_arg!(_rmat12);
	input_array_arg!(_tmat12);
	input_array_arg!(_rmat13);
	input_array_arg!(_tmat13);
	output_array_arg!(_rmat1);
	output_array_arg!(_rmat2);
	output_array_arg!(_rmat3);
	output_array_arg!(_pmat1);
	output_array_arg!(_pmat2);
	output_array_arg!(_pmat3);
	output_array_arg!(_qmat);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(_camera_matrix1.as_raw__InputArray(), _dist_coeffs1.as_raw__InputArray(), _camera_matrix2.as_raw__InputArray(), _dist_coeffs2.as_raw__InputArray(), _camera_matrix3.as_raw__InputArray(), _dist_coeffs3.as_raw__InputArray(), _imgpt1.as_raw__InputArray(), _imgpt3.as_raw__InputArray(), &image_size, _rmat12.as_raw__InputArray(), _tmat12.as_raw__InputArray(), _rmat13.as_raw__InputArray(), _tmat13.as_raw__InputArray(), _rmat1.as_raw__OutputArray(), _rmat2.as_raw__OutputArray(), _rmat3.as_raw__OutputArray(), _pmat1.as_raw__OutputArray(), _pmat2.as_raw__OutputArray(), _pmat3.as_raw__OutputArray(), _qmat.as_raw__OutputArray(), alpha, &new_img_size, roi1, roi2, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reprojects a disparity image to 3D space.
///
/// ## Parameters
/// * disparity: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. The values of 8-bit / 16-bit signed formats are assumed to have no
/// fractional bits. If the disparity is 16-bit signed format, as computed by [StereoBM] or
/// [StereoSGBM] and maybe other algorithms, it should be divided by 16 (and scaled to float) before
/// being used here.
/// * _3dImage: Output 3-channel floating-point image of the same size as disparity. Each element of
/// _3dImage(x,y) contains 3D coordinates of the point (x,y) computed from the disparity map. If one
/// uses Q obtained by [stereoRectify], then the returned points are represented in the first
/// camera's rectified coordinate system.
/// * Q: ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) perspective transformation matrix that can be obtained with
/// [stereoRectify].
/// * handleMissingValues: Indicates, whether the function should handle missing values (i.e.
/// points where the disparity was not computed). If handleMissingValues=true, then pixels with the
/// minimal disparity that corresponds to the outliers (see StereoMatcher::compute ) are transformed
/// to 3D points with a very large Z value (currently set to 10000).
/// * ddepth: The optional output array depth. If it is -1, the output image will have CV_32F
/// depth. ddepth can also be set to CV_16S, CV_32S or CV_32F.
///
/// The function transforms a single-channel disparity map to a 3-channel image representing a 3D
/// surface. That is, for each pixel (x,y) and the corresponding disparity d=disparity(x,y) , it
/// computes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%3D%20Q%20%5Cbegin%7Bbmatrix%7D%0Ax%20%5C%5C%0Ay%20%5C%5C%0A%5Ctexttt%7Bdisparity%7D%20%28x%2Cy%29%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
/// ## See also
/// To reproject a sparse set of points {(x,y,d),...} to 3D space, use perspectiveTransform.
///
/// ## Note
/// This alternative version of [reproject_image_to_3d] function uses the following default values for its arguments:
/// * handle_missing_values: false
/// * ddepth: -1
// cv::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn reproject_image_to_3d_def(disparity: &impl ToInputArray, _3d_image: &mut impl ToOutputArray, q: &impl ToInputArray) -> Result<()> {
	input_array_arg!(disparity);
	output_array_arg!(_3d_image);
	input_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(disparity.as_raw__InputArray(), _3d_image.as_raw__OutputArray(), q.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reprojects a disparity image to 3D space.
///
/// ## Parameters
/// * disparity: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. The values of 8-bit / 16-bit signed formats are assumed to have no
/// fractional bits. If the disparity is 16-bit signed format, as computed by [StereoBM] or
/// [StereoSGBM] and maybe other algorithms, it should be divided by 16 (and scaled to float) before
/// being used here.
/// * _3dImage: Output 3-channel floating-point image of the same size as disparity. Each element of
/// _3dImage(x,y) contains 3D coordinates of the point (x,y) computed from the disparity map. If one
/// uses Q obtained by [stereoRectify], then the returned points are represented in the first
/// camera's rectified coordinate system.
/// * Q: ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) perspective transformation matrix that can be obtained with
/// [stereoRectify].
/// * handleMissingValues: Indicates, whether the function should handle missing values (i.e.
/// points where the disparity was not computed). If handleMissingValues=true, then pixels with the
/// minimal disparity that corresponds to the outliers (see StereoMatcher::compute ) are transformed
/// to 3D points with a very large Z value (currently set to 10000).
/// * ddepth: The optional output array depth. If it is -1, the output image will have CV_32F
/// depth. ddepth can also be set to CV_16S, CV_32S or CV_32F.
///
/// The function transforms a single-channel disparity map to a 3-channel image representing a 3D
/// surface. That is, for each pixel (x,y) and the corresponding disparity d=disparity(x,y) , it
/// computes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%3D%20Q%20%5Cbegin%7Bbmatrix%7D%0Ax%20%5C%5C%0Ay%20%5C%5C%0A%5Ctexttt%7Bdisparity%7D%20%28x%2Cy%29%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
/// ## See also
/// To reproject a sparse set of points {(x,y,d),...} to 3D space, use perspectiveTransform.
///
/// ## C++ default parameters
/// * handle_missing_values: false
/// * ddepth: -1
// reprojectImageTo3D(InputArray, OutputArray, InputArray, bool, int)(InputArray, OutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q", "handleMissingValues", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "int"]), _)]),
#[inline]
pub fn reproject_image_to_3d(disparity: &impl ToInputArray, _3d_image: &mut impl ToOutputArray, q: &impl ToInputArray, handle_missing_values: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(disparity);
	output_array_arg!(_3d_image);
	input_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(disparity.as_raw__InputArray(), _3d_image.as_raw__OutputArray(), q.as_raw__InputArray(), handle_missing_values, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes a rectification transform for an uncalibrated stereo camera.
///
/// ## Parameters
/// * points1: Array of feature points in the first image.
/// * points2: The corresponding points in the second image. The same formats as in
/// [find_fundamental_mat] are supported.
/// * F: Input fundamental matrix. It can be computed from the same set of point pairs using
/// [find_fundamental_mat] .
/// * imgSize: Size of the image.
/// * H1: Output rectification homography matrix for the first image.
/// * H2: Output rectification homography matrix for the second image.
/// * threshold: Optional threshold used to filter out the outliers. If the parameter is greater
/// than zero, all the point pairs that do not comply with the epipolar geometry (that is, the points
/// for which ![inline formula](https://latex.codecogs.com/png.latex?%7C%5Ctexttt%7Bpoints2%5Bi%5D%7D%5ET%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpoints1%5Bi%5D%7D%7C%3E%5Ctexttt%7Bthreshold%7D) )
/// are rejected prior to computing the homographies. Otherwise, all the points are considered inliers.
///
/// The function computes the rectification transformations without knowing intrinsic parameters of the
/// cameras and their relative position in the space, which explains the suffix "uncalibrated". Another
/// related difference from [stereo_rectify] is that the function outputs not the rectification
/// transformations in the object (3D) space, but the planar perspective transformations encoded by the
/// homography matrices H1 and H2 . The function implements the algorithm [Hartley99](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Hartley99) .
///
///
/// Note:
///    While the algorithm does not need to know the intrinsic parameters of the cameras, it heavily
///    depends on the epipolar geometry. Therefore, if the camera lenses have a significant distortion,
///    it would be better to correct it before computing the fundamental matrix and calling this
///    function. For example, distortion coefficients can be estimated for each head of stereo camera
///    separately by using [calibrate_camera] . Then, the images can be corrected using [undistort] , or
///    just the point coordinates can be corrected with [undistort_points] .
///
/// ## Note
/// This alternative version of [stereo_rectify_uncalibrated] function uses the following default values for its arguments:
/// * threshold: 5
// cv::stereoRectifyUncalibrated(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_rectify_uncalibrated_def(points1: &impl ToInputArray, points2: &impl ToInputArray, f: &impl ToInputArray, img_size: core::Size, h1: &mut impl ToOutputArray, h2: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(f);
	output_array_arg!(h1);
	output_array_arg!(h2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), f.as_raw__InputArray(), &img_size, h1.as_raw__OutputArray(), h2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes a rectification transform for an uncalibrated stereo camera.
///
/// ## Parameters
/// * points1: Array of feature points in the first image.
/// * points2: The corresponding points in the second image. The same formats as in
/// [find_fundamental_mat] are supported.
/// * F: Input fundamental matrix. It can be computed from the same set of point pairs using
/// [find_fundamental_mat] .
/// * imgSize: Size of the image.
/// * H1: Output rectification homography matrix for the first image.
/// * H2: Output rectification homography matrix for the second image.
/// * threshold: Optional threshold used to filter out the outliers. If the parameter is greater
/// than zero, all the point pairs that do not comply with the epipolar geometry (that is, the points
/// for which ![inline formula](https://latex.codecogs.com/png.latex?%7C%5Ctexttt%7Bpoints2%5Bi%5D%7D%5ET%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpoints1%5Bi%5D%7D%7C%3E%5Ctexttt%7Bthreshold%7D) )
/// are rejected prior to computing the homographies. Otherwise, all the points are considered inliers.
///
/// The function computes the rectification transformations without knowing intrinsic parameters of the
/// cameras and their relative position in the space, which explains the suffix "uncalibrated". Another
/// related difference from [stereo_rectify] is that the function outputs not the rectification
/// transformations in the object (3D) space, but the planar perspective transformations encoded by the
/// homography matrices H1 and H2 . The function implements the algorithm [Hartley99](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Hartley99) .
///
///
/// Note:
///    While the algorithm does not need to know the intrinsic parameters of the cameras, it heavily
///    depends on the epipolar geometry. Therefore, if the camera lenses have a significant distortion,
///    it would be better to correct it before computing the fundamental matrix and calling this
///    function. For example, distortion coefficients can be estimated for each head of stereo camera
///    separately by using [calibrate_camera] . Then, the images can be corrected using [undistort] , or
///    just the point coordinates can be corrected with [undistort_points] .
///
/// ## C++ default parameters
/// * threshold: 5
// stereoRectifyUncalibrated(InputArray, InputArray, InputArray, Size, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2", "threshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn stereo_rectify_uncalibrated(points1: &impl ToInputArray, points2: &impl ToInputArray, f: &impl ToInputArray, img_size: core::Size, h1: &mut impl ToOutputArray, h2: &mut impl ToOutputArray, threshold: f64) -> Result<bool> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(f);
	output_array_arg!(h1);
	output_array_arg!(h2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(points1.as_raw__InputArray(), points2.as_raw__InputArray(), f.as_raw__InputArray(), &img_size, h1.as_raw__OutputArray(), h2.as_raw__OutputArray(), threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes rectification transforms for each head of a calibrated stereo camera.
///
/// ## Parameters
/// * cameraMatrix1: First camera intrinsic matrix.
/// * distCoeffs1: First camera distortion parameters.
/// * cameraMatrix2: Second camera intrinsic matrix.
/// * distCoeffs2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * T: Translation vector from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera. This matrix
/// brings points given in the unrectified first camera's coordinate system to points in the rectified
/// first camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified first camera's coordinate system to the rectified first camera's coordinate system.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera. This matrix
/// brings points given in the unrectified second camera's coordinate system to points in the rectified
/// second camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified second camera's coordinate system to the rectified second camera's coordinate system.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified first camera's image.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified second camera's image.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reprojectImageTo3D]).
/// * flags: Operation flags that may be zero or [STEREO_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * alpha: Free scaling parameter. If it is -1 or absent, the function performs the default
/// scaling. Otherwise, the parameter should be between 0 and 1. alpha=0 means that the rectified
/// images are zoomed and shifted so that only valid pixels are visible (no black areas after
/// rectification). alpha=1 means that the rectified image is decimated and shifted so that all the
/// pixels from the original images from the cameras are retained in the rectified images (no source
/// image pixels are lost). Any intermediate value yields an intermediate result between
/// those two extreme cases.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to a larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * validPixROI1: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// * validPixROI2: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
///
/// The function computes the rotation matrices for each camera that (virtually) make both camera image
/// planes the same plane. Consequently, this makes all the epipolar lines parallel and thus simplifies
/// the dense stereo correspondence problem. The function takes the matrices computed by [stereo_calibrate]
/// as input. As output, it provides two rotation matrices and also two projection matrices in the new
/// coordinates. The function distinguishes the following two cases:
///
/// *   **Horizontal stereo**: the first and the second camera views are shifted relative to each other
///    mainly along the x-axis (with possible small vertical shift). In the rectified images, the
///    corresponding epipolar lines in the left and right cameras are horizontal and have the same
///    y-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F2%20%26%20T%5Fx%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fx%7D%20%26%20%5Cfrac%7Bcx%5F1%20%2D%20cx%5F2%7D%7BT%5Fx%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fx) is a horizontal shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cx%5F1%3Dcx%5F2) if
///    [STEREO_ZERO_DISPARITY] is set.
///
/// *   **Vertical stereo**: the first and the second camera views are shifted relative to each other
///    mainly in the vertical direction (and probably a bit in the horizontal direction too). The epipolar
///    lines in the rectified images are vertical and have the same x-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F2%20%26%20T%5Fy%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fy%7D%20%26%20%5Cfrac%7Bcy%5F1%20%2D%20cy%5F2%7D%7BT%5Fy%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fy) is a vertical shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cy%5F1%3Dcy%5F2) if
///    [STEREO_ZERO_DISPARITY] is set.
///
/// As you can see, the first three columns of P1 and P2 will effectively be the new "rectified" camera
/// matrices. The matrices, together with R1 and R2 , can then be passed to [init_undistort_rectify_map] to
/// initialize the rectification map for each camera.
///
/// See below the screenshot from the stereo_calib.cpp sample. Some red horizontal lines pass through
/// the corresponding image regions. This means that the images are well rectified, which is what most
/// stereo correspondence algorithms rely on. The green rectangles are roi1 and roi2 . You see that
/// their interiors are all valid pixels.
///
/// ![image](https://docs.opencv.org/5.0.0/stereo_undistort.jpg)
///
/// ## Note
/// This alternative version of [stereo_rectify] function uses the following default values for its arguments:
/// * flags: STEREO_ZERO_DISPARITY
/// * alpha: -1
/// * new_image_size: Size()
/// * valid_pix_roi1: 0
/// * valid_pix_roi2: 0
// cv::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_rectify_def(camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, t: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes rectification transforms for each head of a calibrated stereo camera.
///
/// ## Parameters
/// * cameraMatrix1: First camera intrinsic matrix.
/// * distCoeffs1: First camera distortion parameters.
/// * cameraMatrix2: Second camera intrinsic matrix.
/// * distCoeffs2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * T: Translation vector from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera. This matrix
/// brings points given in the unrectified first camera's coordinate system to points in the rectified
/// first camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified first camera's coordinate system to the rectified first camera's coordinate system.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera. This matrix
/// brings points given in the unrectified second camera's coordinate system to points in the rectified
/// second camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified second camera's coordinate system to the rectified second camera's coordinate system.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified first camera's image.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified second camera's image.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reprojectImageTo3D]).
/// * flags: Operation flags that may be zero or [STEREO_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * alpha: Free scaling parameter. If it is -1 or absent, the function performs the default
/// scaling. Otherwise, the parameter should be between 0 and 1. alpha=0 means that the rectified
/// images are zoomed and shifted so that only valid pixels are visible (no black areas after
/// rectification). alpha=1 means that the rectified image is decimated and shifted so that all the
/// pixels from the original images from the cameras are retained in the rectified images (no source
/// image pixels are lost). Any intermediate value yields an intermediate result between
/// those two extreme cases.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to a larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * validPixROI1: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// * validPixROI2: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
///
/// The function computes the rotation matrices for each camera that (virtually) make both camera image
/// planes the same plane. Consequently, this makes all the epipolar lines parallel and thus simplifies
/// the dense stereo correspondence problem. The function takes the matrices computed by [stereo_calibrate]
/// as input. As output, it provides two rotation matrices and also two projection matrices in the new
/// coordinates. The function distinguishes the following two cases:
///
/// *   **Horizontal stereo**: the first and the second camera views are shifted relative to each other
///    mainly along the x-axis (with possible small vertical shift). In the rectified images, the
///    corresponding epipolar lines in the left and right cameras are horizontal and have the same
///    y-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F2%20%26%20T%5Fx%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fx%7D%20%26%20%5Cfrac%7Bcx%5F1%20%2D%20cx%5F2%7D%7BT%5Fx%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fx) is a horizontal shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cx%5F1%3Dcx%5F2) if
///    [STEREO_ZERO_DISPARITY] is set.
///
/// *   **Vertical stereo**: the first and the second camera views are shifted relative to each other
///    mainly in the vertical direction (and probably a bit in the horizontal direction too). The epipolar
///    lines in the rectified images are vertical and have the same x-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F2%20%26%20T%5Fy%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fy%7D%20%26%20%5Cfrac%7Bcy%5F1%20%2D%20cy%5F2%7D%7BT%5Fy%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fy) is a vertical shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cy%5F1%3Dcy%5F2) if
///    [STEREO_ZERO_DISPARITY] is set.
///
/// As you can see, the first three columns of P1 and P2 will effectively be the new "rectified" camera
/// matrices. The matrices, together with R1 and R2 , can then be passed to [init_undistort_rectify_map] to
/// initialize the rectification map for each camera.
///
/// See below the screenshot from the stereo_calib.cpp sample. Some red horizontal lines pass through
/// the corresponding image regions. This means that the images are well rectified, which is what most
/// stereo correspondence algorithms rely on. The green rectangles are roi1 and roi2 . You see that
/// their interiors are all valid pixels.
///
/// ![image](https://docs.opencv.org/5.0.0/stereo_undistort.jpg)
///
/// ## C++ default parameters
/// * flags: STEREO_ZERO_DISPARITY
/// * alpha: -1
/// * new_image_size: Size()
/// * valid_pix_roi1: 0
/// * valid_pix_roi2: 0
// stereoRectify(InputArray, InputArray, InputArray, InputArray, Size, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, double, Size, Rect *, Rect *)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q", "flags", "alpha", "newImageSize", "validPixROI1", "validPixROI2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "cv::Size", "cv::Rect*", "cv::Rect*"]), _)]),
#[inline]
pub fn stereo_rectify(camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, t: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32, alpha: f64, new_image_size: core::Size, valid_pix_roi1: &mut core::Rect, valid_pix_roi2: &mut core::Rect) -> Result<()> {
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, alpha, &new_image_size, valid_pix_roi1, valid_pix_roi2, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// validates disparity using the left-right check. The matrix "cost" should be computed by the stereo correspondence algorithm
///
/// ## Note
/// This alternative version of [validate_disparity] function uses the following default values for its arguments:
/// * disp12_max_disp: 1
// cv::validateDisparity(InputOutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
#[inline]
pub fn validate_disparity_def(disparity: &mut impl ToInputOutputArray, cost: &impl ToInputArray, min_disparity: i32, number_of_disparities: i32) -> Result<()> {
	input_output_array_arg!(disparity);
	input_array_arg!(cost);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int(disparity.as_raw__InputOutputArray(), cost.as_raw__InputArray(), min_disparity, number_of_disparities, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// validates disparity using the left-right check. The matrix "cost" should be computed by the stereo correspondence algorithm
///
/// ## C++ default parameters
/// * disp12_max_disp: 1
// validateDisparity(InputOutputArray, InputArray, int, int, int)(InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities", "disp12MaxDisp"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
#[inline]
pub fn validate_disparity(disparity: &mut impl ToInputOutputArray, cost: &impl ToInputArray, min_disparity: i32, number_of_disparities: i32, disp12_max_disp: i32) -> Result<()> {
	input_output_array_arg!(disparity);
	input_array_arg!(cost);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(disparity.as_raw__InputOutputArray(), cost.as_raw__InputArray(), min_disparity, number_of_disparities, disp12_max_disp, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::stereo::StereoBM]
// StereoBM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:282
pub trait StereoBMTraitConst: crate::stereo::StereoMatcherTraitConst {
	fn as_raw_StereoBM(&self) -> *const c_void;

	// getPreFilterType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:289
	// ("cv::StereoBM::getPreFilterType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterType_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPreFilterSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:292
	// ("cv::StereoBM::getPreFilterSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:295
	// ("cv::StereoBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterCap_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTextureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:298
	// ("cv::StereoBM::getTextureThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_texture_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getTextureThreshold_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:301
	// ("cv::StereoBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getUniquenessRatio_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSmallerBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:304
	// ("cv::StereoBM::getSmallerBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_smaller_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getSmallerBlockSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getROI1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:307
	// ("cv::StereoBM::getROI1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_roi1(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI1_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getROI2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:310
	// ("cv::StereoBM::getROI2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_roi2(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI2_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stereo::StereoBM]
pub trait StereoBMTrait: crate::stereo::StereoBMTraitConst + crate::stereo::StereoMatcherTrait {
	fn as_raw_mut_StereoBM(&mut self) -> *mut c_void;

	// setPreFilterType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:290
	// ("cv::StereoBM::setPreFilterType", vec![(pred!(mut, ["preFilterType"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_type(&mut self, pre_filter_type: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterType_int(self.as_raw_mut_StereoBM(), pre_filter_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPreFilterSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:293
	// ("cv::StereoBM::setPreFilterSize", vec![(pred!(mut, ["preFilterSize"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_size(&mut self, pre_filter_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterSize_int(self.as_raw_mut_StereoBM(), pre_filter_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:296
	// ("cv::StereoBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterCap_int(self.as_raw_mut_StereoBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTextureThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:299
	// ("cv::StereoBM::setTextureThreshold", vec![(pred!(mut, ["textureThreshold"], ["int"]), _)]),
	#[inline]
	fn set_texture_threshold(&mut self, texture_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setTextureThreshold_int(self.as_raw_mut_StereoBM(), texture_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:302
	// ("cv::StereoBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setUniquenessRatio_int(self.as_raw_mut_StereoBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSmallerBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:305
	// ("cv::StereoBM::setSmallerBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_smaller_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setSmallerBlockSize_int(self.as_raw_mut_StereoBM(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setROI1(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:308
	// ("cv::StereoBM::setROI1", vec![(pred!(mut, ["roi1"], ["cv::Rect"]), _)]),
	#[inline]
	fn set_roi1(&mut self, roi1: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI1_Rect(self.as_raw_mut_StereoBM(), &roi1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setROI2(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:311
	// ("cv::StereoBM::setROI2", vec![(pred!(mut, ["roi2"], ["cv::Rect"]), _)]),
	#[inline]
	fn set_roi2(&mut self, roi2: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI2_Rect(self.as_raw_mut_StereoBM(), &roi2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for computing stereo correspondence using the block matching algorithm, introduced and
/// contributed to OpenCV by K. Konolige.
// StereoBM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:282
pub struct StereoBM {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoBM }

impl Drop for StereoBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoBM_delete(self.as_raw_mut_StereoBM()) };
	}
}

unsafe impl Send for StereoBM {}

impl core::AlgorithmTraitConst for StereoBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::stereo::StereoMatcherTraitConst for StereoBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stereo::StereoMatcherTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, crate::stereo::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::stereo::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl crate::stereo::StereoBMTraitConst for StereoBM {
	#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::stereo::StereoBMTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, crate::stereo::StereoBMTraitConst, as_raw_StereoBM, crate::stereo::StereoBMTrait, as_raw_mut_StereoBM }

impl StereoBM {
	/// Creates StereoBM object
	///
	/// ## Parameters
	/// * numDisparities: the disparity search range. For each pixel algorithm will find the best
	/// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
	/// shifted by changing the minimum disparity.
	/// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
	/// (as the block is centered at the current pixel). Larger block size implies smoother, though less
	/// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
	/// chance for algorithm to find a wrong correspondence.
	///
	/// The function create StereoBM object. You can then call StereoBM::compute() to compute disparity for
	/// a specific stereo pair.
	///
	/// ## C++ default parameters
	/// * num_disparities: 0
	/// * block_size: 21
	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
	// ("cv::StereoBM::create", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
	#[inline]
	pub fn create(num_disparities: i32, block_size: i32) -> Result<core::Ptr<crate::stereo::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create_int_int(num_disparities, block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stereo::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates StereoBM object
	///
	/// ## Parameters
	/// * numDisparities: the disparity search range. For each pixel algorithm will find the best
	/// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
	/// shifted by changing the minimum disparity.
	/// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
	/// (as the block is centered at the current pixel). Larger block size implies smoother, though less
	/// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
	/// chance for algorithm to find a wrong correspondence.
	///
	/// The function create StereoBM object. You can then call StereoBM::compute() to compute disparity for
	/// a specific stereo pair.
	///
	/// ## Note
	/// This alternative version of [StereoBM::create] function uses the following default values for its arguments:
	/// * num_disparities: 0
	/// * block_size: 21
	// cv::StereoBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
	// ("cv::StereoBM::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::stereo::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stereo::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StereoBM, core::Algorithm, cv_StereoBM_to_Algorithm }

boxed_cast_base! { StereoBM, crate::stereo::StereoMatcher, cv_StereoBM_to_StereoMatcher }

impl std::fmt::Debug for StereoBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoBM")
			.finish()
	}
}

/// Constant methods for [crate::stereo::StereoMatcher]
// StereoMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:241
pub trait StereoMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_StereoMatcher(&self) -> *const c_void;

	// getMinDisparity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:259
	// ("cv::StereoMatcher::getMinDisparity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_disparity(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getMinDisparity_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:262
	// ("cv::StereoMatcher::getNumDisparities", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_disparities(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getNumDisparities_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:265
	// ("cv::StereoMatcher::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getBlockSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSpeckleWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:268
	// ("cv::StereoMatcher::getSpeckleWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_speckle_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleWindowSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSpeckleRange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:271
	// ("cv::StereoMatcher::getSpeckleRange", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_speckle_range(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleRange_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDisp12MaxDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:274
	// ("cv::StereoMatcher::getDisp12MaxDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_disp12_max_diff(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getDisp12MaxDiff_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stereo::StereoMatcher]
pub trait StereoMatcherTrait: core::AlgorithmTrait + crate::stereo::StereoMatcherTraitConst {
	fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void;

	/// Computes disparity map for the specified stereo pair
	///
	/// ## Parameters
	/// * left: Left 8-bit single-channel image.
	/// * right: Right image of the same size and the same type as the left one.
	/// * disparity: Output disparity map. It has the same size as the input images. Some algorithms,
	/// like StereoBM or StereoSGBM compute 16-bit fixed-point disparity map (where each disparity value
	/// has 4 fractional bits), whereas other algorithms output 32-bit floating-point disparity map.
	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:256
	// ("cv::StereoMatcher::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute(&mut self, left: &impl ToInputArray, right: &impl ToInputArray, disparity: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(left);
		input_array_arg!(right);
		output_array_arg!(disparity);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StereoMatcher(), left.as_raw__InputArray(), right.as_raw__InputArray(), disparity.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDisparity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:260
	// ("cv::StereoMatcher::setMinDisparity", vec![(pred!(mut, ["minDisparity"], ["int"]), _)]),
	#[inline]
	fn set_min_disparity(&mut self, min_disparity: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setMinDisparity_int(self.as_raw_mut_StereoMatcher(), min_disparity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:263
	// ("cv::StereoMatcher::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
	#[inline]
	fn set_num_disparities(&mut self, num_disparities: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setNumDisparities_int(self.as_raw_mut_StereoMatcher(), num_disparities, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:266
	// ("cv::StereoMatcher::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setBlockSize_int(self.as_raw_mut_StereoMatcher(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSpeckleWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:269
	// ("cv::StereoMatcher::setSpeckleWindowSize", vec![(pred!(mut, ["speckleWindowSize"], ["int"]), _)]),
	#[inline]
	fn set_speckle_window_size(&mut self, speckle_window_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleWindowSize_int(self.as_raw_mut_StereoMatcher(), speckle_window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSpeckleRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:272
	// ("cv::StereoMatcher::setSpeckleRange", vec![(pred!(mut, ["speckleRange"], ["int"]), _)]),
	#[inline]
	fn set_speckle_range(&mut self, speckle_range: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleRange_int(self.as_raw_mut_StereoMatcher(), speckle_range, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDisp12MaxDiff(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:275
	// ("cv::StereoMatcher::setDisp12MaxDiff", vec![(pred!(mut, ["disp12MaxDiff"], ["int"]), _)]),
	#[inline]
	fn set_disp12_max_diff(&mut self, disp12_max_diff: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setDisp12MaxDiff_int(self.as_raw_mut_StereoMatcher(), disp12_max_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// The base class for stereo correspondence algorithms.
// StereoMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:241
pub struct StereoMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoMatcher }

impl Drop for StereoMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoMatcher_delete(self.as_raw_mut_StereoMatcher()) };
	}
}

unsafe impl Send for StereoMatcher {}

impl core::AlgorithmTraitConst for StereoMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::stereo::StereoMatcherTraitConst for StereoMatcher {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stereo::StereoMatcherTrait for StereoMatcher {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoMatcher, crate::stereo::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::stereo::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl StereoMatcher {
}

boxed_cast_descendant! { StereoMatcher, crate::stereo::StereoBM, cv_StereoMatcher_to_StereoBM }

boxed_cast_descendant! { StereoMatcher, crate::stereo::StereoSGBM, cv_StereoMatcher_to_StereoSGBM }

boxed_cast_base! { StereoMatcher, core::Algorithm, cv_StereoMatcher_to_Algorithm }

impl std::fmt::Debug for StereoMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoMatcher")
			.finish()
	}
}

/// Constant methods for [crate::stereo::StereoSGBM]
// StereoSGBM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:347
pub trait StereoSGBMTraitConst: crate::stereo::StereoMatcherTraitConst {
	fn as_raw_StereoSGBM(&self) -> *const c_void;

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:358
	// ("cv::StereoSGBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getPreFilterCap_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:361
	// ("cv::StereoSGBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getUniquenessRatio_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getP1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:364
	// ("cv::StereoSGBM::getP1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_p1(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP1_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getP2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:367
	// ("cv::StereoSGBM::getP2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_p2(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP2_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:370
	// ("cv::StereoSGBM::getMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getMode_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stereo::StereoSGBM]
pub trait StereoSGBMTrait: crate::stereo::StereoMatcherTrait + crate::stereo::StereoSGBMTraitConst {
	fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void;

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:359
	// ("cv::StereoSGBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setPreFilterCap_int(self.as_raw_mut_StereoSGBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:362
	// ("cv::StereoSGBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setUniquenessRatio_int(self.as_raw_mut_StereoSGBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setP1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:365
	// ("cv::StereoSGBM::setP1", vec![(pred!(mut, ["P1"], ["int"]), _)]),
	#[inline]
	fn set_p1(&mut self, p1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP1_int(self.as_raw_mut_StereoSGBM(), p1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setP2(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:368
	// ("cv::StereoSGBM::setP2", vec![(pred!(mut, ["P2"], ["int"]), _)]),
	#[inline]
	fn set_p2(&mut self, p2: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP2_int(self.as_raw_mut_StereoSGBM(), p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:371
	// ("cv::StereoSGBM::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	#[inline]
	fn set_mode(&mut self, mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setMode_int(self.as_raw_mut_StereoSGBM(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// The class implements the modified H. Hirschmuller algorithm [HH08](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HH08) that differs from the original
/// one as follows:
///
/// *   By default, the algorithm is single-pass, which means that you consider only 5 directions
/// instead of 8. Set mode=StereoSGBM::MODE_HH in createStereoSGBM to run the full variant of the
/// algorithm but beware that it may consume a lot of memory.
/// *   The algorithm matches blocks, not individual pixels. Though, setting blockSize=1 reduces the
/// blocks to single pixels.
/// *   Mutual information cost function is not implemented. Instead, a simpler Birchfield-Tomasi
/// sub-pixel metric from [BT98](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BT98) is used. Though, the color images are supported as well.
/// *   Some pre- and post- processing steps from K. Konolige algorithm StereoBM are included, for
/// example: pre-filtering (StereoBM::PREFILTER_XSOBEL type) and post-filtering (uniqueness
/// check, quadratic interpolation and speckle filtering).
///
///
/// Note:
///    *   (Python) An example illustrating the use of the StereoSGBM matching algorithm can be found
///        at opencv_source_code/samples/python/stereo_match.py
// StereoSGBM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:347
pub struct StereoSGBM {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoSGBM }

impl Drop for StereoSGBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoSGBM_delete(self.as_raw_mut_StereoSGBM()) };
	}
}

unsafe impl Send for StereoSGBM {}

impl core::AlgorithmTraitConst for StereoSGBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoSGBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::stereo::StereoMatcherTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stereo::StereoMatcherTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, crate::stereo::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::stereo::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl crate::stereo::StereoSGBMTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::stereo::StereoSGBMTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, crate::stereo::StereoSGBMTraitConst, as_raw_StereoSGBM, crate::stereo::StereoSGBMTrait, as_raw_mut_StereoSGBM }

impl StereoSGBM {
	/// Creates StereoSGBM object
	///
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	///
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	///
	/// ## C++ default parameters
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	// create(int, int, int, int, int, int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
	// ("cv::StereoSGBM::create", vec![(pred!(mut, ["minDisparity", "numDisparities", "blockSize", "P1", "P2", "disp12MaxDiff", "preFilterCap", "uniquenessRatio", "speckleWindowSize", "speckleRange", "mode"], ["int", "int", "int", "int", "int", "int", "int", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn create(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32) -> Result<core::Ptr<crate::stereo::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(min_disparity, num_disparities, block_size, p1, p2, disp12_max_diff, pre_filter_cap, uniqueness_ratio, speckle_window_size, speckle_range, mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stereo::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates StereoSGBM object
	///
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	///
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	///
	/// ## Note
	/// This alternative version of [StereoSGBM::create] function uses the following default values for its arguments:
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	// cv::StereoSGBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
	// ("cv::StereoSGBM::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::stereo::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stereo::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StereoSGBM, core::Algorithm, cv_StereoSGBM_to_Algorithm }

boxed_cast_base! { StereoSGBM, crate::stereo::StereoMatcher, cv_StereoSGBM_to_StereoMatcher }

impl std::fmt::Debug for StereoSGBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoSGBM")
			.finish()
	}
}
