#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#include "tracking_types.hpp"

extern "C" {
	// create(const TrackerCSRT::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:81
	// ("cv::tracking::TrackerCSRT::create", vec![(pred!(mut, ["parameters"], ["const cv::tracking::TrackerCSRT::Params*"]), _)]),
	void cv_tracking_TrackerCSRT_create_const_ParamsR(const cv::tracking::TrackerCSRT::Params* parameters, Result<cv::Ptr<cv::tracking::TrackerCSRT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::tracking::TrackerCSRT> ret = cv::tracking::TrackerCSRT::create(*parameters);
			Ok(new cv::Ptr<cv::tracking::TrackerCSRT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerCSRT::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:81
	// ("cv::tracking::TrackerCSRT::create", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerCSRT_create(Result<cv::Ptr<cv::tracking::TrackerCSRT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::tracking::TrackerCSRT> ret = cv::tracking::TrackerCSRT::create();
			Ok(new cv::Ptr<cv::tracking::TrackerCSRT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:86
	// ("cv::tracking::TrackerCSRT::setInitialMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
	void cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(cv::tracking::TrackerCSRT* instance, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerCSRT::to_Tracker() generated
	// ("cv::tracking::TrackerCSRT::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_tracking_TrackerCSRT_to_Tracker(cv::tracking::TrackerCSRT* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::tracking::TrackerCSRT::delete() generated
	// ("cv::tracking::TrackerCSRT::delete", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerCSRT_delete(cv::tracking::TrackerCSRT* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:43
	// ("cv::tracking::TrackerCSRT::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerCSRT_Params_Params(Result<cv::tracking::TrackerCSRT::Params*>* ocvrs_return) {
		try {
			cv::tracking::TrackerCSRT::Params* ret = new cv::tracking::TrackerCSRT::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerCSRT::Params::implicitClone() generated
	// ("cv::tracking::TrackerCSRT::Params::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::tracking::TrackerCSRT::Params* cv_tracking_TrackerCSRT_Params_implicitClone_const(const cv::tracking::TrackerCSRT::Params* instance) {
			return new cv::tracking::TrackerCSRT::Params(*instance);
	}

	// cv::tracking::TrackerCSRT::Params::use_hog() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:45
	// ("cv::tracking::TrackerCSRT::Params::use_hog", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_hog_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_hog;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_hog(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:45
	// ("cv::tracking::TrackerCSRT::Params::setUse_hog", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_hog_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_hog = val;
	}

	// cv::tracking::TrackerCSRT::Params::use_color_names() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:46
	// ("cv::tracking::TrackerCSRT::Params::use_color_names", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_color_names_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_color_names;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_color_names(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:46
	// ("cv::tracking::TrackerCSRT::Params::setUse_color_names", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_color_names_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_color_names = val;
	}

	// cv::tracking::TrackerCSRT::Params::use_gray() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:47
	// ("cv::tracking::TrackerCSRT::Params::use_gray", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_gray_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_gray;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_gray(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:47
	// ("cv::tracking::TrackerCSRT::Params::setUse_gray", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_gray_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_gray = val;
	}

	// cv::tracking::TrackerCSRT::Params::use_rgb() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:48
	// ("cv::tracking::TrackerCSRT::Params::use_rgb", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_rgb_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_rgb;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_rgb(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:48
	// ("cv::tracking::TrackerCSRT::Params::setUse_rgb", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_rgb_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_rgb = val;
	}

	// cv::tracking::TrackerCSRT::Params::use_channel_weights() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:49
	// ("cv::tracking::TrackerCSRT::Params::use_channel_weights", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_channel_weights_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_channel_weights;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_channel_weights(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:49
	// ("cv::tracking::TrackerCSRT::Params::setUse_channel_weights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_channel_weights_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_channel_weights = val;
	}

	// cv::tracking::TrackerCSRT::Params::use_segmentation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:50
	// ("cv::tracking::TrackerCSRT::Params::use_segmentation", vec![(pred!(const, [], []), _)]),
	bool cv_tracking_TrackerCSRT_Params_propUse_segmentation_const(const cv::tracking::TrackerCSRT::Params* instance) {
			bool ret = instance->use_segmentation;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setUse_segmentation(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:50
	// ("cv::tracking::TrackerCSRT::Params::setUse_segmentation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propUse_segmentation_const_bool(cv::tracking::TrackerCSRT::Params* instance, const bool val) {
			instance->use_segmentation = val;
	}

	// cv::tracking::TrackerCSRT::Params::window_function() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:52
	// ("cv::tracking::TrackerCSRT::Params::window_function", vec![(pred!(const, [], []), _)]),
	void* cv_tracking_TrackerCSRT_Params_propWindow_function_const(const cv::tracking::TrackerCSRT::Params* instance) {
			std::string ret = instance->window_function;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::tracking::TrackerCSRT::Params::setWindow_function(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:52
	// ("cv::tracking::TrackerCSRT::Params::setWindow_function", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propWindow_function_const_string(cv::tracking::TrackerCSRT::Params* instance, const char* val) {
			instance->window_function = std::string(val);
	}

	// cv::tracking::TrackerCSRT::Params::kaiser_alpha() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:53
	// ("cv::tracking::TrackerCSRT::Params::kaiser_alpha", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propKaiser_alpha_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->kaiser_alpha;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setKaiser_alpha(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:53
	// ("cv::tracking::TrackerCSRT::Params::setKaiser_alpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propKaiser_alpha_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->kaiser_alpha = val;
	}

	// cv::tracking::TrackerCSRT::Params::cheb_attenuation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:54
	// ("cv::tracking::TrackerCSRT::Params::cheb_attenuation", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propCheb_attenuation_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->cheb_attenuation;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setCheb_attenuation(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:54
	// ("cv::tracking::TrackerCSRT::Params::setCheb_attenuation", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propCheb_attenuation_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->cheb_attenuation = val;
	}

	// cv::tracking::TrackerCSRT::Params::template_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:56
	// ("cv::tracking::TrackerCSRT::Params::template_size", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propTemplate_size_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->template_size;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setTemplate_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:56
	// ("cv::tracking::TrackerCSRT::Params::setTemplate_size", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propTemplate_size_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->template_size = val;
	}

	// cv::tracking::TrackerCSRT::Params::gsl_sigma() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:57
	// ("cv::tracking::TrackerCSRT::Params::gsl_sigma", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propGsl_sigma_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->gsl_sigma;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setGsl_sigma(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:57
	// ("cv::tracking::TrackerCSRT::Params::setGsl_sigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propGsl_sigma_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->gsl_sigma = val;
	}

	// cv::tracking::TrackerCSRT::Params::hog_orientations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:58
	// ("cv::tracking::TrackerCSRT::Params::hog_orientations", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propHog_orientations_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->hog_orientations;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setHog_orientations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:58
	// ("cv::tracking::TrackerCSRT::Params::setHog_orientations", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propHog_orientations_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->hog_orientations = val;
	}

	// cv::tracking::TrackerCSRT::Params::hog_clip() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:59
	// ("cv::tracking::TrackerCSRT::Params::hog_clip", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propHog_clip_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->hog_clip;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setHog_clip(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:59
	// ("cv::tracking::TrackerCSRT::Params::setHog_clip", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propHog_clip_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->hog_clip = val;
	}

	// cv::tracking::TrackerCSRT::Params::padding() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:60
	// ("cv::tracking::TrackerCSRT::Params::padding", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propPadding_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->padding;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:60
	// ("cv::tracking::TrackerCSRT::Params::setPadding", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propPadding_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->padding = val;
	}

	// cv::tracking::TrackerCSRT::Params::filter_lr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:61
	// ("cv::tracking::TrackerCSRT::Params::filter_lr", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propFilter_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->filter_lr;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setFilter_lr(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:61
	// ("cv::tracking::TrackerCSRT::Params::setFilter_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propFilter_lr_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->filter_lr = val;
	}

	// cv::tracking::TrackerCSRT::Params::weights_lr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:62
	// ("cv::tracking::TrackerCSRT::Params::weights_lr", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propWeights_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->weights_lr;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setWeights_lr(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:62
	// ("cv::tracking::TrackerCSRT::Params::setWeights_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propWeights_lr_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->weights_lr = val;
	}

	// cv::tracking::TrackerCSRT::Params::num_hog_channels_used() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:63
	// ("cv::tracking::TrackerCSRT::Params::num_hog_channels_used", vec![(pred!(const, [], []), _)]),
	int cv_tracking_TrackerCSRT_Params_propNum_hog_channels_used_const(const cv::tracking::TrackerCSRT::Params* instance) {
			int ret = instance->num_hog_channels_used;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setNum_hog_channels_used(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:63
	// ("cv::tracking::TrackerCSRT::Params::setNum_hog_channels_used", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propNum_hog_channels_used_const_int(cv::tracking::TrackerCSRT::Params* instance, const int val) {
			instance->num_hog_channels_used = val;
	}

	// cv::tracking::TrackerCSRT::Params::admm_iterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:64
	// ("cv::tracking::TrackerCSRT::Params::admm_iterations", vec![(pred!(const, [], []), _)]),
	int cv_tracking_TrackerCSRT_Params_propAdmm_iterations_const(const cv::tracking::TrackerCSRT::Params* instance) {
			int ret = instance->admm_iterations;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setAdmm_iterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:64
	// ("cv::tracking::TrackerCSRT::Params::setAdmm_iterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propAdmm_iterations_const_int(cv::tracking::TrackerCSRT::Params* instance, const int val) {
			instance->admm_iterations = val;
	}

	// cv::tracking::TrackerCSRT::Params::histogram_bins() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:65
	// ("cv::tracking::TrackerCSRT::Params::histogram_bins", vec![(pred!(const, [], []), _)]),
	int cv_tracking_TrackerCSRT_Params_propHistogram_bins_const(const cv::tracking::TrackerCSRT::Params* instance) {
			int ret = instance->histogram_bins;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setHistogram_bins(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:65
	// ("cv::tracking::TrackerCSRT::Params::setHistogram_bins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propHistogram_bins_const_int(cv::tracking::TrackerCSRT::Params* instance, const int val) {
			instance->histogram_bins = val;
	}

	// cv::tracking::TrackerCSRT::Params::histogram_lr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:66
	// ("cv::tracking::TrackerCSRT::Params::histogram_lr", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propHistogram_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->histogram_lr;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setHistogram_lr(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:66
	// ("cv::tracking::TrackerCSRT::Params::setHistogram_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propHistogram_lr_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->histogram_lr = val;
	}

	// cv::tracking::TrackerCSRT::Params::background_ratio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:67
	// ("cv::tracking::TrackerCSRT::Params::background_ratio", vec![(pred!(const, [], []), _)]),
	int cv_tracking_TrackerCSRT_Params_propBackground_ratio_const(const cv::tracking::TrackerCSRT::Params* instance) {
			int ret = instance->background_ratio;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setBackground_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:67
	// ("cv::tracking::TrackerCSRT::Params::setBackground_ratio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propBackground_ratio_const_int(cv::tracking::TrackerCSRT::Params* instance, const int val) {
			instance->background_ratio = val;
	}

	// cv::tracking::TrackerCSRT::Params::number_of_scales() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:68
	// ("cv::tracking::TrackerCSRT::Params::number_of_scales", vec![(pred!(const, [], []), _)]),
	int cv_tracking_TrackerCSRT_Params_propNumber_of_scales_const(const cv::tracking::TrackerCSRT::Params* instance) {
			int ret = instance->number_of_scales;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setNumber_of_scales(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:68
	// ("cv::tracking::TrackerCSRT::Params::setNumber_of_scales", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propNumber_of_scales_const_int(cv::tracking::TrackerCSRT::Params* instance, const int val) {
			instance->number_of_scales = val;
	}

	// cv::tracking::TrackerCSRT::Params::scale_sigma_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:69
	// ("cv::tracking::TrackerCSRT::Params::scale_sigma_factor", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propScale_sigma_factor_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->scale_sigma_factor;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setScale_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:69
	// ("cv::tracking::TrackerCSRT::Params::setScale_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propScale_sigma_factor_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->scale_sigma_factor = val;
	}

	// cv::tracking::TrackerCSRT::Params::scale_model_max_area() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:70
	// ("cv::tracking::TrackerCSRT::Params::scale_model_max_area", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propScale_model_max_area_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->scale_model_max_area;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setScale_model_max_area(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:70
	// ("cv::tracking::TrackerCSRT::Params::setScale_model_max_area", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propScale_model_max_area_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->scale_model_max_area = val;
	}

	// cv::tracking::TrackerCSRT::Params::scale_lr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:71
	// ("cv::tracking::TrackerCSRT::Params::scale_lr", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propScale_lr_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->scale_lr;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setScale_lr(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:71
	// ("cv::tracking::TrackerCSRT::Params::setScale_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propScale_lr_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->scale_lr = val;
	}

	// cv::tracking::TrackerCSRT::Params::scale_step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:72
	// ("cv::tracking::TrackerCSRT::Params::scale_step", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propScale_step_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->scale_step;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setScale_step(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:72
	// ("cv::tracking::TrackerCSRT::Params::setScale_step", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propScale_step_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->scale_step = val;
	}

	// cv::tracking::TrackerCSRT::Params::psr_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:74
	// ("cv::tracking::TrackerCSRT::Params::psr_threshold", vec![(pred!(const, [], []), _)]),
	float cv_tracking_TrackerCSRT_Params_propPsr_threshold_const(const cv::tracking::TrackerCSRT::Params* instance) {
			float ret = instance->psr_threshold;
			return ret;
	}

	// cv::tracking::TrackerCSRT::Params::setPsr_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:74
	// ("cv::tracking::TrackerCSRT::Params::setPsr_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_tracking_TrackerCSRT_Params_propPsr_threshold_const_float(cv::tracking::TrackerCSRT::Params* instance, const float val) {
			instance->psr_threshold = val;
	}

	// cv::tracking::TrackerCSRT::Params::delete() generated
	// ("cv::tracking::TrackerCSRT::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerCSRT_Params_delete(cv::tracking::TrackerCSRT::Params* instance) {
			delete instance;
	}

	// create(const TrackerKCF::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:141
	// ("cv::tracking::TrackerKCF::create", vec![(pred!(mut, ["parameters"], ["const cv::tracking::TrackerKCF::Params*"]), _)]),
	void cv_tracking_TrackerKCF_create_const_ParamsR(const cv::tracking::TrackerKCF::Params* parameters, Result<cv::Ptr<cv::tracking::TrackerKCF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::tracking::TrackerKCF> ret = cv::tracking::TrackerKCF::create(*parameters);
			Ok(new cv::Ptr<cv::tracking::TrackerKCF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerKCF::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:141
	// ("cv::tracking::TrackerKCF::create", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerKCF_create(Result<cv::Ptr<cv::tracking::TrackerKCF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::tracking::TrackerKCF> ret = cv::tracking::TrackerKCF::create();
			Ok(new cv::Ptr<cv::tracking::TrackerKCF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFeatureExtractor(FeatureExtractorCallbackFN, bool)(Function, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:148
	// ("cv::tracking::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["callback", "pca_func"], ["cv::tracking::TrackerKCF::FeatureExtractorCallbackFN", "bool"]), _)]),
	void cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(cv::tracking::TrackerKCF* instance, cv::tracking::TrackerKCF::FeatureExtractorCallbackFN callback, bool pca_func, ResultVoid* ocvrs_return) {
		try {
			instance->setFeatureExtractor(callback, pca_func);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerKCF::setFeatureExtractor(Function) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:148
	// ("cv::tracking::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["callback"], ["cv::tracking::TrackerKCF::FeatureExtractorCallbackFN"]), _)]),
	void cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN(cv::tracking::TrackerKCF* instance, cv::tracking::TrackerKCF::FeatureExtractorCallbackFN callback, ResultVoid* ocvrs_return) {
		try {
			instance->setFeatureExtractor(callback);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tracking::TrackerKCF::to_Tracker() generated
	// ("cv::tracking::TrackerKCF::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_tracking_TrackerKCF_to_Tracker(cv::tracking::TrackerKCF* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::tracking::TrackerKCF::delete() generated
	// ("cv::tracking::TrackerKCF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerKCF_delete(cv::tracking::TrackerKCF* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/tracking.hpp:119
	// ("cv::tracking::TrackerKCF::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_tracking_TrackerKCF_Params_Params(Result<cv::tracking::TrackerKCF::Params>* ocvrs_return) {
		try {
			cv::tracking::TrackerKCF::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
