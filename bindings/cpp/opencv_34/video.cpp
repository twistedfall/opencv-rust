#include "common.hpp"
#include <opencv2/video.hpp>
#include "video_types.hpp"

extern "C" {
	Result<void*> cv_CamShift_const__InputArrayX_RectX_TermCriteria(void* probImage, cv::Rect* window, void* criteria) {
		try {
			cv::RotatedRect ret = cv::CamShift(*reinterpret_cast<const cv::_InputArray*>(probImage), *window, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<void*>(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_buildOpticalFlowPyramid_const__InputArrayX_const__OutputArrayX_Size_int_bool_int_int_bool(void* img, void* pyramid, cv::Size winSize, int maxLevel, bool withDerivatives, int pyrBorder, int derivBorder, bool tryReuseInputImage) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_OutputArray*>(pyramid), winSize, maxLevel, withDerivatives, pyrBorder, derivBorder, tryReuseInputImage);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_calcOpticalFlowFarneback_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_double_int_int_int_int_double_int(void* prev, void* next, void* flow, double pyr_scale, int levels, int winsize, int iterations, int poly_n, double poly_sigma, int flags) {
		try {
			cv::calcOpticalFlowFarneback(*reinterpret_cast<const cv::_InputArray*>(prev), *reinterpret_cast<const cv::_InputArray*>(next), *reinterpret_cast<const cv::_InputOutputArray*>(flow), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calcOpticalFlowPyrLK_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_Size_int_TermCriteria_int_double(void* prevImg, void* nextImg, void* prevPts, void* nextPts, void* status, void* err, cv::Size winSize, int maxLevel, void* criteria, int flags, double minEigThreshold) {
		try {
			cv::calcOpticalFlowPyrLK(*reinterpret_cast<const cv::_InputArray*>(prevImg), *reinterpret_cast<const cv::_InputArray*>(nextImg), *reinterpret_cast<const cv::_InputArray*>(prevPts), *reinterpret_cast<const cv::_InputOutputArray*>(nextPts), *reinterpret_cast<const cv::_OutputArray*>(status), *reinterpret_cast<const cv::_OutputArray*>(err), winSize, maxLevel, *reinterpret_cast<cv::TermCriteria*>(criteria), flags, minEigThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_computeECC_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* templateImage, void* inputImage, void* inputMask) {
		try {
			double ret = cv::computeECC(*reinterpret_cast<const cv::_InputArray*>(templateImage), *reinterpret_cast<const cv::_InputArray*>(inputImage), *reinterpret_cast<const cv::_InputArray*>(inputMask));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_createBackgroundSubtractorKNN_int_double_bool(int history, double dist2Threshold, bool detectShadows) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
			return Ok<void*>(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			return Ok<void*>(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createOptFlow_DualTVL1() {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::createOptFlow_DualTVL1();
			return Ok<void*>(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_estimateRigidTransform_const__InputArrayX_const__InputArrayX_bool(void* src, void* dst, bool fullAffine) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst), fullAffine);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_estimateRigidTransform_const__InputArrayX_const__InputArrayX_bool_int_double_int(void* src, void* dst, bool fullAffine, int ransacMaxIters, double ransacGoodRatio, int ransacSize0) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst), fullAffine, ransacMaxIters, ransacGoodRatio, ransacSize0);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_findTransformECC_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_int_TermCriteria_const__InputArrayX(void* templateImage, void* inputImage, void* warpMatrix, int motionType, void* criteria, void* inputMask) {
		try {
			double ret = cv::findTransformECC(*reinterpret_cast<const cv::_InputArray*>(templateImage), *reinterpret_cast<const cv::_InputArray*>(inputImage), *reinterpret_cast<const cv::_InputOutputArray*>(warpMatrix), motionType, *reinterpret_cast<cv::TermCriteria*>(criteria), *reinterpret_cast<const cv::_InputArray*>(inputMask));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_findTransformECC_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_int_TermCriteria_const__InputArrayX_int(void* templateImage, void* inputImage, void* warpMatrix, int motionType, void* criteria, void* inputMask, int gaussFiltSize) {
		try {
			double ret = cv::findTransformECC(*reinterpret_cast<const cv::_InputArray*>(templateImage), *reinterpret_cast<const cv::_InputArray*>(inputImage), *reinterpret_cast<const cv::_InputOutputArray*>(warpMatrix), motionType, *reinterpret_cast<cv::TermCriteria*>(criteria), *reinterpret_cast<const cv::_InputArray*>(inputMask), gaussFiltSize);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_meanShift_const__InputArrayX_RectX_TermCriteria(void* probImage, cv::Rect* window, void* criteria) {
		try {
			int ret = cv::meanShift(*reinterpret_cast<const cv::_InputArray*>(probImage), *window, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractor_apply_const__InputArrayX_const__OutputArrayX_double(void* instance, void* image, void* fgmask, double learningRate) {
		try {
			reinterpret_cast<cv::BackgroundSubtractor*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(fgmask), learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayX(void* instance, void* backgroundImage) {
		try {
			reinterpret_cast<cv::BackgroundSubtractor*>(instance)->getBackgroundImage(*reinterpret_cast<const cv::_OutputArray*>(backgroundImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getHistory_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getHistory();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setHistory_int(void* instance, int history) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setHistory(history);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getNSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getNSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setNSamples_int(void* instance, int _nN) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setNSamples(_nN);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorKNN_getDist2Threshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getDist2Threshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setDist2Threshold_double(void* instance, double _dist2Threshold) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setDist2Threshold(_dist2Threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getkNNSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getkNNSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setkNNSamples_int(void* instance, int _nkNN) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setkNNSamples(_nkNN);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BackgroundSubtractorKNN_getDetectShadows_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getDetectShadows();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setDetectShadows_bool(void* instance, bool detectShadows) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setDetectShadows(detectShadows);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorKNN_getShadowValue_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getShadowValue();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setShadowValue_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setShadowValue(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorKNN_getShadowThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->getShadowThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorKNN_setShadowThreshold_double(void* instance, double threshold) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorKNN*>(instance)->setShadowThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getHistory_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getHistory();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setHistory_int(void* instance, int history) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setHistory(history);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getNMixtures_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getNMixtures();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setNMixtures_int(void* instance, int nmixtures) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setNMixtures(nmixtures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getBackgroundRatio();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(void* instance, double ratio) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setBackgroundRatio(ratio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getVarThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarThreshold_double(void* instance, double varThreshold) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setVarThreshold(varThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getVarThresholdGen();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(void* instance, double varThresholdGen) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setVarThresholdGen(varThresholdGen);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarInit_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getVarInit();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarInit_double(void* instance, double varInit) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setVarInit(varInit);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarMin_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getVarMin();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarMin_double(void* instance, double varMin) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setVarMin(varMin);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getVarMax_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getVarMax();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setVarMax_double(void* instance, double varMax) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setVarMax(varMax);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getComplexityReductionThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(void* instance, double ct) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setComplexityReductionThreshold(ct);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BackgroundSubtractorMOG2_getDetectShadows_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getDetectShadows();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setDetectShadows_bool(void* instance, bool detectShadows) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setDetectShadows(detectShadows);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BackgroundSubtractorMOG2_getShadowValue_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getShadowValue();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setShadowValue_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setShadowValue(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_BackgroundSubtractorMOG2_getShadowThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->getShadowThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_setShadowThreshold_double(void* instance, double threshold) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->setShadowThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BackgroundSubtractorMOG2_apply_const__InputArrayX_const__OutputArrayX_double(void* instance, void* image, void* fgmask, double learningRate) {
		try {
			reinterpret_cast<cv::BackgroundSubtractorMOG2*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(fgmask), learningRate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DenseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(void* instance, void* I0, void* I1, void* flow) {
		try {
			reinterpret_cast<cv::DenseOpticalFlow*>(instance)->calc(*reinterpret_cast<const cv::_InputArray*>(I0), *reinterpret_cast<const cv::_InputArray*>(I1), *reinterpret_cast<const cv::_InputOutputArray*>(flow));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DenseOpticalFlow_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::DenseOpticalFlow*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getTau_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getTau();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setTau_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setTau(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getLambda_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getLambda();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setLambda_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setLambda(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getTheta_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getTheta();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setTheta_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setTheta(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getGamma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getGamma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setGamma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setGamma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getScalesNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getScalesNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setScalesNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setScalesNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getWarpingsNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getWarpingsNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setWarpingsNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setWarpingsNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getEpsilon_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getEpsilon();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setEpsilon_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setEpsilon(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getInnerIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getInnerIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setInnerIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setInnerIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getOuterIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getOuterIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setOuterIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setOuterIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_DualTVL1OpticalFlow_getUseInitialFlow_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getUseInitialFlow();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_DualTVL1OpticalFlow_getScaleStep_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getScaleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setScaleStep_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setScaleStep(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DualTVL1OpticalFlow_getMedianFiltering_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->getMedianFiltering();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DualTVL1OpticalFlow_setMedianFiltering_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DualTVL1OpticalFlow*>(instance)->setMedianFiltering(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow) {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			return Ok<void*>(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_FarnebackOpticalFlow_getNumLevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getNumLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setNumLevels_int(void* instance, int numLevels) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setNumLevels(numLevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_FarnebackOpticalFlow_getPyrScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getPyrScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setPyrScale_double(void* instance, double pyrScale) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setPyrScale(pyrScale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_FarnebackOpticalFlow_getFastPyramids_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getFastPyramids();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setFastPyramids_bool(void* instance, bool fastPyramids) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setFastPyramids(fastPyramids);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FarnebackOpticalFlow_getWinSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getWinSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setWinSize_int(void* instance, int winSize) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setWinSize(winSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FarnebackOpticalFlow_getNumIters_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setNumIters_int(void* instance, int numIters) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setNumIters(numIters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FarnebackOpticalFlow_getPolyN_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getPolyN();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setPolyN_int(void* instance, int polyN) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setPolyN(polyN);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_FarnebackOpticalFlow_getPolySigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getPolySigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setPolySigma_double(void* instance, double polySigma) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setPolySigma(polySigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FarnebackOpticalFlow_getFlags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FarnebackOpticalFlow_setFlags_int(void* instance, int flags) {
		try {
			reinterpret_cast<cv::FarnebackOpticalFlow*>(instance)->setFlags(flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			return Ok<void*>(new cv::Ptr<cv::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_KalmanFilter_statePre(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->statePre;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setStatePre_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->statePre = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_statePost(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->statePost;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setStatePost_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->statePost = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_transitionMatrix(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->transitionMatrix;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTransitionMatrix_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->transitionMatrix = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_controlMatrix(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->controlMatrix;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setControlMatrix_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->controlMatrix = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_measurementMatrix(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->measurementMatrix;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setMeasurementMatrix_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->measurementMatrix = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_processNoiseCov(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->processNoiseCov;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setProcessNoiseCov_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->processNoiseCov = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_measurementNoiseCov(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->measurementNoiseCov;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setMeasurementNoiseCov_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->measurementNoiseCov = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_errorCovPre(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->errorCovPre;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setErrorCovPre_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->errorCovPre = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_gain(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->gain;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setGain_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->gain = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_errorCovPost(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->errorCovPost;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setErrorCovPost_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->errorCovPost = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_temp1(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->temp1;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTemp1_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->temp1 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_temp2(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->temp2;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTemp2_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->temp2 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_temp3(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->temp3;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTemp3_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->temp3 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_temp4(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->temp4;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTemp4_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->temp4 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_temp5(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->temp5;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_setTemp5_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->temp5 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KalmanFilter_delete(cv::KalmanFilter* instance) {
		delete instance;
	}
	Result<void*> cv_KalmanFilter_KalmanFilter() {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_KalmanFilter_KalmanFilter_int_int_int_int(int dynamParams, int measureParams, int controlParams, int type) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams, controlParams, type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KalmanFilter_init_int_int_int_int(void* instance, int dynamParams, int measureParams, int controlParams, int type) {
		try {
			reinterpret_cast<cv::KalmanFilter*>(instance)->init(dynamParams, measureParams, controlParams, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_KalmanFilter_predict_const_MatX(void* instance, void* control) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->predict(*reinterpret_cast<const cv::Mat*>(control));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_KalmanFilter_correct_const_MatX(void* instance, void* measurement) {
		try {
			cv::Mat ret = reinterpret_cast<cv::KalmanFilter*>(instance)->correct(*reinterpret_cast<const cv::Mat*>(measurement));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseOpticalFlow_calc_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* prevImg, void* nextImg, void* prevPts, void* nextPts, void* status, void* err) {
		try {
			reinterpret_cast<cv::SparseOpticalFlow*>(instance)->calc(*reinterpret_cast<const cv::_InputArray*>(prevImg), *reinterpret_cast<const cv::_InputArray*>(nextImg), *reinterpret_cast<const cv::_InputArray*>(prevPts), *reinterpret_cast<const cv::_InputOutputArray*>(nextPts), *reinterpret_cast<const cv::_OutputArray*>(status), *reinterpret_cast<const cv::_OutputArray*>(err));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_SparsePyrLKOpticalFlow_getWinSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->getWinSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setWinSize_Size(void* instance, cv::Size winSize) {
		try {
			reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->setWinSize(winSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparsePyrLKOpticalFlow_getMaxLevel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->getMaxLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setMaxLevel_int(void* instance, int maxLevel) {
		try {
			reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->setMaxLevel(maxLevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparsePyrLKOpticalFlow_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaX(void* instance, void* crit) {
		try {
			reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->setTermCriteria(*reinterpret_cast<cv::TermCriteria*>(crit));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparsePyrLKOpticalFlow_getFlags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setFlags_int(void* instance, int flags) {
		try {
			reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->setFlags(flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->getMinEigThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(void* instance, double minEigThreshold) {
		try {
			reinterpret_cast<cv::SparsePyrLKOpticalFlow*>(instance)->setMinEigThreshold(minEigThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(cv::Size winSize, int maxLevel, void* crit, int flags, double minEigThreshold) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create(winSize, maxLevel, *reinterpret_cast<cv::TermCriteria*>(crit), flags, minEigThreshold);
			return Ok<void*>(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
