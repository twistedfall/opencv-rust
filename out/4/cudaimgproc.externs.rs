// cv::cuda::alphaComp(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:188
// ("cv::cuda::alphaComp", vec![(pred!(mut, ["img1", "img2", "dst", "alpha_op"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(img1: *const c_void, img2: *const c_void, dst: *const c_void, alpha_op: i32, ocvrs_return: *mut Result<()>);
// alphaComp(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:188
// ("cv::cuda::alphaComp", vec![(pred!(mut, ["img1", "img2", "dst", "alpha_op", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(img1: *const c_void, img2: *const c_void, dst: *const c_void, alpha_op: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bilateralFilter(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:724
// ("cv::cuda::bilateralFilter", vec![(pred!(mut, ["src", "dst", "kernel_size", "sigma_color", "sigma_spatial"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float"]), _)]),
pub fn cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float(src: *const c_void, dst: *const c_void, kernel_size: i32, sigma_color: f32, sigma_spatial: f32, ocvrs_return: *mut Result<()>);
// bilateralFilter(InputArray, OutputArray, int, float, float, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:724
// ("cv::cuda::bilateralFilter", vec![(pred!(mut, ["src", "dst", "kernel_size", "sigma_color", "sigma_spatial", "borderMode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float_int_StreamR(src: *const c_void, dst: *const c_void, kernel_size: i32, sigma_color: f32, sigma_spatial: f32, border_mode: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:740
// ("cv::cuda::blendLinear", vec![(pred!(mut, ["img1", "img2", "weights1", "weights2", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(img1: *const c_void, img2: *const c_void, weights1: *const c_void, weights2: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:740
// ("cv::cuda::blendLinear", vec![(pred!(mut, ["img1", "img2", "weights1", "weights2", "result", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(img1: *const c_void, img2: *const c_void, weights1: *const c_void, weights2: *const c_void, result: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcHist(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:212
// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "mask", "hist"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, mask: *const c_void, hist: *const c_void, ocvrs_return: *mut Result<()>);
// calcHist(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:212
// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "mask", "hist", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, mask: *const c_void, hist: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcHist(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:203
// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "hist"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR(src: *const c_void, hist: *const c_void, ocvrs_return: *mut Result<()>);
// calcHist(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:203
// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "hist", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, hist: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::connectedComponents(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:783
// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR(image: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<()>);
// connectedComponents(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:783
// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ocvrs_return: *mut Result<()>);
// connectedComponents(InputArray, OutputArray, int, int, cv::cuda::ConnectedComponentsAlgorithmsTypes)(InputArray, OutputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:772
// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::ConnectedComponentsAlgorithmsTypes"]), _)]),
pub fn cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_ConnectedComponentsAlgorithmsTypes(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ccltype: crate::cudaimgproc::CUDA_ConnectedComponentsAlgorithmsTypes, ocvrs_return: *mut Result<()>);
// convertSpatialMoments(Mat, const MomentsOrder, const int)(TraitClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:819
// ("cv::cuda::convertSpatialMoments", vec![(pred!(mut, ["spatialMoments", "order", "momentsType"], ["cv::Mat", "const cv::cuda::MomentsOrder", "const int"]), _)]),
pub fn cv_cuda_convertSpatialMoments_Mat_const_MomentsOrder_const_int(spatial_moments: *mut c_void, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32, ocvrs_return: *mut Result<core::Moments>);
// cv::cuda::createCLAHE() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:245
// ("cv::cuda::createCLAHE", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createCLAHE(ocvrs_return: *mut Result<*mut c_void>);
// createCLAHE(double, Size)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:245
// ("cv::cuda::createCLAHE", vec![(pred!(mut, ["clipLimit", "tileGridSize"], ["double", "cv::Size"]), _)]),
pub fn cv_cuda_createCLAHE_double_Size(clip_limit: f64, tile_grid_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createCannyEdgeDetector(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:330
// ("cv::cuda::createCannyEdgeDetector", vec![(pred!(mut, ["low_thresh", "high_thresh"], ["double", "double"]), _)]),
pub fn cv_cuda_createCannyEdgeDetector_double_double(low_thresh: f64, high_thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// createCannyEdgeDetector(double, double, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:330
// ("cv::cuda::createCannyEdgeDetector", vec![(pred!(mut, ["low_thresh", "high_thresh", "apperture_size", "L2gradient"], ["double", "double", "int", "bool"]), _)]),
pub fn cv_cuda_createCannyEdgeDetector_double_double_int_bool(low_thresh: f64, high_thresh: f64, apperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
// createGeneralizedHoughBallard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:510
// ("cv::cuda::createGeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createGeneralizedHoughBallard(ocvrs_return: *mut Result<*mut c_void>);
// createGeneralizedHoughGuil()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:514
// ("cv::cuda::createGeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createGeneralizedHoughGuil(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createGoodFeaturesToTrackDetector(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:604
// ("cv::cuda::createGoodFeaturesToTrackDetector", vec![(pred!(mut, ["srcType"], ["int"]), _)]),
pub fn cv_cuda_createGoodFeaturesToTrackDetector_int(src_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// createGoodFeaturesToTrackDetector(int, int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:604
// ("cv::cuda::createGoodFeaturesToTrackDetector", vec![(pred!(mut, ["srcType", "maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "harrisK"], ["int", "int", "double", "double", "int", "bool", "double"]), _)]),
pub fn cv_cuda_createGoodFeaturesToTrackDetector_int_int_double_double_int_bool_double(src_type: i32, max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, harris_k: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createHarrisCorner(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:549
// ("cv::cuda::createHarrisCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "k"], ["int", "int", "int", "double"]), _)]),
pub fn cv_cuda_createHarrisCorner_int_int_int_double(src_type: i32, block_size: i32, ksize: i32, k: f64, ocvrs_return: *mut Result<*mut c_void>);
// createHarrisCorner(int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:549
// ("cv::cuda::createHarrisCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "k", "borderType"], ["int", "int", "int", "double", "int"]), _)]),
pub fn cv_cuda_createHarrisCorner_int_int_int_double_int(src_type: i32, block_size: i32, ksize: i32, k: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createHoughCirclesDetector(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:503
// ("cv::cuda::createHoughCirclesDetector", vec![(pred!(mut, ["dp", "minDist", "cannyThreshold", "votesThreshold", "minRadius", "maxRadius"], ["float", "float", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int(dp: f32, min_dist: f32, canny_threshold: i32, votes_threshold: i32, min_radius: i32, max_radius: i32, ocvrs_return: *mut Result<*mut c_void>);
// createHoughCirclesDetector(float, float, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:503
// ("cv::cuda::createHoughCirclesDetector", vec![(pred!(mut, ["dp", "minDist", "cannyThreshold", "votesThreshold", "minRadius", "maxRadius", "maxCircles"], ["float", "float", "int", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int_int(dp: f32, min_dist: f32, canny_threshold: i32, votes_threshold: i32, min_radius: i32, max_radius: i32, max_circles: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createHoughLinesDetector(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:392
// ("cv::cuda::createHoughLinesDetector", vec![(pred!(mut, ["rho", "theta", "threshold"], ["float", "float", "int"]), _)]),
pub fn cv_cuda_createHoughLinesDetector_float_float_int(rho: f32, theta: f32, threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// createHoughLinesDetector(float, float, int, bool, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:392
// ("cv::cuda::createHoughLinesDetector", vec![(pred!(mut, ["rho", "theta", "threshold", "doSort", "maxLines"], ["float", "float", "int", "bool", "int"]), _)]),
pub fn cv_cuda_createHoughLinesDetector_float_float_int_bool_int(rho: f32, theta: f32, threshold: i32, do_sort: bool, max_lines: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createHoughSegmentDetector(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:444
// ("cv::cuda::createHoughSegmentDetector", vec![(pred!(mut, ["rho", "theta", "minLineLength", "maxLineGap"], ["float", "float", "int", "int"]), _)]),
pub fn cv_cuda_createHoughSegmentDetector_float_float_int_int(rho: f32, theta: f32, min_line_length: i32, max_line_gap: i32, ocvrs_return: *mut Result<*mut c_void>);
// createHoughSegmentDetector(float, float, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:444
// ("cv::cuda::createHoughSegmentDetector", vec![(pred!(mut, ["rho", "theta", "minLineLength", "maxLineGap", "maxLines", "threshold"], ["float", "float", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createHoughSegmentDetector_float_float_int_int_int_int(rho: f32, theta: f32, min_line_length: i32, max_line_gap: i32, max_lines: i32, threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createMinEigenValCorner(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:562
// ("cv::cuda::createMinEigenValCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_createMinEigenValCorner_int_int_int(src_type: i32, block_size: i32, ksize: i32, ocvrs_return: *mut Result<*mut c_void>);
// createMinEigenValCorner(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:562
// ("cv::cuda::createMinEigenValCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "borderType"], ["int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createMinEigenValCorner_int_int_int_int(src_type: i32, block_size: i32, ksize: i32, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createTemplateMatching(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:706
// ("cv::cuda::createTemplateMatching", vec![(pred!(mut, ["srcType", "method"], ["int", "int"]), _)]),
pub fn cv_cuda_createTemplateMatching_int_int(src_type: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// createTemplateMatching(int, int, Size)(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:706
// ("cv::cuda::createTemplateMatching", vec![(pred!(mut, ["srcType", "method", "user_block_size"], ["int", "int", "cv::Size"]), _)]),
pub fn cv_cuda_createTemplateMatching_int_int_Size(src_type: i32, method: i32, user_block_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::cvtColor(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:91
// ("cv::cuda::cvtColor", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result<()>);
// cvtColor(InputArray, OutputArray, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:91
// ("cv::cuda::cvtColor", vec![(pred!(mut, ["src", "dst", "code", "dcn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src: *const c_void, dst: *const c_void, code: i32, dcn: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::demosaicing(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:137
// ("cv::cuda::demosaicing", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result<()>);
// demosaicing(InputArray, OutputArray, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:137
// ("cv::cuda::demosaicing", vec![(pred!(mut, ["src", "dst", "code", "dcn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int_int_StreamR(src: *const c_void, dst: *const c_void, code: i32, dcn: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::equalizeHist(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:222
// ("cv::cuda::equalizeHist", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// equalizeHist(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:222
// ("cv::cuda::equalizeHist", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::evenLevels(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:255
// ("cv::cuda::evenLevels", vec![(pred!(mut, ["levels", "nLevels", "lowerLevel", "upperLevel"], ["const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_cuda_evenLevels_const__OutputArrayR_int_int_int(levels: *const c_void, n_levels: i32, lower_level: i32, upper_level: i32, ocvrs_return: *mut Result<()>);
// evenLevels(OutputArray, int, int, int, Stream &)(OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:255
// ("cv::cuda::evenLevels", vec![(pred!(mut, ["levels", "nLevels", "lowerLevel", "upperLevel", "stream"], ["const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_evenLevels_const__OutputArrayR_int_int_int_StreamR(levels: *const c_void, n_levels: i32, lower_level: i32, upper_level: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::gammaCorrection(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:158
// ("cv::cuda::gammaCorrection", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// gammaCorrection(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:158
// ("cv::cuda::gammaCorrection", vec![(pred!(mut, ["src", "dst", "forward", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR_bool_StreamR(src: *const c_void, dst: *const c_void, forward: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::histEven(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:267
// ("cv::cuda::histEven", vec![(pred!(mut, ["src", "hist", "histSize", "lowerLevel", "upperLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, hist: *const c_void, hist_size: i32, lower_level: i32, upper_level: i32, ocvrs_return: *mut Result<()>);
// histEven(InputArray, OutputArray, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:267
// ("cv::cuda::histEven", vec![(pred!(mut, ["src", "hist", "histSize", "lowerLevel", "upperLevel", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(src: *const c_void, hist: *const c_void, hist_size: i32, lower_level: i32, upper_level: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::histRange(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:279
// ("cv::cuda::histRange", vec![(pred!(mut, ["src", "hist", "levels"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, hist: *const c_void, levels: *const c_void, ocvrs_return: *mut Result<()>);
// histRange(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:279
// ("cv::cuda::histRange", vec![(pred!(mut, ["src", "hist", "levels", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, hist: *const c_void, levels: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::meanShiftFiltering(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:625
// ("cv::cuda::meanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, sp: i32, sr: i32, ocvrs_return: *mut Result<()>);
// meanShiftFiltering(InputArray, OutputArray, int, int, TermCriteria, Stream &)(InputArray, OutputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:625
// ("cv::cuda::meanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src: *const c_void, dst: *const c_void, sp: i32, sr: i32, criteria: *const core::TermCriteria, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::meanShiftProc(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:644
// ("cv::cuda::meanShiftProc", vec![(pred!(mut, ["src", "dstr", "dstsp", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, dstr: *const c_void, dstsp: *const c_void, sp: i32, sr: i32, ocvrs_return: *mut Result<()>);
// meanShiftProc(InputArray, OutputArray, OutputArray, int, int, TermCriteria, Stream &)(InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:644
// ("cv::cuda::meanShiftProc", vec![(pred!(mut, ["src", "dstr", "dstsp", "sp", "sr", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(src: *const c_void, dstr: *const c_void, dstsp: *const c_void, sp: i32, sr: i32, criteria: *const core::TermCriteria, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::meanShiftSegmentation(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:658
// ("cv::cuda::meanShiftSegmentation", vec![(pred!(mut, ["src", "dst", "sp", "sr", "minsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, sp: i32, sr: i32, minsize: i32, ocvrs_return: *mut Result<()>);
// meanShiftSegmentation(InputArray, OutputArray, int, int, int, TermCriteria, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:658
// ("cv::cuda::meanShiftSegmentation", vec![(pred!(mut, ["src", "dst", "sp", "sr", "minsize", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int_TermCriteria_StreamR(src: *const c_void, dst: *const c_void, sp: i32, sr: i32, minsize: i32, criteria: *const core::TermCriteria, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::moments(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:871
// ("cv::cuda::moments", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_moments_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Moments>);
// moments(InputArray, const bool, const MomentsOrder, const int)(InputArray, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:871
// ("cv::cuda::moments", vec![(pred!(mut, ["src", "binaryImage", "order", "momentsType"], ["const cv::_InputArray*", "const bool", "const cv::cuda::MomentsOrder", "const int"]), _)]),
pub fn cv_cuda_moments_const__InputArrayR_const_bool_const_MomentsOrder_const_int(src: *const c_void, binary_image: bool, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32, ocvrs_return: *mut Result<core::Moments>);
// numMoments(const MomentsOrder)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:808
// ("cv::cuda::numMoments", vec![(pred!(mut, ["order"], ["const cv::cuda::MomentsOrder"]), _)]),
pub fn cv_cuda_numMoments_const_MomentsOrder(order: crate::cudaimgproc::CUDA_MomentsOrder, ocvrs_return: *mut Result<i32>);
// cv::cuda::spatialMoments(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:849
// ("cv::cuda::spatialMoments", vec![(pred!(mut, ["src", "moments"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR(src: *const c_void, moments: *const c_void, ocvrs_return: *mut Result<()>);
// spatialMoments(InputArray, OutputArray, const bool, const MomentsOrder, const int, Stream &)(InputArray, OutputArray, Primitive, Enum, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:849
// ("cv::cuda::spatialMoments", vec![(pred!(mut, ["src", "moments", "binaryImage", "order", "momentsType", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const bool", "const cv::cuda::MomentsOrder", "const int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR_const_bool_const_MomentsOrder_const_int_StreamR(src: *const c_void, moments: *const c_void, binary_image: bool, order: crate::cudaimgproc::CUDA_MomentsOrder, moments_type: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::swapChannels(InputOutputArray, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:149
// ("cv::cuda::swapChannels", vec![(pred!(mut, ["image", "dstOrder"], ["const cv::_InputOutputArray*", "const int**"]), _)]),
pub fn cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX(image: *const c_void, dst_order: *const [i32; 4], ocvrs_return: *mut Result<()>);
// swapChannels(InputOutputArray, const int *, Stream &)(InputOutputArray, FixedArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:149
// ("cv::cuda::swapChannels", vec![(pred!(mut, ["image", "dstOrder", "stream"], ["const cv::_InputOutputArray*", "const int**", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX_StreamR(image: *const c_void, dst_order: *const [i32; 4], stream: *mut c_void, ocvrs_return: *mut Result<()>);
// apply(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:236
// ("cv::cuda::CLAHE::apply", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CLAHE_apply_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CLAHE::to_Algorithm() generated
// ("cv::cuda::CLAHE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CLAHE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CLAHE::to_CLAHE() generated
// ("cv::cuda::CLAHE::to_CLAHE", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CLAHE_to_CLAHE(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CLAHE::delete() generated
// ("cv::cuda::CLAHE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CLAHE_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:298
// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["image", "edges", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, image: *const c_void, edges: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CannyEdgeDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:298
// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["image", "edges"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, edges: *const c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:305
// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["dx", "dy", "edges", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, dx: *const c_void, dy: *const c_void, edges: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CannyEdgeDetector::detect(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:305
// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["dx", "dy", "edges"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, dx: *const c_void, dy: *const c_void, edges: *const c_void, ocvrs_return: *mut Result<()>);
// setLowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:307
// ("cv::cuda::CannyEdgeDetector::setLowThreshold", vec![(pred!(mut, ["low_thresh"], ["double"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_setLowThreshold_double(instance: *mut c_void, low_thresh: f64, ocvrs_return: *mut Result<()>);
// getLowThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:308
// ("cv::cuda::CannyEdgeDetector::getLowThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_getLowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setHighThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:310
// ("cv::cuda::CannyEdgeDetector::setHighThreshold", vec![(pred!(mut, ["high_thresh"], ["double"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_setHighThreshold_double(instance: *mut c_void, high_thresh: f64, ocvrs_return: *mut Result<()>);
// getHighThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:311
// ("cv::cuda::CannyEdgeDetector::getHighThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_getHighThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAppertureSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:313
// ("cv::cuda::CannyEdgeDetector::setAppertureSize", vec![(pred!(mut, ["apperture_size"], ["int"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_setAppertureSize_int(instance: *mut c_void, apperture_size: i32, ocvrs_return: *mut Result<()>);
// getAppertureSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:314
// ("cv::cuda::CannyEdgeDetector::getAppertureSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_getAppertureSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setL2Gradient(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:316
// ("cv::cuda::CannyEdgeDetector::setL2Gradient", vec![(pred!(mut, ["L2gradient"], ["bool"]), _)]),
pub fn cv_cuda_CannyEdgeDetector_setL2Gradient_bool(instance: *mut c_void, l2gradient: bool, ocvrs_return: *mut Result<()>);
// getL2Gradient()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:317
// ("cv::cuda::CannyEdgeDetector::getL2Gradient", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_getL2Gradient_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::cuda::CannyEdgeDetector::to_Algorithm() generated
// ("cv::cuda::CannyEdgeDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CannyEdgeDetector::delete() generated
// ("cv::cuda::CannyEdgeDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CannyEdgeDetector_delete(instance: *mut c_void);
// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:535
// ("cv::cuda::CornernessCriteria::compute", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CornernessCriteria::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:535
// ("cv::cuda::CornernessCriteria::compute", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CornernessCriteria::to_Algorithm() generated
// ("cv::cuda::CornernessCriteria::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CornernessCriteria_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CornernessCriteria::delete() generated
// ("cv::cuda::CornernessCriteria::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CornernessCriteria_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:580
// ("cv::cuda::CornersDetector::detect", vec![(pred!(mut, ["image", "corners", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(instance: *mut c_void, image: *const c_void, corners: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CornersDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:580
// ("cv::cuda::CornersDetector::detect", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, corners: *const c_void, ocvrs_return: *mut Result<()>);
// setMaxCorners(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:582
// ("cv::cuda::CornersDetector::setMaxCorners", vec![(pred!(mut, ["maxCorners"], ["int"]), _)]),
pub fn cv_cuda_CornersDetector_setMaxCorners_int(instance: *mut c_void, max_corners: i32, ocvrs_return: *mut Result<()>);
// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:583
// ("cv::cuda::CornersDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
pub fn cv_cuda_CornersDetector_setMinDistance_double(instance: *mut c_void, min_distance: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::CornersDetector::to_Algorithm() generated
// ("cv::cuda::CornersDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CornersDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CornersDetector::delete() generated
// ("cv::cuda::CornersDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CornersDetector_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:463
// ("cv::cuda::HoughCirclesDetector::detect", vec![(pred!(mut, ["src", "circles", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, circles: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HoughCirclesDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:463
// ("cv::cuda::HoughCirclesDetector::detect", vec![(pred!(mut, ["src", "circles"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, circles: *const c_void, ocvrs_return: *mut Result<()>);
// setDp(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:465
// ("cv::cuda::HoughCirclesDetector::setDp", vec![(pred!(mut, ["dp"], ["float"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setDp_float(instance: *mut c_void, dp: f32, ocvrs_return: *mut Result<()>);
// getDp()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:466
// ("cv::cuda::HoughCirclesDetector::getDp", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getDp_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinDist(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:468
// ("cv::cuda::HoughCirclesDetector::setMinDist", vec![(pred!(mut, ["minDist"], ["float"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setMinDist_float(instance: *mut c_void, min_dist: f32, ocvrs_return: *mut Result<()>);
// getMinDist()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:469
// ("cv::cuda::HoughCirclesDetector::getMinDist", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getMinDist_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setCannyThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:471
// ("cv::cuda::HoughCirclesDetector::setCannyThreshold", vec![(pred!(mut, ["cannyThreshold"], ["int"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setCannyThreshold_int(instance: *mut c_void, canny_threshold: i32, ocvrs_return: *mut Result<()>);
// getCannyThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:472
// ("cv::cuda::HoughCirclesDetector::getCannyThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getCannyThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setVotesThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:474
// ("cv::cuda::HoughCirclesDetector::setVotesThreshold", vec![(pred!(mut, ["votesThreshold"], ["int"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setVotesThreshold_int(instance: *mut c_void, votes_threshold: i32, ocvrs_return: *mut Result<()>);
// getVotesThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:475
// ("cv::cuda::HoughCirclesDetector::getVotesThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getVotesThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:477
// ("cv::cuda::HoughCirclesDetector::setMinRadius", vec![(pred!(mut, ["minRadius"], ["int"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setMinRadius_int(instance: *mut c_void, min_radius: i32, ocvrs_return: *mut Result<()>);
// getMinRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:478
// ("cv::cuda::HoughCirclesDetector::getMinRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getMinRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:480
// ("cv::cuda::HoughCirclesDetector::setMaxRadius", vec![(pred!(mut, ["maxRadius"], ["int"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setMaxRadius_int(instance: *mut c_void, max_radius: i32, ocvrs_return: *mut Result<()>);
// getMaxRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:481
// ("cv::cuda::HoughCirclesDetector::getMaxRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getMaxRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxCircles(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:483
// ("cv::cuda::HoughCirclesDetector::setMaxCircles", vec![(pred!(mut, ["maxCircles"], ["int"]), _)]),
pub fn cv_cuda_HoughCirclesDetector_setMaxCircles_int(instance: *mut c_void, max_circles: i32, ocvrs_return: *mut Result<()>);
// getMaxCircles()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:484
// ("cv::cuda::HoughCirclesDetector::getMaxCircles", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_getMaxCircles_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::HoughCirclesDetector::to_Algorithm() generated
// ("cv::cuda::HoughCirclesDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::HoughCirclesDetector::delete() generated
// ("cv::cuda::HoughCirclesDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughCirclesDetector_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:356
// ("cv::cuda::HoughLinesDetector::detect", vec![(pred!(mut, ["src", "lines", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, lines: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HoughLinesDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:356
// ("cv::cuda::HoughLinesDetector::detect", vec![(pred!(mut, ["src", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// downloadResults(InputArray, OutputArray, OutputArray, Stream &)(InputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:365
// ("cv::cuda::HoughLinesDetector::downloadResults", vec![(pred!(mut, ["d_lines", "h_lines", "h_votes", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, d_lines: *const c_void, h_lines: *const c_void, h_votes: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HoughLinesDetector::downloadResults(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:365
// ("cv::cuda::HoughLinesDetector::downloadResults", vec![(pred!(mut, ["d_lines", "h_lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, d_lines: *const c_void, h_lines: *const c_void, ocvrs_return: *mut Result<()>);
// setRho(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:367
// ("cv::cuda::HoughLinesDetector::setRho", vec![(pred!(mut, ["rho"], ["float"]), _)]),
pub fn cv_cuda_HoughLinesDetector_setRho_float(instance: *mut c_void, rho: f32, ocvrs_return: *mut Result<()>);
// getRho()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:368
// ("cv::cuda::HoughLinesDetector::getRho", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_getRho_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setTheta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:370
// ("cv::cuda::HoughLinesDetector::setTheta", vec![(pred!(mut, ["theta"], ["float"]), _)]),
pub fn cv_cuda_HoughLinesDetector_setTheta_float(instance: *mut c_void, theta: f32, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:371
// ("cv::cuda::HoughLinesDetector::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:373
// ("cv::cuda::HoughLinesDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_cuda_HoughLinesDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:374
// ("cv::cuda::HoughLinesDetector::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDoSort(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:376
// ("cv::cuda::HoughLinesDetector::setDoSort", vec![(pred!(mut, ["doSort"], ["bool"]), _)]),
pub fn cv_cuda_HoughLinesDetector_setDoSort_bool(instance: *mut c_void, do_sort: bool, ocvrs_return: *mut Result<()>);
// getDoSort()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:377
// ("cv::cuda::HoughLinesDetector::getDoSort", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_getDoSort_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setMaxLines(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:379
// ("cv::cuda::HoughLinesDetector::setMaxLines", vec![(pred!(mut, ["maxLines"], ["int"]), _)]),
pub fn cv_cuda_HoughLinesDetector_setMaxLines_int(instance: *mut c_void, max_lines: i32, ocvrs_return: *mut Result<()>);
// getMaxLines()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:380
// ("cv::cuda::HoughLinesDetector::getMaxLines", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_getMaxLines_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::HoughLinesDetector::to_Algorithm() generated
// ("cv::cuda::HoughLinesDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::HoughLinesDetector::delete() generated
// ("cv::cuda::HoughLinesDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughLinesDetector_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:413
// ("cv::cuda::HoughSegmentDetector::detect", vec![(pred!(mut, ["src", "lines", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, lines: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HoughSegmentDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:413
// ("cv::cuda::HoughSegmentDetector::detect", vec![(pred!(mut, ["src", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// setRho(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:415
// ("cv::cuda::HoughSegmentDetector::setRho", vec![(pred!(mut, ["rho"], ["float"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setRho_float(instance: *mut c_void, rho: f32, ocvrs_return: *mut Result<()>);
// getRho()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:416
// ("cv::cuda::HoughSegmentDetector::getRho", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getRho_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setTheta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:418
// ("cv::cuda::HoughSegmentDetector::setTheta", vec![(pred!(mut, ["theta"], ["float"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setTheta_float(instance: *mut c_void, theta: f32, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:419
// ("cv::cuda::HoughSegmentDetector::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinLineLength(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:421
// ("cv::cuda::HoughSegmentDetector::setMinLineLength", vec![(pred!(mut, ["minLineLength"], ["int"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setMinLineLength_int(instance: *mut c_void, min_line_length: i32, ocvrs_return: *mut Result<()>);
// getMinLineLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:422
// ("cv::cuda::HoughSegmentDetector::getMinLineLength", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getMinLineLength_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLineGap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:424
// ("cv::cuda::HoughSegmentDetector::setMaxLineGap", vec![(pred!(mut, ["maxLineGap"], ["int"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setMaxLineGap_int(instance: *mut c_void, max_line_gap: i32, ocvrs_return: *mut Result<()>);
// getMaxLineGap()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:425
// ("cv::cuda::HoughSegmentDetector::getMaxLineGap", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getMaxLineGap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLines(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:427
// ("cv::cuda::HoughSegmentDetector::setMaxLines", vec![(pred!(mut, ["maxLines"], ["int"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setMaxLines_int(instance: *mut c_void, max_lines: i32, ocvrs_return: *mut Result<()>);
// getMaxLines()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:428
// ("cv::cuda::HoughSegmentDetector::getMaxLines", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getMaxLines_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:430
// ("cv::cuda::HoughSegmentDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_cuda_HoughSegmentDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:431
// ("cv::cuda::HoughSegmentDetector::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::HoughSegmentDetector::to_Algorithm() generated
// ("cv::cuda::HoughSegmentDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::HoughSegmentDetector::delete() generated
// ("cv::cuda::HoughSegmentDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HoughSegmentDetector_delete(instance: *mut c_void);
// match(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:677
// ("cv::cuda::TemplateMatching::match", vec![(pred!(mut, ["image", "templ", "result", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, image: *const c_void, templ: *const c_void, result: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::TemplateMatching::match(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaimgproc.hpp:677
// ("cv::cuda::TemplateMatching::match", vec![(pred!(mut, ["image", "templ", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, templ: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::TemplateMatching::to_Algorithm() generated
// ("cv::cuda::TemplateMatching::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_TemplateMatching_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::TemplateMatching::delete() generated
// ("cv::cuda::TemplateMatching::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_TemplateMatching_delete(instance: *mut c_void);
