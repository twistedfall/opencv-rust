// isEmpty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:117
// ("cv::dpm::DPMDetector::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_isEmpty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// detect(cv::Mat &, std::vector<ObjectDetection> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:124
// ("cv::dpm::DPMDetector::detect", vec![(pred!(mut, ["image", "objects"], ["cv::Mat*", "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"]), _)]),
pub fn cv_dpm_DPMDetector_detect_MatR_vectorLObjectDetectionGR(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// getClassNames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:129
// ("cv::dpm::DPMDetector::getClassNames", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_getClassNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getClassCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:133
// ("cv::dpm::DPMDetector::getClassCount", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_getClassCount_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// create(const std::vector<std::string> &, const std::vector<std::string> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:142
// ("cv::dpm::DPMDetector::create", vec![(pred!(mut, ["filenames", "classNames"], ["const std::vector<std::string>*", "const std::vector<std::string>*"]), _)]),
pub fn cv_dpm_DPMDetector_create_const_vectorLstringGR_const_vectorLstringGR(filenames: *const c_void, class_names: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dpm::DPMDetector::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:142
// ("cv::dpm::DPMDetector::create", vec![(pred!(mut, ["filenames"], ["const std::vector<std::string>*"]), _)]),
pub fn cv_dpm_DPMDetector_create_const_vectorLstringGR(filenames: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dpm::DPMDetector::delete() generated
// ("cv::dpm::DPMDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dpm_DPMDetector_delete(instance: *mut c_void);
// ObjectDetection()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:110
// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, [], []), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_ObjectDetection(ocvrs_return: *mut Result<*mut c_void>);
// ObjectDetection(const Rect &, float, int)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:111
// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, ["rect", "score", "classID"], ["const cv::Rect*", "float", "int"]), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(rect: *const core::Rect, score: f32, class_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dpm::DPMDetector::ObjectDetection::ObjectDetection(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:111
// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, ["rect", "score"], ["const cv::Rect*", "float"]), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float(rect: *const core::Rect, score: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dpm::DPMDetector::ObjectDetection::rect() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:112
// ("cv::dpm::DPMDetector::ObjectDetection::rect", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propRect_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::dpm::DPMDetector::ObjectDetection::setRect(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:112
// ("cv::dpm::DPMDetector::ObjectDetection::setRect", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propRect_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::dpm::DPMDetector::ObjectDetection::score() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:113
// ("cv::dpm::DPMDetector::ObjectDetection::score", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propScore_const(instance: *const c_void) -> f32;
// cv::dpm::DPMDetector::ObjectDetection::setScore(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:113
// ("cv::dpm::DPMDetector::ObjectDetection::setScore", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propScore_const_float(instance: *mut c_void, val: f32);
// cv::dpm::DPMDetector::ObjectDetection::classID() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:114
// ("cv::dpm::DPMDetector::ObjectDetection::classID", vec![(pred!(const, [], []), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propClassID_const(instance: *const c_void) -> i32;
// cv::dpm::DPMDetector::ObjectDetection::setClassID(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dpm.hpp:114
// ("cv::dpm::DPMDetector::ObjectDetection::setClassID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_propClassID_const_int(instance: *mut c_void, val: i32);
// cv::dpm::DPMDetector::ObjectDetection::delete() generated
// ("cv::dpm::DPMDetector::ObjectDetection::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dpm_DPMDetector_ObjectDetection_delete(instance: *mut c_void);
