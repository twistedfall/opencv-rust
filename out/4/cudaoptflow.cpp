#include "ocvrs_common.hpp"
#include <opencv2/cudaoptflow.hpp>
#include "cudaoptflow_types.hpp"

extern "C" {
	// getFlowSmoothness()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:158
	// ("cv::cuda::BroxOpticalFlow::getFlowSmoothness", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getFlowSmoothness_const(const cv::cuda::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getFlowSmoothness();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlowSmoothness(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:159
	// ("cv::cuda::BroxOpticalFlow::setFlowSmoothness", vec![(pred!(mut, ["alpha"], ["double"]), _)]),
	void cv_cuda_BroxOpticalFlow_setFlowSmoothness_double(cv::cuda::BroxOpticalFlow* instance, double alpha, ResultVoid* ocvrs_return) {
		try {
			instance->setFlowSmoothness(alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientConstancyImportance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:161
	// ("cv::cuda::BroxOpticalFlow::getGradientConstancyImportance", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getGradientConstancyImportance_const(const cv::cuda::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGradientConstancyImportance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientConstancyImportance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:162
	// ("cv::cuda::BroxOpticalFlow::setGradientConstancyImportance", vec![(pred!(mut, ["gamma"], ["double"]), _)]),
	void cv_cuda_BroxOpticalFlow_setGradientConstancyImportance_double(cv::cuda::BroxOpticalFlow* instance, double gamma, ResultVoid* ocvrs_return) {
		try {
			instance->setGradientConstancyImportance(gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyramidScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:164
	// ("cv::cuda::BroxOpticalFlow::getPyramidScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getPyramidScaleFactor_const(const cv::cuda::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyramidScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPyramidScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:165
	// ("cv::cuda::BroxOpticalFlow::setPyramidScaleFactor", vec![(pred!(mut, ["scale_factor"], ["double"]), _)]),
	void cv_cuda_BroxOpticalFlow_setPyramidScaleFactor_double(cv::cuda::BroxOpticalFlow* instance, double scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setPyramidScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:168
	// ("cv::cuda::BroxOpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getInnerIterations_const(const cv::cuda::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:169
	// ("cv::cuda::BroxOpticalFlow::setInnerIterations", vec![(pred!(mut, ["inner_iterations"], ["int"]), _)]),
	void cv_cuda_BroxOpticalFlow_setInnerIterations_int(cv::cuda::BroxOpticalFlow* instance, int inner_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setInnerIterations(inner_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:172
	// ("cv::cuda::BroxOpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getOuterIterations_const(const cv::cuda::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:173
	// ("cv::cuda::BroxOpticalFlow::setOuterIterations", vec![(pred!(mut, ["outer_iterations"], ["int"]), _)]),
	void cv_cuda_BroxOpticalFlow_setOuterIterations_int(cv::cuda::BroxOpticalFlow* instance, int outer_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setOuterIterations(outer_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSolverIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:176
	// ("cv::cuda::BroxOpticalFlow::getSolverIterations", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_getSolverIterations_const(const cv::cuda::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSolverIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSolverIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:177
	// ("cv::cuda::BroxOpticalFlow::setSolverIterations", vec![(pred!(mut, ["solver_iterations"], ["int"]), _)]),
	void cv_cuda_BroxOpticalFlow_setSolverIterations_int(cv::cuda::BroxOpticalFlow* instance, int solver_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setSolverIterations(solver_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, double, double, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:179
	// ("cv::cuda::BroxOpticalFlow::create", vec![(pred!(mut, ["alpha", "gamma", "scale_factor", "inner_iterations", "outer_iterations", "solver_iterations"], ["double", "double", "double", "int", "int", "int"]), _)]),
	void cv_cuda_BroxOpticalFlow_create_double_double_double_int_int_int(double alpha, double gamma, double scale_factor, int inner_iterations, int outer_iterations, int solver_iterations, Result<cv::Ptr<cv::cuda::BroxOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BroxOpticalFlow> ret = cv::cuda::BroxOpticalFlow::create(alpha, gamma, scale_factor, inner_iterations, outer_iterations, solver_iterations);
			Ok(new cv::Ptr<cv::cuda::BroxOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BroxOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:179
	// ("cv::cuda::BroxOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_create(Result<cv::Ptr<cv::cuda::BroxOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BroxOpticalFlow> ret = cv::cuda::BroxOpticalFlow::create();
			Ok(new cv::Ptr<cv::cuda::BroxOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BroxOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::BroxOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_BroxOpticalFlow_to_Algorithm(cv::cuda::BroxOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::BroxOpticalFlow::to_CUDA_DenseOpticalFlow() generated
	// ("cv::cuda::BroxOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DenseOpticalFlow* cv_cuda_BroxOpticalFlow_to_CUDA_DenseOpticalFlow(cv::cuda::BroxOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::DenseOpticalFlow*>(instance);
	}

	// cv::cuda::BroxOpticalFlow::delete() generated
	// ("cv::cuda::BroxOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BroxOpticalFlow_delete(cv::cuda::BroxOpticalFlow* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputOutputArray, Stream &)(InputArray, InputArray, InputOutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:80
	// ("cv::cuda::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR(cv::cuda::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DenseOpticalFlow::calc(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:80
	// ("cv::cuda::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::cuda::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DenseOpticalFlow::to_CUDA_BroxOpticalFlow() generated
	// ("cv::cuda::DenseOpticalFlow::to_CUDA_BroxOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::BroxOpticalFlow* cv_cuda_DenseOpticalFlow_to_CUDA_BroxOpticalFlow(cv::cuda::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::BroxOpticalFlow*>(instance);
	}

	// cv::cuda::DenseOpticalFlow::to_CUDA_DensePyrLKOpticalFlow() generated
	// ("cv::cuda::DenseOpticalFlow::to_CUDA_DensePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DensePyrLKOpticalFlow* cv_cuda_DenseOpticalFlow_to_CUDA_DensePyrLKOpticalFlow(cv::cuda::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::DensePyrLKOpticalFlow*>(instance);
	}

	// cv::cuda::DenseOpticalFlow::to_CUDA_FarnebackOpticalFlow() generated
	// ("cv::cuda::DenseOpticalFlow::to_CUDA_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::FarnebackOpticalFlow* cv_cuda_DenseOpticalFlow_to_CUDA_FarnebackOpticalFlow(cv::cuda::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::FarnebackOpticalFlow*>(instance);
	}

	// cv::cuda::DenseOpticalFlow::to_CUDA_OpticalFlowDual_TVL1() generated
	// ("cv::cuda::DenseOpticalFlow::to_CUDA_OpticalFlowDual_TVL1", vec![(pred!(mut, [], []), _)]),
	cv::cuda::OpticalFlowDual_TVL1* cv_cuda_DenseOpticalFlow_to_CUDA_OpticalFlowDual_TVL1(cv::cuda::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::OpticalFlowDual_TVL1*>(instance);
	}

	// cv::cuda::DenseOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::DenseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_DenseOpticalFlow_to_Algorithm(cv::cuda::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::DenseOpticalFlow::delete() generated
	// ("cv::cuda::DenseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DenseOpticalFlow_delete(cv::cuda::DenseOpticalFlow* instance) {
			delete instance;
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:233
	// ("cv::cuda::DensePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_getWinSize_const(const cv::cuda::DensePyrLKOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:234
	// ("cv::cuda::DensePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_setWinSize_Size(cv::cuda::DensePyrLKOpticalFlow* instance, cv::Size* winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(*winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:236
	// ("cv::cuda::DensePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_getMaxLevel_const(const cv::cuda::DensePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:237
	// ("cv::cuda::DensePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_setMaxLevel_int(cv::cuda::DensePyrLKOpticalFlow* instance, int maxLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(maxLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:239
	// ("cv::cuda::DensePyrLKOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_getNumIters_const(const cv::cuda::DensePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:240
	// ("cv::cuda::DensePyrLKOpticalFlow::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_setNumIters_int(cv::cuda::DensePyrLKOpticalFlow* instance, int iters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(iters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:242
	// ("cv::cuda::DensePyrLKOpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_getUseInitialFlow_const(const cv::cuda::DensePyrLKOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:243
	// ("cv::cuda::DensePyrLKOpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_setUseInitialFlow_bool(cv::cuda::DensePyrLKOpticalFlow* instance, bool useInitialFlow, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:245
	// ("cv::cuda::DensePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "iters", "useInitialFlow"], ["cv::Size", "int", "int", "bool"]), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_create_Size_int_int_bool(cv::Size* winSize, int maxLevel, int iters, bool useInitialFlow, Result<cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DensePyrLKOpticalFlow> ret = cv::cuda::DensePyrLKOpticalFlow::create(*winSize, maxLevel, iters, useInitialFlow);
			Ok(new cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DensePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:245
	// ("cv::cuda::DensePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_create(Result<cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DensePyrLKOpticalFlow> ret = cv::cuda::DensePyrLKOpticalFlow::create();
			Ok(new cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DensePyrLKOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::DensePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_DensePyrLKOpticalFlow_to_Algorithm(cv::cuda::DensePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::DensePyrLKOpticalFlow::to_CUDA_DenseOpticalFlow() generated
	// ("cv::cuda::DensePyrLKOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DenseOpticalFlow* cv_cuda_DensePyrLKOpticalFlow_to_CUDA_DenseOpticalFlow(cv::cuda::DensePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::DenseOpticalFlow*>(instance);
	}

	// cv::cuda::DensePyrLKOpticalFlow::delete() generated
	// ("cv::cuda::DensePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DensePyrLKOpticalFlow_delete(cv::cuda::DensePyrLKOpticalFlow* instance) {
			delete instance;
	}

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:261
	// ("cv::cuda::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getNumLevels_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:262
	// ("cv::cuda::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setNumLevels_int(cv::cuda::FarnebackOpticalFlow* instance, int numLevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLevels(numLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:264
	// ("cv::cuda::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getPyrScale_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:265
	// ("cv::cuda::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setPyrScale_double(cv::cuda::FarnebackOpticalFlow* instance, double pyrScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPyrScale(pyrScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:267
	// ("cv::cuda::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getFastPyramids_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFastPyramids();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:268
	// ("cv::cuda::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setFastPyramids_bool(cv::cuda::FarnebackOpticalFlow* instance, bool fastPyramids, ResultVoid* ocvrs_return) {
		try {
			instance->setFastPyramids(fastPyramids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:270
	// ("cv::cuda::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getWinSize_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:271
	// ("cv::cuda::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setWinSize_int(cv::cuda::FarnebackOpticalFlow* instance, int winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:273
	// ("cv::cuda::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getNumIters_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:274
	// ("cv::cuda::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setNumIters_int(cv::cuda::FarnebackOpticalFlow* instance, int numIters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolyN()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:276
	// ("cv::cuda::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getPolyN_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:277
	// ("cv::cuda::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setPolyN_int(cv::cuda::FarnebackOpticalFlow* instance, int polyN, ResultVoid* ocvrs_return) {
		try {
			instance->setPolyN(polyN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:279
	// ("cv::cuda::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getPolySigma_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:280
	// ("cv::cuda::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setPolySigma_double(cv::cuda::FarnebackOpticalFlow* instance, double polySigma, ResultVoid* ocvrs_return) {
		try {
			instance->setPolySigma(polySigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:282
	// ("cv::cuda::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_getFlags_const(const cv::cuda::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:283
	// ("cv::cuda::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_setFlags_int(cv::cuda::FarnebackOpticalFlow* instance, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:285
	// ("cv::cuda::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
	void cv_cuda_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags, Result<cv::Ptr<cv::cuda::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::FarnebackOpticalFlow> ret = cv::cuda::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			Ok(new cv::Ptr<cv::cuda::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:285
	// ("cv::cuda::FarnebackOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_create(Result<cv::Ptr<cv::cuda::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::FarnebackOpticalFlow> ret = cv::cuda::FarnebackOpticalFlow::create();
			Ok(new cv::Ptr<cv::cuda::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FarnebackOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_FarnebackOpticalFlow_to_Algorithm(cv::cuda::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::FarnebackOpticalFlow::to_CUDA_DenseOpticalFlow() generated
	// ("cv::cuda::FarnebackOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DenseOpticalFlow* cv_cuda_FarnebackOpticalFlow_to_CUDA_DenseOpticalFlow(cv::cuda::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::DenseOpticalFlow*>(instance);
	}

	// cv::cuda::FarnebackOpticalFlow::delete() generated
	// ("cv::cuda::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FarnebackOpticalFlow_delete(cv::cuda::FarnebackOpticalFlow* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputOutputArray, Stream &, InputArray, OutputArray)(InputArray, InputArray, InputOutputArray, TraitClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:132
	// ("cv::cuda::NvidiaHWOpticalFlow::calc", vec![(pred!(mut, ["inputImage", "referenceImage", "flow", "stream", "hint", "cost"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::cuda::Stream*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR_const__InputArrayR_const__OutputArrayR(cv::cuda::NvidiaHWOpticalFlow* instance, const cv::_InputArray* inputImage, const cv::_InputArray* referenceImage, const cv::_InputOutputArray* flow, cv::cuda::Stream* stream, const cv::_InputArray* hint, const cv::_OutputArray* cost, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*inputImage, *referenceImage, *flow, *stream, *hint, *cost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaHWOpticalFlow::calc(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:132
	// ("cv::cuda::NvidiaHWOpticalFlow::calc", vec![(pred!(mut, ["inputImage", "referenceImage", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::cuda::NvidiaHWOpticalFlow* instance, const cv::_InputArray* inputImage, const cv::_InputArray* referenceImage, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*inputImage, *referenceImage, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:142
	// ("cv::cuda::NvidiaHWOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_NvidiaHWOpticalFlow_collectGarbage(cv::cuda::NvidiaHWOpticalFlow* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:146
	// ("cv::cuda::NvidiaHWOpticalFlow::getGridSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_NvidiaHWOpticalFlow_getGridSize_const(const cv::cuda::NvidiaHWOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_1_0() generated
	// ("cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_1_0", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaOpticalFlow_1_0* cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_1_0(cv::cuda::NvidiaHWOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::NvidiaOpticalFlow_1_0*>(instance);
	}

	// cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_2_0() generated
	// ("cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_2_0", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaOpticalFlow_2_0* cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_2_0(cv::cuda::NvidiaHWOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::NvidiaOpticalFlow_2_0*>(instance);
	}

	// cv::cuda::NvidiaHWOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::NvidiaHWOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_NvidiaHWOpticalFlow_to_Algorithm(cv::cuda::NvidiaHWOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::NvidiaHWOpticalFlow::delete() generated
	// ("cv::cuda::NvidiaHWOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_NvidiaHWOpticalFlow_delete(cv::cuda::NvidiaHWOpticalFlow* instance) {
			delete instance;
	}

	// upSampler(InputArray, cv::Size, int, InputOutputArray)(InputArray, SimpleClass, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:424
	// ("cv::cuda::NvidiaOpticalFlow_1_0::upSampler", vec![(pred!(mut, ["flow", "imageSize", "gridSize", "upsampledFlow"], ["const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_1_0_upSampler_const__InputArrayR_Size_int_const__InputOutputArrayR(cv::cuda::NvidiaOpticalFlow_1_0* instance, const cv::_InputArray* flow, cv::Size* imageSize, int gridSize, const cv::_InputOutputArray* upsampledFlow, ResultVoid* ocvrs_return) {
		try {
			instance->upSampler(*flow, *imageSize, gridSize, *upsampledFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(cv::Size, cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:445
	// ("cv::cuda::NvidiaOpticalFlow_1_0::create", vec![(pred!(mut, ["imageSize", "perfPreset", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_1_0_create_Size_NVIDIA_OF_PERF_LEVEL_bool_bool_bool_int_StreamR_StreamR(cv::Size* imageSize, cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL perfPreset, bool enableTemporalHints, bool enableExternalHints, bool enableCostBuffer, int gpuId, cv::cuda::Stream* inputStream, cv::cuda::Stream* outputStream, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0> ret = cv::cuda::NvidiaOpticalFlow_1_0::create(*imageSize, perfPreset, enableTemporalHints, enableExternalHints, enableCostBuffer, gpuId, *inputStream, *outputStream);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaOpticalFlow_1_0::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:445
	// ("cv::cuda::NvidiaOpticalFlow_1_0::create", vec![(pred!(mut, ["imageSize"], ["cv::Size"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_1_0_create_Size(cv::Size* imageSize, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0> ret = cv::cuda::NvidiaOpticalFlow_1_0::create(*imageSize);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_1_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaOpticalFlow_1_0::to_Algorithm() generated
	// ("cv::cuda::NvidiaOpticalFlow_1_0::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_NvidiaOpticalFlow_1_0_to_Algorithm(cv::cuda::NvidiaOpticalFlow_1_0* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::NvidiaOpticalFlow_1_0::to_CUDA_NvidiaHWOpticalFlow() generated
	// ("cv::cuda::NvidiaOpticalFlow_1_0::to_CUDA_NvidiaHWOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaHWOpticalFlow* cv_cuda_NvidiaOpticalFlow_1_0_to_CUDA_NvidiaHWOpticalFlow(cv::cuda::NvidiaOpticalFlow_1_0* instance) {
			return dynamic_cast<cv::cuda::NvidiaHWOpticalFlow*>(instance);
	}

	// cv::cuda::NvidiaOpticalFlow_1_0::delete() generated
	// ("cv::cuda::NvidiaOpticalFlow_1_0::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_NvidiaOpticalFlow_1_0_delete(cv::cuda::NvidiaOpticalFlow_1_0* instance) {
			delete instance;
	}

	// convertToFloat(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:511
	// ("cv::cuda::NvidiaOpticalFlow_2_0::convertToFloat", vec![(pred!(mut, ["flow", "floatFlow"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_convertToFloat_const__InputArrayR_const__InputOutputArrayR(cv::cuda::NvidiaOpticalFlow_2_0* instance, const cv::_InputArray* flow, const cv::_InputOutputArray* floatFlow, ResultVoid* ocvrs_return) {
		try {
			instance->convertToFloat(*flow, *floatFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(cv::Size, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, Enum, Enum, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:535
	// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "perfPreset", "outputGridSize", "hintGridSize", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_create_Size_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(cv::Size* imageSize, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL perfPreset, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE outputGridSize, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE hintGridSize, bool enableTemporalHints, bool enableExternalHints, bool enableCostBuffer, int gpuId, cv::cuda::Stream* inputStream, cv::cuda::Stream* outputStream, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0> ret = cv::cuda::NvidiaOpticalFlow_2_0::create(*imageSize, perfPreset, outputGridSize, hintGridSize, enableTemporalHints, enableExternalHints, enableCostBuffer, gpuId, *inputStream, *outputStream);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaOpticalFlow_2_0::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:535
	// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize"], ["cv::Size"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_create_Size(cv::Size* imageSize, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0> ret = cv::cuda::NvidiaOpticalFlow_2_0::create(*imageSize);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(cv::Size, std::vector<Rect>, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, CppPassByVoidPtr, Enum, Enum, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:573
	// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "roiData", "perfPreset", "outputGridSize", "hintGridSize", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "std::vector<cv::Rect>", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(cv::Size* imageSize, std::vector<cv::Rect>* roiData, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL perfPreset, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE outputGridSize, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE hintGridSize, bool enableTemporalHints, bool enableExternalHints, bool enableCostBuffer, int gpuId, cv::cuda::Stream* inputStream, cv::cuda::Stream* outputStream, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0> ret = cv::cuda::NvidiaOpticalFlow_2_0::create(*imageSize, *roiData, perfPreset, outputGridSize, hintGridSize, enableTemporalHints, enableExternalHints, enableCostBuffer, gpuId, *inputStream, *outputStream);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaOpticalFlow_2_0::create(SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:573
	// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "roiData"], ["cv::Size", "std::vector<cv::Rect>"]), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG(cv::Size* imageSize, std::vector<cv::Rect>* roiData, Result<cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0> ret = cv::cuda::NvidiaOpticalFlow_2_0::create(*imageSize, *roiData);
			Ok(new cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::NvidiaOpticalFlow_2_0::to_Algorithm() generated
	// ("cv::cuda::NvidiaOpticalFlow_2_0::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_NvidiaOpticalFlow_2_0_to_Algorithm(cv::cuda::NvidiaOpticalFlow_2_0* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::NvidiaOpticalFlow_2_0::to_CUDA_NvidiaHWOpticalFlow() generated
	// ("cv::cuda::NvidiaOpticalFlow_2_0::to_CUDA_NvidiaHWOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaHWOpticalFlow* cv_cuda_NvidiaOpticalFlow_2_0_to_CUDA_NvidiaHWOpticalFlow(cv::cuda::NvidiaOpticalFlow_2_0* instance) {
			return dynamic_cast<cv::cuda::NvidiaHWOpticalFlow*>(instance);
	}

	// cv::cuda::NvidiaOpticalFlow_2_0::delete() generated
	// ("cv::cuda::NvidiaOpticalFlow_2_0::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_NvidiaOpticalFlow_2_0_delete(cv::cuda::NvidiaOpticalFlow_2_0* instance) {
			delete instance;
	}

	// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:311
	// ("cv::cuda::OpticalFlowDual_TVL1::getTau", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getTau_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:312
	// ("cv::cuda::OpticalFlowDual_TVL1::setTau", vec![(pred!(mut, ["tau"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setTau_double(cv::cuda::OpticalFlowDual_TVL1* instance, double tau, ResultVoid* ocvrs_return) {
		try {
			instance->setTau(tau);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:320
	// ("cv::cuda::OpticalFlowDual_TVL1::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getLambda_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:321
	// ("cv::cuda::OpticalFlowDual_TVL1::setLambda", vec![(pred!(mut, ["lambda"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setLambda_double(cv::cuda::OpticalFlowDual_TVL1* instance, double lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:329
	// ("cv::cuda::OpticalFlowDual_TVL1::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getGamma_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:330
	// ("cv::cuda::OpticalFlowDual_TVL1::setGamma", vec![(pred!(mut, ["gamma"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setGamma_double(cv::cuda::OpticalFlowDual_TVL1* instance, double gamma, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:338
	// ("cv::cuda::OpticalFlowDual_TVL1::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getTheta_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:339
	// ("cv::cuda::OpticalFlowDual_TVL1::setTheta", vec![(pred!(mut, ["theta"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setTheta_double(cv::cuda::OpticalFlowDual_TVL1* instance, double theta, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(theta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumScales()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:344
	// ("cv::cuda::OpticalFlowDual_TVL1::getNumScales", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getNumScales_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumScales();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:345
	// ("cv::cuda::OpticalFlowDual_TVL1::setNumScales", vec![(pred!(mut, ["nscales"], ["int"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setNumScales_int(cv::cuda::OpticalFlowDual_TVL1* instance, int nscales, ResultVoid* ocvrs_return) {
		try {
			instance->setNumScales(nscales);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumWarps()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:353
	// ("cv::cuda::OpticalFlowDual_TVL1::getNumWarps", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getNumWarps_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumWarps();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumWarps(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:354
	// ("cv::cuda::OpticalFlowDual_TVL1::setNumWarps", vec![(pred!(mut, ["warps"], ["int"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setNumWarps_int(cv::cuda::OpticalFlowDual_TVL1* instance, int warps, ResultVoid* ocvrs_return) {
		try {
			instance->setNumWarps(warps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:360
	// ("cv::cuda::OpticalFlowDual_TVL1::getEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getEpsilon_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:361
	// ("cv::cuda::OpticalFlowDual_TVL1::setEpsilon", vec![(pred!(mut, ["epsilon"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setEpsilon_double(cv::cuda::OpticalFlowDual_TVL1* instance, double epsilon, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(epsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:366
	// ("cv::cuda::OpticalFlowDual_TVL1::getNumIterations", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getNumIterations_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:367
	// ("cv::cuda::OpticalFlowDual_TVL1::setNumIterations", vec![(pred!(mut, ["iterations"], ["int"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setNumIterations_int(cv::cuda::OpticalFlowDual_TVL1* instance, int iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIterations(iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:369
	// ("cv::cuda::OpticalFlowDual_TVL1::getScaleStep", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getScaleStep_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:370
	// ("cv::cuda::OpticalFlowDual_TVL1::setScaleStep", vec![(pred!(mut, ["scaleStep"], ["double"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setScaleStep_double(cv::cuda::OpticalFlowDual_TVL1* instance, double scaleStep, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleStep(scaleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:372
	// ("cv::cuda::OpticalFlowDual_TVL1::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_getUseInitialFlow_const(const cv::cuda::OpticalFlowDual_TVL1* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:373
	// ("cv::cuda::OpticalFlowDual_TVL1::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_setUseInitialFlow_bool(cv::cuda::OpticalFlowDual_TVL1* instance, bool useInitialFlow, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, double, double, int, int, double, int, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:375
	// ("cv::cuda::OpticalFlowDual_TVL1::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "iterations", "scaleStep", "gamma", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "double", "double", "bool"]), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_create_double_double_double_int_int_double_int_double_double_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int iterations, double scaleStep, double gamma, bool useInitialFlow, Result<cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::OpticalFlowDual_TVL1> ret = cv::cuda::OpticalFlowDual_TVL1::create(tau, lambda, theta, nscales, warps, epsilon, iterations, scaleStep, gamma, useInitialFlow);
			Ok(new cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::OpticalFlowDual_TVL1::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:375
	// ("cv::cuda::OpticalFlowDual_TVL1::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_create(Result<cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::OpticalFlowDual_TVL1> ret = cv::cuda::OpticalFlowDual_TVL1::create();
			Ok(new cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::OpticalFlowDual_TVL1::to_Algorithm() generated
	// ("cv::cuda::OpticalFlowDual_TVL1::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_OpticalFlowDual_TVL1_to_Algorithm(cv::cuda::OpticalFlowDual_TVL1* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::OpticalFlowDual_TVL1::to_CUDA_DenseOpticalFlow() generated
	// ("cv::cuda::OpticalFlowDual_TVL1::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DenseOpticalFlow* cv_cuda_OpticalFlowDual_TVL1_to_CUDA_DenseOpticalFlow(cv::cuda::OpticalFlowDual_TVL1* instance) {
			return dynamic_cast<cv::cuda::DenseOpticalFlow*>(instance);
	}

	// cv::cuda::OpticalFlowDual_TVL1::delete() generated
	// ("cv::cuda::OpticalFlowDual_TVL1::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_OpticalFlowDual_TVL1_delete(cv::cuda::OpticalFlowDual_TVL1* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Stream &)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:99
	// ("cv::cuda::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:99
	// ("cv::cuda::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(cv::cuda::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SparseOpticalFlow::to_CUDA_SparsePyrLKOpticalFlow() generated
	// ("cv::cuda::SparseOpticalFlow::to_CUDA_SparsePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::SparsePyrLKOpticalFlow* cv_cuda_SparseOpticalFlow_to_CUDA_SparsePyrLKOpticalFlow(cv::cuda::SparseOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::SparsePyrLKOpticalFlow*>(instance);
	}

	// cv::cuda::SparseOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::SparseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_SparseOpticalFlow_to_Algorithm(cv::cuda::SparseOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::SparseOpticalFlow::delete() generated
	// ("cv::cuda::SparseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SparseOpticalFlow_delete(cv::cuda::SparseOpticalFlow* instance) {
			delete instance;
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:206
	// ("cv::cuda::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_getWinSize_const(const cv::cuda::SparsePyrLKOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:207
	// ("cv::cuda::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_setWinSize_Size(cv::cuda::SparsePyrLKOpticalFlow* instance, cv::Size* winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(*winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:209
	// ("cv::cuda::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::cuda::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:210
	// ("cv::cuda::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::cuda::SparsePyrLKOpticalFlow* instance, int maxLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(maxLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:212
	// ("cv::cuda::SparsePyrLKOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_getNumIters_const(const cv::cuda::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:213
	// ("cv::cuda::SparsePyrLKOpticalFlow::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_setNumIters_int(cv::cuda::SparsePyrLKOpticalFlow* instance, int iters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(iters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:215
	// ("cv::cuda::SparsePyrLKOpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_getUseInitialFlow_const(const cv::cuda::SparsePyrLKOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:216
	// ("cv::cuda::SparsePyrLKOpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_setUseInitialFlow_bool(cv::cuda::SparsePyrLKOpticalFlow* instance, bool useInitialFlow, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(useInitialFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:218
	// ("cv::cuda::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "iters", "useInitialFlow"], ["cv::Size", "int", "int", "bool"]), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_create_Size_int_int_bool(cv::Size* winSize, int maxLevel, int iters, bool useInitialFlow, Result<cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow> ret = cv::cuda::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, iters, useInitialFlow);
			Ok(new cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:218
	// ("cv::cuda::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_create(Result<cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow> ret = cv::cuda::SparsePyrLKOpticalFlow::create();
			Ok(new cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SparsePyrLKOpticalFlow::to_Algorithm() generated
	// ("cv::cuda::SparsePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_SparsePyrLKOpticalFlow_to_Algorithm(cv::cuda::SparsePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::SparsePyrLKOpticalFlow::to_CUDA_SparseOpticalFlow() generated
	// ("cv::cuda::SparsePyrLKOpticalFlow::to_CUDA_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::cuda::SparseOpticalFlow* cv_cuda_SparsePyrLKOpticalFlow_to_CUDA_SparseOpticalFlow(cv::cuda::SparsePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::cuda::SparseOpticalFlow*>(instance);
	}

	// cv::cuda::SparsePyrLKOpticalFlow::delete() generated
	// ("cv::cuda::SparsePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SparsePyrLKOpticalFlow_delete(cv::cuda::SparsePyrLKOpticalFlow* instance) {
			delete instance;
	}

}
