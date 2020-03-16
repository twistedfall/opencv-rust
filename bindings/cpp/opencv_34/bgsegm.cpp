#include "common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*> cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(int minPixelStability, bool useHistory, int maxPixelStability, bool isParallel) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT> ret = cv::bgsegm::createBackgroundSubtractorCNT(minPixelStability, useHistory, maxPixelStability, isParallel);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*> cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*> cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(int mc, int nSamples, float replaceRate, float propagationRate, int hitsThreshold, float alpha, float beta, float blinkingSupressionDecay, float blinkingSupressionMultiplier, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC> ret = cv::bgsegm::createBackgroundSubtractorGSOC(mc, nSamples, replaceRate, propagationRate, hitsThreshold, alpha, beta, blinkingSupressionDecay, blinkingSupressionMultiplier, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*> cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(int mc, int nSamples, int LSBPRadius, float Tlower, float Tupper, float Tinc, float Tdec, float Rscale, float Rincdec, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, int LSBPthreshold, int minCount) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP> ret = cv::bgsegm::createBackgroundSubtractorLSBP(mc, nSamples, LSBPRadius, Tlower, Tupper, Tinc, Tdec, Rscale, Rincdec, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG, LSBPthreshold, minCount);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*> cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*> cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayX_const__InputArrayX_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed) {
		try {
			cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator> ret = cv::bgsegm::createSyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			return Ok(new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayX_const__OutputArrayX_double(cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayX(const cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_OutputArray* backgroundImage) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance) {
		try {
			int ret = instance->getMinPixelStability();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value) {
		try {
			instance->setMinPixelStability(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance) {
		try {
			int ret = instance->getMaxPixelStability();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value) {
		try {
			instance->setMaxPixelStability(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(const cv::bgsegm::BackgroundSubtractorCNT* instance) {
		try {
			bool ret = instance->getUseHistory();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value) {
		try {
			instance->setUseHistory(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(const cv::bgsegm::BackgroundSubtractorCNT* instance) {
		try {
			bool ret = instance->getIsParallel();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value) {
		try {
			instance->setIsParallel(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			int ret = instance->getMaxFeatures();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int maxFeatures) {
		try {
			instance->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			double ret = instance->getDefaultLearningRate();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double lr) {
		try {
			instance->setDefaultLearningRate(lr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			int ret = instance->getNumFrames();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nframes) {
		try {
			instance->setNumFrames(nframes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			int ret = instance->getQuantizationLevels();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nlevels) {
		try {
			instance->setQuantizationLevels(nlevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			double ret = instance->getBackgroundPrior();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double bgprior) {
		try {
			instance->setBackgroundPrior(bgprior);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			int ret = instance->getSmoothingRadius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int radius) {
		try {
			instance->setSmoothingRadius(radius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			double ret = instance->getDecisionThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double thresh) {
		try {
			instance->setDecisionThreshold(thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			bool ret = instance->getUpdateBackgroundModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(cv::bgsegm::BackgroundSubtractorGMG* instance, bool update) {
		try {
			instance->setUpdateBackgroundModel(update);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			double ret = instance->getMinVal();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val) {
		try {
			instance->setMinVal(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance) {
		try {
			double ret = instance->getMaxVal();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val) {
		try {
			instance->setMaxVal(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayX_const__OutputArrayX_double(cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayX(const cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_OutputArray* backgroundImage) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayX_const__OutputArrayX_double(cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayX(const cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_OutputArray* backgroundImage) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BackgroundSubtractorLSBPDesc_delete(cv::bgsegm::BackgroundSubtractorLSBPDesc* instance) {
		delete instance;
	}
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayX_const_MatX(const cv::_OutputArray* localSVDValues, const cv::Mat* frame) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues(*localSVDValues, *frame);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayX_const_MatX_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* localSVDValues, const cv::Point2i* LSBPSamplePoints) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues(*desc, *localSVDValues, LSBPSamplePoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayX_const_MatX_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* frame, const cv::Point2i* LSBPSamplePoints) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::compute(*desc, *frame, LSBPSamplePoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(const cv::bgsegm::BackgroundSubtractorMOG* instance) {
		try {
			int ret = instance->getHistory();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nframes) {
		try {
			instance->setHistory(nframes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(const cv::bgsegm::BackgroundSubtractorMOG* instance) {
		try {
			int ret = instance->getNMixtures();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nmix) {
		try {
			instance->setNMixtures(nmix);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(const cv::bgsegm::BackgroundSubtractorMOG* instance) {
		try {
			double ret = instance->getBackgroundRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double backgroundRatio) {
		try {
			instance->setBackgroundRatio(backgroundRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(const cv::bgsegm::BackgroundSubtractorMOG* instance) {
		try {
			double ret = instance->getNoiseSigma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double noiseSigma) {
		try {
			instance->setNoiseSigma(noiseSigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SyntheticSequenceGenerator_delete(cv::bgsegm::SyntheticSequenceGenerator* instance) {
		delete instance;
	}
	Result<cv::bgsegm::SyntheticSequenceGenerator*> cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayX_const__InputArrayX_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed) {
		try {
			cv::bgsegm::SyntheticSequenceGenerator* ret = new cv::bgsegm::SyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bgsegm::SyntheticSequenceGenerator*>)
	}
	
	Result_void cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayX_const__OutputArrayX(cv::bgsegm::SyntheticSequenceGenerator* instance, const cv::_OutputArray* frame, const cv::_OutputArray* gtMask) {
		try {
			instance->getNextFrame(*frame, *gtMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
