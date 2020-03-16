#include "common.hpp"
#include <opencv2/photo.hpp>
#include "photo_types.hpp"

extern "C" {
	Result_void cv_colorChange_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float_float(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float red_mul, float green_mul, float blue_mul) {
		try {
			cv::colorChange(*src, *mask, *dst, red_mul, green_mul, blue_mul);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::AlignMTB>*> cv_createAlignMTB_int_int_bool(int max_bits, int exclude_range, bool cut) {
		try {
			cv::Ptr<cv::AlignMTB> ret = cv::createAlignMTB(max_bits, exclude_range, cut);
			return Ok(new cv::Ptr<cv::AlignMTB>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::AlignMTB>*>)
	}
	
	Result<cv::Ptr<cv::CalibrateDebevec>*> cv_createCalibrateDebevec_int_float_bool(int samples, float lambda, bool random) {
		try {
			cv::Ptr<cv::CalibrateDebevec> ret = cv::createCalibrateDebevec(samples, lambda, random);
			return Ok(new cv::Ptr<cv::CalibrateDebevec>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::CalibrateDebevec>*>)
	}
	
	Result<cv::Ptr<cv::CalibrateRobertson>*> cv_createCalibrateRobertson_int_float(int max_iter, float threshold) {
		try {
			cv::Ptr<cv::CalibrateRobertson> ret = cv::createCalibrateRobertson(max_iter, threshold);
			return Ok(new cv::Ptr<cv::CalibrateRobertson>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::CalibrateRobertson>*>)
	}
	
	Result<cv::Ptr<cv::MergeDebevec>*> cv_createMergeDebevec() {
		try {
			cv::Ptr<cv::MergeDebevec> ret = cv::createMergeDebevec();
			return Ok(new cv::Ptr<cv::MergeDebevec>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::MergeDebevec>*>)
	}
	
	Result<cv::Ptr<cv::MergeMertens>*> cv_createMergeMertens_float_float_float(float contrast_weight, float saturation_weight, float exposure_weight) {
		try {
			cv::Ptr<cv::MergeMertens> ret = cv::createMergeMertens(contrast_weight, saturation_weight, exposure_weight);
			return Ok(new cv::Ptr<cv::MergeMertens>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::MergeMertens>*>)
	}
	
	Result<cv::Ptr<cv::MergeRobertson>*> cv_createMergeRobertson() {
		try {
			cv::Ptr<cv::MergeRobertson> ret = cv::createMergeRobertson();
			return Ok(new cv::Ptr<cv::MergeRobertson>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::MergeRobertson>*>)
	}
	
	Result<cv::Ptr<cv::TonemapDrago>*> cv_createTonemapDrago_float_float_float(float gamma, float saturation, float bias) {
		try {
			cv::Ptr<cv::TonemapDrago> ret = cv::createTonemapDrago(gamma, saturation, bias);
			return Ok(new cv::Ptr<cv::TonemapDrago>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::TonemapDrago>*>)
	}
	
	Result<cv::Ptr<cv::TonemapMantiuk>*> cv_createTonemapMantiuk_float_float_float(float gamma, float scale, float saturation) {
		try {
			cv::Ptr<cv::TonemapMantiuk> ret = cv::createTonemapMantiuk(gamma, scale, saturation);
			return Ok(new cv::Ptr<cv::TonemapMantiuk>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::TonemapMantiuk>*>)
	}
	
	Result<cv::Ptr<cv::TonemapReinhard>*> cv_createTonemapReinhard_float_float_float_float(float gamma, float intensity, float light_adapt, float color_adapt) {
		try {
			cv::Ptr<cv::TonemapReinhard> ret = cv::createTonemapReinhard(gamma, intensity, light_adapt, color_adapt);
			return Ok(new cv::Ptr<cv::TonemapReinhard>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::TonemapReinhard>*>)
	}
	
	Result<cv::Ptr<cv::Tonemap>*> cv_createTonemap_float(float gamma) {
		try {
			cv::Ptr<cv::Tonemap> ret = cv::createTonemap(gamma);
			return Ok(new cv::Ptr<cv::Tonemap>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Tonemap>*>)
	}
	
	Result_void cv_decolor_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::_InputArray* src, const cv::_OutputArray* grayscale, const cv::_OutputArray* color_boost) {
		try {
			cv::decolor(*src, *grayscale, *color_boost);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_denoise_TVL1_const_vector_Mat_X_MatX_double_int(const std::vector<cv::Mat>* observations, cv::Mat* result, double lambda, int niters) {
		try {
			cv::denoise_TVL1(*observations, *result, lambda, niters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detailEnhance_const__InputArrayX_const__OutputArrayX_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float sigma_s, float sigma_r) {
		try {
			cv::detailEnhance(*src, *dst, sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_edgePreservingFilter_const__InputArrayX_const__OutputArrayX_int_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, float sigma_s, float sigma_r) {
		try {
			cv::edgePreservingFilter(*src, *dst, flags, sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingColoredMulti_const__InputArrayX_const__OutputArrayX_int_int_float_float_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, float hColor, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingColoredMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, h, hColor, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingColored_const__InputArrayX_const__OutputArrayX_float_float_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, float hColor, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingColored(*src, *dst, h, hColor, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingMulti_const__InputArrayX_const__OutputArrayX_int_int_const_vector_float_X_int_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, const std::vector<float>* h, int templateWindowSize, int searchWindowSize, int normType) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, *h, templateWindowSize, searchWindowSize, normType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingMulti_const__InputArrayX_const__OutputArrayX_int_int_float_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, h, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoising_const__InputArrayX_const__OutputArrayX_const_vector_float_X_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const std::vector<float>* h, int templateWindowSize, int searchWindowSize, int normType) {
		try {
			cv::fastNlMeansDenoising(*src, *dst, *h, templateWindowSize, searchWindowSize, normType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoising_const__InputArrayX_const__OutputArrayX_float_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoising(*src, *dst, h, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_illuminationChange_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float alpha, float beta) {
		try {
			cv::illuminationChange(*src, *mask, *dst, alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_inpaint_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_int(const cv::_InputArray* src, const cv::_InputArray* inpaintMask, const cv::_OutputArray* dst, double inpaintRadius, int flags) {
		try {
			cv::inpaint(*src, *inpaintMask, *dst, inpaintRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pencilSketch_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_float_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst1, const cv::_OutputArray* dst2, float sigma_s, float sigma_r, float shade_factor) {
		try {
			cv::pencilSketch(*src, *dst1, *dst2, sigma_s, sigma_r, shade_factor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_seamlessClone_const__InputArrayX_const__InputArrayX_const__InputArrayX_Point_const__OutputArrayX_int(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_InputArray* mask, const cv::Point* p, const cv::_OutputArray* blend, int flags) {
		try {
			cv::seamlessClone(*src, *dst, *mask, *p, *blend, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_stylization_const__InputArrayX_const__OutputArrayX_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float sigma_s, float sigma_r) {
		try {
			cv::stylization(*src, *dst, sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_textureFlattening_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float_int(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float low_threshold, float high_threshold, int kernel_size) {
		try {
			cv::textureFlattening(*src, *mask, *dst, low_threshold, high_threshold, kernel_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignExposures_process_const__InputArrayX_vector_Mat_X_const__InputArrayX_const__InputArrayX(cv::AlignExposures* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_process_const__InputArrayX_vector_Mat_X_const__InputArrayX_const__InputArrayX(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_process_const__InputArrayX_vector_Mat_X(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst) {
		try {
			instance->process(*src, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point> cv_AlignMTB_calculateShift_const__InputArrayX_const__InputArrayX(cv::AlignMTB* instance, const cv::_InputArray* img0, const cv::_InputArray* img1) {
		try {
			cv::Point ret = instance->calculateShift(*img0, *img1);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_AlignMTB_shiftMat_const__InputArrayX_const__OutputArrayX_Point(cv::AlignMTB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Point* shift) {
		try {
			instance->shiftMat(*src, *dst, *shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_computeBitmaps_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::AlignMTB* instance, const cv::_InputArray* img, const cv::_OutputArray* tb, const cv::_OutputArray* eb) {
		try {
			instance->computeBitmaps(*img, *tb, *eb);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AlignMTB_getMaxBits_const(const cv::AlignMTB* instance) {
		try {
			int ret = instance->getMaxBits();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AlignMTB_setMaxBits_int(cv::AlignMTB* instance, int max_bits) {
		try {
			instance->setMaxBits(max_bits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AlignMTB_getExcludeRange_const(const cv::AlignMTB* instance) {
		try {
			int ret = instance->getExcludeRange();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AlignMTB_setExcludeRange_int(cv::AlignMTB* instance, int exclude_range) {
		try {
			instance->setExcludeRange(exclude_range);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AlignMTB_getCut_const(const cv::AlignMTB* instance) {
		try {
			bool ret = instance->getCut();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_AlignMTB_setCut_bool(cv::AlignMTB* instance, bool value) {
		try {
			instance->setCut(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CalibrateCRF_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::CalibrateCRF* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times) {
		try {
			instance->process(*src, *dst, *times);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_CalibrateDebevec_getLambda_const(const cv::CalibrateDebevec* instance) {
		try {
			float ret = instance->getLambda();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CalibrateDebevec_setLambda_float(cv::CalibrateDebevec* instance, float lambda) {
		try {
			instance->setLambda(lambda);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_CalibrateDebevec_getSamples_const(const cv::CalibrateDebevec* instance) {
		try {
			int ret = instance->getSamples();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_CalibrateDebevec_setSamples_int(cv::CalibrateDebevec* instance, int samples) {
		try {
			instance->setSamples(samples);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_CalibrateDebevec_getRandom_const(const cv::CalibrateDebevec* instance) {
		try {
			bool ret = instance->getRandom();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CalibrateDebevec_setRandom_bool(cv::CalibrateDebevec* instance, bool random) {
		try {
			instance->setRandom(random);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_CalibrateRobertson_getMaxIter_const(const cv::CalibrateRobertson* instance) {
		try {
			int ret = instance->getMaxIter();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_CalibrateRobertson_setMaxIter_int(cv::CalibrateRobertson* instance, int max_iter) {
		try {
			instance->setMaxIter(max_iter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_CalibrateRobertson_getThreshold_const(const cv::CalibrateRobertson* instance) {
		try {
			float ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CalibrateRobertson_setThreshold_float(cv::CalibrateRobertson* instance, float threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_CalibrateRobertson_getRadiance_const(const cv::CalibrateRobertson* instance) {
		try {
			cv::Mat ret = instance->getRadiance();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_MergeDebevec_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeDebevec_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times) {
		try {
			instance->process(*src, *dst, *times);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeExposures_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(cv::MergeExposures* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeMertens_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeMertens_process_const__InputArrayX_const__OutputArrayX(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->process(*src, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getContrastWeight_const(const cv::MergeMertens* instance) {
		try {
			float ret = instance->getContrastWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setContrastWeight_float(cv::MergeMertens* instance, float contrast_weiht) {
		try {
			instance->setContrastWeight(contrast_weiht);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getSaturationWeight_const(const cv::MergeMertens* instance) {
		try {
			float ret = instance->getSaturationWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setSaturationWeight_float(cv::MergeMertens* instance, float saturation_weight) {
		try {
			instance->setSaturationWeight(saturation_weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getExposureWeight_const(const cv::MergeMertens* instance) {
		try {
			float ret = instance->getExposureWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setExposureWeight_float(cv::MergeMertens* instance, float exposure_weight) {
		try {
			instance->setExposureWeight(exposure_weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeRobertson_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response) {
		try {
			instance->process(*src, *dst, *times, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeRobertson_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times) {
		try {
			instance->process(*src, *dst, *times);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Tonemap_process_const__InputArrayX_const__OutputArrayX(cv::Tonemap* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->process(*src, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_Tonemap_getGamma_const(const cv::Tonemap* instance) {
		try {
			float ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_Tonemap_setGamma_float(cv::Tonemap* instance, float gamma) {
		try {
			instance->setGamma(gamma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDrago_getSaturation_const(const cv::TonemapDrago* instance) {
		try {
			float ret = instance->getSaturation();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDrago_setSaturation_float(cv::TonemapDrago* instance, float saturation) {
		try {
			instance->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDrago_getBias_const(const cv::TonemapDrago* instance) {
		try {
			float ret = instance->getBias();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDrago_setBias_float(cv::TonemapDrago* instance, float bias) {
		try {
			instance->setBias(bias);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapMantiuk_getScale_const(const cv::TonemapMantiuk* instance) {
		try {
			float ret = instance->getScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapMantiuk_setScale_float(cv::TonemapMantiuk* instance, float scale) {
		try {
			instance->setScale(scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapMantiuk_getSaturation_const(const cv::TonemapMantiuk* instance) {
		try {
			float ret = instance->getSaturation();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapMantiuk_setSaturation_float(cv::TonemapMantiuk* instance, float saturation) {
		try {
			instance->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getIntensity_const(const cv::TonemapReinhard* instance) {
		try {
			float ret = instance->getIntensity();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setIntensity_float(cv::TonemapReinhard* instance, float intensity) {
		try {
			instance->setIntensity(intensity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getLightAdaptation_const(const cv::TonemapReinhard* instance) {
		try {
			float ret = instance->getLightAdaptation();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setLightAdaptation_float(cv::TonemapReinhard* instance, float light_adapt) {
		try {
			instance->setLightAdaptation(light_adapt);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getColorAdaptation_const(const cv::TonemapReinhard* instance) {
		try {
			float ret = instance->getColorAdaptation();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setColorAdaptation_float(cv::TonemapReinhard* instance, float color_adapt) {
		try {
			instance->setColorAdaptation(color_adapt);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
