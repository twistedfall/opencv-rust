#include "common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	Result<void*> cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			return Ok<void*>(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret));
		} OCVRS_CATCH(Result<void*>)
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
	
}
