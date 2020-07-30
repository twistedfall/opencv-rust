#include "common.hpp"
#include <opencv2/cudaoptflow.hpp>
#include "cudaoptflow_types.hpp"

extern "C" {
	Result<double> cv_cuda_BroxOpticalFlow_getFlowSmoothness_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getFlowSmoothness();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setFlowSmoothness_double(cv::cuda::BroxOpticalFlow* instance, double alpha) {
		try {
			instance->setFlowSmoothness(alpha);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_BroxOpticalFlow_getGradientConstancyImportance_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getGradientConstancyImportance();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setGradientConstancyImportance_double(cv::cuda::BroxOpticalFlow* instance, double gamma) {
		try {
			instance->setGradientConstancyImportance(gamma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_BroxOpticalFlow_getPyramidScaleFactor_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			double ret = instance->getPyramidScaleFactor();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setPyramidScaleFactor_double(cv::cuda::BroxOpticalFlow* instance, double scale_factor) {
		try {
			instance->setPyramidScaleFactor(scale_factor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_BroxOpticalFlow_getInnerIterations_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getInnerIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setInnerIterations_int(cv::cuda::BroxOpticalFlow* instance, int inner_iterations) {
		try {
			instance->setInnerIterations(inner_iterations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_BroxOpticalFlow_getOuterIterations_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getOuterIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setOuterIterations_int(cv::cuda::BroxOpticalFlow* instance, int outer_iterations) {
		try {
			instance->setOuterIterations(outer_iterations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_BroxOpticalFlow_getSolverIterations_const(const cv::cuda::BroxOpticalFlow* instance) {
		try {
			int ret = instance->getSolverIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_BroxOpticalFlow_setSolverIterations_int(cv::cuda::BroxOpticalFlow* instance, int solver_iterations) {
		try {
			instance->setSolverIterations(solver_iterations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::BroxOpticalFlow>*> cv_cuda_BroxOpticalFlow_create_double_double_double_int_int_int(double alpha, double gamma, double scale_factor, int inner_iterations, int outer_iterations, int solver_iterations) {
		try {
			cv::Ptr<cv::cuda::BroxOpticalFlow> ret = cv::cuda::BroxOpticalFlow::create(alpha, gamma, scale_factor, inner_iterations, outer_iterations, solver_iterations);
			return Ok(new cv::Ptr<cv::cuda::BroxOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::BroxOpticalFlow>*>))
	}
	
	Result_void cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR(cv::cuda::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, cv::cuda::Stream* stream) {
		try {
			instance->calc(*I0, *I1, *flow, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_cuda_DensePyrLKOpticalFlow_getWinSize_const(const cv::cuda::DensePyrLKOpticalFlow* instance) {
		try {
			cv::Size ret = instance->getWinSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_DensePyrLKOpticalFlow_setWinSize_Size(cv::cuda::DensePyrLKOpticalFlow* instance, cv::Size* winSize) {
		try {
			instance->setWinSize(*winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_DensePyrLKOpticalFlow_getMaxLevel_const(const cv::cuda::DensePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getMaxLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DensePyrLKOpticalFlow_setMaxLevel_int(cv::cuda::DensePyrLKOpticalFlow* instance, int maxLevel) {
		try {
			instance->setMaxLevel(maxLevel);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_DensePyrLKOpticalFlow_getNumIters_const(const cv::cuda::DensePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DensePyrLKOpticalFlow_setNumIters_int(cv::cuda::DensePyrLKOpticalFlow* instance, int iters) {
		try {
			instance->setNumIters(iters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_DensePyrLKOpticalFlow_getUseInitialFlow_const(const cv::cuda::DensePyrLKOpticalFlow* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_DensePyrLKOpticalFlow_setUseInitialFlow_bool(cv::cuda::DensePyrLKOpticalFlow* instance, bool useInitialFlow) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>*> cv_cuda_DensePyrLKOpticalFlow_create_Size_int_int_bool(cv::Size* winSize, int maxLevel, int iters, bool useInitialFlow) {
		try {
			cv::Ptr<cv::cuda::DensePyrLKOpticalFlow> ret = cv::cuda::DensePyrLKOpticalFlow::create(*winSize, maxLevel, iters, useInitialFlow);
			return Ok(new cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>*>))
	}
	
	Result<int> cv_cuda_FarnebackOpticalFlow_getNumLevels_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getNumLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setNumLevels_int(cv::cuda::FarnebackOpticalFlow* instance, int numLevels) {
		try {
			instance->setNumLevels(numLevels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_FarnebackOpticalFlow_getPyrScale_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPyrScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setPyrScale_double(cv::cuda::FarnebackOpticalFlow* instance, double pyrScale) {
		try {
			instance->setPyrScale(pyrScale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_FarnebackOpticalFlow_getFastPyramids_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			bool ret = instance->getFastPyramids();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setFastPyramids_bool(cv::cuda::FarnebackOpticalFlow* instance, bool fastPyramids) {
		try {
			instance->setFastPyramids(fastPyramids);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_FarnebackOpticalFlow_getWinSize_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getWinSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setWinSize_int(cv::cuda::FarnebackOpticalFlow* instance, int winSize) {
		try {
			instance->setWinSize(winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_FarnebackOpticalFlow_getNumIters_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setNumIters_int(cv::cuda::FarnebackOpticalFlow* instance, int numIters) {
		try {
			instance->setNumIters(numIters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_FarnebackOpticalFlow_getPolyN_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getPolyN();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setPolyN_int(cv::cuda::FarnebackOpticalFlow* instance, int polyN) {
		try {
			instance->setPolyN(polyN);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_FarnebackOpticalFlow_getPolySigma_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			double ret = instance->getPolySigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setPolySigma_double(cv::cuda::FarnebackOpticalFlow* instance, double polySigma) {
		try {
			instance->setPolySigma(polySigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_FarnebackOpticalFlow_getFlags_const(const cv::cuda::FarnebackOpticalFlow* instance) {
		try {
			int ret = instance->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_FarnebackOpticalFlow_setFlags_int(cv::cuda::FarnebackOpticalFlow* instance, int flags) {
		try {
			instance->setFlags(flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::FarnebackOpticalFlow>*> cv_cuda_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags) {
		try {
			cv::Ptr<cv::cuda::FarnebackOpticalFlow> ret = cv::cuda::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			return Ok(new cv::Ptr<cv::cuda::FarnebackOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::FarnebackOpticalFlow>*>))
	}
	
	Result_void cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR_const__InputArrayR_const__OutputArrayR(cv::cuda::NvidiaHWOpticalFlow* instance, const cv::_InputArray* inputImage, const cv::_InputArray* referenceImage, const cv::_InputOutputArray* flow, cv::cuda::Stream* stream, const cv::_InputArray* hint, const cv::_OutputArray* cost) {
		try {
			instance->calc(*inputImage, *referenceImage, *flow, *stream, *hint, *cost);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_NvidiaHWOpticalFlow_collectGarbage(cv::cuda::NvidiaHWOpticalFlow* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_NvidiaHWOpticalFlow_getGridSize_const(const cv::cuda::NvidiaHWOpticalFlow* instance) {
		try {
			int ret = instance->getGridSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_NvidiaOpticalFlow_1_0_upSampler_const__InputArrayR_int_int_int_const__InputOutputArrayR(cv::cuda::NvidiaOpticalFlow_1_0* instance, const cv::_InputArray* flow, int width, int height, int gridSize, const cv::_InputOutputArray* upsampledFlow) {
		try {
			instance->upSampler(*flow, width, height, gridSize, *upsampledFlow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>*> cv_cuda_NvidiaOpticalFlow_1_0_create_int_int_NVIDIA_OF_PERF_LEVEL_bool_bool_bool_int_StreamR_StreamR(int width, int height, cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL perfPreset, bool enableTemporalHints, bool enableExternalHints, bool enableCostBuffer, int gpuId, cv::cuda::Stream* inputStream, cv::cuda::Stream* outputStream) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0> ret = cv::cuda::NvidiaOpticalFlow_1_0::create(width, height, perfPreset, enableTemporalHints, enableExternalHints, enableCostBuffer, gpuId, *inputStream, *outputStream);
			return Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>*>))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getTau_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getTau();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setTau_double(cv::cuda::OpticalFlowDual_TVL1* instance, double tau) {
		try {
			instance->setTau(tau);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getLambda_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getLambda();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setLambda_double(cv::cuda::OpticalFlowDual_TVL1* instance, double lambda) {
		try {
			instance->setLambda(lambda);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getGamma_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getGamma();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setGamma_double(cv::cuda::OpticalFlowDual_TVL1* instance, double gamma) {
		try {
			instance->setGamma(gamma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getTheta_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getTheta();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setTheta_double(cv::cuda::OpticalFlowDual_TVL1* instance, double theta) {
		try {
			instance->setTheta(theta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_OpticalFlowDual_TVL1_getNumScales_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			int ret = instance->getNumScales();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setNumScales_int(cv::cuda::OpticalFlowDual_TVL1* instance, int nscales) {
		try {
			instance->setNumScales(nscales);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_OpticalFlowDual_TVL1_getNumWarps_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			int ret = instance->getNumWarps();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setNumWarps_int(cv::cuda::OpticalFlowDual_TVL1* instance, int warps) {
		try {
			instance->setNumWarps(warps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getEpsilon_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getEpsilon();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setEpsilon_double(cv::cuda::OpticalFlowDual_TVL1* instance, double epsilon) {
		try {
			instance->setEpsilon(epsilon);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_OpticalFlowDual_TVL1_getNumIterations_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			int ret = instance->getNumIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setNumIterations_int(cv::cuda::OpticalFlowDual_TVL1* instance, int iterations) {
		try {
			instance->setNumIterations(iterations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_OpticalFlowDual_TVL1_getScaleStep_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			double ret = instance->getScaleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setScaleStep_double(cv::cuda::OpticalFlowDual_TVL1* instance, double scaleStep) {
		try {
			instance->setScaleStep(scaleStep);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_OpticalFlowDual_TVL1_getUseInitialFlow_const(const cv::cuda::OpticalFlowDual_TVL1* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_OpticalFlowDual_TVL1_setUseInitialFlow_bool(cv::cuda::OpticalFlowDual_TVL1* instance, bool useInitialFlow) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>*> cv_cuda_OpticalFlowDual_TVL1_create_double_double_double_int_int_double_int_double_double_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int iterations, double scaleStep, double gamma, bool useInitialFlow) {
		try {
			cv::Ptr<cv::cuda::OpticalFlowDual_TVL1> ret = cv::cuda::OpticalFlowDual_TVL1::create(tau, lambda, theta, nscales, warps, epsilon, iterations, scaleStep, gamma, useInitialFlow);
			return Ok(new cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>*>))
	}
	
	Result_void cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::cuda::Stream* stream) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_cuda_SparsePyrLKOpticalFlow_getWinSize_const(const cv::cuda::SparsePyrLKOpticalFlow* instance) {
		try {
			cv::Size ret = instance->getWinSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_SparsePyrLKOpticalFlow_setWinSize_Size(cv::cuda::SparsePyrLKOpticalFlow* instance, cv::Size* winSize) {
		try {
			instance->setWinSize(*winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::cuda::SparsePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getMaxLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::cuda::SparsePyrLKOpticalFlow* instance, int maxLevel) {
		try {
			instance->setMaxLevel(maxLevel);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_SparsePyrLKOpticalFlow_getNumIters_const(const cv::cuda::SparsePyrLKOpticalFlow* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_SparsePyrLKOpticalFlow_setNumIters_int(cv::cuda::SparsePyrLKOpticalFlow* instance, int iters) {
		try {
			instance->setNumIters(iters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_SparsePyrLKOpticalFlow_getUseInitialFlow_const(const cv::cuda::SparsePyrLKOpticalFlow* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_SparsePyrLKOpticalFlow_setUseInitialFlow_bool(cv::cuda::SparsePyrLKOpticalFlow* instance, bool useInitialFlow) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>*> cv_cuda_SparsePyrLKOpticalFlow_create_Size_int_int_bool(cv::Size* winSize, int maxLevel, int iters, bool useInitialFlow) {
		try {
			cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow> ret = cv::cuda::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, iters, useInitialFlow);
			return Ok(new cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>*>))
	}
	
}
