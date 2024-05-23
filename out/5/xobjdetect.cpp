#include "ocvrs_common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:339
	// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_createFaceDetectionMaskGenerator(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:163
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:163
	// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
	void cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, cv::Size* winDetSize, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:152
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int(std::vector<cv::Rect>* rectList, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:152
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:157
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:154
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:154
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:160
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:160
	// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:183
	// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:184
	// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_BaseCascadeClassifier_load_const_StringR(cv::BaseCascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:185
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:191
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:198
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:207
	// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:208
	// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:209
	// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:210
	// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:219
	// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:220
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

	// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:216
	// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
	void cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:217
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

	// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:232
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_CascadeClassifier(Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:237
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_CascadeClassifier_const_StringR(const char* filename, Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:241
	// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:248
	// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_CascadeClassifier_load_const_StringR(cv::CascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:253
	// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_CascadeClassifier_read_const_FileNodeR(cv::CascadeClassifier* instance, const cv::FileNode* node, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:269
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:269
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:291
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:291
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:316
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:316
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:326
	// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:327
	// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:328
	// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
	void cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:329
	// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:331
	// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_CascadeClassifier_convert_const_StringR_const_StringR(const char* oldcascade, const char* newcascade, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::CascadeClassifier::convert(std::string(oldcascade), std::string(newcascade));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:333
	// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	void cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, ResultVoid* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:334
	// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:336
	// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BaseCascadeClassifier>* cv_CascadeClassifier_propCc(cv::CascadeClassifier* instance) {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return new cv::Ptr<cv::BaseCascadeClassifier>(ret);
	}

	// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:336
	// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
	void cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier>* val) {
			instance->cc = *val;
	}

	// cv::CascadeClassifier::delete() generated
	// ("cv::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
			delete instance;
	}

	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:121
	// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params, Result<cv::DetectionBasedTracker*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:124
	// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:125
	// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetTracking()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:126
	// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetTracking();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:128
	// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
	void cv_DetectionBasedTracker_process_const_MatR(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray, ResultVoid* ocvrs_return) {
		try {
			instance->process(*imageGray);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:130
	// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
	void cv_DetectionBasedTracker_setParameters_const_ParametersR(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setParameters(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:131
	// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance, Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			const cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			Ok(new const cv::DetectionBasedTracker::Parameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:135
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:136
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::Object>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:155
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
	void cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result, ResultVoid* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:158
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

	// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:150
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

	// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:147
	// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_ExtObject_propId_const(const cv::DetectionBasedTracker::ExtObject* instance) {
			int ret = instance->id;
			return ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:147
	// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propId_const_int(cv::DetectionBasedTracker::ExtObject* instance, const int val) {
			instance->id = val;
	}

	// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:148
	// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->location;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:148
	// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(cv::DetectionBasedTracker::ExtObject* instance, const cv::Rect* val) {
			instance->location = *val;
	}

	// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:149
	// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus* ocvrs_return) {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			*ocvrs_return = ret;
	}

	// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:149
	// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	void cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, const cv::DetectionBasedTracker::ObjectStatus val) {
			instance->status = val;
	}

	// cv::DetectionBasedTracker::ExtObject::delete() generated
	// ("cv::DetectionBasedTracker::ExtObject::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
			delete instance;
	}

	// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:78
	// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:80
	// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min, ResultVoid* ocvrs_return) {
		try {
			instance->setMinObjectSize(*min);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:84
	// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxObjectSize(*max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:88
	// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:92
	// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:96
	// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:100
	// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:104
	// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinNeighbours();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:108
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

	// Parameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:65
	// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionBasedTracker_Parameters_Parameters(Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:62
	// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->maxTrackLifetime;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:62
	// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(cv::DetectionBasedTracker::Parameters* instance, const int val) {
			instance->maxTrackLifetime = val;
	}

	// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:63
	// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
	int cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->minDetectionPeriod;
			return ret;
	}

	// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect/detection_based_tracker.hpp:63
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

	// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:350
	// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
	double cv_DetectionROI_propScale_const(const cv::DetectionROI* instance) {
			double ret = instance->scale;
			return ret;
	}

	// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:350
	// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_DetectionROI_propScale_const_double(cv::DetectionROI* instance, const double val) {
			instance->scale = val;
	}

	// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:352
	// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* cv_DetectionROI_propLocations_const(const cv::DetectionROI* instance) {
			std::vector<cv::Point> ret = instance->locations;
			return new std::vector<cv::Point>(ret);
	}

	// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:352
	// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	void cv_DetectionROI_propLocations_const_vectorLPointG(cv::DetectionROI* instance, const std::vector<cv::Point>* val) {
			instance->locations = *val;
	}

	// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:354
	// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
	std::vector<double>* cv_DetectionROI_propConfidences_const(const cv::DetectionROI* instance) {
			std::vector<double> ret = instance->confidences;
			return new std::vector<double>(ret);
	}

	// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:354
	// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	void cv_DetectionROI_propConfidences_const_vectorLdoubleG(cv::DetectionROI* instance, const std::vector<double>* val) {
			instance->confidences = *val;
	}

	// cv::DetectionROI::delete() generated
	// ("cv::DetectionROI::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
			delete instance;
	}

	// HOGDescriptor(Size, Size, Size, Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Enum, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:398
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "cv::HOGDescriptor::HistogramNormType", "double", "bool", "int", "bool"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, cv::HOGDescriptor::HistogramNormType _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::HOGDescriptor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:398
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_HOGDescriptor(Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:414
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_StringR(const char* filename, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:422
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(const cv::HOGDescriptor* d, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:433
	// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:437
	// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkDetectorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:441
	// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWinSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:448
	// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["svmdetector"], ["const cv::_InputArray*"]), _)]),
	void cv_HOGDescriptor_setSVMDetector_const__InputArrayR(cv::HOGDescriptor* instance, const cv::_InputArray* svmdetector, ResultVoid* ocvrs_return) {
		try {
			instance->setSVMDetector(*svmdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:453
	// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
	void cv_HOGDescriptor_read_FileNodeR(cv::HOGDescriptor* instance, cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:459
	// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_write_const_FileStorageR_const_StringR(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:465
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR_const_StringR(cv::HOGDescriptor* instance, const char* filename, const char* objname, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename), std::string(objname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:465
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_load_const_StringR(cv::HOGDescriptor* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:471
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR_const_StringR(const cv::HOGDescriptor* instance, const char* filename, const char* objname, ResultVoid* ocvrs_return) {
		try {
			instance->save(std::string(filename), std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:471
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_HOGDescriptor_save_const_const_StringR(const cv::HOGDescriptor* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:476
	// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
	void cv_HOGDescriptor_copyTo_const_HOGDescriptorR(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:485
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:485
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:501
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:501
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:517
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:517
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:537
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:537
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:556
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:556
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeGradient(InputArray, InputOutputArray, InputOutputArray, Size, Size)(InputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:568
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, cv::Size* paddingTL, cv::Size* paddingBR, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::computeGradient(InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:568
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, ResultVoid* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:573
	// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDefaultPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:579
	// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_getDaimlerPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectROI(InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:637
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, cv::Size* winStride, cv::Size* padding, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:637
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::_InputArray*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScaleROI(InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:650
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR_double_int(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::detectMultiScaleROI(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:650
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
	void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:662
	// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
	void cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps, ResultVoid* ocvrs_return) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:582
	// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propWinSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:582
	// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propWinSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->winSize = *val;
	}

	// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:585
	// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:585
	// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockSize = *val;
	}

	// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:588
	// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propBlockStride_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockStride;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:588
	// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propBlockStride_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->blockStride = *val;
	}

	// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:591
	// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propCellSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->cellSize;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:591
	// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_HOGDescriptor_propCellSize_const_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
			instance->cellSize = *val;
	}

	// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:594
	// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNbins_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nbins;
			return ret;
	}

	// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:594
	// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNbins_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nbins = val;
	}

	// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:597
	// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propDerivAperture_const(const cv::HOGDescriptor* instance) {
			int ret = instance->derivAperture;
			return ret;
	}

	// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:597
	// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propDerivAperture_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->derivAperture = val;
	}

	// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:600
	// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propWinSigma_const(const cv::HOGDescriptor* instance) {
			double ret = instance->winSigma;
			return ret;
	}

	// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:600
	// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propWinSigma_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->winSigma = val;
	}

	// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:603
	// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
	void cv_HOGDescriptor_propHistogramNormType_const(const cv::HOGDescriptor* instance, cv::HOGDescriptor::HistogramNormType* ocvrs_return) {
			cv::HOGDescriptor::HistogramNormType ret = instance->histogramNormType;
			*ocvrs_return = ret;
	}

	// cv::HOGDescriptor::setHistogramNormType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:603
	// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const cv::HOGDescriptor::HistogramNormType"]), _)]),
	void cv_HOGDescriptor_propHistogramNormType_const_HistogramNormType(cv::HOGDescriptor* instance, const cv::HOGDescriptor::HistogramNormType val) {
			instance->histogramNormType = val;
	}

	// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:606
	// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
	double cv_HOGDescriptor_propL2HysThreshold_const(const cv::HOGDescriptor* instance) {
			double ret = instance->L2HysThreshold;
			return ret;
	}

	// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:606
	// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_HOGDescriptor_propL2HysThreshold_const_double(cv::HOGDescriptor* instance, const double val) {
			instance->L2HysThreshold = val;
	}

	// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:609
	// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propGammaCorrection_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->gammaCorrection;
			return ret;
	}

	// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:609
	// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propGammaCorrection_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->gammaCorrection = val;
	}

	// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:612
	// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_HOGDescriptor_propSvmDetector_const(const cv::HOGDescriptor* instance) {
			std::vector<float> ret = instance->svmDetector;
			return new std::vector<float>(ret);
	}

	// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:612
	// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(cv::HOGDescriptor* instance, const std::vector<float>* val) {
			instance->svmDetector = *val;
	}

	// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:615
	// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
	cv::UMat* cv_HOGDescriptor_propOclSvmDetector_const(const cv::HOGDescriptor* instance) {
			cv::UMat ret = instance->oclSvmDetector;
			return new cv::UMat(ret);
	}

	// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:615
	// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	void cv_HOGDescriptor_propOclSvmDetector_const_UMat(cv::HOGDescriptor* instance, const cv::UMat* val) {
			instance->oclSvmDetector = *val;
	}

	// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:618
	// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
	float cv_HOGDescriptor_propFree_coef_const(const cv::HOGDescriptor* instance) {
			float ret = instance->free_coef;
			return ret;
	}

	// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:618
	// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_HOGDescriptor_propFree_coef_const_float(cv::HOGDescriptor* instance, const float val) {
			instance->free_coef = val;
	}

	// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:621
	// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
	int cv_HOGDescriptor_propNlevels_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nlevels;
			return ret;
	}

	// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:621
	// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_HOGDescriptor_propNlevels_const_int(cv::HOGDescriptor* instance, const int val) {
			instance->nlevels = val;
	}

	// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:624
	// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
	bool cv_HOGDescriptor_propSignedGradient_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->signedGradient;
			return ret;
	}

	// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:624
	// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_HOGDescriptor_propSignedGradient_const_bool(cv::HOGDescriptor* instance, const bool val) {
			instance->signedGradient = val;
	}

	// cv::HOGDescriptor::delete() generated
	// ("cv::HOGDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
			delete instance;
	}

	// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:125
	// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
	void cv_SimilarRects_SimilarRects_double(double _eps, Result<cv::SimilarRects*>* ocvrs_return) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:126
	// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
	void cv_SimilarRects_operator___const_const_RectR_const_RectR(const cv::SimilarRects* instance, const cv::Rect* r1, const cv::Rect* r2, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator()(*r1, *r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:134
	// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
	double cv_SimilarRects_propEps_const(const cv::SimilarRects* instance) {
			double ret = instance->eps;
			return ret;
	}

	// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:134
	// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_SimilarRects_propEps_const_double(cv::SimilarRects* instance, const double val) {
			instance->eps = val;
	}

	// cv::SimilarRects::delete() generated
	// ("cv::SimilarRects::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
			delete instance;
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:679
	// ("cv::xobjdetect::WBDetector::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_xobjdetect_WBDetector_read_const_FileNodeR(cv::xobjdetect::WBDetector* instance, const cv::FileNode* node, ResultVoid* ocvrs_return) {
		try {
			instance->read(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:684
	// ("cv::xobjdetect::WBDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_xobjdetect_WBDetector_write_const_FileStorageR(const cv::xobjdetect::WBDetector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:690
	// ("cv::xobjdetect::WBDetector::train", vec![(pred!(mut, ["pos_samples", "neg_imgs"], ["const std::string*", "const std::string*"]), _)]),
	void cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(cv::xobjdetect::WBDetector* instance, const char* pos_samples, const char* neg_imgs, ResultVoid* ocvrs_return) {
		try {
			instance->train(std::string(pos_samples), std::string(neg_imgs));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<Rect> &, std::vector<double> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:699
	// ("cv::xobjdetect::WBDetector::detect", vec![(pred!(mut, ["img", "bboxes", "confidences"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(cv::xobjdetect::WBDetector* instance, const cv::Mat* img, std::vector<cv::Rect>* bboxes, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *bboxes, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xobjdetect.hpp:706
	// ("cv::xobjdetect::WBDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xobjdetect_WBDetector_create(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			Ok(new cv::Ptr<cv::xobjdetect::WBDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xobjdetect::WBDetector::delete() generated
	// ("cv::xobjdetect::WBDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xobjdetect_WBDetector_delete(cv::xobjdetect::WBDetector* instance) {
			delete instance;
	}

}
