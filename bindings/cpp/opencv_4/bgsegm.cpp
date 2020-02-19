#include "common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	Result<void*> cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(int minPixelStability, bool useHistory, int maxPixelStability, bool isParallel) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT> ret = cv::bgsegm::createBackgroundSubtractorCNT(minPixelStability, useHistory, maxPixelStability, isParallel);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(int mc, int nSamples, float replaceRate, float propagationRate, int hitsThreshold, float alpha, float beta, float blinkingSupressionDecay, float blinkingSupressionMultiplier, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC> ret = cv::bgsegm::createBackgroundSubtractorGSOC(mc, nSamples, replaceRate, propagationRate, hitsThreshold, alpha, beta, blinkingSupressionDecay, blinkingSupressionMultiplier, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(int mc, int nSamples, int LSBPRadius, float Tlower, float Tupper, float Tinc, float Tdec, float Rscale, float Rincdec, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, int LSBPthreshold, int minCount) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP> ret = cv::bgsegm::createBackgroundSubtractorLSBP(mc, nSamples, LSBPRadius, Tlower, Tupper, Tinc, Tdec, Rscale, Rincdec, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG, LSBPthreshold, minCount);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayX_const__InputArrayX_double_double_double_double(void* background, void* object, double amplitude, double wavelength, double wavespeed, double objspeed) {
		try {
			cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator> ret = cv::bgsegm::createSyntheticSequenceGenerator(*reinterpret_cast<const cv::_InputArray*>(background), *reinterpret_cast<const cv::_InputArray*>(object), amplitude, wavelength, wavespeed, objspeed);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayX_const__OutputArrayX_double(void* instance, void* image, void* fgmask, double learningRate) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(fgmask), learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayX(void* instance, void* backgroundImage) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->getBackgroundImage(*reinterpret_cast<const cv::_OutputArray*>(backgroundImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->getMinPixelStability();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->setMinPixelStability(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->getMaxPixelStability();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->setMaxPixelStability(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->getUseHistory();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(void* instance, bool value) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->setUseHistory(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->getIsParallel();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(void* instance, bool value) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorCNT*>(instance)->setIsParallel(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getMaxFeatures();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(void* instance, int maxFeatures) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getDefaultLearningRate();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(void* instance, double lr) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setDefaultLearningRate(lr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getNumFrames();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(void* instance, int nframes) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setNumFrames(nframes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getQuantizationLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(void* instance, int nlevels) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setQuantizationLevels(nlevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getBackgroundPrior();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(void* instance, double bgprior) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setBackgroundPrior(bgprior);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getSmoothingRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(void* instance, int radius) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setSmoothingRadius(radius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getDecisionThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(void* instance, double thresh) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setDecisionThreshold(thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getUpdateBackgroundModel();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(void* instance, bool update) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setUpdateBackgroundModel(update);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getMinVal();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setMinVal(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->getMaxVal();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGMG*>(instance)->setMaxVal(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayX_const__OutputArrayX_double(void* instance, void* image, void* fgmask, double learningRate) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGSOC*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(fgmask), learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayX(void* instance, void* backgroundImage) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorGSOC*>(instance)->getBackgroundImage(*reinterpret_cast<const cv::_OutputArray*>(backgroundImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayX_const__OutputArrayX_double(void* instance, void* image, void* fgmask, double learningRate) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorLSBP*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(fgmask), learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayX(void* instance, void* backgroundImage) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorLSBP*>(instance)->getBackgroundImage(*reinterpret_cast<const cv::_OutputArray*>(backgroundImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BackgroundSubtractorLSBPDesc_delete(cv::bgsegm::BackgroundSubtractorLSBPDesc* instance) {
		delete instance;
	}
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayX_const_MatX(void* localSVDValues, void* frame) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues(*reinterpret_cast<const cv::_OutputArray*>(localSVDValues), *reinterpret_cast<const cv::Mat*>(frame));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayX_const_MatX_const_Point2iX(void* desc, void* localSVDValues, const cv::Point2i* LSBPSamplePoints) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues(*reinterpret_cast<const cv::_OutputArray*>(desc), *reinterpret_cast<const cv::Mat*>(localSVDValues), LSBPSamplePoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayX_const_MatX_const_Point2iX(void* desc, void* frame, const cv::Point2i* LSBPSamplePoints) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::compute(*reinterpret_cast<const cv::_OutputArray*>(desc), *reinterpret_cast<const cv::Mat*>(frame), LSBPSamplePoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->getHistory();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(void* instance, int nframes) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->setHistory(nframes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->getNMixtures();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(void* instance, int nmix) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->setNMixtures(nmix);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->getBackgroundRatio();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(void* instance, double backgroundRatio) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->setBackgroundRatio(backgroundRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->getNoiseSigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(void* instance, double noiseSigma) {
		try {
			reinterpret_cast<cv::bgsegm::BackgroundSubtractorMOG*>(instance)->setNoiseSigma(noiseSigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SyntheticSequenceGenerator_delete(cv::bgsegm::SyntheticSequenceGenerator* instance) {
		delete instance;
	}
	Result<void*> cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayX_const__InputArrayX_double_double_double_double(void* background, void* object, double amplitude, double wavelength, double wavespeed, double objspeed) {
		try {
			cv::bgsegm::SyntheticSequenceGenerator* ret = new cv::bgsegm::SyntheticSequenceGenerator(*reinterpret_cast<const cv::_InputArray*>(background), *reinterpret_cast<const cv::_InputArray*>(object), amplitude, wavelength, wavespeed, objspeed);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayX_const__OutputArrayX(void* instance, void* frame, void* gtMask) {
		try {
			reinterpret_cast<cv::bgsegm::SyntheticSequenceGenerator*>(instance)->getNextFrame(*reinterpret_cast<const cv::_OutputArray*>(frame), *reinterpret_cast<const cv::_OutputArray*>(gtMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
