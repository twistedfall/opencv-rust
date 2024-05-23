#include "ocvrs_common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:340
	// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_createFaceDetectionMaskGenerator(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::decodeCurvedQRCode(InputArray, InputArray, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:896
	// ("cv::decodeCurvedQRCode", vec![(pred!(mut, ["in", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*"]), _)]),
	void cv_decodeCurvedQRCode_const__InputArrayR_const__InputArrayR_stringR(const cv::_InputArray* in, const cv::_InputArray* points, void** decoded_info, Result<bool>* ocvrs_return) {
		try {
			std::string decoded_info_out;
			bool ret = cv::decodeCurvedQRCode(*in, *points, decoded_info_out);
			*decoded_info = ocvrs_create_string(decoded_info_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeCurvedQRCode(InputArray, InputArray, std::string &, OutputArray)(InputArray, InputArray, OutString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:896
	// ("cv::decodeCurvedQRCode", vec![(pred!(mut, ["in", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*", "const cv::_OutputArray*"]), _)]),
	void cv_decodeCurvedQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(const cv::_InputArray* in, const cv::_InputArray* points, void** decoded_info, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			std::string decoded_info_out;
			bool ret = cv::decodeCurvedQRCode(*in, *points, decoded_info_out, *straight_qrcode);
			*decoded_info = ocvrs_create_string(decoded_info_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::decodeQRCode(InputArray, InputArray, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:888
	// ("cv::decodeQRCode", vec![(pred!(mut, ["in", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*"]), _)]),
	void cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR(const cv::_InputArray* in, const cv::_InputArray* points, void** decoded_info, Result<bool>* ocvrs_return) {
		try {
			std::string decoded_info_out;
			bool ret = cv::decodeQRCode(*in, *points, decoded_info_out);
			*decoded_info = ocvrs_create_byte_string(decoded_info_out.data(), decoded_info_out.size());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeQRCode(InputArray, InputArray, std::string &, OutputArray)(InputArray, InputArray, OutString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:888
	// ("cv::decodeQRCode", vec![(pred!(mut, ["in", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*", "const cv::_OutputArray*"]), _)]),
	void cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(const cv::_InputArray* in, const cv::_InputArray* points, void** decoded_info, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			std::string decoded_info_out;
			bool ret = cv::decodeQRCode(*in, *points, decoded_info_out, *straight_qrcode);
			*decoded_info = ocvrs_create_byte_string(decoded_info_out.data(), decoded_info_out.size());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detectQRCode(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:880
	// ("cv::detectQRCode", vec![(pred!(mut, ["in", "points"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
	void cv_detectQRCode_const__InputArrayR_vectorLPointGR(const cv::_InputArray* in, std::vector<cv::Point>* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::detectQRCode(*in, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectQRCode(InputArray, std::vector<Point> &, double, double)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:880
	// ("cv::detectQRCode", vec![(pred!(mut, ["in", "points", "eps_x", "eps_y"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "double"]), _)]),
	void cv_detectQRCode_const__InputArrayR_vectorLPointGR_double_double(const cv::_InputArray* in, std::vector<cv::Point>* points, double eps_x, double eps_y, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::detectQRCode(*in, *points, eps_x, eps_y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:162
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:162
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, cv::Size* winDetSize, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:151
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int(std::vector<cv::Rect>* rectList, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:151
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:156
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:153
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:153
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:159
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:159
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:178
	// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:179
	// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_BaseCascadeClassifier_load_const_StringR(cv::BaseCascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:180
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:186
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:193
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:202
	// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:203
	// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:204
	// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:205
	// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:214
	// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:215
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

	// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:211
	// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
	void cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:212
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

	// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:227
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_CascadeClassifier(Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:232
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_CascadeClassifier_const_StringR(const char* filename, Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:236
	// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:243
	// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_load_const_StringR(cv::CascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:248
	// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_CascadeClassifier_read_const_FileNodeR(cv::CascadeClassifier* instance, const cv::FileNode* node, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:270
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:270
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:292
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:292
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:317
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:317
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:327
	// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:328
	// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:329
	// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:330
	// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:332
	// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_CascadeClassifier_convert_const_StringR_const_StringR(const char* oldcascade, const char* newcascade, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::CascadeClassifier::convert(cv::String(oldcascade), cv::String(newcascade));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:334
	// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:335
	// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:337
	// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BaseCascadeClassifier>* cv_CascadeClassifier_propCc(cv::CascadeClassifier* instance) {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return new cv::Ptr<cv::BaseCascadeClassifier>(ret);
	}

	// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:337
	// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
	void cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier>* val) {
			instance->cc = *val;
	}

	// cv::CascadeClassifier::delete() generated
	// ("cv::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
			delete instance;
	}

	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:125
	// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params, Result<cv::DetectionBasedTracker*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:128
	// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:129
	// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetTracking()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:130
	// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetTracking();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:132
	// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
	void cv_DetectionBasedTracker_process_const_MatR(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray, ResultVoid* ocvrs_return) {
		try {
			instance->process(*imageGray);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:134
	// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_setParameters_const_ParametersR(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setParameters(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:135
	// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance, Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			const cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			Ok(new const cv::DetectionBasedTracker::Parameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:139
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:140
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::Object>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:159
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:162
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

	// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:154
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

	// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:151
	// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_ExtObject_propId_const(const cv::DetectionBasedTracker::ExtObject* instance) {
			int ret = instance->id;
			return ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:151
	// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propId_const_int(cv::DetectionBasedTracker::ExtObject* instance, const int val) {
			instance->id = val;
	}

	// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:152
	// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->location;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:152
	// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(cv::DetectionBasedTracker::ExtObject* instance, const cv::Rect* val) {
			instance->location = *val;
	}

	// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:153
	// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus* ocvrs_return) {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:153
	// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, const cv::DetectionBasedTracker::ObjectStatus val) {
			instance->status = val;
	}

	// cv::DetectionBasedTracker::ExtObject::delete() generated
	// ("cv::DetectionBasedTracker::ExtObject::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
			delete instance;
	}

	// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:82
	// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:84
	// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min, ResultVoid* ocvrs_return) {
		try {
			instance->setMinObjectSize(*min);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:88
	// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxObjectSize(*max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:92
	// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:96
	// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:100
	// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:104
	// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:108
	// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinNeighbours();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:112
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

	// Parameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:69
	// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_Parameters_Parameters(Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:66
	// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->maxTrackLifetime;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:66
	// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(cv::DetectionBasedTracker::Parameters* instance, const int val) {
			instance->maxTrackLifetime = val;
	}

	// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:67
	// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->minDetectionPeriod;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:67
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

	// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:348
	// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
	double cv_DetectionROI_propScale_const(const cv::DetectionROI* instance) {
			double ret = instance->scale;
			return ret;
	}

	// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:348
	// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_DetectionROI_propScale_const_double(cv::DetectionROI* instance, const double val) {
			instance->scale = val;
	}

	// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:350
	// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* cv_DetectionROI_propLocations_const(const cv::DetectionROI* instance) {
			std::vector<cv::Point> ret = instance->locations;
			return new std::vector<cv::Point>(ret);
	}

	// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:350
	// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	void cv_DetectionROI_propLocations_const_vectorLPointG(cv::DetectionROI* instance, const std::vector<cv::Point>* val) {
			instance->locations = *val;
	}

	// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:352
	// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
	std::vector<double>* cv_DetectionROI_propConfidences_const(const cv::DetectionROI* instance) {
			std::vector<double> ret = instance->confidences;
			return new std::vector<double>(ret);
	}

	// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:352
	// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	void cv_DetectionROI_propConfidences_const_vectorLdoubleG(cv::DetectionROI* instance, const std::vector<double>* val) {
			instance->confidences = *val;
	}

	// cv::DetectionROI::delete() generated
	// ("cv::DetectionROI::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
			delete instance;
	}

	// HOGDescriptor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:383
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_HOGDescriptor(Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(Size, Size, Size, Size, int, int, double, int, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:403
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "int", "double", "bool", "int", "bool"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, int _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::HOGDescriptor(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:403
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:419
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_StringR(const char* filename, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:427
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(const cv::HOGDescriptor* d, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:438
	// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:442
	// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkDetectorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:446
	// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWinSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:453
	// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["_svmdetector"], ["const cv::_InputArray*"]), _)]),
	void cv_HOGDescriptor_setSVMDetector_const__InputArrayR(cv::HOGDescriptor* instance, const cv::_InputArray* _svmdetector, ResultVoid* ocvrs_return) {
		try {
			instance->setSVMDetector(*_svmdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:458
	// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
	void cv_HOGDescriptor_read_FileNodeR(cv::HOGDescriptor* instance, cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:464
	// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_write_const_FileStorageR_const_StringR(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, cv::String(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:470
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR_const_StringR(cv::HOGDescriptor* instance, const char* filename, const char* objname, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(cv::String(filename), cv::String(objname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:470
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR(cv::HOGDescriptor* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:476
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR_const_StringR(const cv::HOGDescriptor* instance, const char* filename, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->save(cv::String(filename), cv::String(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:476
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR(const cv::HOGDescriptor* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:481
	// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_copyTo_const_HOGDescriptorR(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:492
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:492
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::Mat*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::Mat*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(TraitClass, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::Mat*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::Mat*", "std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:544
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:544
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:563
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:563
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeGradient(const Mat &, Mat &, Mat &, Size, Size)(TraitClass, TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:575
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::Mat*", "cv::Mat*", "cv::Mat*", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR_Size_Size(const cv::HOGDescriptor* instance, const cv::Mat* img, cv::Mat* grad, cv::Mat* angleOfs, cv::Size* paddingTL, cv::Size* paddingBR, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::computeGradient(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:575
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::Mat*", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR(const cv::HOGDescriptor* instance, const cv::Mat* img, cv::Mat* grad, cv::Mat* angleOfs, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:580
	// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDefaultPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:586
	// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDaimlerPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectROI(const cv::Mat &, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:644
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::Mat*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const_MatR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(const cv::HOGDescriptor* instance, const cv::Mat* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, cv::Size* winStride, cv::Size* padding, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectROI(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:644
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::Mat*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const_MatR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::Mat* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScaleROI(const cv::Mat &, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vectorLRectGR_vectorLDetectionROIGR_double_int(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScaleROI(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vectorLRectGR_vectorLDetectionROIGR(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readALTModel(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:666
	// ("cv::HOGDescriptor::readALTModel", vec![(pred!(mut, ["modelfile"], ["cv::String"]), _)]),
	void cv_HOGDescriptor_readALTModel_String(cv::HOGDescriptor* instance, const char* modelfile, ResultVoid* ocvrs_return) {
		try {
			instance->readALTModel(cv::String(modelfile));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:674
	// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:589
	// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propWinSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:589
	// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propWinSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->winSize = *val;
	}

	// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:592
	// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:592
	// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockSize = *val;
	}

	// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockStride_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockStride;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockStride_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockStride = *val;
	}

	// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:598
	// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propCellSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->cellSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:598
	// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propCellSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->cellSize = *val;
	}

	// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:601
	// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNbins_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nbins;
			return ret;
	}

	// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:601
	// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNbins_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nbins = val;
	}

	// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:604
	// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propDerivAperture_const(const cv::HOGDescriptor* instance) {
			int ret = instance->derivAperture;
			return ret;
	}

	// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:604
	// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propDerivAperture_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->derivAperture = val;
	}

	// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propWinSigma_const(const cv::HOGDescriptor* instance) {
			double ret = instance->winSigma;
			return ret;
	}

	// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propWinSigma_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->winSigma = val;
	}

	// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:610
	// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propHistogramNormType_const(const cv::HOGDescriptor* instance) {
			int ret = instance->histogramNormType;
			return ret;
	}

	// cv::HOGDescriptor::setHistogramNormType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:610
	// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propHistogramNormType_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->histogramNormType = val;
	}

	// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:613
	// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propL2HysThreshold_const(const cv::HOGDescriptor* instance) {
			double ret = instance->L2HysThreshold;
			return ret;
	}

	// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:613
	// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propL2HysThreshold_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->L2HysThreshold = val;
	}

	// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:616
	// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propGammaCorrection_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->gammaCorrection;
			return ret;
	}

	// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:616
	// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propGammaCorrection_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->gammaCorrection = val;
	}

	// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:619
	// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_HOGDescriptor_propSvmDetector_const(const cv::HOGDescriptor* instance) {
			std::vector<float> ret = instance->svmDetector;
			return new std::vector<float>(ret);
	}

	// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:619
	// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(cv::HOGDescriptor* instance, const std::vector<float>* val) {
			instance->svmDetector = *val;
	}

	// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:622
	// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
	cv::UMat* cv_HOGDescriptor_propOclSvmDetector_const(const cv::HOGDescriptor* instance) {
			cv::UMat ret = instance->oclSvmDetector;
			return new cv::UMat(ret);
	}

	// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:622
	// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	void cv_HOGDescriptor_propOclSvmDetector_const_UMat(cv::HOGDescriptor* instance, const cv::UMat* val) {
			instance->oclSvmDetector = *val;
	}

	// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:625
	// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
	float cv_HOGDescriptor_propFree_coef_const(const cv::HOGDescriptor* instance) {
			float ret = instance->free_coef;
			return ret;
	}

	// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:625
	// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_HOGDescriptor_propFree_coef_const_float(cv::HOGDescriptor* instance, const float val) {
			instance->free_coef = val;
	}

	// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:628
	// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNlevels_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nlevels;
			return ret;
	}

	// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:628
	// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNlevels_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nlevels = val;
	}

	// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:631
	// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propSignedGradient_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->signedGradient;
			return ret;
	}

	// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:631
	// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propSignedGradient_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->signedGradient = val;
	}

	// cv::HOGDescriptor::delete() generated
	// ("cv::HOGDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
			delete instance;
	}

	// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:743
	// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetector_QRCodeDetector(Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:750
	// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsX(epsX);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:755
	// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
	void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsY(epsY);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:761
	// ("cv::QRCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detect(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:770
	// ("cv::QRCodeDetector::decode", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decode(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:770
	// ("cv::QRCodeDetector::decode", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decode(*img, *points);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:779
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:779
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:787
	// ("cv::QRCodeDetector::detectAndDecode", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->detectAndDecode(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:787
	// ("cv::QRCodeDetector::detectAndDecode", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecode_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->detectAndDecode(*img);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:796
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->detectAndDecodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:796
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->detectAndDecodeCurved(*img);
			Ok(ocvrs_create_byte_string(ret.begin(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:804
	// ("cv::QRCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectMulti(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeMulti(InputArray, InputArray, std::vector<cv::String> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:813
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLStringGR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<cv::String>* decoded_info, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_qrcode);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:813
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::String>*"]), _)]),
	void cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLStringGR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<cv::String>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecodeMulti(InputArray, std::vector<cv::String> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:826
	// ("cv::QRCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_qrcode"], ["const cv::_InputArray*", "std::vector<cv::String>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLStringGR_const__OutputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, std::vector<cv::String>* decoded_info, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info, *points, *straight_qrcode);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:826
	// ("cv::QRCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info"], ["const cv::_InputArray*", "std::vector<cv::String>*"]), _)]),
	void cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLStringGR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, std::vector<cv::String>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:834
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_qrcode);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:834
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	void cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeDetector::delete() generated
	// ("cv::QRCodeDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeDetector_delete(cv::QRCodeDetector* instance) {
			delete instance;
	}

	// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:724
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
	void cv_QRCodeEncoder_create_const_ParamsR(const cv::QRCodeEncoder::Params* parameters, Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create(*parameters);
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:724
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_create(Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create();
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:730
	// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcode, ResultVoid* ocvrs_return) {
		try {
			instance->encode(cv::String(encoded_info), *qrcode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:736
	// ("cv::QRCodeEncoder::encodeStructuredAppend", vec![(pred!(mut, ["encoded_info", "qrcodes"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcodes, ResultVoid* ocvrs_return) {
		try {
			instance->encodeStructuredAppend(cv::String(encoded_info), *qrcodes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QRCodeEncoder::delete() generated
	// ("cv::QRCodeEncoder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_delete(cv::QRCodeEncoder* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:713
	// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_QRCodeEncoder_Params_Params(Result<cv::QRCodeEncoder::Params>* ocvrs_return) {
		try {
			cv::QRCodeEncoder::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:124
	// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
	void cv_SimilarRects_SimilarRects_double(double _eps, Result<cv::SimilarRects*>* ocvrs_return) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:125
	// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
	void cv_SimilarRects_operator___const_const_RectR_const_RectR(const cv::SimilarRects* instance, const cv::Rect* r1, const cv::Rect* r2, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator()(*r1, *r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:133
	// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
	double cv_SimilarRects_propEps_const(const cv::SimilarRects* instance) {
			double ret = instance->eps;
			return ret;
	}

	// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:133
	// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_SimilarRects_propEps_const_double(cv::SimilarRects* instance, const double val) {
			instance->eps = val;
	}

	// cv::SimilarRects::delete() generated
	// ("cv::SimilarRects::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
			delete instance;
	}

}
