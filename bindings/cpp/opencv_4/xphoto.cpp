#include "ocvrs_common.hpp"
#include <opencv2/xphoto.hpp>
#include "xphoto_types.hpp"

extern "C" {
	Result_void cv_xphoto_applyChannelGains_const__InputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float gainB, float gainG, float gainR) {
		try {
			cv::xphoto::applyChannelGains(*src, *dst, gainB, gainG, gainR);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(const cv::_InputArray* src, const cv::_InputOutputArray* dstStep1, const cv::_OutputArray* dstStep2, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dstStep1, *dstStep2, h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dst, h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xphoto::GrayworldWB>*> cv_xphoto_createGrayworldWB() {
		try {
			cv::Ptr<cv::xphoto::GrayworldWB> ret = cv::xphoto::createGrayworldWB();
			return Ok(new cv::Ptr<cv::xphoto::GrayworldWB>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xphoto::GrayworldWB>*>))
	}
	
	Result<cv::Ptr<cv::xphoto::LearningBasedWB>*> cv_xphoto_createLearningBasedWB_const_StringR(const char* path_to_model) {
		try {
			cv::Ptr<cv::xphoto::LearningBasedWB> ret = cv::xphoto::createLearningBasedWB(std::string(path_to_model));
			return Ok(new cv::Ptr<cv::xphoto::LearningBasedWB>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xphoto::LearningBasedWB>*>))
	}
	
	Result<cv::Ptr<cv::xphoto::SimpleWB>*> cv_xphoto_createSimpleWB() {
		try {
			cv::Ptr<cv::xphoto::SimpleWB> ret = cv::xphoto::createSimpleWB();
			return Ok(new cv::Ptr<cv::xphoto::SimpleWB>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xphoto::SimpleWB>*>))
	}
	
	Result<cv::Ptr<cv::xphoto::TonemapDurand>*> cv_xphoto_createTonemapDurand_float_float_float_float_float(float gamma, float contrast, float saturation, float sigma_color, float sigma_space) {
		try {
			cv::Ptr<cv::xphoto::TonemapDurand> ret = cv::xphoto::createTonemapDurand(gamma, contrast, saturation, sigma_color, sigma_space);
			return Ok(new cv::Ptr<cv::xphoto::TonemapDurand>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xphoto::TonemapDurand>*>))
	}
	
	Result_void cv_xphoto_dctDenoising_const_MatR_MatR_const_double_const_int(const cv::Mat* src, cv::Mat* dst, const double sigma, const int psize) {
		try {
			cv::xphoto::dctDenoising(*src, *dst, sigma, psize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_inpaint_const_MatR_const_MatR_MatR_const_int(const cv::Mat* src, const cv::Mat* mask, cv::Mat* dst, const int algorithmType) {
		try {
			cv::xphoto::inpaint(*src, *mask, *dst, algorithmType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int size, int dynRatio) {
		try {
			cv::xphoto::oilPainting(*src, *dst, size, dynRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int size, int dynRatio, int code) {
		try {
			cv::xphoto::oilPainting(*src, *dst, size, dynRatio, code);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_GrayworldWB_getSaturationThreshold_const(const cv::xphoto::GrayworldWB* instance) {
		try {
			float ret = instance->getSaturationThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_GrayworldWB_setSaturationThreshold_float(cv::xphoto::GrayworldWB* instance, float val) {
		try {
			instance->setSaturationThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(cv::xphoto::LearningBasedWB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->extractSimpleFeatures(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xphoto_LearningBasedWB_getRangeMaxVal_const(const cv::xphoto::LearningBasedWB* instance) {
		try {
			int ret = instance->getRangeMaxVal();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xphoto_LearningBasedWB_setRangeMaxVal_int(cv::xphoto::LearningBasedWB* instance, int val) {
		try {
			instance->setRangeMaxVal(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_LearningBasedWB_getSaturationThreshold_const(const cv::xphoto::LearningBasedWB* instance) {
		try {
			float ret = instance->getSaturationThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_LearningBasedWB_setSaturationThreshold_float(cv::xphoto::LearningBasedWB* instance, float val) {
		try {
			instance->setSaturationThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xphoto_LearningBasedWB_getHistBinNum_const(const cv::xphoto::LearningBasedWB* instance) {
		try {
			int ret = instance->getHistBinNum();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xphoto_LearningBasedWB_setHistBinNum_int(cv::xphoto::LearningBasedWB* instance, int val) {
		try {
			instance->setHistBinNum(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_SimpleWB_getInputMin_const(const cv::xphoto::SimpleWB* instance) {
		try {
			float ret = instance->getInputMin();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_SimpleWB_setInputMin_float(cv::xphoto::SimpleWB* instance, float val) {
		try {
			instance->setInputMin(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_SimpleWB_getInputMax_const(const cv::xphoto::SimpleWB* instance) {
		try {
			float ret = instance->getInputMax();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_SimpleWB_setInputMax_float(cv::xphoto::SimpleWB* instance, float val) {
		try {
			instance->setInputMax(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_SimpleWB_getOutputMin_const(const cv::xphoto::SimpleWB* instance) {
		try {
			float ret = instance->getOutputMin();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_SimpleWB_setOutputMin_float(cv::xphoto::SimpleWB* instance, float val) {
		try {
			instance->setOutputMin(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_SimpleWB_getOutputMax_const(const cv::xphoto::SimpleWB* instance) {
		try {
			float ret = instance->getOutputMax();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_SimpleWB_setOutputMax_float(cv::xphoto::SimpleWB* instance, float val) {
		try {
			instance->setOutputMax(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_SimpleWB_getP_const(const cv::xphoto::SimpleWB* instance) {
		try {
			float ret = instance->getP();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_SimpleWB_setP_float(cv::xphoto::SimpleWB* instance, float val) {
		try {
			instance->setP(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSaturation_const(const cv::xphoto::TonemapDurand* instance) {
		try {
			float ret = instance->getSaturation();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_TonemapDurand_setSaturation_float(cv::xphoto::TonemapDurand* instance, float saturation) {
		try {
			instance->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_TonemapDurand_getContrast_const(const cv::xphoto::TonemapDurand* instance) {
		try {
			float ret = instance->getContrast();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_TonemapDurand_setContrast_float(cv::xphoto::TonemapDurand* instance, float contrast) {
		try {
			instance->setContrast(contrast);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSigmaSpace_const(const cv::xphoto::TonemapDurand* instance) {
		try {
			float ret = instance->getSigmaSpace();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_TonemapDurand_setSigmaSpace_float(cv::xphoto::TonemapDurand* instance, float sigma_space) {
		try {
			instance->setSigmaSpace(sigma_space);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSigmaColor_const(const cv::xphoto::TonemapDurand* instance) {
		try {
			float ret = instance->getSigmaColor();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xphoto_TonemapDurand_setSigmaColor_float(cv::xphoto::TonemapDurand* instance, float sigma_color) {
		try {
			instance->setSigmaColor(sigma_color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(cv::xphoto::WhiteBalancer* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->balanceWhite(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
