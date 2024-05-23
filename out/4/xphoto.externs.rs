// applyChannelGains(InputArray, OutputArray, float, float, float)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:225
// ("cv::xphoto::applyChannelGains", vec![(pred!(mut, ["src", "dst", "gainB", "gainG", "gainR"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
pub fn cv_xphoto_applyChannelGains_const__InputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, dst: *const c_void, gain_b: f32, gain_g: f32, gain_r: f32, ocvrs_return: *mut Result<()>);
// cv::xphoto::bm3dDenoising(InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/bm3d_image_denoising.hpp:115
// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dstStep1", "dstStep2"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(src: *const c_void, dst_step1: *const c_void, dst_step2: *const c_void, ocvrs_return: *mut Result<()>);
// bm3dDenoising(InputArray, InputOutputArray, OutputArray, float, int, int, int, int, int, int, float, int, int, int)(InputArray, InputOutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/bm3d_image_denoising.hpp:115
// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dstStep1", "dstStep2", "h", "templateWindowSize", "searchWindowSize", "blockMatchingStep1", "blockMatchingStep2", "groupSize", "slidingStep", "beta", "normType", "step", "transformType"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "int", "int", "int", "float", "int", "int", "int"]), _)]),
pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src: *const c_void, dst_step1: *const c_void, dst_step2: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32, ocvrs_return: *mut Result<()>);
// cv::xphoto::bm3dDenoising(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/bm3d_image_denoising.hpp:168
// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bm3dDenoising(InputArray, OutputArray, float, int, int, int, int, int, int, float, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/bm3d_image_denoising.hpp:168
// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize", "blockMatchingStep1", "blockMatchingStep2", "groupSize", "slidingStep", "beta", "normType", "step", "transformType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "int", "int", "int", "float", "int", "int", "int"]), _)]),
pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src: *const c_void, dst: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32, ocvrs_return: *mut Result<()>);
// createGrayworldWB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:152
// ("cv::xphoto::createGrayworldWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_createGrayworldWB(ocvrs_return: *mut Result<*mut c_void>);
// cv::xphoto::createLearningBasedWB() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:214
// ("cv::xphoto::createLearningBasedWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_createLearningBasedWB(ocvrs_return: *mut Result<*mut c_void>);
// createLearningBasedWB(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:214
// ("cv::xphoto::createLearningBasedWB", vec![(pred!(mut, ["path_to_model"], ["const cv::String*"]), _)]),
pub fn cv_xphoto_createLearningBasedWB_const_StringR(path_to_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createSimpleWB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:115
// ("cv::xphoto::createSimpleWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_createSimpleWB(ocvrs_return: *mut Result<*mut c_void>);
// cv::xphoto::createTonemapDurand() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:53
// ("cv::xphoto::createTonemapDurand", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_createTonemapDurand(ocvrs_return: *mut Result<*mut c_void>);
// createTonemapDurand(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:53
// ("cv::xphoto::createTonemapDurand", vec![(pred!(mut, ["gamma", "contrast", "saturation", "sigma_color", "sigma_space"], ["float", "float", "float", "float", "float"]), _)]),
pub fn cv_xphoto_createTonemapDurand_float_float_float_float_float(gamma: f32, contrast: f32, saturation: f32, sigma_color: f32, sigma_space: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xphoto::dctDenoising(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/dct_image_denoising.hpp:72
// ("cv::xphoto::dctDenoising", vec![(pred!(mut, ["src", "dst", "sigma"], ["const cv::Mat*", "cv::Mat*", "const double"]), _)]),
pub fn cv_xphoto_dctDenoising_const_MatR_MatR_const_double(src: *const c_void, dst: *mut c_void, sigma: f64, ocvrs_return: *mut Result<()>);
// dctDenoising(const Mat &, Mat &, const double, const int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/dct_image_denoising.hpp:72
// ("cv::xphoto::dctDenoising", vec![(pred!(mut, ["src", "dst", "sigma", "psize"], ["const cv::Mat*", "cv::Mat*", "const double", "const int"]), _)]),
pub fn cv_xphoto_dctDenoising_const_MatR_MatR_const_double_const_int(src: *const c_void, dst: *mut c_void, sigma: f64, psize: i32, ocvrs_return: *mut Result<()>);
// inpaint(const Mat &, const Mat &, Mat &, const int)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/inpainting.hpp:113
// ("cv::xphoto::inpaint", vec![(pred!(mut, ["src", "mask", "dst", "algorithmType"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*", "const int"]), _)]),
pub fn cv_xphoto_inpaint_const_MatR_const_MatR_MatR_const_int(src: *const c_void, mask: *const c_void, dst: *mut c_void, algorithm_type: i32, ocvrs_return: *mut Result<()>);
// oilPainting(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/oilpainting.hpp:36
// ("cv::xphoto::oilPainting", vec![(pred!(mut, ["src", "dst", "size", "dynRatio"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, size: i32, dyn_ratio: i32, ocvrs_return: *mut Result<()>);
// oilPainting(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/oilpainting.hpp:28
// ("cv::xphoto::oilPainting", vec![(pred!(mut, ["src", "dst", "size", "dynRatio", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, size: i32, dyn_ratio: i32, code: i32, ocvrs_return: *mut Result<()>);
// getSaturationThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:145
// ("cv::xphoto::GrayworldWB::getSaturationThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_GrayworldWB_getSaturationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturationThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:147
// ("cv::xphoto::GrayworldWB::setSaturationThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_GrayworldWB_setSaturationThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// cv::xphoto::GrayworldWB::to_Algorithm() generated
// ("cv::xphoto::GrayworldWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_GrayworldWB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::GrayworldWB::to_WhiteBalancer() generated
// ("cv::xphoto::GrayworldWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_GrayworldWB_to_WhiteBalancer(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::GrayworldWB::delete() generated
// ("cv::xphoto::GrayworldWB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_GrayworldWB_delete(instance: *mut c_void);
// extractSimpleFeatures(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:185
// ("cv::xphoto::LearningBasedWB::extractSimpleFeatures", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getRangeMaxVal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:190
// ("cv::xphoto::LearningBasedWB::getRangeMaxVal", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_getRangeMaxVal_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRangeMaxVal(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:192
// ("cv::xphoto::LearningBasedWB::setRangeMaxVal", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_xphoto_LearningBasedWB_setRangeMaxVal_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSaturationThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:197
// ("cv::xphoto::LearningBasedWB::getSaturationThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_getSaturationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturationThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:199
// ("cv::xphoto::LearningBasedWB::setSaturationThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_LearningBasedWB_setSaturationThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getHistBinNum()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:205
// ("cv::xphoto::LearningBasedWB::getHistBinNum", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_getHistBinNum_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistBinNum(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:207
// ("cv::xphoto::LearningBasedWB::setHistBinNum", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_xphoto_LearningBasedWB_setHistBinNum_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::xphoto::LearningBasedWB::to_Algorithm() generated
// ("cv::xphoto::LearningBasedWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::LearningBasedWB::to_WhiteBalancer() generated
// ("cv::xphoto::LearningBasedWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_to_WhiteBalancer(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::LearningBasedWB::delete() generated
// ("cv::xphoto::LearningBasedWB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_LearningBasedWB_delete(instance: *mut c_void);
// getInputMin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:84
// ("cv::xphoto::SimpleWB::getInputMin", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_SimpleWB_getInputMin_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setInputMin(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:86
// ("cv::xphoto::SimpleWB::setInputMin", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_SimpleWB_setInputMin_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getInputMax()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:90
// ("cv::xphoto::SimpleWB::getInputMax", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_SimpleWB_getInputMax_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setInputMax(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:92
// ("cv::xphoto::SimpleWB::setInputMax", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_SimpleWB_setInputMax_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getOutputMin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:96
// ("cv::xphoto::SimpleWB::getOutputMin", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_SimpleWB_getOutputMin_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setOutputMin(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:98
// ("cv::xphoto::SimpleWB::setOutputMin", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_SimpleWB_setOutputMin_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getOutputMax()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:102
// ("cv::xphoto::SimpleWB::getOutputMax", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_SimpleWB_getOutputMax_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setOutputMax(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:104
// ("cv::xphoto::SimpleWB::setOutputMax", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_SimpleWB_setOutputMax_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getP()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:108
// ("cv::xphoto::SimpleWB::getP", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_SimpleWB_getP_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setP(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:110
// ("cv::xphoto::SimpleWB::setP", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_xphoto_SimpleWB_setP_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// cv::xphoto::SimpleWB::to_Algorithm() generated
// ("cv::xphoto::SimpleWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_SimpleWB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::SimpleWB::to_WhiteBalancer() generated
// ("cv::xphoto::SimpleWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_SimpleWB_to_WhiteBalancer(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::SimpleWB::delete() generated
// ("cv::xphoto::SimpleWB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_SimpleWB_delete(instance: *mut c_void);
// getSaturation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:28
// ("cv::xphoto::TonemapDurand::getSaturation", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:29
// ("cv::xphoto::TonemapDurand::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
pub fn cv_xphoto_TonemapDurand_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result<()>);
// getContrast()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:31
// ("cv::xphoto::TonemapDurand::getContrast", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_getContrast_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:32
// ("cv::xphoto::TonemapDurand::setContrast", vec![(pred!(mut, ["contrast"], ["float"]), _)]),
pub fn cv_xphoto_TonemapDurand_setContrast_float(instance: *mut c_void, contrast: f32, ocvrs_return: *mut Result<()>);
// getSigmaSpace()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:34
// ("cv::xphoto::TonemapDurand::getSigmaSpace", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_getSigmaSpace_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSigmaSpace(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:35
// ("cv::xphoto::TonemapDurand::setSigmaSpace", vec![(pred!(mut, ["sigma_space"], ["float"]), _)]),
pub fn cv_xphoto_TonemapDurand_setSigmaSpace_float(instance: *mut c_void, sigma_space: f32, ocvrs_return: *mut Result<()>);
// getSigmaColor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:37
// ("cv::xphoto::TonemapDurand::getSigmaColor", vec![(pred!(const, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_getSigmaColor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSigmaColor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/tonemap.hpp:38
// ("cv::xphoto::TonemapDurand::setSigmaColor", vec![(pred!(mut, ["sigma_color"], ["float"]), _)]),
pub fn cv_xphoto_TonemapDurand_setSigmaColor_float(instance: *mut c_void, sigma_color: f32, ocvrs_return: *mut Result<()>);
// cv::xphoto::TonemapDurand::to_Algorithm() generated
// ("cv::xphoto::TonemapDurand::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::TonemapDurand::to_Tonemap() generated
// ("cv::xphoto::TonemapDurand::to_Tonemap", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_to_Tonemap(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::TonemapDurand::delete() generated
// ("cv::xphoto::TonemapDurand::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_TonemapDurand_delete(instance: *mut c_void);
// balanceWhite(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xphoto/white_balance.hpp:72
// ("cv::xphoto::WhiteBalancer::balanceWhite", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::xphoto::WhiteBalancer::to_GrayworldWB() generated
// ("cv::xphoto::WhiteBalancer::to_GrayworldWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_WhiteBalancer_to_GrayworldWB(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::WhiteBalancer::to_LearningBasedWB() generated
// ("cv::xphoto::WhiteBalancer::to_LearningBasedWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_WhiteBalancer_to_LearningBasedWB(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::WhiteBalancer::to_SimpleWB() generated
// ("cv::xphoto::WhiteBalancer::to_SimpleWB", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_WhiteBalancer_to_SimpleWB(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::WhiteBalancer::to_Algorithm() generated
// ("cv::xphoto::WhiteBalancer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_WhiteBalancer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xphoto::WhiteBalancer::delete() generated
// ("cv::xphoto::WhiteBalancer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xphoto_WhiteBalancer_delete(instance: *mut c_void);
