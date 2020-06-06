#include "common.hpp"
#include <opencv2/video.hpp>
#include "video_types.hpp"

extern "C" {
	Result<cv::RotatedRect*> cv_CamShift_const__InputArrayX_RectX_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, const cv::TermCriteria* criteria) {
		try {
			cv::RotatedRect ret = cv::CamShift(*probImage, *window, *criteria);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result<int> cv_buildOpticalFlowPyramid_const__InputArrayX_const__OutputArrayX_Size_int_bool_int_int_bool(const cv::_InputArray* img, const cv::_OutputArray* pyramid, const cv::Size* winSize, int maxLevel, bool withDerivatives, int pyrBorder, int derivBorder, bool tryReuseInputImage) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel, withDerivatives, pyrBorder, derivBorder, tryReuseInputImage);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_calcOpticalFlowFarneback_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_double_int_int_int_int_double_int(const cv::_InputArray* prev, const cv::_InputArray* next, const cv::_InputOutputArray* flow, double pyr_scale, int levels, int winsize, int iterations, int poly_n, double poly_sigma, int flags) {
		try {
			cv::calcOpticalFlowFarneback(*prev, *next, *flow, pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_calcOpticalFlowPyrLK_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_Size_int_TermCriteria_int_double(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, const cv::Size* winSize, int maxLevel, const cv::TermCriteria* criteria, int flags, double minEigThreshold) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *winSize, maxLevel, *criteria, flags, minEigThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::BackgroundSubtractorKNN>*> cv_createBackgroundSubtractorKNN_int_double_bool(int history, double dist2Threshold, bool detectShadows) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
			return Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>))
	}
	
	Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*> cv_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			return Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>))
	}
	
	Result<cv::Ptr<cv::DualTVL1OpticalFlow>*> cv_createOptFlow_DualTVL1() {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::createOptFlow_DualTVL1();
			return Ok(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>))
	}
	
	Result<cv::Mat*> cv_estimateRigidTransform_const__InputArrayX_const__InputArrayX_bool(const cv::_InputArray* src, const cv::_InputArray* dst, bool fullAffine) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*src, *dst, fullAffine);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<double> cv_findTransformECC_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_int_TermCriteria_const__InputArrayX(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, const cv::TermCriteria* criteria, const cv::_InputArray* inputMask) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_meanShift_const__InputArrayX_RectX_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, const cv::TermCriteria* criteria) {
		try {
			int ret = cv::meanShift(*probImage, *window, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractor_apply_const__InputArrayX_const__OutputArrayX_double(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayX(const cv::BackgroundSubtractor* instance, const cv::_OutputArray* backgroundImage) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getHistory_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			int ret = instance->getHistory();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setHistory_int(cv::BackgroundSubtractorKNN* instance, int history) {
		try {
			instance->setHistory(history);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getNSamples_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			int ret = instance->getNSamples();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nN) {
		try {
			instance->setNSamples(_nN);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorKNN_getDist2Threshold_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			double ret = instance->getDist2Threshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setDist2Threshold_double(cv::BackgroundSubtractorKNN* instance, double _dist2Threshold) {
		try {
			instance->setDist2Threshold(_dist2Threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getkNNSamples_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			int ret = instance->getkNNSamples();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setkNNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nkNN) {
		try {
			instance->setkNNSamples(_nkNN);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_BackgroundSubtractorKNN_getDetectShadows_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			bool ret = instance->getDetectShadows();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setDetectShadows_bool(cv::BackgroundSubtractorKNN* instance, bool detectShadows) {
		try {
			instance->setDetectShadows(detectShadows);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getShadowValue_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			int ret = instance->getShadowValue();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setShadowValue_int(cv::BackgroundSubtractorKNN* instance, int value) {
		try {
			instance->setShadowValue(value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorKNN_getShadowThreshold_const(const cv::BackgroundSubtractorKNN* instance) {
		try {
			double ret = instance->getShadowThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorKNN_setShadowThreshold_double(cv::BackgroundSubtractorKNN* instance, double threshold) {
		try {
			instance->setShadowThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getHistory_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			int ret = instance->getHistory();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setHistory_int(cv::BackgroundSubtractorMOG2* instance, int history) {
		try {
			instance->setHistory(history);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getNMixtures_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			int ret = instance->getNMixtures();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setNMixtures_int(cv::BackgroundSubtractorMOG2* instance, int nmixtures) {
		try {
			instance->setNMixtures(nmixtures);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getBackgroundRatio();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(cv::BackgroundSubtractorMOG2* instance, double ratio) {
		try {
			instance->setBackgroundRatio(ratio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarThreshold_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getVarThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarThreshold_double(cv::BackgroundSubtractorMOG2* instance, double varThreshold) {
		try {
			instance->setVarThreshold(varThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getVarThresholdGen();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(cv::BackgroundSubtractorMOG2* instance, double varThresholdGen) {
		try {
			instance->setVarThresholdGen(varThresholdGen);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarInit_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getVarInit();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarInit_double(cv::BackgroundSubtractorMOG2* instance, double varInit) {
		try {
			instance->setVarInit(varInit);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarMin_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getVarMin();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarMin_double(cv::BackgroundSubtractorMOG2* instance, double varMin) {
		try {
			instance->setVarMin(varMin);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarMax_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getVarMax();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarMax_double(cv::BackgroundSubtractorMOG2* instance, double varMax) {
		try {
			instance->setVarMax(varMax);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getComplexityReductionThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(cv::BackgroundSubtractorMOG2* instance, double ct) {
		try {
			instance->setComplexityReductionThreshold(ct);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_BackgroundSubtractorMOG2_getDetectShadows_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			bool ret = instance->getDetectShadows();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setDetectShadows_bool(cv::BackgroundSubtractorMOG2* instance, bool detectShadows) {
		try {
			instance->setDetectShadows(detectShadows);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getShadowValue_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			int ret = instance->getShadowValue();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setShadowValue_int(cv::BackgroundSubtractorMOG2* instance, int value) {
		try {
			instance->setShadowValue(value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getShadowThreshold_const(const cv::BackgroundSubtractorMOG2* instance) {
		try {
			double ret = instance->getShadowThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setShadowThreshold_double(cv::BackgroundSubtractorMOG2* instance, double threshold) {
		try {
			instance->setShadowThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DenseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(cv::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow) {
		try {
			instance->calc(*I0, *I1, *flow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DenseOpticalFlow_collectGarbage(cv::DenseOpticalFlow* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getTau_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTau();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setTau_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTau(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getLambda_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getLambda();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setLambda_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setLambda(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getTheta_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTheta();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setTheta_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTheta(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getGamma_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setGamma_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setGamma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getScalesNumber_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getScalesNumber();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setScalesNumber_int(cv::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setScalesNumber(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getWarpingsNumber();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setWarpingsNumber(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getEpsilon_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getEpsilon();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setEpsilon_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setEpsilon(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getInnerIterations_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getInnerIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setInnerIterations_int(cv::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setInnerIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getOuterIterations_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getOuterIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setOuterIterations_int(cv::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setOuterIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::DualTVL1OpticalFlow* instance, bool val) {
		try {
			instance->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getScaleStep_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getScaleStep();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setScaleStep_double(cv::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setScaleStep(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getMedianFiltering_const(const cv::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getMedianFiltering();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DualTVL1OpticalFlow_setMedianFiltering_int(cv::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setMedianFiltering(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::DualTVL1OpticalFlow>*> cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow) {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			return Ok(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>))
	}
	
	Result<int> cv_FarnebackOpticalFlow_getNumLevels_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getNumLevels();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setNumLevels_int(cv::FarnebackOpticalFlow* instance, int numLevels) {
		try {
			instance->setNumLevels(numLevels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_FarnebackOpticalFlow_getPyrScale_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPyrScale();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setPyrScale_double(cv::FarnebackOpticalFlow* instance, double pyrScale) {
		try {
			instance->setPyrScale(pyrScale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_FarnebackOpticalFlow_getFastPyramids_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			bool ret = instance->getFastPyramids();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setFastPyramids_bool(cv::FarnebackOpticalFlow* instance, bool fastPyramids) {
		try {
			instance->setFastPyramids(fastPyramids);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FarnebackOpticalFlow_getWinSize_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getWinSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setWinSize_int(cv::FarnebackOpticalFlow* instance, int winSize) {
		try {
			instance->setWinSize(winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FarnebackOpticalFlow_getNumIters_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setNumIters_int(cv::FarnebackOpticalFlow* instance, int numIters) {
		try {
			instance->setNumIters(numIters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FarnebackOpticalFlow_getPolyN_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getPolyN();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setPolyN_int(cv::FarnebackOpticalFlow* instance, int polyN) {
		try {
			instance->setPolyN(polyN);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_FarnebackOpticalFlow_getPolySigma_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPolySigma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setPolySigma_double(cv::FarnebackOpticalFlow* instance, double polySigma) {
		try {
			instance->setPolySigma(polySigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FarnebackOpticalFlow_getFlags_const(const cv::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getFlags();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FarnebackOpticalFlow_setFlags_int(cv::FarnebackOpticalFlow* instance, int flags) {
		try {
			instance->setFlags(flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::FarnebackOpticalFlow>*> cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			return Ok(new cv::Ptr<cv::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FarnebackOpticalFlow>*>))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropStatePre(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->statePre;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropStatePre_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->statePre = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropStatePost(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->statePost;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropStatePost_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->statePost = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTransitionMatrix(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->transitionMatrix;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTransitionMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->transitionMatrix = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropControlMatrix(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->controlMatrix;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropControlMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->controlMatrix = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropMeasurementMatrix(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->measurementMatrix;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropMeasurementMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->measurementMatrix = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropProcessNoiseCov(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->processNoiseCov;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropProcessNoiseCov_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->processNoiseCov = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropMeasurementNoiseCov(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->measurementNoiseCov;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropMeasurementNoiseCov_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->measurementNoiseCov = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropErrorCovPre(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->errorCovPre;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropErrorCovPre_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->errorCovPre = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropGain(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->gain;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropGain_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->gain = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropErrorCovPost(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->errorCovPost;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropErrorCovPost_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->errorCovPost = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTemp1(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->temp1;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTemp1_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->temp1 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTemp2(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->temp2;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTemp2_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->temp2 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTemp3(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->temp3;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTemp3_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->temp3 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTemp4(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->temp4;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTemp4_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->temp4 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_getPropTemp5(cv::KalmanFilter* instance) {
		try {
			cv::Mat ret = instance->temp5;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_KalmanFilter_setPropTemp5_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
		try {
			instance->temp5 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_KalmanFilter_delete(cv::KalmanFilter* instance) {
		delete instance;
	}
	Result<cv::KalmanFilter*> cv_KalmanFilter_KalmanFilter() {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KalmanFilter*>))
	}
	
	Result<cv::KalmanFilter*> cv_KalmanFilter_KalmanFilter_int_int_int_int(int dynamParams, int measureParams, int controlParams, int type) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams, controlParams, type);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KalmanFilter*>))
	}
	
	Result_void cv_KalmanFilter_init_int_int_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, int controlParams, int type) {
		try {
			instance->init(dynamParams, measureParams, controlParams, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_predict_const_MatX(cv::KalmanFilter* instance, const cv::Mat* control) {
		try {
			cv::Mat ret = instance->predict(*control);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_KalmanFilter_correct_const_MatX(cv::KalmanFilter* instance, const cv::Mat* measurement) {
		try {
			cv::Mat ret = instance->correct(*measurement);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_SparseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_SparsePyrLKOpticalFlow_getWinSize_const(const cv::SparsePyrLKOpticalFlow* instance) {
		try {
			cv::Size ret = instance->getWinSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setWinSize_Size(cv::SparsePyrLKOpticalFlow* instance, const cv::Size* winSize) {
		try {
			instance->setWinSize(*winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::SparsePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getMaxLevel();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::SparsePyrLKOpticalFlow* instance, int maxLevel) {
		try {
			instance->setMaxLevel(maxLevel);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::TermCriteria> cv_SparsePyrLKOpticalFlow_getTermCriteria_const(const cv::SparsePyrLKOpticalFlow* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaX(cv::SparsePyrLKOpticalFlow* instance, cv::TermCriteria* crit) {
		try {
			instance->setTermCriteria(*crit);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparsePyrLKOpticalFlow_getFlags_const(const cv::SparsePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getFlags();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setFlags_int(cv::SparsePyrLKOpticalFlow* instance, int flags) {
		try {
			instance->setFlags(flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(const cv::SparsePyrLKOpticalFlow* instance) {
		try {
			double ret = instance->getMinEigThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(cv::SparsePyrLKOpticalFlow* instance, double minEigThreshold) {
		try {
			instance->setMinEigThreshold(minEigThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*> cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(const cv::Size* winSize, int maxLevel, const cv::TermCriteria* crit, int flags, double minEigThreshold) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, *crit, flags, minEigThreshold);
			return Ok(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>))
	}
	
}
