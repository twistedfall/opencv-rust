#include "common.hpp"
#include <opencv2/photo.hpp>
#include "photo_types.hpp"

extern "C" {
	Result_void cv_colorChange_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float_float(void* src, void* mask, void* dst, float red_mul, float green_mul, float blue_mul) {
		try {
			cv::colorChange(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(dst), red_mul, green_mul, blue_mul);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_createAlignMTB_int_int_bool(int max_bits, int exclude_range, bool cut) {
		try {
			cv::Ptr<cv::AlignMTB> ret = cv::createAlignMTB(max_bits, exclude_range, cut);
			return Ok<void*>(new cv::Ptr<cv::AlignMTB>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createCalibrateDebevec_int_float_bool(int samples, float lambda, bool random) {
		try {
			cv::Ptr<cv::CalibrateDebevec> ret = cv::createCalibrateDebevec(samples, lambda, random);
			return Ok<void*>(new cv::Ptr<cv::CalibrateDebevec>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createCalibrateRobertson_int_float(int max_iter, float threshold) {
		try {
			cv::Ptr<cv::CalibrateRobertson> ret = cv::createCalibrateRobertson(max_iter, threshold);
			return Ok<void*>(new cv::Ptr<cv::CalibrateRobertson>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createMergeDebevec() {
		try {
			cv::Ptr<cv::MergeDebevec> ret = cv::createMergeDebevec();
			return Ok<void*>(new cv::Ptr<cv::MergeDebevec>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createMergeMertens_float_float_float(float contrast_weight, float saturation_weight, float exposure_weight) {
		try {
			cv::Ptr<cv::MergeMertens> ret = cv::createMergeMertens(contrast_weight, saturation_weight, exposure_weight);
			return Ok<void*>(new cv::Ptr<cv::MergeMertens>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createMergeRobertson() {
		try {
			cv::Ptr<cv::MergeRobertson> ret = cv::createMergeRobertson();
			return Ok<void*>(new cv::Ptr<cv::MergeRobertson>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createTonemapDrago_float_float_float(float gamma, float saturation, float bias) {
		try {
			cv::Ptr<cv::TonemapDrago> ret = cv::createTonemapDrago(gamma, saturation, bias);
			return Ok<void*>(new cv::Ptr<cv::TonemapDrago>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createTonemapDurand_float_float_float_float_float(float gamma, float contrast, float saturation, float sigma_space, float sigma_color) {
		try {
			cv::Ptr<cv::TonemapDurand> ret = cv::createTonemapDurand(gamma, contrast, saturation, sigma_space, sigma_color);
			return Ok<void*>(new cv::Ptr<cv::TonemapDurand>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createTonemapMantiuk_float_float_float(float gamma, float scale, float saturation) {
		try {
			cv::Ptr<cv::TonemapMantiuk> ret = cv::createTonemapMantiuk(gamma, scale, saturation);
			return Ok<void*>(new cv::Ptr<cv::TonemapMantiuk>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createTonemapReinhard_float_float_float_float(float gamma, float intensity, float light_adapt, float color_adapt) {
		try {
			cv::Ptr<cv::TonemapReinhard> ret = cv::createTonemapReinhard(gamma, intensity, light_adapt, color_adapt);
			return Ok<void*>(new cv::Ptr<cv::TonemapReinhard>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createTonemap_float(float gamma) {
		try {
			cv::Ptr<cv::Tonemap> ret = cv::createTonemap(gamma);
			return Ok<void*>(new cv::Ptr<cv::Tonemap>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_decolor_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* src, void* grayscale, void* color_boost) {
		try {
			cv::decolor(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(grayscale), *reinterpret_cast<const cv::_OutputArray*>(color_boost));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_denoise_TVL1_const_vector_Mat_X_MatX_double_int(void* observations, void* result, double lambda, int niters) {
		try {
			cv::denoise_TVL1(*reinterpret_cast<const std::vector<cv::Mat>*>(observations), *reinterpret_cast<cv::Mat*>(result), lambda, niters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detailEnhance_const__InputArrayX_const__OutputArrayX_float_float(void* src, void* dst, float sigma_s, float sigma_r) {
		try {
			cv::detailEnhance(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_edgePreservingFilter_const__InputArrayX_const__OutputArrayX_int_float_float(void* src, void* dst, int flags, float sigma_s, float sigma_r) {
		try {
			cv::edgePreservingFilter(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags, sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingColoredMulti_const__InputArrayX_const__OutputArrayX_int_int_float_float_int_int(void* srcImgs, void* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, float hColor, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingColoredMulti(*reinterpret_cast<const cv::_InputArray*>(srcImgs), *reinterpret_cast<const cv::_OutputArray*>(dst), imgToDenoiseIndex, temporalWindowSize, h, hColor, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingColored_const__InputArrayX_const__OutputArrayX_float_float_int_int(void* src, void* dst, float h, float hColor, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingColored(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), h, hColor, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingMulti_const__InputArrayX_const__OutputArrayX_int_int_const_vector_float_X_int_int_int(void* srcImgs, void* dst, int imgToDenoiseIndex, int temporalWindowSize, void* h, int templateWindowSize, int searchWindowSize, int normType) {
		try {
			cv::fastNlMeansDenoisingMulti(*reinterpret_cast<const cv::_InputArray*>(srcImgs), *reinterpret_cast<const cv::_OutputArray*>(dst), imgToDenoiseIndex, temporalWindowSize, *reinterpret_cast<const std::vector<float>*>(h), templateWindowSize, searchWindowSize, normType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoisingMulti_const__InputArrayX_const__OutputArrayX_int_int_float_int_int(void* srcImgs, void* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoisingMulti(*reinterpret_cast<const cv::_InputArray*>(srcImgs), *reinterpret_cast<const cv::_OutputArray*>(dst), imgToDenoiseIndex, temporalWindowSize, h, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoising_const__InputArrayX_const__OutputArrayX_const_vector_float_X_int_int_int(void* src, void* dst, void* h, int templateWindowSize, int searchWindowSize, int normType) {
		try {
			cv::fastNlMeansDenoising(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const std::vector<float>*>(h), templateWindowSize, searchWindowSize, normType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fastNlMeansDenoising_const__InputArrayX_const__OutputArrayX_float_int_int(void* src, void* dst, float h, int templateWindowSize, int searchWindowSize) {
		try {
			cv::fastNlMeansDenoising(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), h, templateWindowSize, searchWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_illuminationChange_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float(void* src, void* mask, void* dst, float alpha, float beta) {
		try {
			cv::illuminationChange(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(dst), alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_inpaint_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_int(void* src, void* inpaintMask, void* dst, double inpaintRadius, int flags) {
		try {
			cv::inpaint(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(inpaintMask), *reinterpret_cast<const cv::_OutputArray*>(dst), inpaintRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pencilSketch_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_float_float_float(void* src, void* dst1, void* dst2, float sigma_s, float sigma_r, float shade_factor) {
		try {
			cv::pencilSketch(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst1), *reinterpret_cast<const cv::_OutputArray*>(dst2), sigma_s, sigma_r, shade_factor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_seamlessClone_const__InputArrayX_const__InputArrayX_const__InputArrayX_Point_const__OutputArrayX_int(void* src, void* dst, void* mask, cv::Point p, void* blend, int flags) {
		try {
			cv::seamlessClone(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask), p, *reinterpret_cast<const cv::_OutputArray*>(blend), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_stylization_const__InputArrayX_const__OutputArrayX_float_float(void* src, void* dst, float sigma_s, float sigma_r) {
		try {
			cv::stylization(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), sigma_s, sigma_r);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_textureFlattening_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_float_int(void* src, void* mask, void* dst, float low_threshold, float high_threshold, int kernel_size) {
		try {
			cv::textureFlattening(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(dst), low_threshold, high_threshold, kernel_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignExposures_process_const__InputArrayX_vector_Mat_X_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::AlignExposures*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<std::vector<cv::Mat>*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_process_const__InputArrayX_vector_Mat_X_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<std::vector<cv::Mat>*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_process_const__InputArrayX_vector_Mat_X(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<std::vector<cv::Mat>*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point> cv_AlignMTB_calculateShift_const__InputArrayX_const__InputArrayX(void* instance, void* img0, void* img1) {
		try {
			cv::Point ret = reinterpret_cast<cv::AlignMTB*>(instance)->calculateShift(*reinterpret_cast<const cv::_InputArray*>(img0), *reinterpret_cast<const cv::_InputArray*>(img1));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_AlignMTB_shiftMat_const__InputArrayX_const__OutputArrayX_Point(void* instance, void* src, void* dst, cv::Point shift) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->shiftMat(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AlignMTB_computeBitmaps_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* img, void* tb, void* eb) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->computeBitmaps(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_OutputArray*>(tb), *reinterpret_cast<const cv::_OutputArray*>(eb));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AlignMTB_getMaxBits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AlignMTB*>(instance)->getMaxBits();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AlignMTB_setMaxBits_int(void* instance, int max_bits) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->setMaxBits(max_bits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AlignMTB_getExcludeRange_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AlignMTB*>(instance)->getExcludeRange();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AlignMTB_setExcludeRange_int(void* instance, int exclude_range) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->setExcludeRange(exclude_range);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AlignMTB_getCut_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::AlignMTB*>(instance)->getCut();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_AlignMTB_setCut_bool(void* instance, bool value) {
		try {
			reinterpret_cast<cv::AlignMTB*>(instance)->setCut(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CalibrateCRF_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times) {
		try {
			reinterpret_cast<cv::CalibrateCRF*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_CalibrateDebevec_getLambda_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::CalibrateDebevec*>(instance)->getLambda();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CalibrateDebevec_setLambda_float(void* instance, float lambda) {
		try {
			reinterpret_cast<cv::CalibrateDebevec*>(instance)->setLambda(lambda);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_CalibrateDebevec_getSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::CalibrateDebevec*>(instance)->getSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_CalibrateDebevec_setSamples_int(void* instance, int samples) {
		try {
			reinterpret_cast<cv::CalibrateDebevec*>(instance)->setSamples(samples);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_CalibrateDebevec_getRandom_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::CalibrateDebevec*>(instance)->getRandom();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CalibrateDebevec_setRandom_bool(void* instance, bool random) {
		try {
			reinterpret_cast<cv::CalibrateDebevec*>(instance)->setRandom(random);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_CalibrateRobertson_getMaxIter_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::CalibrateRobertson*>(instance)->getMaxIter();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_CalibrateRobertson_setMaxIter_int(void* instance, int max_iter) {
		try {
			reinterpret_cast<cv::CalibrateRobertson*>(instance)->setMaxIter(max_iter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_CalibrateRobertson_getThreshold_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::CalibrateRobertson*>(instance)->getThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CalibrateRobertson_setThreshold_float(void* instance, float threshold) {
		try {
			reinterpret_cast<cv::CalibrateRobertson*>(instance)->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_CalibrateRobertson_getRadiance_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::CalibrateRobertson*>(instance)->getRadiance();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MergeDebevec_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::MergeDebevec*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeDebevec_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times) {
		try {
			reinterpret_cast<cv::MergeDebevec*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeExposures_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::MergeExposures*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeMertens_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::MergeMertens*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeMertens_process_const__InputArrayX_const__OutputArrayX(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::MergeMertens*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getContrastWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::MergeMertens*>(instance)->getContrastWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setContrastWeight_float(void* instance, float contrast_weiht) {
		try {
			reinterpret_cast<cv::MergeMertens*>(instance)->setContrastWeight(contrast_weiht);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getSaturationWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::MergeMertens*>(instance)->getSaturationWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setSaturationWeight_float(void* instance, float saturation_weight) {
		try {
			reinterpret_cast<cv::MergeMertens*>(instance)->setSaturationWeight(saturation_weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_MergeMertens_getExposureWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::MergeMertens*>(instance)->getExposureWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_MergeMertens_setExposureWeight_float(void* instance, float exposure_weight) {
		try {
			reinterpret_cast<cv::MergeMertens*>(instance)->setExposureWeight(exposure_weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeRobertson_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times, void* response) {
		try {
			reinterpret_cast<cv::MergeRobertson*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times), *reinterpret_cast<const cv::_InputArray*>(response));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MergeRobertson_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* src, void* dst, void* times) {
		try {
			reinterpret_cast<cv::MergeRobertson*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(times));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Tonemap_process_const__InputArrayX_const__OutputArrayX(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::Tonemap*>(instance)->process(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_Tonemap_getGamma_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::Tonemap*>(instance)->getGamma();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_Tonemap_setGamma_float(void* instance, float gamma) {
		try {
			reinterpret_cast<cv::Tonemap*>(instance)->setGamma(gamma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDrago_getSaturation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDrago*>(instance)->getSaturation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDrago_setSaturation_float(void* instance, float saturation) {
		try {
			reinterpret_cast<cv::TonemapDrago*>(instance)->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDrago_getBias_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDrago*>(instance)->getBias();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDrago_setBias_float(void* instance, float bias) {
		try {
			reinterpret_cast<cv::TonemapDrago*>(instance)->setBias(bias);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDurand_getSaturation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDurand*>(instance)->getSaturation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDurand_setSaturation_float(void* instance, float saturation) {
		try {
			reinterpret_cast<cv::TonemapDurand*>(instance)->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDurand_getContrast_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDurand*>(instance)->getContrast();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDurand_setContrast_float(void* instance, float contrast) {
		try {
			reinterpret_cast<cv::TonemapDurand*>(instance)->setContrast(contrast);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDurand_getSigmaSpace_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDurand*>(instance)->getSigmaSpace();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDurand_setSigmaSpace_float(void* instance, float sigma_space) {
		try {
			reinterpret_cast<cv::TonemapDurand*>(instance)->setSigmaSpace(sigma_space);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapDurand_getSigmaColor_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapDurand*>(instance)->getSigmaColor();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapDurand_setSigmaColor_float(void* instance, float sigma_color) {
		try {
			reinterpret_cast<cv::TonemapDurand*>(instance)->setSigmaColor(sigma_color);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapMantiuk_getScale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapMantiuk*>(instance)->getScale();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapMantiuk_setScale_float(void* instance, float scale) {
		try {
			reinterpret_cast<cv::TonemapMantiuk*>(instance)->setScale(scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapMantiuk_getSaturation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapMantiuk*>(instance)->getSaturation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapMantiuk_setSaturation_float(void* instance, float saturation) {
		try {
			reinterpret_cast<cv::TonemapMantiuk*>(instance)->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getIntensity_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapReinhard*>(instance)->getIntensity();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setIntensity_float(void* instance, float intensity) {
		try {
			reinterpret_cast<cv::TonemapReinhard*>(instance)->setIntensity(intensity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getLightAdaptation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapReinhard*>(instance)->getLightAdaptation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setLightAdaptation_float(void* instance, float light_adapt) {
		try {
			reinterpret_cast<cv::TonemapReinhard*>(instance)->setLightAdaptation(light_adapt);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_TonemapReinhard_getColorAdaptation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::TonemapReinhard*>(instance)->getColorAdaptation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_TonemapReinhard_setColorAdaptation_float(void* instance, float color_adapt) {
		try {
			reinterpret_cast<cv::TonemapReinhard*>(instance)->setColorAdaptation(color_adapt);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
