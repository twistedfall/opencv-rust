// cv::cuda::calcOpticalFlowBM(TraitClass, TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:194
// ("cv::cuda::calcOpticalFlowBM", vec![(pred!(mut, ["prev", "curr", "block_size", "shift_size", "max_range", "use_previous", "velx", "vely", "buf"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::Size", "cv::Size", "cv::Size", "bool", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR(prev: *const c_void, curr: *const c_void, block_size: *const core::Size, shift_size: *const core::Size, max_range: *const core::Size, use_previous: bool, velx: *mut c_void, vely: *mut c_void, buf: *mut c_void, ocvrs_return: *mut Result<()>);
// calcOpticalFlowBM(const GpuMat &, const GpuMat &, Size, Size, Size, bool, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:194
// ("cv::cuda::calcOpticalFlowBM", vec![(pred!(mut, ["prev", "curr", "block_size", "shift_size", "max_range", "use_previous", "velx", "vely", "buf", "stream"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::Size", "cv::Size", "cv::Size", "bool", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR_StreamR(prev: *const c_void, curr: *const c_void, block_size: *const core::Size, shift_size: *const core::Size, max_range: *const core::Size, use_previous: bool, velx: *mut c_void, vely: *mut c_void, buf: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::connectivityMask(TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:249
// ("cv::cuda::connectivityMask", vec![(pred!(mut, ["image", "mask", "lo", "hi"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "const cv::Scalar*", "const cv::Scalar*"]), _)]),
pub fn cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR(image: *const c_void, mask: *mut c_void, lo: *const core::Scalar, hi: *const core::Scalar, ocvrs_return: *mut Result<()>);
// connectivityMask(const GpuMat &, GpuMat &, const cv::Scalar &, const cv::Scalar &, Stream &)(TraitClass, TraitClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:249
// ("cv::cuda::connectivityMask", vec![(pred!(mut, ["image", "mask", "lo", "hi", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "const cv::Scalar*", "const cv::Scalar*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR_StreamR(image: *const c_void, mask: *mut c_void, lo: *const core::Scalar, hi: *const core::Scalar, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::createBackgroundSubtractorFGD() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:187
// ("cv::cuda::createBackgroundSubtractorFGD", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createBackgroundSubtractorFGD(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorFGD(const FGDParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:187
// ("cv::cuda::createBackgroundSubtractorFGD", vec![(pred!(mut, ["params"], ["const cv::cuda::FGDParams*"]), _)]),
pub fn cv_cuda_createBackgroundSubtractorFGD_const_FGDParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createBackgroundSubtractorGMG() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:131
// ("cv::cuda::createBackgroundSubtractorGMG", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createBackgroundSubtractorGMG(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorGMG(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:131
// ("cv::cuda::createBackgroundSubtractorGMG", vec![(pred!(mut, ["initializationFrames", "decisionThreshold"], ["int", "double"]), _)]),
pub fn cv_cuda_createBackgroundSubtractorGMG_int_double(initialization_frames: i32, decision_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createImagePyramid(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:76
// ("cv::cuda::createImagePyramid", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_createImagePyramid_const__InputArrayR(img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createImagePyramid(InputArray, int, Stream &)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:76
// ("cv::cuda::createImagePyramid", vec![(pred!(mut, ["img", "nLayers", "stream"], ["const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_createImagePyramid_const__InputArrayR_int_StreamR(img: *const c_void, n_layers: i32, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createOpticalFlowNeedleMap(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:232
// ("cv::cuda::createOpticalFlowNeedleMap", vec![(pred!(mut, ["u", "v", "vertex", "colors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_createOpticalFlowNeedleMap_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(u: *const c_void, v: *const c_void, vertex: *mut c_void, colors: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::graphcut(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:239
// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "bottom", "labels", "buf"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(terminals: *mut c_void, left_transp: *mut c_void, right_transp: *mut c_void, top: *mut c_void, bottom: *mut c_void, labels: *mut c_void, buf: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::graphcut(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:243
// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "topLeft", "topRight", "bottom", "bottomLeft", "bottomRight", "labels", "buf"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(terminals: *mut c_void, left_transp: *mut c_void, right_transp: *mut c_void, top: *mut c_void, top_left: *mut c_void, top_right: *mut c_void, bottom: *mut c_void, bottom_left: *mut c_void, bottom_right: *mut c_void, labels: *mut c_void, buf: *mut c_void, ocvrs_return: *mut Result<()>);
// graphcut(GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:243
// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "topLeft", "topRight", "bottom", "bottomLeft", "bottomRight", "labels", "buf", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(terminals: *mut c_void, left_transp: *mut c_void, right_transp: *mut c_void, top: *mut c_void, top_left: *mut c_void, top_right: *mut c_void, bottom: *mut c_void, bottom_left: *mut c_void, bottom_right: *mut c_void, labels: *mut c_void, buf: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// graphcut(GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:239
// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "bottom", "labels", "buf", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(terminals: *mut c_void, left_transp: *mut c_void, right_transp: *mut c_void, top: *mut c_void, bottom: *mut c_void, labels: *mut c_void, buf: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::interpolateFrames(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:226
// ("cv::cuda::interpolateFrames", vec![(pred!(mut, ["frame0", "frame1", "fu", "fv", "bu", "bv", "pos", "newFrame", "buf"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "float", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR(frame0: *const c_void, frame1: *const c_void, fu: *const c_void, fv: *const c_void, bu: *const c_void, bv: *const c_void, pos: f32, new_frame: *mut c_void, buf: *mut c_void, ocvrs_return: *mut Result<()>);
// interpolateFrames(const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, float, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:226
// ("cv::cuda::interpolateFrames", vec![(pred!(mut, ["frame0", "frame1", "fu", "fv", "bu", "bv", "pos", "newFrame", "buf", "stream"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "float", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR_StreamR(frame0: *const c_void, frame1: *const c_void, fu: *const c_void, fv: *const c_void, bu: *const c_void, bv: *const c_void, pos: f32, new_frame: *mut c_void, buf: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::labelComponents(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:252
// ("cv::cuda::labelComponents", vec![(pred!(mut, ["mask", "components"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_labelComponents_const_GpuMatR_GpuMatR(mask: *const c_void, components: *mut c_void, ocvrs_return: *mut Result<()>);
// labelComponents(const GpuMat &, GpuMat &, int, Stream &)(TraitClass, TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:252
// ("cv::cuda::labelComponents", vec![(pred!(mut, ["mask", "components", "flags", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_labelComponents_const_GpuMatR_GpuMatR_int_StreamR(mask: *const c_void, components: *mut c_void, flags: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::projectPoints(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:261
// ("cv::cuda::projectPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "camera_mat", "dist_coef", "dst"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR(src: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_mat: *const c_void, dist_coef: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// projectPoints(const GpuMat &, const Mat &, const Mat &, const Mat &, const Mat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:261
// ("cv::cuda::projectPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "camera_mat", "dist_coef", "dst", "stream"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR_StreamR(src: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_mat: *const c_void, dist_coef: *const c_void, dst: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::solvePnPRansac(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:281
// ("cv::cuda::solvePnPRansac", vec![(pred!(mut, ["object", "image", "camera_mat", "dist_coef", "rvec", "tvec"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR(object: *const c_void, image: *const c_void, camera_mat: *const c_void, dist_coef: *const c_void, rvec: *mut c_void, tvec: *mut c_void, ocvrs_return: *mut Result<()>);
// solvePnPRansac(const Mat &, const Mat &, const Mat &, const Mat &, Mat &, Mat &, bool, int, float, int, std::vector<int> *)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:281
// ("cv::cuda::solvePnPRansac", vec![(pred!(mut, ["object", "image", "camera_mat", "dist_coef", "rvec", "tvec", "use_extrinsic_guess", "num_iters", "max_dist", "min_inlier_count", "inliers"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::Mat*", "cv::Mat*", "bool", "int", "float", "int", "std::vector<int>*"]), _)]),
pub fn cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR_bool_int_float_int_vectorLintGX(object: *const c_void, image: *const c_void, camera_mat: *const c_void, dist_coef: *const c_void, rvec: *mut c_void, tvec: *mut c_void, use_extrinsic_guess: bool, num_iters: i32, max_dist: f32, min_inlier_count: i32, inliers: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::transformPoints(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:258
// ("cv::cuda::transformPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "dst"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR(src: *const c_void, rvec: *const c_void, tvec: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// transformPoints(const GpuMat &, const Mat &, const Mat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:258
// ("cv::cuda::transformPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "dst", "stream"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR_StreamR(src: *const c_void, rvec: *const c_void, tvec: *const c_void, dst: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getForegroundRegions(OutputArrayOfArrays)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:151
// ("cv::cuda::BackgroundSubtractorFGD::getForegroundRegions", vec![(pred!(mut, ["foreground_regions"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorFGD_getForegroundRegions_const__OutputArrayR(instance: *mut c_void, foreground_regions: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::BackgroundSubtractorFGD::to_Algorithm() generated
// ("cv::cuda::BackgroundSubtractorFGD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorFGD_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorFGD::to_BackgroundSubtractor() generated
// ("cv::cuda::BackgroundSubtractorFGD::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorFGD_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorFGD::delete() generated
// ("cv::cuda::BackgroundSubtractorFGD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorFGD_delete(instance: *mut c_void);
// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:92
// ("cv::cuda::BackgroundSubtractorGMG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:94
// ("cv::cuda::BackgroundSubtractorGMG::getMaxFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:95
// ("cv::cuda::BackgroundSubtractorGMG::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getDefaultLearningRate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:97
// ("cv::cuda::BackgroundSubtractorGMG::getDefaultLearningRate", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getDefaultLearningRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDefaultLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:98
// ("cv::cuda::BackgroundSubtractorGMG::setDefaultLearningRate", vec![(pred!(mut, ["lr"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setDefaultLearningRate_double(instance: *mut c_void, lr: f64, ocvrs_return: *mut Result<()>);
// getNumFrames()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:100
// ("cv::cuda::BackgroundSubtractorGMG::getNumFrames", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getNumFrames_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumFrames(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:101
// ("cv::cuda::BackgroundSubtractorGMG::setNumFrames", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setNumFrames_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result<()>);
// getQuantizationLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:103
// ("cv::cuda::BackgroundSubtractorGMG::getQuantizationLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getQuantizationLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setQuantizationLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:104
// ("cv::cuda::BackgroundSubtractorGMG::setQuantizationLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setQuantizationLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result<()>);
// getBackgroundPrior()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:106
// ("cv::cuda::BackgroundSubtractorGMG::getBackgroundPrior", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getBackgroundPrior_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundPrior(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:107
// ("cv::cuda::BackgroundSubtractorGMG::setBackgroundPrior", vec![(pred!(mut, ["bgprior"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setBackgroundPrior_double(instance: *mut c_void, bgprior: f64, ocvrs_return: *mut Result<()>);
// getSmoothingRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:109
// ("cv::cuda::BackgroundSubtractorGMG::getSmoothingRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getSmoothingRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSmoothingRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:110
// ("cv::cuda::BackgroundSubtractorGMG::setSmoothingRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setSmoothingRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result<()>);
// getDecisionThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:112
// ("cv::cuda::BackgroundSubtractorGMG::getDecisionThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getDecisionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDecisionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:113
// ("cv::cuda::BackgroundSubtractorGMG::setDecisionThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setDecisionThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result<()>);
// getUpdateBackgroundModel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:115
// ("cv::cuda::BackgroundSubtractorGMG::getUpdateBackgroundModel", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUpdateBackgroundModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:116
// ("cv::cuda::BackgroundSubtractorGMG::setUpdateBackgroundModel", vec![(pred!(mut, ["update"], ["bool"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(instance: *mut c_void, update: bool, ocvrs_return: *mut Result<()>);
// getMinVal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:118
// ("cv::cuda::BackgroundSubtractorGMG::getMinVal", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getMinVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:119
// ("cv::cuda::BackgroundSubtractorGMG::setMinVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setMinVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxVal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:121
// ("cv::cuda::BackgroundSubtractorGMG::getMaxVal", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_getMaxVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:122
// ("cv::cuda::BackgroundSubtractorGMG::setMaxVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_setMaxVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::BackgroundSubtractorGMG::to_Algorithm() generated
// ("cv::cuda::BackgroundSubtractorGMG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorGMG::to_BackgroundSubtractor() generated
// ("cv::cuda::BackgroundSubtractorGMG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorGMG::delete() generated
// ("cv::cuda::BackgroundSubtractorGMG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorGMG_delete(instance: *mut c_void);
// FGDParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:179
// ("cv::cuda::FGDParams::FGDParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FGDParams_FGDParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::FGDParams::Lc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:156
// ("cv::cuda::FGDParams::Lc", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propLc_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setLc(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:156
// ("cv::cuda::FGDParams::setLc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propLc_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::N1c() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:157
// ("cv::cuda::FGDParams::N1c", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propN1c_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setN1c(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:157
// ("cv::cuda::FGDParams::setN1c", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propN1c_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::N2c() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:158
// ("cv::cuda::FGDParams::N2c", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propN2c_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setN2c(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:158
// ("cv::cuda::FGDParams::setN2c", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propN2c_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::Lcc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:161
// ("cv::cuda::FGDParams::Lcc", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propLcc_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setLcc(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:161
// ("cv::cuda::FGDParams::setLcc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propLcc_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::N1cc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:162
// ("cv::cuda::FGDParams::N1cc", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propN1cc_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setN1cc(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:162
// ("cv::cuda::FGDParams::setN1cc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propN1cc_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::N2cc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:163
// ("cv::cuda::FGDParams::N2cc", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propN2cc_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setN2cc(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:163
// ("cv::cuda::FGDParams::setN2cc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propN2cc_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::is_obj_without_holes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:166
// ("cv::cuda::FGDParams::is_obj_without_holes", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propIs_obj_without_holes_const(instance: *const c_void) -> bool;
// cv::cuda::FGDParams::setIs_obj_without_holes(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:166
// ("cv::cuda::FGDParams::setIs_obj_without_holes", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_cuda_FGDParams_propIs_obj_without_holes_const_bool(instance: *mut c_void, val: bool);
// cv::cuda::FGDParams::perform_morphing() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:167
// ("cv::cuda::FGDParams::perform_morphing", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propPerform_morphing_const(instance: *const c_void) -> i32;
// cv::cuda::FGDParams::setPerform_morphing(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:167
// ("cv::cuda::FGDParams::setPerform_morphing", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_FGDParams_propPerform_morphing_const_int(instance: *mut c_void, val: i32);
// cv::cuda::FGDParams::alpha1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:170
// ("cv::cuda::FGDParams::alpha1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propAlpha1_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setAlpha1(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:170
// ("cv::cuda::FGDParams::setAlpha1", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propAlpha1_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::alpha2() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:171
// ("cv::cuda::FGDParams::alpha2", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propAlpha2_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setAlpha2(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:171
// ("cv::cuda::FGDParams::setAlpha2", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propAlpha2_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::alpha3() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:172
// ("cv::cuda::FGDParams::alpha3", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propAlpha3_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setAlpha3(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:172
// ("cv::cuda::FGDParams::setAlpha3", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propAlpha3_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::delta() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:174
// ("cv::cuda::FGDParams::delta", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propDelta_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setDelta(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:174
// ("cv::cuda::FGDParams::setDelta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propDelta_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::T() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:175
// ("cv::cuda::FGDParams::T", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propT_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setT(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:175
// ("cv::cuda::FGDParams::setT", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propT_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::minArea() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:176
// ("cv::cuda::FGDParams::minArea", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FGDParams_propMinArea_const(instance: *const c_void) -> f32;
// cv::cuda::FGDParams::setMinArea(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:176
// ("cv::cuda::FGDParams::setMinArea", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_FGDParams_propMinArea_const_float(instance: *mut c_void, val: f32);
// cv::cuda::FGDParams::delete() generated
// ("cv::cuda::FGDParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FGDParams_delete(instance: *mut c_void);
// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, int, int, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:202
// ("cv::cuda::FastOpticalFlowBM::operator()", vec![(pred!(mut, ["I0", "I1", "flowx", "flowy", "search_window", "block_window", "s"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_int_int_StreamR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flowx: *mut c_void, flowy: *mut c_void, search_window: i32, block_window: i32, s: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::FastOpticalFlowBM::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:202
// ("cv::cuda::FastOpticalFlowBM::operator()", vec![(pred!(mut, ["I0", "I1", "flowx", "flowy"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flowx: *mut c_void, flowy: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::FastOpticalFlowBM::defaultNew() generated
// ("cv::cuda::FastOpticalFlowBM::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FastOpticalFlowBM_defaultNew_const() -> *mut c_void;
// cv::cuda::FastOpticalFlowBM::delete() generated
// ("cv::cuda::FastOpticalFlowBM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastOpticalFlowBM_delete(instance: *mut c_void);
// getLayer(OutputArray, Size, Stream &)(OutputArray, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:73
// ("cv::cuda::ImagePyramid::getLayer", vec![(pred!(const, ["outImg", "outRoi", "stream"], ["const cv::_OutputArray*", "cv::Size", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size_StreamR(instance: *const c_void, out_img: *const c_void, out_roi: *const core::Size, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::ImagePyramid::getLayer(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudalegacy.hpp:73
// ("cv::cuda::ImagePyramid::getLayer", vec![(pred!(const, ["outImg", "outRoi"], ["const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size(instance: *const c_void, out_img: *const c_void, out_roi: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::cuda::ImagePyramid::to_Algorithm() generated
// ("cv::cuda::ImagePyramid::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ImagePyramid_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::ImagePyramid::delete() generated
// ("cv::cuda::ImagePyramid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ImagePyramid_delete(instance: *mut c_void);
