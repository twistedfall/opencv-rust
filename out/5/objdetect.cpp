#include "ocvrs_common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:127
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:127
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, cv::Scalar* cornerColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners, *charucoIds, *cornerColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:148
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:148
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, const cv::_InputArray* diamondIds, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners, *diamondIds, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:379
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* corners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:379
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* corners, const cv::_InputArray* ids, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners, *ids, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::extendDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:146
	// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
	void cv_aruco_extendDictionary_int_int(int nMarkers, int markerSize, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::extendDictionary(nMarkers, markerSize);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extendDictionary(int, int, const Dictionary &, int)(Primitive, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:146
	// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::aruco::Dictionary*", "int"]), _)]),
	void cv_aruco_extendDictionary_int_int_const_DictionaryR_int(int nMarkers, int markerSize, const cv::aruco::Dictionary* baseDictionary, int randomSeed, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::extendDictionary(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::generateImageMarker(TraitClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:392
	// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(const cv::aruco::Dictionary* dictionary, int id, int sidePixels, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::generateImageMarker(*dictionary, id, sidePixels, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImageMarker(const Dictionary &, int, int, OutputArray, int)(TraitClass, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:392
	// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* dictionary, int id, int sidePixels, const cv::_OutputArray* img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::generateImageMarker(*dictionary, id, sidePixels, *img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(PredefinedDictionaryType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:127
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PredefinedDictionaryType"]), _)]),
	void cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(cv::aruco::PredefinedDictionaryType name, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::getPredefinedDictionary(name);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:132
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
	void cv_aruco_getPredefinedDictionary_int(int dict, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::getPredefinedDictionary(dict);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:29
	// ("cv::FaceDetectorYN::setInputSize", vec![(pred!(mut, ["input_size"], ["const cv::Size*"]), _)]),
	void cv_FaceDetectorYN_setInputSize_const_SizeR(cv::FaceDetectorYN* instance, const cv::Size* input_size, ResultVoid* ocvrs_return) {
		try {
			instance->setInputSize(*input_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInputSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:31
	// ("cv::FaceDetectorYN::getInputSize", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getInputSize(cv::FaceDetectorYN* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getInputSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScoreThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:37
	// ("cv::FaceDetectorYN::setScoreThreshold", vec![(pred!(mut, ["score_threshold"], ["float"]), _)]),
	void cv_FaceDetectorYN_setScoreThreshold_float(cv::FaceDetectorYN* instance, float score_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setScoreThreshold(score_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScoreThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:39
	// ("cv::FaceDetectorYN::getScoreThreshold", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getScoreThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScoreThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMSThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:45
	// ("cv::FaceDetectorYN::setNMSThreshold", vec![(pred!(mut, ["nms_threshold"], ["float"]), _)]),
	void cv_FaceDetectorYN_setNMSThreshold_float(cv::FaceDetectorYN* instance, float nms_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setNMSThreshold(nms_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMSThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:47
	// ("cv::FaceDetectorYN::getNMSThreshold", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getNMSThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNMSThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTopK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:53
	// ("cv::FaceDetectorYN::setTopK", vec![(pred!(mut, ["top_k"], ["int"]), _)]),
	void cv_FaceDetectorYN_setTopK_int(cv::FaceDetectorYN* instance, int top_k, ResultVoid* ocvrs_return) {
		try {
			instance->setTopK(top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTopK()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:55
	// ("cv::FaceDetectorYN::getTopK", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getTopK(cv::FaceDetectorYN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTopK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:72
	// ("cv::FaceDetectorYN::detect", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(cv::FaceDetectorYN* instance, const cv::_InputArray* image, const cv::_OutputArray* faces, Result<int>* ocvrs_return) {
		try {
			int ret = instance->detect(*image, *faces);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, const Size &, float, float, int, int, int)(InString, InString, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:85
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(const char* model, const char* config, const cv::Size* input_size, float score_threshold, float nms_threshold, int top_k, int backend_id, int target_id, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(model), std::string(config), *input_size, score_threshold, nms_threshold, top_k, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceDetectorYN::create(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:85
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size"], ["const cv::String*", "const cv::String*", "const cv::Size*"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR(const char* model, const char* config, const cv::Size* input_size, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(model), std::string(config), *input_size);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, const Size &, float, float, int, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:106
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR_float_float_int_int_int(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, const cv::Size* input_size, float score_threshold, float nms_threshold, int top_k, int backend_id, int target_id, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(framework), *bufferModel, *bufferConfig, *input_size, score_threshold, nms_threshold, top_k, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceDetectorYN::create(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:106
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, const cv::Size* input_size, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(framework), *bufferModel, *bufferConfig, *input_size);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceDetectorYN::delete() generated
	// ("cv::FaceDetectorYN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_delete(cv::FaceDetectorYN* instance) {
			delete instance;
	}

	// alignCrop(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:136
	// ("cv::FaceRecognizerSF::alignCrop", vec![(pred!(const, ["src_img", "face_box", "aligned_img"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::FaceRecognizerSF* instance, const cv::_InputArray* src_img, const cv::_InputArray* face_box, const cv::_OutputArray* aligned_img, ResultVoid* ocvrs_return) {
		try {
			instance->alignCrop(*src_img, *face_box, *aligned_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:142
	// ("cv::FaceRecognizerSF::feature", vec![(pred!(mut, ["aligned_img", "face_feature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(cv::FaceRecognizerSF* instance, const cv::_InputArray* aligned_img, const cv::_OutputArray* face_feature, ResultVoid* ocvrs_return) {
		try {
			instance->feature(*aligned_img, *face_feature);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:149
	// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2", "dis_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(const cv::FaceRecognizerSF* instance, const cv::_InputArray* face_feature1, const cv::_InputArray* face_feature2, int dis_type, Result<double>* ocvrs_return) {
		try {
			double ret = instance->match(*face_feature1, *face_feature2, dis_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::match(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:149
	// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(const cv::FaceRecognizerSF* instance, const cv::_InputArray* face_feature1, const cv::_InputArray* face_feature2, Result<double>* ocvrs_return) {
		try {
			double ret = instance->match(*face_feature1, *face_feature2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, int, int)(InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:157
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "int", "int"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(const char* model, const char* config, int backend_id, int target_id, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(model), std::string(config), backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::create(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:157
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_StringR(const char* model, const char* config, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(model), std::string(config));
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:169
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "int", "int"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_int_int(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, int backend_id, int target_id, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(framework), *bufferModel, *bufferConfig, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::create(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/face.hpp:169
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(framework), *bufferModel, *bufferConfig);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::delete() generated
	// ("cv::FaceRecognizerSF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FaceRecognizerSF_delete(cv::FaceRecognizerSF* instance) {
			delete instance;
	}

	// GraphicalCodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:17
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_GraphicalCodeDetector_GraphicalCodeDetector(Result<cv::GraphicalCodeDetector*>* ocvrs_return) {
		try {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GraphicalCodeDetector(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:19
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
	cv::GraphicalCodeDetector* cv_GraphicalCodeDetector_GraphicalCodeDetector_const_GraphicalCodeDetectorR(const cv::GraphicalCodeDetector* unnamed) {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector(*unnamed);
			return ret;
	}

	// GraphicalCodeDetector(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:20
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
	cv::GraphicalCodeDetector* cv_GraphicalCodeDetector_GraphicalCodeDetector_GraphicalCodeDetectorRR(cv::GraphicalCodeDetector* unnamed) {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector(std::move(*unnamed));
			return ret;
	}

	// operator=(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:21
	// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
	void cv_GraphicalCodeDetector_operatorST_const_GraphicalCodeDetectorR(cv::GraphicalCodeDetector* instance, const cv::GraphicalCodeDetector* unnamed) {
			instance->operator=(*unnamed);
	}

	// operator=(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:22
	// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
	void cv_GraphicalCodeDetector_operatorST_GraphicalCodeDetectorRR(cv::GraphicalCodeDetector* instance, cv::GraphicalCodeDetector* unnamed) {
			instance->operator=(std::move(*unnamed));
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:28
	// ("cv::GraphicalCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detect(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:37
	// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_code, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->decode(*img, *points, *straight_code);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:37
	// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->decode(*img, *points);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:45
	// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_code, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecode(*img, *points, *straight_code);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:45
	// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecode(*img);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:53
	// ("cv::GraphicalCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectMulti(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:61
	// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, const cv::_OutputArray* straight_code, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_code);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:61
	// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	void cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeMulti(InputArray, std::vector<std::string> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:74
	// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_code"], ["const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, const cv::_OutputArray* points, const cv::_OutputArray* straight_code, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info, *points, *straight_code);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/graphical_code_detector.hpp:74
	// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info"], ["const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::implicitClone() generated
	// ("cv::GraphicalCodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GraphicalCodeDetector* cv_GraphicalCodeDetector_implicitClone_const(const cv::GraphicalCodeDetector* instance) {
			return new cv::GraphicalCodeDetector(*instance);
	}

	// cv::GraphicalCodeDetector::delete() generated
	// ("cv::GraphicalCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GraphicalCodeDetector_delete(cv::GraphicalCodeDetector* instance) {
			delete instance;
	}

	// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:160
	// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetector_QRCodeDetector(Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:166
	// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setEpsX(epsX);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:171
	// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setEpsY(epsY);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseAlignmentMarkers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:177
	// ("cv::QRCodeDetector::setUseAlignmentMarkers", vec![(pred!(mut, ["useAlignmentMarkers"], ["bool"]), _)]),
	void cv_QRCodeDetector_setUseAlignmentMarkers_bool(cv::QRCodeDetector* instance, bool useAlignmentMarkers, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setUseAlignmentMarkers(useAlignmentMarkers);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:186
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:186
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:194
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:194
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecodeCurved(*img);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::implicitClone() generated
	// ("cv::QRCodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::QRCodeDetector* cv_QRCodeDetector_implicitClone_const(const cv::QRCodeDetector* instance) {
			return new cv::QRCodeDetector(*instance);
	}

	// cv::QRCodeDetector::to_GraphicalCodeDetector() generated
	// ("cv::QRCodeDetector::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
	cv::GraphicalCodeDetector* cv_QRCodeDetector_to_GraphicalCodeDetector(cv::QRCodeDetector* instance) {
			return dynamic_cast<cv::GraphicalCodeDetector*>(instance);
	}

	// cv::QRCodeDetector::delete() generated
	// ("cv::QRCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetector_delete(cv::QRCodeDetector* instance) {
			delete instance;
	}

	// QRCodeDetectorAruco()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:201
	// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetectorAruco_QRCodeDetectorAruco(Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco* ret = new cv::QRCodeDetectorAruco();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// QRCodeDetectorAruco(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:236
	// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
	void cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(const cv::QRCodeDetectorAruco::Params* params, Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco* ret = new cv::QRCodeDetectorAruco(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:239
	// ("cv::QRCodeDetectorAruco::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_QRCodeDetectorAruco_getDetectorParameters_const(const cv::QRCodeDetectorAruco* instance, Result<cv::QRCodeDetectorAruco::Params>* ocvrs_return) {
		try {
			const cv::QRCodeDetectorAruco::Params ret = instance->getDetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:242
	// ("cv::QRCodeDetectorAruco::setDetectorParameters", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
	void cv_QRCodeDetectorAruco_setDetectorParameters_const_ParamsR(cv::QRCodeDetectorAruco* instance, const cv::QRCodeDetectorAruco::Params* params, Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco ret = instance->setDetectorParameters(*params);
			Ok(new cv::QRCodeDetectorAruco(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getArucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:245
	// ("cv::QRCodeDetectorAruco::getArucoParameters", vec![(pred!(const, [], []), _)]),
	void cv_QRCodeDetectorAruco_getArucoParameters_const(const cv::QRCodeDetectorAruco* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getArucoParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setArucoParameters(const aruco::DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:248
	// ("cv::QRCodeDetectorAruco::setArucoParameters", vec![(pred!(mut, ["params"], ["const cv::aruco::DetectorParameters*"]), _)]),
	void cv_QRCodeDetectorAruco_setArucoParameters_const_DetectorParametersR(cv::QRCodeDetectorAruco* instance, const cv::aruco::DetectorParameters* params, ResultVoid* ocvrs_return) {
		try {
			instance->setArucoParameters(*params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetectorAruco::implicitClone() generated
	// ("cv::QRCodeDetectorAruco::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::QRCodeDetectorAruco* cv_QRCodeDetectorAruco_implicitClone_const(const cv::QRCodeDetectorAruco* instance) {
			return new cv::QRCodeDetectorAruco(*instance);
	}

	// cv::QRCodeDetectorAruco::to_GraphicalCodeDetector() generated
	// ("cv::QRCodeDetectorAruco::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
	cv::GraphicalCodeDetector* cv_QRCodeDetectorAruco_to_GraphicalCodeDetector(cv::QRCodeDetectorAruco* instance) {
			return dynamic_cast<cv::GraphicalCodeDetector*>(instance);
	}

	// cv::QRCodeDetectorAruco::delete() generated
	// ("cv::QRCodeDetectorAruco::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetectorAruco_delete(cv::QRCodeDetectorAruco* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:204
	// ("cv::QRCodeDetectorAruco::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetectorAruco_Params_Params(Result<cv::QRCodeDetectorAruco::Params>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:140
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
	void cv_QRCodeEncoder_create_const_ParamsR(const cv::QRCodeEncoder::Params* parameters, Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create(*parameters);
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:140
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_create(Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create();
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:146
	// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcode, ResultVoid* ocvrs_return) {
		try {
			instance->encode(std::string(encoded_info), *qrcode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:152
	// ("cv::QRCodeEncoder::encodeStructuredAppend", vec![(pred!(mut, ["encoded_info", "qrcodes"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcodes, ResultVoid* ocvrs_return) {
		try {
			instance->encodeStructuredAppend(std::string(encoded_info), *qrcodes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeEncoder::delete() generated
	// ("cv::QRCodeEncoder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_delete(cv::QRCodeEncoder* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect.hpp:121
	// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_Params_Params(Result<cv::QRCodeEncoder::Params>* ocvrs_return) {
		try {
			cv::QRCodeEncoder::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ArucoDetector(const Dictionary &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:284
	// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, ["dictionary", "detectorParams", "refineParams"], ["const cv::aruco::Dictionary*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(const cv::aruco::Dictionary* dictionary, const cv::aruco::DetectorParameters* detectorParams, const cv::aruco::RefineParameters* refineParams, Result<cv::aruco::ArucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::ArucoDetector* ret = new cv::aruco::ArucoDetector(*dictionary, *detectorParams, *refineParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::ArucoDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:284
	// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_ArucoDetector_ArucoDetector(Result<cv::aruco::ArucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::ArucoDetector* ret = new cv::aruco::ArucoDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMarkers(InputArray, OutputArrayOfArrays, OutputArray, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:308
	// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::_OutputArray* rejectedImgPoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectMarkers(*image, *corners, *ids, *rejectedImgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::detectMarkers(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:308
	// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_OutputArray* ids, ResultVoid* ocvrs_return) {
		try {
			instance->detectMarkers(*image, *corners, *ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// refineDetectedMarkers(InputArray, const Board &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, OutputArray)(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:333
	// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "recoveredIdxs"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::aruco::Board* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* recoveredIdxs, ResultVoid* ocvrs_return) {
		try {
			instance->refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, *recoveredIdxs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::refineDetectedMarkers(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:333
	// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::aruco::Board* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, ResultVoid* ocvrs_return) {
		try {
			instance->refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:339
	// ("cv::aruco::ArucoDetector::getDictionary", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getDictionary_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			const cv::aruco::Dictionary ret = instance->getDictionary();
			Ok(new const cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDictionary(const Dictionary &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:340
	// ("cv::aruco::ArucoDetector::setDictionary", vec![(pred!(mut, ["dictionary"], ["const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(cv::aruco::ArucoDetector* instance, const cv::aruco::Dictionary* dictionary, ResultVoid* ocvrs_return) {
		try {
			instance->setDictionary(*dictionary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:342
	// ("cv::aruco::ArucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getDetectorParameters_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getDetectorParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:343
	// ("cv::aruco::ArucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
	void cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(cv::aruco::ArucoDetector* instance, const cv::aruco::DetectorParameters* detectorParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectorParameters(*detectorParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:345
	// ("cv::aruco::ArucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getRefineParameters_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			const cv::aruco::RefineParameters ret = instance->getRefineParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:346
	// ("cv::aruco::ArucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(cv::aruco::ArucoDetector* instance, const cv::aruco::RefineParameters* refineParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineParameters(*refineParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:350
	// ("cv::aruco::ArucoDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_ArucoDetector_write_const_FileStorageR(const cv::aruco::ArucoDetector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:354
	// ("cv::aruco::ArucoDetector::write", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(cv::aruco::ArucoDetector* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:358
	// ("cv::aruco::ArucoDetector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_ArucoDetector_read_const_FileNodeR(cv::aruco::ArucoDetector* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::to_Algorithm() generated
	// ("cv::aruco::ArucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_aruco_ArucoDetector_to_Algorithm(cv::aruco::ArucoDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::aruco::ArucoDetector::delete() generated
	// ("cv::aruco::ArucoDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_ArucoDetector_delete(cv::aruco::ArucoDetector* instance) {
			delete instance;
	}

	// Board(InputArrayOfArrays, const Dictionary &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:33
	// ("cv::aruco::Board::Board", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(const cv::_InputArray* objPoints, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::Board*>* ocvrs_return) {
		try {
			cv::aruco::Board* ret = new cv::aruco::Board(*objPoints, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:37
	// ("cv::aruco::Board::getDictionary", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getDictionary_const(const cv::aruco::Board* instance, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			const cv::aruco::Dictionary ret = instance->getDictionary();
			Ok(new const cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:49
	// ("cv::aruco::Board::getObjPoints", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getObjPoints_const(const cv::aruco::Board* instance, Result<std::vector<std::vector<cv::Point3f>>*>* ocvrs_return) {
		try {
			const std::vector<std::vector<cv::Point3f>> ret = instance->getObjPoints();
			Ok(new const std::vector<std::vector<cv::Point3f>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:54
	// ("cv::aruco::Board::getIds", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getIds_const(const cv::aruco::Board* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getIds();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRightBottomCorner()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:58
	// ("cv::aruco::Board::getRightBottomCorner", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getRightBottomCorner_const(const cv::aruco::Board* instance, Result<cv::Point3f>* ocvrs_return) {
		try {
			const cv::Point3f ret = instance->getRightBottomCorner();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchImagePoints(InputArrayOfArrays, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:78
	// ("cv::aruco::Board::matchImagePoints", vec![(pred!(const, ["detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::Board* instance, const cv::_InputArray* detectedCorners, const cv::_InputArray* detectedIds, const cv::_OutputArray* objPoints, const cv::_OutputArray* imgPoints, ResultVoid* ocvrs_return) {
		try {
			instance->matchImagePoints(*detectedCorners, *detectedIds, *objPoints, *imgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImage(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:91
	// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(const cv::aruco::Board* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->generateImage(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Board::generateImage(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:91
	// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(const cv::aruco::Board* instance, cv::Size* outSize, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->generateImage(*outSize, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Board()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:94
	// ("cv::aruco::Board::Board", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Board_Board(Result<cv::aruco::Board*>* ocvrs_return) {
		try {
			cv::aruco::Board* ret = new cv::aruco::Board();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Board::implicitClone() generated
	// ("cv::aruco::Board::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::Board* cv_aruco_Board_implicitClone_const(const cv::aruco::Board* instance) {
			return new cv::aruco::Board(*instance);
	}

	// cv::aruco::Board::delete() generated
	// ("cv::aruco::Board::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Board_delete(cv::aruco::Board* instance) {
			delete instance;
	}

	// CharucoBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:146
	// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(const cv::Size* size, float squareLength, float markerLength, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard(*size, squareLength, markerLength, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoBoard::CharucoBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:146
	// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(const cv::Size* size, float squareLength, float markerLength, const cv::aruco::Dictionary* dictionary, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard(*size, squareLength, markerLength, *dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLegacyPattern(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:158
	// ("cv::aruco::CharucoBoard::setLegacyPattern", vec![(pred!(mut, ["legacyPattern"], ["bool"]), _)]),
	void cv_aruco_CharucoBoard_setLegacyPattern_bool(cv::aruco::CharucoBoard* instance, bool legacyPattern, ResultVoid* ocvrs_return) {
		try {
			instance->setLegacyPattern(legacyPattern);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLegacyPattern()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:159
	// ("cv::aruco::CharucoBoard::getLegacyPattern", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getLegacyPattern_const(const cv::aruco::CharucoBoard* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getLegacyPattern();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:161
	// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getChessboardSize_const(const cv::aruco::CharucoBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getChessboardSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSquareLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:162
	// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getSquareLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSquareLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:163
	// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getMarkerLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChessboardCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:167
	// ("cv::aruco::CharucoBoard::getChessboardCorners", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getChessboardCorners_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<cv::Point3f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point3f> ret = instance->getChessboardCorners();
			Ok(new std::vector<cv::Point3f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNearestMarkerIdx()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:171
	// ("cv::aruco::CharucoBoard::getNearestMarkerIdx", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getNearestMarkerIdx_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<std::vector<int>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<int>> ret = instance->getNearestMarkerIdx();
			Ok(new std::vector<std::vector<int>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNearestMarkerCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:175
	// ("cv::aruco::CharucoBoard::getNearestMarkerCorners", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getNearestMarkerCorners_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<std::vector<int>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<int>> ret = instance->getNearestMarkerCorners();
			Ok(new std::vector<std::vector<int>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkCharucoCornersCollinear(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:188
	// ("cv::aruco::CharucoBoard::checkCharucoCornersCollinear", vec![(pred!(const, ["charucoIds"], ["const cv::_InputArray*"]), _)]),
	void cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(const cv::aruco::CharucoBoard* instance, const cv::_InputArray* charucoIds, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkCharucoCornersCollinear(*charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CharucoBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:191
	// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoBoard_CharucoBoard(Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoBoard::implicitClone() generated
	// ("cv::aruco::CharucoBoard::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::CharucoBoard* cv_aruco_CharucoBoard_implicitClone_const(const cv::aruco::CharucoBoard* instance) {
			return new cv::aruco::CharucoBoard(*instance);
	}

	// cv::aruco::CharucoBoard::to_Board() generated
	// ("cv::aruco::CharucoBoard::to_Board", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Board* cv_aruco_CharucoBoard_to_Board(cv::aruco::CharucoBoard* instance) {
			return dynamic_cast<cv::aruco::Board*>(instance);
	}

	// cv::aruco::CharucoBoard::delete() generated
	// ("cv::aruco::CharucoBoard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoBoard_delete(cv::aruco::CharucoBoard* instance) {
			delete instance;
	}

	// CharucoDetector(const CharucoBoard &, const CharucoParameters &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:42
	// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board", "charucoParams", "detectorParams", "refineParams"], ["const cv::aruco::CharucoBoard*", "const cv::aruco::CharucoParameters*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(const cv::aruco::CharucoBoard* board, const cv::aruco::CharucoParameters* charucoParams, const cv::aruco::DetectorParameters* detectorParams, const cv::aruco::RefineParameters* refineParams, Result<cv::aruco::CharucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::CharucoDetector* ret = new cv::aruco::CharucoDetector(*board, *charucoParams, *detectorParams, *refineParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::CharucoDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:42
	// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
	void cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(const cv::aruco::CharucoBoard* board, Result<cv::aruco::CharucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::CharucoDetector* ret = new cv::aruco::CharucoDetector(*board);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:47
	// ("cv::aruco::CharucoDetector::getBoard", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getBoard_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			const cv::aruco::CharucoBoard ret = instance->getBoard();
			Ok(new const cv::aruco::CharucoBoard(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBoard(const CharucoBoard &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:48
	// ("cv::aruco::CharucoDetector::setBoard", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
	void cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(cv::aruco::CharucoDetector* instance, const cv::aruco::CharucoBoard* board, ResultVoid* ocvrs_return) {
		try {
			instance->setBoard(*board);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCharucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:50
	// ("cv::aruco::CharucoDetector::getCharucoParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getCharucoParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::CharucoParameters*>* ocvrs_return) {
		try {
			const cv::aruco::CharucoParameters ret = instance->getCharucoParameters();
			Ok(new const cv::aruco::CharucoParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCharucoParameters(CharucoParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:51
	// ("cv::aruco::CharucoDetector::setCharucoParameters", vec![(pred!(mut, ["charucoParameters"], ["cv::aruco::CharucoParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(cv::aruco::CharucoDetector* instance, cv::aruco::CharucoParameters* charucoParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setCharucoParameters(*charucoParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:53
	// ("cv::aruco::CharucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getDetectorParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getDetectorParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:54
	// ("cv::aruco::CharucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(cv::aruco::CharucoDetector* instance, const cv::aruco::DetectorParameters* detectorParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectorParameters(*detectorParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:56
	// ("cv::aruco::CharucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getRefineParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			const cv::aruco::RefineParameters ret = instance->getRefineParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:57
	// ("cv::aruco::CharucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(cv::aruco::CharucoDetector* instance, const cv::aruco::RefineParameters* refineParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineParameters(*refineParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectBoard(InputArray, OutputArray, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:84
	// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputOutputArray* markerCorners, const cv::_InputOutputArray* markerIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectBoard(*image, *charucoCorners, *charucoIds, *markerCorners, *markerIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::detectBoard(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:84
	// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectBoard(*image, *charucoCorners, *charucoIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectDiamonds(InputArray, OutputArrayOfArrays, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:108
	// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputOutputArray* markerCorners, const cv::_InputOutputArray* markerIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectDiamonds(*image, *diamondCorners, *diamondIds, *markerCorners, *markerIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::detectDiamonds(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:108
	// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectDiamonds(*image, *diamondCorners, *diamondIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::to_Algorithm() generated
	// ("cv::aruco::CharucoDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_aruco_CharucoDetector_to_Algorithm(cv::aruco::CharucoDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::aruco::CharucoDetector::delete() generated
	// ("cv::aruco::CharucoDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoDetector_delete(cv::aruco::CharucoDetector* instance) {
			delete instance;
	}

	// CharucoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:16
	// ("cv::aruco::CharucoParameters::CharucoParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoParameters_CharucoParameters(Result<cv::aruco::CharucoParameters*>* ocvrs_return) {
		try {
			cv::aruco::CharucoParameters* ret = new cv::aruco::CharucoParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoParameters::implicitClone() generated
	// ("cv::aruco::CharucoParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::CharucoParameters* cv_aruco_CharucoParameters_implicitClone_const(const cv::aruco::CharucoParameters* instance) {
			return new cv::aruco::CharucoParameters(*instance);
	}

	// cv::aruco::CharucoParameters::cameraMatrix() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:21
	// ("cv::aruco::CharucoParameters::cameraMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_CharucoParameters_propCameraMatrix_const(const cv::aruco::CharucoParameters* instance) {
			cv::Mat ret = instance->cameraMatrix;
			return new cv::Mat(ret);
	}

	// cv::aruco::CharucoParameters::setCameraMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:21
	// ("cv::aruco::CharucoParameters::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_CharucoParameters_propCameraMatrix_const_Mat(cv::aruco::CharucoParameters* instance, const cv::Mat* val) {
			instance->cameraMatrix = *val;
	}

	// cv::aruco::CharucoParameters::distCoeffs() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:24
	// ("cv::aruco::CharucoParameters::distCoeffs", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_CharucoParameters_propDistCoeffs_const(const cv::aruco::CharucoParameters* instance) {
			cv::Mat ret = instance->distCoeffs;
			return new cv::Mat(ret);
	}

	// cv::aruco::CharucoParameters::setDistCoeffs(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:24
	// ("cv::aruco::CharucoParameters::setDistCoeffs", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_CharucoParameters_propDistCoeffs_const_Mat(cv::aruco::CharucoParameters* instance, const cv::Mat* val) {
			instance->distCoeffs = *val;
	}

	// cv::aruco::CharucoParameters::minMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:27
	// ("cv::aruco::CharucoParameters::minMarkers", vec![(pred!(const, [], []), _)]),
	int cv_aruco_CharucoParameters_propMinMarkers_const(const cv::aruco::CharucoParameters* instance) {
			int ret = instance->minMarkers;
			return ret;
	}

	// cv::aruco::CharucoParameters::setMinMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:27
	// ("cv::aruco::CharucoParameters::setMinMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_CharucoParameters_propMinMarkers_const_int(cv::aruco::CharucoParameters* instance, const int val) {
			instance->minMarkers = val;
	}

	// cv::aruco::CharucoParameters::tryRefineMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:30
	// ("cv::aruco::CharucoParameters::tryRefineMarkers", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_CharucoParameters_propTryRefineMarkers_const(const cv::aruco::CharucoParameters* instance) {
			bool ret = instance->tryRefineMarkers;
			return ret;
	}

	// cv::aruco::CharucoParameters::setTryRefineMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/charuco_detector.hpp:30
	// ("cv::aruco::CharucoParameters::setTryRefineMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_CharucoParameters_propTryRefineMarkers_const_bool(cv::aruco::CharucoParameters* instance, const bool val) {
			instance->tryRefineMarkers = val;
	}

	// cv::aruco::CharucoParameters::delete() generated
	// ("cv::aruco::CharucoParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoParameters_delete(cv::aruco::CharucoParameters* instance) {
			delete instance;
	}

	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:26
	// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_DetectorParameters(Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:64
	// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(cv::aruco::DetectorParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDetectorParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeDetectorParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:68
	// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(cv::aruco::DetectorParameters* instance, cv::FileStorage* fs, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeDetectorParameters(*fs, std::string(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::DetectorParameters::writeDetectorParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:68
	// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR(cv::aruco::DetectorParameters* instance, cv::FileStorage* fs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeDetectorParameters(*fs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::DetectorParameters::implicitClone() generated
	// ("cv::aruco::DetectorParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::DetectorParameters* cv_aruco_DetectorParameters_implicitClone_const(const cv::aruco::DetectorParameters* instance) {
			return new cv::aruco::DetectorParameters(*instance);
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:71
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:71
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:74
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:74
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:77
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:77
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:80
	// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:80
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->adaptiveThreshConstant = val;
	}

	// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:86
	// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:86
	// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:92
	// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:92
	// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:95
	// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->polygonalApproxAccuracyRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:95
	// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->polygonalApproxAccuracyRate = val;
	}

	// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:98
	// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minCornerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:98
	// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minCornerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:101
	// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMinDistanceToBorder_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minDistanceToBorder;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:101
	// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->minDistanceToBorder = val;
	}

	// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:115
	// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:115
	// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::minGroupDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:121
	// ("cv::aruco::DetectorParameters::minGroupDistance", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propMinGroupDistance_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->minGroupDistance;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinGroupDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:121
	// ("cv::aruco::DetectorParameters::setMinGroupDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propMinGroupDistance_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->minGroupDistance = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:124
	// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMethod_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMethod;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:124
	// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMethod = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:134
	// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementWinSize;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:134
	// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementWinSize = val;
	}

	// cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:145
	// ("cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->relativeCornerRefinmentWinSize;
			return ret;
	}

	// cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:145
	// ("cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->relativeCornerRefinmentWinSize = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:148
	// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMaxIterations;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:148
	// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMaxIterations = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:151
	// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->cornerRefinementMinAccuracy;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:151
	// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->cornerRefinementMinAccuracy = val;
	}

	// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:154
	// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMarkerBorderBits_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->markerBorderBits;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:154
	// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->markerBorderBits = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:157
	// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->perspectiveRemovePixelPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:157
	// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->perspectiveRemovePixelPerCell = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:163
	// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->perspectiveRemoveIgnoredMarginPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:163
	// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->perspectiveRemoveIgnoredMarginPerCell = val;
	}

	// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:169
	// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxErroneousBitsInBorderRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:169
	// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxErroneousBitsInBorderRate = val;
	}

	// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:174
	// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinOtsuStdDev_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minOtsuStdDev;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:174
	// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minOtsuStdDev = val;
	}

	// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:177
	// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propErrorCorrectionRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->errorCorrectionRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:177
	// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->errorCorrectionRate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:184
	// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadDecimate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:184
	// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadDecimate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:187
	// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadSigma;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:187
	// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadSigma = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:191
	// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinClusterPixels;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:191
	// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinClusterPixels = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:194
	// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMaxNmaxima;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:194
	// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMaxNmaxima = val;
	}

	// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:200
	// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagCriticalRad;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:200
	// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagCriticalRad = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:203
	// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagMaxLineFitMse;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:203
	// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagMaxLineFitMse = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:210
	// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinWhiteBlackDiff;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:210
	// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinWhiteBlackDiff = val;
	}

	// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:213
	// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagDeglitch_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagDeglitch;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:213
	// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagDeglitch = val;
	}

	// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:219
	// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_DetectorParameters_propDetectInvertedMarker_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->detectInvertedMarker;
			return ret;
	}

	// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:219
	// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(cv::aruco::DetectorParameters* instance, const bool val) {
			instance->detectInvertedMarker = val;
	}

	// cv::aruco::DetectorParameters::useAruco3Detection() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:227
	// ("cv::aruco::DetectorParameters::useAruco3Detection", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_DetectorParameters_propUseAruco3Detection_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->useAruco3Detection;
			return ret;
	}

	// cv::aruco::DetectorParameters::setUseAruco3Detection(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:227
	// ("cv::aruco::DetectorParameters::setUseAruco3Detection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_DetectorParameters_propUseAruco3Detection_const_bool(cv::aruco::DetectorParameters* instance, const bool val) {
			instance->useAruco3Detection = val;
	}

	// cv::aruco::DetectorParameters::minSideLengthCanonicalImg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:230
	// ("cv::aruco::DetectorParameters::minSideLengthCanonicalImg", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minSideLengthCanonicalImg;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:230
	// ("cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->minSideLengthCanonicalImg = val;
	}

	// cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:233
	// ("cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->minMarkerLengthRatioOriginalImg;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:233
	// ("cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->minMarkerLengthRatioOriginalImg = val;
	}

	// cv::aruco::DetectorParameters::delete() generated
	// ("cv::aruco::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
			delete instance;
	}

	// Dictionary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:36
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_Dictionary(Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:44
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize", "maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_MatR_int_int(const cv::Mat* bytesList, int _markerSize, int maxcorr, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*bytesList, _markerSize, maxcorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::Dictionary(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:44
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize"], ["const cv::Mat*", "int"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_MatR_int(const cv::Mat* bytesList, int _markerSize, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*bytesList, _markerSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:56
	// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_Dictionary_readDictionary_const_FileNodeR(cv::aruco::Dictionary* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDictionary(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeDictionary(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:60
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(cv::aruco::Dictionary* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->writeDictionary(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::writeDictionary(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:60
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_Dictionary_writeDictionary_FileStorageR(cv::aruco::Dictionary* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeDictionary(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:66
	// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
	void cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(const cv::aruco::Dictionary* instance, const cv::Mat* onlyBits, int* idx, int* rotation, double maxCorrectionRate, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->identify(*onlyBits, *idx, *rotation, maxCorrectionRate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:72
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, bool allRotations, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id, allRotations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:72
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImageMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:77
	// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->generateImageMarker(id, sidePixels, *_img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::generateImageMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:77
	// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, ResultVoid* ocvrs_return) {
		try {
			instance->generateImageMarker(id, sidePixels, *_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:82
	// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
	void cv_aruco_Dictionary_getByteListFromBits_const_MatR(const cv::Mat* bits, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*bits);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:87
	// ("cv::aruco::Dictionary::getBitsFromByteList", vec![(pred!(mut, ["byteList", "markerSize"], ["const cv::Mat*", "int"]), _)]),
	void cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(const cv::Mat* byteList, int markerSize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getBitsFromByteList(*byteList, markerSize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::implicitClone() generated
	// ("cv::aruco::Dictionary::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::Dictionary* cv_aruco_Dictionary_implicitClone_const(const cv::aruco::Dictionary* instance) {
			return new cv::aruco::Dictionary(*instance);
	}

	// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:32
	// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_Dictionary_propBytesList_const(const cv::aruco::Dictionary* instance) {
			cv::Mat ret = instance->bytesList;
			return new cv::Mat(ret);
	}

	// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:32
	// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_Dictionary_propBytesList_const_Mat(cv::aruco::Dictionary* instance, const cv::Mat* val) {
			instance->bytesList = *val;
	}

	// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:33
	// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMarkerSize_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->markerSize;
			return ret;
	}

	// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:33
	// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMarkerSize_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->markerSize = val;
	}

	// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:34
	// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMaxCorrectionBits_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->maxCorrectionBits;
			return ret;
	}

	// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_dictionary.hpp:34
	// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMaxCorrectionBits_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->maxCorrectionBits = val;
	}

	// cv::aruco::Dictionary::delete() generated
	// ("cv::aruco::Dictionary::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_delete(cv::aruco::Dictionary* instance) {
			delete instance;
	}

	// GridBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:118
	// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(const cv::Size* size, float markerLength, float markerSeparation, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::GridBoard*>* ocvrs_return) {
		try {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard(*size, markerLength, markerSeparation, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::GridBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:118
	// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(const cv::Size* size, float markerLength, float markerSeparation, const cv::aruco::Dictionary* dictionary, Result<cv::aruco::GridBoard*>* ocvrs_return) {
		try {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard(*size, markerLength, markerSeparation, *dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:121
	// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getGridSize_const(const cv::aruco::GridBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:122
	// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerLength_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:123
	// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerSeparation_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerSeparation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GridBoard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_board.hpp:126
	// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_GridBoard_GridBoard(Result<cv::aruco::GridBoard*>* ocvrs_return) {
		try {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::implicitClone() generated
	// ("cv::aruco::GridBoard::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::GridBoard* cv_aruco_GridBoard_implicitClone_const(const cv::aruco::GridBoard* instance) {
			return new cv::aruco::GridBoard(*instance);
	}

	// cv::aruco::GridBoard::to_Board() generated
	// ("cv::aruco::GridBoard::to_Board", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Board* cv_aruco_GridBoard_to_Board(cv::aruco::GridBoard* instance) {
			return dynamic_cast<cv::aruco::Board*>(instance);
	}

	// cv::aruco::GridBoard::delete() generated
	// ("cv::aruco::GridBoard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_GridBoard_delete(cv::aruco::GridBoard* instance) {
			delete instance;
	}

	// RefineParameters(float, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:239
	// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, ["minRepDistance", "errorCorrectionRate", "checkAllOrders"], ["float", "float", "bool"]), _)]),
	void cv_aruco_RefineParameters_RefineParameters_float_float_bool(float minRepDistance, float errorCorrectionRate, bool checkAllOrders, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			cv::aruco::RefineParameters ret(minRepDistance, errorCorrectionRate, checkAllOrders);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::RefineParameters::RefineParameters() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:239
	// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_RefineParameters_RefineParameters(Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			cv::aruco::RefineParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readRefineParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:244
	// ("cv::aruco::RefineParameters::readRefineParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(cv::aruco::RefineParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readRefineParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeRefineParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:248
	// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(cv::aruco::RefineParameters* instance, cv::FileStorage* fs, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeRefineParameters(*fs, std::string(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::RefineParameters::writeRefineParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/aruco_detector.hpp:248
	// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(cv::aruco::RefineParameters* instance, cv::FileStorage* fs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeRefineParameters(*fs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BarcodeDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:23
	// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_barcode_BarcodeDetector_BarcodeDetector(Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector* ret = new cv::barcode::BarcodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BarcodeDetector(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:30
	// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, ["prototxt_path", "model_path"], ["const std::string*", "const std::string*"]), _)]),
	void cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(const char* prototxt_path, const char* model_path, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector* ret = new cv::barcode::BarcodeDetector(std::string(prototxt_path), std::string(model_path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeWithType(InputArray, InputArray, std::vector<std::string> &, std::vector<std::string> &)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:43
	// ("cv::barcode::BarcodeDetector::decodeWithType", vec![(pred!(const, ["img", "points", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
	void cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeWithType(*img, *points, *decoded_info, *decoded_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeWithType(InputArray, std::vector<std::string> &, std::vector<std::string> &, OutputArray)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:56
	// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type", "points"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeWithType(*img, *decoded_info, *decoded_type, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::barcode::BarcodeDetector::detectAndDecodeWithType(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:56
	// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
	void cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeWithType(*img, *decoded_info, *decoded_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDownsamplingThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:65
	// ("cv::barcode::BarcodeDetector::getDownsamplingThreshold", vec![(pred!(const, [], []), _)]),
	void cv_barcode_BarcodeDetector_getDownsamplingThreshold_const(const cv::barcode::BarcodeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDownsamplingThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDownsamplingThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:75
	// ("cv::barcode::BarcodeDetector::setDownsamplingThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
	void cv_barcode_BarcodeDetector_setDownsamplingThreshold_double(cv::barcode::BarcodeDetector* instance, double thresh, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector ret = instance->setDownsamplingThreshold(thresh);
			Ok(new cv::barcode::BarcodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorScales(std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:81
	// ("cv::barcode::BarcodeDetector::getDetectorScales", vec![(pred!(const, ["sizes"], ["std::vector<float>*"]), _)]),
	void cv_barcode_BarcodeDetector_getDetectorScales_const_vectorLfloatGR(const cv::barcode::BarcodeDetector* instance, std::vector<float>* sizes, ResultVoid* ocvrs_return) {
		try {
			instance->getDetectorScales(*sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorScales(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:90
	// ("cv::barcode::BarcodeDetector::setDetectorScales", vec![(pred!(mut, ["sizes"], ["const std::vector<float>*"]), _)]),
	void cv_barcode_BarcodeDetector_setDetectorScales_const_vectorLfloatGR(cv::barcode::BarcodeDetector* instance, const std::vector<float>* sizes, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector ret = instance->setDetectorScales(*sizes);
			Ok(new cv::barcode::BarcodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:96
	// ("cv::barcode::BarcodeDetector::getGradientThreshold", vec![(pred!(const, [], []), _)]),
	void cv_barcode_BarcodeDetector_getGradientThreshold_const(const cv::barcode::BarcodeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGradientThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/objdetect/barcode.hpp:105
	// ("cv::barcode::BarcodeDetector::setGradientThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
	void cv_barcode_BarcodeDetector_setGradientThreshold_double(cv::barcode::BarcodeDetector* instance, double thresh, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector ret = instance->setGradientThreshold(thresh);
			Ok(new cv::barcode::BarcodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::barcode::BarcodeDetector::implicitClone() generated
	// ("cv::barcode::BarcodeDetector::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::barcode::BarcodeDetector* cv_barcode_BarcodeDetector_implicitClone_const(const cv::barcode::BarcodeDetector* instance) {
			return new cv::barcode::BarcodeDetector(*instance);
	}

	// cv::barcode::BarcodeDetector::to_GraphicalCodeDetector() generated
	// ("cv::barcode::BarcodeDetector::to_GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
	cv::GraphicalCodeDetector* cv_barcode_BarcodeDetector_to_GraphicalCodeDetector(cv::barcode::BarcodeDetector* instance) {
			return dynamic_cast<cv::GraphicalCodeDetector*>(instance);
	}

	// cv::barcode::BarcodeDetector::delete() generated
	// ("cv::barcode::BarcodeDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_barcode_BarcodeDetector_delete(cv::barcode::BarcodeDetector* instance) {
			delete instance;
	}

}
