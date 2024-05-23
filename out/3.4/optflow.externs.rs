// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:119
// ("cv::motempl::calcGlobalOrientation", vec![(pred!(mut, ["orientation", "mask", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "double"]), _)]),
pub fn cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation: *const c_void, mask: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result<f64>);
// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(mhi: *const c_void, mask: *const c_void, orientation: *const c_void, delta1: f64, delta2: f64, ocvrs_return: *mut Result<()>);
// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2", "apertureSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi: *const c_void, mask: *const c_void, orientation: *const c_void, delta1: f64, delta2: f64, aperture_size: i32, ocvrs_return: *mut Result<()>);
// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:137
// ("cv::motempl::segmentMotion", vec![(pred!(mut, ["mhi", "segmask", "boundingRects", "timestamp", "segThresh"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<cv::Rect>*", "double", "double"]), _)]),
pub fn cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vectorLRectGR_double_double(mhi: *const c_void, segmask: *const c_void, bounding_rects: *mut c_void, timestamp: f64, seg_thresh: f64, ocvrs_return: *mut Result<()>);
// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:71
// ("cv::motempl::updateMotionHistory", vec![(pred!(mut, ["silhouette", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double"]), _)]),
pub fn cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:81
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:110
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow", "sigma_dist", "sigma_color", "postprocess_window", "sigma_dist_fix", "sigma_color_fix", "occ_thr", "upscale_averaging_radius", "upscale_sigma_dist", "upscale_sigma_color", "speed_up_thr"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int", "double", "double", "double", "int", "double", "double", "double"]), _)]),
pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64, ocvrs_return: *mut Result<()>);
// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR(from: *const c_void, to: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow", "grid_step", "k", "sigma", "use_post_proc", "fgs_lambda", "fgs_sigma"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool", "float", "float"]), _)]),
pub fn cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from: *const c_void, to: *const c_void, flow: *const c_void, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, ocvrs_return: *mut Result<()>);
// cv::optflow::createOptFlow_DIS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_DIS(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DIS(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, ["preset"], ["int"]), _)]),
pub fn cv_optflow_createOptFlow_DIS_int(preset: i32, ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:244
// ("cv::optflow::createOptFlow_DeepFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_DeepFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:250
// ("cv::optflow::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_Farneback(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:142
// ("cv::optflow::createOptFlow_PCAFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_PCAFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:247
// ("cv::optflow::createOptFlow_SimpleFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_SimpleFlow(ocvrs_return: *mut Result<*mut c_void>);
// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:253
// ("cv::optflow::createOptFlow_SparseToDense", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createOptFlow_SparseToDense(ocvrs_return: *mut Result<*mut c_void>);
// createVariationalFlowRefinement()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:217
// ("cv::optflow::createVariationalFlowRefinement", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_createVariationalFlowRefinement(ocvrs_return: *mut Result<*mut c_void>);
// readOpticalFlow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:148
// ("cv::optflow::readOpticalFlow", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_optflow_readOpticalFlow_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// writeOpticalFlow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:158
// ("cv::optflow::writeOpticalFlow", vec![(pred!(mut, ["path", "flow"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_optflow_writeOpticalFlow_const_StringR_const__InputArrayR(path: *const c_char, flow: *const c_void, ocvrs_return: *mut Result<bool>);
// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:369
// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
pub fn cv_read_const_FileNodeR_NodeR_Node(fn_: *const c_void, node: *mut crate::optflow::GPCTree_Node, unnamed: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:367
// ("cv::write", vec![(pred!(mut, ["fs", "name", "node"], ["cv::FileStorage*", "const cv::String*", "const cv::optflow::GPCTree::Node*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_NodeR(fs: *mut c_void, name: *const c_char, node: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<()>);
// getFinestScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:280
// ("cv::optflow::DISOpticalFlow::getFinestScale", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getFinestScale_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFinestScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:282
// ("cv::optflow::DISOpticalFlow::setFinestScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setFinestScale_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPatchSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:287
// ("cv::optflow::DISOpticalFlow::getPatchSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:289
// ("cv::optflow::DISOpticalFlow::setPatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setPatchSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPatchStride()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:294
// ("cv::optflow::DISOpticalFlow::getPatchStride", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getPatchStride_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatchStride(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:296
// ("cv::optflow::DISOpticalFlow::setPatchStride", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setPatchStride_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getGradientDescentIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:301
// ("cv::optflow::DISOpticalFlow::getGradientDescentIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getGradientDescentIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGradientDescentIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:303
// ("cv::optflow::DISOpticalFlow::setGradientDescentIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setGradientDescentIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:309
// ("cv::optflow::DISOpticalFlow::getVariationalRefinementIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getVariationalRefinementIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setVariationalRefinementIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:311
// ("cv::optflow::DISOpticalFlow::setVariationalRefinementIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setVariationalRefinementIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:315
// ("cv::optflow::DISOpticalFlow::getVariationalRefinementAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getVariationalRefinementAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:317
// ("cv::optflow::DISOpticalFlow::setVariationalRefinementAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setVariationalRefinementAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:321
// ("cv::optflow::DISOpticalFlow::getVariationalRefinementDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getVariationalRefinementDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:323
// ("cv::optflow::DISOpticalFlow::setVariationalRefinementDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setVariationalRefinementDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVariationalRefinementGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:327
// ("cv::optflow::DISOpticalFlow::getVariationalRefinementGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getVariationalRefinementGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVariationalRefinementGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:329
// ("cv::optflow::DISOpticalFlow::setVariationalRefinementGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setVariationalRefinementGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getUseMeanNormalization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:337
// ("cv::optflow::DISOpticalFlow::getUseMeanNormalization", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getUseMeanNormalization_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseMeanNormalization(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:339
// ("cv::optflow::DISOpticalFlow::setUseMeanNormalization", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setUseMeanNormalization_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseSpatialPropagation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:346
// ("cv::optflow::DISOpticalFlow::getUseSpatialPropagation", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_getUseSpatialPropagation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseSpatialPropagation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:348
// ("cv::optflow::DISOpticalFlow::setUseSpatialPropagation", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_optflow_DISOpticalFlow_setUseSpatialPropagation_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// cv::optflow::DISOpticalFlow::to_Algorithm() generated
// ("cv::optflow::DISOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DISOpticalFlow::to_DenseOpticalFlow() generated
// ("cv::optflow::DISOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::DISOpticalFlow::delete() generated
// ("cv::optflow::DISOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_DISOpticalFlow_delete(instance: *mut c_void);
// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:304
// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
pub fn cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(corr: *mut c_void, ocvrs_return: *mut Result<()>);
// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:306
// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
pub fn cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(img_ch: *const c_void, descr: *mut c_void, mp: *const crate::optflow::GPCMatchingParams, typ: i32, ocvrs_return: *mut Result<()>);
// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:309
// ("cv::optflow::GPCDetails::getCoordinatesFromIndex", vec![(pred!(mut, ["index", "sz", "x", "y"], ["size_t", "cv::Size", "int*", "int*"]), _)]),
pub fn cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index: size_t, sz: *const core::Size, x: *mut i32, y: *mut i32, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCDetails::defaultNew() generated
// ("cv::optflow::GPCDetails::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCDetails_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCDetails::delete() generated
// ("cv::optflow::GPCDetails::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCDetails_delete(instance: *mut c_void);
// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["_useOpenCL"], ["bool"]), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl: bool, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams(ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:149
// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["params"], ["const cv::optflow::GPCMatchingParams*"]), _)]),
pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(params: *const crate::optflow::GPCMatchingParams, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:70
// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
pub fn cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(instance: *const c_void, coef: *const core::VecN<f64, 18>, ocvrs_return: *mut Result<f64>);
// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:72
// ("cv::optflow::GPCPatchDescriptor::markAsSeparated", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_markAsSeparated(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// isSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:74
// ("cv::optflow::GPCPatchDescriptor::isSeparated", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_isSeparated_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::optflow::GPCPatchDescriptor::defaultNew() generated
// ("cv::optflow::GPCPatchDescriptor::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_propFeature_const(instance: *const c_void, ocvrs_return: *mut core::VecN<f64, 18>);
// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
pub fn cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(instance: *mut c_void, val: *const core::VecN<f64, 18>);
// cv::optflow::GPCPatchDescriptor::delete() generated
// ("cv::optflow::GPCPatchDescriptor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchDescriptor_delete(instance: *mut c_void);
// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:83
// ("cv::optflow::GPCPatchSample::getDirections", vec![(pred!(const, ["refdir", "posdir", "negdir", "coef", "rhs"], ["bool*", "bool*", "bool*", "const cv::Vec<double, 18>*", "double"]), _)]),
pub fn cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_VecLdouble__18GR_double(instance: *const c_void, refdir: *mut bool, posdir: *mut bool, negdir: *mut bool, coef: *const core::VecN<f64, 18>, rhs: f64, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCPatchSample::defaultNew() generated
// ("cv::optflow::GPCPatchSample::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propRef_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propPos_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_propNeg_const(instance: *const c_void) -> *mut c_void;
// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
pub fn cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(instance: *mut c_void, val: *const c_void);
// cv::optflow::GPCPatchSample::delete() generated
// ("cv::optflow::GPCPatchSample::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCPatchSample_delete(instance: *mut c_void);
// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, ["_maxTreeDepth", "_minNumberOfSamples", "_descriptorType", "_printProgress"], ["unsigned int", "int", "cv::optflow::GPCDescType", "bool"]), _)]),
pub fn cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool, ocvrs_return: *mut Result<crate::optflow::GPCTrainingParams>);
// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTrainingParams_GPCTrainingParams(ocvrs_return: *mut Result<crate::optflow::GPCTrainingParams>);
// check()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:138
// ("cv::optflow::GPCTrainingParams::check", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingParams_check_const(instance: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result<bool>);
// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:108
// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const std::vector<cv::String>*", "const std::vector<cv::String>*", "const std::vector<cv::String>*", "int"]), _)]),
pub fn cv_optflow_GPCTrainingSamples_create_const_vectorLStringGR_const_vectorLStringGR_const_vectorLStringGR_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:111
// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:114
// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:116
// ("cv::optflow::GPCTrainingSamples::type", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::optflow::GPCTrainingSamples::defaultNew() generated
// ("cv::optflow::GPCTrainingSamples::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_defaultNew_const() -> *mut c_void;
// cv::optflow::GPCTrainingSamples::delete() generated
// ("cv::optflow::GPCTrainingSamples::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTrainingSamples_delete(instance: *mut c_void);
// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples", "params"], ["cv::optflow::GPCTrainingSamples*", "const cv::optflow::GPCTrainingParams"]), _)]),
pub fn cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(instance: *mut c_void, samples: *mut c_void, params: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result<()>);
// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
pub fn cv_optflow_GPCTree_train_GPCTrainingSamplesR(instance: *mut c_void, samples: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:178
// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_optflow_GPCTree_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:180
// ("cv::optflow::GPCTree::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_optflow_GPCTree_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:182
// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
pub fn cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(instance: *const c_void, descr: *const c_void, ocvrs_return: *mut Result<u32>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:184
// ("cv::optflow::GPCTree::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_GPCTree_create(ocvrs_return: *mut Result<*mut c_void>);
// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:186
// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
pub fn cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(instance: *const c_void, t: *const c_void, ocvrs_return: *mut Result<bool>);
// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:188
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
// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:164
// ("cv::optflow::GPCTree::Node::operator==", vec![(pred!(const, ["n"], ["const cv::optflow::GPCTree::Node*"]), _)]),
pub fn cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(instance: *const crate::optflow::GPCTree_Node, n: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<bool>);
// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, ["_prior", "_basisSize", "_sparseRate", "_retainedCornersFraction", "_occlusionsThreshold", "_dampingFactor", "_claheClip"], ["cv::Ptr<const cv::optflow::PCAPrior>", "const cv::Size", "float", "float", "float", "float", "float"]), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_PtrLconst_PCAPriorG_const_Size_float_float_float_float_float(_prior: *const c_void, _basis_size: *const core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow(ocvrs_return: *mut Result<*mut c_void>);
// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:120
// ("cv::optflow::OpticalFlowPCAFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:121
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
// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:83
// ("cv::optflow::PCAPrior::PCAPrior", vec![(pred!(mut, ["pathToPrior"], ["const char*"]), _)]),
pub fn cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getPadding()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:85
// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_PCAPrior_getPadding_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getBasisSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:87
// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_PCAPrior_getBasisSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:89
// ("cv::optflow::PCAPrior::fillConstraints", vec![(pred!(const, ["A1", "A2", "b1", "b2"], ["float*", "float*", "float*", "float*"]), _)]),
pub fn cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(instance: *const c_void, a1: *mut f32, a2: *mut f32, b1: *mut f32, b2: *mut f32, ocvrs_return: *mut Result<()>);
// cv::optflow::PCAPrior::delete() generated
// ("cv::optflow::PCAPrior::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_PCAPrior_delete(instance: *mut c_void);
// calcUV(InputArray, InputArray, InputOutputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:175
// ("cv::optflow::VariationalRefinement::calcUV", vec![(pred!(mut, ["I0", "I1", "flow_u", "flow_v"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_optflow_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow_u: *const c_void, flow_v: *const c_void, ocvrs_return: *mut Result<()>);
// getFixedPointIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:179
// ("cv::optflow::VariationalRefinement::getFixedPointIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getFixedPointIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFixedPointIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:181
// ("cv::optflow::VariationalRefinement::setFixedPointIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_VariationalRefinement_setFixedPointIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSorIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:186
// ("cv::optflow::VariationalRefinement::getSorIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getSorIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSorIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:188
// ("cv::optflow::VariationalRefinement::setSorIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_optflow_VariationalRefinement_setSorIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getOmega()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:192
// ("cv::optflow::VariationalRefinement::getOmega", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getOmega_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setOmega(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:194
// ("cv::optflow::VariationalRefinement::setOmega", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_VariationalRefinement_setOmega_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:198
// ("cv::optflow::VariationalRefinement::getAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:200
// ("cv::optflow::VariationalRefinement::setAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_VariationalRefinement_setAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:204
// ("cv::optflow::VariationalRefinement::getDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:206
// ("cv::optflow::VariationalRefinement::setDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_VariationalRefinement_setDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:210
// ("cv::optflow::VariationalRefinement::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:212
// ("cv::optflow::VariationalRefinement::setGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_optflow_VariationalRefinement_setGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// cv::optflow::VariationalRefinement::to_Algorithm() generated
// ("cv::optflow::VariationalRefinement::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::optflow::VariationalRefinement::to_DenseOpticalFlow() generated
// ("cv::optflow::VariationalRefinement::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_to_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::optflow::VariationalRefinement::delete() generated
// ("cv::optflow::VariationalRefinement::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_optflow_VariationalRefinement_delete(instance: *mut c_void);
