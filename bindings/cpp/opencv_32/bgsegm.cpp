#include "common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*> cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>)
	}
	
	Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*> cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			return Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>)
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
	
}
