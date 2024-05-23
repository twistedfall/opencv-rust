// cv::filterSpeckles(InputOutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff"], ["const cv::_InputOutputArray*", "double", "int", "double"]), _)]),
pub fn cv_filterSpeckles_const__InputOutputArrayR_double_int_double(img: *const c_void, new_val: f64, max_speckle_size: i32, max_diff: f64, ocvrs_return: *mut Result<()>);
// filterSpeckles(InputOutputArray, double, int, double, InputOutputArray)(InputOutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff", "buf"], ["const cv::_InputOutputArray*", "double", "int", "double", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(img: *const c_void, new_val: f64, max_speckle_size: i32, max_diff: f64, buf: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fisheye::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(k1: *const c_void, d1: *const c_void, k2: *const c_void, d2: *const c_void, image_size: *const core::Size, r: *const c_void, tvec: *const c_void, r1: *const c_void, r2: *const c_void, p1: *const c_void, p2: *const c_void, q: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// stereoRectify(InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, const Size &, double, double)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags", "newImageSize", "balance", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::Size*", "double", "double"]), _)]),
pub fn cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(k1: *const c_void, d1: *const c_void, k2: *const c_void, d2: *const c_void, image_size: *const core::Size, r: *const c_void, tvec: *const c_void, r1: *const c_void, r2: *const c_void, p1: *const c_void, p2: *const c_void, q: *const c_void, flags: i32, new_image_size: *const core::Size, balance: f64, fov_scale: f64, ocvrs_return: *mut Result<()>);
// getValidDisparityROI(Rect, Rect, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:434
// ("cv::getValidDisparityROI", vec![(pred!(mut, ["roi1", "roi2", "minDisparity", "numberOfDisparities", "blockSize"], ["cv::Rect", "cv::Rect", "int", "int", "int"]), _)]),
pub fn cv_getValidDisparityROI_Rect_Rect_int_int_int(roi1: *const core::Rect, roi2: *const core::Rect, min_disparity: i32, number_of_disparities: i32, block_size: i32, ocvrs_return: *mut Result<core::Rect>);
// rectify3Collinear(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArrayOfArrays, InputArrayOfArrays, Size, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double, Size, Rect *, Rect *, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:188
// ("cv::rectify3Collinear", vec![(pred!(mut, ["_cameraMatrix1", "_distCoeffs1", "_cameraMatrix2", "_distCoeffs2", "_cameraMatrix3", "_distCoeffs3", "_imgpt1", "_imgpt3", "imageSize", "_Rmat12", "_Tmat12", "_Rmat13", "_Tmat13", "_Rmat1", "_Rmat2", "_Rmat3", "_Pmat1", "_Pmat2", "_Pmat3", "_Qmat", "alpha", "newImgSize", "roi1", "roi2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Size", "cv::Rect*", "cv::Rect*", "int"]), _)]),
pub fn cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(_camera_matrix1: *const c_void, _dist_coeffs1: *const c_void, _camera_matrix2: *const c_void, _dist_coeffs2: *const c_void, _camera_matrix3: *const c_void, _dist_coeffs3: *const c_void, _imgpt1: *const c_void, _imgpt3: *const c_void, image_size: *const core::Size, _rmat12: *const c_void, _tmat12: *const c_void, _rmat13: *const c_void, _tmat13: *const c_void, _rmat1: *const c_void, _rmat2: *const c_void, _rmat3: *const c_void, _pmat1: *const c_void, _pmat2: *const c_void, _pmat3: *const c_void, _qmat: *const c_void, alpha: f64, new_img_size: *const core::Size, roi1: *mut core::Rect, roi2: *mut core::Rect, flags: i32, ocvrs_return: *mut Result<f32>);
// cv::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(disparity: *const c_void, _3d_image: *const c_void, q: *const c_void, ocvrs_return: *mut Result<()>);
// reprojectImageTo3D(InputArray, OutputArray, InputArray, bool, int)(InputArray, OutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q", "handleMissingValues", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "int"]), _)]),
pub fn cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(disparity: *const c_void, _3d_image: *const c_void, q: *const c_void, handle_missing_values: bool, ddepth: i32, ocvrs_return: *mut Result<()>);
// cv::stereoRectifyUncalibrated(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR(points1: *const c_void, points2: *const c_void, f: *const c_void, img_size: *const core::Size, h1: *const c_void, h2: *const c_void, ocvrs_return: *mut Result<bool>);
// stereoRectifyUncalibrated(InputArray, InputArray, InputArray, Size, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2", "threshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(points1: *const c_void, points2: *const c_void, f: *const c_void, img_size: *const core::Size, h1: *const c_void, h2: *const c_void, threshold: f64, ocvrs_return: *mut Result<bool>);
// cv::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, image_size: *const core::Size, r: *const c_void, t: *const c_void, r1: *const c_void, r2: *const c_void, p1: *const c_void, p2: *const c_void, q: *const c_void, ocvrs_return: *mut Result<()>);
// stereoRectify(InputArray, InputArray, InputArray, InputArray, Size, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, double, Size, Rect *, Rect *)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q", "flags", "alpha", "newImageSize", "validPixROI1", "validPixROI2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "cv::Size", "cv::Rect*", "cv::Rect*"]), _)]),
pub fn cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, image_size: *const core::Size, r: *const c_void, t: *const c_void, r1: *const c_void, r2: *const c_void, p1: *const c_void, p2: *const c_void, q: *const c_void, flags: i32, alpha: f64, new_image_size: *const core::Size, valid_pix_roi1: *mut core::Rect, valid_pix_roi2: *mut core::Rect, ocvrs_return: *mut Result<()>);
// cv::validateDisparity(InputOutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int(disparity: *const c_void, cost: *const c_void, min_disparity: i32, number_of_disparities: i32, ocvrs_return: *mut Result<()>);
// validateDisparity(InputOutputArray, InputArray, int, int, int)(InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities", "disp12MaxDisp"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
pub fn cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(disparity: *const c_void, cost: *const c_void, min_disparity: i32, number_of_disparities: i32, disp12_max_disp: i32, ocvrs_return: *mut Result<()>);
// getPreFilterType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:289
// ("cv::StereoBM::getPreFilterType", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getPreFilterType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPreFilterType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:290
// ("cv::StereoBM::setPreFilterType", vec![(pred!(mut, ["preFilterType"], ["int"]), _)]),
pub fn cv_StereoBM_setPreFilterType_int(instance: *mut c_void, pre_filter_type: i32, ocvrs_return: *mut Result<()>);
// getPreFilterSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:292
// ("cv::StereoBM::getPreFilterSize", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getPreFilterSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPreFilterSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:293
// ("cv::StereoBM::setPreFilterSize", vec![(pred!(mut, ["preFilterSize"], ["int"]), _)]),
pub fn cv_StereoBM_setPreFilterSize_int(instance: *mut c_void, pre_filter_size: i32, ocvrs_return: *mut Result<()>);
// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:295
// ("cv::StereoBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getPreFilterCap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:296
// ("cv::StereoBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
pub fn cv_StereoBM_setPreFilterCap_int(instance: *mut c_void, pre_filter_cap: i32, ocvrs_return: *mut Result<()>);
// getTextureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:298
// ("cv::StereoBM::getTextureThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getTextureThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTextureThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:299
// ("cv::StereoBM::setTextureThreshold", vec![(pred!(mut, ["textureThreshold"], ["int"]), _)]),
pub fn cv_StereoBM_setTextureThreshold_int(instance: *mut c_void, texture_threshold: i32, ocvrs_return: *mut Result<()>);
// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:301
// ("cv::StereoBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getUniquenessRatio_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:302
// ("cv::StereoBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
pub fn cv_StereoBM_setUniquenessRatio_int(instance: *mut c_void, uniqueness_ratio: i32, ocvrs_return: *mut Result<()>);
// getSmallerBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:304
// ("cv::StereoBM::getSmallerBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getSmallerBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSmallerBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:305
// ("cv::StereoBM::setSmallerBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
pub fn cv_StereoBM_setSmallerBlockSize_int(instance: *mut c_void, block_size: i32, ocvrs_return: *mut Result<()>);
// getROI1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:307
// ("cv::StereoBM::getROI1", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getROI1_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// setROI1(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:308
// ("cv::StereoBM::setROI1", vec![(pred!(mut, ["roi1"], ["cv::Rect"]), _)]),
pub fn cv_StereoBM_setROI1_Rect(instance: *mut c_void, roi1: *const core::Rect, ocvrs_return: *mut Result<()>);
// getROI2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:310
// ("cv::StereoBM::getROI2", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoBM_getROI2_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// setROI2(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:311
// ("cv::StereoBM::setROI2", vec![(pred!(mut, ["roi2"], ["cv::Rect"]), _)]),
pub fn cv_StereoBM_setROI2_Rect(instance: *mut c_void, roi2: *const core::Rect, ocvrs_return: *mut Result<()>);
// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
// ("cv::StereoBM::create", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
pub fn cv_StereoBM_create_int_int(num_disparities: i32, block_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::StereoBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
// ("cv::StereoBM::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoBM_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::StereoBM::to_Algorithm() generated
// ("cv::StereoBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoBM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::StereoBM::to_StereoMatcher() generated
// ("cv::StereoBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoBM_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::StereoBM::delete() generated
// ("cv::StereoBM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoBM_delete(instance: *mut c_void);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:256
// ("cv::StereoMatcher::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, left: *const c_void, right: *const c_void, disparity: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDisparity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:259
// ("cv::StereoMatcher::getMinDisparity", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getMinDisparity_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinDisparity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:260
// ("cv::StereoMatcher::setMinDisparity", vec![(pred!(mut, ["minDisparity"], ["int"]), _)]),
pub fn cv_StereoMatcher_setMinDisparity_int(instance: *mut c_void, min_disparity: i32, ocvrs_return: *mut Result<()>);
// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:262
// ("cv::StereoMatcher::getNumDisparities", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getNumDisparities_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:263
// ("cv::StereoMatcher::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
pub fn cv_StereoMatcher_setNumDisparities_int(instance: *mut c_void, num_disparities: i32, ocvrs_return: *mut Result<()>);
// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:265
// ("cv::StereoMatcher::getBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:266
// ("cv::StereoMatcher::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
pub fn cv_StereoMatcher_setBlockSize_int(instance: *mut c_void, block_size: i32, ocvrs_return: *mut Result<()>);
// getSpeckleWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:268
// ("cv::StereoMatcher::getSpeckleWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getSpeckleWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSpeckleWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:269
// ("cv::StereoMatcher::setSpeckleWindowSize", vec![(pred!(mut, ["speckleWindowSize"], ["int"]), _)]),
pub fn cv_StereoMatcher_setSpeckleWindowSize_int(instance: *mut c_void, speckle_window_size: i32, ocvrs_return: *mut Result<()>);
// getSpeckleRange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:271
// ("cv::StereoMatcher::getSpeckleRange", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getSpeckleRange_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSpeckleRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:272
// ("cv::StereoMatcher::setSpeckleRange", vec![(pred!(mut, ["speckleRange"], ["int"]), _)]),
pub fn cv_StereoMatcher_setSpeckleRange_int(instance: *mut c_void, speckle_range: i32, ocvrs_return: *mut Result<()>);
// getDisp12MaxDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:274
// ("cv::StereoMatcher::getDisp12MaxDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoMatcher_getDisp12MaxDiff_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDisp12MaxDiff(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:275
// ("cv::StereoMatcher::setDisp12MaxDiff", vec![(pred!(mut, ["disp12MaxDiff"], ["int"]), _)]),
pub fn cv_StereoMatcher_setDisp12MaxDiff_int(instance: *mut c_void, disp12_max_diff: i32, ocvrs_return: *mut Result<()>);
// cv::StereoMatcher::to_StereoBM() generated
// ("cv::StereoMatcher::to_StereoBM", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoMatcher_to_StereoBM(instance: *mut c_void) -> *mut c_void;
// cv::StereoMatcher::to_StereoSGBM() generated
// ("cv::StereoMatcher::to_StereoSGBM", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoMatcher_to_StereoSGBM(instance: *mut c_void) -> *mut c_void;
// cv::StereoMatcher::to_Algorithm() generated
// ("cv::StereoMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::StereoMatcher::delete() generated
// ("cv::StereoMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoMatcher_delete(instance: *mut c_void);
// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:358
// ("cv::StereoSGBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoSGBM_getPreFilterCap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:359
// ("cv::StereoSGBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
pub fn cv_StereoSGBM_setPreFilterCap_int(instance: *mut c_void, pre_filter_cap: i32, ocvrs_return: *mut Result<()>);
// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:361
// ("cv::StereoSGBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoSGBM_getUniquenessRatio_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:362
// ("cv::StereoSGBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
pub fn cv_StereoSGBM_setUniquenessRatio_int(instance: *mut c_void, uniqueness_ratio: i32, ocvrs_return: *mut Result<()>);
// getP1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:364
// ("cv::StereoSGBM::getP1", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoSGBM_getP1_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setP1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:365
// ("cv::StereoSGBM::setP1", vec![(pred!(mut, ["P1"], ["int"]), _)]),
pub fn cv_StereoSGBM_setP1_int(instance: *mut c_void, p1: i32, ocvrs_return: *mut Result<()>);
// getP2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:367
// ("cv::StereoSGBM::getP2", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoSGBM_getP2_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setP2(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:368
// ("cv::StereoSGBM::setP2", vec![(pred!(mut, ["P2"], ["int"]), _)]),
pub fn cv_StereoSGBM_setP2_int(instance: *mut c_void, p2: i32, ocvrs_return: *mut Result<()>);
// getMode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:370
// ("cv::StereoSGBM::getMode", vec![(pred!(const, [], []), _)]),
pub fn cv_StereoSGBM_getMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:371
// ("cv::StereoSGBM::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
pub fn cv_StereoSGBM_setMode_int(instance: *mut c_void, mode: i32, ocvrs_return: *mut Result<()>);
// create(int, int, int, int, int, int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
// ("cv::StereoSGBM::create", vec![(pred!(mut, ["minDisparity", "numDisparities", "blockSize", "P1", "P2", "disp12MaxDiff", "preFilterCap", "uniquenessRatio", "speckleWindowSize", "speckleRange", "mode"], ["int", "int", "int", "int", "int", "int", "int", "int", "int", "int", "int"]), _)]),
pub fn cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::StereoSGBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
// ("cv::StereoSGBM::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoSGBM_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::StereoSGBM::to_Algorithm() generated
// ("cv::StereoSGBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoSGBM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::StereoSGBM::to_StereoMatcher() generated
// ("cv::StereoSGBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoSGBM_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::StereoSGBM::delete() generated
// ("cv::StereoSGBM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereoSGBM_delete(instance: *mut c_void);
