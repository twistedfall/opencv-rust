// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:66
// ("cv::xobjdetect::WBDetector::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_xobjdetect_WBDetector_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:71
// ("cv::xobjdetect::WBDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_xobjdetect_WBDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// train(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:77
// ("cv::xobjdetect::WBDetector::train", vec![(pred!(mut, ["pos_samples", "neg_imgs"], ["const std::string*", "const std::string*"]), _)]),
pub fn cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(instance: *mut c_void, pos_samples: *const c_char, neg_imgs: *const c_char, ocvrs_return: *mut Result<()>);
// detect(const Mat &, std::vector<Rect> &, std::vector<double> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:86
// ("cv::xobjdetect::WBDetector::detect", vec![(pred!(mut, ["img", "bboxes", "confidences"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(instance: *mut c_void, img: *const c_void, bboxes: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:93
// ("cv::xobjdetect::WBDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_xobjdetect_WBDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::xobjdetect::WBDetector::delete() generated
// ("cv::xobjdetect::WBDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_xobjdetect_WBDetector_delete(instance: *mut c_void);
