#include "ocvrs_common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:127
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:127
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, cv::Scalar* cornerColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners, *charucoIds, *cornerColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:148
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:148
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, const cv::_InputArray* diamondIds, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners, *diamondIds, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:379
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* corners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:379
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* corners, const cv::_InputArray* ids, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners, *ids, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::extendDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:146
	// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
	void cv_aruco_extendDictionary_int_int(int nMarkers, int markerSize, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::extendDictionary(nMarkers, markerSize);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extendDictionary(int, int, const Dictionary &, int)(Primitive, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:146
	// ("cv::aruco::extendDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::aruco::Dictionary*", "int"]), _)]),
	void cv_aruco_extendDictionary_int_int_const_DictionaryR_int(int nMarkers, int markerSize, const cv::aruco::Dictionary* baseDictionary, int randomSeed, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::extendDictionary(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::generateImageMarker(TraitClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:392
	// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(const cv::aruco::Dictionary* dictionary, int id, int sidePixels, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::generateImageMarker(*dictionary, id, sidePixels, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImageMarker(const Dictionary &, int, int, OutputArray, int)(TraitClass, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:392
	// ("cv::aruco::generateImageMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::aruco::Dictionary*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* dictionary, int id, int sidePixels, const cv::_OutputArray* img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::generateImageMarker(*dictionary, id, sidePixels, *img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(PredefinedDictionaryType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:127
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PredefinedDictionaryType"]), _)]),
	void cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(cv::aruco::PredefinedDictionaryType name, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::getPredefinedDictionary(name);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:132
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
	void cv_aruco_getPredefinedDictionary_int(int dict, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary ret = cv::aruco::getPredefinedDictionary(dict);
			Ok(new cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:367
	// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_createFaceDetectionMaskGenerator(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:191
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:191
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, cv::Size* winDetSize, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:180
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int(std::vector<cv::Rect>* rectList, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:180
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:185
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:182
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:182
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:188
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:188
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:211
	// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:212
	// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_BaseCascadeClassifier_load_const_StringR(cv::BaseCascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:213
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:219
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:226
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:235
	// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:236
	// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:237
	// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:238
	// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:247
	// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:248
	// ("cv::BaseCascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_getMaskGenerator(cv::BaseCascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BaseCascadeClassifier::to_Algorithm() generated
	// ("cv::BaseCascadeClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BaseCascadeClassifier_to_Algorithm(cv::BaseCascadeClassifier* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BaseCascadeClassifier::delete() generated
	// ("cv::BaseCascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_delete(cv::BaseCascadeClassifier* instance) {
			delete instance;
	}

	// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:244
	// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
	void cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:245
	// ("cv::BaseCascadeClassifier::MaskGenerator::initializeMask", vec![(pred!(mut, ["unnamed"], ["const cv::Mat*"]), _)]),
	void cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->initializeMask(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BaseCascadeClassifier::MaskGenerator::delete() generated
	// ("cv::BaseCascadeClassifier::MaskGenerator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_MaskGenerator_delete(cv::BaseCascadeClassifier::MaskGenerator* instance) {
			delete instance;
	}

	// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:260
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_CascadeClassifier(Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:265
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_CascadeClassifier_const_StringR(const char* filename, Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:269
	// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:276
	// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_load_const_StringR(cv::CascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:281
	// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_CascadeClassifier_read_const_FileNodeR(cv::CascadeClassifier* instance, const cv::FileNode* node, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:297
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:297
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:319
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:319
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:344
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:344
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:354
	// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:355
	// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:356
	// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:357
	// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:359
	// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_CascadeClassifier_convert_const_StringR_const_StringR(const char* oldcascade, const char* newcascade, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::CascadeClassifier::convert(std::string(oldcascade), std::string(newcascade));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:361
	// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:362
	// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:364
	// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BaseCascadeClassifier>* cv_CascadeClassifier_propCc(cv::CascadeClassifier* instance) {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return new cv::Ptr<cv::BaseCascadeClassifier>(ret);
	}

	// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:364
	// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
	void cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier>* val) {
			instance->cc = *val;
	}

	// cv::CascadeClassifier::delete() generated
	// ("cv::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
			delete instance;
	}

	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:121
	// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params, Result<cv::DetectionBasedTracker*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:124
	// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:125
	// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetTracking()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:126
	// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetTracking();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:128
	// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
	void cv_DetectionBasedTracker_process_const_MatR(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray, ResultVoid* ocvrs_return) {
		try {
			instance->process(*imageGray);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:130
	// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_setParameters_const_ParametersR(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setParameters(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:131
	// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance, Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			const cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			Ok(new const cv::DetectionBasedTracker::Parameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:135
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:136
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::Object>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:155
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:158
	// ("cv::DetectionBasedTracker::addObject", vec![(pred!(mut, ["location"], ["const cv::Rect*"]), _)]),
	void cv_DetectionBasedTracker_addObject_const_RectR(cv::DetectionBasedTracker* instance, const cv::Rect* location, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addObject(*location);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::delete() generated
	// ("cv::DetectionBasedTracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_delete(cv::DetectionBasedTracker* instance) {
			delete instance;
	}

	// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:150
	// ("cv::DetectionBasedTracker::ExtObject::ExtObject", vec![(pred!(mut, ["_id", "_location", "_status"], ["int", "cv::Rect", "cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(int _id, cv::Rect* _location, cv::DetectionBasedTracker::ObjectStatus _status, Result<cv::DetectionBasedTracker::ExtObject*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::ExtObject* ret = new cv::DetectionBasedTracker::ExtObject(_id, *_location, _status);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::ExtObject::implicitClone() generated
	// ("cv::DetectionBasedTracker::ExtObject::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::DetectionBasedTracker::ExtObject* cv_DetectionBasedTracker_ExtObject_implicitClone_const(const cv::DetectionBasedTracker::ExtObject* instance) {
			return new cv::DetectionBasedTracker::ExtObject(*instance);
	}

	// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:147
	// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_ExtObject_propId_const(const cv::DetectionBasedTracker::ExtObject* instance) {
			int ret = instance->id;
			return ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:147
	// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propId_const_int(cv::DetectionBasedTracker::ExtObject* instance, const int val) {
			instance->id = val;
	}

	// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:148
	// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->location;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:148
	// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(cv::DetectionBasedTracker::ExtObject* instance, const cv::Rect* val) {
			instance->location = *val;
	}

	// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:149
	// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus* ocvrs_return) {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:149
	// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, const cv::DetectionBasedTracker::ObjectStatus val) {
			instance->status = val;
	}

	// cv::DetectionBasedTracker::ExtObject::delete() generated
	// ("cv::DetectionBasedTracker::ExtObject::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
			delete instance;
	}

	// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:78
	// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:80
	// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min, ResultVoid* ocvrs_return) {
		try {
			instance->setMinObjectSize(*min);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:84
	// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxObjectSize(*max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:88
	// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:92
	// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:96
	// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:100
	// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:104
	// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinNeighbours();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:108
	// ("cv::DetectionBasedTracker::IDetector::setMinNeighbours", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(cv::DetectionBasedTracker::IDetector* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinNeighbours(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::IDetector::delete() generated
	// ("cv::DetectionBasedTracker::IDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_delete(cv::DetectionBasedTracker::IDetector* instance) {
			delete instance;
	}

	// Parameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:65
	// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_Parameters_Parameters(Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:62
	// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->maxTrackLifetime;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:62
	// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(cv::DetectionBasedTracker::Parameters* instance, const int val) {
			instance->maxTrackLifetime = val;
	}

	// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:63
	// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->minDetectionPeriod;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/detection_based_tracker.hpp:63
	// ("cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const_int(cv::DetectionBasedTracker::Parameters* instance, const int val) {
			instance->minDetectionPeriod = val;
	}

	// cv::DetectionBasedTracker::Parameters::delete() generated
	// ("cv::DetectionBasedTracker::Parameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_Parameters_delete(cv::DetectionBasedTracker::Parameters* instance) {
			delete instance;
	}

	// cv::DetectionROI::defaultNew() generated
	// ("cv::DetectionROI::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::DetectionROI* cv_DetectionROI_defaultNew_const() {
			cv::DetectionROI* ret = new cv::DetectionROI();
			return ret;
	}

	// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:378
	// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
	double cv_DetectionROI_propScale_const(const cv::DetectionROI* instance) {
			double ret = instance->scale;
			return ret;
	}

	// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:378
	// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_DetectionROI_propScale_const_double(cv::DetectionROI* instance, const double val) {
			instance->scale = val;
	}

	// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:380
	// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* cv_DetectionROI_propLocations_const(const cv::DetectionROI* instance) {
			std::vector<cv::Point> ret = instance->locations;
			return new std::vector<cv::Point>(ret);
	}

	// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:380
	// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	void cv_DetectionROI_propLocations_const_vectorLPointG(cv::DetectionROI* instance, const std::vector<cv::Point>* val) {
			instance->locations = *val;
	}

	// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:382
	// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
	std::vector<double>* cv_DetectionROI_propConfidences_const(const cv::DetectionROI* instance) {
			std::vector<double> ret = instance->confidences;
			return new std::vector<double>(ret);
	}

	// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:382
	// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	void cv_DetectionROI_propConfidences_const_vectorLdoubleG(cv::DetectionROI* instance, const std::vector<double>* val) {
			instance->confidences = *val;
	}

	// cv::DetectionROI::delete() generated
	// ("cv::DetectionROI::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
			delete instance;
	}

	// setInputSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:29
	// ("cv::FaceDetectorYN::setInputSize", vec![(pred!(mut, ["input_size"], ["const cv::Size*"]), _)]),
	void cv_FaceDetectorYN_setInputSize_const_SizeR(cv::FaceDetectorYN* instance, const cv::Size* input_size, ResultVoid* ocvrs_return) {
		try {
			instance->setInputSize(*input_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInputSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:31
	// ("cv::FaceDetectorYN::getInputSize", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getInputSize(cv::FaceDetectorYN* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getInputSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScoreThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:37
	// ("cv::FaceDetectorYN::setScoreThreshold", vec![(pred!(mut, ["score_threshold"], ["float"]), _)]),
	void cv_FaceDetectorYN_setScoreThreshold_float(cv::FaceDetectorYN* instance, float score_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setScoreThreshold(score_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScoreThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:39
	// ("cv::FaceDetectorYN::getScoreThreshold", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getScoreThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScoreThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMSThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:45
	// ("cv::FaceDetectorYN::setNMSThreshold", vec![(pred!(mut, ["nms_threshold"], ["float"]), _)]),
	void cv_FaceDetectorYN_setNMSThreshold_float(cv::FaceDetectorYN* instance, float nms_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setNMSThreshold(nms_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMSThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:47
	// ("cv::FaceDetectorYN::getNMSThreshold", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getNMSThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNMSThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTopK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:53
	// ("cv::FaceDetectorYN::setTopK", vec![(pred!(mut, ["top_k"], ["int"]), _)]),
	void cv_FaceDetectorYN_setTopK_int(cv::FaceDetectorYN* instance, int top_k, ResultVoid* ocvrs_return) {
		try {
			instance->setTopK(top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTopK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:55
	// ("cv::FaceDetectorYN::getTopK", vec![(pred!(mut, [], []), _)]),
	void cv_FaceDetectorYN_getTopK(cv::FaceDetectorYN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTopK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:72
	// ("cv::FaceDetectorYN::detect", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(cv::FaceDetectorYN* instance, const cv::_InputArray* image, const cv::_OutputArray* faces, Result<int>* ocvrs_return) {
		try {
			int ret = instance->detect(*image, *faces);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, const Size &, float, float, int, int, int)(InString, InString, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:85
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(const char* model, const char* config, const cv::Size* input_size, float score_threshold, float nms_threshold, int top_k, int backend_id, int target_id, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(model), std::string(config), *input_size, score_threshold, nms_threshold, top_k, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceDetectorYN::create(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:85
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["model", "config", "input_size"], ["const cv::String*", "const cv::String*", "const cv::Size*"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR(const char* model, const char* config, const cv::Size* input_size, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(model), std::string(config), *input_size);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, const Size &, float, float, int, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:106
	// ("cv::FaceDetectorYN::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "input_size", "score_threshold", "nms_threshold", "top_k", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "const cv::Size*", "float", "float", "int", "int", "int"]), _)]),
	void cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR_float_float_int_int_int(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, const cv::Size* input_size, float score_threshold, float nms_threshold, int top_k, int backend_id, int target_id, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(framework), *bufferModel, *bufferConfig, *input_size, score_threshold, nms_threshold, top_k, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceDetectorYN::create(InString, CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:106
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

	// alignCrop(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:136
	// ("cv::FaceRecognizerSF::alignCrop", vec![(pred!(const, ["src_img", "face_box", "aligned_img"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::FaceRecognizerSF* instance, const cv::_InputArray* src_img, const cv::_InputArray* face_box, const cv::_OutputArray* aligned_img, ResultVoid* ocvrs_return) {
		try {
			instance->alignCrop(*src_img, *face_box, *aligned_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:142
	// ("cv::FaceRecognizerSF::feature", vec![(pred!(mut, ["aligned_img", "face_feature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(cv::FaceRecognizerSF* instance, const cv::_InputArray* aligned_img, const cv::_OutputArray* face_feature, ResultVoid* ocvrs_return) {
		try {
			instance->feature(*aligned_img, *face_feature);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:149
	// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2", "dis_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(const cv::FaceRecognizerSF* instance, const cv::_InputArray* face_feature1, const cv::_InputArray* face_feature2, int dis_type, Result<double>* ocvrs_return) {
		try {
			double ret = instance->match(*face_feature1, *face_feature2, dis_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::match(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:149
	// ("cv::FaceRecognizerSF::match", vec![(pred!(const, ["face_feature1", "face_feature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(const cv::FaceRecognizerSF* instance, const cv::_InputArray* face_feature1, const cv::_InputArray* face_feature2, Result<double>* ocvrs_return) {
		try {
			double ret = instance->match(*face_feature1, *face_feature2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, int, int)(InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:157
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config", "backend_id", "target_id"], ["const cv::String*", "const cv::String*", "int", "int"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(const char* model, const char* config, int backend_id, int target_id, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(model), std::string(config), backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::create(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:157
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_StringR(const char* model, const char* config, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(model), std::string(config));
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const std::vector<uchar> &, const std::vector<uchar> &, int, int)(InString, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:169
	// ("cv::FaceRecognizerSF::create", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig", "backend_id", "target_id"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*", "int", "int"]), _)]),
	void cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_int_int(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, int backend_id, int target_id, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(framework), *bufferModel, *bufferConfig, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FaceRecognizerSF::create(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/face.hpp:169
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

	// GraphicalCodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:17
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_GraphicalCodeDetector_GraphicalCodeDetector(Result<cv::GraphicalCodeDetector*>* ocvrs_return) {
		try {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GraphicalCodeDetector(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:19
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
	cv::GraphicalCodeDetector* cv_GraphicalCodeDetector_GraphicalCodeDetector_const_GraphicalCodeDetectorR(const cv::GraphicalCodeDetector* unnamed) {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector(*unnamed);
			return ret;
	}

	// GraphicalCodeDetector(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:20
	// ("cv::GraphicalCodeDetector::GraphicalCodeDetector", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
	cv::GraphicalCodeDetector* cv_GraphicalCodeDetector_GraphicalCodeDetector_GraphicalCodeDetectorRR(cv::GraphicalCodeDetector* unnamed) {
			cv::GraphicalCodeDetector* ret = new cv::GraphicalCodeDetector(std::move(*unnamed));
			return ret;
	}

	// operator=(const GraphicalCodeDetector &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:21
	// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::GraphicalCodeDetector*"]), _)]),
	void cv_GraphicalCodeDetector_operatorST_const_GraphicalCodeDetectorR(cv::GraphicalCodeDetector* instance, const cv::GraphicalCodeDetector* unnamed) {
			instance->operator=(*unnamed);
	}

	// operator=(GraphicalCodeDetector &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:22
	// ("cv::GraphicalCodeDetector::operator=", vec![(pred!(mut, ["unnamed"], ["cv::GraphicalCodeDetector*"]), _)]),
	void cv_GraphicalCodeDetector_operatorST_GraphicalCodeDetectorRR(cv::GraphicalCodeDetector* instance, cv::GraphicalCodeDetector* unnamed) {
			instance->operator=(std::move(*unnamed));
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:28
	// ("cv::GraphicalCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detect(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:37
	// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_code, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->decode(*img, *points, *straight_code);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:37
	// ("cv::GraphicalCodeDetector::decode", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->decode(*img, *points);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:45
	// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img", "points", "straight_code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_code, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecode(*img, *points, *straight_code);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:45
	// ("cv::GraphicalCodeDetector::detectAndDecode", vec![(pred!(const, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecode(*img);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:53
	// ("cv::GraphicalCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectMulti(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:61
	// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_code"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, const cv::_OutputArray* straight_code, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_code);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:61
	// ("cv::GraphicalCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	void cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeMulti(InputArray, std::vector<std::string> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:74
	// ("cv::GraphicalCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_code"], ["const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(const cv::GraphicalCodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, const cv::_OutputArray* points, const cv::_OutputArray* straight_code, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info, *points, *straight_code);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GraphicalCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/graphical_code_detector.hpp:74
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

	// HOGDescriptor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:415
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_HOGDescriptor(Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(Size, Size, Size, Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Enum, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:435
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "cv::HOGDescriptor::HistogramNormType", "double", "bool", "int", "bool"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, cv::HOGDescriptor::HistogramNormType _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::HOGDescriptor(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:435
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:451
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_StringR(const char* filename, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:459
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(const cv::HOGDescriptor* d, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:470
	// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:474
	// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkDetectorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:478
	// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWinSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:485
	// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["svmdetector"], ["const cv::_InputArray*"]), _)]),
	void cv_HOGDescriptor_setSVMDetector_const__InputArrayR(cv::HOGDescriptor* instance, const cv::_InputArray* svmdetector, ResultVoid* ocvrs_return) {
		try {
			instance->setSVMDetector(*svmdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:490
	// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
	void cv_HOGDescriptor_read_FileNodeR(cv::HOGDescriptor* instance, cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:496
	// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_write_const_FileStorageR_const_StringR(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:502
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR_const_StringR(cv::HOGDescriptor* instance, const char* filename, const char* objname, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename), std::string(objname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:502
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR(cv::HOGDescriptor* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR_const_StringR(const cv::HOGDescriptor* instance, const char* filename, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->save(std::string(filename), std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR(const cv::HOGDescriptor* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:513
	// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_copyTo_const_HOGDescriptorR(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:540
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:540
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:556
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:556
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:576
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:576
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeGradient(InputArray, InputOutputArray, InputOutputArray, Size, Size)(InputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, cv::Size* paddingTL, cv::Size* paddingBR, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::computeGradient(InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:612
	// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDefaultPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:618
	// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDaimlerPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectROI(InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:676
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, cv::Size* winStride, cv::Size* padding, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:676
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScaleROI(InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:689
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR_double_int(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScaleROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:689
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:701
	// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:621
	// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propWinSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:621
	// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propWinSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->winSize = *val;
	}

	// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:624
	// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:624
	// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockSize = *val;
	}

	// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:627
	// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockStride_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockStride;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:627
	// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockStride_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockStride = *val;
	}

	// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:630
	// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propCellSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->cellSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:630
	// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propCellSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->cellSize = *val;
	}

	// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:633
	// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNbins_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nbins;
			return ret;
	}

	// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:633
	// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNbins_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nbins = val;
	}

	// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:636
	// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propDerivAperture_const(const cv::HOGDescriptor* instance) {
			int ret = instance->derivAperture;
			return ret;
	}

	// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:636
	// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propDerivAperture_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->derivAperture = val;
	}

	// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:639
	// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propWinSigma_const(const cv::HOGDescriptor* instance) {
			double ret = instance->winSigma;
			return ret;
	}

	// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:639
	// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propWinSigma_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->winSigma = val;
	}

	// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:642
	// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propHistogramNormType_const(const cv::HOGDescriptor* instance, cv::HOGDescriptor::HistogramNormType* ocvrs_return) {
			cv::HOGDescriptor::HistogramNormType ret = instance->histogramNormType;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setHistogramNormType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:642
	// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const cv::HOGDescriptor::HistogramNormType"]), _)]),
	void cv_HOGDescriptor_propHistogramNormType_const_HistogramNormType(cv::HOGDescriptor* instance, const cv::HOGDescriptor::HistogramNormType val) {
			instance->histogramNormType = val;
	}

	// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:645
	// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propL2HysThreshold_const(const cv::HOGDescriptor* instance) {
			double ret = instance->L2HysThreshold;
			return ret;
	}

	// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:645
	// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propL2HysThreshold_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->L2HysThreshold = val;
	}

	// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:648
	// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propGammaCorrection_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->gammaCorrection;
			return ret;
	}

	// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:648
	// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propGammaCorrection_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->gammaCorrection = val;
	}

	// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:651
	// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_HOGDescriptor_propSvmDetector_const(const cv::HOGDescriptor* instance) {
			std::vector<float> ret = instance->svmDetector;
			return new std::vector<float>(ret);
	}

	// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:651
	// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(cv::HOGDescriptor* instance, const std::vector<float>* val) {
			instance->svmDetector = *val;
	}

	// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:654
	// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
	cv::UMat* cv_HOGDescriptor_propOclSvmDetector_const(const cv::HOGDescriptor* instance) {
			cv::UMat ret = instance->oclSvmDetector;
			return new cv::UMat(ret);
	}

	// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:654
	// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	void cv_HOGDescriptor_propOclSvmDetector_const_UMat(cv::HOGDescriptor* instance, const cv::UMat* val) {
			instance->oclSvmDetector = *val;
	}

	// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
	float cv_HOGDescriptor_propFree_coef_const(const cv::HOGDescriptor* instance) {
			float ret = instance->free_coef;
			return ret;
	}

	// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_HOGDescriptor_propFree_coef_const_float(cv::HOGDescriptor* instance, const float val) {
			instance->free_coef = val;
	}

	// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:660
	// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNlevels_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nlevels;
			return ret;
	}

	// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:660
	// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNlevels_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nlevels = val;
	}

	// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:663
	// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propSignedGradient_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->signedGradient;
			return ret;
	}

	// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:663
	// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propSignedGradient_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->signedGradient = val;
	}

	// cv::HOGDescriptor::delete() generated
	// ("cv::HOGDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
			delete instance;
	}

	// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:775
	// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetector_QRCodeDetector(Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:781
	// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setEpsX(epsX);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:786
	// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setEpsY(epsY);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseAlignmentMarkers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:792
	// ("cv::QRCodeDetector::setUseAlignmentMarkers", vec![(pred!(mut, ["useAlignmentMarkers"], ["bool"]), _)]),
	void cv_QRCodeDetector_setUseAlignmentMarkers_bool(cv::QRCodeDetector* instance, bool useAlignmentMarkers, Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector ret = instance->setUseAlignmentMarkers(useAlignmentMarkers);
			Ok(new cv::QRCodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:801
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:801
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:809
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:809
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

	// QRCodeDetectorAruco()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:815
	// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetectorAruco_QRCodeDetectorAruco(Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco* ret = new cv::QRCodeDetectorAruco();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// QRCodeDetectorAruco(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:850
	// ("cv::QRCodeDetectorAruco::QRCodeDetectorAruco", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
	void cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(const cv::QRCodeDetectorAruco::Params* params, Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco* ret = new cv::QRCodeDetectorAruco(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:853
	// ("cv::QRCodeDetectorAruco::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_QRCodeDetectorAruco_getDetectorParameters_const(const cv::QRCodeDetectorAruco* instance, Result<cv::QRCodeDetectorAruco::Params>* ocvrs_return) {
		try {
			const cv::QRCodeDetectorAruco::Params ret = instance->getDetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const QRCodeDetectorAruco::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:856
	// ("cv::QRCodeDetectorAruco::setDetectorParameters", vec![(pred!(mut, ["params"], ["const cv::QRCodeDetectorAruco::Params*"]), _)]),
	void cv_QRCodeDetectorAruco_setDetectorParameters_const_ParamsR(cv::QRCodeDetectorAruco* instance, const cv::QRCodeDetectorAruco::Params* params, Result<cv::QRCodeDetectorAruco*>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco ret = instance->setDetectorParameters(*params);
			Ok(new cv::QRCodeDetectorAruco(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getArucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:859
	// ("cv::QRCodeDetectorAruco::getArucoParameters", vec![(pred!(const, [], []), _)]),
	void cv_QRCodeDetectorAruco_getArucoParameters_const(const cv::QRCodeDetectorAruco* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getArucoParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setArucoParameters(const aruco::DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:862
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

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:818
	// ("cv::QRCodeDetectorAruco::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetectorAruco_Params_Params(Result<cv::QRCodeDetectorAruco::Params>* ocvrs_return) {
		try {
			cv::QRCodeDetectorAruco::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:757
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
	void cv_QRCodeEncoder_create_const_ParamsR(const cv::QRCodeEncoder::Params* parameters, Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create(*parameters);
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:757
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_create(Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create();
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:763
	// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcode, ResultVoid* ocvrs_return) {
		try {
			instance->encode(std::string(encoded_info), *qrcode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:769
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

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:738
	// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_Params_Params(Result<cv::QRCodeEncoder::Params>* ocvrs_return) {
		try {
			cv::QRCodeEncoder::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:153
	// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
	void cv_SimilarRects_SimilarRects_double(double _eps, Result<cv::SimilarRects*>* ocvrs_return) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:154
	// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
	void cv_SimilarRects_operator___const_const_RectR_const_RectR(const cv::SimilarRects* instance, const cv::Rect* r1, const cv::Rect* r2, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator()(*r1, *r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:162
	// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
	double cv_SimilarRects_propEps_const(const cv::SimilarRects* instance) {
			double ret = instance->eps;
			return ret;
	}

	// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect.hpp:162
	// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_SimilarRects_propEps_const_double(cv::SimilarRects* instance, const double val) {
			instance->eps = val;
	}

	// cv::SimilarRects::delete() generated
	// ("cv::SimilarRects::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
			delete instance;
	}

	// ArucoDetector(const Dictionary &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:284
	// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, ["dictionary", "detectorParams", "refineParams"], ["const cv::aruco::Dictionary*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(const cv::aruco::Dictionary* dictionary, const cv::aruco::DetectorParameters* detectorParams, const cv::aruco::RefineParameters* refineParams, Result<cv::aruco::ArucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::ArucoDetector* ret = new cv::aruco::ArucoDetector(*dictionary, *detectorParams, *refineParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::ArucoDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:284
	// ("cv::aruco::ArucoDetector::ArucoDetector", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_ArucoDetector_ArucoDetector(Result<cv::aruco::ArucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::ArucoDetector* ret = new cv::aruco::ArucoDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMarkers(InputArray, OutputArrayOfArrays, OutputArray, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:308
	// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::_OutputArray* rejectedImgPoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectMarkers(*image, *corners, *ids, *rejectedImgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::detectMarkers(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:308
	// ("cv::aruco::ArucoDetector::detectMarkers", vec![(pred!(const, ["image", "corners", "ids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_OutputArray* ids, ResultVoid* ocvrs_return) {
		try {
			instance->detectMarkers(*image, *corners, *ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// refineDetectedMarkers(InputArray, const Board &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, OutputArray)(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:333
	// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "recoveredIdxs"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::aruco::Board* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* recoveredIdxs, ResultVoid* ocvrs_return) {
		try {
			instance->refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, *recoveredIdxs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::ArucoDetector::refineDetectedMarkers(InputArray, TraitClass, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:333
	// ("cv::aruco::ArucoDetector::refineDetectedMarkers", vec![(pred!(const, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::aruco::Board*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::ArucoDetector* instance, const cv::_InputArray* image, const cv::aruco::Board* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, ResultVoid* ocvrs_return) {
		try {
			instance->refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:339
	// ("cv::aruco::ArucoDetector::getDictionary", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getDictionary_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			const cv::aruco::Dictionary ret = instance->getDictionary();
			Ok(new const cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDictionary(const Dictionary &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:340
	// ("cv::aruco::ArucoDetector::setDictionary", vec![(pred!(mut, ["dictionary"], ["const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(cv::aruco::ArucoDetector* instance, const cv::aruco::Dictionary* dictionary, ResultVoid* ocvrs_return) {
		try {
			instance->setDictionary(*dictionary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:342
	// ("cv::aruco::ArucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getDetectorParameters_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getDetectorParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:343
	// ("cv::aruco::ArucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
	void cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(cv::aruco::ArucoDetector* instance, const cv::aruco::DetectorParameters* detectorParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectorParameters(*detectorParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:345
	// ("cv::aruco::ArucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_ArucoDetector_getRefineParameters_const(const cv::aruco::ArucoDetector* instance, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			const cv::aruco::RefineParameters ret = instance->getRefineParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:346
	// ("cv::aruco::ArucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(cv::aruco::ArucoDetector* instance, const cv::aruco::RefineParameters* refineParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineParameters(*refineParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:350
	// ("cv::aruco::ArucoDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_ArucoDetector_write_const_FileStorageR(const cv::aruco::ArucoDetector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:354
	// ("cv::aruco::ArucoDetector::write", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(cv::aruco::ArucoDetector* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:358
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

	// Board(InputArrayOfArrays, const Dictionary &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:33
	// ("cv::aruco::Board::Board", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(const cv::_InputArray* objPoints, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::Board*>* ocvrs_return) {
		try {
			cv::aruco::Board* ret = new cv::aruco::Board(*objPoints, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:37
	// ("cv::aruco::Board::getDictionary", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getDictionary_const(const cv::aruco::Board* instance, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			const cv::aruco::Dictionary ret = instance->getDictionary();
			Ok(new const cv::aruco::Dictionary(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjPoints()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:49
	// ("cv::aruco::Board::getObjPoints", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getObjPoints_const(const cv::aruco::Board* instance, Result<std::vector<std::vector<cv::Point3f>>*>* ocvrs_return) {
		try {
			const std::vector<std::vector<cv::Point3f>> ret = instance->getObjPoints();
			Ok(new const std::vector<std::vector<cv::Point3f>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:54
	// ("cv::aruco::Board::getIds", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getIds_const(const cv::aruco::Board* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getIds();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRightBottomCorner()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:58
	// ("cv::aruco::Board::getRightBottomCorner", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_getRightBottomCorner_const(const cv::aruco::Board* instance, Result<cv::Point3f>* ocvrs_return) {
		try {
			const cv::Point3f ret = instance->getRightBottomCorner();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchImagePoints(InputArrayOfArrays, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:78
	// ("cv::aruco::Board::matchImagePoints", vec![(pred!(const, ["detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::Board* instance, const cv::_InputArray* detectedCorners, const cv::_InputArray* detectedIds, const cv::_OutputArray* objPoints, const cv::_OutputArray* imgPoints, ResultVoid* ocvrs_return) {
		try {
			instance->matchImagePoints(*detectedCorners, *detectedIds, *objPoints, *imgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImage(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:91
	// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(const cv::aruco::Board* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->generateImage(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Board::generateImage(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:91
	// ("cv::aruco::Board::generateImage", vec![(pred!(const, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(const cv::aruco::Board* instance, cv::Size* outSize, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->generateImage(*outSize, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Board()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:94
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

	// CharucoBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:146
	// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(const cv::Size* size, float squareLength, float markerLength, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard(*size, squareLength, markerLength, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoBoard::CharucoBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:146
	// ("cv::aruco::CharucoBoard::CharucoBoard", vec![(pred!(mut, ["size", "squareLength", "markerLength", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(const cv::Size* size, float squareLength, float markerLength, const cv::aruco::Dictionary* dictionary, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard(*size, squareLength, markerLength, *dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLegacyPattern(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:158
	// ("cv::aruco::CharucoBoard::setLegacyPattern", vec![(pred!(mut, ["legacyPattern"], ["bool"]), _)]),
	void cv_aruco_CharucoBoard_setLegacyPattern_bool(cv::aruco::CharucoBoard* instance, bool legacyPattern, ResultVoid* ocvrs_return) {
		try {
			instance->setLegacyPattern(legacyPattern);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLegacyPattern()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:159
	// ("cv::aruco::CharucoBoard::getLegacyPattern", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getLegacyPattern_const(const cv::aruco::CharucoBoard* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getLegacyPattern();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:161
	// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getChessboardSize_const(const cv::aruco::CharucoBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getChessboardSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSquareLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:162
	// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getSquareLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSquareLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:163
	// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getMarkerLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChessboardCorners()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:167
	// ("cv::aruco::CharucoBoard::getChessboardCorners", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getChessboardCorners_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<cv::Point3f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point3f> ret = instance->getChessboardCorners();
			Ok(new std::vector<cv::Point3f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNearestMarkerIdx()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:171
	// ("cv::aruco::CharucoBoard::getNearestMarkerIdx", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getNearestMarkerIdx_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<std::vector<int>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<int>> ret = instance->getNearestMarkerIdx();
			Ok(new std::vector<std::vector<int>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNearestMarkerCorners()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:175
	// ("cv::aruco::CharucoBoard::getNearestMarkerCorners", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getNearestMarkerCorners_const(const cv::aruco::CharucoBoard* instance, Result<std::vector<std::vector<int>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<int>> ret = instance->getNearestMarkerCorners();
			Ok(new std::vector<std::vector<int>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkCharucoCornersCollinear(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:188
	// ("cv::aruco::CharucoBoard::checkCharucoCornersCollinear", vec![(pred!(const, ["charucoIds"], ["const cv::_InputArray*"]), _)]),
	void cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(const cv::aruco::CharucoBoard* instance, const cv::_InputArray* charucoIds, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkCharucoCornersCollinear(*charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CharucoBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:191
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

	// CharucoDetector(const CharucoBoard &, const CharucoParameters &, const DetectorParameters &, const RefineParameters &)(TraitClass, TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:42
	// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board", "charucoParams", "detectorParams", "refineParams"], ["const cv::aruco::CharucoBoard*", "const cv::aruco::CharucoParameters*", "const cv::aruco::DetectorParameters*", "const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(const cv::aruco::CharucoBoard* board, const cv::aruco::CharucoParameters* charucoParams, const cv::aruco::DetectorParameters* detectorParams, const cv::aruco::RefineParameters* refineParams, Result<cv::aruco::CharucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::CharucoDetector* ret = new cv::aruco::CharucoDetector(*board, *charucoParams, *detectorParams, *refineParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::CharucoDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:42
	// ("cv::aruco::CharucoDetector::CharucoDetector", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
	void cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(const cv::aruco::CharucoBoard* board, Result<cv::aruco::CharucoDetector*>* ocvrs_return) {
		try {
			cv::aruco::CharucoDetector* ret = new cv::aruco::CharucoDetector(*board);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:47
	// ("cv::aruco::CharucoDetector::getBoard", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getBoard_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::CharucoBoard*>* ocvrs_return) {
		try {
			const cv::aruco::CharucoBoard ret = instance->getBoard();
			Ok(new const cv::aruco::CharucoBoard(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBoard(const CharucoBoard &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:48
	// ("cv::aruco::CharucoDetector::setBoard", vec![(pred!(mut, ["board"], ["const cv::aruco::CharucoBoard*"]), _)]),
	void cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(cv::aruco::CharucoDetector* instance, const cv::aruco::CharucoBoard* board, ResultVoid* ocvrs_return) {
		try {
			instance->setBoard(*board);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCharucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:50
	// ("cv::aruco::CharucoDetector::getCharucoParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getCharucoParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::CharucoParameters*>* ocvrs_return) {
		try {
			const cv::aruco::CharucoParameters ret = instance->getCharucoParameters();
			Ok(new const cv::aruco::CharucoParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCharucoParameters(CharucoParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:51
	// ("cv::aruco::CharucoDetector::setCharucoParameters", vec![(pred!(mut, ["charucoParameters"], ["cv::aruco::CharucoParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(cv::aruco::CharucoDetector* instance, cv::aruco::CharucoParameters* charucoParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setCharucoParameters(*charucoParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:53
	// ("cv::aruco::CharucoDetector::getDetectorParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getDetectorParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			const cv::aruco::DetectorParameters ret = instance->getDetectorParameters();
			Ok(new const cv::aruco::DetectorParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorParameters(const DetectorParameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:54
	// ("cv::aruco::CharucoDetector::setDetectorParameters", vec![(pred!(mut, ["detectorParameters"], ["const cv::aruco::DetectorParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(cv::aruco::CharucoDetector* instance, const cv::aruco::DetectorParameters* detectorParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectorParameters(*detectorParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRefineParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:56
	// ("cv::aruco::CharucoDetector::getRefineParameters", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoDetector_getRefineParameters_const(const cv::aruco::CharucoDetector* instance, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			const cv::aruco::RefineParameters ret = instance->getRefineParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefineParameters(const RefineParameters &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:57
	// ("cv::aruco::CharucoDetector::setRefineParameters", vec![(pred!(mut, ["refineParameters"], ["const cv::aruco::RefineParameters*"]), _)]),
	void cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(cv::aruco::CharucoDetector* instance, const cv::aruco::RefineParameters* refineParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineParameters(*refineParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectBoard(InputArray, OutputArray, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:84
	// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputOutputArray* markerCorners, const cv::_InputOutputArray* markerIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectBoard(*image, *charucoCorners, *charucoIds, *markerCorners, *markerIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::detectBoard(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:84
	// ("cv::aruco::CharucoDetector::detectBoard", vec![(pred!(const, ["image", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectBoard(*image, *charucoCorners, *charucoIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectDiamonds(InputArray, OutputArrayOfArrays, OutputArray, InputOutputArrayOfArrays, InputOutputArray)(InputArray, OutputArray, OutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:108
	// ("cv::aruco::CharucoDetector::detectDiamonds", vec![(pred!(const, ["image", "diamondCorners", "diamondIds", "markerCorners", "markerIds"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::aruco::CharucoDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputOutputArray* markerCorners, const cv::_InputOutputArray* markerIds, ResultVoid* ocvrs_return) {
		try {
			instance->detectDiamonds(*image, *diamondCorners, *diamondIds, *markerCorners, *markerIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoDetector::detectDiamonds(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:108
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

	// CharucoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:16
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

	// cv::aruco::CharucoParameters::cameraMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:21
	// ("cv::aruco::CharucoParameters::cameraMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_CharucoParameters_propCameraMatrix_const(const cv::aruco::CharucoParameters* instance) {
			cv::Mat ret = instance->cameraMatrix;
			return new cv::Mat(ret);
	}

	// cv::aruco::CharucoParameters::setCameraMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:21
	// ("cv::aruco::CharucoParameters::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_CharucoParameters_propCameraMatrix_const_Mat(cv::aruco::CharucoParameters* instance, const cv::Mat* val) {
			instance->cameraMatrix = *val;
	}

	// cv::aruco::CharucoParameters::distCoeffs() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:24
	// ("cv::aruco::CharucoParameters::distCoeffs", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_CharucoParameters_propDistCoeffs_const(const cv::aruco::CharucoParameters* instance) {
			cv::Mat ret = instance->distCoeffs;
			return new cv::Mat(ret);
	}

	// cv::aruco::CharucoParameters::setDistCoeffs(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:24
	// ("cv::aruco::CharucoParameters::setDistCoeffs", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_CharucoParameters_propDistCoeffs_const_Mat(cv::aruco::CharucoParameters* instance, const cv::Mat* val) {
			instance->distCoeffs = *val;
	}

	// cv::aruco::CharucoParameters::minMarkers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:27
	// ("cv::aruco::CharucoParameters::minMarkers", vec![(pred!(const, [], []), _)]),
	int cv_aruco_CharucoParameters_propMinMarkers_const(const cv::aruco::CharucoParameters* instance) {
			int ret = instance->minMarkers;
			return ret;
	}

	// cv::aruco::CharucoParameters::setMinMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:27
	// ("cv::aruco::CharucoParameters::setMinMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_CharucoParameters_propMinMarkers_const_int(cv::aruco::CharucoParameters* instance, const int val) {
			instance->minMarkers = val;
	}

	// cv::aruco::CharucoParameters::tryRefineMarkers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:30
	// ("cv::aruco::CharucoParameters::tryRefineMarkers", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_CharucoParameters_propTryRefineMarkers_const(const cv::aruco::CharucoParameters* instance) {
			bool ret = instance->tryRefineMarkers;
			return ret;
	}

	// cv::aruco::CharucoParameters::setTryRefineMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/charuco_detector.hpp:30
	// ("cv::aruco::CharucoParameters::setTryRefineMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_CharucoParameters_propTryRefineMarkers_const_bool(cv::aruco::CharucoParameters* instance, const bool val) {
			instance->tryRefineMarkers = val;
	}

	// cv::aruco::CharucoParameters::delete() generated
	// ("cv::aruco::CharucoParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoParameters_delete(cv::aruco::CharucoParameters* instance) {
			delete instance;
	}

	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:26
	// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_DetectorParameters(Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:64
	// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(cv::aruco::DetectorParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDetectorParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeDetectorParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:68
	// ("cv::aruco::DetectorParameters::writeDetectorParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(cv::aruco::DetectorParameters* instance, cv::FileStorage* fs, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeDetectorParameters(*fs, std::string(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::DetectorParameters::writeDetectorParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:68
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

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:71
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:71
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:74
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:74
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:77
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:77
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:80
	// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:80
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->adaptiveThreshConstant = val;
	}

	// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:86
	// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:86
	// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:92
	// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:92
	// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:95
	// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->polygonalApproxAccuracyRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:95
	// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->polygonalApproxAccuracyRate = val;
	}

	// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:98
	// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minCornerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:98
	// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minCornerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:101
	// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMinDistanceToBorder_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minDistanceToBorder;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:101
	// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->minDistanceToBorder = val;
	}

	// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:115
	// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:115
	// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::minGroupDistance() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:121
	// ("cv::aruco::DetectorParameters::minGroupDistance", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propMinGroupDistance_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->minGroupDistance;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinGroupDistance(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:121
	// ("cv::aruco::DetectorParameters::setMinGroupDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propMinGroupDistance_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->minGroupDistance = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:124
	// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMethod_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMethod;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:124
	// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMethod = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:134
	// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementWinSize;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:134
	// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementWinSize = val;
	}

	// cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:145
	// ("cv::aruco::DetectorParameters::relativeCornerRefinmentWinSize", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->relativeCornerRefinmentWinSize;
			return ret;
	}

	// cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:145
	// ("cv::aruco::DetectorParameters::setRelativeCornerRefinmentWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->relativeCornerRefinmentWinSize = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:148
	// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMaxIterations;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:148
	// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMaxIterations = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:151
	// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->cornerRefinementMinAccuracy;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:151
	// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->cornerRefinementMinAccuracy = val;
	}

	// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:154
	// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMarkerBorderBits_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->markerBorderBits;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:154
	// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->markerBorderBits = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:157
	// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->perspectiveRemovePixelPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:157
	// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->perspectiveRemovePixelPerCell = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:163
	// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->perspectiveRemoveIgnoredMarginPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:163
	// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->perspectiveRemoveIgnoredMarginPerCell = val;
	}

	// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:169
	// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxErroneousBitsInBorderRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:169
	// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxErroneousBitsInBorderRate = val;
	}

	// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:174
	// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinOtsuStdDev_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minOtsuStdDev;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:174
	// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minOtsuStdDev = val;
	}

	// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:177
	// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propErrorCorrectionRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->errorCorrectionRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:177
	// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->errorCorrectionRate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:184
	// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadDecimate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:184
	// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadDecimate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:187
	// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadSigma;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:187
	// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadSigma = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:191
	// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinClusterPixels;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:191
	// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinClusterPixels = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:194
	// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMaxNmaxima;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:194
	// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMaxNmaxima = val;
	}

	// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:200
	// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagCriticalRad;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:200
	// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagCriticalRad = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:203
	// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagMaxLineFitMse;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:203
	// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagMaxLineFitMse = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:210
	// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinWhiteBlackDiff;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:210
	// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinWhiteBlackDiff = val;
	}

	// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:213
	// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagDeglitch_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagDeglitch;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:213
	// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagDeglitch = val;
	}

	// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:219
	// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_DetectorParameters_propDetectInvertedMarker_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->detectInvertedMarker;
			return ret;
	}

	// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:219
	// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(cv::aruco::DetectorParameters* instance, const bool val) {
			instance->detectInvertedMarker = val;
	}

	// cv::aruco::DetectorParameters::useAruco3Detection() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:227
	// ("cv::aruco::DetectorParameters::useAruco3Detection", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_DetectorParameters_propUseAruco3Detection_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->useAruco3Detection;
			return ret;
	}

	// cv::aruco::DetectorParameters::setUseAruco3Detection(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:227
	// ("cv::aruco::DetectorParameters::setUseAruco3Detection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_DetectorParameters_propUseAruco3Detection_const_bool(cv::aruco::DetectorParameters* instance, const bool val) {
			instance->useAruco3Detection = val;
	}

	// cv::aruco::DetectorParameters::minSideLengthCanonicalImg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:230
	// ("cv::aruco::DetectorParameters::minSideLengthCanonicalImg", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minSideLengthCanonicalImg;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:230
	// ("cv::aruco::DetectorParameters::setMinSideLengthCanonicalImg", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->minSideLengthCanonicalImg = val;
	}

	// cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:233
	// ("cv::aruco::DetectorParameters::minMarkerLengthRatioOriginalImg", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->minMarkerLengthRatioOriginalImg;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:233
	// ("cv::aruco::DetectorParameters::setMinMarkerLengthRatioOriginalImg", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->minMarkerLengthRatioOriginalImg = val;
	}

	// cv::aruco::DetectorParameters::delete() generated
	// ("cv::aruco::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
			delete instance;
	}

	// Dictionary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:36
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_Dictionary(Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:44
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize", "maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_MatR_int_int(const cv::Mat* bytesList, int _markerSize, int maxcorr, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*bytesList, _markerSize, maxcorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::Dictionary(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:44
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["bytesList", "_markerSize"], ["const cv::Mat*", "int"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_MatR_int(const cv::Mat* bytesList, int _markerSize, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*bytesList, _markerSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:56
	// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_Dictionary_readDictionary_const_FileNodeR(cv::aruco::Dictionary* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDictionary(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeDictionary(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:60
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(cv::aruco::Dictionary* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->writeDictionary(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::writeDictionary(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:60
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_Dictionary_writeDictionary_FileStorageR(cv::aruco::Dictionary* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeDictionary(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:66
	// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
	void cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(const cv::aruco::Dictionary* instance, const cv::Mat* onlyBits, int* idx, int* rotation, double maxCorrectionRate, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->identify(*onlyBits, *idx, *rotation, maxCorrectionRate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:72
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, bool allRotations, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id, allRotations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:72
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateImageMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:77
	// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->generateImageMarker(id, sidePixels, *_img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::generateImageMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:77
	// ("cv::aruco::Dictionary::generateImageMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, ResultVoid* ocvrs_return) {
		try {
			instance->generateImageMarker(id, sidePixels, *_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:82
	// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
	void cv_aruco_Dictionary_getByteListFromBits_const_MatR(const cv::Mat* bits, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*bits);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:87
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

	// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:32
	// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_Dictionary_propBytesList_const(const cv::aruco::Dictionary* instance) {
			cv::Mat ret = instance->bytesList;
			return new cv::Mat(ret);
	}

	// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:32
	// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_Dictionary_propBytesList_const_Mat(cv::aruco::Dictionary* instance, const cv::Mat* val) {
			instance->bytesList = *val;
	}

	// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:33
	// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMarkerSize_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->markerSize;
			return ret;
	}

	// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:33
	// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMarkerSize_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->markerSize = val;
	}

	// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:34
	// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMaxCorrectionBits_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->maxCorrectionBits;
			return ret;
	}

	// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_dictionary.hpp:34
	// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMaxCorrectionBits_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->maxCorrectionBits = val;
	}

	// cv::aruco::Dictionary::delete() generated
	// ("cv::aruco::Dictionary::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_delete(cv::aruco::Dictionary* instance) {
			delete instance;
	}

	// GridBoard(const Size &, float, float, const Dictionary &, InputArray)(SimpleClass, Primitive, Primitive, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:118
	// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary", "ids"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(const cv::Size* size, float markerLength, float markerSeparation, const cv::aruco::Dictionary* dictionary, const cv::_InputArray* ids, Result<cv::aruco::GridBoard*>* ocvrs_return) {
		try {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard(*size, markerLength, markerSeparation, *dictionary, *ids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::GridBoard(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:118
	// ("cv::aruco::GridBoard::GridBoard", vec![(pred!(mut, ["size", "markerLength", "markerSeparation", "dictionary"], ["const cv::Size*", "float", "float", "const cv::aruco::Dictionary*"]), _)]),
	void cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(const cv::Size* size, float markerLength, float markerSeparation, const cv::aruco::Dictionary* dictionary, Result<cv::aruco::GridBoard*>* ocvrs_return) {
		try {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard(*size, markerLength, markerSeparation, *dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:121
	// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getGridSize_const(const cv::aruco::GridBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:122
	// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerLength_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:123
	// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerSeparation_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerSeparation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GridBoard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_board.hpp:126
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

	// RefineParameters(float, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:239
	// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, ["minRepDistance", "errorCorrectionRate", "checkAllOrders"], ["float", "float", "bool"]), _)]),
	void cv_aruco_RefineParameters_RefineParameters_float_float_bool(float minRepDistance, float errorCorrectionRate, bool checkAllOrders, Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			cv::aruco::RefineParameters ret(minRepDistance, errorCorrectionRate, checkAllOrders);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::RefineParameters::RefineParameters() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:239
	// ("cv::aruco::RefineParameters::RefineParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_RefineParameters_RefineParameters(Result<cv::aruco::RefineParameters>* ocvrs_return) {
		try {
			cv::aruco::RefineParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readRefineParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:244
	// ("cv::aruco::RefineParameters::readRefineParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(cv::aruco::RefineParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readRefineParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeRefineParameters(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:248
	// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(cv::aruco::RefineParameters* instance, cv::FileStorage* fs, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeRefineParameters(*fs, std::string(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::RefineParameters::writeRefineParameters(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/aruco_detector.hpp:248
	// ("cv::aruco::RefineParameters::writeRefineParameters", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(cv::aruco::RefineParameters* instance, cv::FileStorage* fs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->writeRefineParameters(*fs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BarcodeDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:23
	// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_barcode_BarcodeDetector_BarcodeDetector(Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector* ret = new cv::barcode::BarcodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BarcodeDetector(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:30
	// ("cv::barcode::BarcodeDetector::BarcodeDetector", vec![(pred!(mut, ["prototxt_path", "model_path"], ["const std::string*", "const std::string*"]), _)]),
	void cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(const char* prototxt_path, const char* model_path, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector* ret = new cv::barcode::BarcodeDetector(std::string(prototxt_path), std::string(model_path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeWithType(InputArray, InputArray, std::vector<std::string> &, std::vector<std::string> &)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:43
	// ("cv::barcode::BarcodeDetector::decodeWithType", vec![(pred!(const, ["img", "points", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
	void cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeWithType(*img, *points, *decoded_info, *decoded_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeWithType(InputArray, std::vector<std::string> &, std::vector<std::string> &, OutputArray)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:56
	// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type", "points"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeWithType(*img, *decoded_info, *decoded_type, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::barcode::BarcodeDetector::detectAndDecodeWithType(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:56
	// ("cv::barcode::BarcodeDetector::detectAndDecodeWithType", vec![(pred!(const, ["img", "decoded_info", "decoded_type"], ["const cv::_InputArray*", "std::vector<std::string>*", "std::vector<std::string>*"]), _)]),
	void cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(const cv::barcode::BarcodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, std::vector<std::string>* decoded_type, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeWithType(*img, *decoded_info, *decoded_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDownsamplingThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:65
	// ("cv::barcode::BarcodeDetector::getDownsamplingThreshold", vec![(pred!(const, [], []), _)]),
	void cv_barcode_BarcodeDetector_getDownsamplingThreshold_const(const cv::barcode::BarcodeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDownsamplingThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDownsamplingThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:75
	// ("cv::barcode::BarcodeDetector::setDownsamplingThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
	void cv_barcode_BarcodeDetector_setDownsamplingThreshold_double(cv::barcode::BarcodeDetector* instance, double thresh, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector ret = instance->setDownsamplingThreshold(thresh);
			Ok(new cv::barcode::BarcodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectorScales(std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:81
	// ("cv::barcode::BarcodeDetector::getDetectorScales", vec![(pred!(const, ["sizes"], ["std::vector<float>*"]), _)]),
	void cv_barcode_BarcodeDetector_getDetectorScales_const_vectorLfloatGR(const cv::barcode::BarcodeDetector* instance, std::vector<float>* sizes, ResultVoid* ocvrs_return) {
		try {
			instance->getDetectorScales(*sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectorScales(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:90
	// ("cv::barcode::BarcodeDetector::setDetectorScales", vec![(pred!(mut, ["sizes"], ["const std::vector<float>*"]), _)]),
	void cv_barcode_BarcodeDetector_setDetectorScales_const_vectorLfloatGR(cv::barcode::BarcodeDetector* instance, const std::vector<float>* sizes, Result<cv::barcode::BarcodeDetector*>* ocvrs_return) {
		try {
			cv::barcode::BarcodeDetector ret = instance->setDetectorScales(*sizes);
			Ok(new cv::barcode::BarcodeDetector(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:96
	// ("cv::barcode::BarcodeDetector::getGradientThreshold", vec![(pred!(const, [], []), _)]),
	void cv_barcode_BarcodeDetector_getGradientThreshold_const(const cv::barcode::BarcodeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGradientThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/objdetect/barcode.hpp:105
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
