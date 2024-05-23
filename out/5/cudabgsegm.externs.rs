// cv::cuda::createBackgroundSubtractorMOG() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:116
// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createBackgroundSubtractorMOG(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:155
// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createBackgroundSubtractorMOG2(ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:155
// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
pub fn cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(history: i32, var_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
// createBackgroundSubtractorMOG(int, int, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:116
// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, ["history", "nmixtures", "backgroundRatio", "noiseSigma"], ["int", "int", "double", "double"]), _)]),
pub fn cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:85
// ("cv::cuda::BackgroundSubtractorMOG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:88
// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(instance: *const c_void, background_image: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:90
// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_GpuMatR_StreamR(instance: *mut c_void, background_image: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getHistory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:94
// ("cv::cuda::BackgroundSubtractorMOG::getHistory", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:95
// ("cv::cuda::BackgroundSubtractorMOG::setHistory", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_setHistory_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result<()>);
// getNMixtures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:97
// ("cv::cuda::BackgroundSubtractorMOG::getNMixtures", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:98
// ("cv::cuda::BackgroundSubtractorMOG::setNMixtures", vec![(pred!(mut, ["nmix"], ["int"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(instance: *mut c_void, nmix: i32, ocvrs_return: *mut Result<()>);
// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:100
// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:101
// ("cv::cuda::BackgroundSubtractorMOG::setBackgroundRatio", vec![(pred!(mut, ["backgroundRatio"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(instance: *mut c_void, background_ratio: f64, ocvrs_return: *mut Result<()>);
// getNoiseSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:103
// ("cv::cuda::BackgroundSubtractorMOG::getNoiseSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNoiseSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:104
// ("cv::cuda::BackgroundSubtractorMOG::setNoiseSigma", vec![(pred!(mut, ["noiseSigma"], ["double"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_setNoiseSigma_double(instance: *mut c_void, noise_sigma: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::BackgroundSubtractorMOG::to_Algorithm() generated
// ("cv::cuda::BackgroundSubtractorMOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorMOG::to_BackgroundSubtractor() generated
// ("cv::cuda::BackgroundSubtractorMOG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorMOG::delete() generated
// ("cv::cuda::BackgroundSubtractorMOG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG_delete(instance: *mut c_void);
// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:136
// ("cv::cuda::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:138
// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(instance: *const c_void, background_image: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:140
// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_GpuMatR_StreamR(instance: *mut c_void, background_image: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::BackgroundSubtractorMOG2::to_Algorithm() generated
// ("cv::cuda::BackgroundSubtractorMOG2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractor() generated
// ("cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractor(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractorMOG2() generated
// ("cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractorMOG2(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BackgroundSubtractorMOG2::delete() generated
// ("cv::cuda::BackgroundSubtractorMOG2::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BackgroundSubtractorMOG2_delete(instance: *mut c_void);
