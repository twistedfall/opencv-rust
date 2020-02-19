#include "common.hpp"
#include <opencv2/text.hpp>
#include "text_types.hpp"

extern "C" {
	Result_void cv_text_MSERsToERStats_const__InputArrayX_vector_vector_Point__X_vector_vector_ERStat__X(void* image, void* contours, void* regions) {
		try {
			cv::text::MSERsToERStats(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<std::vector<cv::Point>>*>(contours), *reinterpret_cast<std::vector<std::vector<cv::text::ERStat>>*>(regions));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_computeNMChannels_const__InputArrayX_const__OutputArrayX_int(void* _src, void* _channels, int _mode) {
		try {
			cv::text::computeNMChannels(*reinterpret_cast<const cv::_InputArray*>(_src), *reinterpret_cast<const cv::_OutputArray*>(_channels), _mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_createERFilterNM1_const_Ptr_Callback_X_int_float_float_float_bool_float(void* cb, int thresholdDelta, float minArea, float maxArea, float minProbability, bool nonMaxSuppression, float minProbabilityDiff) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM1(*reinterpret_cast<const cv::Ptr<cv::text::ERFilter::Callback>*>(cb), thresholdDelta, minArea, maxArea, minProbability, nonMaxSuppression, minProbabilityDiff);
			return Ok<void*>(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_createERFilterNM2_const_Ptr_Callback_X_float(void* cb, float minProbability) {
		try {
			cv::Ptr<cv::text::ERFilter> ret = cv::text::createERFilterNM2(*reinterpret_cast<const cv::Ptr<cv::text::ERFilter::Callback>*>(cb), minProbability);
			return Ok<void*>(new cv::Ptr<cv::text::ERFilter>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_createOCRHMMTransitionsTable_const_StringX_vector_String_X(const char* vocabulary, void* lexicon) {
		try {
			cv::Mat ret = cv::text::createOCRHMMTransitionsTable(cv::String(vocabulary), *reinterpret_cast<std::vector<cv::String>*>(lexicon));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_createOCRHMMTransitionsTable_stringX_vector_string_X_const__OutputArrayX(void** vocabulary, void* lexicon, void* transition_probabilities_table) {
		try {
			std::string vocabulary_out;
			cv::text::createOCRHMMTransitionsTable(vocabulary_out, *reinterpret_cast<std::vector<std::string>*>(lexicon), *reinterpret_cast<const cv::_OutputArray*>(transition_probabilities_table));
			*vocabulary = ocvrs_create_string(vocabulary_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_detectRegions_const__InputArrayX_const_Ptr_ERFilter_X_const_Ptr_ERFilter_X_vector_vector_Point__X(void* image, void* er_filter1, void* er_filter2, void* regions) {
		try {
			cv::text::detectRegions(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::Ptr<cv::text::ERFilter>*>(er_filter1), *reinterpret_cast<const cv::Ptr<cv::text::ERFilter>*>(er_filter2), *reinterpret_cast<std::vector<std::vector<cv::Point>>*>(regions));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_ERStat__X_vector_vector_Vec2i__X_vector_Rect_X_int_const_stringX_float(void* img, void* channels, void* regions, void* groups, void* groups_rects, int method, const char* filename, float minProbablity) {
		try {
			cv::text::erGrouping(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(channels), *reinterpret_cast<std::vector<std::vector<cv::text::ERStat>>*>(regions), *reinterpret_cast<std::vector<std::vector<cv::Vec2i>>*>(groups), *reinterpret_cast<std::vector<cv::Rect>*>(groups_rects), method, std::string(filename), minProbablity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_Point___vector_Rect_X_int_const_StringX_float(void* image, void* channel, void* regions, void* groups_rects, int method, const char* filename, float minProbablity) {
		try {
			cv::text::erGrouping(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(channel), *reinterpret_cast<std::vector<std::vector<cv::Point>>*>(regions), *reinterpret_cast<std::vector<cv::Rect>*>(groups_rects), method, cv::String(filename), minProbablity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_loadClassifierNM1_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM1(cv::String(filename));
			return Ok<void*>(new cv::Ptr<cv::text::ERFilter::Callback>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_loadClassifierNM2_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::ERFilter::Callback> ret = cv::text::loadClassifierNM2(cv::String(filename));
			return Ok<void*>(new cv::Ptr<cv::text::ERFilter::Callback>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_loadOCRBeamSearchClassifierCNN_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback> ret = cv::text::loadOCRBeamSearchClassifierCNN(cv::String(filename));
			return Ok<void*>(new cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_loadOCRHMMClassifierCNN_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierCNN(cv::String(filename));
			return Ok<void*>(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_loadOCRHMMClassifierNM_const_StringX(const char* filename) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback> ret = cv::text::loadOCRHMMClassifierNM(cv::String(filename));
			return Ok<void*>(new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_BaseOCR_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::BaseOCR*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_BaseOCR_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void* mask, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::BaseOCR*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), *reinterpret_cast<cv::Mat*>(mask), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_run_const__InputArrayX_vector_ERStat_X(void* instance, void* image, void* regions) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::text::ERStat>*>(regions));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setCallback_const_Ptr_Callback_X(void* instance, void* cb) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setCallback(*reinterpret_cast<const cv::Ptr<cv::text::ERFilter::Callback>*>(cb));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setThresholdDelta_int(void* instance, int thresholdDelta) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setThresholdDelta(thresholdDelta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinArea_float(void* instance, float minArea) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setMinArea(minArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMaxArea_float(void* instance, float maxArea) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setMaxArea(maxArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinProbability_float(void* instance, float minProbability) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setMinProbability(minProbability);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setMinProbabilityDiff_float(void* instance, float minProbabilityDiff) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setMinProbabilityDiff(minProbabilityDiff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_ERFilter_setNonMaxSuppression_bool(void* instance, bool nonMaxSuppression) {
		try {
			reinterpret_cast<cv::text::ERFilter*>(instance)->setNonMaxSuppression(nonMaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERFilter_getNumRejected(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERFilter*>(instance)->getNumRejected();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_text_ERFilter_Callback_eval_const_ERStatX(void* instance, void* stat) {
		try {
			double ret = reinterpret_cast<cv::text::ERFilter::Callback*>(instance)->eval(*reinterpret_cast<const cv::text::ERStat*>(stat));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_text_ERStat_pixel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERStat*>(instance)->pixel;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setPixel_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->pixel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_level_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERStat*>(instance)->level;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setLevel_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->level = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_area_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERStat*>(instance)->area;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setArea_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->area = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_perimeter_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERStat*>(instance)->perimeter;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setPerimeter_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->perimeter = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_ERStat_euler_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::ERStat*>(instance)->euler;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_text_ERStat_setEuler_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->euler = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_text_ERStat_rect_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::text::ERStat*>(instance)->rect;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_text_ERStat_setRect_Rect(void* instance, cv::Rect val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->rect = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double(*)[2]> cv_text_ERStat_raw_moments(void* instance) {
		try {
			double(*ret)[2] = &reinterpret_cast<cv::text::ERStat*>(instance)->raw_moments;
			return Ok<double(*)[2]>(ret);
		} OCVRS_CATCH(Result<double(*)[2]>)
	}
	
	Result<double(*)[3]> cv_text_ERStat_central_moments(void* instance) {
		try {
			double(*ret)[3] = &reinterpret_cast<cv::text::ERStat*>(instance)->central_moments;
			return Ok<double(*)[3]>(ret);
		} OCVRS_CATCH(Result<double(*)[3]>)
	}
	
	Result<float> cv_text_ERStat_med_crossings_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::text::ERStat*>(instance)->med_crossings;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setMed_crossings_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->med_crossings = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_hole_area_ratio_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::text::ERStat*>(instance)->hole_area_ratio;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setHole_area_ratio_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->hole_area_ratio = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_convex_hull_ratio_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::text::ERStat*>(instance)->convex_hull_ratio;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setConvex_hull_ratio_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->convex_hull_ratio = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_text_ERStat_num_inflexion_points_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::text::ERStat*>(instance)->num_inflexion_points;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_text_ERStat_setNum_inflexion_points_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->num_inflexion_points = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_pixels(void* instance) {
		try {
			std::vector<int>* ret = reinterpret_cast<cv::text::ERStat*>(instance)->pixels;
			return Ok<void*>(new std::vector<int>*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setPixels_vector_int_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->pixels = reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_text_ERStat_probability_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::text::ERStat*>(instance)->probability;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_text_ERStat_setProbability_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->probability = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_parent(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->parent;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setParent_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->parent = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_child(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->child;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setChild_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->child = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_next(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->next;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setNext_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->next = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_prev(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->prev;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setPrev_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->prev = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_text_ERStat_local_maxima_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::text::ERStat*>(instance)->local_maxima;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_text_ERStat_setLocal_maxima_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->local_maxima = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_max_probability_ancestor(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->max_probability_ancestor;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setMax_probability_ancestor_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->max_probability_ancestor = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_ERStat_min_probability_ancestor(void* instance) {
		try {
			cv::text::ERStat* ret = reinterpret_cast<cv::text::ERStat*>(instance)->min_probability_ancestor;
			return Ok<void*>(new cv::text::ERStat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_ERStat_setMin_probability_ancestor_ERStatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::text::ERStat*>(instance)->min_probability_ancestor = reinterpret_cast<cv::text::ERStat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ERStat_delete(cv::text::ERStat* instance) {
		delete instance;
	}
	Result<void*> cv_text_ERStat_ERStat_int_int_int_int(int level, int pixel, int x, int y) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat(level, pixel, x, y);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_OCRBeamSearchDecoder_delete(cv::text::OCRBeamSearchDecoder* instance) {
		delete instance;
	}
	Result_void cv_text_OCRBeamSearchDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRBeamSearchDecoder*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRBeamSearchDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void* mask, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRBeamSearchDecoder*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), *reinterpret_cast<cv::Mat*>(mask), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_int_int(void* instance, void* image, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRBeamSearchDecoder*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_const__InputArrayX_int_int(void* instance, void* image, void* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRBeamSearchDecoder*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_stringX_const__InputArrayX_const__InputArrayX_decoder_mode_int(void* classifier, const char* vocabulary, void* transition_probabilities_table, void* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*reinterpret_cast<cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>*>(classifier), std::string(vocabulary), *reinterpret_cast<const cv::_InputArray*>(transition_probabilities_table), *reinterpret_cast<const cv::_InputArray*>(emission_probabilities_table), mode, beam_size);
			return Ok<void*>(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int_int(void* classifier, const char* vocabulary, void* transition_probabilities_table, void* emission_probabilities_table, int mode, int beam_size) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*reinterpret_cast<cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>*>(classifier), cv::String(vocabulary), *reinterpret_cast<const cv::_InputArray*>(transition_probabilities_table), *reinterpret_cast<const cv::_InputArray*>(emission_probabilities_table), mode, beam_size);
			return Ok<void*>(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_OCRBeamSearchDecoder_ClassifierCallback_delete(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
		delete instance;
	}
	Result_void cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayX_vector_vector_double__X_vector_int_X(void* instance, void* image, void* recognition_probabilities, void* oversegmentation) {
		try {
			reinterpret_cast<cv::text::OCRBeamSearchDecoder::ClassifierCallback*>(instance)->eval(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<std::vector<double>>*>(recognition_probabilities), *reinterpret_cast<std::vector<int>*>(oversegmentation));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::OCRBeamSearchDecoder::ClassifierCallback*>(instance)->getWindowSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(void* instance) {
		try {
			int ret = reinterpret_cast<cv::text::OCRBeamSearchDecoder::ClassifierCallback*>(instance)->getStepSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_OCRHMMDecoder_delete(cv::text::OCRHMMDecoder* instance) {
		delete instance;
	}
	Result_void cv_text_OCRHMMDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRHMMDecoder*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRHMMDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void* mask, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRHMMDecoder*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), *reinterpret_cast<cv::Mat*>(mask), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_run_const__InputArrayX_int_int(void* instance, void* image, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRHMMDecoder*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_run_const__InputArrayX_const__InputArrayX_int_int(void* instance, void* image, void* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRHMMDecoder*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_create_Ptr_ClassifierCallback__const_stringX_const__InputArrayX_const__InputArrayX_decoder_mode(void* classifier, const char* vocabulary, void* transition_probabilities_table, void* emission_probabilities_table, cv::text::decoder_mode mode) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*reinterpret_cast<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>(classifier), std::string(vocabulary), *reinterpret_cast<const cv::_InputArray*>(transition_probabilities_table), *reinterpret_cast<const cv::_InputArray*>(emission_probabilities_table), mode);
			return Ok<void*>(new cv::Ptr<cv::text::OCRHMMDecoder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRHMMDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int(void* classifier, const char* vocabulary, void* transition_probabilities_table, void* emission_probabilities_table, int mode) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*reinterpret_cast<cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>*>(classifier), cv::String(vocabulary), *reinterpret_cast<const cv::_InputArray*>(transition_probabilities_table), *reinterpret_cast<const cv::_InputArray*>(emission_probabilities_table), mode);
			return Ok<void*>(new cv::Ptr<cv::text::OCRHMMDecoder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_OCRHMMDecoder_ClassifierCallback_delete(cv::text::OCRHMMDecoder::ClassifierCallback* instance) {
		delete instance;
	}
	Result_void cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayX_vector_int_X_vector_double_X(void* instance, void* image, void* out_class, void* out_confidence) {
		try {
			reinterpret_cast<cv::text::OCRHMMDecoder::ClassifierCallback*>(instance)->eval(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<int>*>(out_class), *reinterpret_cast<std::vector<double>*>(out_confidence));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRTesseract_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRTesseract*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_text_OCRTesseract_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(void* instance, void* image, void* mask, void** output_text, void* component_rects, void* component_texts, void* component_confidences, int component_level) {
		try {
			std::string output_text_out;
			reinterpret_cast<cv::text::OCRTesseract*>(instance)->run(*reinterpret_cast<cv::Mat*>(image), *reinterpret_cast<cv::Mat*>(mask), output_text_out, reinterpret_cast<std::vector<cv::Rect>*>(component_rects), reinterpret_cast<std::vector<std::string>*>(component_texts), reinterpret_cast<std::vector<float>*>(component_confidences), component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRTesseract_run_const__InputArrayX_int_int(void* instance, void* image, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRTesseract*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_text_OCRTesseract_run_const__InputArrayX_const__InputArrayX_int_int(void* instance, void* image, void* mask, int min_confidence, int component_level) {
		try {
			cv::String ret = reinterpret_cast<cv::text::OCRTesseract*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), min_confidence, component_level);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_text_OCRTesseract_setWhiteList_const_StringX(void* instance, const char* char_whitelist) {
		try {
			reinterpret_cast<cv::text::OCRTesseract*>(instance)->setWhiteList(cv::String(char_whitelist));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
			return Ok<void*>(new cv::Ptr<cv::text::OCRTesseract>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
