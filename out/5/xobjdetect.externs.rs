// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:339
// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_createFaceDetectionMaskGenerator(ocvrs_return: *mut Result<*mut c_void>);
// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:163
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
pub fn cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, ocvrs_return: *mut Result<()>);
// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:163
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
pub fn cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, detect_threshold: f64, win_det_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:152
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int(rect_list: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:152
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int_double(rect_list: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:157
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(rect_list: *mut c_void, group_threshold: i32, eps: f64, weights: *mut c_void, level_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:154
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_int(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:154
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:160
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:160
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
pub fn cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:183
// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:184
// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_BaseCascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:185
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:191
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:198
// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result<()>);
// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:207
// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:208
// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getFeatureType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:209
// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getOldCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:210
// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:219
// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
pub fn cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result<()>);
// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:220
// ("cv::BaseCascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::BaseCascadeClassifier::to_Algorithm() generated
// ("cv::BaseCascadeClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::BaseCascadeClassifier::delete() generated
// ("cv::BaseCascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_delete(instance: *mut c_void);
// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:216
// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:217
// ("cv::BaseCascadeClassifier::MaskGenerator::initializeMask", vec![(pred!(mut, ["unnamed"], ["const cv::Mat*"]), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// cv::BaseCascadeClassifier::MaskGenerator::delete() generated
// ("cv::BaseCascadeClassifier::MaskGenerator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_BaseCascadeClassifier_MaskGenerator_delete(instance: *mut c_void);
// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:232
// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_CascadeClassifier(ocvrs_return: *mut Result<*mut c_void>);
// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:237
// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_CascadeClassifier_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:241
// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:248
// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:253
// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_CascadeClassifier_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<bool>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:269
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:269
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:291
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:291
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:316
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result<()>);
// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:316
// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:326
// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:327
// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getFeatureType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:328
// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
pub fn cv_CascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getOldCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:329
// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:331
// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_CascadeClassifier_convert_const_StringR_const_StringR(oldcascade: *const c_char, newcascade: *const c_char, ocvrs_return: *mut Result<bool>);
// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:333
// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
pub fn cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result<()>);
// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:334
// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:336
// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_propCc(instance: *mut c_void) -> *mut c_void;
// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:336
// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
pub fn cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(instance: *mut c_void, val: *const c_void);
// cv::CascadeClassifier::delete() generated
// ("cv::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CascadeClassifier_delete(instance: *mut c_void);
// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:121
// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
pub fn cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(main_detector: *mut c_void, tracking_detector: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:124
// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_run(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:125
// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_stop(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// resetTracking()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:126
// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_resetTracking(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:128
// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
pub fn cv_DetectionBasedTracker_process_const_MatR(instance: *mut c_void, image_gray: *const c_void, ocvrs_return: *mut Result<()>);
// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:130
// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
pub fn cv_DetectionBasedTracker_setParameters_const_ParametersR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// getParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:131
// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_getParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:135
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:136
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:155
// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
pub fn cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result<()>);
// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:158
// ("cv::DetectionBasedTracker::addObject", vec![(pred!(mut, ["location"], ["const cv::Rect*"]), _)]),
pub fn cv_DetectionBasedTracker_addObject_const_RectR(instance: *mut c_void, location: *const core::Rect, ocvrs_return: *mut Result<i32>);
// cv::DetectionBasedTracker::delete() generated
// ("cv::DetectionBasedTracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_delete(instance: *mut c_void);
// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:150
// ("cv::DetectionBasedTracker::ExtObject::ExtObject", vec![(pred!(mut, ["_id", "_location", "_status"], ["int", "cv::Rect", "cv::DetectionBasedTracker::ObjectStatus"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id: i32, _location: *const core::Rect, _status: crate::xobjdetect::DetectionBasedTracker_ObjectStatus, ocvrs_return: *mut Result<*mut c_void>);
// cv::DetectionBasedTracker::ExtObject::implicitClone() generated
// ("cv::DetectionBasedTracker::ExtObject::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:147
// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propId_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:147
// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propId_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:148
// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propLocation_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:148
// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:149
// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propStatus_const(instance: *const c_void, ocvrs_return: *mut crate::xobjdetect::DetectionBasedTracker_ObjectStatus);
// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:149
// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(instance: *mut c_void, val: crate::xobjdetect::DetectionBasedTracker_ObjectStatus);
// cv::DetectionBasedTracker::ExtObject::delete() generated
// ("cv::DetectionBasedTracker::ExtObject::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_ExtObject_delete(instance: *mut c_void);
// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:78
// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(instance: *mut c_void, image: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:80
// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(instance: *mut c_void, min: *const core::Size, ocvrs_return: *mut Result<()>);
// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:84
// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(instance: *mut c_void, max: *const core::Size, ocvrs_return: *mut Result<()>);
// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:88
// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:92
// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:96
// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getScaleFactor(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:100
// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setScaleFactor_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:104
// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_getMinNeighbours(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:108
// ("cv::DetectionBasedTracker::IDetector::setMinNeighbours", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// cv::DetectionBasedTracker::IDetector::delete() generated
// ("cv::DetectionBasedTracker::IDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_IDetector_delete(instance: *mut c_void);
// Parameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:65
// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_Parameters(ocvrs_return: *mut Result<*mut c_void>);
// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:62
// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:62
// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:63
// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(instance: *const c_void) -> i32;
// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:63
// ("cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const_int(instance: *mut c_void, val: i32);
// cv::DetectionBasedTracker::Parameters::delete() generated
// ("cv::DetectionBasedTracker::Parameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionBasedTracker_Parameters_delete(instance: *mut c_void);
// cv::DetectionROI::defaultNew() generated
// ("cv::DetectionROI::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_defaultNew_const() -> *mut c_void;
// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:350
// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propScale_const(instance: *const c_void) -> f64;
// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:350
// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_DetectionROI_propScale_const_double(instance: *mut c_void, val: f64);
// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:352
// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propLocations_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:352
// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
pub fn cv_DetectionROI_propLocations_const_vectorLPointG(instance: *mut c_void, val: *const c_void);
// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:354
// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
pub fn cv_DetectionROI_propConfidences_const(instance: *const c_void) -> *mut c_void;
// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:354
// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
pub fn cv_DetectionROI_propConfidences_const_vectorLdoubleG(instance: *mut c_void, val: *const c_void);
// cv::DetectionROI::delete() generated
// ("cv::DetectionROI::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DetectionROI_delete(instance: *mut c_void);
// HOGDescriptor(Size, Size, Size, Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Enum, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:398
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "cv::HOGDescriptor::HistogramNormType", "double", "bool", "int", "bool"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(_win_size: *const core::Size, _block_size: *const core::Size, _block_stride: *const core::Size, _cell_size: *const core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: crate::xobjdetect::HOGDescriptor_HistogramNormType, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::HOGDescriptor::HOGDescriptor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:398
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor(ocvrs_return: *mut Result<*mut c_void>);
// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:414
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:422
// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
pub fn cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:433
// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:437
// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_checkDetectorSize_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getWinSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:441
// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_getWinSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:448
// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["svmdetector"], ["const cv::_InputArray*"]), _)]),
pub fn cv_HOGDescriptor_setSVMDetector_const__InputArrayR(instance: *mut c_void, svmdetector: *const c_void, ocvrs_return: *mut Result<()>);
// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:453
// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
pub fn cv_HOGDescriptor_read_FileNodeR(instance: *mut c_void, fn_: *mut c_void, ocvrs_return: *mut Result<bool>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:459
// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, objname: *const c_char, ocvrs_return: *mut Result<()>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:465
// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_load_const_StringR_const_StringR(instance: *mut c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:465
// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:471
// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_save_const_const_StringR_const_StringR(instance: *const c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:471
// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_HOGDescriptor_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:476
// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
pub fn cv_HOGDescriptor_copyTo_const_HOGDescriptorR(instance: *const c_void, c: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:485
// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, descriptors: *mut c_void, win_stride: *const core::Size, padding: *const core::Size, locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:485
// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
pub fn cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(instance: *const c_void, img: *const c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:501
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:501
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, weights: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:517
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:517
// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:537
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, found_weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:537
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, found_weights: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:556
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:556
// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// computeGradient(InputArray, InputOutputArray, InputOutputArray, Size, Size)(InputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:568
// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size"]), _)]),
pub fn cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(instance: *const c_void, img: *const c_void, grad: *const c_void, angle_ofs: *const c_void, padding_tl: *const core::Size, padding_br: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::computeGradient(InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:568
// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, img: *const c_void, grad: *const c_void, angle_ofs: *const c_void, ocvrs_return: *mut Result<()>);
// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:573
// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_getDefaultPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:579
// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_getDaimlerPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
// detectROI(InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:637
// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
pub fn cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(instance: *const c_void, img: *const c_void, locations: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:637
// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(instance: *const c_void, img: *const c_void, locations: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScaleROI(InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:650
// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR_double_int(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, locations: *mut c_void, hit_threshold: f64, group_threshold: i32, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::detectMultiScaleROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:650
// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
pub fn cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, locations: *mut c_void, ocvrs_return: *mut Result<()>);
// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:662
// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
pub fn cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(instance: *const c_void, rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result<()>);
// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:582
// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propWinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:582
// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propWinSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:585
// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propBlockSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:585
// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propBlockSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:588
// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propBlockStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:588
// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propBlockStride_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:591
// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propCellSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:591
// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_HOGDescriptor_propCellSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:594
// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propNbins_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:594
// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propNbins_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:597
// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propDerivAperture_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:597
// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propDerivAperture_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:600
// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propWinSigma_const(instance: *const c_void) -> f64;
// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:600
// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_HOGDescriptor_propWinSigma_const_double(instance: *mut c_void, val: f64);
// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:603
// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propHistogramNormType_const(instance: *const c_void, ocvrs_return: *mut crate::xobjdetect::HOGDescriptor_HistogramNormType);
// cv::HOGDescriptor::setHistogramNormType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:603
// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const cv::HOGDescriptor::HistogramNormType"]), _)]),
pub fn cv_HOGDescriptor_propHistogramNormType_const_HistogramNormType(instance: *mut c_void, val: crate::xobjdetect::HOGDescriptor_HistogramNormType);
// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:606
// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propL2HysThreshold_const(instance: *const c_void) -> f64;
// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:606
// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_HOGDescriptor_propL2HysThreshold_const_double(instance: *mut c_void, val: f64);
// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:609
// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propGammaCorrection_const(instance: *const c_void) -> bool;
// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:609
// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_HOGDescriptor_propGammaCorrection_const_bool(instance: *mut c_void, val: bool);
// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:612
// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propSvmDetector_const(instance: *const c_void) -> *mut c_void;
// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:612
// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
pub fn cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(instance: *mut c_void, val: *const c_void);
// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:615
// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propOclSvmDetector_const(instance: *const c_void) -> *mut c_void;
// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:615
// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
pub fn cv_HOGDescriptor_propOclSvmDetector_const_UMat(instance: *mut c_void, val: *const c_void);
// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:618
// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propFree_coef_const(instance: *const c_void) -> f32;
// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:618
// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_HOGDescriptor_propFree_coef_const_float(instance: *mut c_void, val: f32);
// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:621
// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propNlevels_const(instance: *const c_void) -> i32;
// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:621
// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_HOGDescriptor_propNlevels_const_int(instance: *mut c_void, val: i32);
// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:624
// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
pub fn cv_HOGDescriptor_propSignedGradient_const(instance: *const c_void) -> bool;
// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:624
// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_HOGDescriptor_propSignedGradient_const_bool(instance: *mut c_void, val: bool);
// cv::HOGDescriptor::delete() generated
// ("cv::HOGDescriptor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_HOGDescriptor_delete(instance: *mut c_void);
// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:125
// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
pub fn cv_SimilarRects_SimilarRects_double(_eps: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:126
// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
pub fn cv_SimilarRects_operator___const_const_RectR_const_RectR(instance: *const c_void, r1: *const core::Rect, r2: *const core::Rect, ocvrs_return: *mut Result<bool>);
// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:134
// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
pub fn cv_SimilarRects_propEps_const(instance: *const c_void) -> f64;
// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:134
// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_SimilarRects_propEps_const_double(instance: *mut c_void, val: f64);
// cv::SimilarRects::delete() generated
// ("cv::SimilarRects::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SimilarRects_delete(instance: *mut c_void);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:679
// ("cv::xobjdetect::WBDetector::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_xobjdetect_WBDetector_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:684
// ("cv::xobjdetect::WBDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_xobjdetect_WBDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// train(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:690
// ("cv::xobjdetect::WBDetector::train", vec![(pred!(mut, ["pos_samples", "neg_imgs"], ["const std::string*", "const std::string*"]), _)]),
pub fn cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(instance: *mut c_void, pos_samples: *const c_char, neg_imgs: *const c_char, ocvrs_return: *mut Result<()>);
// detect(const Mat &, std::vector<Rect> &, std::vector<double> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:699
// ("cv::xobjdetect::WBDetector::detect", vec![(pred!(mut, ["img", "bboxes", "confidences"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(instance: *mut c_void, img: *const c_void, bboxes: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:706
// ("cv::xobjdetect::WBDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xobjdetect_WBDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xobjdetect::WBDetector::delete() generated
// ("cv::xobjdetect::WBDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xobjdetect_WBDetector_delete(instance: *mut c_void);
