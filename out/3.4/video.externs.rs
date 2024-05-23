// CamShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:79
// ("cv::CamShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
pub fn cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<core::RotatedRect>);
// cv::buildOpticalFlowPyramid(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int"]), _)]),
pub fn cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int(img: *const c_void, pyramid: *const c_void, win_size: *const core::Size, max_level: i32, ocvrs_return: *mut Result<i32>);
// buildOpticalFlowPyramid(InputArray, OutputArrayOfArrays, Size, int, bool, int, int, bool)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel", "withDerivatives", "pyrBorder", "derivBorder", "tryReuseInputImage"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "bool", "int", "int", "bool"]), _)]),
pub fn cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img: *const c_void, pyramid: *const c_void, win_size: *const core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool, ocvrs_return: *mut Result<i32>);
// calcOpticalFlowFarneback(InputArray, InputArray, InputOutputArray, double, int, int, int, int, double, int)(InputArray, InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:223
// ("cv::calcOpticalFlowFarneback", vec![(pred!(mut, ["prev", "next", "flow", "pyr_scale", "levels", "winsize", "iterations", "poly_n", "poly_sigma", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "int", "int", "int", "int", "double", "int"]), _)]),
pub fn cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev: *const c_void, next: *const c_void, flow: *const c_void, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<()>);
// cv::calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Size, int, TermCriteria, int, double)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "winSize", "maxLevel", "criteria", "flags", "minEigThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
pub fn cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, win_size: *const core::Size, max_level: i32, criteria: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result<()>);
// cv::computeECC(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_computeECC_const__InputArrayR_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, ocvrs_return: *mut Result<f64>);
// computeECC(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::createBackgroundSubtractorKNN() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
pub fn cv_createBackgroundSubtractorKNN(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorKNN(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, ["history", "dist2Threshold", "detectShadows"], ["int", "double", "bool"]), _)]),
pub fn cv_createBackgroundSubtractorKNN_int_double_bool(history: i32, dist2_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
pub fn cv_createBackgroundSubtractorMOG2(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
pub fn cv_createBackgroundSubtractorMOG2_int_double_bool(history: i32, var_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:573
// ("cv::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_createOptFlow_DualTVL1(ocvrs_return: *mut Result<*mut c_void>);
// estimateRigidTransform(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:255
// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src: *const c_void, dst: *const c_void, full_affine: bool, ocvrs_return: *mut Result<*mut c_void>);
// estimateRigidTransform(InputArray, InputArray, bool, int, double, int)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:256
// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine", "ransacMaxIters", "ransacGoodRatio", "ransacSize0"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool", "int", "double", "int"]), _)]),
pub fn cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool_int_double_int(src: *const c_void, dst: *const c_void, full_affine: bool, ransac_max_iters: i32, ransac_good_ratio: f64, ransac_size0: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::findTransformECC(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, ocvrs_return: *mut Result<f64>);
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray, int)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:336
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask", "gaussFiltSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, gauss_filt_size: i32, ocvrs_return: *mut Result<f64>);
// meanShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:104
// ("cv::meanShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
pub fn cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<i32>);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractor::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:81
// ("cv::BackgroundSubtractor::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractor::to_BackgroundSubtractorKNN() generated
// ("cv::BackgroundSubtractor::to_BackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractor_to_BackgroundSubtractorKNN(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractor::to_BackgroundSubtractorMOG2() generated
// ("cv::BackgroundSubtractor::to_BackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractor_to_BackgroundSubtractorMOG2(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractor::to_Algorithm() generated
// ("cv::BackgroundSubtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractor::delete() generated
// ("cv::BackgroundSubtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractor_delete(instance: *mut c_void);
// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:234
// ("cv::BackgroundSubtractorKNN::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:237
// ("cv::BackgroundSubtractorKNN::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result<()>);
// getNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:241
// ("cv::BackgroundSubtractorKNN::getNSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:246
// ("cv::BackgroundSubtractorKNN::setNSamples", vec![(pred!(mut, ["_nN"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setNSamples_int(instance: *mut c_void, _n_n: i32, ocvrs_return: *mut Result<()>);
// getDist2Threshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:253
// ("cv::BackgroundSubtractorKNN::getDist2Threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getDist2Threshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDist2Threshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:256
// ("cv::BackgroundSubtractorKNN::setDist2Threshold", vec![(pred!(mut, ["_dist2Threshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setDist2Threshold_double(instance: *mut c_void, _dist2_threshold: f64, ocvrs_return: *mut Result<()>);
// getkNNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:263
// ("cv::BackgroundSubtractorKNN::getkNNSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getkNNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setkNNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:266
// ("cv::BackgroundSubtractorKNN::setkNNSamples", vec![(pred!(mut, ["_nkNN"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setkNNSamples_int(instance: *mut c_void, _nk_nn: i32, ocvrs_return: *mut Result<()>);
// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:273
// ("cv::BackgroundSubtractorKNN::getDetectShadows", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:276
// ("cv::BackgroundSubtractorKNN::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result<()>);
// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:283
// ("cv::BackgroundSubtractorKNN::getShadowValue", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:286
// ("cv::BackgroundSubtractorKNN::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:295
// ("cv::BackgroundSubtractorKNN::getShadowThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:298
// ("cv::BackgroundSubtractorKNN::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setShadowThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractorKNN::to_Algorithm() generated
// ("cv::BackgroundSubtractorKNN::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractorKNN::to_BackgroundSubtractor() generated
// ("cv::BackgroundSubtractorKNN::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractorKNN::delete() generated
// ("cv::BackgroundSubtractorKNN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_delete(instance: *mut c_void);
// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:95
// ("cv::BackgroundSubtractorMOG2::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:98
// ("cv::BackgroundSubtractorMOG2::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result<()>);
// getNMixtures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:102
// ("cv::BackgroundSubtractorMOG2::getNMixtures", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:107
// ("cv::BackgroundSubtractorMOG2::setNMixtures", vec![(pred!(mut, ["nmixtures"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setNMixtures_int(instance: *mut c_void, nmixtures: i32, ocvrs_return: *mut Result<()>);
// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:115
// ("cv::BackgroundSubtractorMOG2::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:118
// ("cv::BackgroundSubtractorMOG2::setBackgroundRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(instance: *mut c_void, ratio: f64, ocvrs_return: *mut Result<()>);
// getVarThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:125
// ("cv::BackgroundSubtractorMOG2::getVarThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:128
// ("cv::BackgroundSubtractorMOG2::setVarThreshold", vec![(pred!(mut, ["varThreshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarThreshold_double(instance: *mut c_void, var_threshold: f64, ocvrs_return: *mut Result<()>);
// getVarThresholdGen()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:138
// ("cv::BackgroundSubtractorMOG2::getVarThresholdGen", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarThresholdGen(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:141
// ("cv::BackgroundSubtractorMOG2::setVarThresholdGen", vec![(pred!(mut, ["varThresholdGen"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(instance: *mut c_void, var_threshold_gen: f64, ocvrs_return: *mut Result<()>);
// getVarInit()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:145
// ("cv::BackgroundSubtractorMOG2::getVarInit", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarInit_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarInit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:148
// ("cv::BackgroundSubtractorMOG2::setVarInit", vec![(pred!(mut, ["varInit"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarInit_double(instance: *mut c_void, var_init: f64, ocvrs_return: *mut Result<()>);
// getVarMin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:150
// ("cv::BackgroundSubtractorMOG2::getVarMin", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarMin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:151
// ("cv::BackgroundSubtractorMOG2::setVarMin", vec![(pred!(mut, ["varMin"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarMin_double(instance: *mut c_void, var_min: f64, ocvrs_return: *mut Result<()>);
// getVarMax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:153
// ("cv::BackgroundSubtractorMOG2::getVarMax", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarMax_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:154
// ("cv::BackgroundSubtractorMOG2::setVarMax", vec![(pred!(mut, ["varMax"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarMax_double(instance: *mut c_void, var_max: f64, ocvrs_return: *mut Result<()>);
// getComplexityReductionThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:162
// ("cv::BackgroundSubtractorMOG2::getComplexityReductionThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setComplexityReductionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:165
// ("cv::BackgroundSubtractorMOG2::setComplexityReductionThreshold", vec![(pred!(mut, ["ct"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(instance: *mut c_void, ct: f64, ocvrs_return: *mut Result<()>);
// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:172
// ("cv::BackgroundSubtractorMOG2::getDetectShadows", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:175
// ("cv::BackgroundSubtractorMOG2::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result<()>);
// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:182
// ("cv::BackgroundSubtractorMOG2::getShadowValue", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:185
// ("cv::BackgroundSubtractorMOG2::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:194
// ("cv::BackgroundSubtractorMOG2::getShadowThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:197
// ("cv::BackgroundSubtractorMOG2::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setShadowThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractorMOG2::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractorMOG2::to_Algorithm() generated
// ("cv::BackgroundSubtractorMOG2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractorMOG2::to_BackgroundSubtractor() generated
// ("cv::BackgroundSubtractorMOG2::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::BackgroundSubtractorMOG2::delete() generated
// ("cv::BackgroundSubtractorMOG2::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:422
// ("cv::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:425
// ("cv::DenseOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::DenseOpticalFlow::to_DualTVL1OpticalFlow() generated
// ("cv::DenseOpticalFlow::to_DualTVL1OpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_DualTVL1OpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::to_FarnebackOpticalFlow() generated
// ("cv::DenseOpticalFlow::to_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_FarnebackOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::to_Algorithm() generated
// ("cv::DenseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::delete() generated
// ("cv::DenseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_delete(instance: *mut c_void);
// getTau()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:496
// ("cv::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:498
// ("cv::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:501
// ("cv::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:503
// ("cv::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:506
// ("cv::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:508
// ("cv::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setTheta_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:511
// ("cv::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:513
// ("cv::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:516
// ("cv::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getScalesNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:518
// ("cv::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setScalesNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:521
// ("cv::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getWarpingsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:523
// ("cv::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setWarpingsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getEpsilon()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:526
// ("cv::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:528
// ("cv::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setEpsilon_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:531
// ("cv::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:533
// ("cv::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setInnerIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:536
// ("cv::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:538
// ("cv::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setOuterIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:541
// ("cv::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:543
// ("cv::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getScaleStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:546
// ("cv::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:548
// ("cv::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setScaleStep_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:551
// ("cv::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_getMedianFiltering_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:553
// ("cv::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DualTVL1OpticalFlow_setMedianFiltering_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
pub fn cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::DualTVL1OpticalFlow::to_Algorithm() generated
// ("cv::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DualTVL1OpticalFlow::to_DenseOpticalFlow() generated
// ("cv::DualTVL1OpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DualTVL1OpticalFlow::delete() generated
// ("cv::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DualTVL1OpticalFlow_delete(instance: *mut c_void);
// getNumLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:580
// ("cv::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:581
// ("cv::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setNumLevels_int(instance: *mut c_void, num_levels: i32, ocvrs_return: *mut Result<()>);
// getPyrScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:583
// ("cv::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:584
// ("cv::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, pyr_scale: f64, ocvrs_return: *mut Result<()>);
// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:586
// ("cv::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getFastPyramids_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:587
// ("cv::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
pub fn cv_FarnebackOpticalFlow_setFastPyramids_bool(instance: *mut c_void, fast_pyramids: bool, ocvrs_return: *mut Result<()>);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:589
// ("cv::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:590
// ("cv::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setWinSize_int(instance: *mut c_void, win_size: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:592
// ("cv::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:593
// ("cv::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setNumIters_int(instance: *mut c_void, num_iters: i32, ocvrs_return: *mut Result<()>);
// getPolyN()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:595
// ("cv::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:596
// ("cv::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, poly_n: i32, ocvrs_return: *mut Result<()>);
// getPolySigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:598
// ("cv::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:599
// ("cv::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, poly_sigma: f64, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:601
// ("cv::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:602
// ("cv::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result<()>);
// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
pub fn cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::FarnebackOpticalFlow::to_Algorithm() generated
// ("cv::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::FarnebackOpticalFlow::to_DenseOpticalFlow() generated
// ("cv::FarnebackOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::FarnebackOpticalFlow::delete() generated
// ("cv::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_delete(instance: *mut c_void);
// KalmanFilter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:363
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_KalmanFilter(ocvrs_return: *mut Result<*mut c_void>);
// KalmanFilter(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
pub fn cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::KalmanFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
pub fn cv_KalmanFilter_KalmanFilter_int_int(dynam_params: i32, measure_params: i32, ocvrs_return: *mut Result<*mut c_void>);
// init(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
pub fn cv_KalmanFilter_init_int_int_int_int(instance: *mut c_void, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result<()>);
// cv::KalmanFilter::init(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
pub fn cv_KalmanFilter_init_int_int(instance: *mut c_void, dynam_params: i32, measure_params: i32, ocvrs_return: *mut Result<()>);
// predict(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
// ("cv::KalmanFilter::predict", vec![(pred!(mut, ["control"], ["const cv::Mat*"]), _)]),
pub fn cv_KalmanFilter_predict_const_MatR(instance: *mut c_void, control: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::predict() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
// ("cv::KalmanFilter::predict", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_predict(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// correct(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:391
// ("cv::KalmanFilter::correct", vec![(pred!(mut, ["measurement"], ["const cv::Mat*"]), _)]),
pub fn cv_KalmanFilter_correct_const_MatR(instance: *mut c_void, measurement: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::statePre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
// ("cv::KalmanFilter::statePre", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propStatePre_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setStatePre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
// ("cv::KalmanFilter::setStatePre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propStatePre_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::statePost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
// ("cv::KalmanFilter::statePost", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propStatePost_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setStatePost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
// ("cv::KalmanFilter::setStatePost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propStatePost_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::transitionMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
// ("cv::KalmanFilter::transitionMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTransitionMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTransitionMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
// ("cv::KalmanFilter::setTransitionMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTransitionMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::controlMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
// ("cv::KalmanFilter::controlMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propControlMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setControlMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
// ("cv::KalmanFilter::setControlMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propControlMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::measurementMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
// ("cv::KalmanFilter::measurementMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propMeasurementMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setMeasurementMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
// ("cv::KalmanFilter::setMeasurementMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propMeasurementMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::processNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
// ("cv::KalmanFilter::processNoiseCov", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propProcessNoiseCov_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setProcessNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
// ("cv::KalmanFilter::setProcessNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propProcessNoiseCov_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::measurementNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
// ("cv::KalmanFilter::measurementNoiseCov", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propMeasurementNoiseCov_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setMeasurementNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
// ("cv::KalmanFilter::setMeasurementNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propMeasurementNoiseCov_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::errorCovPre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
// ("cv::KalmanFilter::errorCovPre", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propErrorCovPre_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setErrorCovPre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
// ("cv::KalmanFilter::setErrorCovPre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propErrorCovPre_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::gain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
// ("cv::KalmanFilter::gain", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propGain_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setGain(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
// ("cv::KalmanFilter::setGain", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propGain_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::errorCovPost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
// ("cv::KalmanFilter::errorCovPost", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propErrorCovPost_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setErrorCovPost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
// ("cv::KalmanFilter::setErrorCovPost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propErrorCovPost_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
// ("cv::KalmanFilter::temp1", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp1_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp1(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
// ("cv::KalmanFilter::setTemp1", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp1_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
// ("cv::KalmanFilter::temp2", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp2_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp2(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
// ("cv::KalmanFilter::setTemp2", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp2_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp3() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
// ("cv::KalmanFilter::temp3", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp3_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp3(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
// ("cv::KalmanFilter::setTemp3", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp3_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp4() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
// ("cv::KalmanFilter::temp4", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp4_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp4(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
// ("cv::KalmanFilter::setTemp4", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp4_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp5() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
// ("cv::KalmanFilter::temp5", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp5_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp5(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
// ("cv::KalmanFilter::setTemp5", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp5_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::delete() generated
// ("cv::KalmanFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result<()>);
// cv::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, ocvrs_return: *mut Result<()>);
// cv::SparseOpticalFlow::to_SparsePyrLKOpticalFlow() generated
// ("cv::SparseOpticalFlow::to_SparsePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseOpticalFlow_to_SparsePyrLKOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::SparseOpticalFlow::to_Algorithm() generated
// ("cv::SparseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::SparseOpticalFlow::delete() generated
// ("cv::SparseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseOpticalFlow_delete(instance: *mut c_void);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:627
// ("cv::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:628
// ("cv::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setWinSize_Size(instance: *mut c_void, win_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:630
// ("cv::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:631
// ("cv::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, max_level: i32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:633
// ("cv::SparsePyrLKOpticalFlow::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:634
// ("cv::SparsePyrLKOpticalFlow::setTermCriteria", vec![(pred!(mut, ["crit"], ["cv::TermCriteria*"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(instance: *mut c_void, crit: *mut core::TermCriteria, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:636
// ("cv::SparsePyrLKOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:637
// ("cv::SparsePyrLKOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result<()>);
// getMinEigThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:639
// ("cv::SparsePyrLKOpticalFlow::getMinEigThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinEigThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:640
// ("cv::SparsePyrLKOpticalFlow::setMinEigThreshold", vec![(pred!(mut, ["minEigThreshold"], ["double"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(instance: *mut c_void, min_eig_threshold: f64, ocvrs_return: *mut Result<()>);
// create(Size, int, TermCriteria, int, double)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "crit", "flags", "minEigThreshold"], ["cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size: *const core::Size, max_level: i32, crit: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::SparsePyrLKOpticalFlow::to_Algorithm() generated
// ("cv::SparsePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::SparsePyrLKOpticalFlow::to_SparseOpticalFlow() generated
// ("cv::SparsePyrLKOpticalFlow::to_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_to_SparseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::SparsePyrLKOpticalFlow::delete() generated
// ("cv::SparsePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_delete(instance: *mut c_void);
