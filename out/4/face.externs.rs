// createFacemarkAAM()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark.hpp:79
// ("cv::face::createFacemarkAAM", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_createFacemarkAAM(ocvrs_return: *mut Result<*mut c_void>);
// createFacemarkKazemi()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark.hpp:85
// ("cv::face::createFacemarkKazemi", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_createFacemarkKazemi(ocvrs_return: *mut Result<*mut c_void>);
// createFacemarkLBF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark.hpp:82
// ("cv::face::createFacemarkLBF", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_createFacemarkLBF(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::drawFacemarks(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:226
// ("cv::face::drawFacemarks", vec![(pred!(mut, ["image", "points"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// drawFacemarks(InputOutputArray, InputArray, Scalar)(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:226
// ("cv::face::drawFacemarks", vec![(pred!(mut, ["image", "points", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(image: *const c_void, points: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// getFacesHAAR(InputArray, OutputArray, const String &)(InputArray, OutputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:70
// ("cv::face::getFacesHAAR", vec![(pred!(mut, ["image", "faces", "face_cascade_name"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::String*"]), _)]),
pub fn cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(image: *const c_void, faces: *const c_void, face_cascade_name: *const c_char, ocvrs_return: *mut Result<bool>);
// getFaces(InputArray, OutputArray, CParams *)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:68
// ("cv::face::getFaces", vec![(pred!(mut, ["image", "faces", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::face::CParams*"]), _)]),
pub fn cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(image: *const c_void, faces: *const c_void, params: *mut c_void, ocvrs_return: *mut Result<bool>);
// loadDatasetList(String, String, std::vector<String> &, std::vector<String> &)(InString, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:87
// ("cv::face::loadDatasetList", vec![(pred!(mut, ["imageList", "annotationList", "images", "annotations"], ["cv::String", "cv::String", "std::vector<cv::String>*", "std::vector<cv::String>*"]), _)]),
pub fn cv_face_loadDatasetList_String_String_vectorLStringGR_vectorLStringGR(image_list: *const c_char, annotation_list: *const c_char, images: *mut c_void, annotations: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::face::loadFacePoints(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:206
// ("cv::face::loadFacePoints", vec![(pred!(mut, ["filename", "points"], ["cv::String", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_loadFacePoints_String_const__OutputArrayR(filename: *const c_char, points: *const c_void, ocvrs_return: *mut Result<bool>);
// loadFacePoints(String, OutputArray, float)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:206
// ("cv::face::loadFacePoints", vec![(pred!(mut, ["filename", "points", "offset"], ["cv::String", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_face_loadFacePoints_String_const__OutputArrayR_float(filename: *const c_char, points: *const c_void, offset: f32, ocvrs_return: *mut Result<bool>);
// cv::face::loadTrainingData(InString, InString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:157
// ("cv::face::loadTrainingData", vec![(pred!(mut, ["imageList", "groundTruth", "images", "facePoints"], ["cv::String", "cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR(image_list: *const c_char, ground_truth: *const c_char, images: *mut c_void, face_points: *const c_void, ocvrs_return: *mut Result<bool>);
// loadTrainingData(String, String, std::vector<String> &, OutputArray, float)(InString, InString, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:157
// ("cv::face::loadTrainingData", vec![(pred!(mut, ["imageList", "groundTruth", "images", "facePoints", "offset"], ["cv::String", "cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*", "float"]), _)]),
pub fn cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR_float(image_list: *const c_char, ground_truth: *const c_char, images: *mut c_void, face_points: *const c_void, offset: f32, ocvrs_return: *mut Result<bool>);
// cv::face::loadTrainingData(InString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:117
// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "images", "facePoints"], ["cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR(filename: *const c_char, images: *mut c_void, face_points: *const c_void, ocvrs_return: *mut Result<bool>);
// loadTrainingData(String, std::vector<String> &, OutputArray, char, float)(InString, CppPassByVoidPtr, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:117
// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "images", "facePoints", "delim", "offset"], ["cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*", "char", "float"]), _)]),
pub fn cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR_char_float(filename: *const c_char, images: *mut c_void, face_points: *const c_void, delim: c_char, offset: f32, ocvrs_return: *mut Result<bool>);
// loadTrainingData(std::vector<String>, std::vector<std::vector<Point2f>> &, std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:178
// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "trainlandmarks", "trainimages"], ["std::vector<cv::String>", "std::vector<std::vector<cv::Point2f>>*", "std::vector<cv::String>*"]), _)]),
pub fn cv_face_loadTrainingData_vectorLStringG_vectorLvectorLPoint2fGGR_vectorLStringGR(filename: *mut c_void, trainlandmarks: *mut c_void, trainimages: *mut c_void, ocvrs_return: *mut Result<bool>);
// getNumBands()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/bif.hpp:60
// ("cv::face::BIF::getNumBands", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BIF_getNumBands_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNumRotations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/bif.hpp:63
// ("cv::face::BIF::getNumRotations", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BIF_getNumRotations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/bif.hpp:69
// ("cv::face::BIF::compute", vec![(pred!(const, ["image", "features"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, features: *const c_void, ocvrs_return: *mut Result<()>);
// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/bif.hpp:77
// ("cv::face::BIF::create", vec![(pred!(mut, ["num_bands", "num_rotations"], ["int", "int"]), _)]),
pub fn cv_face_BIF_create_int_int(num_bands: i32, num_rotations: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::BIF::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/bif.hpp:77
// ("cv::face::BIF::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BIF_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::BIF::to_Algorithm() generated
// ("cv::face::BIF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BIF_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::BIF::delete() generated
// ("cv::face::BIF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BIF_delete(instance: *mut c_void);
// getNumComponents()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:24
// ("cv::face::BasicFaceRecognizer::getNumComponents", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getNumComponents_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumComponents(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:26
// ("cv::face::BasicFaceRecognizer::setNumComponents", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_face_BasicFaceRecognizer_setNumComponents_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:28
// ("cv::face::BasicFaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:30
// ("cv::face::BasicFaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_face_BasicFaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getProjections()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:31
// ("cv::face::BasicFaceRecognizer::getProjections", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getProjections_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLabels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:32
// ("cv::face::BasicFaceRecognizer::getLabels", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getEigenValues()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:33
// ("cv::face::BasicFaceRecognizer::getEigenValues", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getEigenValues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getEigenVectors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:34
// ("cv::face::BasicFaceRecognizer::getEigenVectors", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getEigenVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMean()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:35
// ("cv::face::BasicFaceRecognizer::getMean", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_getMean_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:37
// ("cv::face::BasicFaceRecognizer::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_face_BasicFaceRecognizer_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:38
// ("cv::face::BasicFaceRecognizer::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_face_BasicFaceRecognizer_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:39
// ("cv::face::BasicFaceRecognizer::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::face::BasicFaceRecognizer::to_EigenFaceRecognizer() generated
// ("cv::face::BasicFaceRecognizer::to_EigenFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_to_EigenFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::BasicFaceRecognizer::to_FisherFaceRecognizer() generated
// ("cv::face::BasicFaceRecognizer::to_FisherFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_to_FisherFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::BasicFaceRecognizer::to_Algorithm() generated
// ("cv::face::BasicFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::BasicFaceRecognizer::to_FaceRecognizer() generated
// ("cv::face::BasicFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_to_FaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::BasicFaceRecognizer::delete() generated
// ("cv::face::BasicFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_BasicFaceRecognizer_delete(instance: *mut c_void);
// CParams(String, double, int, Size, Size)(InString, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:36
// ("cv::face::CParams::CParams", vec![(pred!(mut, ["cascade_model", "sf", "minN", "minSz", "maxSz"], ["cv::String", "double", "int", "cv::Size", "cv::Size"]), _)]),
pub fn cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model: *const c_char, sf: f64, min_n: i32, min_sz: *const core::Size, max_sz: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::CParams::CParams(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:36
// ("cv::face::CParams::CParams", vec![(pred!(mut, ["cascade_model"], ["cv::String"]), _)]),
pub fn cv_face_CParams_CParams_String(cascade_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::CParams::cascade() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:30
// ("cv::face::CParams::cascade", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propCascade_const(instance: *const c_void) -> *mut c_void;
// cv::face::CParams::setCascade(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:30
// ("cv::face::CParams::setCascade", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_face_CParams_propCascade_const_String(instance: *mut c_void, val: *const c_char);
// cv::face::CParams::scaleFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:31
// ("cv::face::CParams::scaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propScaleFactor_const(instance: *const c_void) -> f64;
// cv::face::CParams::setScaleFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:31
// ("cv::face::CParams::setScaleFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_face_CParams_propScaleFactor_const_double(instance: *mut c_void, val: f64);
// cv::face::CParams::minNeighbors() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:32
// ("cv::face::CParams::minNeighbors", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propMinNeighbors_const(instance: *const c_void) -> i32;
// cv::face::CParams::setMinNeighbors(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:32
// ("cv::face::CParams::setMinNeighbors", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_CParams_propMinNeighbors_const_int(instance: *mut c_void, val: i32);
// cv::face::CParams::minSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:33
// ("cv::face::CParams::minSize", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propMinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::face::CParams::setMinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:33
// ("cv::face::CParams::setMinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_face_CParams_propMinSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::face::CParams::maxSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:34
// ("cv::face::CParams::maxSize", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propMaxSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::face::CParams::setMaxSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:34
// ("cv::face::CParams::setMaxSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_face_CParams_propMaxSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::face::CParams::face_cascade() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:44
// ("cv::face::CParams::face_cascade", vec![(pred!(const, [], []), _)]),
pub fn cv_face_CParams_propFace_cascade_const(instance: *const c_void) -> *mut c_void;
// cv::face::CParams::setFace_cascade(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:44
// ("cv::face::CParams::setFace_cascade", vec![(pred!(mut, ["val"], ["const cv::CascadeClassifier"]), _)]),
pub fn cv_face_CParams_propFace_cascade_const_CascadeClassifier(instance: *mut c_void, val: *const c_void);
// cv::face::CParams::delete() generated
// ("cv::face::CParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_CParams_delete(instance: *mut c_void);
// create(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:86
// ("cv::face::EigenFaceRecognizer::create", vec![(pred!(mut, ["num_components", "threshold"], ["int", "double"]), _)]),
pub fn cv_face_EigenFaceRecognizer_create_int_double(num_components: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::EigenFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:86
// ("cv::face::EigenFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_EigenFaceRecognizer_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::EigenFaceRecognizer::to_Algorithm() generated
// ("cv::face::EigenFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_EigenFaceRecognizer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::EigenFaceRecognizer::to_BasicFaceRecognizer() generated
// ("cv::face::EigenFaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_EigenFaceRecognizer_to_BasicFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::EigenFaceRecognizer::to_FaceRecognizer() generated
// ("cv::face::EigenFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_EigenFaceRecognizer_to_FaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::EigenFaceRecognizer::delete() generated
// ("cv::face::EigenFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_EigenFaceRecognizer_delete(instance: *mut c_void);
// train(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:209
// ("cv::face::FaceRecognizer::train", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<()>);
// update(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:258
// ("cv::face::FaceRecognizer::update", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<()>);
// predict(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:261
// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR(instance: *const c_void, src: *const c_void, ocvrs_return: *mut Result<i32>);
// predict(InputArray, int &, double &)(InputArray, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:299
// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src", "label", "confidence"], ["const cv::_InputArray*", "int*", "double*"]), _)]),
pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(instance: *const c_void, src: *const c_void, label: *mut i32, confidence: *mut f64, ocvrs_return: *mut Result<()>);
// predict(InputArray, Ptr<PredictCollector>)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:309
// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src", "collector"], ["const cv::_InputArray*", "cv::Ptr<cv::face::PredictCollector>"]), _)]),
pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR_PtrLPredictCollectorG(instance: *const c_void, src: *const c_void, collector: *mut c_void, ocvrs_return: *mut Result<()>);
// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:323
// ("cv::face::FaceRecognizer::write", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_face_FaceRecognizer_write_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:332
// ("cv::face::FaceRecognizer::read", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_face_FaceRecognizer_read_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:338
// ("cv::face::FaceRecognizer::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_face_FaceRecognizer_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:341
// ("cv::face::FaceRecognizer::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_face_FaceRecognizer_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:344
// ("cv::face::FaceRecognizer::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FaceRecognizer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setLabelInfo(int, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:350
// ("cv::face::FaceRecognizer::setLabelInfo", vec![(pred!(mut, ["label", "strInfo"], ["int", "const cv::String*"]), _)]),
pub fn cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(instance: *mut c_void, label: i32, str_info: *const c_char, ocvrs_return: *mut Result<()>);
// getLabelInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:357
// ("cv::face::FaceRecognizer::getLabelInfo", vec![(pred!(const, ["label"], ["int"]), _)]),
pub fn cv_face_FaceRecognizer_getLabelInfo_const_int(instance: *const c_void, label: i32, ocvrs_return: *mut Result<*mut c_void>);
// getLabelsByString(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:364
// ("cv::face::FaceRecognizer::getLabelsByString", vec![(pred!(const, ["str"], ["const cv::String*"]), _)]),
pub fn cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(instance: *const c_void, str: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:366
// ("cv::face::FaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face.hpp:368
// ("cv::face::FaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_face_FaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::face::FaceRecognizer::to_BasicFaceRecognizer() generated
// ("cv::face::FaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_to_BasicFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FaceRecognizer::to_EigenFaceRecognizer() generated
// ("cv::face::FaceRecognizer::to_EigenFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_to_EigenFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FaceRecognizer::to_FisherFaceRecognizer() generated
// ("cv::face::FaceRecognizer::to_FisherFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_to_FisherFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FaceRecognizer::to_LBPHFaceRecognizer() generated
// ("cv::face::FaceRecognizer::to_LBPHFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_to_LBPHFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FaceRecognizer::to_Algorithm() generated
// ("cv::face::FaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FaceRecognizer::delete() generated
// ("cv::face::FaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FaceRecognizer_delete(instance: *mut c_void);
// loadModel(String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark.hpp:55
// ("cv::face::Facemark::loadModel", vec![(pred!(mut, ["model"], ["cv::String"]), _)]),
pub fn cv_face_Facemark_loadModel_String(instance: *mut c_void, model: *const c_char, ocvrs_return: *mut Result<()>);
// fit(InputArray, InputArray, OutputArrayOfArrays)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark.hpp:72
// ("cv::face::Facemark::fit", vec![(pred!(mut, ["image", "faces", "landmarks"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, landmarks: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::face::Facemark::to_FacemarkAAM() generated
// ("cv::face::Facemark::to_FacemarkAAM", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_to_FacemarkAAM(instance: *mut c_void) -> *mut c_void;
// cv::face::Facemark::to_FacemarkKazemi() generated
// ("cv::face::Facemark::to_FacemarkKazemi", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_to_FacemarkKazemi(instance: *mut c_void) -> *mut c_void;
// cv::face::Facemark::to_FacemarkLBF() generated
// ("cv::face::Facemark::to_FacemarkLBF", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_to_FacemarkLBF(instance: *mut c_void) -> *mut c_void;
// cv::face::Facemark::to_FacemarkTrain() generated
// ("cv::face::Facemark::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_to_FacemarkTrain(instance: *mut c_void) -> *mut c_void;
// cv::face::Facemark::to_Algorithm() generated
// ("cv::face::Facemark::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::Facemark::delete() generated
// ("cv::face::Facemark::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_Facemark_delete(instance: *mut c_void);
// fitConfig(InputArray, InputArray, OutputArrayOfArrays, const std::vector<Config> &)(InputArray, InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:149
// ("cv::face::FacemarkAAM::fitConfig", vec![(pred!(mut, ["image", "roi", "_landmarks", "runtime_params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<cv::face::FacemarkAAM::Config>*"]), _)]),
pub fn cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vectorLConfigGR(instance: *mut c_void, image: *const c_void, roi: *const c_void, _landmarks: *const c_void, runtime_params: *const c_void, ocvrs_return: *mut Result<bool>);
// create(const FacemarkAAM::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:153
// ("cv::face::FacemarkAAM::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkAAM::Params*"]), _)]),
pub fn cv_face_FacemarkAAM_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkAAM::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:153
// ("cv::face::FacemarkAAM::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkAAM::to_Algorithm() generated
// ("cv::face::FacemarkAAM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkAAM::to_Facemark() generated
// ("cv::face::FacemarkAAM::to_Facemark", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_to_Facemark(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkAAM::to_FacemarkTrain() generated
// ("cv::face::FacemarkAAM::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_to_FacemarkTrain(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkAAM::delete() generated
// ("cv::face::FacemarkAAM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_delete(instance: *mut c_void);
// Config(Mat, Point2f, float, int)(TraitClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:82
// ("cv::face::FacemarkAAM::Config::Config", vec![(pred!(mut, ["rot", "trans", "scaling", "scale_id"], ["cv::Mat", "cv::Point2f", "float", "int"]), _)]),
pub fn cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot: *mut c_void, trans: *const core::Point2f, scaling: f32, scale_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkAAM::Config::Config() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:82
// ("cv::face::FacemarkAAM::Config::Config", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_Config(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkAAM::Config::R() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:88
// ("cv::face::FacemarkAAM::Config::R", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_propR_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Config::setR(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:88
// ("cv::face::FacemarkAAM::Config::setR", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Config_propR_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Config::t() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:89
// ("cv::face::FacemarkAAM::Config::t", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_propT_const(instance: *const c_void, ocvrs_return: *mut core::Point2f);
// cv::face::FacemarkAAM::Config::setT(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:89
// ("cv::face::FacemarkAAM::Config::setT", vec![(pred!(mut, ["val"], ["const cv::Point2f"]), _)]),
pub fn cv_face_FacemarkAAM_Config_propT_const_Point2f(instance: *mut c_void, val: *const core::Point2f);
// cv::face::FacemarkAAM::Config::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:90
// ("cv::face::FacemarkAAM::Config::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_propScale_const(instance: *const c_void) -> f32;
// cv::face::FacemarkAAM::Config::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:90
// ("cv::face::FacemarkAAM::Config::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_face_FacemarkAAM_Config_propScale_const_float(instance: *mut c_void, val: f32);
// cv::face::FacemarkAAM::Config::model_scale_idx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:91
// ("cv::face::FacemarkAAM::Config::model_scale_idx", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_propModel_scale_idx_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Config::setModel_scale_idx(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:91
// ("cv::face::FacemarkAAM::Config::setModel_scale_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Config_propModel_scale_idx_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Config::delete() generated
// ("cv::face::FacemarkAAM::Config::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Config_delete(instance: *mut c_void);
// cv::face::FacemarkAAM::Data::defaultNew() generated
// ("cv::face::FacemarkAAM::Data::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Data_defaultNew_const() -> *mut c_void;
// cv::face::FacemarkAAM::Data::s0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:100
// ("cv::face::FacemarkAAM::Data::s0", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Data_propS0_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Data::setS0(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:100
// ("cv::face::FacemarkAAM::Data::setS0", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
pub fn cv_face_FacemarkAAM_Data_propS0_const_vectorLPoint2fG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Data::delete() generated
// ("cv::face::FacemarkAAM::Data::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Data_delete(instance: *mut c_void);
// cv::face::FacemarkAAM::Model::defaultNew() generated
// ("cv::face::FacemarkAAM::Model::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_defaultNew_const() -> *mut c_void;
// cv::face::FacemarkAAM::Model::scales() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:108
// ("cv::face::FacemarkAAM::Model::scales", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propScales_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:108
// ("cv::face::FacemarkAAM::Model::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propScales_const_vectorLfloatG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::triangles() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:112
// ("cv::face::FacemarkAAM::Model::triangles", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propTriangles_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setTriangles(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:112
// ("cv::face::FacemarkAAM::Model::setTriangles", vec![(pred!(mut, ["val"], ["const std::vector<cv::Vec3i>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propTriangles_const_vectorLVec3iG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::textures() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:137
// ("cv::face::FacemarkAAM::Model::textures", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propTextures_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setTextures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:137
// ("cv::face::FacemarkAAM::Model::setTextures", vec![(pred!(mut, ["val"], ["const std::vector<cv::face::FacemarkAAM::Model::Texture>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propTextures_const_vectorLTextureG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::s0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:141
// ("cv::face::FacemarkAAM::Model::s0", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propS0_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setS0(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:141
// ("cv::face::FacemarkAAM::Model::setS0", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propS0_const_vectorLPoint2fG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::S() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:143
// ("cv::face::FacemarkAAM::Model::S", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propS_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setS(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:143
// ("cv::face::FacemarkAAM::Model::setS", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propS_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Q() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:143
// ("cv::face::FacemarkAAM::Model::Q", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_propQ_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::setQ(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:143
// ("cv::face::FacemarkAAM::Model::setQ", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_propQ_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::delete() generated
// ("cv::face::FacemarkAAM::Model::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_delete(instance: *mut c_void);
// cv::face::FacemarkAAM::Model::Texture::defaultNew() generated
// ("cv::face::FacemarkAAM::Model::Texture::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_defaultNew_const() -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::max_m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:116
// ("cv::face::FacemarkAAM::Model::Texture::max_m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propMax_m_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Model::Texture::setMax_m(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:116
// ("cv::face::FacemarkAAM::Model::Texture::setMax_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propMax_m_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Model::Texture::resolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:117
// ("cv::face::FacemarkAAM::Model::Texture::resolution", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propResolution_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::face::FacemarkAAM::Model::Texture::setResolution(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:117
// ("cv::face::FacemarkAAM::Model::Texture::setResolution", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propResolution_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::face::FacemarkAAM::Model::Texture::A() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:119
// ("cv::face::FacemarkAAM::Model::Texture::A", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propA_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setA(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:119
// ("cv::face::FacemarkAAM::Model::Texture::setA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propA_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::A0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:121
// ("cv::face::FacemarkAAM::Model::Texture::A0", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propA0_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setA0(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:121
// ("cv::face::FacemarkAAM::Model::Texture::setA0", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propA0_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::AA() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:123
// ("cv::face::FacemarkAAM::Model::Texture::AA", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propAA_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setAA(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:123
// ("cv::face::FacemarkAAM::Model::Texture::setAA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propAA_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::AA0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:125
// ("cv::face::FacemarkAAM::Model::Texture::AA0", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propAA0_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setAA0(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:125
// ("cv::face::FacemarkAAM::Model::Texture::setAA0", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propAA0_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::textureIdx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:128
// ("cv::face::FacemarkAAM::Model::Texture::textureIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propTextureIdx_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setTextureIdx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:128
// ("cv::face::FacemarkAAM::Model::Texture::setTextureIdx", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Point>>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propTextureIdx_const_vectorLvectorLPointGG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::base_shape() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:130
// ("cv::face::FacemarkAAM::Model::Texture::base_shape", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propBase_shape_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setBase_shape(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:130
// ("cv::face::FacemarkAAM::Model::Texture::setBase_shape", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propBase_shape_const_vectorLPoint2fG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::ind1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:132
// ("cv::face::FacemarkAAM::Model::Texture::ind1", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propInd1_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setInd1(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:132
// ("cv::face::FacemarkAAM::Model::Texture::setInd1", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propInd1_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::ind2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:134
// ("cv::face::FacemarkAAM::Model::Texture::ind2", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propInd2_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Model::Texture::setInd2(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:134
// ("cv::face::FacemarkAAM::Model::Texture::setInd2", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_propInd2_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Model::Texture::delete() generated
// ("cv::face::FacemarkAAM::Model::Texture::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Model_Texture_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:55
// ("cv::face::FacemarkAAM::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:60
// ("cv::face::FacemarkAAM::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_face_FacemarkAAM_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:65
// ("cv::face::FacemarkAAM::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_face_FacemarkAAM_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::face::FacemarkAAM::Params::model_filename() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:67
// ("cv::face::FacemarkAAM::Params::model_filename", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propModel_filename_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Params::setModel_filename(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:67
// ("cv::face::FacemarkAAM::Params::setModel_filename", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propModel_filename_const_string(instance: *mut c_void, val: *const c_char);
// cv::face::FacemarkAAM::Params::m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:68
// ("cv::face::FacemarkAAM::Params::m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propM_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setM(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:68
// ("cv::face::FacemarkAAM::Params::setM", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propM_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::n() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:69
// ("cv::face::FacemarkAAM::Params::n", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propN_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setN(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:69
// ("cv::face::FacemarkAAM::Params::setN", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propN_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::n_iter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:70
// ("cv::face::FacemarkAAM::Params::n_iter", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propN_iter_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setN_iter(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:70
// ("cv::face::FacemarkAAM::Params::setN_iter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propN_iter_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::verbose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:71
// ("cv::face::FacemarkAAM::Params::verbose", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propVerbose_const(instance: *const c_void) -> bool;
// cv::face::FacemarkAAM::Params::setVerbose(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:71
// ("cv::face::FacemarkAAM::Params::setVerbose", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propVerbose_const_bool(instance: *mut c_void, val: bool);
// cv::face::FacemarkAAM::Params::save_model() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:72
// ("cv::face::FacemarkAAM::Params::save_model", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propSave_model_const(instance: *const c_void) -> bool;
// cv::face::FacemarkAAM::Params::setSave_model(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:72
// ("cv::face::FacemarkAAM::Params::setSave_model", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propSave_model_const_bool(instance: *mut c_void, val: bool);
// cv::face::FacemarkAAM::Params::max_m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::max_m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propMax_m_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setMax_m(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::setMax_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propMax_m_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::max_n() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::max_n", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propMax_n_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setMax_n(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::setMax_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propMax_n_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::texture_max_m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::texture_max_m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propTexture_max_m_const(instance: *const c_void) -> i32;
// cv::face::FacemarkAAM::Params::setTexture_max_m(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:73
// ("cv::face::FacemarkAAM::Params::setTexture_max_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propTexture_max_m_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkAAM::Params::scales() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:74
// ("cv::face::FacemarkAAM::Params::scales", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_propScales_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkAAM::Params::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkAAM.hpp:74
// ("cv::face::FacemarkAAM::Params::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
pub fn cv_face_FacemarkAAM_Params_propScales_const_vectorLfloatG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkAAM::Params::delete() generated
// ("cv::face::FacemarkAAM::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkAAM_Params_delete(instance: *mut c_void);
// create(const FacemarkKazemi::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:39
// ("cv::face::FacemarkKazemi::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkKazemi::Params*"]), _)]),
pub fn cv_face_FacemarkKazemi_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkKazemi::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:39
// ("cv::face::FacemarkKazemi::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_create(ocvrs_return: *mut Result<*mut c_void>);
// training(std::vector<Mat> &, std::vector<std::vector<Point2f>> &, std::string, Size, std::string)(CppPassByVoidPtr, CppPassByVoidPtr, InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:51
// ("cv::face::FacemarkKazemi::training", vec![(pred!(mut, ["images", "landmarks", "configfile", "scale", "modelFilename"], ["std::vector<cv::Mat>*", "std::vector<std::vector<cv::Point2f>>*", "std::string", "cv::Size", "std::string"]), _)]),
pub fn cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size_string(instance: *mut c_void, images: *mut c_void, landmarks: *mut c_void, configfile: *const c_char, scale: *const core::Size, model_filename: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::face::FacemarkKazemi::training(CppPassByVoidPtr, CppPassByVoidPtr, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:51
// ("cv::face::FacemarkKazemi::training", vec![(pred!(mut, ["images", "landmarks", "configfile", "scale"], ["std::vector<cv::Mat>*", "std::vector<std::vector<cv::Point2f>>*", "std::string", "cv::Size"]), _)]),
pub fn cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size(instance: *mut c_void, images: *mut c_void, landmarks: *mut c_void, configfile: *const c_char, scale: *const core::Size, ocvrs_return: *mut Result<bool>);
// setFaceDetector(bool (*)(InputArray, OutputArray, void *), void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:54
// ("cv::face::FacemarkKazemi::setFaceDetector", vec![(pred!(mut, ["f", "userData"], ["bool (*)(const cv::_InputArray&, const cv::_OutputArray&, void*)", "void*"]), _)]),
pub fn cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(instance: *mut c_void, f: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> bool>, user_data: *mut c_void, ocvrs_return: *mut Result<bool>);
// getFaces(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:56
// ("cv::face::FacemarkKazemi::getFaces", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::face::FacemarkKazemi::to_Algorithm() generated
// ("cv::face::FacemarkKazemi::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkKazemi::to_Facemark() generated
// ("cv::face::FacemarkKazemi::to_Facemark", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_to_Facemark(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkKazemi::delete() generated
// ("cv::face::FacemarkKazemi::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:19
// ("cv::face::FacemarkKazemi::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkKazemi::Params::cascade_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:21
// ("cv::face::FacemarkKazemi::Params::cascade_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propCascade_depth_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setCascade_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:21
// ("cv::face::FacemarkKazemi::Params::setCascade_depth", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propCascade_depth_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::tree_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:23
// ("cv::face::FacemarkKazemi::Params::tree_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propTree_depth_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setTree_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:23
// ("cv::face::FacemarkKazemi::Params::setTree_depth", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propTree_depth_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::num_trees_per_cascade_level() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:25
// ("cv::face::FacemarkKazemi::Params::num_trees_per_cascade_level", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setNum_trees_per_cascade_level(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:25
// ("cv::face::FacemarkKazemi::Params::setNum_trees_per_cascade_level", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::learning_rate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:27
// ("cv::face::FacemarkKazemi::Params::learning_rate", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propLearning_rate_const(instance: *const c_void) -> f32;
// cv::face::FacemarkKazemi::Params::setLearning_rate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:27
// ("cv::face::FacemarkKazemi::Params::setLearning_rate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propLearning_rate_const_float(instance: *mut c_void, val: f32);
// cv::face::FacemarkKazemi::Params::oversampling_amount() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:29
// ("cv::face::FacemarkKazemi::Params::oversampling_amount", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propOversampling_amount_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setOversampling_amount(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:29
// ("cv::face::FacemarkKazemi::Params::setOversampling_amount", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propOversampling_amount_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::num_test_coordinates() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:31
// ("cv::face::FacemarkKazemi::Params::num_test_coordinates", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_test_coordinates_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setNum_test_coordinates(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:31
// ("cv::face::FacemarkKazemi::Params::setNum_test_coordinates", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_test_coordinates_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::lambda() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:33
// ("cv::face::FacemarkKazemi::Params::lambda", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propLambda_const(instance: *const c_void) -> f32;
// cv::face::FacemarkKazemi::Params::setLambda(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:33
// ("cv::face::FacemarkKazemi::Params::setLambda", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propLambda_const_float(instance: *mut c_void, val: f32);
// cv::face::FacemarkKazemi::Params::num_test_splits() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:35
// ("cv::face::FacemarkKazemi::Params::num_test_splits", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_test_splits_const(instance: *const c_void) -> u32;
// cv::face::FacemarkKazemi::Params::setNum_test_splits(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:35
// ("cv::face::FacemarkKazemi::Params::setNum_test_splits", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propNum_test_splits_const_unsigned_long(instance: *mut c_void, val: u32);
// cv::face::FacemarkKazemi::Params::configfile() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:37
// ("cv::face::FacemarkKazemi::Params::configfile", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_propConfigfile_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkKazemi::Params::setConfigfile(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/face_alignment.hpp:37
// ("cv::face::FacemarkKazemi::Params::setConfigfile", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_face_FacemarkKazemi_Params_propConfigfile_const_String(instance: *mut c_void, val: *const c_char);
// cv::face::FacemarkKazemi::Params::delete() generated
// ("cv::face::FacemarkKazemi::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkKazemi_Params_delete(instance: *mut c_void);
// create(const FacemarkLBF::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:111
// ("cv::face::FacemarkLBF::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkLBF::Params*"]), _)]),
pub fn cv_face_FacemarkLBF_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkLBF::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:111
// ("cv::face::FacemarkLBF::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FacemarkLBF::to_Algorithm() generated
// ("cv::face::FacemarkLBF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkLBF::to_Facemark() generated
// ("cv::face::FacemarkLBF::to_Facemark", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_to_Facemark(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkLBF::to_FacemarkTrain() generated
// ("cv::face::FacemarkLBF::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_to_FacemarkTrain(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkLBF::delete() generated
// ("cv::face::FacemarkLBF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:56
// ("cv::face::FacemarkLBF::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:91
// ("cv::face::FacemarkLBF::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_face_FacemarkLBF_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:92
// ("cv::face::FacemarkLBF::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_face_FacemarkLBF_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::face::FacemarkLBF::Params::shape_offset() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:58
// ("cv::face::FacemarkLBF::Params::shape_offset", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propShape_offset_const(instance: *const c_void) -> f64;
// cv::face::FacemarkLBF::Params::setShape_offset(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:58
// ("cv::face::FacemarkLBF::Params::setShape_offset", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propShape_offset_const_double(instance: *mut c_void, val: f64);
// cv::face::FacemarkLBF::Params::cascade_face() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:60
// ("cv::face::FacemarkLBF::Params::cascade_face", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propCascade_face_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkLBF::Params::setCascade_face(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:60
// ("cv::face::FacemarkLBF::Params::setCascade_face", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propCascade_face_const_String(instance: *mut c_void, val: *const c_char);
// cv::face::FacemarkLBF::Params::verbose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:62
// ("cv::face::FacemarkLBF::Params::verbose", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propVerbose_const(instance: *const c_void) -> bool;
// cv::face::FacemarkLBF::Params::setVerbose(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:62
// ("cv::face::FacemarkLBF::Params::setVerbose", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propVerbose_const_bool(instance: *mut c_void, val: bool);
// cv::face::FacemarkLBF::Params::n_landmarks() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:65
// ("cv::face::FacemarkLBF::Params::n_landmarks", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propN_landmarks_const(instance: *const c_void) -> i32;
// cv::face::FacemarkLBF::Params::setN_landmarks(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:65
// ("cv::face::FacemarkLBF::Params::setN_landmarks", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propN_landmarks_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkLBF::Params::initShape_n() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:67
// ("cv::face::FacemarkLBF::Params::initShape_n", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propInitShape_n_const(instance: *const c_void) -> i32;
// cv::face::FacemarkLBF::Params::setInitShape_n(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:67
// ("cv::face::FacemarkLBF::Params::setInitShape_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propInitShape_n_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkLBF::Params::stages_n() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:70
// ("cv::face::FacemarkLBF::Params::stages_n", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propStages_n_const(instance: *const c_void) -> i32;
// cv::face::FacemarkLBF::Params::setStages_n(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:70
// ("cv::face::FacemarkLBF::Params::setStages_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propStages_n_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkLBF::Params::tree_n() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:72
// ("cv::face::FacemarkLBF::Params::tree_n", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propTree_n_const(instance: *const c_void) -> i32;
// cv::face::FacemarkLBF::Params::setTree_n(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:72
// ("cv::face::FacemarkLBF::Params::setTree_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propTree_n_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkLBF::Params::tree_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:74
// ("cv::face::FacemarkLBF::Params::tree_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propTree_depth_const(instance: *const c_void) -> i32;
// cv::face::FacemarkLBF::Params::setTree_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:74
// ("cv::face::FacemarkLBF::Params::setTree_depth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propTree_depth_const_int(instance: *mut c_void, val: i32);
// cv::face::FacemarkLBF::Params::bagging_overlap() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:76
// ("cv::face::FacemarkLBF::Params::bagging_overlap", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propBagging_overlap_const(instance: *const c_void) -> f64;
// cv::face::FacemarkLBF::Params::setBagging_overlap(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:76
// ("cv::face::FacemarkLBF::Params::setBagging_overlap", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propBagging_overlap_const_double(instance: *mut c_void, val: f64);
// cv::face::FacemarkLBF::Params::model_filename() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:79
// ("cv::face::FacemarkLBF::Params::model_filename", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propModel_filename_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkLBF::Params::setModel_filename(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:79
// ("cv::face::FacemarkLBF::Params::setModel_filename", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propModel_filename_const_string(instance: *mut c_void, val: *const c_char);
// cv::face::FacemarkLBF::Params::save_model() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:81
// ("cv::face::FacemarkLBF::Params::save_model", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propSave_model_const(instance: *const c_void) -> bool;
// cv::face::FacemarkLBF::Params::setSave_model(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:81
// ("cv::face::FacemarkLBF::Params::setSave_model", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propSave_model_const_bool(instance: *mut c_void, val: bool);
// cv::face::FacemarkLBF::Params::seed() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:82
// ("cv::face::FacemarkLBF::Params::seed", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propSeed_const(instance: *const c_void) -> u32;
// cv::face::FacemarkLBF::Params::setSeed(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:82
// ("cv::face::FacemarkLBF::Params::setSeed", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propSeed_const_unsigned_int(instance: *mut c_void, val: u32);
// cv::face::FacemarkLBF::Params::feats_m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:84
// ("cv::face::FacemarkLBF::Params::feats_m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propFeats_m_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkLBF::Params::setFeats_m(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:84
// ("cv::face::FacemarkLBF::Params::setFeats_m", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propFeats_m_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkLBF::Params::radius_m() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:85
// ("cv::face::FacemarkLBF::Params::radius_m", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propRadius_m_const(instance: *const c_void) -> *mut c_void;
// cv::face::FacemarkLBF::Params::setRadius_m(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:85
// ("cv::face::FacemarkLBF::Params::setRadius_m", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propRadius_m_const_vectorLdoubleG(instance: *mut c_void, val: *const c_void);
// cv::face::FacemarkLBF::Params::detectROI() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:89
// ("cv::face::FacemarkLBF::Params::detectROI", vec![(pred!(const, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_propDetectROI_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::face::FacemarkLBF::Params::setDetectROI(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemarkLBF.hpp:89
// ("cv::face::FacemarkLBF::Params::setDetectROI", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_face_FacemarkLBF_Params_propDetectROI_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::face::FacemarkLBF::Params::delete() generated
// ("cv::face::FacemarkLBF::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkLBF_Params_delete(instance: *mut c_void);
// addTrainingSample(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:302
// ("cv::face::FacemarkTrain::addTrainingSample", vec![(pred!(mut, ["image", "landmarks"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(instance: *mut c_void, image: *const c_void, landmarks: *const c_void, ocvrs_return: *mut Result<bool>);
// training(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:322
// ("cv::face::FacemarkTrain::training", vec![(pred!(mut, ["parameters"], ["void*"]), _)]),
pub fn cv_face_FacemarkTrain_training_voidX(instance: *mut c_void, parameters: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::face::FacemarkTrain::training() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:322
// ("cv::face::FacemarkTrain::training", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_training(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setFaceDetector(FN_FaceDetector, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:345
// ("cv::face::FacemarkTrain::setFaceDetector", vec![(pred!(mut, ["detector", "userData"], ["cv::face::FN_FaceDetector", "void*"]), _)]),
pub fn cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(instance: *mut c_void, detector: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> bool>, user_data: *mut c_void, ocvrs_return: *mut Result<bool>);
// getFaces(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:362
// ("cv::face::FacemarkTrain::getFaces", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<bool>);
// getData(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:380
// ("cv::face::FacemarkTrain::getData", vec![(pred!(mut, ["items"], ["void*"]), _)]),
pub fn cv_face_FacemarkTrain_getData_voidX(instance: *mut c_void, items: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::face::FacemarkTrain::getData() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facemark_train.hpp:380
// ("cv::face::FacemarkTrain::getData", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_getData(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::face::FacemarkTrain::to_FacemarkAAM() generated
// ("cv::face::FacemarkTrain::to_FacemarkAAM", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_to_FacemarkAAM(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkTrain::to_FacemarkLBF() generated
// ("cv::face::FacemarkTrain::to_FacemarkLBF", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_to_FacemarkLBF(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkTrain::to_Algorithm() generated
// ("cv::face::FacemarkTrain::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkTrain::to_Facemark() generated
// ("cv::face::FacemarkTrain::to_Facemark", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_to_Facemark(instance: *mut c_void) -> *mut c_void;
// cv::face::FacemarkTrain::delete() generated
// ("cv::face::FacemarkTrain::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FacemarkTrain_delete(instance: *mut c_void);
// create(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:122
// ("cv::face::FisherFaceRecognizer::create", vec![(pred!(mut, ["num_components", "threshold"], ["int", "double"]), _)]),
pub fn cv_face_FisherFaceRecognizer_create_int_double(num_components: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FisherFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:122
// ("cv::face::FisherFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FisherFaceRecognizer_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::FisherFaceRecognizer::to_Algorithm() generated
// ("cv::face::FisherFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FisherFaceRecognizer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::FisherFaceRecognizer::to_BasicFaceRecognizer() generated
// ("cv::face::FisherFaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FisherFaceRecognizer_to_BasicFaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FisherFaceRecognizer::to_FaceRecognizer() generated
// ("cv::face::FisherFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FisherFaceRecognizer_to_FaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::FisherFaceRecognizer::delete() generated
// ("cv::face::FisherFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_FisherFaceRecognizer_delete(instance: *mut c_void);
// getGridX()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:130
// ("cv::face::LBPHFaceRecognizer::getGridX", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getGridX_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGridX(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:132
// ("cv::face::LBPHFaceRecognizer::setGridX", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_setGridX_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getGridY()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:134
// ("cv::face::LBPHFaceRecognizer::getGridY", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getGridY_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setGridY(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:136
// ("cv::face::LBPHFaceRecognizer::setGridY", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_setGridY_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:138
// ("cv::face::LBPHFaceRecognizer::getRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:140
// ("cv::face::LBPHFaceRecognizer::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getNeighbors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:142
// ("cv::face::LBPHFaceRecognizer::getNeighbors", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getNeighbors_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:144
// ("cv::face::LBPHFaceRecognizer::setNeighbors", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_setNeighbors_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:146
// ("cv::face::LBPHFaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:148
// ("cv::face::LBPHFaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getHistograms()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:149
// ("cv::face::LBPHFaceRecognizer::getHistograms", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getHistograms_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLabels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:150
// ("cv::face::LBPHFaceRecognizer::getLabels", vec![(pred!(const, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_getLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:184
// ("cv::face::LBPHFaceRecognizer::create", vec![(pred!(mut, ["radius", "neighbors", "grid_x", "grid_y", "threshold"], ["int", "int", "int", "int", "double"]), _)]),
pub fn cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::LBPHFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/facerec.hpp:184
// ("cv::face::LBPHFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::LBPHFaceRecognizer::to_Algorithm() generated
// ("cv::face::LBPHFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::LBPHFaceRecognizer::to_FaceRecognizer() generated
// ("cv::face::LBPHFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_to_FaceRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::face::LBPHFaceRecognizer::delete() generated
// ("cv::face::LBPHFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_LBPHFaceRecognizer_delete(instance: *mut c_void);
// salt(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:78
// ("cv::face::MACE::salt", vec![(pred!(mut, ["passphrase"], ["const cv::String*"]), _)]),
pub fn cv_face_MACE_salt_const_StringR(instance: *mut c_void, passphrase: *const c_char, ocvrs_return: *mut Result<()>);
// train(cv::InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:86
// ("cv::face::MACE::train", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
pub fn cv_face_MACE_train_const__InputArrayR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result<()>);
// same(cv::InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:92
// ("cv::face::MACE::same", vec![(pred!(const, ["query"], ["const cv::_InputArray*"]), _)]),
pub fn cv_face_MACE_same_const_const__InputArrayR(instance: *const c_void, query: *const c_void, ocvrs_return: *mut Result<bool>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:100
// ("cv::face::MACE::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_face_MACE_load_const_StringR_const_StringR(filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::MACE::load(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:100
// ("cv::face::MACE::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_face_MACE_load_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:106
// ("cv::face::MACE::create", vec![(pred!(mut, ["IMGSIZE"], ["int"]), _)]),
pub fn cv_face_MACE_create_int(imgsize: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::MACE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/mace.hpp:106
// ("cv::face::MACE::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_MACE_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::MACE::to_Algorithm() generated
// ("cv::face::MACE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_MACE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::face::MACE::delete() generated
// ("cv::face::MACE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_MACE_delete(instance: *mut c_void);
// init(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:69
// ("cv::face::PredictCollector::init", vec![(pred!(mut, ["size"], ["size_t"]), _)]),
pub fn cv_face_PredictCollector_init_size_t(instance: *mut c_void, size: size_t, ocvrs_return: *mut Result<()>);
// collect(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:75
// ("cv::face::PredictCollector::collect", vec![(pred!(mut, ["label", "dist"], ["int", "double"]), _)]),
pub fn cv_face_PredictCollector_collect_int_double(instance: *mut c_void, label: i32, dist: f64, ocvrs_return: *mut Result<bool>);
// cv::face::PredictCollector::to_StandardCollector() generated
// ("cv::face::PredictCollector::to_StandardCollector", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_PredictCollector_to_StandardCollector(instance: *mut c_void) -> *mut c_void;
// cv::face::PredictCollector::delete() generated
// ("cv::face::PredictCollector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_PredictCollector_delete(instance: *mut c_void);
// StandardCollector(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:99
// ("cv::face::StandardCollector::StandardCollector", vec![(pred!(mut, ["threshold_"], ["double"]), _)]),
pub fn cv_face_StandardCollector_StandardCollector_double(threshold_: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::StandardCollector::StandardCollector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:99
// ("cv::face::StandardCollector::StandardCollector", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_StandardCollector_StandardCollector(ocvrs_return: *mut Result<*mut c_void>);
// init(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:101
// ("cv::face::StandardCollector::init", vec![(pred!(mut, ["size"], ["size_t"]), _)]),
pub fn cv_face_StandardCollector_init_size_t(instance: *mut c_void, size: size_t, ocvrs_return: *mut Result<()>);
// collect(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:103
// ("cv::face::StandardCollector::collect", vec![(pred!(mut, ["label", "dist"], ["int", "double"]), _)]),
pub fn cv_face_StandardCollector_collect_int_double(instance: *mut c_void, label: i32, dist: f64, ocvrs_return: *mut Result<bool>);
// getMinLabel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:105
// ("cv::face::StandardCollector::getMinLabel", vec![(pred!(const, [], []), _)]),
pub fn cv_face_StandardCollector_getMinLabel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getMinDist()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:107
// ("cv::face::StandardCollector::getMinDist", vec![(pred!(const, [], []), _)]),
pub fn cv_face_StandardCollector_getMinDist_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getResults(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:112
// ("cv::face::StandardCollector::getResults", vec![(pred!(const, ["sorted"], ["bool"]), _)]),
pub fn cv_face_StandardCollector_getResults_const_bool(instance: *const c_void, sorted: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::StandardCollector::getResults() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:112
// ("cv::face::StandardCollector::getResults", vec![(pred!(const, [], []), _)]),
pub fn cv_face_StandardCollector_getResults_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:120
// ("cv::face::StandardCollector::create", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_face_StandardCollector_create_double(threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::face::StandardCollector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:120
// ("cv::face::StandardCollector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_StandardCollector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::face::StandardCollector::to_PredictCollector() generated
// ("cv::face::StandardCollector::to_PredictCollector", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_StandardCollector_to_PredictCollector(instance: *mut c_void) -> *mut c_void;
// cv::face::StandardCollector::delete() generated
// ("cv::face::StandardCollector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_StandardCollector_delete(instance: *mut c_void);
// PredictResult(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:89
// ("cv::face::StandardCollector::PredictResult::PredictResult", vec![(pred!(mut, ["label_", "distance_"], ["int", "double"]), _)]),
pub fn cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_: i32, distance_: f64, ocvrs_return: *mut Result<crate::face::StandardCollector_PredictResult>);
// cv::face::StandardCollector::PredictResult::PredictResult() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/face/predict_collector.hpp:89
// ("cv::face::StandardCollector::PredictResult::PredictResult", vec![(pred!(mut, [], []), _)]),
pub fn cv_face_StandardCollector_PredictResult_PredictResult(ocvrs_return: *mut Result<crate::face::StandardCollector_PredictResult>);
