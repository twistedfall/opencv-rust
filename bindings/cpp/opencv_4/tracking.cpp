#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#include "tracking_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::tracking::TrackerCSRT>*> cv_tracking_TrackerCSRT_create_const_ParamsR(const cv::tracking::TrackerCSRT::Params* parameters) {
		try {
			cv::Ptr<cv::tracking::TrackerCSRT> ret = cv::tracking::TrackerCSRT::create(*parameters);
			return Ok(new cv::Ptr<cv::tracking::TrackerCSRT>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::tracking::TrackerCSRT>*>))
	}
	
	Result_void cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(cv::tracking::TrackerCSRT* instance, const cv::_InputArray* mask) {
		try {
			instance->setInitialMask(*mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_hog_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_hog;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_hog_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_hog = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_color_names_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_color_names;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_color_names_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_color_names = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_gray_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_gray;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_gray_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_gray = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_rgb_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_rgb;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_rgb_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_rgb = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_channel_weights_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_channel_weights;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_channel_weights_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_channel_weights = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_tracking_TrackerCSRT_Params_getPropUse_segmentation_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_segmentation;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropUse_segmentation_bool(cv::tracking::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_segmentation = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_tracking_TrackerCSRT_Params_getPropWindow_function_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			std::string ret = instance->window_function;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropWindow_function_string(cv::tracking::TrackerCSRT::Params* instance, char* val) {
		try {
			instance->window_function = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropKaiser_alpha_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->kaiser_alpha;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropKaiser_alpha_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->kaiser_alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropCheb_attenuation_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->cheb_attenuation;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropCheb_attenuation_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->cheb_attenuation = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropTemplate_size_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->template_size;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropTemplate_size_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->template_size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropGsl_sigma_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->gsl_sigma;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropGsl_sigma_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->gsl_sigma = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropHog_orientations_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->hog_orientations;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropHog_orientations_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->hog_orientations = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropHog_clip_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->hog_clip;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropHog_clip_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->hog_clip = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropPadding_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->padding;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropPadding_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->padding = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropFilter_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->filter_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropFilter_lr_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->filter_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropWeights_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->weights_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropWeights_lr_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->weights_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_tracking_TrackerCSRT_Params_getPropNum_hog_channels_used_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->num_hog_channels_used;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropNum_hog_channels_used_int(cv::tracking::TrackerCSRT::Params* instance, int val) {
		try {
			instance->num_hog_channels_used = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_tracking_TrackerCSRT_Params_getPropAdmm_iterations_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->admm_iterations;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropAdmm_iterations_int(cv::tracking::TrackerCSRT::Params* instance, int val) {
		try {
			instance->admm_iterations = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_tracking_TrackerCSRT_Params_getPropHistogram_bins_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->histogram_bins;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropHistogram_bins_int(cv::tracking::TrackerCSRT::Params* instance, int val) {
		try {
			instance->histogram_bins = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropHistogram_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->histogram_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropHistogram_lr_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->histogram_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_tracking_TrackerCSRT_Params_getPropBackground_ratio_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->background_ratio;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropBackground_ratio_int(cv::tracking::TrackerCSRT::Params* instance, int val) {
		try {
			instance->background_ratio = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_tracking_TrackerCSRT_Params_getPropNumber_of_scales_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->number_of_scales;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropNumber_of_scales_int(cv::tracking::TrackerCSRT::Params* instance, int val) {
		try {
			instance->number_of_scales = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropScale_sigma_factor_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_sigma_factor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropScale_sigma_factor_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_sigma_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropScale_model_max_area_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_model_max_area;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropScale_model_max_area_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_model_max_area = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropScale_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropScale_lr_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropScale_step_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_step;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropScale_step_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_step = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_tracking_TrackerCSRT_Params_getPropPsr_threshold_const(const cv::tracking::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->psr_threshold;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_tracking_TrackerCSRT_Params_setPropPsr_threshold_float(cv::tracking::TrackerCSRT::Params* instance, float val) {
		try {
			instance->psr_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerCSRT_Params_delete(cv::tracking::TrackerCSRT::Params* instance) {
		delete instance;
	}
	Result<cv::tracking::TrackerCSRT::Params*> cv_tracking_TrackerCSRT_Params_Params() {
		try {
			cv::tracking::TrackerCSRT::Params* ret = new cv::tracking::TrackerCSRT::Params();
			return Ok<cv::tracking::TrackerCSRT::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::tracking::TrackerCSRT::Params*>))
	}
	
	Result<cv::Ptr<cv::tracking::TrackerKCF>*> cv_tracking_TrackerKCF_create_const_ParamsR(const cv::tracking::TrackerKCF::Params* parameters) {
		try {
			cv::Ptr<cv::tracking::TrackerKCF> ret = cv::tracking::TrackerKCF::create(*parameters);
			return Ok(new cv::Ptr<cv::tracking::TrackerKCF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::tracking::TrackerKCF>*>))
	}
	
	Result_void cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(cv::tracking::TrackerKCF* instance, cv::tracking::TrackerKCF::FeatureExtractorCallbackFN callback, bool pca_func) {
		try {
			instance->setFeatureExtractor(callback, pca_func);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::tracking::TrackerKCF::Params> cv_tracking_TrackerKCF_Params_Params() {
		try {
			cv::tracking::TrackerKCF::Params ret;
			return Ok<cv::tracking::TrackerKCF::Params>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::tracking::TrackerKCF::Params>))
	}
	
}
