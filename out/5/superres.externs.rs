// cv::superres::createFrameSource_Camera() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:80
// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createFrameSource_Camera(ocvrs_return: *mut Result<*mut c_void>);
// createFrameSource_Camera(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:80
// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, ["deviceId"], ["int"]), _)]),
pub fn cv_superres_createFrameSource_Camera_int(device_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// createFrameSource_Empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:75
// ("cv::superres::createFrameSource_Empty", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createFrameSource_Empty(ocvrs_return: *mut Result<*mut c_void>);
// createFrameSource_Video_CUDA(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:78
// ("cv::superres::createFrameSource_Video_CUDA", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_superres_createFrameSource_Video_CUDA_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createFrameSource_Video(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:77
// ("cv::superres::createFrameSource_Video", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_superres_createFrameSource_Video_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_Brox_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:177
// ("cv::superres::createOptFlow_Brox_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_Brox_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:139
// ("cv::superres::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_DualTVL1(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DualTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:140
// ("cv::superres::createOptFlow_DualTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_DualTVL1_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:96
// ("cv::superres::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_Farneback(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_Farneback_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:97
// ("cv::superres::createOptFlow_Farneback_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_Farneback_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_PyrLK_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:196
// ("cv::superres::createOptFlow_PyrLK_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createOptFlow_PyrLK_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// createSuperResolution_BTVL1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:199
// ("cv::superres::createSuperResolution_BTVL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createSuperResolution_BTVL1(ocvrs_return: *mut Result<*mut c_void>);
// createSuperResolution_BTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:200
// ("cv::superres::createSuperResolution_BTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_createSuperResolution_BTVL1_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// getAlpha()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:148
// ("cv::superres::BroxOpticalFlow::getAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:150
// ("cv::superres::BroxOpticalFlow::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setAlpha_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:153
// ("cv::superres::BroxOpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:155
// ("cv::superres::BroxOpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:158
// ("cv::superres::BroxOpticalFlow::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:160
// ("cv::superres::BroxOpticalFlow::setScaleFactor", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setScaleFactor_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:163
// ("cv::superres::BroxOpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:165
// ("cv::superres::BroxOpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setInnerIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:168
// ("cv::superres::BroxOpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:170
// ("cv::superres::BroxOpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setOuterIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSolverIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:173
// ("cv::superres::BroxOpticalFlow::getSolverIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_getSolverIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSolverIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:175
// ("cv::superres::BroxOpticalFlow::setSolverIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_BroxOpticalFlow_setSolverIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::superres::BroxOpticalFlow::to_Algorithm() generated
// ("cv::superres::BroxOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::BroxOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
// ("cv::superres::BroxOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_to_SuperRes_DenseOpticalFlowExt(instance: *mut c_void) -> *mut c_void;
// cv::superres::BroxOpticalFlow::delete() generated
// ("cv::superres::BroxOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_BroxOpticalFlow_delete(instance: *mut c_void);
// calc(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:59
// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1", "flow2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow1: *const c_void, flow2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::superres::DenseOpticalFlowExt::calc(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:59
// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow1: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:60
// ("cv::superres::DenseOpticalFlowExt::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::superres::DenseOpticalFlowExt::to_SuperRes_BroxOpticalFlow() generated
// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_BroxOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_to_SuperRes_BroxOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::superres::DenseOpticalFlowExt::to_SuperRes_DualTVL1OpticalFlow() generated
// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_DualTVL1OpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_to_SuperRes_DualTVL1OpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::superres::DenseOpticalFlowExt::to_SuperRes_FarnebackOpticalFlow() generated
// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_to_SuperRes_FarnebackOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::superres::DenseOpticalFlowExt::to_SuperRes_PyrLKOpticalFlow() generated
// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_PyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_to_SuperRes_PyrLKOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::superres::DenseOpticalFlowExt::to_Algorithm() generated
// ("cv::superres::DenseOpticalFlowExt::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::DenseOpticalFlowExt::delete() generated
// ("cv::superres::DenseOpticalFlowExt::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DenseOpticalFlowExt_delete(instance: *mut c_void);
// getTau()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:107
// ("cv::superres::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:109
// ("cv::superres::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:111
// ("cv::superres::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:113
// ("cv::superres::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:115
// ("cv::superres::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:117
// ("cv::superres::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setTheta_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:119
// ("cv::superres::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:121
// ("cv::superres::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:123
// ("cv::superres::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:125
// ("cv::superres::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getEpsilon()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:127
// ("cv::superres::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:129
// ("cv::superres::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setEpsilon_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:131
// ("cv::superres::DualTVL1OpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:133
// ("cv::superres::DualTVL1OpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:135
// ("cv::superres::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:137
// ("cv::superres::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// cv::superres::DualTVL1OpticalFlow::to_Algorithm() generated
// ("cv::superres::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::DualTVL1OpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
// ("cv::superres::DualTVL1OpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_to_SuperRes_DenseOpticalFlowExt(instance: *mut c_void) -> *mut c_void;
// cv::superres::DualTVL1OpticalFlow::delete() generated
// ("cv::superres::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_DualTVL1OpticalFlow_delete(instance: *mut c_void);
// getPyrScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:68
// ("cv::superres::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:70
// ("cv::superres::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getLevelsNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:72
// ("cv::superres::FarnebackOpticalFlow::getLevelsNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setLevelsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:74
// ("cv::superres::FarnebackOpticalFlow::setLevelsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:76
// ("cv::superres::FarnebackOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:78
// ("cv::superres::FarnebackOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:80
// ("cv::superres::FarnebackOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:82
// ("cv::superres::FarnebackOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPolyN()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:84
// ("cv::superres::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:86
// ("cv::superres::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPolySigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:88
// ("cv::superres::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:90
// ("cv::superres::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:92
// ("cv::superres::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:94
// ("cv::superres::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::superres::FarnebackOpticalFlow::to_Algorithm() generated
// ("cv::superres::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::FarnebackOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
// ("cv::superres::FarnebackOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_to_SuperRes_DenseOpticalFlowExt(instance: *mut c_void) -> *mut c_void;
// cv::superres::FarnebackOpticalFlow::delete() generated
// ("cv::superres::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FarnebackOpticalFlow_delete(instance: *mut c_void);
// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:71
// ("cv::superres::FrameSource::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_superres_FrameSource_nextFrame_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:72
// ("cv::superres::FrameSource::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::superres::FrameSource::to_SuperRes_SuperResolution() generated
// ("cv::superres::FrameSource::to_SuperRes_SuperResolution", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FrameSource_to_SuperRes_SuperResolution(instance: *mut c_void) -> *mut c_void;
// cv::superres::FrameSource::delete() generated
// ("cv::superres::FrameSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_FrameSource_delete(instance: *mut c_void);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:184
// ("cv::superres::PyrLKOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:186
// ("cv::superres::PyrLKOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_PyrLKOpticalFlow_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:188
// ("cv::superres::PyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:190
// ("cv::superres::PyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_PyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:192
// ("cv::superres::PyrLKOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:194
// ("cv::superres::PyrLKOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_PyrLKOpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::superres::PyrLKOpticalFlow::to_Algorithm() generated
// ("cv::superres::PyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::PyrLKOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
// ("cv::superres::PyrLKOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_to_SuperRes_DenseOpticalFlowExt(instance: *mut c_void) -> *mut c_void;
// cv::superres::PyrLKOpticalFlow::delete() generated
// ("cv::superres::PyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_PyrLKOpticalFlow_delete(instance: *mut c_void);
// setInput(const Ptr<FrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:94
// ("cv::superres::SuperResolution::setInput", vec![(pred!(mut, ["frameSource"], ["const cv::Ptr<cv::superres::FrameSource>*"]), _)]),
pub fn cv_superres_SuperResolution_setInput_const_PtrLFrameSourceGR(instance: *mut c_void, frame_source: *const c_void, ocvrs_return: *mut Result<()>);
// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:100
// ("cv::superres::SuperResolution::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_superres_SuperResolution_nextFrame_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:101
// ("cv::superres::SuperResolution::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_SuperResolution_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:105
// ("cv::superres::SuperResolution::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_SuperResolution_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:109
// ("cv::superres::SuperResolution::getScale", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:111
// ("cv::superres::SuperResolution::setScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_SuperResolution_setScale_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:115
// ("cv::superres::SuperResolution::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:117
// ("cv::superres::SuperResolution::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_SuperResolution_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getTau()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:121
// ("cv::superres::SuperResolution::getTau", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:123
// ("cv::superres::SuperResolution::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_SuperResolution_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:127
// ("cv::superres::SuperResolution::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:129
// ("cv::superres::SuperResolution::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_SuperResolution_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAlpha()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:133
// ("cv::superres::SuperResolution::getAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:135
// ("cv::superres::SuperResolution::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_SuperResolution_setAlpha_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getKernelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:139
// ("cv::superres::SuperResolution::getKernelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:141
// ("cv::superres::SuperResolution::setKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_SuperResolution_setKernelSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getBlurKernelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:145
// ("cv::superres::SuperResolution::getBlurKernelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getBlurKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBlurKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:147
// ("cv::superres::SuperResolution::setBlurKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_SuperResolution_setBlurKernelSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getBlurSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:151
// ("cv::superres::SuperResolution::getBlurSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getBlurSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBlurSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:153
// ("cv::superres::SuperResolution::setBlurSigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_superres_SuperResolution_setBlurSigma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTemporalAreaRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:157
// ("cv::superres::SuperResolution::getTemporalAreaRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getTemporalAreaRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTemporalAreaRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:159
// ("cv::superres::SuperResolution::setTemporalAreaRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_superres_SuperResolution_setTemporalAreaRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOpticalFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:163
// ("cv::superres::SuperResolution::getOpticalFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_superres_SuperResolution_getOpticalFlow_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:165
// ("cv::superres::SuperResolution::setOpticalFlow", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::superres::DenseOpticalFlowExt>*"]), _)]),
pub fn cv_superres_SuperResolution_setOpticalFlow_const_PtrLDenseOpticalFlowExtGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// cv::superres::SuperResolution::to_Algorithm() generated
// ("cv::superres::SuperResolution::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_SuperResolution_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::superres::SuperResolution::to_SuperRes_FrameSource() generated
// ("cv::superres::SuperResolution::to_SuperRes_FrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_SuperResolution_to_SuperRes_FrameSource(instance: *mut c_void) -> *mut c_void;
// cv::superres::SuperResolution::delete() generated
// ("cv::superres::SuperResolution::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_superres_SuperResolution_delete(instance: *mut c_void);
