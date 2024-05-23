// createBFMatcher(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:92
// ("cv::cuda::DescriptorMatcher::createBFMatcher", vec![(pred!(mut, ["normType"], ["int"]), _)]),
pub fn cv_cuda_DescriptorMatcher_createBFMatcher_int(norm_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::DescriptorMatcher::createBFMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:92
// ("cv::cuda::DescriptorMatcher::createBFMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_createBFMatcher(ocvrs_return: *mut Result<*mut c_void>);
// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:100
// ("cv::cuda::DescriptorMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// add(const std::vector<GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:113
// ("cv::cuda::DescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_add_const_vectorLGpuMatGR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:117
// ("cv::cuda::DescriptorMatcher::getTrainDescriptors", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_getTrainDescriptors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:121
// ("cv::cuda::DescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:125
// ("cv::cuda::DescriptorMatcher::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// train()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:132
// ("cv::cuda::DescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:154
// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:154
// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// match(InputArray, std::vector<DMatch> &, const std::vector<GpuMat> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:160
// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*", "const std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const_vectorLGpuMatGR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:160
// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// matchAsync(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:181
// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::matchAsync(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:181
// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, ocvrs_return: *mut Result<()>);
// matchAsync(InputArray, OutputArray, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:188
// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR_const_vectorLGpuMatGR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, masks: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::matchAsync(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:188
// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, ocvrs_return: *mut Result<()>);
// matchConvert(InputArray, std::vector<DMatch> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:201
// ("cv::cuda::DescriptorMatcher::matchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_matchConvert_const__InputArrayR_vectorLDMatchGR(instance: *mut c_void, gpu_matches: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:226
// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:226
// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, const std::vector<GpuMat> &, bool)(InputArray, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:234
// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const std::vector<cv::cuda::GpuMat>*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const_vectorLGpuMatGR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:234
// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// knnMatchAsync(InputArray, InputArray, OutputArray, int, InputArray, Stream &)(InputArray, InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:257
// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, k: i32, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::knnMatchAsync(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:257
// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, k: i32, ocvrs_return: *mut Result<()>);
// knnMatchAsync(InputArray, OutputArray, int, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:265
// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int_const_vectorLGpuMatGR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, k: i32, masks: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::knnMatchAsync(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:265
// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, k: i32, ocvrs_return: *mut Result<()>);
// knnMatchConvert(InputArray, std::vector<std::vector<DMatch>> &, bool)(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:282
// ("cv::cuda::DescriptorMatcher::knnMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(instance: *mut c_void, gpu_matches: *const c_void, matches: *mut c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::knnMatchConvert(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:282
// ("cv::cuda::DescriptorMatcher::knnMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(instance: *mut c_void, gpu_matches: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:309
// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:309
// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, const std::vector<GpuMat> &, bool)(InputArray, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:317
// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const std::vector<cv::cuda::GpuMat>*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const_vectorLGpuMatGR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:317
// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// radiusMatchAsync(InputArray, InputArray, OutputArray, float, InputArray, Stream &)(InputArray, InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:341
// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_const__InputArrayR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, max_distance: f32, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::radiusMatchAsync(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:341
// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float(instance: *mut c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *const c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// radiusMatchAsync(InputArray, OutputArray, float, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:349
// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float_const_vectorLGpuMatGR_StreamR(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, max_distance: f32, masks: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::radiusMatchAsync(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:349
// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float(instance: *mut c_void, query_descriptors: *const c_void, matches: *const c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// radiusMatchConvert(InputArray, std::vector<std::vector<DMatch>> &, bool)(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:366
// ("cv::cuda::DescriptorMatcher::radiusMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "bool"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(instance: *mut c_void, gpu_matches: *const c_void, matches: *mut c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::radiusMatchConvert(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:366
// ("cv::cuda::DescriptorMatcher::radiusMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*"]), _)]),
pub fn cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(instance: *mut c_void, gpu_matches: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DescriptorMatcher::to_Algorithm() generated
// ("cv::cuda::DescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DescriptorMatcher::delete() generated
// ("cv::cuda::DescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DescriptorMatcher_delete(instance: *mut c_void);
// create(int, bool, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:434
// ("cv::cuda::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type", "max_npoints"], ["int", "bool", "int", "int"]), _)]),
pub fn cv_cuda_FastFeatureDetector_create_int_bool_int_int(threshold: i32, nonmax_suppression: bool, typ: i32, max_npoints: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:434
// ("cv::cuda::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:438
// ("cv::cuda::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_cuda_FastFeatureDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// setMaxNumPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:440
// ("cv::cuda::FastFeatureDetector::setMaxNumPoints", vec![(pred!(mut, ["max_npoints"], ["int"]), _)]),
pub fn cv_cuda_FastFeatureDetector_setMaxNumPoints_int(instance: *mut c_void, max_npoints: i32, ocvrs_return: *mut Result<()>);
// getMaxNumPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:441
// ("cv::cuda::FastFeatureDetector::getMaxNumPoints", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_getMaxNumPoints_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::FastFeatureDetector::to_Algorithm() generated
// ("cv::cuda::FastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::FastFeatureDetector::to_Feature2D() generated
// ("cv::cuda::FastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::cuda::FastFeatureDetector::to_CUDA_Feature2DAsync() generated
// ("cv::cuda::FastFeatureDetector::to_CUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_to_CUDA_Feature2DAsync(instance: *mut c_void) -> *mut c_void;
// cv::cuda::FastFeatureDetector::delete() generated
// ("cv::cuda::FastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FastFeatureDetector_delete(instance: *mut c_void);
// detectAsync(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:390
// ("cv::cuda::Feature2DAsync::detectAsync", vec![(pred!(mut, ["image", "keypoints", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(instance: *mut c_void, image: *const c_void, keypoints: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Feature2DAsync::detectAsync(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:390
// ("cv::cuda::Feature2DAsync::detectAsync", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *const c_void, ocvrs_return: *mut Result<()>);
// computeAsync(InputArray, OutputArray, OutputArray, Stream &)(InputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:402
// ("cv::cuda::Feature2DAsync::computeAsync", vec![(pred!(mut, ["image", "keypoints", "descriptors", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, image: *const c_void, keypoints: *const c_void, descriptors: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Feature2DAsync::computeAsync(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:402
// ("cv::cuda::Feature2DAsync::computeAsync", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// detectAndComputeAsync(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:408
// ("cv::cuda::Feature2DAsync::detectAndComputeAsync", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *const c_void, descriptors: *const c_void, use_provided_keypoints: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Feature2DAsync::detectAndComputeAsync(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:408
// ("cv::cuda::Feature2DAsync::detectAndComputeAsync", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// convert(InputArray, std::vector<KeyPoint> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:416
// ("cv::cuda::Feature2DAsync::convert", vec![(pred!(mut, ["gpu_keypoints", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_cuda_Feature2DAsync_convert_const__InputArrayR_vectorLKeyPointGR(instance: *mut c_void, gpu_keypoints: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Feature2DAsync::to_CUDA_FastFeatureDetector() generated
// ("cv::cuda::Feature2DAsync::to_CUDA_FastFeatureDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Feature2DAsync_to_CUDA_FastFeatureDetector(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Feature2DAsync::to_CUDA_ORB() generated
// ("cv::cuda::Feature2DAsync::to_CUDA_ORB", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Feature2DAsync_to_CUDA_ORB(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Feature2DAsync::to_Algorithm() generated
// ("cv::cuda::Feature2DAsync::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Feature2DAsync_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Feature2DAsync::to_Feature2D() generated
// ("cv::cuda::Feature2DAsync::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Feature2DAsync_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Feature2DAsync::delete() generated
// ("cv::cuda::Feature2DAsync::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Feature2DAsync_delete(instance: *mut c_void);
// create(int, float, int, int, int, int, int, int, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:463
// ("cv::cuda::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold", "blurForDescriptor"], ["int", "float", "int", "int", "int", "int", "int", "int", "int", "bool"]), _)]),
pub fn cv_cuda_ORB_create_int_float_int_int_int_int_int_int_int_bool(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: i32, patch_size: i32, fast_threshold: i32, blur_for_descriptor: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::ORB::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:463
// ("cv::cuda::ORB::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ORB_create(ocvrs_return: *mut Result<*mut c_void>);
// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:474
// ("cv::cuda::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_cuda_ORB_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:475
// ("cv::cuda::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:477
// ("cv::cuda::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
pub fn cv_cuda_ORB_setScaleFactor_double(instance: *mut c_void, scale_factor: f64, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:478
// ("cv::cuda::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:480
// ("cv::cuda::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
pub fn cv_cuda_ORB_setNLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result<()>);
// getNLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:481
// ("cv::cuda::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getNLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:483
// ("cv::cuda::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
pub fn cv_cuda_ORB_setEdgeThreshold_int(instance: *mut c_void, edge_threshold: i32, ocvrs_return: *mut Result<()>);
// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:484
// ("cv::cuda::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getEdgeThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:486
// ("cv::cuda::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
pub fn cv_cuda_ORB_setFirstLevel_int(instance: *mut c_void, first_level: i32, ocvrs_return: *mut Result<()>);
// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:487
// ("cv::cuda::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getFirstLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:489
// ("cv::cuda::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
pub fn cv_cuda_ORB_setWTA_K_int(instance: *mut c_void, wta_k: i32, ocvrs_return: *mut Result<()>);
// getWTA_K()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:490
// ("cv::cuda::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getWTA_K_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScoreType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:492
// ("cv::cuda::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["int"]), _)]),
pub fn cv_cuda_ORB_setScoreType_int(instance: *mut c_void, score_type: i32, ocvrs_return: *mut Result<()>);
// getScoreType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:493
// ("cv::cuda::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getScoreType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:495
// ("cv::cuda::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
pub fn cv_cuda_ORB_setPatchSize_int(instance: *mut c_void, patch_size: i32, ocvrs_return: *mut Result<()>);
// getPatchSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:496
// ("cv::cuda::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:498
// ("cv::cuda::ORB::setFastThreshold", vec![(pred!(mut, ["fastThreshold"], ["int"]), _)]),
pub fn cv_cuda_ORB_setFastThreshold_int(instance: *mut c_void, fast_threshold: i32, ocvrs_return: *mut Result<()>);
// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:499
// ("cv::cuda::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getFastThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBlurForDescriptor(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:502
// ("cv::cuda::ORB::setBlurForDescriptor", vec![(pred!(mut, ["blurForDescriptor"], ["bool"]), _)]),
pub fn cv_cuda_ORB_setBlurForDescriptor_bool(instance: *mut c_void, blur_for_descriptor: bool, ocvrs_return: *mut Result<()>);
// getBlurForDescriptor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:503
// ("cv::cuda::ORB::getBlurForDescriptor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_ORB_getBlurForDescriptor_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::cuda::ORB::to_Algorithm() generated
// ("cv::cuda::ORB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ORB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::ORB::to_Feature2D() generated
// ("cv::cuda::ORB::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ORB_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::cuda::ORB::to_CUDA_Feature2DAsync() generated
// ("cv::cuda::ORB::to_CUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ORB_to_CUDA_Feature2DAsync(instance: *mut c_void) -> *mut c_void;
// cv::cuda::ORB::delete() generated
// ("cv::cuda::ORB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_ORB_delete(instance: *mut c_void);
