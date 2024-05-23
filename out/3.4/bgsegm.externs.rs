// cv::bgsegm::createBackgroundSubtractorCNT() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:240
// ("cv::bgsegm::createBackgroundSubtractorCNT", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorCNT(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorCNT(int, bool, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:240
// ("cv::bgsegm::createBackgroundSubtractorCNT", vec![(pred!(mut, ["minPixelStability", "useHistory", "maxPixelStability", "isParallel"], ["int", "bool", "int", "bool"]), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(min_pixel_stability: i32, use_history: bool, max_pixel_stability: i32, is_parallel: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::bgsegm::createBackgroundSubtractorGMG() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:185
// ("cv::bgsegm::createBackgroundSubtractorGMG", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorGMG(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorGMG(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:185
// ("cv::bgsegm::createBackgroundSubtractorGMG", vec![(pred!(mut, ["initializationFrames", "decisionThreshold"], ["int", "double"]), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorGMG_int_double(initialization_frames: i32, decision_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::bgsegm::createBackgroundSubtractorGSOC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:302
// ("cv::bgsegm::createBackgroundSubtractorGSOC", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorGSOC(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorGSOC(int, int, float, float, int, float, float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:302
// ("cv::bgsegm::createBackgroundSubtractorGSOC", vec![(pred!(mut, ["mc", "nSamples", "replaceRate", "propagationRate", "hitsThreshold", "alpha", "beta", "blinkingSupressionDecay", "blinkingSupressionMultiplier", "noiseRemovalThresholdFacBG", "noiseRemovalThresholdFacFG"], ["int", "int", "float", "float", "int", "float", "float", "float", "float", "float", "float"]), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(mc: i32, n_samples: i32, replace_rate: f32, propagation_rate: f32, hits_threshold: i32, alpha: f32, beta: f32, blinking_supression_decay: f32, blinking_supression_multiplier: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::bgsegm::createBackgroundSubtractorLSBP() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:322
// ("cv::bgsegm::createBackgroundSubtractorLSBP", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorLSBP(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorLSBP(int, int, int, float, float, float, float, float, float, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:322
// ("cv::bgsegm::createBackgroundSubtractorLSBP", vec![(pred!(mut, ["mc", "nSamples", "LSBPRadius", "Tlower", "Tupper", "Tinc", "Tdec", "Rscale", "Rincdec", "noiseRemovalThresholdFacBG", "noiseRemovalThresholdFacFG", "LSBPthreshold", "minCount"], ["int", "int", "int", "float", "float", "float", "float", "float", "float", "float", "float", "int", "int"]), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(mc: i32, n_samples: i32, lsbp_radius: i32, tlower: f32, tupper: f32, tinc: f32, tdec: f32, rscale: f32, rincdec: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32, lsb_pthreshold: i32, min_count: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::bgsegm::createBackgroundSubtractorMOG() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:87
// ("cv::bgsegm::createBackgroundSubtractorMOG", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorMOG(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorMOG(int, int, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:87
// ("cv::bgsegm::createBackgroundSubtractorMOG", vec![(pred!(mut, ["history", "nmixtures", "backgroundRatio", "noiseSigma"], ["int", "int", "double", "double"]), _)]),
pub fn cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::bgsegm::createSyntheticSequenceGenerator(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:372
// ("cv::bgsegm::createSyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR(background: *const c_void, object: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSyntheticSequenceGenerator(InputArray, InputArray, double, double, double, double)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:372
// ("cv::bgsegm::createSyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object", "amplitude", "wavelength", "wavespeed", "objspeed"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "double", "double", "double"]), _)]),
pub fn cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(background: *const c_void, object: *const c_void, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64, ocvrs_return: *mut Result<*mut c_void>);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:199
// ("cv::bgsegm::BackgroundSubtractorCNT::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorCNT::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:199
// ("cv::bgsegm::BackgroundSubtractorCNT::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:200
// ("cv::bgsegm::BackgroundSubtractorCNT::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result<()>);
// getMinPixelStability()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:204
// ("cv::bgsegm::BackgroundSubtractorCNT::getMinPixelStability", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinPixelStability(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:207
// ("cv::bgsegm::BackgroundSubtractorCNT::setMinPixelStability", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getMaxPixelStability()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:211
// ("cv::bgsegm::BackgroundSubtractorCNT::getMaxPixelStability", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxPixelStability(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:214
// ("cv::bgsegm::BackgroundSubtractorCNT::setMaxPixelStability", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getUseHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:218
// ("cv::bgsegm::BackgroundSubtractorCNT::getUseHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseHistory(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:221
// ("cv::bgsegm::BackgroundSubtractorCNT::setUseHistory", vec![(pred!(mut, ["value"], ["bool"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result<()>);
// getIsParallel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:225
// ("cv::bgsegm::BackgroundSubtractorCNT::getIsParallel", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setIsParallel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:228
// ("cv::bgsegm::BackgroundSubtractorCNT::setIsParallel", vec![(pred!(mut, ["value"], ["bool"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorCNT::to_Algorithm() generated
// ("cv::bgsegm::BackgroundSubtractorCNT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorCNT::to_BackgroundSubtractor() generated
// ("cv::bgsegm::BackgroundSubtractorCNT::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorCNT::delete() generated
// ("cv::bgsegm::BackgroundSubtractorCNT::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorCNT_delete(instance: *mut c_void);
// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:104
// ("cv::bgsegm::BackgroundSubtractorGMG::getMaxFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:107
// ("cv::bgsegm::BackgroundSubtractorGMG::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getDefaultLearningRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:114
// ("cv::bgsegm::BackgroundSubtractorGMG::getDefaultLearningRate", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDefaultLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:117
// ("cv::bgsegm::BackgroundSubtractorGMG::setDefaultLearningRate", vec![(pred!(mut, ["lr"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(instance: *mut c_void, lr: f64, ocvrs_return: *mut Result<()>);
// getNumFrames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:121
// ("cv::bgsegm::BackgroundSubtractorGMG::getNumFrames", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumFrames(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:124
// ("cv::bgsegm::BackgroundSubtractorGMG::setNumFrames", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result<()>);
// getQuantizationLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:130
// ("cv::bgsegm::BackgroundSubtractorGMG::getQuantizationLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setQuantizationLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:133
// ("cv::bgsegm::BackgroundSubtractorGMG::setQuantizationLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result<()>);
// getBackgroundPrior()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:137
// ("cv::bgsegm::BackgroundSubtractorGMG::getBackgroundPrior", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundPrior(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:140
// ("cv::bgsegm::BackgroundSubtractorGMG::setBackgroundPrior", vec![(pred!(mut, ["bgprior"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(instance: *mut c_void, bgprior: f64, ocvrs_return: *mut Result<()>);
// getSmoothingRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:144
// ("cv::bgsegm::BackgroundSubtractorGMG::getSmoothingRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSmoothingRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:147
// ("cv::bgsegm::BackgroundSubtractorGMG::setSmoothingRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result<()>);
// getDecisionThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:153
// ("cv::bgsegm::BackgroundSubtractorGMG::getDecisionThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDecisionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:156
// ("cv::bgsegm::BackgroundSubtractorGMG::setDecisionThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result<()>);
// getUpdateBackgroundModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:160
// ("cv::bgsegm::BackgroundSubtractorGMG::getUpdateBackgroundModel", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUpdateBackgroundModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:163
// ("cv::bgsegm::BackgroundSubtractorGMG::setUpdateBackgroundModel", vec![(pred!(mut, ["update"], ["bool"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(instance: *mut c_void, update: bool, ocvrs_return: *mut Result<()>);
// getMinVal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:167
// ("cv::bgsegm::BackgroundSubtractorGMG::getMinVal", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:170
// ("cv::bgsegm::BackgroundSubtractorGMG::setMinVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxVal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:174
// ("cv::bgsegm::BackgroundSubtractorGMG::getMaxVal", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:177
// ("cv::bgsegm::BackgroundSubtractorGMG::setMaxVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorGMG::to_Algorithm() generated
// ("cv::bgsegm::BackgroundSubtractorGMG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorGMG::to_BackgroundSubtractor() generated
// ("cv::bgsegm::BackgroundSubtractorGMG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorGMG::delete() generated
// ("cv::bgsegm::BackgroundSubtractorGMG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGMG_delete(instance: *mut c_void);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:258
// ("cv::bgsegm::BackgroundSubtractorGSOC::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorGSOC::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:258
// ("cv::bgsegm::BackgroundSubtractorGSOC::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:260
// ("cv::bgsegm::BackgroundSubtractorGSOC::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorGSOC::to_Algorithm() generated
// ("cv::bgsegm::BackgroundSubtractorGSOC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorGSOC::to_BackgroundSubtractor() generated
// ("cv::bgsegm::BackgroundSubtractorGSOC::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorGSOC::delete() generated
// ("cv::bgsegm::BackgroundSubtractorGSOC::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorGSOC_delete(instance: *mut c_void);
// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:269
// ("cv::bgsegm::BackgroundSubtractorLSBP::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorLSBP::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:269
// ("cv::bgsegm::BackgroundSubtractorLSBP::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:271
// ("cv::bgsegm::BackgroundSubtractorLSBP::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorLSBP::to_Algorithm() generated
// ("cv::bgsegm::BackgroundSubtractorLSBP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorLSBP::to_BackgroundSubtractor() generated
// ("cv::bgsegm::BackgroundSubtractorLSBP::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorLSBP::delete() generated
// ("cv::bgsegm::BackgroundSubtractorLSBP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBP_delete(instance: *mut c_void);
// calcLocalSVDValues(OutputArray, const Mat &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:279
// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues", vec![(pred!(mut, ["localSVDValues", "frame"], ["const cv::_OutputArray*", "const cv::Mat*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayR_const_MatR(local_svd_values: *const c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// computeFromLocalSVDValues(OutputArray, const Mat &, const Point2i *)(OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:281
// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues", vec![(pred!(mut, ["desc", "localSVDValues", "LSBPSamplePoints"], ["const cv::_OutputArray*", "const cv::Mat*", "const cv::Point2i*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayR_const_MatR_const_Point2iX(desc: *const c_void, local_svd_values: *const c_void, lsbp_sample_points: *const core::Point2i, ocvrs_return: *mut Result<()>);
// compute(OutputArray, const Mat &, const Point2i *)(OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:283
// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::compute", vec![(pred!(mut, ["desc", "frame", "LSBPSamplePoints"], ["const cv::_OutputArray*", "const cv::Mat*", "const cv::Point2i*"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayR_const_MatR_const_Point2iX(desc: *const c_void, frame: *const c_void, lsbp_sample_points: *const core::Point2i, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorLSBPDesc::defaultNew() generated
// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_defaultNew_const() -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorLSBPDesc::delete() generated
// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_delete(instance: *mut c_void);
// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:65
// ("cv::bgsegm::BackgroundSubtractorMOG::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:66
// ("cv::bgsegm::BackgroundSubtractorMOG::setHistory", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result<()>);
// getNMixtures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:68
// ("cv::bgsegm::BackgroundSubtractorMOG::getNMixtures", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:69
// ("cv::bgsegm::BackgroundSubtractorMOG::setNMixtures", vec![(pred!(mut, ["nmix"], ["int"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(instance: *mut c_void, nmix: i32, ocvrs_return: *mut Result<()>);
// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:71
// ("cv::bgsegm::BackgroundSubtractorMOG::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:72
// ("cv::bgsegm::BackgroundSubtractorMOG::setBackgroundRatio", vec![(pred!(mut, ["backgroundRatio"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(instance: *mut c_void, background_ratio: f64, ocvrs_return: *mut Result<()>);
// getNoiseSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:74
// ("cv::bgsegm::BackgroundSubtractorMOG::getNoiseSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNoiseSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:75
// ("cv::bgsegm::BackgroundSubtractorMOG::setNoiseSigma", vec![(pred!(mut, ["noiseSigma"], ["double"]), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(instance: *mut c_void, noise_sigma: f64, ocvrs_return: *mut Result<()>);
// cv::bgsegm::BackgroundSubtractorMOG::to_Algorithm() generated
// ("cv::bgsegm::BackgroundSubtractorMOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorMOG::to_BackgroundSubtractor() generated
// ("cv::bgsegm::BackgroundSubtractorMOG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::BackgroundSubtractorMOG::delete() generated
// ("cv::bgsegm::BackgroundSubtractorMOG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_BackgroundSubtractorMOG_delete(instance: *mut c_void);
// SyntheticSequenceGenerator(InputArray, InputArray, double, double, double, double)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:353
// ("cv::bgsegm::SyntheticSequenceGenerator::SyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object", "amplitude", "wavelength", "wavespeed", "objspeed"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "double", "double", "double"]), _)]),
pub fn cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(background: *const c_void, object: *const c_void, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64, ocvrs_return: *mut Result<*mut c_void>);
// getNextFrame(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/bgsegm.hpp:360
// ("cv::bgsegm::SyntheticSequenceGenerator::getNextFrame", vec![(pred!(mut, ["frame", "gtMask"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, gt_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bgsegm::SyntheticSequenceGenerator::to_Algorithm() generated
// ("cv::bgsegm::SyntheticSequenceGenerator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_SyntheticSequenceGenerator_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bgsegm::SyntheticSequenceGenerator::delete() generated
// ("cv::bgsegm::SyntheticSequenceGenerator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bgsegm_SyntheticSequenceGenerator_delete(instance: *mut c_void);
