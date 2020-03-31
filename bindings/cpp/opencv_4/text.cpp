#include "common.hpp"
#include <opencv2/text.hpp>
#include "text_types.hpp"

extern "C" {
	Result_void cv_text_MSERsToERStats_const__InputArrayX_vector_vector_Point__X_vector_vector_ERStat__X(const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* contours, std::vector<std::vector<cv::text::ERStat>>* regions) {
		try {
			cv::text::MSERsToERStats(*image, *contours, *regions);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_computeNMChannels_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* _src, const cv::_OutputArray* _channels, int _mode) {
		try {
			cv::text::computeNMChannels(*_src, *_channels, _mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::text::ERFilter>*> cv_text_createERFilterNM1_const_Ptr_Callback_X_int_float_float_float_bool_float(const cv::Ptr<cv::text::ERFilter::Callback>* cb, int thresholdDelta, float minArea, float maxArea, float minProbability, bool nonMaxSuppression, float minProbabilityDiff) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(*cb, thresholdDelta, minArea, maxArea, minProbability, nonMaxSuppression, minProbabilityDiff);
			return Ok(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter>*>)
	}
	
	Result<cv::Ptr<cv::text::ERFilter>*> cv_text_createERFilterNM1_const_StringX_int_float_float_float_bool_float(const char* filename, int thresholdDelta, float minArea, float maxArea, float minProbability, bool nonMaxSuppression, float minProbabilityDiff) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(std::string(filename), thresholdDelta, minArea, maxArea, minProbability, nonMaxSuppression, minProbabilityDiff);
			return Ok(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter>*>)
	}
	
	Result<cv::Ptr<cv::text::ERFilter>*> cv_text_createERFilterNM2_const_Ptr_Callback_X_float(const cv::Ptr<cv::text::ERFilter::Callback>* cb, float minProbability) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(*cb, minProbability);
			return Ok(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter>*>)
	}
	
	Result<cv::Ptr<cv::text::ERFilter>*> cv_text_createERFilterNM2_const_StringX_float(const char* filename, float minProbability) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(std::string(filename), minProbability);
			return Ok(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter>*>)
	}
	
	Result<cv::Mat*> cv_text_createOCRHMMTransitionsTable_const_StringX_vector_String_X(const char* vocabulary, std::vector<cv::String>* lexicon) {
		try {
			cv::Mat ret = cv::text::createOCRHMMTransitionsTable(std::string(vocabulary), *lexicon);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_text_createOCRHMMTransitionsTable_stringX_vector_string_X_const__OutputArrayX(void** vocabulary, std::vector<std::string>* lexicon, const cv::_OutputArray* transition_probabilities_table) {
		try {
			std::string vocabulary_out;
			cv::text::createOCRHMMTransitionsTable(vocabulary_out, *lexicon, *transition_probabilities_table);
			*vocabulary = ocvrs_create_string(vocabulary_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_detectRegions_const__InputArrayX_const_Ptr_ERFilter_X_const_Ptr_ERFilter_X_vector_Rect_X_int_const_StringX_float(const cv::_InputArray* image, const cv::Ptr<cv::text::ERFilter>* er_filter1, const cv::Ptr<cv::text::ERFilter>* er_filter2, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbability) {
		try {
			cv::text::detectRegions(*image, *er_filter1, *er_filter2, *groups_rects, method, std::string(filename), minProbability);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_detectRegions_const__InputArrayX_const_Ptr_ERFilter_X_const_Ptr_ERFilter_X_vector_vector_Point__X(const cv::_InputArray* image, const cv::Ptr<cv::text::ERFilter>* er_filter1, const cv::Ptr<cv::text::ERFilter>* er_filter2, std::vector<std::vector<cv::Point>>* regions) {
		try {
			cv::text::detectRegions(*image, *er_filter1, *er_filter2, *regions);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_ERStat__X_vector_vector_Vec2i__X_vector_Rect_X_int_const_stringX_float(const cv::_InputArray* img, const cv::_InputArray* channels, std::vector<std::vector<cv::text::ERStat>>* regions, std::vector<std::vector<cv::Vec2i>>* groups, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbablity) {
		try {
			cv::text::erGrouping(*img, *channels, *regions, *groups, *groups_rects, method, std::string(filename), minProbablity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_Point___vector_Rect_X_int_const_StringX_float(const cv::_InputArray* image, const cv::_InputArray* channel, std::vector<std::vector<cv::Point>>* regions, std::vector<cv::Rect>* groups_rects, int method, const char* filename, float minProbablity) {
		try {
			cv::text::erGrouping(*image, *channel, *regions, *groups_rects, method, std::string(filename), minProbablity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::text::ERFilter::Callback>*> cv_text_loadClassifierNM1_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM1(std::string(filename));
			return Ok(new cv::Ptr<cv::text::ERFilter::Callback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter::Callback>*>)
	}
	
	Result<cv::Ptr<cv::text::ERFilter::Callback>*> cv_text_loadClassifierNM2_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM2(std::string(filename));
			return Ok(new cv::Ptr<cv::text::ERFilter::Callback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::ERFilter::Callback>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>*> cv_text_loadOCRBeamSearchClassifierCNN_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback> ret = cv::text::loadOCRBeamSearchClassifierCNN(std::string(filename));
			return Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*> cv_text_loadOCRHMMClassifierCNN_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierCNN(std::string(filename));
			return Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*> cv_text_loadOCRHMMClassifierNM_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierNM(std::string(filename));
			return Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*> cv_text_loadOCRHMMClassifier_const_StringX_int(const char* filename, int classifier) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifier(std::string(filename), classifier);
			return Ok(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>)
	}
	
	Result_void cv_text_BaseOCR_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::BaseOCR* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_BaseOCR_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::BaseOCR* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_run_const__InputArrayX_vector_ERStat_X(cv::text::ERFilter* instance, const cv::_InputArray* image, std::vector<cv::text::ERStat>* regions) {
		try {
			instance->run(*image, *regions);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setCallback_const_Ptr_Callback_X(cv::text::ERFilter* instance, const cv::Ptr<cv::text::ERFilter::Callback>* cb) {
		try {
			instance->setCallback(*cb);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setThresholdDelta_int(cv::text::ERFilter* instance, int thresholdDelta) {
		try {
			instance->setThresholdDelta(thresholdDelta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinArea_float(cv::text::ERFilter* instance, float minArea) {
		try {
			instance->setMinArea(minArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMaxArea_float(cv::text::ERFilter* instance, float maxArea) {
		try {
			instance->setMaxArea(maxArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinProbability_float(cv::text::ERFilter* instance, float minProbability) {
		try {
			instance->setMinProbability(minProbability);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinProbabilityDiff_float(cv::text::ERFilter* instance, float minProbabilityDiff) {
		try {
			instance->setMinProbabilityDiff(minProbabilityDiff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setNonMaxSuppression_bool(cv::text::ERFilter* instance, bool nonMaxSuppression) {
		try {
			instance->setNonMaxSuppression(nonMaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERFilter_getNumRejected_const(const cv::text::ERFilter* instance) {
		try {
			int ret = instance->getNumRejected();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_text_ERFilter_Callback_eval_const_ERStatX(cv::text::ERFilter::Callback* instance, const cv::text::ERStat* stat) {
		try {
			double ret = instance->eval(*stat);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_text_ERStat_pixel_const(const cv::text::ERStat* instance) {
		try {
			int ret = instance->pixel;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setPixel_int(cv::text::ERStat* instance, int val) {
		try {
			instance->pixel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_level_const(const cv::text::ERStat* instance) {
		try {
			int ret = instance->level;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setLevel_int(cv::text::ERStat* instance, int val) {
		try {
			instance->level = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_area_const(const cv::text::ERStat* instance) {
		try {
			int ret = instance->area;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setArea_int(cv::text::ERStat* instance, int val) {
		try {
			instance->area = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_perimeter_const(const cv::text::ERStat* instance) {
		try {
			int ret = instance->perimeter;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setPerimeter_int(cv::text::ERStat* instance, int val) {
		try {
			instance->perimeter = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_euler_const(const cv::text::ERStat* instance) {
		try {
			int ret = instance->euler;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setEuler_int(cv::text::ERStat* instance, int val) {
		try {
			instance->euler = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_text_ERStat_rect_const(const cv::text::ERStat* instance) {
		try {
			cv::Rect ret = instance->rect;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_text_ERStat_setRect_Rect(cv::text::ERStat* instance, const cv::Rect* val) {
		try {
			instance->rect = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double(*)[2]> cv_text_ERStat_raw_moments(cv::text::ERStat* instance) {
		try {
			double(*ret)[2] = &instance->raw_moments;
			return Ok(ret);
		} OCVRS_CATCH(Result<double(*)[2]>)
	}
	
	Result<double(*)[3]> cv_text_ERStat_central_moments(cv::text::ERStat* instance) {
		try {
			double(*ret)[3] = &instance->central_moments;
			return Ok(ret);
		} OCVRS_CATCH(Result<double(*)[3]>)
	}
	
	Result<float> cv_text_ERStat_med_crossings_const(const cv::text::ERStat* instance) {
		try {
			float ret = instance->med_crossings;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setMed_crossings_float(cv::text::ERStat* instance, float val) {
		try {
			instance->med_crossings = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_hole_area_ratio_const(const cv::text::ERStat* instance) {
		try {
			float ret = instance->hole_area_ratio;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setHole_area_ratio_float(cv::text::ERStat* instance, float val) {
		try {
			instance->hole_area_ratio = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_convex_hull_ratio_const(const cv::text::ERStat* instance) {
		try {
			float ret = instance->convex_hull_ratio;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setConvex_hull_ratio_float(cv::text::ERStat* instance, float val) {
		try {
			instance->convex_hull_ratio = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_num_inflexion_points_const(const cv::text::ERStat* instance) {
		try {
			float ret = instance->num_inflexion_points;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setNum_inflexion_points_float(cv::text::ERStat* instance, float val) {
		try {
			instance->num_inflexion_points = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_text_ERStat_probability_const(const cv::text::ERStat* instance) {
		try {
			double ret = instance->probability;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_text_ERStat_setProbability_double(cv::text::ERStat* instance, double val) {
		try {
			instance->probability = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_parent(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->parent;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setParent_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->parent = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_child(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->child;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setChild_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->child = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_next(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->next;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setNext_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->next = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_prev(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->prev;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setPrev_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->prev = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_text_ERStat_local_maxima_const(const cv::text::ERStat* instance) {
		try {
			bool ret = instance->local_maxima;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_text_ERStat_setLocal_maxima_bool(cv::text::ERStat* instance, bool val) {
		try {
			instance->local_maxima = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_max_probability_ancestor(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->max_probability_ancestor;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setMax_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->max_probability_ancestor = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::text::ERStat**> cv_text_ERStat_min_probability_ancestor(cv::text::ERStat* instance) {
		try {
			cv::text::ERStat* ret = instance->min_probability_ancestor;
			return Ok(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<cv::text::ERStat**>)
	}
	
	Result_void cv_text_ERStat_setMin_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* val) {
		try {
			instance->min_probability_ancestor = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ERStat_delete(cv::text::ERStat* instance) {
		delete instance;
	}
	Result<cv::text::ERStat*> cv_text_ERStat_ERStat_int_int_int_int(int level, int pixel, int x, int y) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat(level, pixel, x, y);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::text::ERStat*>)
	}
	
	void cv_OCRBeamSearchDecoder_delete(cv::text::OCRBeamSearchDecoder* instance) {
		delete instance;
	}
	Result_void cv_text_OCRBeamSearchDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRBeamSearchDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_const__InputArrayX_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*> cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_stringX_const__InputArrayX_const__InputArrayX_decoder_mode_int(const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			return Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*> cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int_int(const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, int beam_size) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			return Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*> cv_text_OCRBeamSearchDecoder_create_const_StringX_const_StringX_const__InputArrayX_const__InputArrayX_int_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, int beam_size) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			return Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>)
	}
	
	void cv_OCRBeamSearchDecoder_ClassifierCallback_delete(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
		delete instance;
	}
	Result_void cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayX_vector_vector_double__X_vector_int_X(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<std::vector<double>>* recognition_probabilities, std::vector<int>* oversegmentation) {
		try {
			instance->eval(*image, *recognition_probabilities, *oversegmentation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
		try {
			int ret = instance->getWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
		try {
			int ret = instance->getStepSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_OCRHMMDecoder_delete(cv::text::OCRHMMDecoder* instance) {
		delete instance;
	}
	Result_void cv_text_OCRHMMDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRHMMDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_run_const__InputArrayX_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_run_const__InputArrayX_const__InputArrayX_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Ptr<cv::text::OCRHMMDecoder>*> cv_text_OCRHMMDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode);
			return Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHMMDecoder>*>)
	}
	
	Result<cv::Ptr<cv::text::OCRHMMDecoder>*> cv_text_OCRHMMDecoder_create_const_StringX_const_StringX_const__InputArrayX_const__InputArrayX_int_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, int classifier) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, classifier);
			return Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHMMDecoder>*>)
	}
	
	void cv_OCRHMMDecoder_ClassifierCallback_delete(cv::text::OCRHMMDecoder::ClassifierCallback* instance) {
		delete instance;
	}
	Result_void cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayX_vector_int_X_vector_double_X(cv::text::OCRHMMDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<int>* out_class, std::vector<double>* out_confidence) {
		try {
			instance->eval(*image, *out_class, *out_confidence);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRHolisticWordRecognizer_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRHolisticWordRecognizer_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::text::OCRHolisticWordRecognizer>*> cv_text_OCRHolisticWordRecognizer_create_const_stringX_const_stringX_const_stringX(const char* archFilename, const char* weightsFilename, const char* wordsFilename) {
		try {
			cv::Ptr<cv::text::OCRHolisticWordRecognizer> ret = cv::text::OCRHolisticWordRecognizer::create(std::string(archFilename), std::string(weightsFilename), std::string(wordsFilename));
			return Ok(new cv::Ptr<cv::text::OCRHolisticWordRecognizer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRHolisticWordRecognizer>*>)
	}
	
	Result_void cv_text_OCRTesseract_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRTesseract* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRTesseract_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(cv::text::OCRTesseract* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRTesseract_run_const__InputArrayX_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRTesseract_run_const__InputArrayX_const__InputArrayX_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_OCRTesseract_setWhiteList_const_StringX(cv::text::OCRTesseract* instance, const char* char_whitelist) {
		try {
			instance->setWhiteList(std::string(char_whitelist));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::text::OCRTesseract>*> cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
			return Ok(new cv::Ptr<cv::text::OCRTesseract>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::OCRTesseract>*>)
	}
	
	Result_void cv_text_TextDetector_detect_const__InputArrayX_vector_Rect_X_vector_float_X(cv::text::TextDetector* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_TextDetectorCNN_detect_const__InputArrayX_vector_Rect_X_vector_float_X(cv::text::TextDetectorCNN* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::text::TextDetectorCNN>*> cv_text_TextDetectorCNN_create_const_StringX_const_StringX_vector_Size_(const char* modelArchFilename, const char* modelWeightsFilename, std::vector<cv::Size>* detectionSizes) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename), *detectionSizes);
			return Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::TextDetectorCNN>*>)
	}
	
	Result<cv::Ptr<cv::text::TextDetectorCNN>*> cv_text_TextDetectorCNN_create_const_StringX_const_StringX(const char* modelArchFilename, const char* modelWeightsFilename) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename));
			return Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::text::TextDetectorCNN>*>)
	}
	
}
