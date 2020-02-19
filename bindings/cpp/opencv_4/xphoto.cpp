#include "common.hpp"
#include <opencv2/xphoto.hpp>
#include "xphoto_types.hpp"

extern "C" {
	Result_void cv_xphoto_applyChannelGains_const__InputArrayX_const__OutputArrayX_float_float_float(void* src, void* dst, float gainB, float gainG, float gainR) {
		try {
			cv::xphoto::applyChannelGains(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), gainB, gainG, gainR);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_bm3dDenoising_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_float_int_int_int_int_int_int_float_int_int_int(void* src, void* dstStep1, void* dstStep2, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType) {
		try {
			cv::xphoto::bm3dDenoising(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dstStep1), *reinterpret_cast<const cv::_OutputArray*>(dstStep2), h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_bm3dDenoising_const__InputArrayX_const__OutputArrayX_float_int_int_int_int_int_int_float_int_int_int(void* src, void* dst, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType) {
		try {
			cv::xphoto::bm3dDenoising(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xphoto_createGrayworldWB() {
		try {
			cv::Ptr<cv::xphoto::GrayworldWB> ret = cv::xphoto::createGrayworldWB();
			return Ok<void*>(new cv::Ptr<cv::xphoto::GrayworldWB>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xphoto_createLearningBasedWB_const_StringX(const char* path_to_model) {
		try {
			cv::Ptr<cv::xphoto::LearningBasedWB> ret = cv::xphoto::createLearningBasedWB(std::string(path_to_model));
			return Ok<void*>(new cv::Ptr<cv::xphoto::LearningBasedWB>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xphoto_createSimpleWB() {
		try {
			cv::Ptr<cv::xphoto::SimpleWB> ret = cv::xphoto::createSimpleWB();
			return Ok<void*>(new cv::Ptr<cv::xphoto::SimpleWB>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xphoto_createTonemapDurand_float_float_float_float_float(float gamma, float contrast, float saturation, float sigma_space, float sigma_color) {
		try {
			cv::Ptr<cv::xphoto::TonemapDurand> ret = cv::xphoto::createTonemapDurand(gamma, contrast, saturation, sigma_space, sigma_color);
			return Ok<void*>(new cv::Ptr<cv::xphoto::TonemapDurand>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xphoto_dctDenoising_const_MatX_MatX_double_int(void* src, void* dst, double sigma, int psize) {
		try {
			cv::xphoto::dctDenoising(*reinterpret_cast<const cv::Mat*>(src), *reinterpret_cast<cv::Mat*>(dst), sigma, psize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_inpaint_const_MatX_const_MatX_MatX_int(void* src, void* mask, void* dst, int algorithmType) {
		try {
			cv::xphoto::inpaint(*reinterpret_cast<const cv::Mat*>(src), *reinterpret_cast<const cv::Mat*>(mask), *reinterpret_cast<cv::Mat*>(dst), algorithmType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_oilPainting_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int size, int dynRatio) {
		try {
			cv::xphoto::oilPainting(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), size, dynRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_oilPainting_const__InputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, int size, int dynRatio, int code) {
		try {
			cv::xphoto::oilPainting(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), size, dynRatio, code);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_GrayworldWB_getSaturationThreshold_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::GrayworldWB*>(instance)->getSaturationThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_GrayworldWB_setSaturationThreshold_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::GrayworldWB*>(instance)->setSaturationThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayX_const__OutputArrayX(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->extractSimpleFeatures(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xphoto_LearningBasedWB_getRangeMaxVal_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->getRangeMaxVal();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xphoto_LearningBasedWB_setRangeMaxVal_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->setRangeMaxVal(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_LearningBasedWB_getSaturationThreshold_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->getSaturationThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_LearningBasedWB_setSaturationThreshold_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->setSaturationThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xphoto_LearningBasedWB_getHistBinNum_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->getHistBinNum();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xphoto_LearningBasedWB_setHistBinNum_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::xphoto::LearningBasedWB*>(instance)->setHistBinNum(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_SimpleWB_getInputMin_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->getInputMin();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_SimpleWB_setInputMin_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->setInputMin(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_SimpleWB_getInputMax_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->getInputMax();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_SimpleWB_setInputMax_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->setInputMax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_SimpleWB_getOutputMin_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->getOutputMin();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_SimpleWB_setOutputMin_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->setOutputMin(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_SimpleWB_getOutputMax_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->getOutputMax();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_SimpleWB_setOutputMax_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->setOutputMax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_SimpleWB_getP_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->getP();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_SimpleWB_setP_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xphoto::SimpleWB*>(instance)->setP(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSaturation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->getSaturation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_TonemapDurand_setSaturation_float(void* instance, float saturation) {
		try {
			reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->setSaturation(saturation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_TonemapDurand_getContrast_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->getContrast();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_TonemapDurand_setContrast_float(void* instance, float contrast) {
		try {
			reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->setContrast(contrast);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSigmaSpace_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->getSigmaSpace();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_TonemapDurand_setSigmaSpace_float(void* instance, float sigma_space) {
		try {
			reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->setSigmaSpace(sigma_space);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xphoto_TonemapDurand_getSigmaColor_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->getSigmaColor();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xphoto_TonemapDurand_setSigmaColor_float(void* instance, float sigma_color) {
		try {
			reinterpret_cast<cv::xphoto::TonemapDurand*>(instance)->setSigmaColor(sigma_color);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayX_const__OutputArrayX(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::xphoto::WhiteBalancer*>(instance)->balanceWhite(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
