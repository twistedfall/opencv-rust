// cv::colorChange(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:795
// ("cv::colorChange", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, mask: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// colorChange(InputArray, InputArray, OutputArray, float, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:795
// ("cv::colorChange", vec![(pred!(mut, ["src", "mask", "dst", "red_mul", "green_mul", "blue_mul"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
pub fn cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, mask: *const c_void, dst: *const c_void, red_mul: f32, green_mul: f32, blue_mul: f32, ocvrs_return: *mut Result<()>);
// cv::createAlignMTB() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:526
// ("cv::createAlignMTB", vec![(pred!(mut, [], []), _)]),
pub fn cv_createAlignMTB(ocvrs_return: *mut Result<*mut c_void>);
// createAlignMTB(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:526
// ("cv::createAlignMTB", vec![(pred!(mut, ["max_bits", "exclude_range", "cut"], ["int", "int", "bool"]), _)]),
pub fn cv_createAlignMTB_int_int_bool(max_bits: i32, exclude_range: i32, cut: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::createCalibrateDebevec() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:569
// ("cv::createCalibrateDebevec", vec![(pred!(mut, [], []), _)]),
pub fn cv_createCalibrateDebevec(ocvrs_return: *mut Result<*mut c_void>);
// createCalibrateDebevec(int, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:569
// ("cv::createCalibrateDebevec", vec![(pred!(mut, ["samples", "lambda", "random"], ["int", "float", "bool"]), _)]),
pub fn cv_createCalibrateDebevec_int_float_bool(samples: i32, lambda: f32, random: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::createCalibrateRobertson() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:593
// ("cv::createCalibrateRobertson", vec![(pred!(mut, [], []), _)]),
pub fn cv_createCalibrateRobertson(ocvrs_return: *mut Result<*mut c_void>);
// createCalibrateRobertson(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:593
// ("cv::createCalibrateRobertson", vec![(pred!(mut, ["max_iter", "threshold"], ["int", "float"]), _)]),
pub fn cv_createCalibrateRobertson_int_float(max_iter: i32, threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// createMergeDebevec()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:627
// ("cv::createMergeDebevec", vec![(pred!(mut, [], []), _)]),
pub fn cv_createMergeDebevec(ocvrs_return: *mut Result<*mut c_void>);
// cv::createMergeMertens() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:669
// ("cv::createMergeMertens", vec![(pred!(mut, [], []), _)]),
pub fn cv_createMergeMertens(ocvrs_return: *mut Result<*mut c_void>);
// createMergeMertens(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:669
// ("cv::createMergeMertens", vec![(pred!(mut, ["contrast_weight", "saturation_weight", "exposure_weight"], ["float", "float", "float"]), _)]),
pub fn cv_createMergeMertens_float_float_float(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32, ocvrs_return: *mut Result<*mut c_void>);
// createMergeRobertson()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:686
// ("cv::createMergeRobertson", vec![(pred!(mut, [], []), _)]),
pub fn cv_createMergeRobertson(ocvrs_return: *mut Result<*mut c_void>);
// cv::createTonemap() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:355
// ("cv::createTonemap", vec![(pred!(mut, [], []), _)]),
pub fn cv_createTonemap(ocvrs_return: *mut Result<*mut c_void>);
// cv::createTonemapDrago() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:386
// ("cv::createTonemapDrago", vec![(pred!(mut, [], []), _)]),
pub fn cv_createTonemapDrago(ocvrs_return: *mut Result<*mut c_void>);
// createTonemapDrago(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:386
// ("cv::createTonemapDrago", vec![(pred!(mut, ["gamma", "saturation", "bias"], ["float", "float", "float"]), _)]),
pub fn cv_createTonemapDrago_float_float_float(gamma: f32, saturation: f32, bias: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createTonemapMantiuk() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:445
// ("cv::createTonemapMantiuk", vec![(pred!(mut, [], []), _)]),
pub fn cv_createTonemapMantiuk(ocvrs_return: *mut Result<*mut c_void>);
// createTonemapMantiuk(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:445
// ("cv::createTonemapMantiuk", vec![(pred!(mut, ["gamma", "scale", "saturation"], ["float", "float", "float"]), _)]),
pub fn cv_createTonemapMantiuk_float_float_float(gamma: f32, scale: f32, saturation: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createTonemapReinhard() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:419
// ("cv::createTonemapReinhard", vec![(pred!(mut, [], []), _)]),
pub fn cv_createTonemapReinhard(ocvrs_return: *mut Result<*mut c_void>);
// createTonemapReinhard(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:419
// ("cv::createTonemapReinhard", vec![(pred!(mut, ["gamma", "intensity", "light_adapt", "color_adapt"], ["float", "float", "float", "float"]), _)]),
pub fn cv_createTonemapReinhard_float_float_float_float(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32, ocvrs_return: *mut Result<*mut c_void>);
// createTonemap(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:355
// ("cv::createTonemap", vec![(pred!(mut, ["gamma"], ["float"]), _)]),
pub fn cv_createTonemap_float(gamma: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::fastNlMeansDenoisingColored(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:144
// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "float"]), _)]),
pub fn cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float(src: *const c_void, dst: *mut c_void, h_luminance: f32, photo_render: f32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingColored(const GpuMat &, GpuMat &, float, float, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:144
// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render", "search_window", "block_size", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float_int_int_StreamR(src: *const c_void, dst: *mut c_void, h_luminance: f32, photo_render: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::fastNlMeansDenoisingColored(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:139
// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
pub fn cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, dst: *const c_void, h_luminance: f32, photo_render: f32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingColored(InputArray, OutputArray, float, float, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:139
// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render", "search_window", "block_size", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int_StreamR(src: *const c_void, dst: *const c_void, h_luminance: f32, photo_render: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::fastNlMeansDenoising(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:109
// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float"]), _)]),
pub fn cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float(src: *const c_void, dst: *mut c_void, h: f32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoising(const GpuMat &, GpuMat &, float, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:109
// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float_int_int_StreamR(src: *const c_void, dst: *mut c_void, h: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::fastNlMeansDenoising(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:104
// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float(src: *const c_void, dst: *const c_void, h: f32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoising(InputArray, OutputArray, float, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:104
// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_StreamR(src: *const c_void, dst: *const c_void, h: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::nonLocalMeans(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:73
// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float"]), _)]),
pub fn cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float(src: *const c_void, dst: *mut c_void, h: f32, ocvrs_return: *mut Result<()>);
// nonLocalMeans(const GpuMat &, GpuMat &, float, int, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:73
// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "borderMode", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float_int_int_int_StreamR(src: *const c_void, dst: *mut c_void, h: f32, search_window: i32, block_size: i32, border_mode: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::nonLocalMeans(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:67
// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float(src: *const c_void, dst: *const c_void, h: f32, ocvrs_return: *mut Result<()>);
// nonLocalMeans(InputArray, OutputArray, float, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:67
// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "borderMode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float_int_int_int_StreamR(src: *const c_void, dst: *const c_void, h: f32, search_window: i32, block_size: i32, border_mode: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// decolor(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:703
// ("cv::decolor", vec![(pred!(mut, ["src", "grayscale", "color_boost"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_decolor_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, grayscale: *const c_void, color_boost: *const c_void, ocvrs_return: *mut Result<()>);
// cv::denoise_TVL1(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:324
// ("cv::denoise_TVL1", vec![(pred!(mut, ["observations", "result"], ["const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
pub fn cv_denoise_TVL1_const_vectorLMatGR_MatR(observations: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// denoise_TVL1(const std::vector<Mat> &, Mat &, double, int)(CppPassByVoidPtr, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:324
// ("cv::denoise_TVL1", vec![(pred!(mut, ["observations", "result", "lambda", "niters"], ["const std::vector<cv::Mat>*", "cv::Mat*", "double", "int"]), _)]),
pub fn cv_denoise_TVL1_const_vectorLMatGR_MatR_double_int(observations: *const c_void, result: *mut c_void, lambda: f64, niters: i32, ocvrs_return: *mut Result<()>);
// cv::detailEnhance(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:862
// ("cv::detailEnhance", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detailEnhance_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// detailEnhance(InputArray, OutputArray, float, float)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:862
// ("cv::detailEnhance", vec![(pred!(mut, ["src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
pub fn cv_detailEnhance_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, dst: *const c_void, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result<()>);
// cv::edgePreservingFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:852
// ("cv::edgePreservingFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// edgePreservingFilter(InputArray, OutputArray, int, float, float)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:852
// ("cv::edgePreservingFilter", vec![(pred!(mut, ["src", "dst", "flags", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float"]), _)]),
pub fn cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_float_float(src: *const c_void, dst: *const c_void, flags: i32, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoisingColoredMulti(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:282
// ("cv::fastNlMeansDenoisingColoredMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingColoredMulti(InputArrayOfArrays, OutputArray, int, int, float, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:282
// ("cv::fastNlMeansDenoisingColoredMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "hColor", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "float", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int_float_float_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoisingColored(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:197
// ("cv::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingColored(InputArray, OutputArray, float, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:197
// ("cv::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h", "hColor", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int(src: *const c_void, dst: *const c_void, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoisingMulti(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:224
// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoisingMulti(InputArray, OutputArray, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:253
// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const std::vector<float>*"]), _)]),
pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vectorLfloatGR(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: *const c_void, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingMulti(InputArrayOfArrays, OutputArray, int, int, const std::vector<float> &, int, int, int)(InputArray, OutputArray, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:253
// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "templateWindowSize", "searchWindowSize", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const std::vector<float>*", "int", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vectorLfloatGR_int_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: *const c_void, template_window_size: i32, search_window_size: i32, norm_type: i32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoisingMulti(InputArrayOfArrays, OutputArray, int, int, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:224
// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_float_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoising(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:147
// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fastNlMeansDenoising(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:174
// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*"]), _)]),
pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR(src: *const c_void, dst: *const c_void, h: *const c_void, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoising(InputArray, OutputArray, const std::vector<float> &, int, int, int)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:174
// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*", "int", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR_int_int_int(src: *const c_void, dst: *const c_void, h: *const c_void, template_window_size: i32, search_window_size: i32, norm_type: i32, ocvrs_return: *mut Result<()>);
// fastNlMeansDenoising(InputArray, OutputArray, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:147
// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int"]), _)]),
pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int(src: *const c_void, dst: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result<()>);
// cv::illuminationChange(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:809
// ("cv::illuminationChange", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, mask: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// illuminationChange(InputArray, InputArray, OutputArray, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:809
// ("cv::illuminationChange", vec![(pred!(mut, ["src", "mask", "dst", "alpha", "beta"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
pub fn cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, mask: *const c_void, dst: *const c_void, alpha: f32, beta: f32, ocvrs_return: *mut Result<()>);
// inpaint(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:119
// ("cv::inpaint", vec![(pred!(mut, ["src", "inpaintMask", "dst", "inpaintRadius", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
pub fn cv_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src: *const c_void, inpaint_mask: *const c_void, dst: *const c_void, inpaint_radius: f64, flags: i32, ocvrs_return: *mut Result<()>);
// cv::pencilSketch(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:877
// ("cv::pencilSketch", vec![(pred!(mut, ["src", "dst1", "dst2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, dst1: *const c_void, dst2: *const c_void, ocvrs_return: *mut Result<()>);
// pencilSketch(InputArray, OutputArray, OutputArray, float, float, float)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:877
// ("cv::pencilSketch", vec![(pred!(mut, ["src", "dst1", "dst2", "sigma_s", "sigma_r", "shade_factor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
pub fn cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, dst1: *const c_void, dst2: *const c_void, sigma_s: f32, sigma_r: f32, shade_factor: f32, ocvrs_return: *mut Result<()>);
// seamlessClone(InputArray, InputArray, InputArray, Point, OutputArray, int)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:780
// ("cv::seamlessClone", vec![(pred!(mut, ["src", "dst", "mask", "p", "blend", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_seamlessClone_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, mask: *const c_void, p: *const core::Point, blend: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// cv::stylization(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:889
// ("cv::stylization", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_stylization_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// stylization(InputArray, OutputArray, float, float)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:889
// ("cv::stylization", vec![(pred!(mut, ["src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
pub fn cv_stylization_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, dst: *const c_void, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result<()>);
// cv::textureFlattening(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:827
// ("cv::textureFlattening", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, mask: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// textureFlattening(InputArray, InputArray, OutputArray, float, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:827
// ("cv::textureFlattening", vec![(pred!(mut, ["src", "mask", "dst", "low_threshold", "high_threshold", "kernel_size"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int"]), _)]),
pub fn cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_int(src: *const c_void, mask: *const c_void, dst: *const c_void, low_threshold: f32, high_threshold: f32, kernel_size: i32, ocvrs_return: *mut Result<()>);
// process(InputArrayOfArrays, std::vector<Mat> &, InputArray, InputArray)(InputArray, CppPassByVoidPtr, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:460
// ("cv::AlignExposures::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_AlignExposures_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *mut c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// cv::AlignExposures::to_AlignMTB() generated
// ("cv::AlignExposures::to_AlignMTB", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignExposures_to_AlignMTB(instance: *mut c_void) -> *mut c_void;
// cv::AlignExposures::to_Algorithm() generated
// ("cv::AlignExposures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignExposures_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AlignExposures::delete() generated
// ("cv::AlignExposures::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignExposures_delete(instance: *mut c_void);
// process(InputArrayOfArrays, std::vector<Mat> &, InputArray, InputArray)(InputArray, CppPassByVoidPtr, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:476
// ("cv::AlignMTB::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_AlignMTB_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *mut c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// process(InputArrayOfArrays, std::vector<Mat> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:484
// ("cv::AlignMTB::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_AlignMTB_process_const__InputArrayR_vectorLMatGR(instance: *mut c_void, src: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// calculateShift(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:492
// ("cv::AlignMTB::calculateShift", vec![(pred!(mut, ["img0", "img1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_AlignMTB_calculateShift_const__InputArrayR_const__InputArrayR(instance: *mut c_void, img0: *const c_void, img1: *const c_void, ocvrs_return: *mut Result<core::Point>);
// shiftMat(InputArray, OutputArray, const Point)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:499
// ("cv::AlignMTB::shiftMat", vec![(pred!(mut, ["src", "dst", "shift"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Point"]), _)]),
pub fn cv_AlignMTB_shiftMat_const__InputArrayR_const__OutputArrayR_const_Point(instance: *mut c_void, src: *const c_void, dst: *const c_void, shift: *const core::Point, ocvrs_return: *mut Result<()>);
// computeBitmaps(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:506
// ("cv::AlignMTB::computeBitmaps", vec![(pred!(mut, ["img", "tb", "eb"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_AlignMTB_computeBitmaps_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, tb: *const c_void, eb: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxBits()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:508
// ("cv::AlignMTB::getMaxBits", vec![(pred!(const, [], []), _)]),
pub fn cv_AlignMTB_getMaxBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:509
// ("cv::AlignMTB::setMaxBits", vec![(pred!(mut, ["max_bits"], ["int"]), _)]),
pub fn cv_AlignMTB_setMaxBits_int(instance: *mut c_void, max_bits: i32, ocvrs_return: *mut Result<()>);
// getExcludeRange()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:511
// ("cv::AlignMTB::getExcludeRange", vec![(pred!(const, [], []), _)]),
pub fn cv_AlignMTB_getExcludeRange_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setExcludeRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:512
// ("cv::AlignMTB::setExcludeRange", vec![(pred!(mut, ["exclude_range"], ["int"]), _)]),
pub fn cv_AlignMTB_setExcludeRange_int(instance: *mut c_void, exclude_range: i32, ocvrs_return: *mut Result<()>);
// getCut()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:514
// ("cv::AlignMTB::getCut", vec![(pred!(const, [], []), _)]),
pub fn cv_AlignMTB_getCut_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setCut(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:515
// ("cv::AlignMTB::setCut", vec![(pred!(mut, ["value"], ["bool"]), _)]),
pub fn cv_AlignMTB_setCut_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result<()>);
// cv::AlignMTB::to_Algorithm() generated
// ("cv::AlignMTB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignMTB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AlignMTB::to_AlignExposures() generated
// ("cv::AlignMTB::to_AlignExposures", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignMTB_to_AlignExposures(instance: *mut c_void) -> *mut c_void;
// cv::AlignMTB::delete() generated
// ("cv::AlignMTB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AlignMTB_delete(instance: *mut c_void);
// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:539
// ("cv::CalibrateCRF::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_CalibrateCRF_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result<()>);
// cv::CalibrateCRF::to_CalibrateDebevec() generated
// ("cv::CalibrateCRF::to_CalibrateDebevec", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateCRF_to_CalibrateDebevec(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateCRF::to_CalibrateRobertson() generated
// ("cv::CalibrateCRF::to_CalibrateRobertson", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateCRF_to_CalibrateRobertson(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateCRF::to_Algorithm() generated
// ("cv::CalibrateCRF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateCRF_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateCRF::delete() generated
// ("cv::CalibrateCRF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateCRF_delete(instance: *mut c_void);
// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:551
// ("cv::CalibrateDebevec::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateDebevec_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:552
// ("cv::CalibrateDebevec::setLambda", vec![(pred!(mut, ["lambda"], ["float"]), _)]),
pub fn cv_CalibrateDebevec_setLambda_float(instance: *mut c_void, lambda: f32, ocvrs_return: *mut Result<()>);
// getSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:554
// ("cv::CalibrateDebevec::getSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateDebevec_getSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:555
// ("cv::CalibrateDebevec::setSamples", vec![(pred!(mut, ["samples"], ["int"]), _)]),
pub fn cv_CalibrateDebevec_setSamples_int(instance: *mut c_void, samples: i32, ocvrs_return: *mut Result<()>);
// getRandom()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:557
// ("cv::CalibrateDebevec::getRandom", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateDebevec_getRandom_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setRandom(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:558
// ("cv::CalibrateDebevec::setRandom", vec![(pred!(mut, ["random"], ["bool"]), _)]),
pub fn cv_CalibrateDebevec_setRandom_bool(instance: *mut c_void, random: bool, ocvrs_return: *mut Result<()>);
// cv::CalibrateDebevec::to_Algorithm() generated
// ("cv::CalibrateDebevec::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateDebevec_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateDebevec::to_CalibrateCRF() generated
// ("cv::CalibrateDebevec::to_CalibrateCRF", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateDebevec_to_CalibrateCRF(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateDebevec::delete() generated
// ("cv::CalibrateDebevec::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateDebevec_delete(instance: *mut c_void);
// getMaxIter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:579
// ("cv::CalibrateRobertson::getMaxIter", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateRobertson_getMaxIter_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxIter(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:580
// ("cv::CalibrateRobertson::setMaxIter", vec![(pred!(mut, ["max_iter"], ["int"]), _)]),
pub fn cv_CalibrateRobertson_setMaxIter_int(instance: *mut c_void, max_iter: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:582
// ("cv::CalibrateRobertson::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateRobertson_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:583
// ("cv::CalibrateRobertson::setThreshold", vec![(pred!(mut, ["threshold"], ["float"]), _)]),
pub fn cv_CalibrateRobertson_setThreshold_float(instance: *mut c_void, threshold: f32, ocvrs_return: *mut Result<()>);
// getRadiance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:585
// ("cv::CalibrateRobertson::getRadiance", vec![(pred!(const, [], []), _)]),
pub fn cv_CalibrateRobertson_getRadiance_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::CalibrateRobertson::to_Algorithm() generated
// ("cv::CalibrateRobertson::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateRobertson_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateRobertson::to_CalibrateCRF() generated
// ("cv::CalibrateRobertson::to_CalibrateCRF", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateRobertson_to_CalibrateCRF(instance: *mut c_void) -> *mut c_void;
// cv::CalibrateRobertson::delete() generated
// ("cv::CalibrateRobertson::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CalibrateRobertson_delete(instance: *mut c_void);
// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:620
// ("cv::MergeDebevec::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:622
// ("cv::MergeDebevec::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result<()>);
// cv::MergeDebevec::to_Algorithm() generated
// ("cv::MergeDebevec::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeDebevec_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MergeDebevec::to_MergeExposures() generated
// ("cv::MergeDebevec::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeDebevec_to_MergeExposures(instance: *mut c_void) -> *mut c_void;
// cv::MergeDebevec::delete() generated
// ("cv::MergeDebevec::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeDebevec_delete(instance: *mut c_void);
// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:608
// ("cv::MergeExposures::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeExposures_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// cv::MergeExposures::to_MergeDebevec() generated
// ("cv::MergeExposures::to_MergeDebevec", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeExposures_to_MergeDebevec(instance: *mut c_void) -> *mut c_void;
// cv::MergeExposures::to_MergeMertens() generated
// ("cv::MergeExposures::to_MergeMertens", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeExposures_to_MergeMertens(instance: *mut c_void) -> *mut c_void;
// cv::MergeExposures::to_MergeRobertson() generated
// ("cv::MergeExposures::to_MergeRobertson", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeExposures_to_MergeRobertson(instance: *mut c_void) -> *mut c_void;
// cv::MergeExposures::to_Algorithm() generated
// ("cv::MergeExposures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeExposures_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MergeExposures::delete() generated
// ("cv::MergeExposures::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeExposures_delete(instance: *mut c_void);
// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:643
// ("cv::MergeMertens::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// process(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:650
// ("cv::MergeMertens::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getContrastWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:652
// ("cv::MergeMertens::getContrastWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_MergeMertens_getContrastWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setContrastWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:653
// ("cv::MergeMertens::setContrastWeight", vec![(pred!(mut, ["contrast_weiht"], ["float"]), _)]),
pub fn cv_MergeMertens_setContrastWeight_float(instance: *mut c_void, contrast_weiht: f32, ocvrs_return: *mut Result<()>);
// getSaturationWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:655
// ("cv::MergeMertens::getSaturationWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_MergeMertens_getSaturationWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturationWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:656
// ("cv::MergeMertens::setSaturationWeight", vec![(pred!(mut, ["saturation_weight"], ["float"]), _)]),
pub fn cv_MergeMertens_setSaturationWeight_float(instance: *mut c_void, saturation_weight: f32, ocvrs_return: *mut Result<()>);
// getExposureWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:658
// ("cv::MergeMertens::getExposureWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_MergeMertens_getExposureWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setExposureWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:659
// ("cv::MergeMertens::setExposureWeight", vec![(pred!(mut, ["exposure_weight"], ["float"]), _)]),
pub fn cv_MergeMertens_setExposureWeight_float(instance: *mut c_void, exposure_weight: f32, ocvrs_return: *mut Result<()>);
// cv::MergeMertens::to_Algorithm() generated
// ("cv::MergeMertens::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeMertens_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MergeMertens::to_MergeExposures() generated
// ("cv::MergeMertens::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeMertens_to_MergeExposures(instance: *mut c_void) -> *mut c_void;
// cv::MergeMertens::delete() generated
// ("cv::MergeMertens::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeMertens_delete(instance: *mut c_void);
// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:679
// ("cv::MergeRobertson::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:681
// ("cv::MergeRobertson::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result<()>);
// cv::MergeRobertson::to_Algorithm() generated
// ("cv::MergeRobertson::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeRobertson_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MergeRobertson::to_MergeExposures() generated
// ("cv::MergeRobertson::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeRobertson_to_MergeExposures(instance: *mut c_void) -> *mut c_void;
// cv::MergeRobertson::delete() generated
// ("cv::MergeRobertson::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MergeRobertson_delete(instance: *mut c_void);
// process(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:343
// ("cv::Tonemap::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Tonemap_process_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:345
// ("cv::Tonemap::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_Tonemap_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:346
// ("cv::Tonemap::setGamma", vec![(pred!(mut, ["gamma"], ["float"]), _)]),
pub fn cv_Tonemap_setGamma_float(instance: *mut c_void, gamma: f32, ocvrs_return: *mut Result<()>);
// cv::Tonemap::to_TonemapDrago() generated
// ("cv::Tonemap::to_TonemapDrago", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tonemap_to_TonemapDrago(instance: *mut c_void) -> *mut c_void;
// cv::Tonemap::to_TonemapMantiuk() generated
// ("cv::Tonemap::to_TonemapMantiuk", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tonemap_to_TonemapMantiuk(instance: *mut c_void) -> *mut c_void;
// cv::Tonemap::to_TonemapReinhard() generated
// ("cv::Tonemap::to_TonemapReinhard", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tonemap_to_TonemapReinhard(instance: *mut c_void) -> *mut c_void;
// cv::Tonemap::to_Algorithm() generated
// ("cv::Tonemap::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tonemap_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::Tonemap::delete() generated
// ("cv::Tonemap::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tonemap_delete(instance: *mut c_void);
// getSaturation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:371
// ("cv::TonemapDrago::getSaturation", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapDrago_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:372
// ("cv::TonemapDrago::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
pub fn cv_TonemapDrago_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result<()>);
// getBias()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:374
// ("cv::TonemapDrago::getBias", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapDrago_getBias_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setBias(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:375
// ("cv::TonemapDrago::setBias", vec![(pred!(mut, ["bias"], ["float"]), _)]),
pub fn cv_TonemapDrago_setBias_float(instance: *mut c_void, bias: f32, ocvrs_return: *mut Result<()>);
// cv::TonemapDrago::to_Algorithm() generated
// ("cv::TonemapDrago::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapDrago_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TonemapDrago::to_Tonemap() generated
// ("cv::TonemapDrago::to_Tonemap", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapDrago_to_Tonemap(instance: *mut c_void) -> *mut c_void;
// cv::TonemapDrago::delete() generated
// ("cv::TonemapDrago::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapDrago_delete(instance: *mut c_void);
// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:430
// ("cv::TonemapMantiuk::getScale", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapMantiuk_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:431
// ("cv::TonemapMantiuk::setScale", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_TonemapMantiuk_setScale_float(instance: *mut c_void, scale: f32, ocvrs_return: *mut Result<()>);
// getSaturation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:433
// ("cv::TonemapMantiuk::getSaturation", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapMantiuk_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:434
// ("cv::TonemapMantiuk::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
pub fn cv_TonemapMantiuk_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result<()>);
// cv::TonemapMantiuk::to_Algorithm() generated
// ("cv::TonemapMantiuk::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapMantiuk_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TonemapMantiuk::to_Tonemap() generated
// ("cv::TonemapMantiuk::to_Tonemap", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapMantiuk_to_Tonemap(instance: *mut c_void) -> *mut c_void;
// cv::TonemapMantiuk::delete() generated
// ("cv::TonemapMantiuk::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapMantiuk_delete(instance: *mut c_void);
// getIntensity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:399
// ("cv::TonemapReinhard::getIntensity", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapReinhard_getIntensity_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setIntensity(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:400
// ("cv::TonemapReinhard::setIntensity", vec![(pred!(mut, ["intensity"], ["float"]), _)]),
pub fn cv_TonemapReinhard_setIntensity_float(instance: *mut c_void, intensity: f32, ocvrs_return: *mut Result<()>);
// getLightAdaptation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:402
// ("cv::TonemapReinhard::getLightAdaptation", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapReinhard_getLightAdaptation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setLightAdaptation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:403
// ("cv::TonemapReinhard::setLightAdaptation", vec![(pred!(mut, ["light_adapt"], ["float"]), _)]),
pub fn cv_TonemapReinhard_setLightAdaptation_float(instance: *mut c_void, light_adapt: f32, ocvrs_return: *mut Result<()>);
// getColorAdaptation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:405
// ("cv::TonemapReinhard::getColorAdaptation", vec![(pred!(const, [], []), _)]),
pub fn cv_TonemapReinhard_getColorAdaptation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setColorAdaptation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:406
// ("cv::TonemapReinhard::setColorAdaptation", vec![(pred!(mut, ["color_adapt"], ["float"]), _)]),
pub fn cv_TonemapReinhard_setColorAdaptation_float(instance: *mut c_void, color_adapt: f32, ocvrs_return: *mut Result<()>);
// cv::TonemapReinhard::to_Algorithm() generated
// ("cv::TonemapReinhard::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapReinhard_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TonemapReinhard::to_Tonemap() generated
// ("cv::TonemapReinhard::to_Tonemap", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapReinhard_to_Tonemap(instance: *mut c_void) -> *mut c_void;
// cv::TonemapReinhard::delete() generated
// ("cv::TonemapReinhard::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TonemapReinhard_delete(instance: *mut c_void);
