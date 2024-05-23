// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:127
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, charuco_corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:127
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, corner_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:148
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, diamond_corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:148
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:379
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:379
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, corners: *const c_void, ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::extendDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:146
// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
pub fn cv_aruco_extendDictionary_int_int(n_markers: i32, marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// extendDictionary(int, int, const Dictionary &, int)(Primitive, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:146
// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::aruco::Dictionary*", "int"]), _)]),
pub fn cv_aruco_extendDictionary_int_int_const_DictionaryR_int(n_markers: i32, marker_size: i32, base_dictionary: *const c_void, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::generateImageMarker(TraitClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:392
// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(dictionary: *const c_void, id: i32, side_pixels: i32, img: *const c_void, ocvrs_return: *mut Result<()>);
// generateImageMarker(const Dictionary &, int, int, OutputArray, int)(TraitClass, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:392
// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(dictionary: *const c_void, id: i32, side_pixels: i32, img: *const c_void, border_bits: i32, ocvrs_return: *mut Result<()>);
// getPredefinedDictionary(PredefinedDictionaryType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:127
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PredefinedDictionaryType"]), _)]),
pub fn cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(name: crate::objdetect::PredefinedDictionaryType, ocvrs_return: *mut Result<*mut c_void>);
// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:132
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
pub fn cv_aruco_getPredefinedDictionary_int(dict: i32, ocvrs_return: *mut Result<*mut c_void>);
// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:367
// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_createFaceDetectionMaskGenerator(ocvrs_return: *mut Result<*mut c_void>);
// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:191
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
pub fn cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, ocvrs_return: *mut Result<()>);
// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:191
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
pub fn cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, detect_threshold: f64, win_det_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:180
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int(rect_list: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:180
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int_double(rect_list: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:185
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(rect_list: *mut c_void, group_threshold: i32, eps: f64, weights: *mut c_void, level_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:182
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_int(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:182
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:188
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:188
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:211
// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:212
// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_BaseCascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:213
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:219
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:226
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result<()>);
// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:235
// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:236
// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getFeatureType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:237
// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getOldCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:238
// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:247
// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
pub fn cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result<()>);
// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:248
// ("cv::BaseCascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BaseCascadeClassifier::to_Algorithm() generated
// ("cv::BaseCascadeClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BaseCascadeClassifier::delete() generated
// ("cv::BaseCascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_delete(instance: *mut c_void);
// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:244
// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:245
// ("cv::BaseCascadeClassifier::MaskGenerator::initializeMask", vec![(pred!(mut, ["unnamed"], ["const cv::Mat*"]), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// cv::BaseCascadeClassifier::MaskGenerator::delete() generated
// ("cv::BaseCascadeClassifier::MaskGenerator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_delete(instance: *mut c_void);
// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:260
// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_CascadeClassifier(ocvrs_return: *mut Result<*mut c_void>);
// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:265
// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_CascadeClassifier_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:269
// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:276
// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:281
// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_CascadeClassifier_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<bool>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:297
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:297
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:319
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:319
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:344
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:344
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:354
// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:355
// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getFeatureType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:356
// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getOldCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:357
// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:359
// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_convert_const_StringR_const_StringR(oldcascade: *const c_char, newcascade: *const c_char, ocvrs_return: *mut Result<bool>);
// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:361
// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
pub fn cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result<()>);
// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:362
// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:364
// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_propCc(instance: *mut c_void) -> *mut c_void;
// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:364
// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
pub fn cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(instance: *mut c_void, val: *const c_void);
// cv::CascadeClassifier::delete() generated
// ("cv::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_delete(instance: *mut c_void);
// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:121
// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
pub fn cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(main_detector: *mut c_void, tracking_detector: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// run()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:124
// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_run(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// stop()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:125
// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_stop(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// resetTracking()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:126
// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_resetTracking(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:128
// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
pub fn cv_DetectionBasedTracker_process_const_MatR(instance: *mut c_void, image_gray: *const c_void, ocvrs_return: *mut Result<()>);
// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:130
// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
pub fn cv_DetectionBasedTracker_setParameters_const_ParametersR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// getParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:131
// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_getParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:135
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:136
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:155
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:158
// ("cv::DetectionBasedTracker::addObject", vec![(pred!(mut, ["location"], ["const cv::Rect*"]), _)]),
pub fn cv_DetectionBasedTracker_addObject_const_RectR(instance: *mut c_void, location: *const core::Rect, ocvrs_return: *mut Result<i32>);
// cv::DetectionBasedTracker::delete() generated
// ("cv::DetectionBasedTracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_delete(instance: *mut c_void);
// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:150
// ("cv::DetectionBasedTracker::ExtObject::ExtObject", vec![(pred!(mut, ["_id", "_location", "_status"], ["int", "cv::Rect", "cv::DetectionBasedTracker::ObjectStatus"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id: i32, _location: *const core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus, ocvrs_return: *mut Result<*mut c_void>);
// cv::DetectionBasedTracker::ExtObject::implicitClone() generated
// ("cv::DetectionBasedTracker::ExtObject::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:147
// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propId_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:147
// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propId_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:148
// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propLocation_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:148
// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:149
// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propStatus_const(instance: *const c_void, ocvrs_return: *mut crate::objdetect::DetectionBasedTracker_ObjectStatus);
// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:149
// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(instance: *mut c_void, val: crate::objdetect::DetectionBasedTracker_ObjectStatus);
// cv::DetectionBasedTracker::ExtObject::delete() generated
// ("cv::DetectionBasedTracker::ExtObject::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_delete(instance: *mut c_void);
// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:78
// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:80
// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(instance: *mut c_void, min: *const core::Size, ocvrs_return: *mut Result<()>);
// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:84
// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(instance: *mut c_void, max: *const core::Size, ocvrs_return: *mut Result<()>);
// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:88
// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:92
// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:96
// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getScaleFactor(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:100
// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setScaleFactor_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:104
// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMinNeighbours(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:108
// ("cv::DetectionBasedTracker::IDetector::setMinNeighbours", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// cv::DetectionBasedTracker::IDetector::delete() generated
// ("cv::DetectionBasedTracker::IDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_delete(instance: *mut c_void);
// Parameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:65
// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_Parameters(ocvrs_return: *mut Result<*mut c_void>);
// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:62
// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:62
// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:63
// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:63
// ("cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::Parameters::delete() generated
// ("cv::DetectionBasedTracker::Parameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_delete(instance: *mut c_void);
// cv::DetectionROI::defaultNew() generated
// ("cv::DetectionROI::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_defaultNew_const() -> *mut c_void;
// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:378
// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propScale_const(instance: *const c_void) -> f64;
// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:378
// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_DetectionROI_propScale_const_double(instance: *mut c_void, val: f64);
// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:380
// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propLocations_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:380
// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
pub fn cv_DetectionROI_propLocations_const_vectorLPointG(instance: *mut c_void, val: *const c_void);
// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:382
// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propConfidences_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:382
// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
pub fn cv_DetectionROI_propConfidences_const_vectorLdoubleG(instance: *mut c_void, val: *const c_void);
// cv::DetectionROI::delete() generated
// ("cv::DetectionROI::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionROI_delete(instance: *mut c_void);
// setInputSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:29
// ("cv::FaceDetectorYN::setInputSize", vec![(pred!(mut, ["input_size"], ["const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_setInputSize_const_SizeR(instance: *mut c_void, input_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getInputSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:31
// ("cv::FaceDetectorYN::getInputSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getInputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
// setScoreThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:37
// ("cv::FaceDetectorYN::setScoreThreshold", vec![(pred!(mut, ["score_threshold"], ["float"]), _)]),
pub fn cv_FaceDetectorYN_setScoreThreshold_float(instance: *mut c_void, score_threshold: f32, ocvrs_return: *mut Result<()>);
// getScoreThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:39
// ("cv::FaceDetectorYN::getScoreThreshold", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getScoreThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setNMSThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:45
// ("cv::FaceDetectorYN::setNMSThreshold", vec![(pred!(mut, ["nms_threshold"], ["float"]), _)]),
pub fn cv_FaceDetectorYN_setNMSThreshold_float(instance: *mut c_void, nms_threshold: f32, ocvrs_return: *mut Result<()>);
// getNMSThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:47
// ("cv::FaceDetectorYN::getNMSThreshold", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getNMSThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setTopK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:53
// ("cv::FaceDetectorYN::setTopK", vec![(pred!(mut, ["top_k"], ["int"]), _)]),
pub fn cv_FaceDetectorYN_setTopK_int(instance: *mut c_void, top_k: i32, ocvrs_return: *mut Result<()>);
// getTopK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:55
// ("cv::FaceDetectorYN::getTopK", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getTopK(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:72
// ("cv::FaceDetectorYN::detect", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<i32>);
// create(const String &, const String &, const Size &, float, float, int, int, int)(InString, InString, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:85
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(model: *const c_char, config: *const c_char, input_size: *const core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::create(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:85
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size"], ["const cv::String*", "const cv::String*", "const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR(model: *const c_char, config: *const c_char, input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, const Size &, float, float, int, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:106
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR_float_float_int_int_int(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, input_size: *const core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::create(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:106
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::delete() generated
// ("cv::FaceDetectorYN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_delete(instance: *mut c_void);
// alignCrop(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:136
// ("cv::FaceRecognizerSF::alignCrop", vec![(pred!(const, ["src_img", "face_box", "aligned_img"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src_img: *const c_void, face_box: *const c_void, aligned_img: *const c_void, ocvrs_return: *mut Result<()>);
// feature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:142
// ("cv::FaceRecognizerSF::feature", vec![(pred!(mut, ["aligned_img", "face_feature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, aligned_img: *const c_void, face_feature: *const c_void, ocvrs_return: *mut Result<()>);
// match(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:149
// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2", "dis_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(instance: *const c_void, face_feature1: *const c_void, face_feature2: *const c_void, dis_type: i32, ocvrs_return: *mut Result<f64>);
// cv::FaceRecognizerSF::match(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:149
// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, face_feature1: *const c_void, face_feature2: *const c_void, ocvrs_return: *mut Result<f64>);
// create(const String &, const String &, int, int)(InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:157
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "int", "int"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(model: *const c_char, config: *const c_char, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::create(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:157
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:169
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "int", "int"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_int_int(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::create(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:169
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::delete() generated
// ("cv::FaceRecognizerSF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceRecognizerSF_delete(instance: *mut c_void);
// GraphicalCodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:17
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// GraphicalCodeDetector(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:19
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector_const_GraphicalCodeDetectorR(unnamed: *const c_void) -> *mut c_void;
// GraphicalCodeDetector(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:20
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector_GraphicalCodeDetectorRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:21
// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_operatorST_const_GraphicalCodeDetectorR(instance: *mut c_void, unnamed: *const c_void);
// operator=(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:22
// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_operatorST_GraphicalCodeDetectorRR(instance: *mut c_void, unnamed: *mut c_void);
// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:28
// ("cv::GraphicalCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:37
// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GraphicalCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:37
// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:45
// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GraphicalCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:45
// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(instance: *const c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:53
// ("cv::GraphicalCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:61
// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, straight_code: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:61
// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
pub fn cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, ocvrs_return: *mut Result<bool>);
// detectAndDecodeMulti(InputArray, std::vector<std::string> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:74
// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_code"], ["const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:74
// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info"], ["const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::implicitClone() generated
// ("cv::GraphicalCodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GraphicalCodeDetector_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GraphicalCodeDetector::delete() generated
// ("cv::GraphicalCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GraphicalCodeDetector_delete(instance: *mut c_void);
// HOGDescriptor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:415
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor(ocvrs_return: *mut Result<*mut c_void>);
// HOGDescriptor(Size, Size, Size, Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Enum, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:435
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "cv::HOGDescriptor::HistogramNormType", "double", "bool", "int", "bool"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(_win_size: *const core::Size, _block_size: *const core::Size, _block_stride: *const core::Size, _cell_size: *const core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: crate::objdetect::HOGDescriptor_HistogramNormType, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::HOGDescriptor::HOGDescriptor(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:435
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int(_win_size: *const core::Size, _block_size: *const core::Size, _block_stride: *const core::Size, _cell_size: *const core::Size, _nbins: i32, ocvrs_return: *mut Result<*mut c_void>);
// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:451
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:459
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:470
// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:474
// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_checkDetectorSize_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getWinSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:478
// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_getWinSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:485
// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["svmdetector"], ["const cv::_InputArray*"]), _)]),
pub fn cv_HOGDescriptor_setSVMDetector_const__InputArrayR(instance: *mut c_void, svmdetector: *const c_void, ocvrs_return: *mut Result<()>);
// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:490
// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
pub fn cv_HOGDescriptor_read_FileNodeR(instance: *mut c_void, fn_: *mut c_void, ocvrs_return: *mut Result<bool>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:496
// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, objname: *const c_char, ocvrs_return: *mut Result<()>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:502
// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_load_const_StringR_const_StringR(instance: *mut c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:502
// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:508
// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_save_const_const_StringR_const_StringR(instance: *const c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:508
// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:513
// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
pub fn cv_HOGDescriptor_copyTo_const_HOGDescriptorR(instance: *const c_void, c: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:524
// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, descriptors: *mut c_void, win_stride: *const core::Size, padding: *const core::Size, locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:524
// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
pub fn cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(instance: *const c_void, img: *const c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:540
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:540
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, weights: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:556
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:556
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:576
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, found_weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:576
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, found_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:595
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:595
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// computeGradient(InputArray, InputOutputArray, InputOutputArray, Size, Size)(InputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:607
// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size"]), _)]),
pub fn cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(instance: *const c_void, img: *const c_void, grad: *const c_void, angle_ofs: *const c_void, padding_tl: *const core::Size, padding_br: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::computeGradient(InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:607
// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, img: *const c_void, grad: *const c_void, angle_ofs: *const c_void, ocvrs_return: *mut Result<()>);
// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:612
// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_getDefaultPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:618
// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_getDaimlerPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
// detectROI(InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:676
// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
pub fn cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(instance: *const c_void, img: *const c_void, locations: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:676
// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, locations: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScaleROI(InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:689
// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR_double_int(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, locations: *mut c_void, hit_threshold: f64, group_threshold: i32, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScaleROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:689
// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, locations: *mut c_void, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:701
// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
pub fn cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(instance: *const c_void, rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:621
// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propWinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:621
// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propWinSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:624
// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propBlockSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:624
// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propBlockSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:627
// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propBlockStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:627
// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propBlockStride_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:630
// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propCellSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:630
// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propCellSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:633
// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propNbins_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:633
// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propNbins_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:636
// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propDerivAperture_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:636
// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propDerivAperture_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:639
// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propWinSigma_const(instance: *const c_void) -> f64;
// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:639
// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_HOGDescriptor_propWinSigma_const_double(instance: *mut c_void, val: f64);
// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:642
// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propHistogramNormType_const(instance: *const c_void, ocvrs_return: *mut crate::objdetect::HOGDescriptor_HistogramNormType);
// cv::HOGDescriptor::setHistogramNormType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:642
// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const cv::HOGDescriptor::HistogramNormType"]), _)]),
pub fn cv_HOGDescriptor_propHistogramNormType_const_HistogramNormType(instance: *mut c_void, val: crate::objdetect::HOGDescriptor_HistogramNormType);
// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:645
// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propL2HysThreshold_const(instance: *const c_void) -> f64;
// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:645
// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_HOGDescriptor_propL2HysThreshold_const_double(instance: *mut c_void, val: f64);
// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:648
// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propGammaCorrection_const(instance: *const c_void) -> bool;
// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:648
// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_HOGDescriptor_propGammaCorrection_const_bool(instance: *mut c_void, val: bool);
// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:651
// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propSvmDetector_const(instance: *const c_void) -> *mut c_void;
// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:651
// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
pub fn cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(instance: *mut c_void, val: *const c_void);
// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:654
// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propOclSvmDetector_const(instance: *const c_void) -> *mut c_void;
// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:654
// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
pub fn cv_HOGDescriptor_propOclSvmDetector_const_UMat(instance: *mut c_void, val: *const c_void);
// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:657
// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propFree_coef_const(instance: *const c_void) -> f32;
// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:657
// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_HOGDescriptor_propFree_coef_const_float(instance: *mut c_void, val: f32);
// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:660
// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propNlevels_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:660
// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propNlevels_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:663
// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propSignedGradient_const(instance: *const c_void) -> bool;
// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:663
// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_HOGDescriptor_propSignedGradient_const_bool(instance: *mut c_void, val: bool);
// cv::HOGDescriptor::delete() generated
// ("cv::HOGDescriptor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_delete(instance: *mut c_void);
// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:775
// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetector_QRCodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:781
// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
pub fn cv_QRCodeDetector_setEpsX_double(instance: *mut c_void, eps_x: f64, ocvrs_return: *mut Result<*mut c_void>);
// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:786
// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
pub fn cv_QRCodeDetector_setEpsY_double(instance: *mut c_void, eps_y: f64, ocvrs_return: *mut Result<*mut c_void>);
// setUseAlignmentMarkers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:792
// ("cv::QRCodeDetector::setUseAlignmentMarkers", vec![(pred!(mut, ["useAlignmentMarkers"], ["bool"]), _)]),
pub fn cv_QRCodeDetector_setUseAlignmentMarkers_bool(instance: *mut c_void, use_alignment_markers: bool, ocvrs_return: *mut Result<*mut c_void>);
// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:801
// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:801
// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:809
// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:809
// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeDetector::implicitClone() generated
// ("cv::QRCodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetector_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::QRCodeDetector::to_GraphicalCodeDetector() generated
// ("cv::QRCodeDetector::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetector_to_GraphicalCodeDetector(instance: *mut c_void) -> *mut c_void;
// cv::QRCodeDetector::delete() generated
// ("cv::QRCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetector_delete(instance: *mut c_void);
// QRCodeDetectorAruco()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:815
// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_QRCodeDetectorAruco(ocvrs_return: *mut Result<*mut c_void>);
// QRCodeDetectorAruco(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:850
// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
pub fn cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(params: *const crate::objdetect::QRCodeDetectorAruco_Params, ocvrs_return: *mut Result<*mut c_void>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:853
// ("cv::QRCodeDetectorAruco::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::QRCodeDetectorAruco_Params>);
// setDetectorParameters(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:856
// ("cv::QRCodeDetectorAruco::setDetectorParameters", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
pub fn cv_QRCodeDetectorAruco_setDetectorParameters_const_ParamsR(instance: *mut c_void, params: *const crate::objdetect::QRCodeDetectorAruco_Params, ocvrs_return: *mut Result<*mut c_void>);
// getArucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:859
// ("cv::QRCodeDetectorAruco::getArucoParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_getArucoParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setArucoParameters(const aruco::DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:862
// ("cv::QRCodeDetectorAruco::setArucoParameters", vec![(pred!(mut, ["params"], ["const cv::aruco::DetectorParameters*"]), _)]),
pub fn cv_QRCodeDetectorAruco_setArucoParameters_const_DetectorParametersR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<()>);
// cv::QRCodeDetectorAruco::implicitClone() generated
// ("cv::QRCodeDetectorAruco::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::QRCodeDetectorAruco::to_GraphicalCodeDetector() generated
// ("cv::QRCodeDetectorAruco::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_to_GraphicalCodeDetector(instance: *mut c_void) -> *mut c_void;
// cv::QRCodeDetectorAruco::delete() generated
// ("cv::QRCodeDetectorAruco::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:818
// ("cv::QRCodeDetectorAruco::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_Params_Params(ocvrs_return: *mut Result<crate::objdetect::QRCodeDetectorAruco_Params>);
// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:757
// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
pub fn cv_QRCodeEncoder_create_const_ParamsR(parameters: *const crate::objdetect::QRCodeEncoder_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:757
// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_create(ocvrs_return: *mut Result<*mut c_void>);
// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:763
// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcode: *const c_void, ocvrs_return: *mut Result<()>);
// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:769
// ("cv::QRCodeEncoder::encodeStructuredAppend", vec![(pred!(mut, ["encoded_info", "qrcodes"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcodes: *const c_void, ocvrs_return: *mut Result<()>);
// cv::QRCodeEncoder::delete() generated
// ("cv::QRCodeEncoder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:738
// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_Params_Params(ocvrs_return: *mut Result<crate::objdetect::QRCodeEncoder_Params>);
// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:153
// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
pub fn cv_SimilarRects_SimilarRects_double(_eps: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:154
// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
pub fn cv_SimilarRects_operator___const_const_RectR_const_RectR(instance: *const c_void, r1: *const core::Rect, r2: *const core::Rect, ocvrs_return: *mut Result<bool>);
// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:162
// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
pub fn cv_SimilarRects_propEps_const(instance: *const c_void) -> f64;
// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:162
// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_SimilarRects_propEps_const_double(instance: *mut c_void, val: f64);
// cv::SimilarRects::delete() generated
// ("cv::SimilarRects::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimilarRects_delete(instance: *mut c_void);
// ArucoDetector(const Dictionary &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:284
// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, ["dictionary", "detectorParams", "refineParams"], ["const cv::aruco::Dictionary*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(dictionary: *const c_void, detector_params: *const c_void, refine_params: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::ArucoDetector::ArucoDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:284
// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_ArucoDetector(ocvrs_return: *mut Result<*mut c_void>);
// detectMarkers(InputArray, OutputArrayOfArrays, OutputArray, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:308
// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, corners: *const c_void, ids: *const c_void, rejected_img_points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::detectMarkers(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:308
// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, corners: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<()>);
// refineDetectedMarkers(InputArray, const Board &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, OutputArray)(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:333
// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "recoveredIdxs"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, recovered_idxs: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::refineDetectedMarkers(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:333
// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, ocvrs_return: *mut Result<()>);
// getDictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:339
// ("cv::aruco::ArucoDetector::getDictionary", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getDictionary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDictionary(const Dictionary &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:340
// ("cv::aruco::ArucoDetector::setDictionary", vec![(pred!(mut, ["dictionary"], ["const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(instance: *mut c_void, dictionary: *const c_void, ocvrs_return: *mut Result<()>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:342
// ("cv::aruco::ArucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:343
// ("cv::aruco::ArucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(instance: *mut c_void, detector_parameters: *const c_void, ocvrs_return: *mut Result<()>);
// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:345
// ("cv::aruco::ArucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getRefineParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:346
// ("cv::aruco::ArucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(instance: *mut c_void, refine_parameters: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:350
// ("cv::aruco::ArucoDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_ArucoDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:354
// ("cv::aruco::ArucoDetector::write", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:358
// ("cv::aruco::ArucoDetector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_ArucoDetector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::to_Algorithm() generated
// ("cv::aruco::ArucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::aruco::ArucoDetector::delete() generated
// ("cv::aruco::ArucoDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_delete(instance: *mut c_void);
// Board(InputArrayOfArrays, const Dictionary &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:33
// ("cv::aruco::Board::Board", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(obj_points: *const c_void, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:37
// ("cv::aruco::Board::getDictionary", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getDictionary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getObjPoints()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:49
// ("cv::aruco::Board::getObjPoints", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getObjPoints_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:54
// ("cv::aruco::Board::getIds", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getRightBottomCorner()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:58
// ("cv::aruco::Board::getRightBottomCorner", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getRightBottomCorner_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point3f>);
// matchImagePoints(InputArrayOfArrays, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:78
// ("cv::aruco::Board::matchImagePoints", vec![(pred!(const, ["detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, obj_points: *const c_void, img_points: *const c_void, ocvrs_return: *mut Result<()>);
// generateImage(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:91
// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(instance: *const c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result<()>);
// cv::aruco::Board::generateImage(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:91
// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(instance: *const c_void, out_size: *const core::Size, img: *const c_void, ocvrs_return: *mut Result<()>);
// Board()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:94
// ("cv::aruco::Board::Board", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Board_Board(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Board::implicitClone() generated
// ("cv::aruco::Board::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Board::delete() generated
// ("cv::aruco::Board::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Board_delete(instance: *mut c_void);
// CharucoBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:146
// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(size: *const core::Size, square_length: f32, marker_length: f32, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoBoard::CharucoBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:146
// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(size: *const core::Size, square_length: f32, marker_length: f32, dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setLegacyPattern(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:158
// ("cv::aruco::CharucoBoard::setLegacyPattern", vec![(pred!(mut, ["legacyPattern"], ["bool"]), _)]),
pub fn cv_aruco_CharucoBoard_setLegacyPattern_bool(instance: *mut c_void, legacy_pattern: bool, ocvrs_return: *mut Result<()>);
// getLegacyPattern()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:159
// ("cv::aruco::CharucoBoard::getLegacyPattern", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getLegacyPattern_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:161
// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getChessboardSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getSquareLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:162
// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getSquareLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:163
// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getChessboardCorners()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:167
// ("cv::aruco::CharucoBoard::getChessboardCorners", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getChessboardCorners_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNearestMarkerIdx()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:171
// ("cv::aruco::CharucoBoard::getNearestMarkerIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getNearestMarkerIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNearestMarkerCorners()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:175
// ("cv::aruco::CharucoBoard::getNearestMarkerCorners", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getNearestMarkerCorners_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// checkCharucoCornersCollinear(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:188
// ("cv::aruco::CharucoBoard::checkCharucoCornersCollinear", vec![(pred!(const, ["charucoIds"], ["const cv::_InputArray*"]), _)]),
pub fn cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(instance: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<bool>);
// CharucoBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:191
// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoBoard_CharucoBoard(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoBoard::implicitClone() generated
// ("cv::aruco::CharucoBoard::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoBoard::to_Board() generated
// ("cv::aruco::CharucoBoard::to_Board", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoBoard_to_Board(instance: *mut c_void) -> *mut c_void;
// cv::aruco::CharucoBoard::delete() generated
// ("cv::aruco::CharucoBoard::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoBoard_delete(instance: *mut c_void);
// CharucoDetector(const CharucoBoard &, const CharucoParameters &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:42
// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board", "charucoParams", "detectorParams", "refineParams"], ["const cv::aruco::CharucoBoard*", "const cv::aruco::CharucoParameters*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(board: *const c_void, charuco_params: *const c_void, detector_params: *const c_void, refine_params: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoDetector::CharucoDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:42
// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
pub fn cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(board: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:47
// ("cv::aruco::CharucoDetector::getBoard", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getBoard_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setBoard(const CharucoBoard &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:48
// ("cv::aruco::CharucoDetector::setBoard", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
pub fn cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(instance: *mut c_void, board: *const c_void, ocvrs_return: *mut Result<()>);
// getCharucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:50
// ("cv::aruco::CharucoDetector::getCharucoParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getCharucoParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCharucoParameters(CharucoParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:51
// ("cv::aruco::CharucoDetector::setCharucoParameters", vec![(pred!(mut, ["charucoParameters"], ["cv::aruco::CharucoParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(instance: *mut c_void, charuco_parameters: *mut c_void, ocvrs_return: *mut Result<()>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:53
// ("cv::aruco::CharucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:54
// ("cv::aruco::CharucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(instance: *mut c_void, detector_parameters: *const c_void, ocvrs_return: *mut Result<()>);
// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:56
// ("cv::aruco::CharucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getRefineParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:57
// ("cv::aruco::CharucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(instance: *mut c_void, refine_parameters: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<()>);
// detectBoard(InputArray, OutputArray, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:84
// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::detectBoard(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:84
// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<()>);
// detectDiamonds(InputArray, OutputArrayOfArrays, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:108
// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::detectDiamonds(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:108
// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::to_Algorithm() generated
// ("cv::aruco::CharucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::aruco::CharucoDetector::delete() generated
// ("cv::aruco::CharucoDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoDetector_delete(instance: *mut c_void);
// CharucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:16
// ("cv::aruco::CharucoParameters::CharucoParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoParameters_CharucoParameters(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoParameters::implicitClone() generated
// ("cv::aruco::CharucoParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::cameraMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:21
// ("cv::aruco::CharucoParameters::cameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propCameraMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::setCameraMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:21
// ("cv::aruco::CharucoParameters::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_CharucoParameters_propCameraMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::CharucoParameters::distCoeffs() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:24
// ("cv::aruco::CharucoParameters::distCoeffs", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propDistCoeffs_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::setDistCoeffs(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:24
// ("cv::aruco::CharucoParameters::setDistCoeffs", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_CharucoParameters_propDistCoeffs_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::CharucoParameters::minMarkers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:27
// ("cv::aruco::CharucoParameters::minMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propMinMarkers_const(instance: *const c_void) -> i32;
// cv::aruco::CharucoParameters::setMinMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:27
// ("cv::aruco::CharucoParameters::setMinMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_CharucoParameters_propMinMarkers_const_int(instance: *mut c_void, val: i32);
// cv::aruco::CharucoParameters::tryRefineMarkers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:30
// ("cv::aruco::CharucoParameters::tryRefineMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propTryRefineMarkers_const(instance: *const c_void) -> bool;
// cv::aruco::CharucoParameters::setTryRefineMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:30
// ("cv::aruco::CharucoParameters::setTryRefineMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_CharucoParameters_propTryRefineMarkers_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::CharucoParameters::delete() generated
// ("cv::aruco::CharucoParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoParameters_delete(instance: *mut c_void);
// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:26
// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return: *mut Result<*mut c_void>);
// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:64
// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeDetectorParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:68
// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::aruco::DetectorParameters::writeDetectorParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:68
// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::aruco::DetectorParameters::implicitClone() generated
// ("cv::aruco::DetectorParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:71
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:71
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:74
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:74
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:77
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:77
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:80
// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:80
// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:86
// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:86
// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:92
// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:92
// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:95
// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:95
// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:98
// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:98
// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:101
// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinDistanceToBorder_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:101
// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:115
// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:115
// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minGroupDistance() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:121
// ("cv::aruco::DetectorParameters::minGroupDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinGroupDistance_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setMinGroupDistance(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:121
// ("cv::aruco::DetectorParameters::setMinGroupDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinGroupDistance_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:124
// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMethod_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:124
// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:134
// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:134
// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:145
// ("cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:145
// ("cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:148
// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:148
// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:151
// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:151
// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:154
// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMarkerBorderBits_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:154
// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:157
// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:157
// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:163
// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:163
// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:169
// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:169
// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:174
// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinOtsuStdDev_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:174
// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:177
// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propErrorCorrectionRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:177
// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:184
// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:184
// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:187
// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:187
// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:191
// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:191
// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:194
// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:194
// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:200
// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:200
// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:203
// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:203
// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:210
// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:210
// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:213
// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagDeglitch_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:213
// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:219
// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propDetectInvertedMarker_const(instance: *const c_void) -> bool;
// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:219
// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::DetectorParameters::useAruco3Detection() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:227
// ("cv::aruco::DetectorParameters::useAruco3Detection", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propUseAruco3Detection_const(instance: *const c_void) -> bool;
// cv::aruco::DetectorParameters::setUseAruco3Detection(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:227
// ("cv::aruco::DetectorParameters::setUseAruco3Detection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_DetectorParameters_propUseAruco3Detection_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::DetectorParameters::minSideLengthCanonicalImg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:230
// ("cv::aruco::DetectorParameters::minSideLengthCanonicalImg", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:230
// ("cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:233
// ("cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:233
// ("cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::delete() generated
// ("cv::aruco::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_DetectorParameters_delete(instance: *mut c_void);
// Dictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:36
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Dictionary_Dictionary(ocvrs_return: *mut Result<*mut c_void>);
// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:44
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize", "maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_aruco_Dictionary_Dictionary_const_MatR_int_int(bytes_list: *const c_void, _marker_size: i32, maxcorr: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Dictionary::Dictionary(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:44
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize"], ["const cv::Mat*", "int"]), _)]),
pub fn cv_aruco_Dictionary_Dictionary_const_MatR_int(bytes_list: *const c_void, _marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:56
// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_Dictionary_readDictionary_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeDictionary(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:60
// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::aruco::Dictionary::writeDictionary(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:60
// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_Dictionary_writeDictionary_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:66
// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
pub fn cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(instance: *const c_void, only_bits: *const c_void, idx: *mut i32, rotation: *mut i32, max_correction_rate: f64, ocvrs_return: *mut Result<bool>);
// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:72
// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
pub fn cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(instance: *const c_void, bits: *const c_void, id: i32, all_rotations: bool, ocvrs_return: *mut Result<i32>);
// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:72
// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(instance: *const c_void, bits: *const c_void, id: i32, ocvrs_return: *mut Result<i32>);
// generateImageMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:77
// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(instance: *const c_void, id: i32, side_pixels: i32, _img: *const c_void, border_bits: i32, ocvrs_return: *mut Result<()>);
// cv::aruco::Dictionary::generateImageMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:77
// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(instance: *const c_void, id: i32, side_pixels: i32, _img: *const c_void, ocvrs_return: *mut Result<()>);
// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:82
// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
pub fn cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:87
// ("cv::aruco::Dictionary::getBitsFromByteList", vec![(pred!(mut, ["byteList", "markerSize"], ["const cv::Mat*", "int"]), _)]),
pub fn cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list: *const c_void, marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Dictionary::implicitClone() generated
// ("cv::aruco::Dictionary::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:32
// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propBytesList_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:32
// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_Dictionary_propBytesList_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:33
// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propMarkerSize_const(instance: *const c_void) -> i32;
// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:33
// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_Dictionary_propMarkerSize_const_int(instance: *mut c_void, val: i32);
// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:34
// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propMaxCorrectionBits_const(instance: *const c_void) -> i32;
// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:34
// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_Dictionary_propMaxCorrectionBits_const_int(instance: *mut c_void, val: i32);
// cv::aruco::Dictionary::delete() generated
// ("cv::aruco::Dictionary::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Dictionary_delete(instance: *mut c_void);
// GridBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:118
// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(size: *const core::Size, marker_length: f32, marker_separation: f32, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::GridBoard::GridBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:118
// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(size: *const core::Size, marker_length: f32, marker_separation: f32, dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:121
// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:122
// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:123
// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getMarkerSeparation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// GridBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:126
// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_GridBoard_GridBoard(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::GridBoard::implicitClone() generated
// ("cv::aruco::GridBoard::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::GridBoard::to_Board() generated
// ("cv::aruco::GridBoard::to_Board", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_GridBoard_to_Board(instance: *mut c_void) -> *mut c_void;
// cv::aruco::GridBoard::delete() generated
// ("cv::aruco::GridBoard::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_GridBoard_delete(instance: *mut c_void);
// RefineParameters(float, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:239
// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, ["minRepDistance", "errorCorrectionRate", "checkAllOrders"], ["float", "float", "bool"]), _)]),
pub fn cv_aruco_RefineParameters_RefineParameters_float_float_bool(min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// cv::aruco::RefineParameters::RefineParameters() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:239
// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_RefineParameters_RefineParameters(ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// readRefineParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:244
// ("cv::aruco::RefineParameters::readRefineParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(instance: *const crate::objdetect::RefineParameters, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeRefineParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:248
// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(instance: *const crate::objdetect::RefineParameters, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::aruco::RefineParameters::writeRefineParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:248
// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(instance: *const crate::objdetect::RefineParameters, fs: *mut c_void, ocvrs_return: *mut Result<bool>);
// BarcodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:23
// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_BarcodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// BarcodeDetector(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:30
// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, ["prototxt_path", "model_path"], ["const std::string*", "const std::string*"]), _)]),
pub fn cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(prototxt_path: *const c_char, model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// decodeWithType(InputArray, InputArray, std::vector<std::string> &, std::vector<std::string> &)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:43
// ("cv::barcode::BarcodeDetector::decodeWithType", vec![(pred!(const, ["img", "points", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, ocvrs_return: *mut Result<bool>);
// detectAndDecodeWithType(InputArray, std::vector<std::string> &, std::vector<std::string> &, OutputArray)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:56
// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type", "points"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::barcode::BarcodeDetector::detectAndDecodeWithType(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:56
// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, ocvrs_return: *mut Result<bool>);
// getDownsamplingThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:65
// ("cv::barcode::BarcodeDetector::getDownsamplingThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_getDownsamplingThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDownsamplingThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:75
// ("cv::barcode::BarcodeDetector::setDownsamplingThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
pub fn cv_barcode_BarcodeDetector_setDownsamplingThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// getDetectorScales(std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:81
// ("cv::barcode::BarcodeDetector::getDetectorScales", vec![(pred!(const, ["sizes"], ["std::vector<float>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_getDetectorScales_const_vectorLfloatGR(instance: *const c_void, sizes: *mut c_void, ocvrs_return: *mut Result<()>);
// setDetectorScales(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:90
// ("cv::barcode::BarcodeDetector::setDetectorScales", vec![(pred!(mut, ["sizes"], ["const std::vector<float>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_setDetectorScales_const_vectorLfloatGR(instance: *mut c_void, sizes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGradientThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:96
// ("cv::barcode::BarcodeDetector::getGradientThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_getGradientThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGradientThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:105
// ("cv::barcode::BarcodeDetector::setGradientThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
pub fn cv_barcode_BarcodeDetector_setGradientThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::barcode::BarcodeDetector::implicitClone() generated
// ("cv::barcode::BarcodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::barcode::BarcodeDetector::to_GraphicalCodeDetector() generated
// ("cv::barcode::BarcodeDetector::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_to_GraphicalCodeDetector(instance: *mut c_void) -> *mut c_void;
// cv::barcode::BarcodeDetector::delete() generated
// ("cv::barcode::BarcodeDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_delete(instance: *mut c_void);
