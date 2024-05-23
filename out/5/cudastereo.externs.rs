// cv::cuda::createDisparityBilateralFilter() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:340
// ("cv::cuda::createDisparityBilateralFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createDisparityBilateralFilter(ocvrs_return: *mut Result<*mut c_void>);
// createDisparityBilateralFilter(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:340
// ("cv::cuda::createDisparityBilateralFilter", vec![(pred!(mut, ["ndisp", "radius", "iters"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_createDisparityBilateralFilter_int_int_int(ndisp: i32, radius: i32, iters: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createStereoBM() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:91
// ("cv::cuda::createStereoBM", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createStereoBM(ocvrs_return: *mut Result<*mut c_void>);
// createStereoBM(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:91
// ("cv::cuda::createStereoBM", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
pub fn cv_cuda_createStereoBM_int_int(num_disparities: i32, block_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createStereoBeliefPropagation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:190
// ("cv::cuda::createStereoBeliefPropagation", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createStereoBeliefPropagation(ocvrs_return: *mut Result<*mut c_void>);
// createStereoBeliefPropagation(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:190
// ("cv::cuda::createStereoBeliefPropagation", vec![(pred!(mut, ["ndisp", "iters", "levels", "msg_type"], ["int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createStereoBeliefPropagation_int_int_int_int(ndisp: i32, iters: i32, levels: i32, msg_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createStereoConstantSpaceBP() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:243
// ("cv::cuda::createStereoConstantSpaceBP", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createStereoConstantSpaceBP(ocvrs_return: *mut Result<*mut c_void>);
// createStereoConstantSpaceBP(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:243
// ("cv::cuda::createStereoConstantSpaceBP", vec![(pred!(mut, ["ndisp", "iters", "levels", "nr_plane", "msg_type"], ["int", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createStereoConstantSpaceBP_int_int_int_int_int(ndisp: i32, iters: i32, levels: i32, nr_plane: i32, msg_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createStereoSGM() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:290
// ("cv::cuda::createStereoSGM", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createStereoSGM(ocvrs_return: *mut Result<*mut c_void>);
// createStereoSGM(int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:290
// ("cv::cuda::createStereoSGM", vec![(pred!(mut, ["minDisparity", "numDisparities", "P1", "P2", "uniquenessRatio", "mode"], ["int", "int", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createStereoSGM_int_int_int_int_int_int(min_disparity: i32, num_disparities: i32, p1: i32, p2: i32, uniqueness_ratio: i32, mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::drawColorDisp(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:378
// ("cv::cuda::drawColorDisp", vec![(pred!(mut, ["src_disp", "dst_disp", "ndisp"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int(src_disp: *const c_void, dst_disp: *const c_void, ndisp: i32, ocvrs_return: *mut Result<()>);
// drawColorDisp(InputArray, OutputArray, int, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:378
// ("cv::cuda::drawColorDisp", vec![(pred!(mut, ["src_disp", "dst_disp", "ndisp", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int_StreamR(src_disp: *const c_void, dst_disp: *const c_void, ndisp: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::reprojectImageTo3D(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:360
// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q"], ["cv::cuda::GpuMat", "cv::cuda::GpuMat*", "cv::Mat"]), _)]),
pub fn cv_cuda_reprojectImageTo3D_GpuMat_GpuMatR_Mat(disp: *mut c_void, xyzw: *mut c_void, q: *mut c_void, ocvrs_return: *mut Result<()>);
// reprojectImageTo3D(GpuMat, GpuMat &, Mat, int, Stream &)(TraitClass, TraitClass, TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:360
// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q", "dst_cn", "stream"], ["cv::cuda::GpuMat", "cv::cuda::GpuMat*", "cv::Mat", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_reprojectImageTo3D_GpuMat_GpuMatR_Mat_int_StreamR(disp: *mut c_void, xyzw: *mut c_void, q: *mut c_void, dst_cn: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:359
// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(disp: *const c_void, xyzw: *const c_void, q: *const c_void, ocvrs_return: *mut Result<()>);
// reprojectImageTo3D(InputArray, OutputArray, InputArray, int, Stream &)(InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:359
// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q", "dst_cn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(disp: *const c_void, xyzw: *const c_void, q: *const c_void, dst_cn: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// apply(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:309
// ("cv::cuda::DisparityBilateralFilter::apply", vec![(pred!(mut, ["disparity", "image", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, disparity: *const c_void, image: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DisparityBilateralFilter::apply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:309
// ("cv::cuda::DisparityBilateralFilter::apply", vec![(pred!(mut, ["disparity", "image", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, disparity: *const c_void, image: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:311
// ("cv::cuda::DisparityBilateralFilter::getNumDisparities", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getNumDisparities_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:312
// ("cv::cuda::DisparityBilateralFilter::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setNumDisparities_int(instance: *mut c_void, num_disparities: i32, ocvrs_return: *mut Result<()>);
// getRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:314
// ("cv::cuda::DisparityBilateralFilter::getRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:315
// ("cv::cuda::DisparityBilateralFilter::setRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:317
// ("cv::cuda::DisparityBilateralFilter::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:318
// ("cv::cuda::DisparityBilateralFilter::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setNumIters_int(instance: *mut c_void, iters: i32, ocvrs_return: *mut Result<()>);
// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:321
// ("cv::cuda::DisparityBilateralFilter::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getEdgeThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:322
// ("cv::cuda::DisparityBilateralFilter::setEdgeThreshold", vec![(pred!(mut, ["edge_threshold"], ["double"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setEdgeThreshold_double(instance: *mut c_void, edge_threshold: f64, ocvrs_return: *mut Result<()>);
// getMaxDiscThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:325
// ("cv::cuda::DisparityBilateralFilter::getMaxDiscThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getMaxDiscThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDiscThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:326
// ("cv::cuda::DisparityBilateralFilter::setMaxDiscThreshold", vec![(pred!(mut, ["max_disc_threshold"], ["double"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setMaxDiscThreshold_double(instance: *mut c_void, max_disc_threshold: f64, ocvrs_return: *mut Result<()>);
// getSigmaRange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:329
// ("cv::cuda::DisparityBilateralFilter::getSigmaRange", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_getSigmaRange_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSigmaRange(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:330
// ("cv::cuda::DisparityBilateralFilter::setSigmaRange", vec![(pred!(mut, ["sigma_range"], ["double"]), _)]),
pub fn cv_cuda_DisparityBilateralFilter_setSigmaRange_double(instance: *mut c_void, sigma_range: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::DisparityBilateralFilter::to_Algorithm() generated
// ("cv::cuda::DisparityBilateralFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DisparityBilateralFilter::delete() generated
// ("cv::cuda::DisparityBilateralFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DisparityBilateralFilter_delete(instance: *mut c_void);
// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:78
// ("cv::cuda::StereoBM::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_StereoBM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, left: *const c_void, right: *const c_void, disparity: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::StereoBM::to_Algorithm() generated
// ("cv::cuda::StereoBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBM::to_StereoBM() generated
// ("cv::cuda::StereoBM::to_StereoBM", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBM_to_StereoBM(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBM::to_StereoMatcher() generated
// ("cv::cuda::StereoBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBM_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBM::delete() generated
// ("cv::cuda::StereoBM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBM_delete(instance: *mut c_void);
// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:135
// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, left: *const c_void, right: *const c_void, disparity: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:146
// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["data", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, data: *const c_void, disparity: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::StereoBeliefPropagation::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:146
// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["data", "disparity"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, data: *const c_void, disparity: *const c_void, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:149
// ("cv::cuda::StereoBeliefPropagation::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:150
// ("cv::cuda::StereoBeliefPropagation::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setNumIters_int(instance: *mut c_void, iters: i32, ocvrs_return: *mut Result<()>);
// getNumLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:153
// ("cv::cuda::StereoBeliefPropagation::getNumLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:154
// ("cv::cuda::StereoBeliefPropagation::setNumLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setNumLevels_int(instance: *mut c_void, levels: i32, ocvrs_return: *mut Result<()>);
// getMaxDataTerm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:157
// ("cv::cuda::StereoBeliefPropagation::getMaxDataTerm", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getMaxDataTerm_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDataTerm(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:158
// ("cv::cuda::StereoBeliefPropagation::setMaxDataTerm", vec![(pred!(mut, ["max_data_term"], ["double"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setMaxDataTerm_double(instance: *mut c_void, max_data_term: f64, ocvrs_return: *mut Result<()>);
// getDataWeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:161
// ("cv::cuda::StereoBeliefPropagation::getDataWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getDataWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDataWeight(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:162
// ("cv::cuda::StereoBeliefPropagation::setDataWeight", vec![(pred!(mut, ["data_weight"], ["double"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setDataWeight_double(instance: *mut c_void, data_weight: f64, ocvrs_return: *mut Result<()>);
// getMaxDiscTerm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:165
// ("cv::cuda::StereoBeliefPropagation::getMaxDiscTerm", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getMaxDiscTerm_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDiscTerm(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:166
// ("cv::cuda::StereoBeliefPropagation::setMaxDiscTerm", vec![(pred!(mut, ["max_disc_term"], ["double"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setMaxDiscTerm_double(instance: *mut c_void, max_disc_term: f64, ocvrs_return: *mut Result<()>);
// getDiscSingleJump()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:169
// ("cv::cuda::StereoBeliefPropagation::getDiscSingleJump", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getDiscSingleJump_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDiscSingleJump(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:170
// ("cv::cuda::StereoBeliefPropagation::setDiscSingleJump", vec![(pred!(mut, ["disc_single_jump"], ["double"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setDiscSingleJump_double(instance: *mut c_void, disc_single_jump: f64, ocvrs_return: *mut Result<()>);
// getMsgType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:173
// ("cv::cuda::StereoBeliefPropagation::getMsgType", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_getMsgType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMsgType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:174
// ("cv::cuda::StereoBeliefPropagation::setMsgType", vec![(pred!(mut, ["msg_type"], ["int"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_setMsgType_int(instance: *mut c_void, msg_type: i32, ocvrs_return: *mut Result<()>);
// estimateRecommendedParams(int, int, int &, int &, int &)(Primitive, Primitive, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:179
// ("cv::cuda::StereoBeliefPropagation::estimateRecommendedParams", vec![(pred!(mut, ["width", "height", "ndisp", "iters", "levels"], ["int", "int", "int*", "int*", "int*"]), _)]),
pub fn cv_cuda_StereoBeliefPropagation_estimateRecommendedParams_int_int_intR_intR_intR(width: i32, height: i32, ndisp: *mut i32, iters: *mut i32, levels: *mut i32, ocvrs_return: *mut Result<()>);
// cv::cuda::StereoBeliefPropagation::to_CUDA_StereoConstantSpaceBP() generated
// ("cv::cuda::StereoBeliefPropagation::to_CUDA_StereoConstantSpaceBP", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_to_CUDA_StereoConstantSpaceBP(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBeliefPropagation::to_Algorithm() generated
// ("cv::cuda::StereoBeliefPropagation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBeliefPropagation::to_StereoMatcher() generated
// ("cv::cuda::StereoBeliefPropagation::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoBeliefPropagation::delete() generated
// ("cv::cuda::StereoBeliefPropagation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoBeliefPropagation_delete(instance: *mut c_void);
// getNrPlane()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:222
// ("cv::cuda::StereoConstantSpaceBP::getNrPlane", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_getNrPlane_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNrPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:223
// ("cv::cuda::StereoConstantSpaceBP::setNrPlane", vec![(pred!(mut, ["nr_plane"], ["int"]), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_setNrPlane_int(instance: *mut c_void, nr_plane: i32, ocvrs_return: *mut Result<()>);
// getUseLocalInitDataCost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:225
// ("cv::cuda::StereoConstantSpaceBP::getUseLocalInitDataCost", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_getUseLocalInitDataCost_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseLocalInitDataCost(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:226
// ("cv::cuda::StereoConstantSpaceBP::setUseLocalInitDataCost", vec![(pred!(mut, ["use_local_init_data_cost"], ["bool"]), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_setUseLocalInitDataCost_bool(instance: *mut c_void, use_local_init_data_cost: bool, ocvrs_return: *mut Result<()>);
// estimateRecommendedParams(int, int, int &, int &, int &, int &)(Primitive, Primitive, Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:231
// ("cv::cuda::StereoConstantSpaceBP::estimateRecommendedParams", vec![(pred!(mut, ["width", "height", "ndisp", "iters", "levels", "nr_plane"], ["int", "int", "int*", "int*", "int*", "int*"]), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_estimateRecommendedParams_int_int_intR_intR_intR_intR(width: i32, height: i32, ndisp: *mut i32, iters: *mut i32, levels: *mut i32, nr_plane: *mut i32, ocvrs_return: *mut Result<()>);
// cv::cuda::StereoConstantSpaceBP::to_Algorithm() generated
// ("cv::cuda::StereoConstantSpaceBP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoConstantSpaceBP::to_CUDA_StereoBeliefPropagation() generated
// ("cv::cuda::StereoConstantSpaceBP::to_CUDA_StereoBeliefPropagation", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_to_CUDA_StereoBeliefPropagation(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoConstantSpaceBP::to_StereoMatcher() generated
// ("cv::cuda::StereoConstantSpaceBP::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoConstantSpaceBP::delete() generated
// ("cv::cuda::StereoConstantSpaceBP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoConstantSpaceBP_delete(instance: *mut c_void);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:269
// ("cv::cuda::StereoSGM::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_StereoSGM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, left: *const c_void, right: *const c_void, disparity: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:275
// ("cv::cuda::StereoSGM::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_StereoSGM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, left: *const c_void, right: *const c_void, disparity: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::StereoSGM::to_Algorithm() generated
// ("cv::cuda::StereoSGM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoSGM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoSGM::to_StereoMatcher() generated
// ("cv::cuda::StereoSGM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoSGM_to_StereoMatcher(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoSGM::to_StereoSGBM() generated
// ("cv::cuda::StereoSGM::to_StereoSGBM", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoSGM_to_StereoSGBM(instance: *mut c_void) -> *mut c_void;
// cv::cuda::StereoSGM::delete() generated
// ("cv::cuda::StereoSGM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_StereoSGM_delete(instance: *mut c_void);
