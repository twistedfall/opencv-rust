// cv::AGAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
pub fn cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int(image: *const c_void, keypoints: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// AGAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
pub fn cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, ocvrs_return: *mut Result<()>);
// AGAST(InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:679
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
pub fn cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result<()>);
// cv::FAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
pub fn cv_FAST_const__InputArrayR_vectorLKeyPointGR_int(image: *const c_void, keypoints: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// FAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
pub fn cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, ocvrs_return: *mut Result<()>);
// FAST(InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:623
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
pub fn cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result<()>);
// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1431
// ("cv::computeRecallPrecisionCurve", vec![(pred!(mut, ["matches1to2", "correctMatches1to2Mask", "recallPrecisionCurve"], ["const std::vector<std::vector<cv::DMatch>>*", "const std::vector<std::vector<unsigned char>>*", "std::vector<cv::Point2f>*"]), _)]),
pub fn cv_computeRecallPrecisionCurve_const_vectorLvectorLDMatchGGR_const_vectorLvectorLunsigned_charGGR_vectorLPoint2fGR(matches1to2: *const c_void, correct_matches1to2_mask: *const c_void, recall_precision_curve: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::drawKeypoints(InputArray, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR(image: *const c_void, keypoints: *const c_void, out_image: *const c_void, ocvrs_return: *mut Result<()>);
// drawKeypoints(InputArray, const std::vector<KeyPoint> &, InputOutputArray, const Scalar &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputOutputArray, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage", "color", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "cv::DrawMatchesFlags"]), _)]),
pub fn cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image: *const c_void, keypoints: *const c_void, out_image: *const c_void, color: *const core::Scalar, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result<()>);
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, ocvrs_return: *mut Result<()>);
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result<()>);
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, matches_thickness: i32, ocvrs_return: *mut Result<()>);
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const int, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, matches_thickness: i32, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result<()>);
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, ocvrs_return: *mut Result<()>);
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<std::vector<char>> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<std::vector<char>>*", "cv::DrawMatchesFlags"]), _)]),
pub fn cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result<()>);
// cv::evaluateFeatureDetector(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*"]), _)]),
pub fn cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR(img1: *const c_void, img2: *const c_void, h1to2: *const c_void, keypoints1: *mut c_void, keypoints2: *mut c_void, repeatability: *mut f32, corresp_count: *mut i32, ocvrs_return: *mut Result<()>);
// evaluateFeatureDetector(const Mat &, const Mat &, const Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<FeatureDetector> &)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount", "fdetector"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*", "const cv::Ptr<cv::Feature2D>*"]), _)]),
pub fn cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(img1: *const c_void, img2: *const c_void, h1to2: *const c_void, keypoints1: *mut c_void, keypoints2: *mut c_void, repeatability: *mut f32, corresp_count: *mut i32, fdetector: *const c_void, ocvrs_return: *mut Result<()>);
// getNearestPoint(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1436
// ("cv::getNearestPoint", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
pub fn cv_getNearestPoint_const_vectorLPoint2fGR_float(recall_precision_curve: *const c_void, l_precision: f32, ocvrs_return: *mut Result<i32>);
// getRecall(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1435
// ("cv::getRecall", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
pub fn cv_getRecall_const_vectorLPoint2fGR_float(recall_precision_curve: *const c_void, l_precision: f32, ocvrs_return: *mut Result<f32>);
// create(AKAZE::DescriptorType, int, int, float, int, int, KAZE::DiffusivityType, int)(Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
// ("cv::AKAZE::create", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity", "max_points"], ["cv::AKAZE::DescriptorType", "int", "int", "float", "int", "int", "cv::KAZE::DiffusivityType", "int"]), _)]),
pub fn cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType_int(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType, max_points: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::AKAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
// ("cv::AKAZE::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_AKAZE_create(ocvrs_return: *mut Result<*mut c_void>);
// setDescriptorType(AKAZE::DescriptorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:890
// ("cv::AKAZE::setDescriptorType", vec![(pred!(mut, ["dtype"], ["cv::AKAZE::DescriptorType"]), _)]),
pub fn cv_AKAZE_setDescriptorType_DescriptorType(instance: *mut c_void, dtype: crate::features2d::AKAZE_DescriptorType, ocvrs_return: *mut Result<()>);
// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:891
// ("cv::AKAZE::getDescriptorType", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getDescriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::AKAZE_DescriptorType>);
// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:893
// ("cv::AKAZE::setDescriptorSize", vec![(pred!(mut, ["dsize"], ["int"]), _)]),
pub fn cv_AKAZE_setDescriptorSize_int(instance: *mut c_void, dsize: i32, ocvrs_return: *mut Result<()>);
// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:894
// ("cv::AKAZE::getDescriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDescriptorChannels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:896
// ("cv::AKAZE::setDescriptorChannels", vec![(pred!(mut, ["dch"], ["int"]), _)]),
pub fn cv_AKAZE_setDescriptorChannels_int(instance: *mut c_void, dch: i32, ocvrs_return: *mut Result<()>);
// getDescriptorChannels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:897
// ("cv::AKAZE::getDescriptorChannels", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getDescriptorChannels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:899
// ("cv::AKAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_AKAZE_setThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:900
// ("cv::AKAZE::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:902
// ("cv::AKAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
pub fn cv_AKAZE_setNOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result<()>);
// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:903
// ("cv::AKAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:905
// ("cv::AKAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
pub fn cv_AKAZE_setNOctaveLayers_int(instance: *mut c_void, octave_layers: i32, ocvrs_return: *mut Result<()>);
// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:906
// ("cv::AKAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:908
// ("cv::AKAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
pub fn cv_AKAZE_setDiffusivity_DiffusivityType(instance: *mut c_void, diff: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result<()>);
// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:909
// ("cv::AKAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getDiffusivity_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::KAZE_DiffusivityType>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:910
// ("cv::AKAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMaxPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:912
// ("cv::AKAZE::setMaxPoints", vec![(pred!(mut, ["max_points"], ["int"]), _)]),
pub fn cv_AKAZE_setMaxPoints_int(instance: *mut c_void, max_points: i32, ocvrs_return: *mut Result<()>);
// getMaxPoints()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:913
// ("cv::AKAZE::getMaxPoints", vec![(pred!(const, [], []), _)]),
pub fn cv_AKAZE_getMaxPoints_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::AKAZE::to_Algorithm() generated
// ("cv::AKAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AKAZE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AKAZE::to_Feature2D() generated
// ("cv::AKAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_AKAZE_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::AKAZE::delete() generated
// ("cv::AKAZE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AKAZE_delete(instance: *mut c_void);
// create(const Ptr<Feature2D> &, int, int, float, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend", "maxTilt", "minTilt", "tiltStep", "rotateStepBase"], ["const cv::Ptr<cv::Feature2D>*", "int", "int", "float", "float"]), _)]),
pub fn cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(backend: *const c_void, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::AffineFeature::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend"], ["const cv::Ptr<cv::Feature2D>*"]), _)]),
pub fn cv_AffineFeature_create_const_PtrLFeature2DGR(backend: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setViewParams(const std::vector<float> &, const std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:254
// ("cv::AffineFeature::setViewParams", vec![(pred!(mut, ["tilts", "rolls"], ["const std::vector<float>*", "const std::vector<float>*"]), _)]),
pub fn cv_AffineFeature_setViewParams_const_vectorLfloatGR_const_vectorLfloatGR(instance: *mut c_void, tilts: *const c_void, rolls: *const c_void, ocvrs_return: *mut Result<()>);
// getViewParams(std::vector<float> &, std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:255
// ("cv::AffineFeature::getViewParams", vec![(pred!(const, ["tilts", "rolls"], ["std::vector<float>*", "std::vector<float>*"]), _)]),
pub fn cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(instance: *const c_void, tilts: *mut c_void, rolls: *mut c_void, ocvrs_return: *mut Result<()>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:256
// ("cv::AffineFeature::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_AffineFeature_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::AffineFeature::to_Algorithm() generated
// ("cv::AffineFeature::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineFeature_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AffineFeature::to_Feature2D() generated
// ("cv::AffineFeature::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineFeature_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::AffineFeature::delete() generated
// ("cv::AffineFeature::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineFeature_delete(instance: *mut c_void);
// create(int, bool, AgastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
pub fn cv_AgastFeatureDetector_create_int_bool_DetectorType(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result<*mut c_void>);
// cv::AgastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_AgastFeatureDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:646
// ("cv::AgastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_AgastFeatureDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:647
// ("cv::AgastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_AgastFeatureDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:649
// ("cv::AgastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
pub fn cv_AgastFeatureDetector_setNonmaxSuppression_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result<()>);
// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:650
// ("cv::AgastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
pub fn cv_AgastFeatureDetector_getNonmaxSuppression_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setType(AgastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:652
// ("cv::AgastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::AgastFeatureDetector::DetectorType"]), _)]),
pub fn cv_AgastFeatureDetector_setType_DetectorType(instance: *mut c_void, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result<()>);
// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:653
// ("cv::AgastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
pub fn cv_AgastFeatureDetector_getType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::AgastFeatureDetector_DetectorType>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:654
// ("cv::AgastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_AgastFeatureDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::AgastFeatureDetector::to_Algorithm() generated
// ("cv::AgastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AgastFeatureDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AgastFeatureDetector::to_Feature2D() generated
// ("cv::AgastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_AgastFeatureDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::AgastFeatureDetector::delete() generated
// ("cv::AgastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AgastFeatureDetector_delete(instance: *mut c_void);
// BFMatcher(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
pub fn cv_BFMatcher_BFMatcher_int_bool(norm_type: i32, cross_check: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::BFMatcher::BFMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_BFMatcher_BFMatcher(ocvrs_return: *mut Result<*mut c_void>);
// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1257
// ("cv::BFMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
pub fn cv_BFMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
// ("cv::BFMatcher::create", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
pub fn cv_BFMatcher_create_int_bool(norm_type: i32, cross_check: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::BFMatcher::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
// ("cv::BFMatcher::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_BFMatcher_create(ocvrs_return: *mut Result<*mut c_void>);
// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
// ("cv::BFMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
pub fn cv_BFMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::BFMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
// ("cv::BFMatcher::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_BFMatcher_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BFMatcher::to_Algorithm() generated
// ("cv::BFMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BFMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BFMatcher::to_DescriptorMatcher() generated
// ("cv::BFMatcher::to_DescriptorMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_BFMatcher_to_DescriptorMatcher(instance: *mut c_void) -> *mut c_void;
// cv::BFMatcher::delete() generated
// ("cv::BFMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BFMatcher_delete(instance: *mut c_void);
// BOWImgDescriptorExtractor(const Ptr<Feature2D> &, const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1541
// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dextractor", "dmatcher"], ["const cv::Ptr<cv::Feature2D>*", "const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(dextractor: *const c_void, dmatcher: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BOWImgDescriptorExtractor(const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1544
// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dmatcher"], ["const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(dmatcher: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setVocabulary(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1552
// ("cv::BOWImgDescriptorExtractor::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const cv::Mat*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(instance: *mut c_void, vocabulary: *const c_void, ocvrs_return: *mut Result<()>);
// getVocabulary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1556
// ("cv::BOWImgDescriptorExtractor::getVocabulary", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWImgDescriptorExtractor_getVocabulary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, std::vector<KeyPoint> &, OutputArray, std::vector<std::vector<int>> *, Mat *)(InputArray, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor", "pointIdxsOfClusters", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*", "cv::Mat*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, img_descriptor: *const c_void, point_idxs_of_clusters: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::BOWImgDescriptorExtractor::compute(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, img_descriptor: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, OutputArray, std::vector<std::vector<int>> *)(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor", "pointIdxsOfClusters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(instance: *mut c_void, keypoint_descriptors: *const c_void, img_descriptor: *const c_void, point_idxs_of_clusters: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::BOWImgDescriptorExtractor::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, keypoint_descriptors: *const c_void, img_descriptor: *const c_void, ocvrs_return: *mut Result<()>);
// compute2(const Mat &, std::vector<KeyPoint> &, Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1581
// ("cv::BOWImgDescriptorExtractor::compute2", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::Mat*", "std::vector<cv::KeyPoint>*", "cv::Mat*"]), _)]),
pub fn cv_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, img_descriptor: *mut c_void, ocvrs_return: *mut Result<()>);
// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1586
// ("cv::BOWImgDescriptorExtractor::descriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWImgDescriptorExtractor_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1590
// ("cv::BOWImgDescriptorExtractor::descriptorType", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWImgDescriptorExtractor_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::BOWImgDescriptorExtractor::delete() generated
// ("cv::BOWImgDescriptorExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWImgDescriptorExtractor_delete(instance: *mut c_void);
// BOWKMeansTrainer(int, const TermCriteria &, int, int)(Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount", "termcrit", "attempts", "flags"], ["int", "const cv::TermCriteria*", "int", "int"]), _)]),
pub fn cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count: i32, termcrit: *const core::TermCriteria, attempts: i32, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::BOWKMeansTrainer::BOWKMeansTrainer(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount"], ["int"]), _)]),
pub fn cv_BOWKMeansTrainer_BOWKMeansTrainer_int(cluster_count: i32, ocvrs_return: *mut Result<*mut c_void>);
// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1510
// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWKMeansTrainer_cluster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1511
// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
pub fn cv_BOWKMeansTrainer_cluster_const_const_MatR(instance: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BOWKMeansTrainer::to_BOWTrainer() generated
// ("cv::BOWKMeansTrainer::to_BOWTrainer", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWKMeansTrainer_to_BOWTrainer(instance: *mut c_void) -> *mut c_void;
// cv::BOWKMeansTrainer::delete() generated
// ("cv::BOWKMeansTrainer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWKMeansTrainer_delete(instance: *mut c_void);
// add(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1465
// ("cv::BOWTrainer::add", vec![(pred!(mut, ["descriptors"], ["const cv::Mat*"]), _)]),
pub fn cv_BOWTrainer_add_const_MatR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// getDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1469
// ("cv::BOWTrainer::getDescriptors", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWTrainer_getDescriptors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// descriptorsCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1473
// ("cv::BOWTrainer::descriptorsCount", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWTrainer_descriptorsCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1475
// ("cv::BOWTrainer::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWTrainer_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1478
// ("cv::BOWTrainer::cluster", vec![(pred!(const, [], []), _)]),
pub fn cv_BOWTrainer_cluster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1489
// ("cv::BOWTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
pub fn cv_BOWTrainer_cluster_const_const_MatR(instance: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BOWTrainer::to_BOWKMeansTrainer() generated
// ("cv::BOWTrainer::to_BOWKMeansTrainer", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWTrainer_to_BOWKMeansTrainer(instance: *mut c_void) -> *mut c_void;
// cv::BOWTrainer::delete() generated
// ("cv::BOWTrainer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BOWTrainer_delete(instance: *mut c_void);
// create(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "patternScale"], ["int", "int", "float"]), _)]),
pub fn cv_BRISK_create_int_int_float(thresh: i32, octaves: i32, pattern_scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::BRISK::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
// ("cv::BRISK::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_BRISK_create(ocvrs_return: *mut Result<*mut c_void>);
// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList", "dMax", "dMin", "indexChange"], ["const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
pub fn cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(radius_list: *const c_void, number_list: *const c_void, d_max: f32, d_min: f32, index_change: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BRISK::create(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList"], ["const std::vector<float>*", "const std::vector<int>*"]), _)]),
pub fn cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR(radius_list: *const c_void, number_list: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList", "dMax", "dMin", "indexChange"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
pub fn cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(thresh: i32, octaves: i32, radius_list: *const c_void, number_list: *const c_void, d_max: f32, d_min: f32, index_change: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BRISK::create(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*"]), _)]),
pub fn cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR(thresh: i32, octaves: i32, radius_list: *const c_void, number_list: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:395
// ("cv::BRISK::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_BRISK_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:400
// ("cv::BRISK::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_BRISK_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:401
// ("cv::BRISK::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_BRISK_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:406
// ("cv::BRISK::setOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
pub fn cv_BRISK_setOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result<()>);
// getOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:407
// ("cv::BRISK::getOctaves", vec![(pred!(const, [], []), _)]),
pub fn cv_BRISK_getOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPatternScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:412
// ("cv::BRISK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["float"]), _)]),
pub fn cv_BRISK_setPatternScale_float(instance: *mut c_void, pattern_scale: f32, ocvrs_return: *mut Result<()>);
// getPatternScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:413
// ("cv::BRISK::getPatternScale", vec![(pred!(const, [], []), _)]),
pub fn cv_BRISK_getPatternScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::BRISK::to_Algorithm() generated
// ("cv::BRISK::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BRISK_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BRISK::to_Feature2D() generated
// ("cv::BRISK::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_BRISK_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::BRISK::delete() generated
// ("cv::BRISK::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BRISK_delete(instance: *mut c_void);
// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1017
// ("cv::DescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
pub fn cv_DescriptorMatcher_add_const__InputArrayR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1021
// ("cv::DescriptorMatcher::getTrainDescriptors", vec![(pred!(const, [], []), _)]),
pub fn cv_DescriptorMatcher_getTrainDescriptors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1025
// ("cv::DescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1029
// ("cv::DescriptorMatcher::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_DescriptorMatcher_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1033
// ("cv::DescriptorMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
pub fn cv_DescriptorMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1042
// ("cv::DescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
pub fn cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// match(InputArray, std::vector<DMatch> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
pub fn cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1146
// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_DescriptorMatcher_write_const_const_StringR(instance: *const c_void, file_name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1152
// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_DescriptorMatcher_read_const_StringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1159
// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_DescriptorMatcher_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1161
// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_DescriptorMatcher_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
// ("cv::DescriptorMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
pub fn cv_DescriptorMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::DescriptorMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
// ("cv::DescriptorMatcher::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_DescriptorMatcher_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1182
// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["descriptorMatcherType"], ["const cv::String*"]), _)]),
pub fn cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const DescriptorMatcher::MatcherType &)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1184
// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["matcherType"], ["const cv::DescriptorMatcher::MatcherType*"]), _)]),
pub fn cv_DescriptorMatcher_create_const_MatcherTypeR(matcher_type: crate::features2d::DescriptorMatcher_MatcherType, ocvrs_return: *mut Result<*mut c_void>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1188
// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_DescriptorMatcher_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1190
// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
pub fn cv_DescriptorMatcher_write_const_const_PtrLFileStorageGR_const_StringR(instance: *const c_void, fs: *const c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::DescriptorMatcher::to_BFMatcher() generated
// ("cv::DescriptorMatcher::to_BFMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_to_BFMatcher(instance: *mut c_void) -> *mut c_void;
// cv::DescriptorMatcher::to_FlannBasedMatcher() generated
// ("cv::DescriptorMatcher::to_FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_to_FlannBasedMatcher(instance: *mut c_void) -> *mut c_void;
// cv::DescriptorMatcher::to_Algorithm() generated
// ("cv::DescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DescriptorMatcher::delete() generated
// ("cv::DescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DescriptorMatcher_delete(instance: *mut c_void);
// create(int, bool, FastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
// ("cv::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
pub fn cv_FastFeatureDetector_create_int_bool_DetectorType(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result<*mut c_void>);
// cv::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
// ("cv::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_FastFeatureDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:590
// ("cv::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
pub fn cv_FastFeatureDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:591
// ("cv::FastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_FastFeatureDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:593
// ("cv::FastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
pub fn cv_FastFeatureDetector_setNonmaxSuppression_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result<()>);
// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:594
// ("cv::FastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
pub fn cv_FastFeatureDetector_getNonmaxSuppression_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setType(FastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:596
// ("cv::FastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::FastFeatureDetector::DetectorType"]), _)]),
pub fn cv_FastFeatureDetector_setType_DetectorType(instance: *mut c_void, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result<()>);
// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:597
// ("cv::FastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
pub fn cv_FastFeatureDetector_getType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::FastFeatureDetector_DetectorType>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:598
// ("cv::FastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_FastFeatureDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FastFeatureDetector::to_Algorithm() generated
// ("cv::FastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_FastFeatureDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::FastFeatureDetector::to_Feature2D() generated
// ("cv::FastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_FastFeatureDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::FastFeatureDetector::delete() generated
// ("cv::FastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FastFeatureDetector_delete(instance: *mut c_void);
// detect(InputArray, std::vector<KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_InputArray*"]), _)]),
pub fn cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR_const__InputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints", "masks"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_InputArray*"]), _)]),
pub fn cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR_const__InputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*"]), _)]),
pub fn cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:177
// ("cv::Feature2D::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Feature2D_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:191
// ("cv::Feature2D::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Feature2D_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// detectAndCompute(InputArray, InputArray, std::vector<KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result<()>);
// cv::Feature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:201
// ("cv::Feature2D::descriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:202
// ("cv::Feature2D::descriptorType", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// defaultNorm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:203
// ("cv::Feature2D::defaultNorm", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:205
// ("cv::Feature2D::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_Feature2D_write_const_const_StringR(instance: *const c_void, file_name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:207
// ("cv::Feature2D::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
pub fn cv_Feature2D_read_const_StringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:209
// ("cv::Feature2D::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_Feature2D_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:212
// ("cv::Feature2D::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_Feature2D_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:215
// ("cv::Feature2D::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:216
// ("cv::Feature2D::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:219
// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_Feature2D_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:221
// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
pub fn cv_Feature2D_write_const_const_PtrLFileStorageGR_const_StringR(instance: *const c_void, fs: *const c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::Feature2D::defaultNew() generated
// ("cv::Feature2D::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_Feature2D_defaultNew_const() -> *mut c_void;
// cv::Feature2D::to_AKAZE() generated
// ("cv::Feature2D::to_AKAZE", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_AKAZE(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_AffineFeature() generated
// ("cv::Feature2D::to_AffineFeature", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_AffineFeature(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_AgastFeatureDetector() generated
// ("cv::Feature2D::to_AgastFeatureDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_AgastFeatureDetector(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_BRISK() generated
// ("cv::Feature2D::to_BRISK", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_BRISK(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_FastFeatureDetector() generated
// ("cv::Feature2D::to_FastFeatureDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_FastFeatureDetector(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_GFTTDetector() generated
// ("cv::Feature2D::to_GFTTDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_GFTTDetector(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_KAZE() generated
// ("cv::Feature2D::to_KAZE", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_KAZE(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_MSER() generated
// ("cv::Feature2D::to_MSER", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_MSER(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_ORB() generated
// ("cv::Feature2D::to_ORB", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_ORB(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_SIFT() generated
// ("cv::Feature2D::to_SIFT", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_SIFT(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_SimpleBlobDetector() generated
// ("cv::Feature2D::to_SimpleBlobDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_SimpleBlobDetector(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::to_Algorithm() generated
// ("cv::Feature2D::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::Feature2D::delete() generated
// ("cv::Feature2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Feature2D_delete(instance: *mut c_void);
// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, ["indexParams", "searchParams"], ["const cv::Ptr<cv::flann::IndexParams>*", "const cv::Ptr<cv::flann::SearchParams>*"]), _)]),
pub fn cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(index_params: *const c_void, search_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FlannBasedMatcher::FlannBasedMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_FlannBasedMatcher(ocvrs_return: *mut Result<*mut c_void>);
// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1299
// ("cv::FlannBasedMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
pub fn cv_FlannBasedMatcher_add_const__InputArrayR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1300
// ("cv::FlannBasedMatcher::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1303
// ("cv::FlannBasedMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_FlannBasedMatcher_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1305
// ("cv::FlannBasedMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_FlannBasedMatcher_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1307
// ("cv::FlannBasedMatcher::train", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1308
// ("cv::FlannBasedMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
pub fn cv_FlannBasedMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1310
// ("cv::FlannBasedMatcher::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_create(ocvrs_return: *mut Result<*mut c_void>);
// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
pub fn cv_FlannBasedMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::FlannBasedMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_FlannBasedMatcher_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FlannBasedMatcher::to_Algorithm() generated
// ("cv::FlannBasedMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::FlannBasedMatcher::to_DescriptorMatcher() generated
// ("cv::FlannBasedMatcher::to_DescriptorMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_to_DescriptorMatcher(instance: *mut c_void) -> *mut c_void;
// cv::FlannBasedMatcher::delete() generated
// ("cv::FlannBasedMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FlannBasedMatcher_delete(instance: *mut c_void);
// create(int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "bool", "double"]), _)]),
pub fn cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::GFTTDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
// ("cv::GFTTDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFTTDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// create(int, double, double, int, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "int", "bool", "double"]), _)]),
pub fn cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::GFTTDetector::create(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize"], ["int", "double", "double", "int", "int"]), _)]),
pub fn cv_GFTTDetector_create_int_double_double_int_int(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:691
// ("cv::GFTTDetector::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_GFTTDetector_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:692
// ("cv::GFTTDetector::getMaxFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setQualityLevel(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:694
// ("cv::GFTTDetector::setQualityLevel", vec![(pred!(mut, ["qlevel"], ["double"]), _)]),
pub fn cv_GFTTDetector_setQualityLevel_double(instance: *mut c_void, qlevel: f64, ocvrs_return: *mut Result<()>);
// getQualityLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:695
// ("cv::GFTTDetector::getQualityLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getQualityLevel_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:697
// ("cv::GFTTDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
pub fn cv_GFTTDetector_setMinDistance_double(instance: *mut c_void, min_distance: f64, ocvrs_return: *mut Result<()>);
// getMinDistance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:698
// ("cv::GFTTDetector::getMinDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getMinDistance_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:700
// ("cv::GFTTDetector::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
pub fn cv_GFTTDetector_setBlockSize_int(instance: *mut c_void, block_size: i32, ocvrs_return: *mut Result<()>);
// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:701
// ("cv::GFTTDetector::getBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGradientSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:703
// ("cv::GFTTDetector::setGradientSize", vec![(pred!(mut, ["gradientSize_"], ["int"]), _)]),
pub fn cv_GFTTDetector_setGradientSize_int(instance: *mut c_void, gradient_size_: i32, ocvrs_return: *mut Result<()>);
// getGradientSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:704
// ("cv::GFTTDetector::getGradientSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFTTDetector_getGradientSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setHarrisDetector(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:706
// ("cv::GFTTDetector::setHarrisDetector", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_GFTTDetector_setHarrisDetector_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getHarrisDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:707
// ("cv::GFTTDetector::getHarrisDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getHarrisDetector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setK(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:709
// ("cv::GFTTDetector::setK", vec![(pred!(mut, ["k"], ["double"]), _)]),
pub fn cv_GFTTDetector_setK_double(instance: *mut c_void, k: f64, ocvrs_return: *mut Result<()>);
// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:710
// ("cv::GFTTDetector::getK", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getK_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:711
// ("cv::GFTTDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_GFTTDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GFTTDetector::to_Algorithm() generated
// ("cv::GFTTDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFTTDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::GFTTDetector::to_Feature2D() generated
// ("cv::GFTTDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFTTDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::GFTTDetector::delete() generated
// ("cv::GFTTDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFTTDetector_delete(instance: *mut c_void);
// create(bool, bool, float, int, int, KAZE::DiffusivityType)(Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
// ("cv::KAZE::create", vec![(pred!(mut, ["extended", "upright", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["bool", "bool", "float", "int", "int", "cv::KAZE::DiffusivityType"]), _)]),
pub fn cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result<*mut c_void>);
// cv::KAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
// ("cv::KAZE::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_KAZE_create(ocvrs_return: *mut Result<*mut c_void>);
// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:821
// ("cv::KAZE::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
pub fn cv_KAZE_setExtended_bool(instance: *mut c_void, extended: bool, ocvrs_return: *mut Result<()>);
// getExtended()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:822
// ("cv::KAZE::getExtended", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getExtended_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:824
// ("cv::KAZE::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
pub fn cv_KAZE_setUpright_bool(instance: *mut c_void, upright: bool, ocvrs_return: *mut Result<()>);
// getUpright()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:825
// ("cv::KAZE::getUpright", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getUpright_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:827
// ("cv::KAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_KAZE_setThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:828
// ("cv::KAZE::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:830
// ("cv::KAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
pub fn cv_KAZE_setNOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result<()>);
// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:831
// ("cv::KAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:833
// ("cv::KAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
pub fn cv_KAZE_setNOctaveLayers_int(instance: *mut c_void, octave_layers: i32, ocvrs_return: *mut Result<()>);
// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:834
// ("cv::KAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:836
// ("cv::KAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
pub fn cv_KAZE_setDiffusivity_DiffusivityType(instance: *mut c_void, diff: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result<()>);
// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:837
// ("cv::KAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getDiffusivity_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::KAZE_DiffusivityType>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:838
// ("cv::KAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_KAZE_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::KAZE::to_Algorithm() generated
// ("cv::KAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_KAZE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::KAZE::to_Feature2D() generated
// ("cv::KAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_KAZE_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::KAZE::delete() generated
// ("cv::KAZE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_KAZE_delete(instance: *mut c_void);
// KeyPointsFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:95
// ("cv::KeyPointsFilter::KeyPointsFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return: *mut Result<*mut c_void>);
// runByImageBorder(std::vector<KeyPoint> &, Size, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:100
// ("cv::KeyPointsFilter::runByImageBorder", vec![(pred!(mut, ["keypoints", "imageSize", "borderSize"], ["std::vector<cv::KeyPoint>*", "cv::Size", "int"]), _)]),
pub fn cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(keypoints: *mut c_void, image_size: *const core::Size, border_size: i32, ocvrs_return: *mut Result<()>);
// runByKeypointSize(std::vector<KeyPoint> &, float, float)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize", "maxSize"], ["std::vector<cv::KeyPoint>*", "float", "float"]), _)]),
pub fn cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(keypoints: *mut c_void, min_size: f32, max_size: f32, ocvrs_return: *mut Result<()>);
// cv::KeyPointsFilter::runByKeypointSize(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize"], ["std::vector<cv::KeyPoint>*", "float"]), _)]),
pub fn cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(keypoints: *mut c_void, min_size: f32, ocvrs_return: *mut Result<()>);
// runByPixelsMask(std::vector<KeyPoint> &, const Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:109
// ("cv::KeyPointsFilter::runByPixelsMask", vec![(pred!(mut, ["keypoints", "mask"], ["std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
pub fn cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// runByPixelsMask2VectorPoint(std::vector<KeyPoint> &, std::vector<std::vector<Point>> &, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:113
// ("cv::KeyPointsFilter::runByPixelsMask2VectorPoint", vec![(pred!(mut, ["keypoints", "removeFrom", "mask"], ["std::vector<cv::KeyPoint>*", "std::vector<std::vector<cv::Point>>*", "const cv::Mat*"]), _)]),
pub fn cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(keypoints: *mut c_void, remove_from: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// removeDuplicated(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:117
// ("cv::KeyPointsFilter::removeDuplicated", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// removeDuplicatedSorted(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:121
// ("cv::KeyPointsFilter::removeDuplicatedSorted", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// retainBest(std::vector<KeyPoint> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:126
// ("cv::KeyPointsFilter::retainBest", vec![(pred!(mut, ["keypoints", "npoints"], ["std::vector<cv::KeyPoint>*", "int"]), _)]),
pub fn cv_KeyPointsFilter_retainBest_vectorLKeyPointGR_int(keypoints: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// cv::KeyPointsFilter::delete() generated
// ("cv::KeyPointsFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_KeyPointsFilter_delete(instance: *mut c_void);
// create(int, int, int, double, double, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
// ("cv::MSER::create", vec![(pred!(mut, ["delta", "min_area", "max_area", "max_variation", "min_diversity", "max_evolution", "area_threshold", "min_margin", "edge_blur_size"], ["int", "int", "int", "double", "double", "int", "double", "double", "int"]), _)]),
pub fn cv_MSER_create_int_int_int_double_double_int_double_double_int(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MSER::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
// ("cv::MSER::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_MSER_create(ocvrs_return: *mut Result<*mut c_void>);
// detectRegions(InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:533
// ("cv::MSER::detectRegions", vec![(pred!(mut, ["image", "msers", "bboxes"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(instance: *mut c_void, image: *const c_void, msers: *mut c_void, bboxes: *mut c_void, ocvrs_return: *mut Result<()>);
// setDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:537
// ("cv::MSER::setDelta", vec![(pred!(mut, ["delta"], ["int"]), _)]),
pub fn cv_MSER_setDelta_int(instance: *mut c_void, delta: i32, ocvrs_return: *mut Result<()>);
// getDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:538
// ("cv::MSER::getDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getDelta_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:540
// ("cv::MSER::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
pub fn cv_MSER_setMinArea_int(instance: *mut c_void, min_area: i32, ocvrs_return: *mut Result<()>);
// getMinArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:541
// ("cv::MSER::getMinArea", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMinArea_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:543
// ("cv::MSER::setMaxArea", vec![(pred!(mut, ["maxArea"], ["int"]), _)]),
pub fn cv_MSER_setMaxArea_int(instance: *mut c_void, max_area: i32, ocvrs_return: *mut Result<()>);
// getMaxArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:544
// ("cv::MSER::getMaxArea", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMaxArea_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxVariation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:546
// ("cv::MSER::setMaxVariation", vec![(pred!(mut, ["maxVariation"], ["double"]), _)]),
pub fn cv_MSER_setMaxVariation_double(instance: *mut c_void, max_variation: f64, ocvrs_return: *mut Result<()>);
// getMaxVariation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:547
// ("cv::MSER::getMaxVariation", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMaxVariation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDiversity(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:549
// ("cv::MSER::setMinDiversity", vec![(pred!(mut, ["minDiversity"], ["double"]), _)]),
pub fn cv_MSER_setMinDiversity_double(instance: *mut c_void, min_diversity: f64, ocvrs_return: *mut Result<()>);
// getMinDiversity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:550
// ("cv::MSER::getMinDiversity", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMinDiversity_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxEvolution(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:552
// ("cv::MSER::setMaxEvolution", vec![(pred!(mut, ["maxEvolution"], ["int"]), _)]),
pub fn cv_MSER_setMaxEvolution_int(instance: *mut c_void, max_evolution: i32, ocvrs_return: *mut Result<()>);
// getMaxEvolution()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:553
// ("cv::MSER::getMaxEvolution", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMaxEvolution_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setAreaThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:555
// ("cv::MSER::setAreaThreshold", vec![(pred!(mut, ["areaThreshold"], ["double"]), _)]),
pub fn cv_MSER_setAreaThreshold_double(instance: *mut c_void, area_threshold: f64, ocvrs_return: *mut Result<()>);
// getAreaThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:556
// ("cv::MSER::getAreaThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getAreaThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinMargin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:558
// ("cv::MSER::setMinMargin", vec![(pred!(mut, ["min_margin"], ["double"]), _)]),
pub fn cv_MSER_setMinMargin_double(instance: *mut c_void, min_margin: f64, ocvrs_return: *mut Result<()>);
// getMinMargin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:559
// ("cv::MSER::getMinMargin", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getMinMargin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEdgeBlurSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:561
// ("cv::MSER::setEdgeBlurSize", vec![(pred!(mut, ["edge_blur_size"], ["int"]), _)]),
pub fn cv_MSER_setEdgeBlurSize_int(instance: *mut c_void, edge_blur_size: i32, ocvrs_return: *mut Result<()>);
// getEdgeBlurSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:562
// ("cv::MSER::getEdgeBlurSize", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getEdgeBlurSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPass2Only(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:564
// ("cv::MSER::setPass2Only", vec![(pred!(mut, ["f"], ["bool"]), _)]),
pub fn cv_MSER_setPass2Only_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result<()>);
// getPass2Only()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:565
// ("cv::MSER::getPass2Only", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getPass2Only_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:567
// ("cv::MSER::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_MSER_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::MSER::to_Algorithm() generated
// ("cv::MSER::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MSER_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MSER::to_Feature2D() generated
// ("cv::MSER::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_MSER_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::MSER::delete() generated
// ("cv::MSER::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MSER_delete(instance: *mut c_void);
// create(int, float, int, int, int, int, ORB::ScoreType, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
// ("cv::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold"], ["int", "float", "int", "int", "int", "int", "cv::ORB::ScoreType", "int", "int"]), _)]),
pub fn cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features2d::ORB_ScoreType, patch_size: i32, fast_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ORB::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
// ("cv::ORB::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ORB_create(ocvrs_return: *mut Result<*mut c_void>);
// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:463
// ("cv::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_ORB_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:464
// ("cv::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:466
// ("cv::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
pub fn cv_ORB_setScaleFactor_double(instance: *mut c_void, scale_factor: f64, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:467
// ("cv::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:469
// ("cv::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
pub fn cv_ORB_setNLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result<()>);
// getNLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:470
// ("cv::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getNLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:472
// ("cv::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
pub fn cv_ORB_setEdgeThreshold_int(instance: *mut c_void, edge_threshold: i32, ocvrs_return: *mut Result<()>);
// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:473
// ("cv::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getEdgeThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:475
// ("cv::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
pub fn cv_ORB_setFirstLevel_int(instance: *mut c_void, first_level: i32, ocvrs_return: *mut Result<()>);
// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:476
// ("cv::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getFirstLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:478
// ("cv::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
pub fn cv_ORB_setWTA_K_int(instance: *mut c_void, wta_k: i32, ocvrs_return: *mut Result<()>);
// getWTA_K()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:479
// ("cv::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getWTA_K_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setScoreType(ORB::ScoreType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:481
// ("cv::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["cv::ORB::ScoreType"]), _)]),
pub fn cv_ORB_setScoreType_ScoreType(instance: *mut c_void, score_type: crate::features2d::ORB_ScoreType, ocvrs_return: *mut Result<()>);
// getScoreType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:482
// ("cv::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getScoreType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::ORB_ScoreType>);
// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:484
// ("cv::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
pub fn cv_ORB_setPatchSize_int(instance: *mut c_void, patch_size: i32, ocvrs_return: *mut Result<()>);
// getPatchSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:485
// ("cv::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:487
// ("cv::ORB::setFastThreshold", vec![(pred!(mut, ["fastThreshold"], ["int"]), _)]),
pub fn cv_ORB_setFastThreshold_int(instance: *mut c_void, fast_threshold: i32, ocvrs_return: *mut Result<()>);
// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:488
// ("cv::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getFastThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:489
// ("cv::ORB::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_ORB_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ORB::to_Algorithm() generated
// ("cv::ORB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ORB_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ORB::to_Feature2D() generated
// ("cv::ORB::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_ORB_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::ORB::delete() generated
// ("cv::ORB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ORB_delete(instance: *mut c_void);
// create(int, int, double, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "bool"]), _)]),
pub fn cv_SIFT_create_int_int_double_double_double_bool(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, enable_precise_upscale: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::SIFT::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
// ("cv::SIFT::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_SIFT_create(ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, double, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "int", "bool"]), _)]),
pub fn cv_SIFT_create_int_int_double_double_double_int_bool(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, enable_precise_upscale: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::SIFT::create(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType"], ["int", "int", "double", "double", "double", "int"]), _)]),
pub fn cv_SIFT_create_int_int_double_double_double_int(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:329
// ("cv::SIFT::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setNFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:331
// ("cv::SIFT::setNFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
pub fn cv_SIFT_setNFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result<()>);
// getNFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:332
// ("cv::SIFT::getNFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getNFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:334
// ("cv::SIFT::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
pub fn cv_SIFT_setNOctaveLayers_int(instance: *mut c_void, n_octave_layers: i32, ocvrs_return: *mut Result<()>);
// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:335
// ("cv::SIFT::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setContrastThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:337
// ("cv::SIFT::setContrastThreshold", vec![(pred!(mut, ["contrastThreshold"], ["double"]), _)]),
pub fn cv_SIFT_setContrastThreshold_double(instance: *mut c_void, contrast_threshold: f64, ocvrs_return: *mut Result<()>);
// getContrastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:338
// ("cv::SIFT::getContrastThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getContrastThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:340
// ("cv::SIFT::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["double"]), _)]),
pub fn cv_SIFT_setEdgeThreshold_double(instance: *mut c_void, edge_threshold: f64, ocvrs_return: *mut Result<()>);
// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:341
// ("cv::SIFT::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getEdgeThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:343
// ("cv::SIFT::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
pub fn cv_SIFT_setSigma_double(instance: *mut c_void, sigma: f64, ocvrs_return: *mut Result<()>);
// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:344
// ("cv::SIFT::getSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_SIFT_getSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::SIFT::to_Algorithm() generated
// ("cv::SIFT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_SIFT_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::SIFT::to_Feature2D() generated
// ("cv::SIFT::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_SIFT_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::SIFT::delete() generated
// ("cv::SIFT::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SIFT_delete(instance: *mut c_void);
// create(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, ["parameters"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
pub fn cv_SimpleBlobDetector_create_const_ParamsR(parameters: *const crate::features2d::SimpleBlobDetector_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::SimpleBlobDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimpleBlobDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// setParams(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:781
// ("cv::SimpleBlobDetector::setParams", vec![(pred!(mut, ["params"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
pub fn cv_SimpleBlobDetector_setParams_const_ParamsR(instance: *mut c_void, params: *const crate::features2d::SimpleBlobDetector_Params, ocvrs_return: *mut Result<()>);
// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:782
// ("cv::SimpleBlobDetector::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_SimpleBlobDetector_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::SimpleBlobDetector_Params>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:784
// ("cv::SimpleBlobDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_SimpleBlobDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBlobContours()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:785
// ("cv::SimpleBlobDetector::getBlobContours", vec![(pred!(const, [], []), _)]),
pub fn cv_SimpleBlobDetector_getBlobContours_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::SimpleBlobDetector::to_Algorithm() generated
// ("cv::SimpleBlobDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimpleBlobDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::SimpleBlobDetector::to_Feature2D() generated
// ("cv::SimpleBlobDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimpleBlobDetector_to_Feature2D(instance: *mut c_void) -> *mut c_void;
// cv::SimpleBlobDetector::delete() generated
// ("cv::SimpleBlobDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimpleBlobDetector_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:750
// ("cv::SimpleBlobDetector::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimpleBlobDetector_Params_Params(ocvrs_return: *mut Result<crate::features2d::SimpleBlobDetector_Params>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:774
// ("cv::SimpleBlobDetector::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_SimpleBlobDetector_Params_read_const_FileNodeR(instance: *const crate::features2d::SimpleBlobDetector_Params, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:775
// ("cv::SimpleBlobDetector::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_SimpleBlobDetector_Params_write_const_FileStorageR(instance: *const crate::features2d::SimpleBlobDetector_Params, fs: *mut c_void, ocvrs_return: *mut Result<()>);
