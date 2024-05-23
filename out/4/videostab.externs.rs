// calcBlurriness(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:57
// ("cv::videostab::calcBlurriness", vec![(pred!(mut, ["frame"], ["const cv::Mat*"]), _)]),
pub fn cv_videostab_calcBlurriness_const_MatR(frame: *const c_void, ocvrs_return: *mut Result<f32>);
// calcFlowMask(const Mat &, const Mat &, const Mat &, float, const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:199
// ("cv::videostab::calcFlowMask", vec![(pred!(mut, ["flowX", "flowY", "errors", "maxError", "mask0", "mask1", "flowMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(flow_x: *const c_void, flow_y: *const c_void, errors: *const c_void, max_error: f32, mask0: *const c_void, mask1: *const c_void, flow_mask: *mut c_void, ocvrs_return: *mut Result<()>);
// completeFrameAccordingToFlow(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, float, Mat &, Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:203
// ("cv::videostab::completeFrameAccordingToFlow", vec![(pred!(mut, ["flowMask", "flowX", "flowY", "frame1", "mask1", "distThresh", "frame0", "mask0"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(flow_mask: *const c_void, flow_x: *const c_void, flow_y: *const c_void, frame1: *const c_void, mask1: *const c_void, dist_thresh: f32, frame0: *mut c_void, mask0: *mut c_void, ocvrs_return: *mut Result<()>);
// ensureInclusionConstraint(const Mat &, Size, float)(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:165
// ("cv::videostab::ensureInclusionConstraint", vec![(pred!(mut, ["M", "size", "trimRatio"], ["const cv::Mat*", "cv::Size", "float"]), _)]),
pub fn cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(m: *const c_void, size: *const core::Size, trim_ratio: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:77
// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR(points0: *const c_void, points1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray, int, float *)(InputOutputArray, InputOutputArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:77
// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1", "model", "rmse"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int", "float*"]), _)]),
pub fn cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(points0: *const c_void, points1: *const c_void, model: i32, rmse: *mut f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::estimateGlobalMotionRansac(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:90
// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR(points0: *const c_void, points1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimateGlobalMotionRansac(InputArray, InputArray, int, const RansacParams &, float *, int *)(InputArray, InputArray, Primitive, TraitClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:90
// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1", "model", "params", "rmse", "ninliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::videostab::RansacParams*", "float*", "int*"]), _)]),
pub fn cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(points0: *const c_void, points1: *const c_void, model: i32, params: *const c_void, rmse: *mut f32, ninliers: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
// estimateOptimalTrimRatio(const Mat &, Size)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:167
// ("cv::videostab::estimateOptimalTrimRatio", vec![(pred!(mut, ["M", "size"], ["const cv::Mat*", "cv::Size"]), _)]),
pub fn cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(m: *const c_void, size: *const core::Size, ocvrs_return: *mut Result<f32>);
// getMotion(int, int, const std::vector<Mat> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:304
// ("cv::videostab::getMotion", vec![(pred!(mut, ["from", "to", "motions"], ["int", "int", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_getMotion_int_int_const_vectorLMatGR(from: i32, to: i32, motions: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:177
// ("cv::videostab::ColorAverageInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::ColorAverageInpainter::defaultNew() generated
// ("cv::videostab::ColorAverageInpainter::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_ColorAverageInpainter_defaultNew_const() -> *mut c_void;
// cv::videostab::ColorAverageInpainter::to_InpainterBase() generated
// ("cv::videostab::ColorAverageInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ColorAverageInpainter_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ColorAverageInpainter::delete() generated
// ("cv::videostab::ColorAverageInpainter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ColorAverageInpainter_delete(instance: *mut c_void);
// ColorInpainter(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:186
// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, ["method", "radius"], ["int", "double"]), _)]),
pub fn cv_videostab_ColorInpainter_ColorInpainter_int_double(method: i32, radius: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::ColorInpainter::ColorInpainter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:186
// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ColorInpainter_ColorInpainter(ocvrs_return: *mut Result<*mut c_void>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:188
// ("cv::videostab::ColorInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::ColorInpainter::to_InpainterBase() generated
// ("cv::videostab::ColorInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ColorInpainter_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ColorInpainter::delete() generated
// ("cv::videostab::ColorInpainter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ColorInpainter_delete(instance: *mut c_void);
// ConsistentMosaicInpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:130
// ("cv::videostab::ConsistentMosaicInpainter::ConsistentMosaicInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter(ocvrs_return: *mut Result<*mut c_void>);
// setStdevThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:132
// ("cv::videostab::ConsistentMosaicInpainter::setStdevThresh", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// stdevThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:133
// ("cv::videostab::ConsistentMosaicInpainter::stdevThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:135
// ("cv::videostab::ConsistentMosaicInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::ConsistentMosaicInpainter::to_InpainterBase() generated
// ("cv::videostab::ConsistentMosaicInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ConsistentMosaicInpainter::delete() generated
// ("cv::videostab::ConsistentMosaicInpainter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ConsistentMosaicInpainter_delete(instance: *mut c_void);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:66
// ("cv::videostab::DeblurerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_DeblurerBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:67
// ("cv::videostab::DeblurerBase::radius", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_DeblurerBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:69
// ("cv::videostab::DeblurerBase::deblur", vec![(pred!(mut, ["idx", "frame", "range"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
pub fn cv_videostab_DeblurerBase_deblur_int_MatR_const_RangeR(instance: *mut c_void, idx: i32, frame: *mut c_void, range: *const c_void, ocvrs_return: *mut Result<()>);
// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:74
// ("cv::videostab::DeblurerBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_DeblurerBase_setFrames_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// frames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:75
// ("cv::videostab::DeblurerBase::frames", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_DeblurerBase_frames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:77
// ("cv::videostab::DeblurerBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_DeblurerBase_setMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:78
// ("cv::videostab::DeblurerBase::motions", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_DeblurerBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setBlurrinessRates(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:80
// ("cv::videostab::DeblurerBase::setBlurrinessRates", vec![(pred!(mut, ["val"], ["const std::vector<float>*"]), _)]),
pub fn cv_videostab_DeblurerBase_setBlurrinessRates_const_vectorLfloatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// blurrinessRates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:81
// ("cv::videostab::DeblurerBase::blurrinessRates", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_DeblurerBase_blurrinessRates_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::DeblurerBase::to_NullDeblurer() generated
// ("cv::videostab::DeblurerBase::to_NullDeblurer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DeblurerBase_to_NullDeblurer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::DeblurerBase::to_WeightingDeblurer() generated
// ("cv::videostab::DeblurerBase::to_WeightingDeblurer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DeblurerBase_to_WeightingDeblurer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::DeblurerBase::delete() generated
// ("cv::videostab::DeblurerBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DeblurerBase_delete(instance: *mut c_void);
// DensePyrLkOptFlowEstimatorGpu()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:132
// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::DensePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DensePyrLkOptFlowEstimatorGpu_DensePyrLkOptFlowEstimatorGpu(ocvrs_return: *mut Result<*mut c_void>);
// run(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:134
// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "flowX", "flowY", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_DensePyrLkOptFlowEstimatorGpu_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow_x: *const c_void, flow_y: *const c_void, errors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::DensePyrLkOptFlowEstimatorGpu::to_IDenseOptFlowEstimator() generated
// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::to_IDenseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DensePyrLkOptFlowEstimatorGpu_to_IDenseOptFlowEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::DensePyrLkOptFlowEstimatorGpu::to_PyrLkOptFlowEstimatorBase() generated
// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::to_PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DensePyrLkOptFlowEstimatorGpu_to_PyrLkOptFlowEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::DensePyrLkOptFlowEstimatorGpu::delete() generated
// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_DensePyrLkOptFlowEstimatorGpu_delete(instance: *mut c_void);
// FastMarchingMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:66
// ("cv::videostab::FastMarchingMethod::FastMarchingMethod", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_FastMarchingMethod_FastMarchingMethod(ocvrs_return: *mut Result<*mut c_void>);
// distanceMap()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:81
// ("cv::videostab::FastMarchingMethod::distanceMap", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_FastMarchingMethod_distanceMap_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::FastMarchingMethod::delete() generated
// ("cv::videostab::FastMarchingMethod::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_FastMarchingMethod_delete(instance: *mut c_void);
// FromFileMotionReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:201
// ("cv::videostab::FromFileMotionReader::FromFileMotionReader", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:203
// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
pub fn cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::FromFileMotionReader::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:203
// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::FromFileMotionReader::to_ImageMotionEstimatorBase() generated
// ("cv::videostab::FromFileMotionReader::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_FromFileMotionReader_to_ImageMotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::FromFileMotionReader::delete() generated
// ("cv::videostab::FromFileMotionReader::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_FromFileMotionReader_delete(instance: *mut c_void);
// GaussianMotionFilter(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:100
// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
pub fn cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(radius: i32, stdev: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::GaussianMotionFilter::GaussianMotionFilter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:100
// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_GaussianMotionFilter(ocvrs_return: *mut Result<*mut c_void>);
// setParams(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:102
// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
pub fn cv_videostab_GaussianMotionFilter_setParams_int_float(instance: *mut c_void, radius: i32, stdev: f32, ocvrs_return: *mut Result<()>);
// cv::videostab::GaussianMotionFilter::setParams(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:102
// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius"], ["int"]), _)]),
pub fn cv_videostab_GaussianMotionFilter_setParams_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result<()>);
// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:103
// ("cv::videostab::GaussianMotionFilter::radius", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// stdev()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:104
// ("cv::videostab::GaussianMotionFilter::stdev", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_stdev_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// stabilize(int, const std::vector<Mat> &, const Range &)(Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:106
// ("cv::videostab::GaussianMotionFilter::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
pub fn cv_videostab_GaussianMotionFilter_stabilize_int_const_vectorLMatGR_const_RangeR(instance: *mut c_void, idx: i32, motions: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::GaussianMotionFilter::to_IMotionStabilizer() generated
// ("cv::videostab::GaussianMotionFilter::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_to_IMotionStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::GaussianMotionFilter::to_MotionFilterBase() generated
// ("cv::videostab::GaussianMotionFilter::to_MotionFilterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_to_MotionFilterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::GaussianMotionFilter::delete() generated
// ("cv::videostab::GaussianMotionFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_GaussianMotionFilter_delete(instance: *mut c_void);
// run(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:74
// ("cv::videostab::IDenseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "flowX", "flowY", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow_x: *const c_void, flow_y: *const c_void, errors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::IDenseOptFlowEstimator::to_DensePyrLkOptFlowEstimatorGpu() generated
// ("cv::videostab::IDenseOptFlowEstimator::to_DensePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IDenseOptFlowEstimator_to_DensePyrLkOptFlowEstimatorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IDenseOptFlowEstimator::delete() generated
// ("cv::videostab::IDenseOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IDenseOptFlowEstimator_delete(instance: *mut c_void);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:61
// ("cv::videostab::IFrameSource::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:62
// ("cv::videostab::IFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::IFrameSource::to_MaskFrameSource() generated
// ("cv::videostab::IFrameSource::to_MaskFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_to_MaskFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IFrameSource::to_NullFrameSource() generated
// ("cv::videostab::IFrameSource::to_NullFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_to_NullFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IFrameSource::to_OnePassStabilizer() generated
// ("cv::videostab::IFrameSource::to_OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_to_OnePassStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IFrameSource::to_TwoPassStabilizer() generated
// ("cv::videostab::IFrameSource::to_TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_to_TwoPassStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IFrameSource::to_VideoFileSource() generated
// ("cv::videostab::IFrameSource::to_VideoFileSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_to_VideoFileSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IFrameSource::delete() generated
// ("cv::videostab::IFrameSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IFrameSource_delete(instance: *mut c_void);
// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:60
// ("cv::videostab::ILog::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
pub fn cv_videostab_ILog_print_const_charX(instance: *mut c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::videostab::ILog::to_LogToStdout() generated
// ("cv::videostab::ILog::to_LogToStdout", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ILog_to_LogToStdout(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ILog::to_NullLog() generated
// ("cv::videostab::ILog::to_NullLog", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ILog_to_NullLog(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ILog::delete() generated
// ("cv::videostab::ILog::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ILog_delete(instance: *mut c_void);
// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:65
// ("cv::videostab::IMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
pub fn cv_videostab_IMotionStabilizer_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::IMotionStabilizer::to_GaussianMotionFilter() generated
// ("cv::videostab::IMotionStabilizer::to_GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IMotionStabilizer_to_GaussianMotionFilter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IMotionStabilizer::to_LpMotionStabilizer() generated
// ("cv::videostab::IMotionStabilizer::to_LpMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IMotionStabilizer_to_LpMotionStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IMotionStabilizer::to_MotionFilterBase() generated
// ("cv::videostab::IMotionStabilizer::to_MotionFilterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IMotionStabilizer_to_MotionFilterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IMotionStabilizer::to_MotionStabilizationPipeline() generated
// ("cv::videostab::IMotionStabilizer::to_MotionStabilizationPipeline", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IMotionStabilizer_to_MotionStabilizationPipeline(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IMotionStabilizer::delete() generated
// ("cv::videostab::IMotionStabilizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IMotionStabilizer_delete(instance: *mut c_void);
// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:63
// ("cv::videostab::IOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::IOutlierRejector::to_NullOutlierRejector() generated
// ("cv::videostab::IOutlierRejector::to_NullOutlierRejector", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IOutlierRejector_to_NullOutlierRejector(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IOutlierRejector::to_TranslationBasedLocalOutlierRejector() generated
// ("cv::videostab::IOutlierRejector::to_TranslationBasedLocalOutlierRejector", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IOutlierRejector_to_TranslationBasedLocalOutlierRejector(instance: *mut c_void) -> *mut c_void;
// cv::videostab::IOutlierRejector::delete() generated
// ("cv::videostab::IOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_IOutlierRejector_delete(instance: *mut c_void);
// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:65
// ("cv::videostab::ISparseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *const c_void, status: *const c_void, errors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimator() generated
// ("cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ISparseOptFlowEstimator_to_SparsePyrLkOptFlowEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimatorGpu() generated
// ("cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ISparseOptFlowEstimator_to_SparsePyrLkOptFlowEstimatorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ISparseOptFlowEstimator::delete() generated
// ("cv::videostab::ISparseOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ISparseOptFlowEstimator_delete(instance: *mut c_void);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:180
// ("cv::videostab::ImageMotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:181
// ("cv::videostab::ImageMotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:183
// ("cv::videostab::ImageMotionEstimatorBase::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:189
// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::ImageMotionEstimatorBase::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:189
// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::ImageMotionEstimatorBase::to_FromFileMotionReader() generated
// ("cv::videostab::ImageMotionEstimatorBase::to_FromFileMotionReader", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_to_FromFileMotionReader(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimator() generated
// ("cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_to_KeypointBasedMotionEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimatorGpu() generated
// ("cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_to_KeypointBasedMotionEstimatorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ImageMotionEstimatorBase::to_ToFileMotionWriter() generated
// ("cv::videostab::ImageMotionEstimatorBase::to_ToFileMotionWriter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_to_ToFileMotionWriter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ImageMotionEstimatorBase::delete() generated
// ("cv::videostab::ImageMotionEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ImageMotionEstimatorBase_delete(instance: *mut c_void);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:70
// ("cv::videostab::InpainterBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_InpainterBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:71
// ("cv::videostab::InpainterBase::radius", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:73
// ("cv::videostab::InpainterBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_InpainterBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:74
// ("cv::videostab::InpainterBase::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:76
// ("cv::videostab::InpainterBase::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_InpainterBase_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:81
// ("cv::videostab::InpainterBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpainterBase_setFrames_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// frames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:82
// ("cv::videostab::InpainterBase::frames", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_frames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:84
// ("cv::videostab::InpainterBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpainterBase_setMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:85
// ("cv::videostab::InpainterBase::motions", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:87
// ("cv::videostab::InpainterBase::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpainterBase_setStabilizedFrames_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// stabilizedFrames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:88
// ("cv::videostab::InpainterBase::stabilizedFrames", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_stabilizedFrames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:90
// ("cv::videostab::InpainterBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpainterBase_setStabilizationMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:91
// ("cv::videostab::InpainterBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpainterBase_stabilizationMotions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::InpainterBase::to_ColorAverageInpainter() generated
// ("cv::videostab::InpainterBase::to_ColorAverageInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_ColorAverageInpainter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::to_ColorInpainter() generated
// ("cv::videostab::InpainterBase::to_ColorInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_ColorInpainter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::to_ConsistentMosaicInpainter() generated
// ("cv::videostab::InpainterBase::to_ConsistentMosaicInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_ConsistentMosaicInpainter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::to_InpaintingPipeline() generated
// ("cv::videostab::InpainterBase::to_InpaintingPipeline", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_InpaintingPipeline(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::to_MotionInpainter() generated
// ("cv::videostab::InpainterBase::to_MotionInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_MotionInpainter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::to_NullInpainter() generated
// ("cv::videostab::InpainterBase::to_NullInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_to_NullInpainter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpainterBase::delete() generated
// ("cv::videostab::InpainterBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpainterBase_delete(instance: *mut c_void);
// pushBack(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:111
// ("cv::videostab::InpaintingPipeline::pushBack", vec![(pred!(mut, ["inpainter"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
pub fn cv_videostab_InpaintingPipeline_pushBack_PtrLInpainterBaseG(instance: *mut c_void, inpainter: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:112
// ("cv::videostab::InpaintingPipeline::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpaintingPipeline_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:114
// ("cv::videostab::InpaintingPipeline::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:115
// ("cv::videostab::InpaintingPipeline::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:116
// ("cv::videostab::InpaintingPipeline::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setFrames_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:117
// ("cv::videostab::InpaintingPipeline::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:118
// ("cv::videostab::InpaintingPipeline::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:119
// ("cv::videostab::InpaintingPipeline::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:121
// ("cv::videostab::InpaintingPipeline::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::InpaintingPipeline::defaultNew() generated
// ("cv::videostab::InpaintingPipeline::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_InpaintingPipeline_defaultNew_const() -> *mut c_void;
// cv::videostab::InpaintingPipeline::to_InpainterBase() generated
// ("cv::videostab::InpaintingPipeline::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpaintingPipeline_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::InpaintingPipeline::delete() generated
// ("cv::videostab::InpaintingPipeline::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_InpaintingPipeline_delete(instance: *mut c_void);
// KeypointBasedMotionEstimator(Ptr<MotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:232
// ("cv::videostab::KeypointBasedMotionEstimator::KeypointBasedMotionEstimator", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::videostab::MotionEstimatorBase>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrLMotionEstimatorBaseG(estimator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:234
// ("cv::videostab::KeypointBasedMotionEstimator::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:235
// ("cv::videostab::KeypointBasedMotionEstimator::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// setDetector(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:237
// ("cv::videostab::KeypointBasedMotionEstimator::setDetector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::Feature2D>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_setDetector_PtrLFeature2DG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// detector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:238
// ("cv::videostab::KeypointBasedMotionEstimator::detector", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_detector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setOpticalFlowEstimator(Ptr<ISparseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:240
// ("cv::videostab::KeypointBasedMotionEstimator::setOpticalFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ISparseOptFlowEstimator>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_PtrLISparseOptFlowEstimatorG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// opticalFlowEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:241
// ("cv::videostab::KeypointBasedMotionEstimator::opticalFlowEstimator", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setOutlierRejector(Ptr<IOutlierRejector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:243
// ("cv::videostab::KeypointBasedMotionEstimator::setOutlierRejector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IOutlierRejector>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_PtrLIOutlierRejectorG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// outlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:244
// ("cv::videostab::KeypointBasedMotionEstimator::outlierRejector", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:246
// ("cv::videostab::KeypointBasedMotionEstimator::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:248
// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimator::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:248
// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:249
// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimator::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:249
// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimator::to_ImageMotionEstimatorBase() generated
// ("cv::videostab::KeypointBasedMotionEstimator::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_to_ImageMotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::KeypointBasedMotionEstimator::delete() generated
// ("cv::videostab::KeypointBasedMotionEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimator_delete(instance: *mut c_void);
// KeypointBasedMotionEstimatorGpu(Ptr<MotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:269
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::KeypointBasedMotionEstimatorGpu", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::videostab::MotionEstimatorBase>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_KeypointBasedMotionEstimatorGpu_PtrLMotionEstimatorBaseG(estimator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:271
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:272
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// setOutlierRejector(Ptr<IOutlierRejector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:274
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::setOutlierRejector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IOutlierRejector>"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_setOutlierRejector_PtrLIOutlierRejectorG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// outlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:275
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::outlierRejector", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_outlierRejector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:277
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimatorGpu::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:277
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_MatR_const_MatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimate(const cuda::GpuMat &, const cuda::GpuMat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:278
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "bool*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_GpuMatR_const_GpuMatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimatorGpu::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:278
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*"]), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_GpuMatR_const_GpuMatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::KeypointBasedMotionEstimatorGpu::to_ImageMotionEstimatorBase() generated
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_to_ImageMotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::KeypointBasedMotionEstimatorGpu::delete() generated
// ("cv::videostab::KeypointBasedMotionEstimatorGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_KeypointBasedMotionEstimatorGpu_delete(instance: *mut c_void);
// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:72
// ("cv::videostab::LogToStdout::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
pub fn cv_videostab_LogToStdout_print_const_charX(instance: *mut c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::videostab::LogToStdout::defaultNew() generated
// ("cv::videostab::LogToStdout::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LogToStdout_defaultNew_const() -> *mut c_void;
// cv::videostab::LogToStdout::to_ILog() generated
// ("cv::videostab::LogToStdout::to_ILog", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_LogToStdout_to_ILog(instance: *mut c_void) -> *mut c_void;
// cv::videostab::LogToStdout::delete() generated
// ("cv::videostab::LogToStdout::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_LogToStdout_delete(instance: *mut c_void);
// LpMotionStabilizer(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:120
// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::LpMotionStabilizer::LpMotionStabilizer() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:120
// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_LpMotionStabilizer(ocvrs_return: *mut Result<*mut c_void>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:122
// ("cv::videostab::LpMotionStabilizer::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:123
// ("cv::videostab::LpMotionStabilizer::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// setFrameSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:125
// ("cv::videostab::LpMotionStabilizer::setFrameSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setFrameSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result<()>);
// frameSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:126
// ("cv::videostab::LpMotionStabilizer::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_frameSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:128
// ("cv::videostab::LpMotionStabilizer::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setTrimRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// trimRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:129
// ("cv::videostab::LpMotionStabilizer::trimRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_trimRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeight1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:131
// ("cv::videostab::LpMotionStabilizer::setWeight1", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setWeight1_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// weight1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:132
// ("cv::videostab::LpMotionStabilizer::weight1", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_weight1_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeight2(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:134
// ("cv::videostab::LpMotionStabilizer::setWeight2", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setWeight2_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// weight2()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:135
// ("cv::videostab::LpMotionStabilizer::weight2", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_weight2_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeight3(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:137
// ("cv::videostab::LpMotionStabilizer::setWeight3", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setWeight3_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// weight3()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:138
// ("cv::videostab::LpMotionStabilizer::weight3", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_weight3_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeight4(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:140
// ("cv::videostab::LpMotionStabilizer::setWeight4", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_setWeight4_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// weight4()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:141
// ("cv::videostab::LpMotionStabilizer::weight4", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_weight4_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:143
// ("cv::videostab::LpMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
pub fn cv_videostab_LpMotionStabilizer_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::LpMotionStabilizer::to_IMotionStabilizer() generated
// ("cv::videostab::LpMotionStabilizer::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_to_IMotionStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::LpMotionStabilizer::delete() generated
// ("cv::videostab::LpMotionStabilizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_LpMotionStabilizer_delete(instance: *mut c_void);
// MaskFrameSource(const Ptr<IFrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:92
// ("cv::videostab::MaskFrameSource::MaskFrameSource", vec![(pred!(mut, ["source"], ["const cv::Ptr<cv::videostab::IFrameSource>*"]), _)]),
pub fn cv_videostab_MaskFrameSource_MaskFrameSource_const_PtrLIFrameSourceGR(source: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:94
// ("cv::videostab::MaskFrameSource::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MaskFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:95
// ("cv::videostab::MaskFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MaskFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MaskFrameSource::to_IFrameSource() generated
// ("cv::videostab::MaskFrameSource::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MaskFrameSource_to_IFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MaskFrameSource::delete() generated
// ("cv::videostab::MaskFrameSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MaskFrameSource_delete(instance: *mut c_void);
// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:116
// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::MoreAccurateMotionWobbleSuppressor::defaultNew() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_defaultNew_const() -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressor::to_MoreAccurateMotionWobbleSuppressorBase() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::to_MoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_to_MoreAccurateMotionWobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressor::to_WobbleSuppressorBase() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_to_WobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressor::delete() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_delete(instance: *mut c_void);
// setPeriod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:104
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::setPeriod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// period()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:105
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::period", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorGpu() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_WobbleSuppressorBase() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_WobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::delete() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_delete(instance: *mut c_void);
// suppress(int, const cuda::GpuMat &, cuda::GpuMat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:126
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_suppress_int_const_GpuMatR_GpuMatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:127
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::defaultNew() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_defaultNew_const() -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::to_MoreAccurateMotionWobbleSuppressorBase() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::to_MoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_to_MoreAccurateMotionWobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::to_WobbleSuppressorBase() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_to_WobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::delete() generated
// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_delete(instance: *mut c_void);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:106
// ("cv::videostab::MotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:111
// ("cv::videostab::MotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionEstimatorBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:120
// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
pub fn cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorBase::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:120
// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorBase::to_MotionEstimatorL1() generated
// ("cv::videostab::MotionEstimatorBase::to_MotionEstimatorL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorBase_to_MotionEstimatorL1(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionEstimatorBase::to_MotionEstimatorRansacL2() generated
// ("cv::videostab::MotionEstimatorBase::to_MotionEstimatorRansacL2", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorBase_to_MotionEstimatorRansacL2(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionEstimatorBase::delete() generated
// ("cv::videostab::MotionEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorBase_delete(instance: *mut c_void);
// MotionEstimatorL1(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:156
// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorL1::MotionEstimatorL1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:156
// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorL1_MotionEstimatorL1(ocvrs_return: *mut Result<*mut c_void>);
// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:158
// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
pub fn cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorL1::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:158
// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorL1::to_MotionEstimatorBase() generated
// ("cv::videostab::MotionEstimatorL1::to_MotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorL1_to_MotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionEstimatorL1::delete() generated
// ("cv::videostab::MotionEstimatorL1::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorL1_delete(instance: *mut c_void);
// MotionEstimatorRansacL2(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:134
// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:134
// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2(ocvrs_return: *mut Result<*mut c_void>);
// setRansacParams(const RansacParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:136
// ("cv::videostab::MotionEstimatorRansacL2::setRansacParams", vec![(pred!(mut, ["val"], ["const cv::videostab::RansacParams*"]), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// ransacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:137
// ("cv::videostab::MotionEstimatorRansacL2::ransacParams", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_ransacParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMinInlierRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:139
// ("cv::videostab::MotionEstimatorRansacL2::setMinInlierRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// minInlierRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:140
// ("cv::videostab::MotionEstimatorRansacL2::minInlierRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:142
// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorRansacL2::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:142
// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::MotionEstimatorRansacL2::to_MotionEstimatorBase() generated
// ("cv::videostab::MotionEstimatorRansacL2::to_MotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_to_MotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionEstimatorRansacL2::delete() generated
// ("cv::videostab::MotionEstimatorRansacL2::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionEstimatorRansacL2_delete(instance: *mut c_void);
// stabilize(int, const std::vector<Mat> &, const Range &)(Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:89
// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
pub fn cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_const_RangeR(instance: *mut c_void, idx: i32, motions: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:92
// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
pub fn cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::MotionFilterBase::to_GaussianMotionFilter() generated
// ("cv::videostab::MotionFilterBase::to_GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionFilterBase_to_GaussianMotionFilter(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionFilterBase::to_IMotionStabilizer() generated
// ("cv::videostab::MotionFilterBase::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionFilterBase_to_IMotionStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionFilterBase::delete() generated
// ("cv::videostab::MotionFilterBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionFilterBase_delete(instance: *mut c_void);
// MotionInpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:144
// ("cv::videostab::MotionInpainter::MotionInpainter", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionInpainter_MotionInpainter(ocvrs_return: *mut Result<*mut c_void>);
// setOptFlowEstimator(Ptr<IDenseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:146
// ("cv::videostab::MotionInpainter::setOptFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IDenseOptFlowEstimator>"]), _)]),
pub fn cv_videostab_MotionInpainter_setOptFlowEstimator_PtrLIDenseOptFlowEstimatorG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// optFlowEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:147
// ("cv::videostab::MotionInpainter::optFlowEstimator", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionInpainter_optFlowEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setFlowErrorThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:149
// ("cv::videostab::MotionInpainter::setFlowErrorThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_MotionInpainter_setFlowErrorThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// flowErrorThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:150
// ("cv::videostab::MotionInpainter::flowErrorThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionInpainter_flowErrorThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setDistThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:152
// ("cv::videostab::MotionInpainter::setDistThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_MotionInpainter_setDistThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// distThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:153
// ("cv::videostab::MotionInpainter::distThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionInpainter_distThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:155
// ("cv::videostab::MotionInpainter::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_MotionInpainter_setBorderMode_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// borderMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:156
// ("cv::videostab::MotionInpainter::borderMode", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionInpainter_borderMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:158
// ("cv::videostab::MotionInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::MotionInpainter::to_InpainterBase() generated
// ("cv::videostab::MotionInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionInpainter_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionInpainter::delete() generated
// ("cv::videostab::MotionInpainter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionInpainter_delete(instance: *mut c_void);
// pushBack(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:73
// ("cv::videostab::MotionStabilizationPipeline::pushBack", vec![(pred!(mut, ["stabilizer"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_pushBack_PtrLIMotionStabilizerG(instance: *mut c_void, stabilizer: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:74
// ("cv::videostab::MotionStabilizationPipeline::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:76
// ("cv::videostab::MotionStabilizationPipeline::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::MotionStabilizationPipeline::defaultNew() generated
// ("cv::videostab::MotionStabilizationPipeline::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_defaultNew_const() -> *mut c_void;
// cv::videostab::MotionStabilizationPipeline::to_IMotionStabilizer() generated
// ("cv::videostab::MotionStabilizationPipeline::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_to_IMotionStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::MotionStabilizationPipeline::delete() generated
// ("cv::videostab::MotionStabilizationPipeline::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_MotionStabilizationPipeline_delete(instance: *mut c_void);
// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:93
// ("cv::videostab::NullDeblurer::deblur", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
pub fn cv_videostab_NullDeblurer_deblur_int_MatR_const_RangeR(instance: *mut c_void, unnamed: i32, unnamed_1: *mut c_void, unnamed_2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::NullDeblurer::defaultNew() generated
// ("cv::videostab::NullDeblurer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullDeblurer_defaultNew_const() -> *mut c_void;
// cv::videostab::NullDeblurer::to_DeblurerBase() generated
// ("cv::videostab::NullDeblurer::to_DeblurerBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullDeblurer_to_DeblurerBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullDeblurer::delete() generated
// ("cv::videostab::NullDeblurer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullDeblurer_delete(instance: *mut c_void);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:68
// ("cv::videostab::NullFrameSource::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:69
// ("cv::videostab::NullFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::NullFrameSource::defaultNew() generated
// ("cv::videostab::NullFrameSource::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullFrameSource_defaultNew_const() -> *mut c_void;
// cv::videostab::NullFrameSource::to_IFrameSource() generated
// ("cv::videostab::NullFrameSource::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullFrameSource_to_IFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullFrameSource::delete() generated
// ("cv::videostab::NullFrameSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullFrameSource_delete(instance: *mut c_void);
// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:105
// ("cv::videostab::NullInpainter::inpaint", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_NullInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, unnamed: i32, unnamed_1: *mut c_void, unnamed_2: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::NullInpainter::defaultNew() generated
// ("cv::videostab::NullInpainter::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullInpainter_defaultNew_const() -> *mut c_void;
// cv::videostab::NullInpainter::to_InpainterBase() generated
// ("cv::videostab::NullInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullInpainter_to_InpainterBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullInpainter::delete() generated
// ("cv::videostab::NullInpainter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullInpainter_delete(instance: *mut c_void);
// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:66
// ("cv::videostab::NullLog::print", vec![(pred!(mut, ["unnamed"], ["const char*"]), _)]),
pub fn cv_videostab_NullLog_print_const_charX(instance: *mut c_void, unnamed: *const c_char, ocvrs_return: *mut Result<()>);
// cv::videostab::NullLog::defaultNew() generated
// ("cv::videostab::NullLog::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullLog_defaultNew_const() -> *mut c_void;
// cv::videostab::NullLog::to_ILog() generated
// ("cv::videostab::NullLog::to_ILog", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullLog_to_ILog(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullLog::delete() generated
// ("cv::videostab::NullLog::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullLog_delete(instance: *mut c_void);
// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:70
// ("cv::videostab::NullOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::NullOutlierRejector::defaultNew() generated
// ("cv::videostab::NullOutlierRejector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullOutlierRejector_defaultNew_const() -> *mut c_void;
// cv::videostab::NullOutlierRejector::to_IOutlierRejector() generated
// ("cv::videostab::NullOutlierRejector::to_IOutlierRejector", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullOutlierRejector_to_IOutlierRejector(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullOutlierRejector::delete() generated
// ("cv::videostab::NullOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullOutlierRejector_delete(instance: *mut c_void);
// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:98
// ("cv::videostab::NullWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::NullWobbleSuppressor::defaultNew() generated
// ("cv::videostab::NullWobbleSuppressor::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_NullWobbleSuppressor_defaultNew_const() -> *mut c_void;
// cv::videostab::NullWobbleSuppressor::to_WobbleSuppressorBase() generated
// ("cv::videostab::NullWobbleSuppressor::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullWobbleSuppressor_to_WobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::NullWobbleSuppressor::delete() generated
// ("cv::videostab::NullWobbleSuppressor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_NullWobbleSuppressor_delete(instance: *mut c_void);
// OnePassStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:146
// ("cv::videostab::OnePassStabilizer::OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_OnePassStabilizer(ocvrs_return: *mut Result<*mut c_void>);
// setMotionFilter(Ptr<MotionFilterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:148
// ("cv::videostab::OnePassStabilizer::setMotionFilter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::MotionFilterBase>"]), _)]),
pub fn cv_videostab_OnePassStabilizer_setMotionFilter_PtrLMotionFilterBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// motionFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:149
// ("cv::videostab::OnePassStabilizer::motionFilter", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_motionFilter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:151
// ("cv::videostab::OnePassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:152
// ("cv::videostab::OnePassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::OnePassStabilizer::to_IFrameSource() generated
// ("cv::videostab::OnePassStabilizer::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_to_IFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::OnePassStabilizer::to_StabilizerBase() generated
// ("cv::videostab::OnePassStabilizer::to_StabilizerBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_to_StabilizerBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::OnePassStabilizer::delete() generated
// ("cv::videostab::OnePassStabilizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_OnePassStabilizer_delete(instance: *mut c_void);
// PyrLkOptFlowEstimatorBase()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:82
// ("cv::videostab::PyrLkOptFlowEstimatorBase::PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase(ocvrs_return: *mut Result<*mut c_void>);
// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:84
// ("cv::videostab::PyrLkOptFlowEstimatorBase::setWinSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result<()>);
// winSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:85
// ("cv::videostab::PyrLkOptFlowEstimatorBase::winSize", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:87
// ("cv::videostab::PyrLkOptFlowEstimatorBase::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// maxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:88
// ("cv::videostab::PyrLkOptFlowEstimatorBase::maxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::videostab::PyrLkOptFlowEstimatorBase::to_DensePyrLkOptFlowEstimatorGpu() generated
// ("cv::videostab::PyrLkOptFlowEstimatorBase::to_DensePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_to_DensePyrLkOptFlowEstimatorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimator() generated
// ("cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimatorGpu() generated
// ("cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimatorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::PyrLkOptFlowEstimatorBase::delete() generated
// ("cv::videostab::PyrLkOptFlowEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_PyrLkOptFlowEstimatorBase_delete(instance: *mut c_void);
// RansacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:80
// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_RansacParams_RansacParams(ocvrs_return: *mut Result<*mut c_void>);
// RansacParams(int, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:87
// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, ["size", "thresh", "eps", "prob"], ["int", "float", "float", "float"]), _)]),
pub fn cv_videostab_RansacParams_RansacParams_int_float_float_float(size: i32, thresh: f32, eps: f32, prob: f32, ocvrs_return: *mut Result<*mut c_void>);
// niters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:92
// ("cv::videostab::RansacParams::niters", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_RansacParams_niters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// default2dMotion(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:102
// ("cv::videostab::RansacParams::default2dMotion", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_RansacParams_default2dMotion_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::RansacParams::size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:75
// ("cv::videostab::RansacParams::size", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_RansacParams_propSize_const(instance: *const c_void) -> i32;
// cv::videostab::RansacParams::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:75
// ("cv::videostab::RansacParams::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_videostab_RansacParams_propSize_const_int(instance: *mut c_void, val: i32);
// cv::videostab::RansacParams::thresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:76
// ("cv::videostab::RansacParams::thresh", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_RansacParams_propThresh_const(instance: *const c_void) -> f32;
// cv::videostab::RansacParams::setThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:76
// ("cv::videostab::RansacParams::setThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_videostab_RansacParams_propThresh_const_float(instance: *mut c_void, val: f32);
// cv::videostab::RansacParams::eps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:77
// ("cv::videostab::RansacParams::eps", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_RansacParams_propEps_const(instance: *const c_void) -> f32;
// cv::videostab::RansacParams::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:77
// ("cv::videostab::RansacParams::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_videostab_RansacParams_propEps_const_float(instance: *mut c_void, val: f32);
// cv::videostab::RansacParams::prob() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:78
// ("cv::videostab::RansacParams::prob", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_RansacParams_propProb_const(instance: *const c_void) -> f32;
// cv::videostab::RansacParams::setProb(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:78
// ("cv::videostab::RansacParams::setProb", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_videostab_RansacParams_propProb_const_float(instance: *mut c_void, val: f32);
// cv::videostab::RansacParams::delete() generated
// ("cv::videostab::RansacParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_RansacParams_delete(instance: *mut c_void);
// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:100
// ("cv::videostab::SparsePyrLkOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *const c_void, status: *const c_void, errors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::SparsePyrLkOptFlowEstimator::defaultNew() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimator::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimator_defaultNew_const() -> *mut c_void;
// cv::videostab::SparsePyrLkOptFlowEstimator::to_ISparseOptFlowEstimator() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimator::to_ISparseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimator_to_ISparseOptFlowEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::SparsePyrLkOptFlowEstimator::to_PyrLkOptFlowEstimatorBase() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimator::to_PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimator_to_PyrLkOptFlowEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::SparsePyrLkOptFlowEstimator::delete() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimator_delete(instance: *mut c_void);
// SparsePyrLkOptFlowEstimatorGpu()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:111
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::SparsePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_SparsePyrLkOptFlowEstimatorGpu(ocvrs_return: *mut Result<*mut c_void>);
// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:113
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *const c_void, status: *const c_void, errors: *const c_void, ocvrs_return: *mut Result<()>);
// run(const cuda::GpuMat &, const cuda::GpuMat &, const cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:117
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const_GpuMatR_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_GpuMatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *mut c_void, status: *mut c_void, errors: *mut c_void, ocvrs_return: *mut Result<()>);
// run(const cuda::GpuMat &, const cuda::GpuMat &, const cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:120
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const_GpuMatR_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *mut c_void, status: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::SparsePyrLkOptFlowEstimatorGpu::to_ISparseOptFlowEstimator() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::to_ISparseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_to_ISparseOptFlowEstimator(instance: *mut c_void) -> *mut c_void;
// cv::videostab::SparsePyrLkOptFlowEstimatorGpu::to_PyrLkOptFlowEstimatorBase() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::to_PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_to_PyrLkOptFlowEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::SparsePyrLkOptFlowEstimatorGpu::delete() generated
// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_SparsePyrLkOptFlowEstimatorGpu_delete(instance: *mut c_void);
// setLog(Ptr<ILog>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:71
// ("cv::videostab::StabilizerBase::setLog", vec![(pred!(mut, ["ilog"], ["cv::Ptr<cv::videostab::ILog>"]), _)]),
pub fn cv_videostab_StabilizerBase_setLog_PtrLILogG(instance: *mut c_void, ilog: *mut c_void, ocvrs_return: *mut Result<()>);
// log()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:72
// ("cv::videostab::StabilizerBase::log", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_log_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:74
// ("cv::videostab::StabilizerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_StabilizerBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:75
// ("cv::videostab::StabilizerBase::radius", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFrameSource(Ptr<IFrameSource>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:77
// ("cv::videostab::StabilizerBase::setFrameSource", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IFrameSource>"]), _)]),
pub fn cv_videostab_StabilizerBase_setFrameSource_PtrLIFrameSourceG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// frameSource()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:78
// ("cv::videostab::StabilizerBase::frameSource", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_frameSource_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMaskSource(const Ptr<IFrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:80
// ("cv::videostab::StabilizerBase::setMaskSource", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::videostab::IFrameSource>*"]), _)]),
pub fn cv_videostab_StabilizerBase_setMaskSource_const_PtrLIFrameSourceGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// maskSource()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:81
// ("cv::videostab::StabilizerBase::maskSource", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_maskSource_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:83
// ("cv::videostab::StabilizerBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
pub fn cv_videostab_StabilizerBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// motionEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:84
// ("cv::videostab::StabilizerBase::motionEstimator", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_motionEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDeblurer(Ptr<DeblurerBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:86
// ("cv::videostab::StabilizerBase::setDeblurer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::DeblurerBase>"]), _)]),
pub fn cv_videostab_StabilizerBase_setDeblurer_PtrLDeblurerBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// deblurrer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:87
// ("cv::videostab::StabilizerBase::deblurrer", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_deblurrer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:89
// ("cv::videostab::StabilizerBase::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_StabilizerBase_setTrimRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// trimRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:90
// ("cv::videostab::StabilizerBase::trimRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_trimRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setCorrectionForInclusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:92
// ("cv::videostab::StabilizerBase::setCorrectionForInclusion", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// doCorrectionForInclusion()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:93
// ("cv::videostab::StabilizerBase::doCorrectionForInclusion", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_doCorrectionForInclusion_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:95
// ("cv::videostab::StabilizerBase::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_StabilizerBase_setBorderMode_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// borderMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:96
// ("cv::videostab::StabilizerBase::borderMode", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_borderMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInpainter(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:98
// ("cv::videostab::StabilizerBase::setInpainter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
pub fn cv_videostab_StabilizerBase_setInpainter_PtrLInpainterBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// inpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:99
// ("cv::videostab::StabilizerBase::inpainter", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_StabilizerBase_inpainter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::StabilizerBase::to_OnePassStabilizer() generated
// ("cv::videostab::StabilizerBase::to_OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_StabilizerBase_to_OnePassStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::StabilizerBase::to_TwoPassStabilizer() generated
// ("cv::videostab::StabilizerBase::to_TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_StabilizerBase_to_TwoPassStabilizer(instance: *mut c_void) -> *mut c_void;
// cv::videostab::StabilizerBase::delete() generated
// ("cv::videostab::StabilizerBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_StabilizerBase_delete(instance: *mut c_void);
// ToFileMotionWriter(const String &, Ptr<ImageMotionEstimatorBase>)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:212
// ("cv::videostab::ToFileMotionWriter::ToFileMotionWriter", vec![(pred!(mut, ["path", "estimator"], ["const cv::String*", "cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
pub fn cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_PtrLImageMotionEstimatorBaseG(path: *const c_char, estimator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:214
// ("cv::videostab::ToFileMotionWriter::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
pub fn cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result<()>);
// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:215
// ("cv::videostab::ToFileMotionWriter::motionModel", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_ToFileMotionWriter_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:217
// ("cv::videostab::ToFileMotionWriter::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
pub fn cv_videostab_ToFileMotionWriter_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:219
// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
pub fn cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::ToFileMotionWriter::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:219
// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::ToFileMotionWriter::to_ImageMotionEstimatorBase() generated
// ("cv::videostab::ToFileMotionWriter::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ToFileMotionWriter_to_ImageMotionEstimatorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::ToFileMotionWriter::delete() generated
// ("cv::videostab::ToFileMotionWriter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_ToFileMotionWriter_delete(instance: *mut c_void);
// TranslationBasedLocalOutlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:77
// ("cv::videostab::TranslationBasedLocalOutlierRejector::TranslationBasedLocalOutlierRejector", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector(ocvrs_return: *mut Result<*mut c_void>);
// setCellSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:79
// ("cv::videostab::TranslationBasedLocalOutlierRejector::setCellSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result<()>);
// cellSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:80
// ("cv::videostab::TranslationBasedLocalOutlierRejector::cellSize", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setRansacParams(RansacParams)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:82
// ("cv::videostab::TranslationBasedLocalOutlierRejector::setRansacParams", vec![(pred!(mut, ["val"], ["cv::videostab::RansacParams"]), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// ransacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:83
// ("cv::videostab::TranslationBasedLocalOutlierRejector::ransacParams", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:85
// ("cv::videostab::TranslationBasedLocalOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::TranslationBasedLocalOutlierRejector::to_IOutlierRejector() generated
// ("cv::videostab::TranslationBasedLocalOutlierRejector::to_IOutlierRejector", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_to_IOutlierRejector(instance: *mut c_void) -> *mut c_void;
// cv::videostab::TranslationBasedLocalOutlierRejector::delete() generated
// ("cv::videostab::TranslationBasedLocalOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TranslationBasedLocalOutlierRejector_delete(instance: *mut c_void);
// TwoPassStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:166
// ("cv::videostab::TwoPassStabilizer::TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_TwoPassStabilizer(ocvrs_return: *mut Result<*mut c_void>);
// setMotionStabilizer(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:168
// ("cv::videostab::TwoPassStabilizer::setMotionStabilizer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
pub fn cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrLIMotionStabilizerG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// motionStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:169
// ("cv::videostab::TwoPassStabilizer::motionStabilizer", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_motionStabilizer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setWobbleSuppressor(Ptr<WobbleSuppressorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:171
// ("cv::videostab::TwoPassStabilizer::setWobbleSuppressor", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::WobbleSuppressorBase>"]), _)]),
pub fn cv_videostab_TwoPassStabilizer_setWobbleSuppressor_PtrLWobbleSuppressorBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// wobbleSuppressor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:172
// ("cv::videostab::TwoPassStabilizer::wobbleSuppressor", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setEstimateTrimRatio(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:174
// ("cv::videostab::TwoPassStabilizer::setEstimateTrimRatio", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// mustEstimateTrimaRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:175
// ("cv::videostab::TwoPassStabilizer::mustEstimateTrimaRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:177
// ("cv::videostab::TwoPassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:178
// ("cv::videostab::TwoPassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::TwoPassStabilizer::to_IFrameSource() generated
// ("cv::videostab::TwoPassStabilizer::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_to_IFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::TwoPassStabilizer::to_StabilizerBase() generated
// ("cv::videostab::TwoPassStabilizer::to_StabilizerBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_to_StabilizerBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::TwoPassStabilizer::delete() generated
// ("cv::videostab::TwoPassStabilizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_TwoPassStabilizer_delete(instance: *mut c_void);
// VideoFileSource(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:75
// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path", "volatileFrame"], ["const cv::String*", "bool"]), _)]),
pub fn cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(path: *const c_char, volatile_frame: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::VideoFileSource::VideoFileSource(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:75
// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_videostab_VideoFileSource_VideoFileSource_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:77
// ("cv::videostab::VideoFileSource::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:78
// ("cv::videostab::VideoFileSource::nextFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// width()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:80
// ("cv::videostab::VideoFileSource::width", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_width(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// height()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:81
// ("cv::videostab::VideoFileSource::height", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_height(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// count()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:82
// ("cv::videostab::VideoFileSource::count", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_count(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// fps()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:83
// ("cv::videostab::VideoFileSource::fps", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_fps(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// cv::videostab::VideoFileSource::to_IFrameSource() generated
// ("cv::videostab::VideoFileSource::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_to_IFrameSource(instance: *mut c_void) -> *mut c_void;
// cv::videostab::VideoFileSource::delete() generated
// ("cv::videostab::VideoFileSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_VideoFileSource_delete(instance: *mut c_void);
// WeightingDeblurer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:99
// ("cv::videostab::WeightingDeblurer::WeightingDeblurer", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WeightingDeblurer_WeightingDeblurer(ocvrs_return: *mut Result<*mut c_void>);
// setSensitivity(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:101
// ("cv::videostab::WeightingDeblurer::setSensitivity", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_videostab_WeightingDeblurer_setSensitivity_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// sensitivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:102
// ("cv::videostab::WeightingDeblurer::sensitivity", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WeightingDeblurer_sensitivity_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:104
// ("cv::videostab::WeightingDeblurer::deblur", vec![(pred!(mut, ["idx", "frame", "range"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
pub fn cv_videostab_WeightingDeblurer_deblur_int_MatR_const_RangeR(instance: *mut c_void, idx: i32, frame: *mut c_void, range: *const c_void, ocvrs_return: *mut Result<()>);
// cv::videostab::WeightingDeblurer::to_DeblurerBase() generated
// ("cv::videostab::WeightingDeblurer::to_DeblurerBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WeightingDeblurer_to_DeblurerBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::WeightingDeblurer::delete() generated
// ("cv::videostab::WeightingDeblurer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WeightingDeblurer_delete(instance: *mut c_void);
// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:67
// ("cv::videostab::WobbleSuppressorBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result<()>);
// motionEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:68
// ("cv::videostab::WobbleSuppressorBase::motionEstimator", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_motionEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:70
// ("cv::videostab::WobbleSuppressorBase::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// setFrameCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:75
// ("cv::videostab::WobbleSuppressorBase::setFrameCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_setFrameCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// frameCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:76
// ("cv::videostab::WobbleSuppressorBase::frameCount", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_frameCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:78
// ("cv::videostab::WobbleSuppressorBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_setMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:79
// ("cv::videostab::WobbleSuppressorBase::motions", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMotions2(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:81
// ("cv::videostab::WobbleSuppressorBase::setMotions2", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_setMotions2_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// motions2()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:82
// ("cv::videostab::WobbleSuppressorBase::motions2", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_motions2_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:84
// ("cv::videostab::WobbleSuppressorBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vectorLMatGR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:85
// ("cv::videostab::WobbleSuppressorBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor() generated
// ("cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor(instance: *mut c_void) -> *mut c_void;
// cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorBase() generated
// ("cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorBase(instance: *mut c_void) -> *mut c_void;
// cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorGpu() generated
// ("cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorGpu(instance: *mut c_void) -> *mut c_void;
// cv::videostab::WobbleSuppressorBase::to_NullWobbleSuppressor() generated
// ("cv::videostab::WobbleSuppressorBase::to_NullWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_to_NullWobbleSuppressor(instance: *mut c_void) -> *mut c_void;
// cv::videostab::WobbleSuppressorBase::delete() generated
// ("cv::videostab::WobbleSuppressorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_videostab_WobbleSuppressorBase_delete(instance: *mut c_void);
