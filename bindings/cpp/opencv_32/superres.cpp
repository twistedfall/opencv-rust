#include "common.hpp"
#include <opencv2/superres.hpp>
#include "superres_types.hpp"

extern "C" {
	Result<void*> cv_superres_createFrameSource_Camera_int(int deviceId) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Camera(deviceId);
			return Ok<void*>(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createFrameSource_Empty() {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Empty();
			return Ok<void*>(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createFrameSource_Video_CUDA_const_StringX(const char* fileName) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video_CUDA(cv::String(fileName));
			return Ok<void*>(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createFrameSource_Video_const_StringX(const char* fileName) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video(cv::String(fileName));
			return Ok<void*>(new cv::Ptr<cv::superres::FrameSource>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_Brox_CUDA() {
		try {
			cv::Ptr<cv::superres::BroxOpticalFlow> ret = cv::superres::createOptFlow_Brox_CUDA();
			return Ok<void*>(new cv::Ptr<cv::superres::BroxOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_DualTVL1() {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1();
			return Ok<void*>(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_DualTVL1_CUDA() {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1_CUDA();
			return Ok<void*>(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_Farneback() {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback();
			return Ok<void*>(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_Farneback_CUDA() {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback_CUDA();
			return Ok<void*>(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createOptFlow_PyrLK_CUDA() {
		try {
			cv::Ptr<cv::superres::PyrLKOpticalFlow> ret = cv::superres::createOptFlow_PyrLK_CUDA();
			return Ok<void*>(new cv::Ptr<cv::superres::PyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createSuperResolution_BTVL1() {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1();
			return Ok<void*>(new cv::Ptr<cv::superres::SuperResolution>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_superres_createSuperResolution_BTVL1_CUDA() {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1_CUDA();
			return Ok<void*>(new cv::Ptr<cv::superres::SuperResolution>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getAlpha_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getAlpha();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setAlpha_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setAlpha(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getGamma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getGamma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setGamma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setGamma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_BroxOpticalFlow_getScaleFactor_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getScaleFactor();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setScaleFactor_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setScaleFactor(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getInnerIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getInnerIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setInnerIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setInnerIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getOuterIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getOuterIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setOuterIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setOuterIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_BroxOpticalFlow_getSolverIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->getSolverIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_BroxOpticalFlow_setSolverIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::BroxOpticalFlow*>(instance)->setSolverIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_DenseOpticalFlowExt_calc_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* frame0, void* frame1, void* flow1, void* flow2) {
		try {
			reinterpret_cast<cv::superres::DenseOpticalFlowExt*>(instance)->calc(*reinterpret_cast<const cv::_InputArray*>(frame0), *reinterpret_cast<const cv::_InputArray*>(frame1), *reinterpret_cast<const cv::_OutputArray*>(flow1), *reinterpret_cast<const cv::_OutputArray*>(flow2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_DenseOpticalFlowExt_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::superres::DenseOpticalFlowExt*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getTau_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getTau();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setTau_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setTau(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getLambda_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getLambda();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setLambda_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setLambda(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getTheta_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getTheta();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setTheta_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setTheta(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getScalesNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setScalesNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getWarpingsNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setWarpingsNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_DualTVL1OpticalFlow_getEpsilon_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getEpsilon();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setEpsilon_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setEpsilon(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_DualTVL1OpticalFlow_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->getUseInitialFlow();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::superres::DualTVL1OpticalFlow*>(instance)->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_FarnebackOpticalFlow_getPyrScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getPyrScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPyrScale_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setPyrScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getLevelsNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setLevelsNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getWindowSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getWindowSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setWindowSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getPolyN_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getPolyN();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPolyN_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setPolyN(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_FarnebackOpticalFlow_getPolySigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getPolySigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setPolySigma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setPolySigma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_FarnebackOpticalFlow_getFlags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_FarnebackOpticalFlow_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::FarnebackOpticalFlow*>(instance)->setFlags(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_FrameSource_nextFrame_const__OutputArrayX(void* instance, void* frame) {
		try {
			reinterpret_cast<cv::superres::FrameSource*>(instance)->nextFrame(*reinterpret_cast<const cv::_OutputArray*>(frame));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_FrameSource_reset(void* instance) {
		try {
			reinterpret_cast<cv::superres::FrameSource*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getWindowSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->getWindowSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setWindowSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getMaxLevel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->getMaxLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setMaxLevel_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->setMaxLevel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_PyrLKOpticalFlow_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_PyrLKOpticalFlow_setIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::PyrLKOpticalFlow*>(instance)->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_setInput_const_Ptr_FrameSource_X(void* instance, void* frameSource) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setInput(*reinterpret_cast<const cv::Ptr<cv::superres::FrameSource>*>(frameSource));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_nextFrame_const__OutputArrayX(void* instance, void* frame) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->nextFrame(*reinterpret_cast<const cv::_OutputArray*>(frame));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_reset(void* instance) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_superres_SuperResolution_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getScale_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getScale();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setScale_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getTau_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getTau();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setTau_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setTau(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getLabmda_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getLabmda();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setLabmda_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setLabmda(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getAlpha_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getAlpha();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setAlpha_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setAlpha(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getKernelSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getKernelSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setKernelSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setKernelSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getBlurKernelSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getBlurKernelSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setBlurKernelSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setBlurKernelSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_superres_SuperResolution_getBlurSigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getBlurSigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_superres_SuperResolution_setBlurSigma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setBlurSigma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_superres_SuperResolution_getTemporalAreaRadius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getTemporalAreaRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_superres_SuperResolution_setTemporalAreaRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setTemporalAreaRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_superres_SuperResolution_getOpticalFlow_const(void* instance) {
		try {
			cv::Ptr<cv::superres::DenseOpticalFlowExt> ret = reinterpret_cast<cv::superres::SuperResolution*>(instance)->getOpticalFlow();
			return Ok<void*>(new cv::Ptr<cv::superres::DenseOpticalFlowExt>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_superres_SuperResolution_setOpticalFlow_const_Ptr_DenseOpticalFlowExt_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::superres::SuperResolution*>(instance)->setOpticalFlow(*reinterpret_cast<const cv::Ptr<cv::superres::DenseOpticalFlowExt>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
