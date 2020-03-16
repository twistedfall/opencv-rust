#include "common.hpp"
#include <opencv2/superres.hpp>
#include "superres_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::superres::FrameSource>*> cv_superres_createFrameSource_Camera_int(int deviceId) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Camera(deviceId);
			return Ok(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FrameSource>*>)
	}
	
	Result<cv::Ptr<cv::superres::FrameSource>*> cv_superres_createFrameSource_Empty() {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Empty();
			return Ok(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FrameSource>*>)
	}
	
	Result<cv::Ptr<cv::superres::FrameSource>*> cv_superres_createFrameSource_Video_CUDA_const_StringX(const char* fileName) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video_CUDA(std::string(fileName));
			return Ok(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FrameSource>*>)
	}
	
	Result<cv::Ptr<cv::superres::FrameSource>*> cv_superres_createFrameSource_Video_const_StringX(const char* fileName) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video(std::string(fileName));
			return Ok(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FrameSource>*>)
	}
	
	Result<cv::Ptr<cv::superres::BroxOpticalFlow>*> cv_superres_createOptFlow_Brox_CUDA() {
		try {
			cv::Ptr<cv::superres::BroxOpticalFlow> ret = cv::superres::createOptFlow_Brox_CUDA();
			return Ok(new cv::Ptr<cv::superres::BroxOpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::BroxOpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*> cv_superres_createOptFlow_DualTVL1() {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1();
			return Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*> cv_superres_createOptFlow_DualTVL1_CUDA() {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1_CUDA();
			return Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*> cv_superres_createOptFlow_Farneback() {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback();
			return Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*> cv_superres_createOptFlow_Farneback_CUDA() {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback_CUDA();
			return Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*> cv_superres_createOptFlow_PyrLK_CUDA() {
		try {
			cv::Ptr<cv::superres::PyrLKOpticalFlow> ret = cv::superres::createOptFlow_PyrLK_CUDA();
			return Ok(new cv::Ptr<cv::superres::PyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*>)
	}
	
	Result<cv::Ptr<cv::superres::SuperResolution>*> cv_superres_createSuperResolution_BTVL1() {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1();
			return Ok(new cv::Ptr<cv::superres::SuperResolution>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::SuperResolution>*>)
	}
	
	Result<cv::Ptr<cv::superres::SuperResolution>*> cv_superres_createSuperResolution_BTVL1_CUDA() {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1_CUDA();
			return Ok(new cv::Ptr<cv::superres::SuperResolution>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::SuperResolution>*>)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getAlpha_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getAlpha();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setAlpha_double(cv::superres::BroxOpticalFlow* instance, double val) {
		try {
			instance->setAlpha(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getGamma_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setGamma_double(cv::superres::BroxOpticalFlow* instance, double val) {
		try {
			instance->setGamma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getScaleFactor_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getScaleFactor();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setScaleFactor_double(cv::superres::BroxOpticalFlow* instance, double val) {
		try {
			instance->setScaleFactor(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getInnerIterations_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getInnerIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setInnerIterations_int(cv::superres::BroxOpticalFlow* instance, int val) {
		try {
			instance->setInnerIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getOuterIterations_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getOuterIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setOuterIterations_int(cv::superres::BroxOpticalFlow* instance, int val) {
		try {
			instance->setOuterIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getSolverIterations_const(const cv::superres::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getSolverIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setSolverIterations_int(cv::superres::BroxOpticalFlow* instance, int val) {
		try {
			instance->setSolverIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_DenseOpticalFlowExt_calc_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::superres::DenseOpticalFlowExt* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_OutputArray* flow1, const cv::_OutputArray* flow2) {
		try {
			instance->calc(*frame0, *frame1, *flow1, *flow2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_DenseOpticalFlowExt_collectGarbage(cv::superres::DenseOpticalFlowExt* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getTau_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTau();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setTau_double(cv::superres::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTau(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getLambda_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getLambda();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setLambda_double(cv::superres::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setLambda(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getTheta_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTheta();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setTheta_double(cv::superres::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTheta(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getScalesNumber();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setScalesNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getWarpingsNumber();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setWarpingsNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getEpsilon_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getEpsilon();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setEpsilon_double(cv::superres::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setEpsilon(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getIterations_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setIterations_int(cv::superres::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::superres::DualTVL1OpticalFlow* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::superres::DualTVL1OpticalFlow* instance, bool val) {
		try {
			instance->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_FarnebackOpticalFlow_getPyrScale_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPyrScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPyrScale_double(cv::superres::FarnebackOpticalFlow* instance, double val) {
		try {
			instance->setPyrScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getLevelsNumber();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(cv::superres::FarnebackOpticalFlow* instance, int val) {
		try {
			instance->setLevelsNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getWindowSize_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setWindowSize_int(cv::superres::FarnebackOpticalFlow* instance, int val) {
		try {
			instance->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getIterations_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setIterations_int(cv::superres::FarnebackOpticalFlow* instance, int val) {
		try {
			instance->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getPolyN_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getPolyN();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPolyN_int(cv::superres::FarnebackOpticalFlow* instance, int val) {
		try {
			instance->setPolyN(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_FarnebackOpticalFlow_getPolySigma_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPolySigma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPolySigma_double(cv::superres::FarnebackOpticalFlow* instance, double val) {
		try {
			instance->setPolySigma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getFlags_const(const cv::superres::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getFlags();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setFlags_int(cv::superres::FarnebackOpticalFlow* instance, int val) {
		try {
			instance->setFlags(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_FrameSource_nextFrame_const__OutputArrayX(cv::superres::FrameSource* instance, const cv::_OutputArray* frame) {
		try {
			instance->nextFrame(*frame);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_FrameSource_reset(cv::superres::FrameSource* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getWindowSize_const(const cv::superres::PyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setWindowSize_int(cv::superres::PyrLKOpticalFlow* instance, int val) {
		try {
			instance->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getMaxLevel_const(const cv::superres::PyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getMaxLevel();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setMaxLevel_int(cv::superres::PyrLKOpticalFlow* instance, int val) {
		try {
			instance->setMaxLevel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getIterations_const(const cv::superres::PyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setIterations_int(cv::superres::PyrLKOpticalFlow* instance, int val) {
		try {
			instance->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_setInput_const_Ptr_FrameSource_X(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::FrameSource>* frameSource) {
		try {
			instance->setInput(*frameSource);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_nextFrame_const__OutputArrayX(cv::superres::SuperResolution* instance, const cv::_OutputArray* frame) {
		try {
			instance->nextFrame(*frame);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_reset(cv::superres::SuperResolution* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_collectGarbage(cv::superres::SuperResolution* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getScale_const(const cv::superres::SuperResolution* instance) {
		try {
			int ret = instance->getScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setScale_int(cv::superres::SuperResolution* instance, int val) {
		try {
			instance->setScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getIterations_const(const cv::superres::SuperResolution* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setIterations_int(cv::superres::SuperResolution* instance, int val) {
		try {
			instance->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getTau_const(const cv::superres::SuperResolution* instance) {
		try {
			double ret = instance->getTau();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setTau_double(cv::superres::SuperResolution* instance, double val) {
		try {
			instance->setTau(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getLambda_const(const cv::superres::SuperResolution* instance) {
		try {
			double ret = instance->getLambda();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setLambda_double(cv::superres::SuperResolution* instance, double val) {
		try {
			instance->setLambda(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getAlpha_const(const cv::superres::SuperResolution* instance) {
		try {
			double ret = instance->getAlpha();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setAlpha_double(cv::superres::SuperResolution* instance, double val) {
		try {
			instance->setAlpha(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getKernelSize_const(const cv::superres::SuperResolution* instance) {
		try {
			int ret = instance->getKernelSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setKernelSize_int(cv::superres::SuperResolution* instance, int val) {
		try {
			instance->setKernelSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getBlurKernelSize_const(const cv::superres::SuperResolution* instance) {
		try {
			int ret = instance->getBlurKernelSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setBlurKernelSize_int(cv::superres::SuperResolution* instance, int val) {
		try {
			instance->setBlurKernelSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getBlurSigma_const(const cv::superres::SuperResolution* instance) {
		try {
			double ret = instance->getBlurSigma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setBlurSigma_double(cv::superres::SuperResolution* instance, double val) {
		try {
			instance->setBlurSigma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getTemporalAreaRadius_const(const cv::superres::SuperResolution* instance) {
		try {
			int ret = instance->getTemporalAreaRadius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setTemporalAreaRadius_int(cv::superres::SuperResolution* instance, int val) {
		try {
			instance->setTemporalAreaRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*> cv_superres_SuperResolution_getOpticalFlow_const(const cv::superres::SuperResolution* instance) {
		try {
			cv::Ptr<cv::superres::DenseOpticalFlowExt> ret = instance->getOpticalFlow();
			return Ok(new cv::Ptr<cv::superres::DenseOpticalFlowExt>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*>)
	}
	
	Result_void cv_superres_SuperResolution_setOpticalFlow_const_Ptr_DenseOpticalFlowExt_X(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::DenseOpticalFlowExt>* val) {
		try {
			instance->setOpticalFlow(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
