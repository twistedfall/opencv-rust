// cv::xfeatures2d::FASTForPointSet(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:963
// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
pub fn cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int(image: *const c_void, keypoints: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// FASTForPointSet(InputArray, std::vector<KeyPoint> &, int, bool, int)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:963
// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "int"]), _)]),
pub fn cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int_bool_int(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: i32, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::matchGMS(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:989
// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR(size1: *const core::Size, size2: *const core::Size, keypoints1: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, matches_gms: *mut c_void, ocvrs_return: *mut Result<()>);
// matchGMS(const Size &, const Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double)(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:989
// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS", "withRotation", "withScale", "thresholdFactor"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*", "const bool", "const bool", "const double"]), _)]),
pub fn cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR_const_bool_const_bool_const_double(size1: *const core::Size, size2: *const core::Size, keypoints1: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, matches_gms: *mut c_void, with_rotation: bool, with_scale: bool, threshold_factor: f64, ocvrs_return: *mut Result<()>);
// SURF_CUDA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:102
// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_SURF_CUDA(ocvrs_return: *mut Result<*mut c_void>);
// SURF_CUDA(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:104
// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::SURF_CUDA::SURF_CUDA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:104
// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
pub fn cv_cuda_SURF_CUDA_SURF_CUDA_double(_hessian_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// descriptorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:108
// ("cv::cuda::SURF_CUDA::descriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// defaultNorm()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:110
// ("cv::cuda::SURF_CUDA::defaultNorm", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// uploadKeypoints(const std::vector<KeyPoint> &, GpuMat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:113
// ("cv::cuda::SURF_CUDA::uploadKeypoints", vec![(pred!(mut, ["keypoints", "keypointsGPU"], ["const std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_SURF_CUDA_uploadKeypoints_const_vectorLKeyPointGR_GpuMatR(instance: *mut c_void, keypoints: *const c_void, keypoints_gpu: *mut c_void, ocvrs_return: *mut Result<()>);
// downloadKeypoints(const GpuMat &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:115
// ("cv::cuda::SURF_CUDA::downloadKeypoints", vec![(pred!(mut, ["keypointsGPU", "keypoints"], ["const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vectorLKeyPointGR(instance: *mut c_void, keypoints_gpu: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// downloadDescriptors(const GpuMat &, std::vector<float> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:118
// ("cv::cuda::SURF_CUDA::downloadDescriptors", vec![(pred!(mut, ["descriptorsGPU", "descriptors"], ["const cv::cuda::GpuMat*", "std::vector<float>*"]), _)]),
pub fn cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vectorLfloatGR(instance: *mut c_void, descriptors_gpu: *const c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:130
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:133
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:133
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:136
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, GpuMat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:137
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*", "bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR_bool(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:137
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, std::vector<float> &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:140
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*", "bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR_bool(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result<()>);
// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:140
// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*"]), _)]),
pub fn cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// releaseMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:143
// ("cv::cuda::SURF_CUDA::releaseMemory", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_releaseMemory(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::SURF_CUDA::hessianThreshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:146
// ("cv::cuda::SURF_CUDA::hessianThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propHessianThreshold_const(instance: *const c_void) -> f64;
// cv::cuda::SURF_CUDA::setHessianThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:146
// ("cv::cuda::SURF_CUDA::setHessianThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_cuda_SURF_CUDA_propHessianThreshold_const_double(instance: *mut c_void, val: f64);
// cv::cuda::SURF_CUDA::nOctaves() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:147
// ("cv::cuda::SURF_CUDA::nOctaves", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propNOctaves_const(instance: *const c_void) -> i32;
// cv::cuda::SURF_CUDA::setNOctaves(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:147
// ("cv::cuda::SURF_CUDA::setNOctaves", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_SURF_CUDA_propNOctaves_const_int(instance: *mut c_void, val: i32);
// cv::cuda::SURF_CUDA::nOctaveLayers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:148
// ("cv::cuda::SURF_CUDA::nOctaveLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propNOctaveLayers_const(instance: *const c_void) -> i32;
// cv::cuda::SURF_CUDA::setNOctaveLayers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:148
// ("cv::cuda::SURF_CUDA::setNOctaveLayers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_SURF_CUDA_propNOctaveLayers_const_int(instance: *mut c_void, val: i32);
// cv::cuda::SURF_CUDA::extended() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:149
// ("cv::cuda::SURF_CUDA::extended", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propExtended_const(instance: *const c_void) -> bool;
// cv::cuda::SURF_CUDA::setExtended(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:149
// ("cv::cuda::SURF_CUDA::setExtended", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_propExtended_const_bool(instance: *mut c_void, val: bool);
// cv::cuda::SURF_CUDA::upright() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:150
// ("cv::cuda::SURF_CUDA::upright", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propUpright_const(instance: *const c_void) -> bool;
// cv::cuda::SURF_CUDA::setUpright(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:150
// ("cv::cuda::SURF_CUDA::setUpright", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_cuda_SURF_CUDA_propUpright_const_bool(instance: *mut c_void, val: bool);
// cv::cuda::SURF_CUDA::keypointsRatio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:153
// ("cv::cuda::SURF_CUDA::keypointsRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propKeypointsRatio_const(instance: *const c_void) -> f32;
// cv::cuda::SURF_CUDA::setKeypointsRatio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:153
// ("cv::cuda::SURF_CUDA::setKeypointsRatio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_cuda_SURF_CUDA_propKeypointsRatio_const_float(instance: *mut c_void, val: f32);
// cv::cuda::SURF_CUDA::sum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::sum", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propSum_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setSum(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::setSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propSum_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::mask1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::mask1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propMask1_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setMask1(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::setMask1", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propMask1_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::maskSum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::maskSum", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propMaskSum_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setMaskSum(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
// ("cv::cuda::SURF_CUDA::setMaskSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propMaskSum_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::det() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
// ("cv::cuda::SURF_CUDA::det", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propDet_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setDet(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
// ("cv::cuda::SURF_CUDA::setDet", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propDet_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::trace() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
// ("cv::cuda::SURF_CUDA::trace", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propTrace_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setTrace(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
// ("cv::cuda::SURF_CUDA::setTrace", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propTrace_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::maxPosBuffer() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:159
// ("cv::cuda::SURF_CUDA::maxPosBuffer", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_propMaxPosBuffer_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::SURF_CUDA::setMaxPosBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:159
// ("cv::cuda::SURF_CUDA::setMaxPosBuffer", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
pub fn cv_cuda_SURF_CUDA_propMaxPosBuffer_const_GpuMat(instance: *mut c_void, val: *const c_void);
// cv::cuda::SURF_CUDA::delete() generated
// ("cv::cuda::SURF_CUDA::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SURF_CUDA_delete(instance: *mut c_void);
// create(Ptr<FeatureDetector>, Ptr<DescriptorExtractor>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:910
// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector", "descriptor_extractor"], ["cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG_PtrLFeature2DG(keypoint_detector: *mut c_void, descriptor_extractor: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:918
// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG(keypoint_detector: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// detect(InputArray, std::vector<Elliptic_KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:929
// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_InputArray*"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR_const__InputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::AffineFeature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:929
// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// detectAndCompute(InputArray, InputArray, std::vector<Elliptic_KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:939
// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::AffineFeature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:939
// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::AffineFeature2D::to_Algorithm() generated
// ("cv::xfeatures2d::AffineFeature2D::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::AffineFeature2D::to_Feature2D() generated
// ("cv::xfeatures2d::AffineFeature2D::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::AffineFeature2D::delete() generated
// ("cv::xfeatures2d::AffineFeature2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_AffineFeature2D_delete(instance: *mut c_void);
// create(int, bool, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:373
// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, ["desc", "use_scale_orientation", "scale_factor"], ["int", "bool", "float"]), _)]),
pub fn cv_xfeatures2d_BoostDesc_create_int_bool_float(desc: i32, use_scale_orientation: bool, scale_factor: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::BoostDesc::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:373
// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_create(ocvrs_return: *mut Result<*mut c_void>);
// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:376
// ("cv::xfeatures2d::BoostDesc::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
pub fn cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(instance: *mut c_void, use_scale_orientation: bool, ocvrs_return: *mut Result<()>);
// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:377
// ("cv::xfeatures2d::BoostDesc::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:379
// ("cv::xfeatures2d::BoostDesc::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
pub fn cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(instance: *mut c_void, scale_factor: f32, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:380
// ("cv::xfeatures2d::BoostDesc::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::xfeatures2d::BoostDesc::to_Algorithm() generated
// ("cv::xfeatures2d::BoostDesc::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::BoostDesc::to_Feature2D() generated
// ("cv::xfeatures2d::BoostDesc::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::BoostDesc::delete() generated
// ("cv::xfeatures2d::BoostDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BoostDesc_delete(instance: *mut c_void);
// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:132
// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, ["bytes", "use_orientation"], ["int", "bool"]), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes: i32, use_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::BriefDescriptorExtractor::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:132
// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::BriefDescriptorExtractor::defaultNew() generated
// ("cv::xfeatures2d::BriefDescriptorExtractor::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::BriefDescriptorExtractor::to_Algorithm() generated
// ("cv::xfeatures2d::BriefDescriptorExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::BriefDescriptorExtractor::to_Feature2D() generated
// ("cv::xfeatures2d::BriefDescriptorExtractor::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::BriefDescriptorExtractor::delete() generated
// ("cv::xfeatures2d::BriefDescriptorExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_BriefDescriptorExtractor_delete(instance: *mut c_void);
// create(float, int, int, int, int, InputArray, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:204
// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, ["radius", "q_radius", "q_theta", "q_hist", "norm", "H", "interpolation", "use_orientation"], ["float", "int", "int", "int", "int", "const cv::_InputArray*", "bool", "bool"]), _)]),
pub fn cv_xfeatures2d_DAISY_create_float_int_int_int_int_const__InputArrayR_bool_bool(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: i32, h: *const c_void, interpolation: bool, use_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::DAISY::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:204
// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_DAISY_create(ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:213
// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:215
// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, Rect, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:224
// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "roi", "descriptors"], ["const cv::_InputArray*", "cv::Rect", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(instance: *mut c_void, image: *const c_void, roi: *const core::Rect, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:230
// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// GetDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:238
// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
pub fn cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, ocvrs_return: *mut Result<()>);
// GetDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:247
// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
pub fn cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, h: *mut f64, ocvrs_return: *mut Result<bool>);
// GetUnnormalizedDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:255
// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
pub fn cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, ocvrs_return: *mut Result<()>);
// GetUnnormalizedDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:264
// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
pub fn cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, h: *mut f64, ocvrs_return: *mut Result<bool>);
// cv::xfeatures2d::DAISY::to_Algorithm() generated
// ("cv::xfeatures2d::DAISY::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_DAISY_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::DAISY::to_Feature2D() generated
// ("cv::xfeatures2d::DAISY::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_DAISY_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::DAISY::delete() generated
// ("cv::xfeatures2d::DAISY::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_DAISY_delete(instance: *mut c_void);
// Elliptic_KeyPoint()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:865
// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(ocvrs_return: *mut Result<*mut c_void>);
// Elliptic_KeyPoint(Point2f, float, Size, float, float)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:866
// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, ["pt", "angle", "axes", "size", "si"], ["cv::Point2f", "float", "cv::Size", "float", "float"]), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(pt: *const core::Point2f, angle: f32, axes: *const core::Size, size: f32, si: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::Elliptic_KeyPoint::axes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:862
// ("cv::xfeatures2d::Elliptic_KeyPoint::axes", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const(instance: *const c_void, ocvrs_return: *mut core::Size_<f32>);
// cv::xfeatures2d::Elliptic_KeyPoint::setAxes(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:862
// ("cv::xfeatures2d::Elliptic_KeyPoint::setAxes", vec![(pred!(mut, ["val"], ["const cv::Size_<float>"]), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const_Size_LfloatG(instance: *mut c_void, val: *const core::Size_<f32>);
// cv::xfeatures2d::Elliptic_KeyPoint::si() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:863
// ("cv::xfeatures2d::Elliptic_KeyPoint::si", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propSi_const(instance: *const c_void) -> f32;
// cv::xfeatures2d::Elliptic_KeyPoint::setSi(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:863
// ("cv::xfeatures2d::Elliptic_KeyPoint::setSi", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propSi_const_float(instance: *mut c_void, val: f32);
// cv::xfeatures2d::Elliptic_KeyPoint::transf() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:864
// ("cv::xfeatures2d::Elliptic_KeyPoint::transf", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const(instance: *const c_void, ocvrs_return: *mut core::Matx23f);
// cv::xfeatures2d::Elliptic_KeyPoint::setTransf(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:864
// ("cv::xfeatures2d::Elliptic_KeyPoint::setTransf", vec![(pred!(mut, ["val"], ["const cv::Matx23f"]), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const_Matx23f(instance: *mut c_void, val: *const core::Matx23f);
// cv::xfeatures2d::Elliptic_KeyPoint::to_KeyPoint() generated
// ("cv::xfeatures2d::Elliptic_KeyPoint::to_KeyPoint", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_to_KeyPoint(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::Elliptic_KeyPoint::delete() generated
// ("cv::xfeatures2d::Elliptic_KeyPoint::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_Elliptic_KeyPoint_delete(instance: *mut c_void);
// create(bool, bool, float, int, const std::vector<int> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:99
// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, ["orientationNormalized", "scaleNormalized", "patternScale", "nOctaves", "selectedPairs"], ["bool", "bool", "float", "int", "const std::vector<int>*"]), _)]),
pub fn cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vectorLintGR(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::FREAK::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:99
// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_FREAK_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::FREAK::defaultNew() generated
// ("cv::xfeatures2d::FREAK::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_FREAK_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::FREAK::to_Algorithm() generated
// ("cv::xfeatures2d::FREAK::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_FREAK_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::FREAK::to_Feature2D() generated
// ("cv::xfeatures2d::FREAK::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_FREAK_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::FREAK::delete() generated
// ("cv::xfeatures2d::FREAK::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_FREAK_delete(instance: *mut c_void);
// create(int, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:885
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, ["numOctaves", "corn_thresh", "DOG_thresh", "maxCorners", "num_layers"], ["int", "float", "float", "int", "int"]), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::HarrisLaplaceFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:885
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::HarrisLaplaceFeatureDetector::defaultNew() generated
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Algorithm() generated
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Feature2D() generated
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::HarrisLaplaceFeatureDetector::delete() generated
// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_delete(instance: *mut c_void);
// create(int, bool, int, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:178
// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, ["bytes", "rotationInvariance", "half_ssd_size", "sigma"], ["int", "bool", "int", "double"]), _)]),
pub fn cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::LATCH::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:178
// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LATCH_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::LATCH::defaultNew() generated
// ("cv::xfeatures2d::LATCH::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_LATCH_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::LATCH::to_Algorithm() generated
// ("cv::xfeatures2d::LATCH::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LATCH_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::LATCH::to_Feature2D() generated
// ("cv::xfeatures2d::LATCH::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LATCH_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::LATCH::delete() generated
// ("cv::xfeatures2d::LATCH::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LATCH_delete(instance: *mut c_void);
// create(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:149
// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, ["lucid_kernel", "blur_kernel"], ["const int", "const int"]), _)]),
pub fn cv_xfeatures2d_LUCID_create_const_int_const_int(lucid_kernel: i32, blur_kernel: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::LUCID::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:149
// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LUCID_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::LUCID::defaultNew() generated
// ("cv::xfeatures2d::LUCID::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_LUCID_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::LUCID::to_Algorithm() generated
// ("cv::xfeatures2d::LUCID::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LUCID_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::LUCID::to_Feature2D() generated
// ("cv::xfeatures2d::LUCID::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LUCID_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::LUCID::delete() generated
// ("cv::xfeatures2d::LUCID::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_LUCID_delete(instance: *mut c_void);
// create(int, int, int, int, float, int, float, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:285
// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, ["m_patch_radius", "m_search_area_radius", "m_nms_radius", "m_nms_scale_radius", "m_th_saliency", "m_kNN", "m_scale_factor", "m_n_scales", "m_compute_orientation"], ["int", "int", "int", "int", "float", "int", "float", "int", "bool"]), _)]),
pub fn cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::MSDDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:285
// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_MSDDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::MSDDetector::defaultNew() generated
// ("cv::xfeatures2d::MSDDetector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_MSDDetector_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::MSDDetector::to_Algorithm() generated
// ("cv::xfeatures2d::MSDDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_MSDDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::MSDDetector::to_Feature2D() generated
// ("cv::xfeatures2d::MSDDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_MSDDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::MSDDetector::delete() generated
// ("cv::xfeatures2d::MSDDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_MSDDetector_delete(instance: *mut c_void);
// create(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:451
// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSampleCount", "initSeedCount", "pointDistribution"], ["const int", "const int", "const int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(init_sample_count: i32, init_seed_count: i32, point_distribution: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::PCTSignatures::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:451
// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_create(ocvrs_return: *mut Result<*mut c_void>);
// create(const std::vector<Point2f> &, const int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:465
// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initSeedCount"], ["const std::vector<cv::Point2f>*", "const int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_int(init_sampling_points: *const c_void, init_seed_count: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(const std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:477
// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initClusterSeedIndexes"], ["const std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_vectorLintGR(init_sampling_points: *const c_void, init_cluster_seed_indexes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// computeSignature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:488
// ("cv::xfeatures2d::PCTSignatures::computeSignature", vec![(pred!(const, ["image", "signature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, signature: *const c_void, ocvrs_return: *mut Result<()>);
// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:497
// ("cv::xfeatures2d::PCTSignatures::computeSignatures", vec![(pred!(const, ["images", "signatures"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vectorLMatGR_vectorLMatGR(instance: *const c_void, images: *const c_void, signatures: *mut c_void, ocvrs_return: *mut Result<()>);
// drawSignature(InputArray, InputArray, OutputArray, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:513
// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result", "radiusToShorterSideRatio", "borderThickness"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(source: *const c_void, signature: *const c_void, result: *const c_void, radius_to_shorter_side_ratio: f32, border_thickness: i32, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::PCTSignatures::drawSignature(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:513
// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR(source: *const c_void, signature: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// generateInitPoints(std::vector<Point2f> &, const int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:528
// ("cv::xfeatures2d::PCTSignatures::generateInitPoints", vec![(pred!(mut, ["initPoints", "count", "pointDistribution"], ["std::vector<cv::Point2f>*", "const int", "int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_generateInitPoints_vectorLPoint2fGR_const_int_int(init_points: *mut c_void, count: i32, point_distribution: i32, ocvrs_return: *mut Result<()>);
// getSampleCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:539
// ("cv::xfeatures2d::PCTSignatures::getSampleCount", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getSampleCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getGrayscaleBits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:546
// ("cv::xfeatures2d::PCTSignatures::getGrayscaleBits", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGrayscaleBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:552
// ("cv::xfeatures2d::PCTSignatures::setGrayscaleBits", vec![(pred!(mut, ["grayscaleBits"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(instance: *mut c_void, grayscale_bits: i32, ocvrs_return: *mut Result<()>);
// getWindowRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:559
// ("cv::xfeatures2d::PCTSignatures::getWindowRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWindowRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:565
// ("cv::xfeatures2d::PCTSignatures::setWindowRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWindowRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result<()>);
// getWeightX()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:572
// ("cv::xfeatures2d::PCTSignatures::getWeightX", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightX_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightX(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:577
// ("cv::xfeatures2d::PCTSignatures::setWeightX", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightX_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightY()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:583
// ("cv::xfeatures2d::PCTSignatures::getWeightY", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightY_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightY(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:588
// ("cv::xfeatures2d::PCTSignatures::setWeightY", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightY_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightL()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:594
// ("cv::xfeatures2d::PCTSignatures::getWeightL", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightL_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightL(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:599
// ("cv::xfeatures2d::PCTSignatures::setWeightL", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightL_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:605
// ("cv::xfeatures2d::PCTSignatures::getWeightA", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightA_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightA(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:610
// ("cv::xfeatures2d::PCTSignatures::setWeightA", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightA_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightB()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:616
// ("cv::xfeatures2d::PCTSignatures::getWeightB", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightB_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightB(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:621
// ("cv::xfeatures2d::PCTSignatures::setWeightB", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightB_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightContrast()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:627
// ("cv::xfeatures2d::PCTSignatures::getWeightContrast", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightContrast_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:632
// ("cv::xfeatures2d::PCTSignatures::setWeightContrast", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightContrast_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getWeightEntropy()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:638
// ("cv::xfeatures2d::PCTSignatures::getWeightEntropy", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setWeightEntropy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:643
// ("cv::xfeatures2d::PCTSignatures::setWeightEntropy", vec![(pred!(mut, ["weight"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// getSamplingPoints()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:649
// ("cv::xfeatures2d::PCTSignatures::getSamplingPoints", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setWeight(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:667
// ("cv::xfeatures2d::PCTSignatures::setWeight", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeight_int_float(instance: *mut c_void, idx: i32, value: f32, ocvrs_return: *mut Result<()>);
// setWeights(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:681
// ("cv::xfeatures2d::PCTSignatures::setWeights", vec![(pred!(mut, ["weights"], ["const std::vector<float>*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setWeights_const_vectorLfloatGR(instance: *mut c_void, weights: *const c_void, ocvrs_return: *mut Result<()>);
// setTranslation(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:697
// ("cv::xfeatures2d::PCTSignatures::setTranslation", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setTranslation_int_float(instance: *mut c_void, idx: i32, value: f32, ocvrs_return: *mut Result<()>);
// setTranslations(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:711
// ("cv::xfeatures2d::PCTSignatures::setTranslations", vec![(pred!(mut, ["translations"], ["const std::vector<float>*"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setTranslations_const_vectorLfloatGR(instance: *mut c_void, translations: *const c_void, ocvrs_return: *mut Result<()>);
// setSamplingPoints(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:718
// ("cv::xfeatures2d::PCTSignatures::setSamplingPoints", vec![(pred!(mut, ["samplingPoints"], ["std::vector<cv::Point2f>"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setSamplingPoints_vectorLPoint2fG(instance: *mut c_void, sampling_points: *mut c_void, ocvrs_return: *mut Result<()>);
// getInitSeedIndexes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:726
// ("cv::xfeatures2d::PCTSignatures::getInitSeedIndexes", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setInitSeedIndexes(std::vector<int>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:730
// ("cv::xfeatures2d::PCTSignatures::setInitSeedIndexes", vec![(pred!(mut, ["initSeedIndexes"], ["std::vector<int>"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vectorLintG(instance: *mut c_void, init_seed_indexes: *mut c_void, ocvrs_return: *mut Result<()>);
// getInitSeedCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:734
// ("cv::xfeatures2d::PCTSignatures::getInitSeedCount", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getIterationCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:741
// ("cv::xfeatures2d::PCTSignatures::getIterationCount", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getIterationCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterationCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:747
// ("cv::xfeatures2d::PCTSignatures::setIterationCount", vec![(pred!(mut, ["iterationCount"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setIterationCount_int(instance: *mut c_void, iteration_count: i32, ocvrs_return: *mut Result<()>);
// getMaxClustersCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:753
// ("cv::xfeatures2d::PCTSignatures::getMaxClustersCount", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxClustersCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:758
// ("cv::xfeatures2d::PCTSignatures::setMaxClustersCount", vec![(pred!(mut, ["maxClustersCount"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(instance: *mut c_void, max_clusters_count: i32, ocvrs_return: *mut Result<()>);
// getClusterMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:765
// ("cv::xfeatures2d::PCTSignatures::getClusterMinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setClusterMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:771
// ("cv::xfeatures2d::PCTSignatures::setClusterMinSize", vec![(pred!(mut, ["clusterMinSize"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(instance: *mut c_void, cluster_min_size: i32, ocvrs_return: *mut Result<()>);
// getJoiningDistance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:778
// ("cv::xfeatures2d::PCTSignatures::getJoiningDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setJoiningDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:784
// ("cv::xfeatures2d::PCTSignatures::setJoiningDistance", vec![(pred!(mut, ["joiningDistance"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(instance: *mut c_void, joining_distance: f32, ocvrs_return: *mut Result<()>);
// getDropThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:789
// ("cv::xfeatures2d::PCTSignatures::getDropThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getDropThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setDropThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:793
// ("cv::xfeatures2d::PCTSignatures::setDropThreshold", vec![(pred!(mut, ["dropThreshold"], ["float"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setDropThreshold_float(instance: *mut c_void, drop_threshold: f32, ocvrs_return: *mut Result<()>);
// getDistanceFunction()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:798
// ("cv::xfeatures2d::PCTSignatures::getDistanceFunction", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDistanceFunction(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:803
// ("cv::xfeatures2d::PCTSignatures::setDistanceFunction", vec![(pred!(mut, ["distanceFunction"], ["int"]), _)]),
pub fn cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(instance: *mut c_void, distance_function: i32, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::PCTSignatures::to_Algorithm() generated
// ("cv::xfeatures2d::PCTSignatures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::PCTSignatures::delete() generated
// ("cv::xfeatures2d::PCTSignatures::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignatures_delete(instance: *mut c_void);
// create(const int, const int, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:828
// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, ["distanceFunction", "similarityFunction", "similarityParameter"], ["const int", "const int", "const float"]), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(distance_function: i32, similarity_function: i32, similarity_parameter: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::PCTSignaturesSQFD::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:828
// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_create(ocvrs_return: *mut Result<*mut c_void>);
// computeQuadraticFormDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:838
// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistance", vec![(pred!(const, ["_signature0", "_signature1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, _signature0: *const c_void, _signature1: *const c_void, ocvrs_return: *mut Result<f32>);
// computeQuadraticFormDistances(const Mat &, const std::vector<Mat> &, std::vector<float> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:849
// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistances", vec![(pred!(const, ["sourceSignature", "imageSignatures", "distances"], ["const cv::Mat*", "const std::vector<cv::Mat>*", "std::vector<float>*"]), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vectorLMatGR_vectorLfloatGR(instance: *const c_void, source_signature: *const c_void, image_signatures: *const c_void, distances: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::xfeatures2d::PCTSignaturesSQFD::to_Algorithm() generated
// ("cv::xfeatures2d::PCTSignaturesSQFD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::PCTSignaturesSQFD::delete() generated
// ("cv::xfeatures2d::PCTSignaturesSQFD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_PCTSignaturesSQFD_delete(instance: *mut c_void);
// create(double, int, int, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:94
// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, ["hessianThreshold", "nOctaves", "nOctaveLayers", "extended", "upright"], ["double", "int", "int", "bool", "bool"]), _)]),
pub fn cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::SURF::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:94
// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_SURF_create(ocvrs_return: *mut Result<*mut c_void>);
// setHessianThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:98
// ("cv::xfeatures2d::SURF::setHessianThreshold", vec![(pred!(mut, ["hessianThreshold"], ["double"]), _)]),
pub fn cv_xfeatures2d_SURF_setHessianThreshold_double(instance: *mut c_void, hessian_threshold: f64, ocvrs_return: *mut Result<()>);
// getHessianThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:99
// ("cv::xfeatures2d::SURF::getHessianThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_SURF_getHessianThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:101
// ("cv::xfeatures2d::SURF::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
pub fn cv_xfeatures2d_SURF_setNOctaves_int(instance: *mut c_void, n_octaves: i32, ocvrs_return: *mut Result<()>);
// getNOctaves()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:102
// ("cv::xfeatures2d::SURF::getNOctaves", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_SURF_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:104
// ("cv::xfeatures2d::SURF::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
pub fn cv_xfeatures2d_SURF_setNOctaveLayers_int(instance: *mut c_void, n_octave_layers: i32, ocvrs_return: *mut Result<()>);
// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:105
// ("cv::xfeatures2d::SURF::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_SURF_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:107
// ("cv::xfeatures2d::SURF::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
pub fn cv_xfeatures2d_SURF_setExtended_bool(instance: *mut c_void, extended: bool, ocvrs_return: *mut Result<()>);
// getExtended()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:108
// ("cv::xfeatures2d::SURF::getExtended", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_SURF_getExtended_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:110
// ("cv::xfeatures2d::SURF::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
pub fn cv_xfeatures2d_SURF_setUpright_bool(instance: *mut c_void, upright: bool, ocvrs_return: *mut Result<()>);
// getUpright()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:111
// ("cv::xfeatures2d::SURF::getUpright", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_SURF_getUpright_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::xfeatures2d::SURF::to_Algorithm() generated
// ("cv::xfeatures2d::SURF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_SURF_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::SURF::to_Feature2D() generated
// ("cv::xfeatures2d::SURF::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_SURF_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::SURF::delete() generated
// ("cv::xfeatures2d::SURF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_SURF_delete(instance: *mut c_void);
// create(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:113
// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, ["maxSize", "responseThreshold", "lineThresholdProjected", "lineThresholdBinarized", "suppressNonmaxSize"], ["int", "int", "int", "int", "int"]), _)]),
pub fn cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::StarDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:113
// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_StarDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::StarDetector::defaultNew() generated
// ("cv::xfeatures2d::StarDetector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_StarDetector_defaultNew_const() -> *mut c_void;
// cv::xfeatures2d::StarDetector::to_Algorithm() generated
// ("cv::xfeatures2d::StarDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_StarDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::StarDetector::to_Feature2D() generated
// ("cv::xfeatures2d::StarDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_StarDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::StarDetector::delete() generated
// ("cv::xfeatures2d::StarDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_StarDetector_delete(instance: *mut c_void);
// create(int, float, bool, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:316
// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, ["desc", "isigma", "img_normalize", "use_scale_orientation", "scale_factor", "dsc_normalize"], ["int", "float", "bool", "bool", "float", "bool"]), _)]),
pub fn cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::xfeatures2d::VGG::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:316
// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_VGG_create(ocvrs_return: *mut Result<*mut c_void>);
// setSigma(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:320
// ("cv::xfeatures2d::VGG::setSigma", vec![(pred!(mut, ["isigma"], ["const float"]), _)]),
pub fn cv_xfeatures2d_VGG_setSigma_const_float(instance: *mut c_void, isigma: f32, ocvrs_return: *mut Result<()>);
// getSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:321
// ("cv::xfeatures2d::VGG::getSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_VGG_getSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setUseNormalizeImage(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:323
// ("cv::xfeatures2d::VGG::setUseNormalizeImage", vec![(pred!(mut, ["img_normalize"], ["const bool"]), _)]),
pub fn cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(instance: *mut c_void, img_normalize: bool, ocvrs_return: *mut Result<()>);
// getUseNormalizeImage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:324
// ("cv::xfeatures2d::VGG::getUseNormalizeImage", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_VGG_getUseNormalizeImage_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:326
// ("cv::xfeatures2d::VGG::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
pub fn cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(instance: *mut c_void, use_scale_orientation: bool, ocvrs_return: *mut Result<()>);
// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:327
// ("cv::xfeatures2d::VGG::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_VGG_getUseScaleOrientation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:329
// ("cv::xfeatures2d::VGG::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
pub fn cv_xfeatures2d_VGG_setScaleFactor_const_float(instance: *mut c_void, scale_factor: f32, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:330
// ("cv::xfeatures2d::VGG::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_VGG_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setUseNormalizeDescriptor(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:332
// ("cv::xfeatures2d::VGG::setUseNormalizeDescriptor", vec![(pred!(mut, ["dsc_normalize"], ["const bool"]), _)]),
pub fn cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(instance: *mut c_void, dsc_normalize: bool, ocvrs_return: *mut Result<()>);
// getUseNormalizeDescriptor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:333
// ("cv::xfeatures2d::VGG::getUseNormalizeDescriptor", vec![(pred!(const, [], []), _)]),
pub fn cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::xfeatures2d::VGG::to_Algorithm() generated
// ("cv::xfeatures2d::VGG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_VGG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::VGG::to_Feature2D() generated
// ("cv::xfeatures2d::VGG::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_VGG_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::xfeatures2d::VGG::delete() generated
// ("cv::xfeatures2d::VGG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xfeatures2d_VGG_delete(instance: *mut c_void);
