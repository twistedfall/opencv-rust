// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:127
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, charuco_corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:127
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, corner_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:148
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, diamond_corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:148
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:379
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(image: *const c_void, corners: *const c_void, ocvrs_return: *mut Result<()>);
// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:379
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
pub fn cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, corners: *const c_void, ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::aruco::extendDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:146
// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
pub fn cv_aruco_extendDictionary_int_int(n_markers: i32, marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// extendDictionary(int, int, const Dictionary &, int)(Primitive, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:146
// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::aruco::Dictionary*", "int"]), _)]),
pub fn cv_aruco_extendDictionary_int_int_const_DictionaryR_int(n_markers: i32, marker_size: i32, base_dictionary: *const c_void, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::generateImageMarker(TraitClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:392
// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(dictionary: *const c_void, id: i32, side_pixels: i32, img: *const c_void, ocvrs_return: *mut Result<()>);
// generateImageMarker(const Dictionary &, int, int, OutputArray, int)(TraitClass, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:392
// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(dictionary: *const c_void, id: i32, side_pixels: i32, img: *const c_void, border_bits: i32, ocvrs_return: *mut Result<()>);
// getPredefinedDictionary(PredefinedDictionaryType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:127
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PredefinedDictionaryType"]), _)]),
pub fn cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(name: crate::objdetect::PredefinedDictionaryType, ocvrs_return: *mut Result<*mut c_void>);
// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:132
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
pub fn cv_aruco_getPredefinedDictionary_int(dict: i32, ocvrs_return: *mut Result<*mut c_void>);
// setInputSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:29
// ("cv::FaceDetectorYN::setInputSize", vec![(pred!(mut, ["input_size"], ["const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_setInputSize_const_SizeR(instance: *mut c_void, input_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getInputSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:31
// ("cv::FaceDetectorYN::getInputSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getInputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
// setScoreThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:37
// ("cv::FaceDetectorYN::setScoreThreshold", vec![(pred!(mut, ["score_threshold"], ["float"]), _)]),
pub fn cv_FaceDetectorYN_setScoreThreshold_float(instance: *mut c_void, score_threshold: f32, ocvrs_return: *mut Result<()>);
// getScoreThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:39
// ("cv::FaceDetectorYN::getScoreThreshold", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getScoreThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setNMSThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:45
// ("cv::FaceDetectorYN::setNMSThreshold", vec![(pred!(mut, ["nms_threshold"], ["float"]), _)]),
pub fn cv_FaceDetectorYN_setNMSThreshold_float(instance: *mut c_void, nms_threshold: f32, ocvrs_return: *mut Result<()>);
// getNMSThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:47
// ("cv::FaceDetectorYN::getNMSThreshold", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getNMSThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setTopK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:53
// ("cv::FaceDetectorYN::setTopK", vec![(pred!(mut, ["top_k"], ["int"]), _)]),
pub fn cv_FaceDetectorYN_setTopK_int(instance: *mut c_void, top_k: i32, ocvrs_return: *mut Result<()>);
// getTopK()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:55
// ("cv::FaceDetectorYN::getTopK", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_getTopK(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:72
// ("cv::FaceDetectorYN::detect", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<i32>);
// create(const String &, const String &, const Size &, float, float, int, int, int)(InString, InString, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:85
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(model: *const c_char, config: *const c_char, input_size: *const core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::create(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:85
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size"], ["const cv::String*", "const cv::String*", "const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR(model: *const c_char, config: *const c_char, input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, const Size &, float, float, int, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:106
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR_float_float_int_int_int(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, input_size: *const core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::create(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:106
// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*"]), _)]),
pub fn cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceDetectorYN::delete() generated
// ("cv::FaceDetectorYN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceDetectorYN_delete(instance: *mut c_void);
// alignCrop(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:136
// ("cv::FaceRecognizerSF::alignCrop", vec![(pred!(const, ["src_img", "face_box", "aligned_img"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src_img: *const c_void, face_box: *const c_void, aligned_img: *const c_void, ocvrs_return: *mut Result<()>);
// feature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:142
// ("cv::FaceRecognizerSF::feature", vec![(pred!(mut, ["aligned_img", "face_feature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, aligned_img: *const c_void, face_feature: *const c_void, ocvrs_return: *mut Result<()>);
// match(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:149
// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2", "dis_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(instance: *const c_void, face_feature1: *const c_void, face_feature2: *const c_void, dis_type: i32, ocvrs_return: *mut Result<f64>);
// cv::FaceRecognizerSF::match(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:149
// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, face_feature1: *const c_void, face_feature2: *const c_void, ocvrs_return: *mut Result<f64>);
// create(const String &, const String &, int, int)(InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:157
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "int", "int"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(model: *const c_char, config: *const c_char, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::create(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:157
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:169
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "int", "int"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_int_int(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::create(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:169
// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FaceRecognizerSF::delete() generated
// ("cv::FaceRecognizerSF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FaceRecognizerSF_delete(instance: *mut c_void);
// GraphicalCodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:17
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// GraphicalCodeDetector(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:19
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector_const_GraphicalCodeDetectorR(unnamed: *const c_void) -> *mut c_void;
// GraphicalCodeDetector(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:20
// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_GraphicalCodeDetector_GraphicalCodeDetectorRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:21
// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_operatorST_const_GraphicalCodeDetectorR(instance: *mut c_void, unnamed: *const c_void);
// operator=(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:22
// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
pub fn cv_GraphicalCodeDetector_operatorST_GraphicalCodeDetectorRR(instance: *mut c_void, unnamed: *mut c_void);
// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:28
// ("cv::GraphicalCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:37
// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GraphicalCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:37
// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:45
// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GraphicalCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:45
// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(instance: *const c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:53
// ("cv::GraphicalCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:61
// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, straight_code: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:61
// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
pub fn cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, ocvrs_return: *mut Result<bool>);
// detectAndDecodeMulti(InputArray, std::vector<std::string> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:74
// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_code"], ["const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, points: *const c_void, straight_code: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:74
// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info"], ["const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
pub fn cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::GraphicalCodeDetector::implicitClone() generated
// ("cv::GraphicalCodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GraphicalCodeDetector_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GraphicalCodeDetector::delete() generated
// ("cv::GraphicalCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GraphicalCodeDetector_delete(instance: *mut c_void);
// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:160
// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetector_QRCodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:166
// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
pub fn cv_QRCodeDetector_setEpsX_double(instance: *mut c_void, eps_x: f64, ocvrs_return: *mut Result<*mut c_void>);
// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:171
// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
pub fn cv_QRCodeDetector_setEpsY_double(instance: *mut c_void, eps_y: f64, ocvrs_return: *mut Result<*mut c_void>);
// setUseAlignmentMarkers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:177
// ("cv::QRCodeDetector::setUseAlignmentMarkers", vec![(pred!(mut, ["useAlignmentMarkers"], ["bool"]), _)]),
pub fn cv_QRCodeDetector_setUseAlignmentMarkers_bool(instance: *mut c_void, use_alignment_markers: bool, ocvrs_return: *mut Result<*mut c_void>);
// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:186
// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:186
// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:194
// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:194
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
// QRCodeDetectorAruco()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:201
// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_QRCodeDetectorAruco(ocvrs_return: *mut Result<*mut c_void>);
// QRCodeDetectorAruco(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:236
// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
pub fn cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(params: *const crate::objdetect::QRCodeDetectorAruco_Params, ocvrs_return: *mut Result<*mut c_void>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:239
// ("cv::QRCodeDetectorAruco::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::QRCodeDetectorAruco_Params>);
// setDetectorParameters(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:242
// ("cv::QRCodeDetectorAruco::setDetectorParameters", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
pub fn cv_QRCodeDetectorAruco_setDetectorParameters_const_ParamsR(instance: *mut c_void, params: *const crate::objdetect::QRCodeDetectorAruco_Params, ocvrs_return: *mut Result<*mut c_void>);
// getArucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:245
// ("cv::QRCodeDetectorAruco::getArucoParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_getArucoParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setArucoParameters(const aruco::DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:248
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
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:204
// ("cv::QRCodeDetectorAruco::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeDetectorAruco_Params_Params(ocvrs_return: *mut Result<crate::objdetect::QRCodeDetectorAruco_Params>);
// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:140
// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
pub fn cv_QRCodeEncoder_create_const_ParamsR(parameters: *const crate::objdetect::QRCodeEncoder_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:140
// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_create(ocvrs_return: *mut Result<*mut c_void>);
// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:146
// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcode: *const c_void, ocvrs_return: *mut Result<()>);
// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:152
// ("cv::QRCodeEncoder::encodeStructuredAppend", vec![(pred!(mut, ["encoded_info", "qrcodes"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcodes: *const c_void, ocvrs_return: *mut Result<()>);
// cv::QRCodeEncoder::delete() generated
// ("cv::QRCodeEncoder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:121
// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_QRCodeEncoder_Params_Params(ocvrs_return: *mut Result<crate::objdetect::QRCodeEncoder_Params>);
// ArucoDetector(const Dictionary &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:284
// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, ["dictionary", "detectorParams", "refineParams"], ["const cv::aruco::Dictionary*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(dictionary: *const c_void, detector_params: *const c_void, refine_params: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::ArucoDetector::ArucoDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:284
// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_ArucoDetector(ocvrs_return: *mut Result<*mut c_void>);
// detectMarkers(InputArray, OutputArrayOfArrays, OutputArray, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:308
// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, corners: *const c_void, ids: *const c_void, rejected_img_points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::detectMarkers(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:308
// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, corners: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<()>);
// refineDetectedMarkers(InputArray, const Board &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, OutputArray)(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:333
// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "recoveredIdxs"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, recovered_idxs: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::refineDetectedMarkers(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:333
// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, ocvrs_return: *mut Result<()>);
// getDictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:339
// ("cv::aruco::ArucoDetector::getDictionary", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getDictionary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDictionary(const Dictionary &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:340
// ("cv::aruco::ArucoDetector::setDictionary", vec![(pred!(mut, ["dictionary"], ["const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(instance: *mut c_void, dictionary: *const c_void, ocvrs_return: *mut Result<()>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:342
// ("cv::aruco::ArucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:343
// ("cv::aruco::ArucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(instance: *mut c_void, detector_parameters: *const c_void, ocvrs_return: *mut Result<()>);
// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:345
// ("cv::aruco::ArucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_ArucoDetector_getRefineParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:346
// ("cv::aruco::ArucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(instance: *mut c_void, refine_parameters: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:350
// ("cv::aruco::ArucoDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_ArucoDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:354
// ("cv::aruco::ArucoDetector::write", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:358
// ("cv::aruco::ArucoDetector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_ArucoDetector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::ArucoDetector::to_Algorithm() generated
// ("cv::aruco::ArucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::aruco::ArucoDetector::delete() generated
// ("cv::aruco::ArucoDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_ArucoDetector_delete(instance: *mut c_void);
// Board(InputArrayOfArrays, const Dictionary &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:33
// ("cv::aruco::Board::Board", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(obj_points: *const c_void, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:37
// ("cv::aruco::Board::getDictionary", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getDictionary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getObjPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:49
// ("cv::aruco::Board::getObjPoints", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getObjPoints_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getIds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:54
// ("cv::aruco::Board::getIds", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getRightBottomCorner()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:58
// ("cv::aruco::Board::getRightBottomCorner", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_getRightBottomCorner_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point3f>);
// matchImagePoints(InputArrayOfArrays, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:78
// ("cv::aruco::Board::matchImagePoints", vec![(pred!(const, ["detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, obj_points: *const c_void, img_points: *const c_void, ocvrs_return: *mut Result<()>);
// generateImage(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:91
// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(instance: *const c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result<()>);
// cv::aruco::Board::generateImage(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:91
// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(instance: *const c_void, out_size: *const core::Size, img: *const c_void, ocvrs_return: *mut Result<()>);
// Board()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:94
// ("cv::aruco::Board::Board", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Board_Board(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Board::implicitClone() generated
// ("cv::aruco::Board::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Board_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Board::delete() generated
// ("cv::aruco::Board::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Board_delete(instance: *mut c_void);
// CharucoBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:146
// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(size: *const core::Size, square_length: f32, marker_length: f32, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoBoard::CharucoBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:146
// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(size: *const core::Size, square_length: f32, marker_length: f32, dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setLegacyPattern(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:158
// ("cv::aruco::CharucoBoard::setLegacyPattern", vec![(pred!(mut, ["legacyPattern"], ["bool"]), _)]),
pub fn cv_aruco_CharucoBoard_setLegacyPattern_bool(instance: *mut c_void, legacy_pattern: bool, ocvrs_return: *mut Result<()>);
// getLegacyPattern()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:159
// ("cv::aruco::CharucoBoard::getLegacyPattern", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getLegacyPattern_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:161
// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getChessboardSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getSquareLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:162
// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getSquareLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:163
// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getChessboardCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:167
// ("cv::aruco::CharucoBoard::getChessboardCorners", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getChessboardCorners_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNearestMarkerIdx()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:171
// ("cv::aruco::CharucoBoard::getNearestMarkerIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getNearestMarkerIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNearestMarkerCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:175
// ("cv::aruco::CharucoBoard::getNearestMarkerCorners", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoBoard_getNearestMarkerCorners_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// checkCharucoCornersCollinear(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:188
// ("cv::aruco::CharucoBoard::checkCharucoCornersCollinear", vec![(pred!(const, ["charucoIds"], ["const cv::_InputArray*"]), _)]),
pub fn cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(instance: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<bool>);
// CharucoBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:191
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
// CharucoDetector(const CharucoBoard &, const CharucoParameters &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:42
// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board", "charucoParams", "detectorParams", "refineParams"], ["const cv::aruco::CharucoBoard*", "const cv::aruco::CharucoParameters*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(board: *const c_void, charuco_params: *const c_void, detector_params: *const c_void, refine_params: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoDetector::CharucoDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:42
// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
pub fn cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(board: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:47
// ("cv::aruco::CharucoDetector::getBoard", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getBoard_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setBoard(const CharucoBoard &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:48
// ("cv::aruco::CharucoDetector::setBoard", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
pub fn cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(instance: *mut c_void, board: *const c_void, ocvrs_return: *mut Result<()>);
// getCharucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:50
// ("cv::aruco::CharucoDetector::getCharucoParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getCharucoParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCharucoParameters(CharucoParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:51
// ("cv::aruco::CharucoDetector::setCharucoParameters", vec![(pred!(mut, ["charucoParameters"], ["cv::aruco::CharucoParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(instance: *mut c_void, charuco_parameters: *mut c_void, ocvrs_return: *mut Result<()>);
// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:53
// ("cv::aruco::CharucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getDetectorParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:54
// ("cv::aruco::CharucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(instance: *mut c_void, detector_parameters: *const c_void, ocvrs_return: *mut Result<()>);
// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:56
// ("cv::aruco::CharucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoDetector_getRefineParameters_const(instance: *const c_void, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:57
// ("cv::aruco::CharucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
pub fn cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(instance: *mut c_void, refine_parameters: *const crate::objdetect::RefineParameters, ocvrs_return: *mut Result<()>);
// detectBoard(InputArray, OutputArray, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:84
// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::detectBoard(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:84
// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<()>);
// detectDiamonds(InputArray, OutputArrayOfArrays, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:108
// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::detectDiamonds(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:108
// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::CharucoDetector::to_Algorithm() generated
// ("cv::aruco::CharucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::aruco::CharucoDetector::delete() generated
// ("cv::aruco::CharucoDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoDetector_delete(instance: *mut c_void);
// CharucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:16
// ("cv::aruco::CharucoParameters::CharucoParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoParameters_CharucoParameters(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::CharucoParameters::implicitClone() generated
// ("cv::aruco::CharucoParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::cameraMatrix() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:21
// ("cv::aruco::CharucoParameters::cameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propCameraMatrix_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::setCameraMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:21
// ("cv::aruco::CharucoParameters::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_CharucoParameters_propCameraMatrix_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::CharucoParameters::distCoeffs() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:24
// ("cv::aruco::CharucoParameters::distCoeffs", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propDistCoeffs_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::CharucoParameters::setDistCoeffs(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:24
// ("cv::aruco::CharucoParameters::setDistCoeffs", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_CharucoParameters_propDistCoeffs_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::CharucoParameters::minMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:27
// ("cv::aruco::CharucoParameters::minMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propMinMarkers_const(instance: *const c_void) -> i32;
// cv::aruco::CharucoParameters::setMinMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:27
// ("cv::aruco::CharucoParameters::setMinMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_CharucoParameters_propMinMarkers_const_int(instance: *mut c_void, val: i32);
// cv::aruco::CharucoParameters::tryRefineMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:30
// ("cv::aruco::CharucoParameters::tryRefineMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_CharucoParameters_propTryRefineMarkers_const(instance: *const c_void) -> bool;
// cv::aruco::CharucoParameters::setTryRefineMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:30
// ("cv::aruco::CharucoParameters::setTryRefineMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_CharucoParameters_propTryRefineMarkers_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::CharucoParameters::delete() generated
// ("cv::aruco::CharucoParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_CharucoParameters_delete(instance: *mut c_void);
// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:26
// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return: *mut Result<*mut c_void>);
// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:64
// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeDetectorParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:68
// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::aruco::DetectorParameters::writeDetectorParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:68
// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::aruco::DetectorParameters::implicitClone() generated
// ("cv::aruco::DetectorParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:71
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:71
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:74
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:74
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:77
// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:77
// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:80
// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:80
// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:86
// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:86
// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:92
// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:92
// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:95
// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:95
// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:98
// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:98
// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:101
// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinDistanceToBorder_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:101
// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:115
// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:115
// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minGroupDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:121
// ("cv::aruco::DetectorParameters::minGroupDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinGroupDistance_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setMinGroupDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:121
// ("cv::aruco::DetectorParameters::setMinGroupDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinGroupDistance_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:124
// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMethod_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:124
// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:134
// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:134
// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:145
// ("cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:145
// ("cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:148
// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:148
// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:151
// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:151
// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:154
// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMarkerBorderBits_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:154
// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:157
// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:157
// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:163
// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:163
// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:169
// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:169
// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:174
// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinOtsuStdDev_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:174
// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:177
// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propErrorCorrectionRate_const(instance: *const c_void) -> f64;
// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:177
// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(instance: *mut c_void, val: f64);
// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:184
// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:184
// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:187
// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:187
// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:191
// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:191
// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:194
// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:194
// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:200
// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:200
// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:203
// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:203
// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:210
// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:210
// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:213
// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagDeglitch_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:213
// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:219
// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propDetectInvertedMarker_const(instance: *const c_void) -> bool;
// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:219
// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::DetectorParameters::useAruco3Detection() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:227
// ("cv::aruco::DetectorParameters::useAruco3Detection", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propUseAruco3Detection_const(instance: *const c_void) -> bool;
// cv::aruco::DetectorParameters::setUseAruco3Detection(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:227
// ("cv::aruco::DetectorParameters::setUseAruco3Detection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_DetectorParameters_propUseAruco3Detection_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::DetectorParameters::minSideLengthCanonicalImg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:230
// ("cv::aruco::DetectorParameters::minSideLengthCanonicalImg", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const(instance: *const c_void) -> i32;
// cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:230
// ("cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const_int(instance: *mut c_void, val: i32);
// cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:233
// ("cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const(instance: *const c_void) -> f32;
// cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:233
// ("cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const_float(instance: *mut c_void, val: f32);
// cv::aruco::DetectorParameters::delete() generated
// ("cv::aruco::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_DetectorParameters_delete(instance: *mut c_void);
// Dictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:36
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Dictionary_Dictionary(ocvrs_return: *mut Result<*mut c_void>);
// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:44
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize", "maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_aruco_Dictionary_Dictionary_const_MatR_int_int(bytes_list: *const c_void, _marker_size: i32, maxcorr: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Dictionary::Dictionary(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:44
// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize"], ["const cv::Mat*", "int"]), _)]),
pub fn cv_aruco_Dictionary_Dictionary_const_MatR_int(bytes_list: *const c_void, _marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:56
// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_Dictionary_readDictionary_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeDictionary(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:60
// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(instance: *mut c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::aruco::Dictionary::writeDictionary(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:60
// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_Dictionary_writeDictionary_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:66
// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
pub fn cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(instance: *const c_void, only_bits: *const c_void, idx: *mut i32, rotation: *mut i32, max_correction_rate: f64, ocvrs_return: *mut Result<bool>);
// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:72
// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
pub fn cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(instance: *const c_void, bits: *const c_void, id: i32, all_rotations: bool, ocvrs_return: *mut Result<i32>);
// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:72
// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(instance: *const c_void, bits: *const c_void, id: i32, ocvrs_return: *mut Result<i32>);
// generateImageMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:77
// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(instance: *const c_void, id: i32, side_pixels: i32, _img: *const c_void, border_bits: i32, ocvrs_return: *mut Result<()>);
// cv::aruco::Dictionary::generateImageMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:77
// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(instance: *const c_void, id: i32, side_pixels: i32, _img: *const c_void, ocvrs_return: *mut Result<()>);
// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:82
// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
pub fn cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:87
// ("cv::aruco::Dictionary::getBitsFromByteList", vec![(pred!(mut, ["byteList", "markerSize"], ["const cv::Mat*", "int"]), _)]),
pub fn cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list: *const c_void, marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::Dictionary::implicitClone() generated
// ("cv::aruco::Dictionary::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:32
// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propBytesList_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:32
// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_aruco_Dictionary_propBytesList_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:33
// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propMarkerSize_const(instance: *const c_void) -> i32;
// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:33
// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_Dictionary_propMarkerSize_const_int(instance: *mut c_void, val: i32);
// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:34
// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_Dictionary_propMaxCorrectionBits_const(instance: *const c_void) -> i32;
// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:34
// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_Dictionary_propMaxCorrectionBits_const_int(instance: *mut c_void, val: i32);
// cv::aruco::Dictionary::delete() generated
// ("cv::aruco::Dictionary::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_Dictionary_delete(instance: *mut c_void);
// GridBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:118
// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(size: *const core::Size, marker_length: f32, marker_separation: f32, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::GridBoard::GridBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:118
// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
pub fn cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(size: *const core::Size, marker_length: f32, marker_separation: f32, dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGridSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:121
// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:122
// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:123
// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_GridBoard_getMarkerSeparation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// GridBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:126
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
// RefineParameters(float, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:239
// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, ["minRepDistance", "errorCorrectionRate", "checkAllOrders"], ["float", "float", "bool"]), _)]),
pub fn cv_aruco_RefineParameters_RefineParameters_float_float_bool(min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// cv::aruco::RefineParameters::RefineParameters() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:239
// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_RefineParameters_RefineParameters(ocvrs_return: *mut Result<crate::objdetect::RefineParameters>);
// readRefineParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:244
// ("cv::aruco::RefineParameters::readRefineParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(instance: *const crate::objdetect::RefineParameters, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
// writeRefineParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:248
// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(instance: *const crate::objdetect::RefineParameters, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::aruco::RefineParameters::writeRefineParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:248
// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(instance: *const crate::objdetect::RefineParameters, fs: *mut c_void, ocvrs_return: *mut Result<bool>);
// BarcodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:23
// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_BarcodeDetector(ocvrs_return: *mut Result<*mut c_void>);
// BarcodeDetector(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:30
// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, ["prototxt_path", "model_path"], ["const std::string*", "const std::string*"]), _)]),
pub fn cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(prototxt_path: *const c_char, model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// decodeWithType(InputArray, InputArray, std::vector<std::string> &, std::vector<std::string> &)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:43
// ("cv::barcode::BarcodeDetector::decodeWithType", vec![(pred!(const, ["img", "points", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, ocvrs_return: *mut Result<bool>);
// detectAndDecodeWithType(InputArray, std::vector<std::string> &, std::vector<std::string> &, OutputArray)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:56
// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type", "points"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::barcode::BarcodeDetector::detectAndDecodeWithType(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:56
// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, ocvrs_return: *mut Result<bool>);
// getDownsamplingThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:65
// ("cv::barcode::BarcodeDetector::getDownsamplingThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_getDownsamplingThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDownsamplingThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:75
// ("cv::barcode::BarcodeDetector::setDownsamplingThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
pub fn cv_barcode_BarcodeDetector_setDownsamplingThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// getDetectorScales(std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:81
// ("cv::barcode::BarcodeDetector::getDetectorScales", vec![(pred!(const, ["sizes"], ["std::vector<float>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_getDetectorScales_const_vectorLfloatGR(instance: *const c_void, sizes: *mut c_void, ocvrs_return: *mut Result<()>);
// setDetectorScales(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:90
// ("cv::barcode::BarcodeDetector::setDetectorScales", vec![(pred!(mut, ["sizes"], ["const std::vector<float>*"]), _)]),
pub fn cv_barcode_BarcodeDetector_setDetectorScales_const_vectorLfloatGR(instance: *mut c_void, sizes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGradientThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:96
// ("cv::barcode::BarcodeDetector::getGradientThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_barcode_BarcodeDetector_getGradientThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGradientThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:105
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
