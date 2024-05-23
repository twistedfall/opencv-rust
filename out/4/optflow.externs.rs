// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:119
// ("cv::motempl::calcGlobalOrientation", vec![(pred!(mut, ["orientation", "mask", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "double"]), _)]),
pub fn cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation: *const c_void, mask: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result<f64>);
// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(mhi: *const c_void, mask: *const c_void, orientation: *const c_void, delta1: f64, delta2: f64, ocvrs_return: *mut Result<()>);
// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2", "apertureSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi: *const c_void, mask: *const c_void, orientation: *const c_void, delta1: f64, delta2: f64, aperture_size: i32, ocvrs_return: *mut Result<()>);
// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:137
// ("cv::motempl::segmentMotion", vec![(pred!(mut, ["mhi", "segmask", "boundingRects", "timestamp", "segThresh"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<cv::Rect>*", "double", "double"]), _)]),
pub fn cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vectorLRectGR_double_double(mhi: *const c_void, segmask: *const c_void, bounding_rects: *mut c_void, timestamp: f64, seg_thresh: f64, ocvrs_return: *mut Result<()>);
// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:71
// ("cv::motempl::updateMotionHistory", vec![(pred!(mut, ["silhouette", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double"]), _)]),
pub fn cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result<()>);
// cv::optflow::calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:501
// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray, Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(InputArray, InputArray, InputOutputArray, CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:501
// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow", "rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
pub fn cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(i0: *const c_void, i1: *const c_void, flow: *const c_void, rlof_param: *mut c_void, forward_backward_threshold: f32, grid_step: *const core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:81
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:110
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow", "sigma_dist", "sigma_color", "postprocess_window", "sigma_dist_fix", "sigma_color_fix", "occ_thr", "upscale_averaging_radius", "upscale_sigma_dist", "upscale_sigma_color", "speed_up_thr"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int", "double", "double", "double", "int", "double", "double", "double"]), _)]),
pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64, ocvrs_return: *mut Result<()>);
// cv::optflow::calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:538
// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Ptr<RLOFOpticalFlowParameter>, float)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:538
// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "rlofParam", "forwardBackwardThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_PtrLRLOFOpticalFlowParameterG_float(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, rlof_param: *mut c_void, forward_backward_threshold: f32, ocvrs_return: *mut Result<()>);
// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR(from: *const c_void, to: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow", "grid_step", "k", "sigma", "use_post_proc", "fgs_lambda", "fgs_sigma"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool", "float", "float"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from: *const c_void, to: *const c_void, flow: *const c_void, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, ocvrs_return: *mut Result<()>);
// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:165
// ("cv::optflow::createOptFlow_DeepFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_DeepFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DenseRLOF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:545
// ("cv::optflow::createOptFlow_DenseRLOF", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_DenseRLOF(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:300
// ("cv::optflow::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_DualTVL1(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:171
// ("cv::optflow::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_Farneback(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:142
// ("cv::optflow::createOptFlow_PCAFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_PCAFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:168
// ("cv::optflow::createOptFlow_SimpleFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_SimpleFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_SparseRLOF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:548
// ("cv::optflow::createOptFlow_SparseRLOF", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_SparseRLOF(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:174
// ("cv::optflow::createOptFlow_SparseToDense", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_SparseToDense(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:369
// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
pub fn cv_read_const_FileNodeR_NodeR_Node(fn_: *const c_void, node: *mut crate::optflow::GPCTree_Node, unnamed: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:367
// ("cv::write", vec![(pred!(mut, ["fs", "name", "node"], ["cv::FileStorage*", "const cv::String*", "const cv::optflow::GPCTree::Node*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_NodeR(fs: *mut c_void, name: *const c_char, node: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<()>);
// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:240
// ("cv::optflow::DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:244
// ("cv::optflow::DenseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:253
// ("cv::optflow::DenseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:257
// ("cv::optflow::DenseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getGridStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:263
// ("cv::optflow::DenseRLOFOpticalFlow::getGridStep", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setGridStep(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:267
// ("cv::optflow::DenseRLOFOpticalFlow::setGridStep", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result<()>);
// setInterpolation(InterpolationType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:275
// ("cv::optflow::DenseRLOFOpticalFlow::setInterpolation", vec![(pred!(mut, ["val"], ["cv::optflow::InterpolationType"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(instance: *mut c_void, val: crate::optflow::InterpolationType, ocvrs_return: *mut Result<()>);
// getInterpolation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:279
// ("cv::optflow::DenseRLOFOpticalFlow::getInterpolation", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::InterpolationType>);
// getEPICK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:285
// ("cv::optflow::DenseRLOFOpticalFlow::getEPICK", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setEPICK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:289
// ("cv::optflow::DenseRLOFOpticalFlow::setEPICK", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getEPICSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:296
// ("cv::optflow::DenseRLOFOpticalFlow::getEPICSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEPICSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:300
// ("cv::optflow::DenseRLOFOpticalFlow::setEPICSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getEPICLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:306
// ("cv::optflow::DenseRLOFOpticalFlow::getEPICLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEPICLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:310
// ("cv::optflow::DenseRLOFOpticalFlow::setEPICLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getFgsLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:315
// ("cv::optflow::DenseRLOFOpticalFlow::getFgsLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setFgsLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:319
// ("cv::optflow::DenseRLOFOpticalFlow::setFgsLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getFgsSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:324
// ("cv::optflow::DenseRLOFOpticalFlow::getFgsSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setFgsSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:328
// ("cv::optflow::DenseRLOFOpticalFlow::setFgsSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// setUsePostProc(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:333
// ("cv::optflow::DenseRLOFOpticalFlow::setUsePostProc", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUsePostProc()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:337
// ("cv::optflow::DenseRLOFOpticalFlow::getUsePostProc", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseVariationalRefinement(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:342
// ("cv::optflow::DenseRLOFOpticalFlow::setUseVariationalRefinement", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseVariationalRefinement()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:346
// ("cv::optflow::DenseRLOFOpticalFlow::getUseVariationalRefinement", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setRICSPSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:351
// ("cv::optflow::DenseRLOFOpticalFlow::setRICSPSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRICSPSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:355
// ("cv::optflow::DenseRLOFOpticalFlow::getRICSPSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRICSLICType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:362
// ("cv::optflow::DenseRLOFOpticalFlow::setRICSLICType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRICSLICType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:366
// ("cv::optflow::DenseRLOFOpticalFlow::getRICSLICType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// create(Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:383
// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(rlof_param: *mut c_void, forward_backward_threshold: f32, grid_step: *const core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::DenseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:383
// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::DenseRLOFOpticalFlow::to_Algorithm() generated
// ("cv::optflow::DenseRLOFOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DenseRLOFOpticalFlow::to_DenseOpticalFlow() generated
// ("cv::optflow::DenseRLOFOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DenseRLOFOpticalFlow::delete() generated
// ("cv::optflow::DenseRLOFOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DenseRLOFOpticalFlow_delete(instance: *mut c_void);
// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:223
// ("cv::optflow::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:225
// ("cv::optflow::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:228
// ("cv::optflow::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:230
// ("cv::optflow::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:233
// ("cv::optflow::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:235
// ("cv::optflow::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setTheta_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:238
// ("cv::optflow::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:240
// ("cv::optflow::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:243
// ("cv::optflow::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:245
// ("cv::optflow::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:248
// ("cv::optflow::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:250
// ("cv::optflow::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:253
// ("cv::optflow::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:255
// ("cv::optflow::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:258
// ("cv::optflow::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:260
// ("cv::optflow::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:263
// ("cv::optflow::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:265
// ("cv::optflow::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:268
// ("cv::optflow::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:270
// ("cv::optflow::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:273
// ("cv::optflow::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:275
// ("cv::optflow::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:278
// ("cv::optflow::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:280
// ("cv::optflow::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:283
// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:283
// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::DualTVL1OpticalFlow::to_Algorithm() generated
// ("cv::optflow::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DualTVL1OpticalFlow::to_DenseOpticalFlow() generated
// ("cv::optflow::DualTVL1OpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DualTVL1OpticalFlow::delete() generated
// ("cv::optflow::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DualTVL1OpticalFlow_delete(instance: *mut c_void);
// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:304
// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
pub fn cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(corr: *mut c_void, ocvrs_return: *mut Result<()>);
// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:306
// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
pub fn cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(img_ch: *const c_void, descr: *mut c_void, mp: *const crate::optflow::GPCMatchingParams, typ: i32, ocvrs_return: *mut Result<()>);
// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:309
// ("cv::optflow::GPCDetails::getCoordinatesFromIndex", vec![(pred!(mut, ["index", "sz", "x", "y"], ["size_t", "cv::Size", "int*", "int*"]), _)]),
pub fn cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index: size_t, sz: *const core::Size, x: *mut i32, y: *mut i32, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCDetails::defaultNew() generated
// ("cv::optflow::GPCDetails::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCDetails_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCDetails::delete() generated
// ("cv::optflow::GPCDetails::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCDetails_delete(instance: *mut c_void);
// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:147
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["_useOpenCL"], ["bool"]), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl: bool, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:147
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams(ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:149
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["params"], ["const cv::optflow::GPCMatchingParams*"]), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(params: *const crate::optflow::GPCMatchingParams, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:70
// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
pub fn cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(instance: *const c_void, coef: *const core::VecN<f64, 18>, ocvrs_return: *mut Result<f64>);
// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:72
// ("cv::optflow::GPCPatchDescriptor::markAsSeparated", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_markAsSeparated(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// isSeparated()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:74
// ("cv::optflow::GPCPatchDescriptor::isSeparated", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_isSeparated_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::optflow::GPCPatchDescriptor::defaultNew() generated
// ("cv::optflow::GPCPatchDescriptor::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:68
// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_propFeature_const(instance: *const c_void, ocvrs_return: *mut core::VecN<f64, 18>);
// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:68
// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
pub fn cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(instance: *mut c_void, val: *const core::VecN<f64, 18>);
// cv::optflow::GPCPatchDescriptor::delete() generated
// ("cv::optflow::GPCPatchDescriptor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_delete(instance: *mut c_void);
// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:83
// ("cv::optflow::GPCPatchSample::getDirections", vec![(pred!(const, ["refdir", "posdir", "negdir", "coef", "rhs"], ["bool*", "bool*", "bool*", "const cv::Vec<double, 18>*", "double"]), _)]),
pub fn cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_VecLdouble__18GR_double(instance: *const c_void, refdir: *mut bool, posdir: *mut bool, negdir: *mut bool, coef: *const core::VecN<f64, 18>, rhs: f64, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCPatchSample::defaultNew() generated
// ("cv::optflow::GPCPatchSample::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:79
// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propRef_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:79
// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:80
// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propPos_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:80
// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:81
// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propNeg_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:81
// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::delete() generated
// ("cv::optflow::GPCPatchSample::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_delete(instance: *mut c_void);
// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:130
// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, ["_maxTreeDepth", "_minNumberOfSamples", "_descriptorType", "_printProgress"], ["unsigned int", "int", "cv::optflow::GPCDescType", "bool"]), _)]),
pub fn cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool, ocvrs_return: *mut Result<crate::optflow::GPCTrainingParams>);
// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:130
// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTrainingParams_GPCTrainingParams(ocvrs_return: *mut Result<crate::optflow::GPCTrainingParams>);
// check()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:138
// ("cv::optflow::GPCTrainingParams::check", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingParams_check_const(instance: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result<bool>);
// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:108
// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const std::vector<cv::String>*", "const std::vector<cv::String>*", "const std::vector<cv::String>*", "int"]), _)]),
pub fn cv_optflow_GPCTrainingSamples_create_const_vectorLStringGR_const_vectorLStringGR_const_vectorLStringGR_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:111
// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:114
// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:116
// ("cv::optflow::GPCTrainingSamples::type", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::optflow::GPCTrainingSamples::defaultNew() generated
// ("cv::optflow::GPCTrainingSamples::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCTrainingSamples::delete() generated
// ("cv::optflow::GPCTrainingSamples::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_delete(instance: *mut c_void);
// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:176
// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples", "params"], ["cv::optflow::GPCTrainingSamples*", "const cv::optflow::GPCTrainingParams"]), _)]),
pub fn cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(instance: *mut c_void, samples: *mut c_void, params: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:176
// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
pub fn cv_optflow_GPCTree_train_GPCTrainingSamplesR(instance: *mut c_void, samples: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:178
// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_optflow_GPCTree_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:180
// ("cv::optflow::GPCTree::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_optflow_GPCTree_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:182
// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
pub fn cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(instance: *const c_void, descr: *const c_void, ocvrs_return: *mut Result<u32>);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:184
// ("cv::optflow::GPCTree::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTree_create(ocvrs_return: *mut Result<*mut c_void>);
// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:186
// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
pub fn cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(instance: *const c_void, t: *const c_void, ocvrs_return: *mut Result<bool>);
// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:188
// ("cv::optflow::GPCTree::getDescriptorType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTree_getDescriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::optflow::GPCTree::defaultNew() generated
// ("cv::optflow::GPCTree::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTree_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCTree::to_Algorithm() generated
// ("cv::optflow::GPCTree::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTree_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::GPCTree::delete() generated
// ("cv::optflow::GPCTree::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTree_delete(instance: *mut c_void);
// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:164
// ("cv::optflow::GPCTree::Node::operator==", vec![(pred!(const, ["n"], ["const cv::optflow::GPCTree::Node*"]), _)]),
pub fn cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(instance: *const crate::optflow::GPCTree_Node, n: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<bool>);
// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:116
// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, ["_prior", "_basisSize", "_sparseRate", "_retainedCornersFraction", "_occlusionsThreshold", "_dampingFactor", "_claheClip"], ["cv::Ptr<const cv::optflow::PCAPrior>", "const cv::Size", "float", "float", "float", "float", "float"]), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_PtrLconst_PCAPriorG_const_Size_float_float_float_float_float(_prior: *const c_void, _basis_size: *const core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:116
// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow(ocvrs_return: *mut Result<*mut c_void>);
// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:120
// ("cv::optflow::OpticalFlowPCAFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:121
// ("cv::optflow::OpticalFlowPCAFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::optflow::OpticalFlowPCAFlow::to_Algorithm() generated
// ("cv::optflow::OpticalFlowPCAFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::OpticalFlowPCAFlow::to_DenseOpticalFlow() generated
// ("cv::optflow::OpticalFlowPCAFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::OpticalFlowPCAFlow::delete() generated
// ("cv::optflow::OpticalFlowPCAFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_delete(instance: *mut c_void);
// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:83
// ("cv::optflow::PCAPrior::PCAPrior", vec![(pred!(mut, ["pathToPrior"], ["const char*"]), _)]),
pub fn cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getPadding()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:85
// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_PCAPrior_getPadding_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getBasisSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:87
// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_PCAPrior_getBasisSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:89
// ("cv::optflow::PCAPrior::fillConstraints", vec![(pred!(const, ["A1", "A2", "b1", "b2"], ["float*", "float*", "float*", "float*"]), _)]),
pub fn cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(instance: *const c_void, a1: *mut f32, a2: *mut f32, b1: *mut f32, b2: *mut f32, ocvrs_return: *mut Result<()>);
// cv::optflow::PCAPrior::delete() generated
// ("cv::optflow::PCAPrior::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_PCAPrior_delete(instance: *mut c_void);
// RLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:66
// ("cv::optflow::RLOFOpticalFlowParameter::RLOFOpticalFlowParameter", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(ocvrs_return: *mut Result<*mut c_void>);
// setUseMEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:160
// ("cv::optflow::RLOFOpticalFlowParameter::setUseMEstimator", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// setSolverType(SolverType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:162
// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["cv::optflow::SolverType"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(instance: *mut c_void, val: crate::optflow::SolverType, ocvrs_return: *mut Result<()>);
// getSolverType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:163
// ("cv::optflow::RLOFOpticalFlowParameter::getSolverType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::SolverType>);
// setSupportRegionType(SupportRegionType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:165
// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["cv::optflow::SupportRegionType"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(instance: *mut c_void, val: crate::optflow::SupportRegionType, ocvrs_return: *mut Result<()>);
// getSupportRegionType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:166
// ("cv::optflow::RLOFOpticalFlowParameter::getSupportRegionType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::SupportRegionType>);
// setNormSigma0(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:168
// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getNormSigma0()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:169
// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma0", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setNormSigma1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:171
// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getNormSigma1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:172
// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma1", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSmallWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:174
// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSmallWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:175
// ("cv::optflow::RLOFOpticalFlowParameter::getSmallWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setLargeWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:177
// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getLargeWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:178
// ("cv::optflow::RLOFOpticalFlowParameter::getLargeWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCrossSegmentationThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:180
// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCrossSegmentationThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:181
// ("cv::optflow::RLOFOpticalFlowParameter::getCrossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:183
// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:184
// ("cv::optflow::RLOFOpticalFlowParameter::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:186
// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:187
// ("cv::optflow::RLOFOpticalFlowParameter::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseIlluminationModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:189
// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseIlluminationModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:190
// ("cv::optflow::RLOFOpticalFlowParameter::getUseIlluminationModel", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseGlobalMotionPrior(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:192
// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseGlobalMotionPrior()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:193
// ("cv::optflow::RLOFOpticalFlowParameter::getUseGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setMaxIteration(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:195
// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxIteration()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:196
// ("cv::optflow::RLOFOpticalFlowParameter::getMaxIteration", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinEigenValue(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:198
// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMinEigenValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:199
// ("cv::optflow::RLOFOpticalFlowParameter::getMinEigenValue", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setGlobalMotionRansacThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:201
// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getGlobalMotionRansacThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:202
// ("cv::optflow::RLOFOpticalFlowParameter::getGlobalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:205
// ("cv::optflow::RLOFOpticalFlowParameter::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::RLOFOpticalFlowParameter::solverType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:83
// ("cv::optflow::RLOFOpticalFlowParameter::solverType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSolverType_const(instance: *const c_void, ocvrs_return: *mut crate::optflow::SolverType);
// cv::optflow::RLOFOpticalFlowParameter::setSolverType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:83
// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["const cv::optflow::SolverType"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSolverType_const_SolverType(instance: *mut c_void, val: crate::optflow::SolverType);
// cv::optflow::RLOFOpticalFlowParameter::supportRegionType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:88
// ("cv::optflow::RLOFOpticalFlowParameter::supportRegionType", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const(instance: *const c_void, ocvrs_return: *mut crate::optflow::SupportRegionType);
// cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:88
// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["const cv::optflow::SupportRegionType"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const_SupportRegionType(instance: *mut c_void, val: crate::optflow::SupportRegionType);
// cv::optflow::RLOFOpticalFlowParameter::normSigma0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:92
// ("cv::optflow::RLOFOpticalFlowParameter::normSigma0", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const(instance: *const c_void) -> f32;
// cv::optflow::RLOFOpticalFlowParameter::setNormSigma0(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:92
// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const_float(instance: *mut c_void, val: f32);
// cv::optflow::RLOFOpticalFlowParameter::normSigma1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:98
// ("cv::optflow::RLOFOpticalFlowParameter::normSigma1", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const(instance: *const c_void) -> f32;
// cv::optflow::RLOFOpticalFlowParameter::setNormSigma1(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:98
// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const_float(instance: *mut c_void, val: f32);
// cv::optflow::RLOFOpticalFlowParameter::smallWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:104
// ("cv::optflow::RLOFOpticalFlowParameter::smallWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const(instance: *const c_void) -> i32;
// cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:104
// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const_int(instance: *mut c_void, val: i32);
// cv::optflow::RLOFOpticalFlowParameter::largeWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:107
// ("cv::optflow::RLOFOpticalFlowParameter::largeWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const(instance: *const c_void) -> i32;
// cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:107
// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const_int(instance: *mut c_void, val: i32);
// cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:112
// ("cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const(instance: *const c_void) -> i32;
// cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:112
// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const_int(instance: *mut c_void, val: i32);
// cv::optflow::RLOFOpticalFlowParameter::maxLevel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:117
// ("cv::optflow::RLOFOpticalFlowParameter::maxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const(instance: *const c_void) -> i32;
// cv::optflow::RLOFOpticalFlowParameter::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:117
// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const_int(instance: *mut c_void, val: i32);
// cv::optflow::RLOFOpticalFlowParameter::useInitialFlow() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:122
// ("cv::optflow::RLOFOpticalFlowParameter::useInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const(instance: *const c_void) -> bool;
// cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:122
// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const_bool(instance: *mut c_void, val: bool);
// cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:126
// ("cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const(instance: *const c_void) -> bool;
// cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:126
// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const_bool(instance: *mut c_void, val: bool);
// cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:134
// ("cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const(instance: *const c_void) -> bool;
// cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:134
// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const_bool(instance: *mut c_void, val: bool);
// cv::optflow::RLOFOpticalFlowParameter::maxIteration() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:139
// ("cv::optflow::RLOFOpticalFlowParameter::maxIteration", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const(instance: *const c_void) -> i32;
// cv::optflow::RLOFOpticalFlowParameter::setMaxIteration(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:139
// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const_int(instance: *mut c_void, val: i32);
// cv::optflow::RLOFOpticalFlowParameter::minEigenValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:143
// ("cv::optflow::RLOFOpticalFlowParameter::minEigenValue", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const(instance: *const c_void) -> f32;
// cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:143
// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const_float(instance: *mut c_void, val: f32);
// cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:147
// ("cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const(instance: *const c_void) -> f32;
// cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:147
// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const_float(instance: *mut c_void, val: f32);
// cv::optflow::RLOFOpticalFlowParameter::delete() generated
// ("cv::optflow::RLOFOpticalFlowParameter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_RLOFOpticalFlowParameter_delete(instance: *mut c_void);
// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:417
// ("cv::optflow::SparseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:421
// ("cv::optflow::SparseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:430
// ("cv::optflow::SparseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:434
// ("cv::optflow::SparseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// create(Ptr<RLOFOpticalFlowParameter>, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:441
// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float(rlof_param: *mut c_void, forward_backward_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::SparseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:441
// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::SparseRLOFOpticalFlow::to_Algorithm() generated
// ("cv::optflow::SparseRLOFOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::SparseRLOFOpticalFlow::to_SparseOpticalFlow() generated
// ("cv::optflow::SparseRLOFOpticalFlow::to_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_to_SparseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::SparseRLOFOpticalFlow::delete() generated
// ("cv::optflow::SparseRLOFOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_SparseRLOFOpticalFlow_delete(instance: *mut c_void);
