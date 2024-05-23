// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_linemod_colormap_const_MatR_MatR(quantized: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:448
// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINE(ocvrs_return: *mut Result<*mut c_void>);
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:456
// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINEMOD(ocvrs_return: *mut Result<*mut c_void>);
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:333
// ("cv::rgbd::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, in_k: *const c_void, in_points: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:67
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const double*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_doubleR(depth: *const f64, ocvrs_return: *mut Result<bool>);
// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:61
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_floatR(depth: *const f32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:86
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_intR(depth: *const i32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:73
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_shortR(depth: *const i16, ocvrs_return: *mut Result<bool>);
// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:92
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_unsigned_intR(depth: *const u32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:79
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned short*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_unsigned_shortR(depth: *const u16, ocvrs_return: *mut Result<bool>);
// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, ocvrs_return: *mut Result<()>);
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, depth_dilation: bool, ocvrs_return: *mut Result<()>);
// rescaleDepth(InputArray, int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:358
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_: *const c_void, depth: i32, out: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(image: *const c_void, depth: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, dist_coeff: *const c_void, warped_image: *const c_void, ocvrs_return: *mut Result<()>);
// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage", "warpedDepth", "warpedMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, depth: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, dist_coeff: *const c_void, warped_image: *const c_void, warped_depth: *const c_void, warped_mask: *const c_void, ocvrs_return: *mut Result<()>);
// ColorGradient()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:209
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient(ocvrs_return: *mut Result<*mut c_void>);
// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:219
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:221
// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:223
// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:225
// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_ColorGradient_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:226
// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_ColorGradient_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::to_LineMod_Modality() generated
// ("cv::linemod::ColorGradient::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::ColorGradient::delete() generated
// ("cv::linemod::ColorGradient::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_delete(instance: *mut c_void);
// DepthNormal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:246
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal(ocvrs_return: *mut Result<*mut c_void>);
// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:258
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:261
// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:264
// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:266
// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_DepthNormal_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:267
// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_DepthNormal_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::to_LineMod_Modality() generated
// ("cv::linemod::DepthNormal::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::DepthNormal::delete() generated
// ("cv::linemod::DepthNormal::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_delete(instance: *mut c_void);
// Detector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:332
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_Detector(ocvrs_return: *mut Result<*mut c_void>);
// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:341
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
pub fn cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(modalities: *const c_void, t_pyramid: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, class_ids: *const c_void, quantized_images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<i32>);
// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, ocvrs_return: *mut Result<i32>);
// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:379
// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(instance: *mut c_void, templates: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// getModalities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:387
// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_getModalities_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:392
// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
pub fn cv_linemod_Detector_getT_const_int(instance: *const c_void, pyramid_level: i32, ocvrs_return: *mut Result<i32>);
// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:397
// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_pyramidLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:405
// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
pub fn cv_linemod_Detector_getTemplates_const_const_StringR_int(instance: *const c_void, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// numTemplates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:407
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numTemplates_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:408
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_numTemplates_const_const_StringR(instance: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// numClasses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:409
// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numClasses_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// classIds()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:411
// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_classIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:413
// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:414
// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(instance: *mut c_void, fn_: *const c_void, class_id_override: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:417
// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(instance: *const c_void, class_id: *const c_char, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(instance: *mut c_void, class_ids: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR(instance: *mut c_void, class_ids: *const c_void, ocvrs_return: *mut Result<()>);
// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_writeClasses_const_const_StringR(instance: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_writeClasses_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::delete() generated
// ("cv::linemod::Detector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_delete(instance: *mut c_void);
// Feature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:69
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Feature_Feature(ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:70
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
pub fn cv_linemod_Feature_Feature_int_int_int(x: i32, y: i32, label: i32, ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:72
// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Feature_read_const_FileNodeR(instance: *const crate::rgbd::LineMod_Feature, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:73
// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Feature_write_const_FileStorageR(instance: *const crate::rgbd::LineMod_Feature, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// Match()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:289
// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_Match(ocvrs_return: *mut Result<*mut c_void>);
// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:293
// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
pub fn cv_linemod_Match_Match_int_int_float_const_StringR_int(x: i32, y: i32, similarity: f32, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:296
// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorL_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:305
// ("cv::linemod::Match::operator==", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorEQ_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::linemod::Match::implicitClone() generated
// ("cv::linemod::Match::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propX_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propX_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propY_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propY_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propSimilarity_const(instance: *const c_void) -> f32;
// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_Match_propSimilarity_const_float(instance: *mut c_void, val: f32);
// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propClass_id_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_linemod_Match_propClass_id_const_String(instance: *mut c_void, val: *const c_char);
// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propTemplate_id_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propTemplate_id_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::delete() generated
// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_delete(instance: *mut c_void);
// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR_const_MatR(instance: *const c_void, src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR(instance: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:175
// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Modality_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:177
// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Modality_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:178
// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Modality_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:187
// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Modality_create_const_StringR(modality_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:192
// ("cv::linemod::Modality::create", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Modality_create_const_FileNodeR(fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Modality::to_LineMod_ColorGradient() generated
// ("cv::linemod::Modality::to_LineMod_ColorGradient", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_to_LineMod_ColorGradient(instance: *mut c_void) -> *mut c_void;
// cv::linemod::Modality::to_LineMod_DepthNormal() generated
// ("cv::linemod::Modality::to_LineMod_DepthNormal", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_to_LineMod_DepthNormal(instance: *mut c_void) -> *mut c_void;
// cv::linemod::Modality::delete() generated
// ("cv::linemod::Modality::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_delete(instance: *mut c_void);
// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:104
// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_quantize_const_MatR(instance: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:111
// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(instance: *const c_void, templ: *mut c_void, ocvrs_return: *mut Result<bool>);
// pyrDown()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:118
// ("cv::linemod::QuantizedPyramid::pyrDown", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_pyrDown(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::QuantizedPyramid::delete() generated
// ("cv::linemod::QuantizedPyramid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_delete(instance: *mut c_void);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:85
// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Template_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:86
// ("cv::linemod::Template::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Template_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Template::implicitClone() generated
// ("cv::linemod::Template::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::defaultNew() generated
// ("cv::linemod::Template::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_defaultNew_const() -> *mut c_void;
// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propWidth_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propWidth_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propHeight_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propHeight_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propPyramid_level_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propPyramid_level_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propFeatures_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
pub fn cv_linemod_Template_propFeatures_const_vectorLFeatureG(instance: *mut c_void, val: *const c_void);
// cv::linemod::Template::delete() generated
// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Template_delete(instance: *mut c_void);
// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:232
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner(ocvrs_return: *mut Result<*mut c_void>);
// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
pub fn cv_rgbd_DepthCleaner_create_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_create_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:257
// ("cv::rgbd::DepthCleaner::operator()", vec![(pred!(const, ["points", "depth"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_DepthCleaner_operator___const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<()>);
// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:263
// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_initialize_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:265
// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:269
// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:273
// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:277
// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:281
// ("cv::rgbd::DepthCleaner::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:285
// ("cv::rgbd::DepthCleaner::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::DepthCleaner::to_Algorithm() generated
// ("cv::rgbd::DepthCleaner::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::DepthCleaner::delete() generated
// ("cv::rgbd::DepthCleaner::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_delete(instance: *mut c_void);
// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:795
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:814
// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:816
// ("cv::rgbd::ICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:820
// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:824
// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:828
// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:832
// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:836
// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:840
// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:844
// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:848
// ("cv::rgbd::ICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:852
// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:856
// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:860
// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:864
// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:868
// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_ICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:872
// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:876
// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:880
// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:884
// ("cv::rgbd::ICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:888
// ("cv::rgbd::ICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::to_Algorithm() generated
// ("cv::rgbd::ICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::ICPOdometry::to_Odometry() generated
// ("cv::rgbd::ICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::ICPOdometry::delete() generated
// ("cv::rgbd::ICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_delete(instance: *mut c_void);
// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:568
// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:573
// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:578
// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:583
// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:588
// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:593
// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return: *mut Result<f32>);
// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt", "initRt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(instance: *const c_void, src_image: *const c_void, src_depth: *const c_void, src_mask: *const c_void, dst_image: *const c_void, dst_depth: *const c_void, dst_mask: *const c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(instance: *const c_void, src_image: *const c_void, src_depth: *const c_void, src_mask: *const c_void, dst_image: *const c_void, dst_depth: *const c_void, dst_mask: *const c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt", "initRt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR_const_MatR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:632
// ("cv::rgbd::Odometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_Odometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:634
// ("cv::rgbd::Odometry::create", vec![(pred!(mut, ["odometryType"], ["const cv::String*"]), _)]),
pub fn cv_rgbd_Odometry_create_const_StringR(odometry_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:637
// ("cv::rgbd::Odometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_Odometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:639
// ("cv::rgbd::Odometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:641
// ("cv::rgbd::Odometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_Odometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:643
// ("cv::rgbd::Odometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_Odometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::Odometry::to_ICPOdometry() generated
// ("cv::rgbd::Odometry::to_ICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_ICPOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_RgbdICPOdometry() generated
// ("cv::rgbd::Odometry::to_RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_RgbdICPOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_RgbdOdometry() generated
// ("cv::rgbd::Odometry::to_RgbdOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_RgbdOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_Algorithm() generated
// ("cv::rgbd::Odometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::delete() generated
// ("cv::rgbd::Odometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_delete(instance: *mut c_void);
// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:530
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame(ocvrs_return: *mut Result<*mut c_void>);
// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR(image: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_create(ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:536
// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// releasePyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:539
// ("cv::rgbd::OdometryFrame::releasePyramids", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_releasePyramids(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidImage_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidDepth_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidCloud_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormals_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
// ("cv::rgbd::OdometryFrame::pyramidNormalsMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormalsMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
// ("cv::rgbd::OdometryFrame::setPyramidNormalsMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormalsMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::to_RgbdFrame() generated
// ("cv::rgbd::OdometryFrame::to_RgbdFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_to_RgbdFrame(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::delete() generated
// ("cv::rgbd::OdometryFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_delete(instance: *mut c_void);
// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:496
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame(ocvrs_return: *mut Result<*mut c_void>);
// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR(image: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_create(ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:503
// ("cv::rgbd::RgbdFrame::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propID_const(instance: *const c_void) -> i32;
// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_rgbd_RgbdFrame_propID_const_int(instance: *mut c_void, val: i32);
// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propImage_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propImage_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propDepth_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propDepth_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propMask_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
// ("cv::rgbd::RgbdFrame::normals", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propNormals_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
// ("cv::rgbd::RgbdFrame::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propNormals_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::to_OdometryFrame() generated
// ("cv::rgbd::RgbdFrame::to_OdometryFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_to_OdometryFrame(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::delete() generated
// ("cv::rgbd::RgbdFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_delete(instance: *mut c_void);
// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:923
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:948
// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:950
// ("cv::rgbd::RgbdICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:954
// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:958
// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:962
// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:966
// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:970
// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:974
// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:978
// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:982
// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:986
// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:990
// ("cv::rgbd::RgbdICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:994
// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:998
// ("cv::rgbd::RgbdICPOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1002
// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1006
// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1010
// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1014
// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1018
// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1022
// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1026
// ("cv::rgbd::RgbdICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1030
// ("cv::rgbd::RgbdICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::to_Algorithm() generated
// ("cv::rgbd::RgbdICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdICPOdometry::to_Odometry() generated
// ("cv::rgbd::RgbdICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdICPOdometry::delete() generated
// ("cv::rgbd::RgbdICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_delete(instance: *mut c_void);
// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:117
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals(ocvrs_return: *mut Result<*mut c_void>);
// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR(rows: i32, cols: i32, depth: i32, k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR(rows: i32, cols: i32, depth: i32, k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:150
// ("cv::rgbd::RgbdNormals::operator()", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_operator___const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:156
// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_initialize_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// getRows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:158
// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getRows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:162
// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setRows_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCols()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:166
// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getCols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:170
// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setCols_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:174
// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:178
// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:182
// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:186
// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:190
// ("cv::rgbd::RgbdNormals::getK", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getK_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:194
// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdNormals_setK_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:198
// ("cv::rgbd::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:202
// ("cv::rgbd::RgbdNormals::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdNormals::to_Algorithm() generated
// ("cv::rgbd::RgbdNormals::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdNormals::delete() generated
// ("cv::rgbd::RgbdNormals::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_delete(instance: *mut c_void);
// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:660
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry(ocvrs_return: *mut Result<*mut c_void>);
// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:683
// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:685
// ("cv::rgbd::RgbdOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:689
// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:693
// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:697
// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:701
// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:705
// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:709
// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:713
// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:717
// ("cv::rgbd::RgbdOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:721
// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:725
// ("cv::rgbd::RgbdOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:729
// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:733
// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:737
// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:741
// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:745
// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:749
// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:753
// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:757
// ("cv::rgbd::RgbdOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:761
// ("cv::rgbd::RgbdOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdOdometry::to_Algorithm() generated
// ("cv::rgbd::RgbdOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdOdometry::to_Odometry() generated
// ("cv::rgbd::RgbdOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdOdometry::delete() generated
// ("cv::rgbd::RgbdOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_delete(instance: *mut c_void);
// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int(method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane(ocvrs_return: *mut Result<*mut c_void>);
// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double(method: i32, block_size: i32, min_size: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_create_int_int_int_double(method: i32, block_size: i32, min_size: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:410
// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, normals: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, ocvrs_return: *mut Result<()>);
// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:420
// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, ocvrs_return: *mut Result<()>);
// getBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:422
// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:426
// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setBlockSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:430
// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:434
// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setMinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:438
// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:442
// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:446
// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:450
// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:454
// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorA_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:458
// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorA_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:462
// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorB_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:466
// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorB_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:470
// ("cv::rgbd::RgbdPlane::getSensorErrorC", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorC_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:474
// ("cv::rgbd::RgbdPlane::setSensorErrorC", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorC_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdPlane::to_Algorithm() generated
// ("cv::rgbd::RgbdPlane::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdPlane::delete() generated
// ("cv::rgbd::RgbdPlane::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_delete(instance: *mut c_void);
