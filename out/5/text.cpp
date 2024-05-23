#include "ocvrs_common.hpp"
#include <opencv2/text.hpp>
#include "text_types.hpp"

extern "C" {
	// MSERsToERStats(InputArray, std::vector<std::vector<Point>> &, std::vector<std::vector<ERStat>> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:347
	// ("cv::text::MSERsToERStats", vec![(pred!(mut, ["image", "contours", "regions"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<std::vector<cv::text::ERStat>>*"]), _)]),
	void cv_text_MSERsToERStats_const__InputArrayR_vectorLvectorLPointGGR_vectorLvectorLERStatGGR(const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* contours, std::vector<std::vector<cv::text::ERStat>>* regions, ResultVoid* ocvrs_return) {
		try {
			cv::text::MSERsToERStats(*image, *contours, *regions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::computeNMChannels(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:262
	// ("cv::text::computeNMChannels", vec![(pred!(mut, ["_src", "_channels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* _src, const cv::_OutputArray* _channels, ResultVoid* ocvrs_return) {
		try {
			cv::text::computeNMChannels(*_src, *_channels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeNMChannels(InputArray, OutputArrayOfArrays, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:262
	// ("cv::text::computeNMChannels", vec![(pred!(mut, ["_src", "_channels", "_mode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* _src, const cv::_OutputArray* _channels, int _mode, ResultVoid* ocvrs_return) {
		try {
			cv::text::computeNMChannels(*_src, *_channels, _mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::createERFilterNM1(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:187
	// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
	void cv_text_createERFilterNM1_const_PtrLCallbackGR(const cv::Ptr<cv::text::ERFilter::Callback>* cb, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(*cb);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createERFilterNM1(const Ptr<ERFilter::Callback> &, int, float, float, float, bool, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:187
	// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["cb", "thresholdDelta", "minArea", "maxArea", "minProbability", "nonMaxSuppression", "minProbabilityDiff"], ["const cv::Ptr<cv::text::ERFilter::Callback>*", "int", "float", "float", "float", "bool", "float"]), _)]),
	void cv_text_createERFilterNM1_const_PtrLCallbackGR_int_float_float_float_bool_float(const cv::Ptr<cv::text::ERFilter::Callback>* cb, int thresholdDelta, float minArea, float maxArea, float minProbability, bool nonMaxSuppression, float minProbabilityDiff, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(*cb, thresholdDelta, minArea, maxArea, minProbability, nonMaxSuppression, minProbabilityDiff);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::createERFilterNM1(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:212
	// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_createERFilterNM1_const_StringR(const char* filename, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(std::string(filename));
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createERFilterNM1(const String &, int, float, float, float, bool, float)(InString, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:212
	// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["filename", "thresholdDelta", "minArea", "maxArea", "minProbability", "nonMaxSuppression", "minProbabilityDiff"], ["const cv::String*", "int", "float", "float", "float", "bool", "float"]), _)]),
	void cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float(const char* filename, int thresholdDelta, float minArea, float maxArea, float minProbability, bool nonMaxSuppression, float minProbabilityDiff, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(std::string(filename), thresholdDelta, minArea, maxArea, minProbability, nonMaxSuppression, minProbabilityDiff);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::createERFilterNM2(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:204
	// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
	void cv_text_createERFilterNM2_const_PtrLCallbackGR(const cv::Ptr<cv::text::ERFilter::Callback>* cb, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(*cb);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createERFilterNM2(const Ptr<ERFilter::Callback> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:204
	// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["cb", "minProbability"], ["const cv::Ptr<cv::text::ERFilter::Callback>*", "float"]), _)]),
	void cv_text_createERFilterNM2_const_PtrLCallbackGR_float(const cv::Ptr<cv::text::ERFilter::Callback>* cb, float minProbability, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(*cb, minProbability);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::createERFilterNM2(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:223
	// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_createERFilterNM2_const_StringR(const char* filename, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(std::string(filename));
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createERFilterNM2(const String &, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:223
	// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["filename", "minProbability"], ["const cv::String*", "float"]), _)]),
	void cv_text_createERFilterNM2_const_StringR_float(const char* filename, float minProbability, Result<cv::Ptr<cv::text::ERFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(std::string(filename), minProbability);
			Ok(new cv::Ptr<cv::text::ERFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOCRHMMTransitionsTable(const String &, std::vector<cv::String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:382
	// ("cv::text::createOCRHMMTransitionsTable", vec![(pred!(mut, ["vocabulary", "lexicon"], ["const cv::String*", "std::vector<cv::String>*"]), _)]),
	void cv_text_createOCRHMMTransitionsTable_const_StringR_vectorLStringGR(const char* vocabulary, std::vector<cv::String>* lexicon, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::text::createOCRHMMTransitionsTable(std::string(vocabulary), *lexicon);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOCRHMMTransitionsTable(std::string &, std::vector<std::string> &, OutputArray)(OutString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:380
	// ("cv::text::createOCRHMMTransitionsTable", vec![(pred!(mut, ["vocabulary", "lexicon", "transition_probabilities_table"], ["std::string*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	void cv_text_createOCRHMMTransitionsTable_stringR_vectorLstringGR_const__OutputArrayR(void** vocabulary, std::vector<std::string>* lexicon, const cv::_OutputArray* transition_probabilities_table, ResultVoid* ocvrs_return) {
		try {
			std::string vocabulary_out;
			cv::text::createOCRHMMTransitionsTable(vocabulary_out, *lexicon, *transition_probabilities_table);
			*vocabulary = ocvrs_create_string(vocabulary_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::detectRegions(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:366
	// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "groups_rects"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<cv::Rect>*"]), _)]),
	void cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR(const cv::_InputArray* image, const cv::Ptr<cv::text::ERFilter>* er_filter1, const cv::Ptr<cv::text::ERFilter>* er_filter2, std::vector<cv::Rect>* groups_rects, ResultVoid* ocvrs_return) {
		try {
			cv::text::detectRegions(*image, *er_filter1, *er_filter2, *groups_rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectRegions(InputArray, const Ptr<ERFilter> &, const Ptr<ERFilter> &, std::vector<Rect> &, int, const String &, float)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:366
	// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "groups_rects", "method", "filename", "minProbability"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<cv::Rect>*", "int", "const cv::String*", "float"]), _)]),
	void cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR_int_const_StringR_float(const cv::_InputArray* image, const cv::Ptr<cv::text::ERFilter>* er_filter1, const cv::Ptr<cv::text::ERFilter>* er_filter2, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbability, ResultVoid* ocvrs_return) {
		try {
			cv::text::detectRegions(*image, *er_filter1, *er_filter2, *groups_rects, method, std::string(filename), minProbability);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectRegions(InputArray, const Ptr<ERFilter> &, const Ptr<ERFilter> &, std::vector<std::vector<Point>> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:351
	// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "regions"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<std::vector<cv::Point>>*"]), _)]),
	void cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLvectorLPointGGR(const cv::_InputArray* image, const cv::Ptr<cv::text::ERFilter>* er_filter1, const cv::Ptr<cv::text::ERFilter>* er_filter2, std::vector<std::vector<cv::Point>>* regions, ResultVoid* ocvrs_return) {
		try {
			cv::text::detectRegions(*image, *er_filter1, *er_filter2, *regions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::detectTextSWT(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/swt_text_detection.hpp:20
	// ("cv::text::detectTextSWT", vec![(pred!(mut, ["input", "result", "dark_on_light"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "bool"]), _)]),
	void cv_text_detectTextSWT_const__InputArrayR_vectorLRectGR_bool(const cv::_InputArray* input, std::vector<cv::Rect>* result, bool dark_on_light, ResultVoid* ocvrs_return) {
		try {
			cv::text::detectTextSWT(*input, *result, dark_on_light);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectTextSWT(InputArray, std::vector<cv::Rect> &, bool, const _OutputArray &, const _OutputArray &)(InputArray, CppPassByVoidPtr, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/swt_text_detection.hpp:20
	// ("cv::text::detectTextSWT", vec![(pred!(mut, ["input", "result", "dark_on_light", "draw", "chainBBs"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "bool", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_text_detectTextSWT_const__InputArrayR_vectorLRectGR_bool_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* input, std::vector<cv::Rect>* result, bool dark_on_light, const cv::_OutputArray* draw, const cv::_OutputArray* chainBBs, ResultVoid* ocvrs_return) {
		try {
			cv::text::detectTextSWT(*input, *result, dark_on_light, *draw, *chainBBs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::erGrouping(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:316
	// ("cv::text::erGrouping", vec![(pred!(mut, ["img", "channels", "regions", "groups", "groups_rects"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::text::ERStat>>*", "std::vector<std::vector<cv::Vec2i>>*", "std::vector<cv::Rect>*"]), _)]),
	void cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR(const cv::_InputArray* img, const cv::_InputArray* channels, std::vector<std::vector<cv::text::ERStat>>* regions, std::vector<std::vector<cv::Vec2i>>* groups, std::vector<cv::Rect>* groups_rects, ResultVoid* ocvrs_return) {
		try {
			cv::text::erGrouping(*img, *channels, *regions, *groups, *groups_rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erGrouping(InputArray, InputArrayOfArrays, std::vector<std::vector<ERStat>> &, std::vector<std::vector<Vec2i>> &, std::vector<Rect> &, int, const std::string &, float)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:316
	// ("cv::text::erGrouping", vec![(pred!(mut, ["img", "channels", "regions", "groups", "groups_rects", "method", "filename", "minProbablity"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::text::ERStat>>*", "std::vector<std::vector<cv::Vec2i>>*", "std::vector<cv::Rect>*", "int", "const std::string*", "float"]), _)]),
	void cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR_int_const_stringR_float(const cv::_InputArray* img, const cv::_InputArray* channels, std::vector<std::vector<cv::text::ERStat>>* regions, std::vector<std::vector<cv::Vec2i>>* groups, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbablity, ResultVoid* ocvrs_return) {
		try {
			cv::text::erGrouping(*img, *channels, *regions, *groups, *groups_rects, method, std::string(filename), minProbablity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::erGrouping(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:324
	// ("cv::text::erGrouping", vec![(pred!(mut, ["image", "channel", "regions", "groups_rects"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::Point>>", "std::vector<cv::Rect>*"]), _)]),
	void cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR(const cv::_InputArray* image, const cv::_InputArray* channel, std::vector<std::vector<cv::Point>>* regions, std::vector<cv::Rect>* groups_rects, ResultVoid* ocvrs_return) {
		try {
			cv::text::erGrouping(*image, *channel, *regions, *groups_rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erGrouping(InputArray, InputArray, std::vector<std::vector<Point>>, std::vector<Rect> &, int, const String &, float)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:324
	// ("cv::text::erGrouping", vec![(pred!(mut, ["image", "channel", "regions", "groups_rects", "method", "filename", "minProbablity"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::Point>>", "std::vector<cv::Rect>*", "int", "const cv::String*", "float"]), _)]),
	void cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR_int_const_StringR_float(const cv::_InputArray* image, const cv::_InputArray* channel, std::vector<std::vector<cv::Point>>* regions, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbablity, ResultVoid* ocvrs_return) {
		try {
			cv::text::erGrouping(*image, *channel, *regions, *groups_rects, method, std::string(filename), minProbablity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadClassifierNM1(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:232
	// ("cv::text::loadClassifierNM1", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_loadClassifierNM1_const_StringR(const char* filename, Result<cv::Ptr<cv::text::ERFilter::Callback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM1(std::string(filename));
			Ok(new cv::Ptr<cv::text::ERFilter::Callback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadClassifierNM2(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:240
	// ("cv::text::loadClassifierNM2", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_loadClassifierNM2_const_StringR(const char* filename, Result<cv::Ptr<cv::text::ERFilter::Callback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM2(std::string(filename));
			Ok(new cv::Ptr<cv::text::ERFilter::Callback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadOCRBeamSearchClassifierCNN(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:526
	// ("cv::text::loadOCRBeamSearchClassifierCNN", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_loadOCRBeamSearchClassifierCNN_const_StringR(const char* filename, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback> ret = cv::text::loadOCRBeamSearchClassifierCNN(std::string(filename));
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadOCRHMMClassifierCNN(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:356
	// ("cv::text::loadOCRHMMClassifierCNN", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_loadOCRHMMClassifierCNN_const_StringR(const char* filename, Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierCNN(std::string(filename));
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadOCRHMMClassifierNM(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:343
	// ("cv::text::loadOCRHMMClassifierNM", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_text_loadOCRHMMClassifierNM_const_StringR(const char* filename, Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierNM(std::string(filename));
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadOCRHMMClassifier(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:365
	// ("cv::text::loadOCRHMMClassifier", vec![(pred!(mut, ["filename", "classifier"], ["const cv::String*", "int"]), _)]),
	void cv_text_loadOCRHMMClassifier_const_StringR_int(const char* filename, int classifier, Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifier(std::string(filename), classifier);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:96
	// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_BaseOCR_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::BaseOCR* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::BaseOCR::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:96
	// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
	void cv_text_BaseOCR_run_MatR_stringR(cv::text::BaseOCR* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:99
	// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_BaseOCR_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::BaseOCR* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::BaseOCR::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:99
	// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
	void cv_text_BaseOCR_run_MatR_MatR_stringR(cv::text::BaseOCR* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::BaseOCR::to_OCRBeamSearchDecoder() generated
	// ("cv::text::BaseOCR::to_OCRBeamSearchDecoder", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRBeamSearchDecoder* cv_text_BaseOCR_to_OCRBeamSearchDecoder(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRBeamSearchDecoder*>(instance);
	}

	// cv::text::BaseOCR::to_OCRHMMDecoder() generated
	// ("cv::text::BaseOCR::to_OCRHMMDecoder", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRHMMDecoder* cv_text_BaseOCR_to_OCRHMMDecoder(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRHMMDecoder*>(instance);
	}

	// cv::text::BaseOCR::to_OCRHolisticWordRecognizer() generated
	// ("cv::text::BaseOCR::to_OCRHolisticWordRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRHolisticWordRecognizer* cv_text_BaseOCR_to_OCRHolisticWordRecognizer(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRHolisticWordRecognizer*>(instance);
	}

	// cv::text::BaseOCR::to_OCRTesseract() generated
	// ("cv::text::BaseOCR::to_OCRTesseract", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRTesseract* cv_text_BaseOCR_to_OCRTesseract(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRTesseract*>(instance);
	}

	// cv::text::BaseOCR::delete() generated
	// ("cv::text::BaseOCR::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_BaseOCR_delete(cv::text::BaseOCR* instance) {
			delete instance;
	}

	// run(InputArray, std::vector<ERStat> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:151
	// ("cv::text::ERFilter::run", vec![(pred!(mut, ["image", "regions"], ["const cv::_InputArray*", "std::vector<cv::text::ERStat>*"]), _)]),
	void cv_text_ERFilter_run_const__InputArrayR_vectorLERStatGR(cv::text::ERFilter* instance, const cv::_InputArray* image, std::vector<cv::text::ERStat>* regions, ResultVoid* ocvrs_return) {
		try {
			instance->run(*image, *regions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCallback(const Ptr<ERFilter::Callback> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:155
	// ("cv::text::ERFilter::setCallback", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
	void cv_text_ERFilter_setCallback_const_PtrLCallbackGR(cv::text::ERFilter* instance, const cv::Ptr<cv::text::ERFilter::Callback>* cb, ResultVoid* ocvrs_return) {
		try {
			instance->setCallback(*cb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThresholdDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:156
	// ("cv::text::ERFilter::setThresholdDelta", vec![(pred!(mut, ["thresholdDelta"], ["int"]), _)]),
	void cv_text_ERFilter_setThresholdDelta_int(cv::text::ERFilter* instance, int thresholdDelta, ResultVoid* ocvrs_return) {
		try {
			instance->setThresholdDelta(thresholdDelta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:157
	// ("cv::text::ERFilter::setMinArea", vec![(pred!(mut, ["minArea"], ["float"]), _)]),
	void cv_text_ERFilter_setMinArea_float(cv::text::ERFilter* instance, float minArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:158
	// ("cv::text::ERFilter::setMaxArea", vec![(pred!(mut, ["maxArea"], ["float"]), _)]),
	void cv_text_ERFilter_setMaxArea_float(cv::text::ERFilter* instance, float maxArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxArea(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinProbability(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:159
	// ("cv::text::ERFilter::setMinProbability", vec![(pred!(mut, ["minProbability"], ["float"]), _)]),
	void cv_text_ERFilter_setMinProbability_float(cv::text::ERFilter* instance, float minProbability, ResultVoid* ocvrs_return) {
		try {
			instance->setMinProbability(minProbability);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinProbabilityDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:160
	// ("cv::text::ERFilter::setMinProbabilityDiff", vec![(pred!(mut, ["minProbabilityDiff"], ["float"]), _)]),
	void cv_text_ERFilter_setMinProbabilityDiff_float(cv::text::ERFilter* instance, float minProbabilityDiff, ResultVoid* ocvrs_return) {
		try {
			instance->setMinProbabilityDiff(minProbabilityDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNonMaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:161
	// ("cv::text::ERFilter::setNonMaxSuppression", vec![(pred!(mut, ["nonMaxSuppression"], ["bool"]), _)]),
	void cv_text_ERFilter_setNonMaxSuppression_bool(cv::text::ERFilter* instance, bool nonMaxSuppression, ResultVoid* ocvrs_return) {
		try {
			instance->setNonMaxSuppression(nonMaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumRejected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:162
	// ("cv::text::ERFilter::getNumRejected", vec![(pred!(const, [], []), _)]),
	void cv_text_ERFilter_getNumRejected_const(const cv::text::ERFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumRejected();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::ERFilter::to_Algorithm() generated
	// ("cv::text::ERFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_text_ERFilter_to_Algorithm(cv::text::ERFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::text::ERFilter::delete() generated
	// ("cv::text::ERFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_ERFilter_delete(cv::text::ERFilter* instance) {
			delete instance;
	}

	// eval(const ERStat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:135
	// ("cv::text::ERFilter::Callback::eval", vec![(pred!(mut, ["stat"], ["const cv::text::ERStat*"]), _)]),
	void cv_text_ERFilter_Callback_eval_const_ERStatR(cv::text::ERFilter::Callback* instance, const cv::text::ERStat* stat, Result<double>* ocvrs_return) {
		try {
			double ret = instance->eval(*stat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::ERFilter::Callback::delete() generated
	// ("cv::text::ERFilter::Callback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_ERFilter_Callback_delete(cv::text::ERFilter::Callback* instance) {
			delete instance;
	}

	// ERStat(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:70
	// ("cv::text::ERStat::ERStat", vec![(pred!(mut, ["level", "pixel", "x", "y"], ["int", "int", "int", "int"]), _)]),
	void cv_text_ERStat_ERStat_int_int_int_int(int level, int pixel, int x, int y, Result<cv::text::ERStat*>* ocvrs_return) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat(level, pixel, x, y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::ERStat::ERStat() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:70
	// ("cv::text::ERStat::ERStat", vec![(pred!(mut, [], []), _)]),
	void cv_text_ERStat_ERStat(Result<cv::text::ERStat*>* ocvrs_return) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::ERStat::pixel() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:75
	// ("cv::text::ERStat::pixel", vec![(pred!(const, [], []), _)]),
	int cv_text_ERStat_propPixel_const(const cv::text::ERStat* instance) {
			int ret = instance->pixel;
			return ret;
	}

	// cv::text::ERStat::setPixel(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:75
	// ("cv::text::ERStat::setPixel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_text_ERStat_propPixel_const_int(cv::text::ERStat* instance, const int val) {
			instance->pixel = val;
	}

	// cv::text::ERStat::level() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:76
	// ("cv::text::ERStat::level", vec![(pred!(const, [], []), _)]),
	int cv_text_ERStat_propLevel_const(const cv::text::ERStat* instance) {
			int ret = instance->level;
			return ret;
	}

	// cv::text::ERStat::setLevel(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:76
	// ("cv::text::ERStat::setLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_text_ERStat_propLevel_const_int(cv::text::ERStat* instance, const int val) {
			instance->level = val;
	}

	// cv::text::ERStat::area() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:79
	// ("cv::text::ERStat::area", vec![(pred!(const, [], []), _)]),
	int cv_text_ERStat_propArea_const(const cv::text::ERStat* instance) {
			int ret = instance->area;
			return ret;
	}

	// cv::text::ERStat::setArea(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:79
	// ("cv::text::ERStat::setArea", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_text_ERStat_propArea_const_int(cv::text::ERStat* instance, const int val) {
			instance->area = val;
	}

	// cv::text::ERStat::perimeter() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:80
	// ("cv::text::ERStat::perimeter", vec![(pred!(const, [], []), _)]),
	int cv_text_ERStat_propPerimeter_const(const cv::text::ERStat* instance) {
			int ret = instance->perimeter;
			return ret;
	}

	// cv::text::ERStat::setPerimeter(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:80
	// ("cv::text::ERStat::setPerimeter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_text_ERStat_propPerimeter_const_int(cv::text::ERStat* instance, const int val) {
			instance->perimeter = val;
	}

	// cv::text::ERStat::euler() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:81
	// ("cv::text::ERStat::euler", vec![(pred!(const, [], []), _)]),
	int cv_text_ERStat_propEuler_const(const cv::text::ERStat* instance) {
			int ret = instance->euler;
			return ret;
	}

	// cv::text::ERStat::setEuler(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:81
	// ("cv::text::ERStat::setEuler", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_text_ERStat_propEuler_const_int(cv::text::ERStat* instance, const int val) {
			instance->euler = val;
	}

	// cv::text::ERStat::rect() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:82
	// ("cv::text::ERStat::rect", vec![(pred!(const, [], []), _)]),
	void cv_text_ERStat_propRect_const(const cv::text::ERStat* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->rect;
			*ocvrs_return = ret;
	}

	// cv::text::ERStat::setRect(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:82
	// ("cv::text::ERStat::setRect", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_text_ERStat_propRect_const_Rect(cv::text::ERStat* instance, const cv::Rect* val) {
			instance->rect = *val;
	}

	// cv::text::ERStat::raw_moments() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:83
	// ("cv::text::ERStat::raw_moments", vec![(pred!(const, [], []), _)]),
	const double** cv_text_ERStat_propRaw_moments_const(const cv::text::ERStat* instance) {
			const double(*ret)[2] = &instance->raw_moments;
			return (const double**)ret;
	}

	// cv::text::ERStat::raw_momentsMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:83
	// ("cv::text::ERStat::raw_momentsMut", vec![(pred!(mut, [], []), _)]),
	double** cv_text_ERStat_propRaw_moments(cv::text::ERStat* instance) {
			double(*ret)[2] = &instance->raw_moments;
			return (double**)ret;
	}

	// cv::text::ERStat::central_moments() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:84
	// ("cv::text::ERStat::central_moments", vec![(pred!(const, [], []), _)]),
	const double** cv_text_ERStat_propCentral_moments_const(const cv::text::ERStat* instance) {
			const double(*ret)[3] = &instance->central_moments;
			return (const double**)ret;
	}

	// cv::text::ERStat::central_momentsMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:84
	// ("cv::text::ERStat::central_momentsMut", vec![(pred!(mut, [], []), _)]),
	double** cv_text_ERStat_propCentral_moments(cv::text::ERStat* instance) {
			double(*ret)[3] = &instance->central_moments;
			return (double**)ret;
	}

	// cv::text::ERStat::med_crossings() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:86
	// ("cv::text::ERStat::med_crossings", vec![(pred!(const, [], []), _)]),
	float cv_text_ERStat_propMed_crossings_const(const cv::text::ERStat* instance) {
			float ret = instance->med_crossings;
			return ret;
	}

	// cv::text::ERStat::setMed_crossings(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:86
	// ("cv::text::ERStat::setMed_crossings", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_text_ERStat_propMed_crossings_const_float(cv::text::ERStat* instance, const float val) {
			instance->med_crossings = val;
	}

	// cv::text::ERStat::hole_area_ratio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:89
	// ("cv::text::ERStat::hole_area_ratio", vec![(pred!(const, [], []), _)]),
	float cv_text_ERStat_propHole_area_ratio_const(const cv::text::ERStat* instance) {
			float ret = instance->hole_area_ratio;
			return ret;
	}

	// cv::text::ERStat::setHole_area_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:89
	// ("cv::text::ERStat::setHole_area_ratio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_text_ERStat_propHole_area_ratio_const_float(cv::text::ERStat* instance, const float val) {
			instance->hole_area_ratio = val;
	}

	// cv::text::ERStat::convex_hull_ratio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:90
	// ("cv::text::ERStat::convex_hull_ratio", vec![(pred!(const, [], []), _)]),
	float cv_text_ERStat_propConvex_hull_ratio_const(const cv::text::ERStat* instance) {
			float ret = instance->convex_hull_ratio;
			return ret;
	}

	// cv::text::ERStat::setConvex_hull_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:90
	// ("cv::text::ERStat::setConvex_hull_ratio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_text_ERStat_propConvex_hull_ratio_const_float(cv::text::ERStat* instance, const float val) {
			instance->convex_hull_ratio = val;
	}

	// cv::text::ERStat::num_inflexion_points() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:91
	// ("cv::text::ERStat::num_inflexion_points", vec![(pred!(const, [], []), _)]),
	float cv_text_ERStat_propNum_inflexion_points_const(const cv::text::ERStat* instance) {
			float ret = instance->num_inflexion_points;
			return ret;
	}

	// cv::text::ERStat::setNum_inflexion_points(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:91
	// ("cv::text::ERStat::setNum_inflexion_points", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_text_ERStat_propNum_inflexion_points_const_float(cv::text::ERStat* instance, const float val) {
			instance->num_inflexion_points = val;
	}

	// cv::text::ERStat::probability() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:100
	// ("cv::text::ERStat::probability", vec![(pred!(const, [], []), _)]),
	double cv_text_ERStat_propProbability_const(const cv::text::ERStat* instance) {
			double ret = instance->probability;
			return ret;
	}

	// cv::text::ERStat::setProbability(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:100
	// ("cv::text::ERStat::setProbability", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_text_ERStat_propProbability_const_double(cv::text::ERStat* instance, const double val) {
			instance->probability = val;
	}

	// cv::text::ERStat::parent() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:103
	// ("cv::text::ERStat::parent", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propParent(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->parent;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setParent(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:103
	// ("cv::text::ERStat::setParent", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propParent_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->parent = val;
	}

	// cv::text::ERStat::child() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:104
	// ("cv::text::ERStat::child", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propChild(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->child;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setChild(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:104
	// ("cv::text::ERStat::setChild", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propChild_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->child = val;
	}

	// cv::text::ERStat::next() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:105
	// ("cv::text::ERStat::next", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propNext(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->next;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setNext(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:105
	// ("cv::text::ERStat::setNext", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propNext_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->next = val;
	}

	// cv::text::ERStat::prev() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:106
	// ("cv::text::ERStat::prev", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propPrev(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->prev;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setPrev(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:106
	// ("cv::text::ERStat::setPrev", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propPrev_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->prev = val;
	}

	// cv::text::ERStat::local_maxima() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:109
	// ("cv::text::ERStat::local_maxima", vec![(pred!(const, [], []), _)]),
	bool cv_text_ERStat_propLocal_maxima_const(const cv::text::ERStat* instance) {
			bool ret = instance->local_maxima;
			return ret;
	}

	// cv::text::ERStat::setLocal_maxima(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:109
	// ("cv::text::ERStat::setLocal_maxima", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_text_ERStat_propLocal_maxima_const_bool(cv::text::ERStat* instance, const bool val) {
			instance->local_maxima = val;
	}

	// cv::text::ERStat::max_probability_ancestor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:110
	// ("cv::text::ERStat::max_probability_ancestor", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propMax_probability_ancestor(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->max_probability_ancestor;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setMax_probability_ancestor(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:110
	// ("cv::text::ERStat::setMax_probability_ancestor", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propMax_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->max_probability_ancestor = val;
	}

	// cv::text::ERStat::min_probability_ancestor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:111
	// ("cv::text::ERStat::min_probability_ancestor", vec![(pred!(mut, [], []), _)]),
	cv::text::ERStat* cv_text_ERStat_propMin_probability_ancestor(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->min_probability_ancestor;
			return new cv::text::ERStat(*ret);
	}

	// cv::text::ERStat::setMin_probability_ancestor(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/erfilter.hpp:111
	// ("cv::text::ERStat::setMin_probability_ancestor", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
	void cv_text_ERStat_propMin_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->min_probability_ancestor = val;
	}

	// cv::text::ERStat::delete() generated
	// ("cv::text::ERStat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_ERStat_delete(cv::text::ERStat* instance) {
			delete instance;
	}

	// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:447
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:447
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_MatR_stringR(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:451
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:451
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:456
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:456
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:458
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:458
	// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<OCRBeamSearchDecoder::ClassifierCallback>, const std::string &, InputArray, InputArray, text::decoder_mode, int)(CppPassByVoidPtr, InString, InputArray, InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:479
	// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "beam_size"], ["const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>", "const std::string*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::text::decoder_mode", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, InputArray, InputArray, text::decoder_mode, int)(InString, InString, InputArray, InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:496
	// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "beam_size"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::text::decoder_mode", "int"]), _)]),
	void cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::create(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:496
	// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::defaultNew() generated
	// ("cv::text::OCRBeamSearchDecoder::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::text::OCRBeamSearchDecoder* cv_text_OCRBeamSearchDecoder_defaultNew_const() {
			cv::text::OCRBeamSearchDecoder* ret = new cv::text::OCRBeamSearchDecoder();
			return ret;
	}

	// cv::text::OCRBeamSearchDecoder::to_BaseOCR() generated
	// ("cv::text::OCRBeamSearchDecoder::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::text::BaseOCR* cv_text_OCRBeamSearchDecoder_to_BaseOCR(cv::text::OCRBeamSearchDecoder* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}

	// cv::text::OCRBeamSearchDecoder::delete() generated
	// ("cv::text::OCRBeamSearchDecoder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRBeamSearchDecoder_delete(cv::text::OCRBeamSearchDecoder* instance) {
			delete instance;
	}

	// eval(InputArray, std::vector<std::vector<double>> &, std::vector<int> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:419
	// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::eval", vec![(pred!(mut, ["image", "recognition_probabilities", "oversegmentation"], ["const cv::_InputArray*", "std::vector<std::vector<double>>*", "std::vector<int>*"]), _)]),
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLvectorLdoubleGGR_vectorLintGR(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<std::vector<double>>* recognition_probabilities, std::vector<int>* oversegmentation, ResultVoid* ocvrs_return) {
		try {
			instance->eval(*image, *recognition_probabilities, *oversegmentation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:421
	// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::getWindowSize", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStepSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:422
	// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::getStepSize", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getStepSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRBeamSearchDecoder::ClassifierCallback::defaultNew() generated
	// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::text::OCRBeamSearchDecoder::ClassifierCallback* cv_text_OCRBeamSearchDecoder_ClassifierCallback_defaultNew_const() {
			cv::text::OCRBeamSearchDecoder::ClassifierCallback* ret = new cv::text::OCRBeamSearchDecoder::ClassifierCallback();
			return ret;
	}

	// cv::text::OCRBeamSearchDecoder::ClassifierCallback::delete() generated
	// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_delete(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
			delete instance;
	}

	// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:243
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:243
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRHMMDecoder_run_MatR_stringR(cv::text::OCRHMMDecoder* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:270
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:270
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRHMMDecoder_run_MatR_MatR_stringR(cv::text::OCRHMMDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:275
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:275
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:277
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:277
	// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<OCRHMMDecoder::ClassifierCallback>, const String &, InputArray, InputArray, int)(CppPassByVoidPtr, InString, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:296
	// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR_int(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::create(CppPassByVoidPtr, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:296
	// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, InputArray, InputArray, int, int)(InString, InString, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:309
	// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "classifier"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, int classifier, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, classifier);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::create(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:309
	// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::defaultNew() generated
	// ("cv::text::OCRHMMDecoder::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::text::OCRHMMDecoder* cv_text_OCRHMMDecoder_defaultNew_const() {
			cv::text::OCRHMMDecoder* ret = new cv::text::OCRHMMDecoder();
			return ret;
	}

	// cv::text::OCRHMMDecoder::to_BaseOCR() generated
	// ("cv::text::OCRHMMDecoder::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::text::BaseOCR* cv_text_OCRHMMDecoder_to_BaseOCR(cv::text::OCRHMMDecoder* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}

	// cv::text::OCRHMMDecoder::delete() generated
	// ("cv::text::OCRHMMDecoder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRHMMDecoder_delete(cv::text::OCRHMMDecoder* instance) {
			delete instance;
	}

	// eval(InputArray, std::vector<int> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:218
	// ("cv::text::OCRHMMDecoder::ClassifierCallback::eval", vec![(pred!(mut, ["image", "out_class", "out_confidence"], ["const cv::_InputArray*", "std::vector<int>*", "std::vector<double>*"]), _)]),
	void cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLintGR_vectorLdoubleGR(cv::text::OCRHMMDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<int>* out_class, std::vector<double>* out_confidence, ResultVoid* ocvrs_return) {
		try {
			instance->eval(*image, *out_class, *out_confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHMMDecoder::ClassifierCallback::defaultNew() generated
	// ("cv::text::OCRHMMDecoder::ClassifierCallback::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::text::OCRHMMDecoder::ClassifierCallback* cv_text_OCRHMMDecoder_ClassifierCallback_defaultNew_const() {
			cv::text::OCRHMMDecoder::ClassifierCallback* ret = new cv::text::OCRHMMDecoder::ClassifierCallback();
			return ret;
	}

	// cv::text::OCRHMMDecoder::ClassifierCallback::delete() generated
	// ("cv::text::OCRHMMDecoder::ClassifierCallback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRHMMDecoder_ClassifierCallback_delete(cv::text::OCRHMMDecoder::ClassifierCallback* instance) {
			delete instance;
	}

	// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:540
	// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHolisticWordRecognizer::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:540
	// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRHolisticWordRecognizer_run_MatR_stringR(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:570
	// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHolisticWordRecognizer::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:570
	// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::string &, const std::string &, const std::string &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:580
	// ("cv::text::OCRHolisticWordRecognizer::create", vec![(pred!(mut, ["archFilename", "weightsFilename", "wordsFilename"], ["const std::string*", "const std::string*", "const std::string*"]), _)]),
	void cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(const char* archFilename, const char* weightsFilename, const char* wordsFilename, Result<cv::Ptr<cv::text::OCRHolisticWordRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHolisticWordRecognizer> ret = cv::text::OCRHolisticWordRecognizer::create(std::string(archFilename), std::string(weightsFilename), std::string(wordsFilename));
			Ok(new cv::Ptr<cv::text::OCRHolisticWordRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRHolisticWordRecognizer::to_BaseOCR() generated
	// ("cv::text::OCRHolisticWordRecognizer::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::text::BaseOCR* cv_text_OCRHolisticWordRecognizer_to_BaseOCR(cv::text::OCRHolisticWordRecognizer* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}

	// cv::text::OCRHolisticWordRecognizer::delete() generated
	// ("cv::text::OCRHolisticWordRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRHolisticWordRecognizer_delete(cv::text::OCRHolisticWordRecognizer* instance) {
			delete instance;
	}

	// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:135
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRTesseract_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRTesseract* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:135
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRTesseract_run_MatR_stringR(cv::text::OCRTesseract* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:139
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
	void cv_text_OCRTesseract_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRTesseract* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:139
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
	void cv_text_OCRTesseract_run_MatR_MatR_stringR(cv::text::OCRTesseract* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:144
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRTesseract_run_const__InputArrayR_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:144
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRTesseract_run_const__InputArrayR_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:146
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:146
	// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWhiteList(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:148
	// ("cv::text::OCRTesseract::setWhiteList", vec![(pred!(mut, ["char_whitelist"], ["const cv::String*"]), _)]),
	void cv_text_OCRTesseract_setWhiteList_const_StringR(cv::text::OCRTesseract* instance, const char* char_whitelist, ResultVoid* ocvrs_return) {
		try {
			instance->setWhiteList(std::string(char_whitelist));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const char *, const char *, const char *, int, int)(InString, InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:167
	// ("cv::text::OCRTesseract::create", vec![(pred!(mut, ["datapath", "language", "char_whitelist", "oem", "psmode"], ["const char*", "const char*", "const char*", "int", "int"]), _)]),
	void cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode, Result<cv::Ptr<cv::text::OCRTesseract>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
			Ok(new cv::Ptr<cv::text::OCRTesseract>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/ocr.hpp:167
	// ("cv::text::OCRTesseract::create", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRTesseract_create(Result<cv::Ptr<cv::text::OCRTesseract>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create();
			Ok(new cv::Ptr<cv::text::OCRTesseract>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::OCRTesseract::to_BaseOCR() generated
	// ("cv::text::OCRTesseract::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::text::BaseOCR* cv_text_OCRTesseract_to_BaseOCR(cv::text::OCRTesseract* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}

	// cv::text::OCRTesseract::delete() generated
	// ("cv::text::OCRTesseract::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_OCRTesseract_delete(cv::text::OCRTesseract* instance) {
			delete instance;
	}

	// detect(InputArray, std::vector<Rect> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/textDetector.hpp:30
	// ("cv::text::TextDetector::detect", vec![(pred!(mut, ["inputImage", "Bbox", "confidence"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<float>*"]), _)]),
	void cv_text_TextDetector_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(cv::text::TextDetector* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::TextDetector::to_TextDetectorCNN() generated
	// ("cv::text::TextDetector::to_TextDetectorCNN", vec![(pred!(mut, [], []), _)]),
	cv::text::TextDetectorCNN* cv_text_TextDetector_to_TextDetectorCNN(cv::text::TextDetector* instance) {
			return dynamic_cast<cv::text::TextDetectorCNN*>(instance);
	}

	// cv::text::TextDetector::delete() generated
	// ("cv::text::TextDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_TextDetector_delete(cv::text::TextDetector* instance) {
			delete instance;
	}

	// detect(InputArray, std::vector<Rect> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/textDetector.hpp:51
	// ("cv::text::TextDetectorCNN::detect", vec![(pred!(mut, ["inputImage", "Bbox", "confidence"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<float>*"]), _)]),
	void cv_text_TextDetectorCNN_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(cv::text::TextDetectorCNN* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &, std::vector<Size>)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/textDetector.hpp:60
	// ("cv::text::TextDetectorCNN::create", vec![(pred!(mut, ["modelArchFilename", "modelWeightsFilename", "detectionSizes"], ["const cv::String*", "const cv::String*", "std::vector<cv::Size>"]), _)]),
	void cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vectorLSizeG(const char* modelArchFilename, const char* modelWeightsFilename, std::vector<cv::Size>* detectionSizes, Result<cv::Ptr<cv::text::TextDetectorCNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename), *detectionSizes);
			Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/text/textDetector.hpp:65
	// ("cv::text::TextDetectorCNN::create", vec![(pred!(mut, ["modelArchFilename", "modelWeightsFilename"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_text_TextDetectorCNN_create_const_StringR_const_StringR(const char* modelArchFilename, const char* modelWeightsFilename, Result<cv::Ptr<cv::text::TextDetectorCNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename));
			Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::text::TextDetectorCNN::to_TextDetector() generated
	// ("cv::text::TextDetectorCNN::to_TextDetector", vec![(pred!(mut, [], []), _)]),
	cv::text::TextDetector* cv_text_TextDetectorCNN_to_TextDetector(cv::text::TextDetectorCNN* instance) {
			return dynamic_cast<cv::text::TextDetector*>(instance);
	}

	// cv::text::TextDetectorCNN::delete() generated
	// ("cv::text::TextDetectorCNN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_text_TextDetectorCNN_delete(cv::text::TextDetectorCNN* instance) {
			delete instance;
	}

}
