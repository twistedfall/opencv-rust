// CamShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:79
// ("cv::CamShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
pub fn cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<core::RotatedRect>);
// cv::buildOpticalFlowPyramid(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int"]), _)]),
pub fn cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int(img: *const c_void, pyramid: *const c_void, win_size: *const core::Size, max_level: i32, ocvrs_return: *mut Result<i32>);
// buildOpticalFlowPyramid(InputArray, OutputArrayOfArrays, Size, int, bool, int, int, bool)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:121
// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel", "withDerivatives", "pyrBorder", "derivBorder", "tryReuseInputImage"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "bool", "int", "int", "bool"]), _)]),
pub fn cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img: *const c_void, pyramid: *const c_void, win_size: *const core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool, ocvrs_return: *mut Result<i32>);
// calcOpticalFlowFarneback(InputArray, InputArray, InputOutputArray, double, int, int, int, int, double, int)(InputArray, InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:223
// ("cv::calcOpticalFlowFarneback", vec![(pred!(mut, ["prev", "next", "flow", "pyr_scale", "levels", "winsize", "iterations", "poly_n", "poly_sigma", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "int", "int", "int", "int", "double", "int"]), _)]),
pub fn cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev: *const c_void, next: *const c_void, flow: *const c_void, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<()>);
// cv::calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Size, int, TermCriteria, int, double)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:178
// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "winSize", "maxLevel", "criteria", "flags", "minEigThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
pub fn cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, win_size: *const core::Size, max_level: i32, criteria: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result<()>);
// cv::computeECC(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_computeECC_const__InputArrayR_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, ocvrs_return: *mut Result<f64>);
// computeECC(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:279
// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::createBackgroundSubtractorKNN() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
pub fn cv_createBackgroundSubtractorKNN(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorKNN(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:310
// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, ["history", "dist2Threshold", "detectShadows"], ["int", "double", "bool"]), _)]),
pub fn cv_createBackgroundSubtractorKNN_int_double_bool(history: i32, dist2_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
pub fn cv_createBackgroundSubtractorMOG2(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:221
// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
pub fn cv_createBackgroundSubtractorMOG2_int_double_bool(history: i32, var_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
// estimateRigidTransform(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:258
// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src: *const c_void, dst: *const c_void, full_affine: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::findTransformECC(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, ocvrs_return: *mut Result<f64>);
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:343
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray, int)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:336
// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask", "gaussFiltSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, gauss_filt_size: i32, ocvrs_return: *mut Result<f64>);
// meanShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:104
// ("cv::meanShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
pub fn cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<i32>);
// readOpticalFlow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:421
// ("cv::readOpticalFlow", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_readOpticalFlow_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// writeOpticalFlow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:431
// ("cv::writeOpticalFlow", vec![(pred!(mut, ["path", "flow"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_writeOpticalFlow_const_StringR_const__InputArrayR(path: *const c_char, flow: *const c_void, ocvrs_return: *mut Result<bool>);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:72
// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractor::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:72
// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:81
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
// getHistory()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:234
// ("cv::BackgroundSubtractorKNN::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:237
// ("cv::BackgroundSubtractorKNN::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result<()>);
// getNSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:241
// ("cv::BackgroundSubtractorKNN::getNSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:246
// ("cv::BackgroundSubtractorKNN::setNSamples", vec![(pred!(mut, ["_nN"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setNSamples_int(instance: *mut c_void, _n_n: i32, ocvrs_return: *mut Result<()>);
// getDist2Threshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:253
// ("cv::BackgroundSubtractorKNN::getDist2Threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getDist2Threshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDist2Threshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:256
// ("cv::BackgroundSubtractorKNN::setDist2Threshold", vec![(pred!(mut, ["_dist2Threshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setDist2Threshold_double(instance: *mut c_void, _dist2_threshold: f64, ocvrs_return: *mut Result<()>);
// getkNNSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:263
// ("cv::BackgroundSubtractorKNN::getkNNSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getkNNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setkNNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:266
// ("cv::BackgroundSubtractorKNN::setkNNSamples", vec![(pred!(mut, ["_nkNN"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setkNNSamples_int(instance: *mut c_void, _nk_nn: i32, ocvrs_return: *mut Result<()>);
// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:273
// ("cv::BackgroundSubtractorKNN::getDetectShadows", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:276
// ("cv::BackgroundSubtractorKNN::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result<()>);
// getShadowValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:283
// ("cv::BackgroundSubtractorKNN::getShadowValue", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:286
// ("cv::BackgroundSubtractorKNN::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorKNN_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:295
// ("cv::BackgroundSubtractorKNN::getShadowThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorKNN_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:298
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
// getHistory()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:95
// ("cv::BackgroundSubtractorMOG2::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:98
// ("cv::BackgroundSubtractorMOG2::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result<()>);
// getNMixtures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:102
// ("cv::BackgroundSubtractorMOG2::getNMixtures", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:107
// ("cv::BackgroundSubtractorMOG2::setNMixtures", vec![(pred!(mut, ["nmixtures"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setNMixtures_int(instance: *mut c_void, nmixtures: i32, ocvrs_return: *mut Result<()>);
// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:115
// ("cv::BackgroundSubtractorMOG2::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:118
// ("cv::BackgroundSubtractorMOG2::setBackgroundRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(instance: *mut c_void, ratio: f64, ocvrs_return: *mut Result<()>);
// getVarThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:125
// ("cv::BackgroundSubtractorMOG2::getVarThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:128
// ("cv::BackgroundSubtractorMOG2::setVarThreshold", vec![(pred!(mut, ["varThreshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarThreshold_double(instance: *mut c_void, var_threshold: f64, ocvrs_return: *mut Result<()>);
// getVarThresholdGen()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:138
// ("cv::BackgroundSubtractorMOG2::getVarThresholdGen", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarThresholdGen(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:141
// ("cv::BackgroundSubtractorMOG2::setVarThresholdGen", vec![(pred!(mut, ["varThresholdGen"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(instance: *mut c_void, var_threshold_gen: f64, ocvrs_return: *mut Result<()>);
// getVarInit()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:145
// ("cv::BackgroundSubtractorMOG2::getVarInit", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarInit_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarInit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:148
// ("cv::BackgroundSubtractorMOG2::setVarInit", vec![(pred!(mut, ["varInit"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarInit_double(instance: *mut c_void, var_init: f64, ocvrs_return: *mut Result<()>);
// getVarMin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:150
// ("cv::BackgroundSubtractorMOG2::getVarMin", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarMin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:151
// ("cv::BackgroundSubtractorMOG2::setVarMin", vec![(pred!(mut, ["varMin"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarMin_double(instance: *mut c_void, var_min: f64, ocvrs_return: *mut Result<()>);
// getVarMax()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:153
// ("cv::BackgroundSubtractorMOG2::getVarMax", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getVarMax_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setVarMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:154
// ("cv::BackgroundSubtractorMOG2::setVarMax", vec![(pred!(mut, ["varMax"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setVarMax_double(instance: *mut c_void, var_max: f64, ocvrs_return: *mut Result<()>);
// getComplexityReductionThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:162
// ("cv::BackgroundSubtractorMOG2::getComplexityReductionThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setComplexityReductionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:165
// ("cv::BackgroundSubtractorMOG2::setComplexityReductionThreshold", vec![(pred!(mut, ["ct"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(instance: *mut c_void, ct: f64, ocvrs_return: *mut Result<()>);
// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:172
// ("cv::BackgroundSubtractorMOG2::getDetectShadows", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:175
// ("cv::BackgroundSubtractorMOG2::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result<()>);
// getShadowValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:182
// ("cv::BackgroundSubtractorMOG2::getShadowValue", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:185
// ("cv::BackgroundSubtractorMOG2::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:194
// ("cv::BackgroundSubtractorMOG2::getShadowThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BackgroundSubtractorMOG2_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:197
// ("cv::BackgroundSubtractorMOG2::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_setShadowThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:208
// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::BackgroundSubtractorMOG2::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:208
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
// getFinestScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:603
// ("cv::DISOpticalFlow::getFinestScale", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getFinestScale_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFinestScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:605
// ("cv::DISOpticalFlow::setFinestScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_setFinestScale_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPatchSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:610
// ("cv::DISOpticalFlow::getPatchSize", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:612
// ("cv::DISOpticalFlow::setPatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_setPatchSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPatchStride()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:617
// ("cv::DISOpticalFlow::getPatchStride", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getPatchStride_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatchStride(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:619
// ("cv::DISOpticalFlow::setPatchStride", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_setPatchStride_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getGradientDescentIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:624
// ("cv::DISOpticalFlow::getGradientDescentIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getGradientDescentIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGradientDescentIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:626
// ("cv::DISOpticalFlow::setGradientDescentIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_setGradientDescentIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:632
// ("cv::DISOpticalFlow::getVariationalRefinementIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getVariationalRefinementIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setVariationalRefinementIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:634
// ("cv::DISOpticalFlow::setVariationalRefinementIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_setVariationalRefinementIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:638
// ("cv::DISOpticalFlow::getVariationalRefinementAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getVariationalRefinementAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:640
// ("cv::DISOpticalFlow::setVariationalRefinementAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_DISOpticalFlow_setVariationalRefinementAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:644
// ("cv::DISOpticalFlow::getVariationalRefinementDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getVariationalRefinementDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:646
// ("cv::DISOpticalFlow::setVariationalRefinementDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_DISOpticalFlow_setVariationalRefinementDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:650
// ("cv::DISOpticalFlow::getVariationalRefinementGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getVariationalRefinementGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:652
// ("cv::DISOpticalFlow::setVariationalRefinementGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_DISOpticalFlow_setVariationalRefinementGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:656
// ("cv::DISOpticalFlow::getVariationalRefinementEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getVariationalRefinementEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementEpsilon(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:658
// ("cv::DISOpticalFlow::setVariationalRefinementEpsilon", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_DISOpticalFlow_setVariationalRefinementEpsilon_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getUseMeanNormalization()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:666
// ("cv::DISOpticalFlow::getUseMeanNormalization", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getUseMeanNormalization_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseMeanNormalization(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:668
// ("cv::DISOpticalFlow::setUseMeanNormalization", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_DISOpticalFlow_setUseMeanNormalization_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseSpatialPropagation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:675
// ("cv::DISOpticalFlow::getUseSpatialPropagation", vec![(pred!(const, [], []), _)]),
pub fn cv_DISOpticalFlow_getUseSpatialPropagation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseSpatialPropagation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:677
// ("cv::DISOpticalFlow::setUseSpatialPropagation", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_DISOpticalFlow_setUseSpatialPropagation_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:683
// ("cv::DISOpticalFlow::create", vec![(pred!(mut, ["preset"], ["int"]), _)]),
pub fn cv_DISOpticalFlow_create_int(preset: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::DISOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:683
// ("cv::DISOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_DISOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::DISOpticalFlow::to_Algorithm() generated
// ("cv::DISOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DISOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DISOpticalFlow::to_DenseOpticalFlow() generated
// ("cv::DISOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DISOpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DISOpticalFlow::delete() generated
// ("cv::DISOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DISOpticalFlow_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:445
// ("cv::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:448
// ("cv::DenseOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::DenseOpticalFlow::to_DISOpticalFlow() generated
// ("cv::DenseOpticalFlow::to_DISOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_DISOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::to_FarnebackOpticalFlow() generated
// ("cv::DenseOpticalFlow::to_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_FarnebackOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::to_VariationalRefinement() generated
// ("cv::DenseOpticalFlow::to_VariationalRefinement", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_VariationalRefinement(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::to_Algorithm() generated
// ("cv::DenseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DenseOpticalFlow::delete() generated
// ("cv::DenseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DenseOpticalFlow_delete(instance: *mut c_void);
// getNumLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:478
// ("cv::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:479
// ("cv::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setNumLevels_int(instance: *mut c_void, num_levels: i32, ocvrs_return: *mut Result<()>);
// getPyrScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:481
// ("cv::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:482
// ("cv::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, pyr_scale: f64, ocvrs_return: *mut Result<()>);
// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:484
// ("cv::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getFastPyramids_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:485
// ("cv::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
pub fn cv_FarnebackOpticalFlow_setFastPyramids_bool(instance: *mut c_void, fast_pyramids: bool, ocvrs_return: *mut Result<()>);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:487
// ("cv::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:488
// ("cv::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setWinSize_int(instance: *mut c_void, win_size: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:490
// ("cv::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:491
// ("cv::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setNumIters_int(instance: *mut c_void, num_iters: i32, ocvrs_return: *mut Result<()>);
// getPolyN()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:493
// ("cv::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:494
// ("cv::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, poly_n: i32, ocvrs_return: *mut Result<()>);
// getPolySigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:496
// ("cv::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:497
// ("cv::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
pub fn cv_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, poly_sigma: f64, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:499
// ("cv::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:500
// ("cv::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result<()>);
// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:502
// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
pub fn cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:502
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
// KalmanFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:363
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_KalmanFilter(ocvrs_return: *mut Result<*mut c_void>);
// KalmanFilter(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:370
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
pub fn cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::KalmanFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:370
// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
pub fn cv_KalmanFilter_KalmanFilter_int_int(dynam_params: i32, measure_params: i32, ocvrs_return: *mut Result<*mut c_void>);
// init(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:379
// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
pub fn cv_KalmanFilter_init_int_int_int_int(instance: *mut c_void, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result<()>);
// cv::KalmanFilter::init(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:379
// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
pub fn cv_KalmanFilter_init_int_int(instance: *mut c_void, dynam_params: i32, measure_params: i32, ocvrs_return: *mut Result<()>);
// predict(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:385
// ("cv::KalmanFilter::predict", vec![(pred!(mut, ["control"], ["const cv::Mat*"]), _)]),
pub fn cv_KalmanFilter_predict_const_MatR(instance: *mut c_void, control: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::predict() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:385
// ("cv::KalmanFilter::predict", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_predict(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// correct(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:391
// ("cv::KalmanFilter::correct", vec![(pred!(mut, ["measurement"], ["const cv::Mat*"]), _)]),
pub fn cv_KalmanFilter_correct_const_MatR(instance: *mut c_void, measurement: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::KalmanFilter::statePre() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:393
// ("cv::KalmanFilter::statePre", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propStatePre_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setStatePre(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:393
// ("cv::KalmanFilter::setStatePre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propStatePre_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::statePost() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:394
// ("cv::KalmanFilter::statePost", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propStatePost_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setStatePost(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:394
// ("cv::KalmanFilter::setStatePost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propStatePost_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::transitionMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:395
// ("cv::KalmanFilter::transitionMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTransitionMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTransitionMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:395
// ("cv::KalmanFilter::setTransitionMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTransitionMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::controlMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:396
// ("cv::KalmanFilter::controlMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propControlMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setControlMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:396
// ("cv::KalmanFilter::setControlMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propControlMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::measurementMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:397
// ("cv::KalmanFilter::measurementMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propMeasurementMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setMeasurementMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:397
// ("cv::KalmanFilter::setMeasurementMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propMeasurementMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::processNoiseCov() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:398
// ("cv::KalmanFilter::processNoiseCov", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propProcessNoiseCov_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setProcessNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:398
// ("cv::KalmanFilter::setProcessNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propProcessNoiseCov_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::measurementNoiseCov() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:399
// ("cv::KalmanFilter::measurementNoiseCov", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propMeasurementNoiseCov_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setMeasurementNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:399
// ("cv::KalmanFilter::setMeasurementNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propMeasurementNoiseCov_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::errorCovPre() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:400
// ("cv::KalmanFilter::errorCovPre", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propErrorCovPre_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setErrorCovPre(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:400
// ("cv::KalmanFilter::setErrorCovPre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propErrorCovPre_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::gain() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:401
// ("cv::KalmanFilter::gain", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propGain_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setGain(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:401
// ("cv::KalmanFilter::setGain", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propGain_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::errorCovPost() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:402
// ("cv::KalmanFilter::errorCovPost", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propErrorCovPost_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setErrorCovPost(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:402
// ("cv::KalmanFilter::setErrorCovPost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propErrorCovPost_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:405
// ("cv::KalmanFilter::temp1", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp1_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp1(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:405
// ("cv::KalmanFilter::setTemp1", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp1_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:406
// ("cv::KalmanFilter::temp2", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp2_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp2(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:406
// ("cv::KalmanFilter::setTemp2", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp2_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp3() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:407
// ("cv::KalmanFilter::temp3", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp3_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp3(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:407
// ("cv::KalmanFilter::setTemp3", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp3_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp4() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:408
// ("cv::KalmanFilter::temp4", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp4_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp4(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:408
// ("cv::KalmanFilter::setTemp4", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp4_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::temp5() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:409
// ("cv::KalmanFilter::temp5", vec![(pred!(const, [], []), _)]),
pub fn cv_KalmanFilter_propTemp5_const(instance: *const c_void) -> *mut c_void;
// cv::KalmanFilter::setTemp5(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:409
// ("cv::KalmanFilter::setTemp5", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_KalmanFilter_propTemp5_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::KalmanFilter::delete() generated
// ("cv::KalmanFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_KalmanFilter_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:466
// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result<()>);
// cv::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:466
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
// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:697
// ("cv::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:698
// ("cv::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setWinSize_Size(instance: *mut c_void, win_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:700
// ("cv::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:701
// ("cv::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, max_level: i32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:703
// ("cv::SparsePyrLKOpticalFlow::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:704
// ("cv::SparsePyrLKOpticalFlow::setTermCriteria", vec![(pred!(mut, ["crit"], ["cv::TermCriteria*"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(instance: *mut c_void, crit: *mut core::TermCriteria, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:706
// ("cv::SparsePyrLKOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:707
// ("cv::SparsePyrLKOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result<()>);
// getMinEigThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:709
// ("cv::SparsePyrLKOpticalFlow::getMinEigThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinEigThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:710
// ("cv::SparsePyrLKOpticalFlow::setMinEigThreshold", vec![(pred!(mut, ["minEigThreshold"], ["double"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(instance: *mut c_void, min_eig_threshold: f64, ocvrs_return: *mut Result<()>);
// create(Size, int, TermCriteria, int, double)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:712
// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "crit", "flags", "minEigThreshold"], ["cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
pub fn cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size: *const core::Size, max_level: i32, crit: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:712
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
// init(InputArray, const Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:737
// ("cv::Tracker::init", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "const cv::Rect*"]), _)]),
pub fn cv_Tracker_init_const__InputArrayR_const_RectR(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect, ocvrs_return: *mut Result<()>);
// update(InputArray, Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:749
// ("cv::Tracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "cv::Rect*"]), _)]),
pub fn cv_Tracker_update_const__InputArrayR_RectR(instance: *mut c_void, image: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<bool>);
// cv::Tracker::to_TrackerDaSiamRPN() generated
// ("cv::Tracker::to_TrackerDaSiamRPN", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerDaSiamRPN(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerGOTURN() generated
// ("cv::Tracker::to_TrackerGOTURN", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerGOTURN(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerMIL() generated
// ("cv::Tracker::to_TrackerMIL", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerMIL(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerNano() generated
// ("cv::Tracker::to_TrackerNano", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerNano(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerVit() generated
// ("cv::Tracker::to_TrackerVit", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerVit(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::delete() generated
// ("cv::Tracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_delete(instance: *mut c_void);
// create(const TrackerDaSiamRPN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:854
// ("cv::TrackerDaSiamRPN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerDaSiamRPN::Params*"]), _)]),
pub fn cv_TrackerDaSiamRPN_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerDaSiamRPN::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:854
// ("cv::TrackerDaSiamRPN::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_create(ocvrs_return: *mut Result<*mut c_void>);
// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:858
// ("cv::TrackerDaSiamRPN::getTrackingScore", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_getTrackingScore(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// cv::TrackerDaSiamRPN::to_Tracker() generated
// ("cv::TrackerDaSiamRPN::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerDaSiamRPN::delete() generated
// ("cv::TrackerDaSiamRPN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:842
// ("cv::TrackerDaSiamRPN::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerDaSiamRPN::Params::implicitClone() generated
// ("cv::TrackerDaSiamRPN::Params::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerDaSiamRPN::Params::model() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:843
// ("cv::TrackerDaSiamRPN::Params::model", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propModel_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerDaSiamRPN::Params::setModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:843
// ("cv::TrackerDaSiamRPN::Params::setModel", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propModel_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerDaSiamRPN::Params::kernel_cls1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:844
// ("cv::TrackerDaSiamRPN::Params::kernel_cls1", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propKernel_cls1_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerDaSiamRPN::Params::setKernel_cls1(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:844
// ("cv::TrackerDaSiamRPN::Params::setKernel_cls1", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propKernel_cls1_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerDaSiamRPN::Params::kernel_r1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:845
// ("cv::TrackerDaSiamRPN::Params::kernel_r1", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propKernel_r1_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerDaSiamRPN::Params::setKernel_r1(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:845
// ("cv::TrackerDaSiamRPN::Params::setKernel_r1", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propKernel_r1_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerDaSiamRPN::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:846
// ("cv::TrackerDaSiamRPN::Params::backend", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propBackend_const(instance: *const c_void) -> i32;
// cv::TrackerDaSiamRPN::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:846
// ("cv::TrackerDaSiamRPN::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propBackend_const_int(instance: *mut c_void, val: i32);
// cv::TrackerDaSiamRPN::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:847
// ("cv::TrackerDaSiamRPN::Params::target", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propTarget_const(instance: *const c_void) -> i32;
// cv::TrackerDaSiamRPN::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:847
// ("cv::TrackerDaSiamRPN::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerDaSiamRPN_Params_propTarget_const_int(instance: *mut c_void, val: i32);
// cv::TrackerDaSiamRPN::Params::delete() generated
// ("cv::TrackerDaSiamRPN::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerDaSiamRPN_Params_delete(instance: *mut c_void);
// create(const TrackerGOTURN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:827
// ("cv::TrackerGOTURN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerGOTURN::Params*"]), _)]),
pub fn cv_TrackerGOTURN_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerGOTURN::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:827
// ("cv::TrackerGOTURN::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerGOTURN::to_Tracker() generated
// ("cv::TrackerGOTURN::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerGOTURN::delete() generated
// ("cv::TrackerGOTURN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:818
// ("cv::TrackerGOTURN::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerGOTURN::Params::implicitClone() generated
// ("cv::TrackerGOTURN::Params::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerGOTURN::Params::modelTxt() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:819
// ("cv::TrackerGOTURN::Params::modelTxt", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_propModelTxt_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerGOTURN::Params::setModelTxt(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:819
// ("cv::TrackerGOTURN::Params::setModelTxt", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerGOTURN_Params_propModelTxt_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerGOTURN::Params::modelBin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:820
// ("cv::TrackerGOTURN::Params::modelBin", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_propModelBin_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerGOTURN::Params::setModelBin(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:820
// ("cv::TrackerGOTURN::Params::setModelBin", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerGOTURN_Params_propModelBin_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerGOTURN::Params::delete() generated
// ("cv::TrackerGOTURN::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_delete(instance: *mut c_void);
// create(const TrackerMIL::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:786
// ("cv::TrackerMIL::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMIL::Params*"]), _)]),
pub fn cv_TrackerMIL_create_const_ParamsR(parameters: *const crate::video::TrackerMIL_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerMIL::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:786
// ("cv::TrackerMIL::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerMIL::to_Tracker() generated
// ("cv::TrackerMIL::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMIL::delete() generated
// ("cv::TrackerMIL::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:771
// ("cv::TrackerMIL::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_Params_Params(ocvrs_return: *mut Result<crate::video::TrackerMIL_Params>);
// create(const TrackerNano::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:892
// ("cv::TrackerNano::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerNano::Params*"]), _)]),
pub fn cv_TrackerNano_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerNano::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:892
// ("cv::TrackerNano::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_create(ocvrs_return: *mut Result<*mut c_void>);
// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:896
// ("cv::TrackerNano::getTrackingScore", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_getTrackingScore(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// cv::TrackerNano::to_Tracker() generated
// ("cv::TrackerNano::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerNano::delete() generated
// ("cv::TrackerNano::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:881
// ("cv::TrackerNano::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerNano::Params::implicitClone() generated
// ("cv::TrackerNano::Params::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerNano_Params_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerNano::Params::backbone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:882
// ("cv::TrackerNano::Params::backbone", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerNano_Params_propBackbone_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerNano::Params::setBackbone(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:882
// ("cv::TrackerNano::Params::setBackbone", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerNano_Params_propBackbone_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerNano::Params::neckhead() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:883
// ("cv::TrackerNano::Params::neckhead", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerNano_Params_propNeckhead_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerNano::Params::setNeckhead(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:883
// ("cv::TrackerNano::Params::setNeckhead", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerNano_Params_propNeckhead_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerNano::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:884
// ("cv::TrackerNano::Params::backend", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerNano_Params_propBackend_const(instance: *const c_void) -> i32;
// cv::TrackerNano::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:884
// ("cv::TrackerNano::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerNano_Params_propBackend_const_int(instance: *mut c_void, val: i32);
// cv::TrackerNano::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:885
// ("cv::TrackerNano::Params::target", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerNano_Params_propTarget_const(instance: *const c_void) -> i32;
// cv::TrackerNano::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:885
// ("cv::TrackerNano::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerNano_Params_propTarget_const_int(instance: *mut c_void, val: i32);
// cv::TrackerNano::Params::delete() generated
// ("cv::TrackerNano::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerNano_Params_delete(instance: *mut c_void);
// create(const TrackerVit::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:930
// ("cv::TrackerVit::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerVit::Params*"]), _)]),
pub fn cv_TrackerVit_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerVit::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:930
// ("cv::TrackerVit::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_create(ocvrs_return: *mut Result<*mut c_void>);
// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:934
// ("cv::TrackerVit::getTrackingScore", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_getTrackingScore(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// cv::TrackerVit::to_Tracker() generated
// ("cv::TrackerVit::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerVit::delete() generated
// ("cv::TrackerVit::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:917
// ("cv::TrackerVit::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerVit::Params::implicitClone() generated
// ("cv::TrackerVit::Params::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerVit::Params::net() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:918
// ("cv::TrackerVit::Params::net", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propNet_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerVit::Params::setNet(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:918
// ("cv::TrackerVit::Params::setNet", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerVit_Params_propNet_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerVit::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:919
// ("cv::TrackerVit::Params::backend", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propBackend_const(instance: *const c_void) -> i32;
// cv::TrackerVit::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:919
// ("cv::TrackerVit::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerVit_Params_propBackend_const_int(instance: *mut c_void, val: i32);
// cv::TrackerVit::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:920
// ("cv::TrackerVit::Params::target", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propTarget_const(instance: *const c_void) -> i32;
// cv::TrackerVit::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:920
// ("cv::TrackerVit::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerVit_Params_propTarget_const_int(instance: *mut c_void, val: i32);
// cv::TrackerVit::Params::meanvalue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:921
// ("cv::TrackerVit::Params::meanvalue", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propMeanvalue_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::TrackerVit::Params::setMeanvalue(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:921
// ("cv::TrackerVit::Params::setMeanvalue", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_TrackerVit_Params_propMeanvalue_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::TrackerVit::Params::stdvalue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:922
// ("cv::TrackerVit::Params::stdvalue", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propStdvalue_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::TrackerVit::Params::setStdvalue(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:922
// ("cv::TrackerVit::Params::setStdvalue", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_TrackerVit_Params_propStdvalue_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::TrackerVit::Params::tracking_score_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:923
// ("cv::TrackerVit::Params::tracking_score_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerVit_Params_propTracking_score_threshold_const(instance: *const c_void) -> f32;
// cv::TrackerVit::Params::setTracking_score_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:923
// ("cv::TrackerVit::Params::setTracking_score_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerVit_Params_propTracking_score_threshold_const_float(instance: *mut c_void, val: f32);
// cv::TrackerVit::Params::delete() generated
// ("cv::TrackerVit::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerVit_Params_delete(instance: *mut c_void);
// calcUV(InputArray, InputArray, InputOutputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:528
// ("cv::VariationalRefinement::calcUV", vec![(pred!(mut, ["I0", "I1", "flow_u", "flow_v"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow_u: *const c_void, flow_v: *const c_void, ocvrs_return: *mut Result<()>);
// getFixedPointIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:532
// ("cv::VariationalRefinement::getFixedPointIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getFixedPointIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFixedPointIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:534
// ("cv::VariationalRefinement::setFixedPointIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VariationalRefinement_setFixedPointIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSorIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:539
// ("cv::VariationalRefinement::getSorIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getSorIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSorIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:541
// ("cv::VariationalRefinement::setSorIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VariationalRefinement_setSorIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOmega()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:545
// ("cv::VariationalRefinement::getOmega", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getOmega_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setOmega(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:547
// ("cv::VariationalRefinement::setOmega", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VariationalRefinement_setOmega_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:551
// ("cv::VariationalRefinement::getAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:553
// ("cv::VariationalRefinement::setAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VariationalRefinement_setAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:557
// ("cv::VariationalRefinement::getDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:559
// ("cv::VariationalRefinement::setDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VariationalRefinement_setDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:563
// ("cv::VariationalRefinement::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:565
// ("cv::VariationalRefinement::setGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VariationalRefinement_setGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:569
// ("cv::VariationalRefinement::getEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_VariationalRefinement_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEpsilon(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:571
// ("cv::VariationalRefinement::setEpsilon", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VariationalRefinement_setEpsilon_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:575
// ("cv::VariationalRefinement::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_VariationalRefinement_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::VariationalRefinement::to_Algorithm() generated
// ("cv::VariationalRefinement::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_VariationalRefinement_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::VariationalRefinement::to_DenseOpticalFlow() generated
// ("cv::VariationalRefinement::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_VariationalRefinement_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::VariationalRefinement::delete() generated
// ("cv::VariationalRefinement::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VariationalRefinement_delete(instance: *mut c_void);
