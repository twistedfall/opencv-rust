#include "ocvrs_common.hpp"
#include <opencv2/superres.hpp>
#include "superres_types.hpp"

extern "C" {
	// cv::superres::createFrameSource_Camera() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:80
	// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createFrameSource_Camera(Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Camera();
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFrameSource_Camera(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:80
	// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, ["deviceId"], ["int"]), _)]),
	void cv_superres_createFrameSource_Camera_int(int deviceId, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Camera(deviceId);
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFrameSource_Empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:75
	// ("cv::superres::createFrameSource_Empty", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createFrameSource_Empty(Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Empty();
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFrameSource_Video_CUDA(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:78
	// ("cv::superres::createFrameSource_Video_CUDA", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	void cv_superres_createFrameSource_Video_CUDA_const_StringR(const char* fileName, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video_CUDA(std::string(fileName));
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFrameSource_Video(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:77
	// ("cv::superres::createFrameSource_Video", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	void cv_superres_createFrameSource_Video_const_StringR(const char* fileName, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video(std::string(fileName));
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_Brox_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:177
	// ("cv::superres::createOptFlow_Brox_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_Brox_CUDA(Result<cv::Ptr<cv::superres::BroxOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::BroxOpticalFlow> ret = cv::superres::createOptFlow_Brox_CUDA();
			Ok(new cv::Ptr<cv::superres::BroxOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:139
	// ("cv::superres::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_DualTVL1(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1();
			Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DualTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:140
	// ("cv::superres::createOptFlow_DualTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_DualTVL1_CUDA(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1_CUDA();
			Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:96
	// ("cv::superres::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_Farneback(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback();
			Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_Farneback_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:97
	// ("cv::superres::createOptFlow_Farneback_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_Farneback_CUDA(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback_CUDA();
			Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_PyrLK_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:196
	// ("cv::superres::createOptFlow_PyrLK_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createOptFlow_PyrLK_CUDA(Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::PyrLKOpticalFlow> ret = cv::superres::createOptFlow_PyrLK_CUDA();
			Ok(new cv::Ptr<cv::superres::PyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSuperResolution_BTVL1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:199
	// ("cv::superres::createSuperResolution_BTVL1", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createSuperResolution_BTVL1(Result<cv::Ptr<cv::superres::SuperResolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1();
			Ok(new cv::Ptr<cv::superres::SuperResolution>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSuperResolution_BTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:200
	// ("cv::superres::createSuperResolution_BTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_superres_createSuperResolution_BTVL1_CUDA(Result<cv::Ptr<cv::superres::SuperResolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1_CUDA();
			Ok(new cv::Ptr<cv::superres::SuperResolution>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:148
	// ("cv::superres::BroxOpticalFlow::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getAlpha_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:150
	// ("cv::superres::BroxOpticalFlow::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_BroxOpticalFlow_setAlpha_double(cv::superres::BroxOpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:153
	// ("cv::superres::BroxOpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getGamma_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:155
	// ("cv::superres::BroxOpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_BroxOpticalFlow_setGamma_double(cv::superres::BroxOpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:158
	// ("cv::superres::BroxOpticalFlow::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getScaleFactor_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:160
	// ("cv::superres::BroxOpticalFlow::setScaleFactor", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_BroxOpticalFlow_setScaleFactor_double(cv::superres::BroxOpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:163
	// ("cv::superres::BroxOpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getInnerIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:165
	// ("cv::superres::BroxOpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_BroxOpticalFlow_setInnerIterations_int(cv::superres::BroxOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setInnerIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:168
	// ("cv::superres::BroxOpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getOuterIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:170
	// ("cv::superres::BroxOpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_BroxOpticalFlow_setOuterIterations_int(cv::superres::BroxOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setOuterIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSolverIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:173
	// ("cv::superres::BroxOpticalFlow::getSolverIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_BroxOpticalFlow_getSolverIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSolverIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSolverIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:175
	// ("cv::superres::BroxOpticalFlow::setSolverIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_BroxOpticalFlow_setSolverIterations_int(cv::superres::BroxOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setSolverIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::BroxOpticalFlow::to_Algorithm() generated
	// ("cv::superres::BroxOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_BroxOpticalFlow_to_Algorithm(cv::superres::BroxOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::BroxOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
	// ("cv::superres::BroxOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::superres::DenseOpticalFlowExt* cv_superres_BroxOpticalFlow_to_SuperRes_DenseOpticalFlowExt(cv::superres::BroxOpticalFlow* instance) {
			return dynamic_cast<cv::superres::DenseOpticalFlowExt*>(instance);
	}

	// cv::superres::BroxOpticalFlow::delete() generated
	// ("cv::superres::BroxOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_BroxOpticalFlow_delete(cv::superres::BroxOpticalFlow* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:59
	// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1", "flow2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::superres::DenseOpticalFlowExt* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_OutputArray* flow1, const cv::_OutputArray* flow2, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*frame0, *frame1, *flow1, *flow2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::DenseOpticalFlowExt::calc(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:59
	// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::superres::DenseOpticalFlowExt* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_OutputArray* flow1, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*frame0, *frame1, *flow1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:60
	// ("cv::superres::DenseOpticalFlowExt::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_superres_DenseOpticalFlowExt_collectGarbage(cv::superres::DenseOpticalFlowExt* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::DenseOpticalFlowExt::to_SuperRes_BroxOpticalFlow() generated
	// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_BroxOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::superres::BroxOpticalFlow* cv_superres_DenseOpticalFlowExt_to_SuperRes_BroxOpticalFlow(cv::superres::DenseOpticalFlowExt* instance) {
			return dynamic_cast<cv::superres::BroxOpticalFlow*>(instance);
	}

	// cv::superres::DenseOpticalFlowExt::to_SuperRes_DualTVL1OpticalFlow() generated
	// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_DualTVL1OpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::superres::DualTVL1OpticalFlow* cv_superres_DenseOpticalFlowExt_to_SuperRes_DualTVL1OpticalFlow(cv::superres::DenseOpticalFlowExt* instance) {
			return dynamic_cast<cv::superres::DualTVL1OpticalFlow*>(instance);
	}

	// cv::superres::DenseOpticalFlowExt::to_SuperRes_FarnebackOpticalFlow() generated
	// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::superres::FarnebackOpticalFlow* cv_superres_DenseOpticalFlowExt_to_SuperRes_FarnebackOpticalFlow(cv::superres::DenseOpticalFlowExt* instance) {
			return dynamic_cast<cv::superres::FarnebackOpticalFlow*>(instance);
	}

	// cv::superres::DenseOpticalFlowExt::to_SuperRes_PyrLKOpticalFlow() generated
	// ("cv::superres::DenseOpticalFlowExt::to_SuperRes_PyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::superres::PyrLKOpticalFlow* cv_superres_DenseOpticalFlowExt_to_SuperRes_PyrLKOpticalFlow(cv::superres::DenseOpticalFlowExt* instance) {
			return dynamic_cast<cv::superres::PyrLKOpticalFlow*>(instance);
	}

	// cv::superres::DenseOpticalFlowExt::to_Algorithm() generated
	// ("cv::superres::DenseOpticalFlowExt::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_DenseOpticalFlowExt_to_Algorithm(cv::superres::DenseOpticalFlowExt* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::DenseOpticalFlowExt::delete() generated
	// ("cv::superres::DenseOpticalFlowExt::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_DenseOpticalFlowExt_delete(cv::superres::DenseOpticalFlowExt* instance) {
			delete instance;
	}

	// getTau()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:107
	// ("cv::superres::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getTau_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:109
	// ("cv::superres::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setTau_double(cv::superres::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:111
	// ("cv::superres::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getLambda_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:113
	// ("cv::superres::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setLambda_double(cv::superres::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:115
	// ("cv::superres::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getTheta_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:117
	// ("cv::superres::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setTheta_double(cv::superres::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:119
	// ("cv::superres::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScalesNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:121
	// ("cv::superres::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setScalesNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:123
	// ("cv::superres::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWarpingsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:125
	// ("cv::superres::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWarpingsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:127
	// ("cv::superres::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getEpsilon_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:129
	// ("cv::superres::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setEpsilon_double(cv::superres::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:131
	// ("cv::superres::DualTVL1OpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getIterations_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:133
	// ("cv::superres::DualTVL1OpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setIterations_int(cv::superres::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:135
	// ("cv::superres::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:137
	// ("cv::superres::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::superres::DualTVL1OpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::DualTVL1OpticalFlow::to_Algorithm() generated
	// ("cv::superres::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_DualTVL1OpticalFlow_to_Algorithm(cv::superres::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::DualTVL1OpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
	// ("cv::superres::DualTVL1OpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::superres::DenseOpticalFlowExt* cv_superres_DualTVL1OpticalFlow_to_SuperRes_DenseOpticalFlowExt(cv::superres::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::superres::DenseOpticalFlowExt*>(instance);
	}

	// cv::superres::DualTVL1OpticalFlow::delete() generated
	// ("cv::superres::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_DualTVL1OpticalFlow_delete(cv::superres::DualTVL1OpticalFlow* instance) {
			delete instance;
	}

	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:68
	// ("cv::superres::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getPyrScale_const(const cv::superres::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:70
	// ("cv::superres::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setPyrScale_double(cv::superres::FarnebackOpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setPyrScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLevelsNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:72
	// ("cv::superres::FarnebackOpticalFlow::getLevelsNumber", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevelsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLevelsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:74
	// ("cv::superres::FarnebackOpticalFlow::setLevelsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(cv::superres::FarnebackOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setLevelsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:76
	// ("cv::superres::FarnebackOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getWindowSize_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:78
	// ("cv::superres::FarnebackOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setWindowSize_int(cv::superres::FarnebackOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:80
	// ("cv::superres::FarnebackOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getIterations_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:82
	// ("cv::superres::FarnebackOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setIterations_int(cv::superres::FarnebackOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolyN()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:84
	// ("cv::superres::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getPolyN_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:86
	// ("cv::superres::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setPolyN_int(cv::superres::FarnebackOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPolyN(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:88
	// ("cv::superres::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getPolySigma_const(const cv::superres::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:90
	// ("cv::superres::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setPolySigma_double(cv::superres::FarnebackOpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setPolySigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:92
	// ("cv::superres::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_getFlags_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:94
	// ("cv::superres::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_FarnebackOpticalFlow_setFlags_int(cv::superres::FarnebackOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::FarnebackOpticalFlow::to_Algorithm() generated
	// ("cv::superres::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_FarnebackOpticalFlow_to_Algorithm(cv::superres::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::FarnebackOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
	// ("cv::superres::FarnebackOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::superres::DenseOpticalFlowExt* cv_superres_FarnebackOpticalFlow_to_SuperRes_DenseOpticalFlowExt(cv::superres::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::superres::DenseOpticalFlowExt*>(instance);
	}

	// cv::superres::FarnebackOpticalFlow::delete() generated
	// ("cv::superres::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_FarnebackOpticalFlow_delete(cv::superres::FarnebackOpticalFlow* instance) {
			delete instance;
	}

	// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:71
	// ("cv::superres::FrameSource::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	void cv_superres_FrameSource_nextFrame_const__OutputArrayR(cv::superres::FrameSource* instance, const cv::_OutputArray* frame, ResultVoid* ocvrs_return) {
		try {
			instance->nextFrame(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:72
	// ("cv::superres::FrameSource::reset", vec![(pred!(mut, [], []), _)]),
	void cv_superres_FrameSource_reset(cv::superres::FrameSource* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::FrameSource::to_SuperRes_SuperResolution() generated
	// ("cv::superres::FrameSource::to_SuperRes_SuperResolution", vec![(pred!(mut, [], []), _)]),
	cv::superres::SuperResolution* cv_superres_FrameSource_to_SuperRes_SuperResolution(cv::superres::FrameSource* instance) {
			return dynamic_cast<cv::superres::SuperResolution*>(instance);
	}

	// cv::superres::FrameSource::delete() generated
	// ("cv::superres::FrameSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_FrameSource_delete(cv::superres::FrameSource* instance) {
			delete instance;
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:184
	// ("cv::superres::PyrLKOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_superres_PyrLKOpticalFlow_getWindowSize_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:186
	// ("cv::superres::PyrLKOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_PyrLKOpticalFlow_setWindowSize_int(cv::superres::PyrLKOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:188
	// ("cv::superres::PyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_superres_PyrLKOpticalFlow_getMaxLevel_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:190
	// ("cv::superres::PyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_PyrLKOpticalFlow_setMaxLevel_int(cv::superres::PyrLKOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:192
	// ("cv::superres::PyrLKOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_PyrLKOpticalFlow_getIterations_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres/optical_flow.hpp:194
	// ("cv::superres::PyrLKOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_PyrLKOpticalFlow_setIterations_int(cv::superres::PyrLKOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::PyrLKOpticalFlow::to_Algorithm() generated
	// ("cv::superres::PyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_PyrLKOpticalFlow_to_Algorithm(cv::superres::PyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::PyrLKOpticalFlow::to_SuperRes_DenseOpticalFlowExt() generated
	// ("cv::superres::PyrLKOpticalFlow::to_SuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::superres::DenseOpticalFlowExt* cv_superres_PyrLKOpticalFlow_to_SuperRes_DenseOpticalFlowExt(cv::superres::PyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::superres::DenseOpticalFlowExt*>(instance);
	}

	// cv::superres::PyrLKOpticalFlow::delete() generated
	// ("cv::superres::PyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_PyrLKOpticalFlow_delete(cv::superres::PyrLKOpticalFlow* instance) {
			delete instance;
	}

	// setInput(const Ptr<FrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:94
	// ("cv::superres::SuperResolution::setInput", vec![(pred!(mut, ["frameSource"], ["const cv::Ptr<cv::superres::FrameSource>*"]), _)]),
	void cv_superres_SuperResolution_setInput_const_PtrLFrameSourceGR(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::FrameSource>* frameSource, ResultVoid* ocvrs_return) {
		try {
			instance->setInput(*frameSource);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:100
	// ("cv::superres::SuperResolution::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	void cv_superres_SuperResolution_nextFrame_const__OutputArrayR(cv::superres::SuperResolution* instance, const cv::_OutputArray* frame, ResultVoid* ocvrs_return) {
		try {
			instance->nextFrame(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:101
	// ("cv::superres::SuperResolution::reset", vec![(pred!(mut, [], []), _)]),
	void cv_superres_SuperResolution_reset(cv::superres::SuperResolution* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:105
	// ("cv::superres::SuperResolution::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_superres_SuperResolution_collectGarbage(cv::superres::SuperResolution* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:109
	// ("cv::superres::SuperResolution::getScale", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getScale_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:111
	// ("cv::superres::SuperResolution::setScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_SuperResolution_setScale_int(cv::superres::SuperResolution* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:115
	// ("cv::superres::SuperResolution::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getIterations_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:117
	// ("cv::superres::SuperResolution::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_SuperResolution_setIterations_int(cv::superres::SuperResolution* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTau()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:121
	// ("cv::superres::SuperResolution::getTau", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getTau_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:123
	// ("cv::superres::SuperResolution::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_SuperResolution_setTau_double(cv::superres::SuperResolution* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:127
	// ("cv::superres::SuperResolution::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getLambda_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:129
	// ("cv::superres::SuperResolution::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_SuperResolution_setLambda_double(cv::superres::SuperResolution* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:133
	// ("cv::superres::SuperResolution::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getAlpha_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:135
	// ("cv::superres::SuperResolution::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_SuperResolution_setAlpha_double(cv::superres::SuperResolution* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getKernelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:139
	// ("cv::superres::SuperResolution::getKernelSize", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getKernelSize_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:141
	// ("cv::superres::SuperResolution::setKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_SuperResolution_setKernelSize_int(cv::superres::SuperResolution* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setKernelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlurKernelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:145
	// ("cv::superres::SuperResolution::getBlurKernelSize", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getBlurKernelSize_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlurKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlurKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:147
	// ("cv::superres::SuperResolution::setBlurKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_SuperResolution_setBlurKernelSize_int(cv::superres::SuperResolution* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBlurKernelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlurSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:151
	// ("cv::superres::SuperResolution::getBlurSigma", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getBlurSigma_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBlurSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlurSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:153
	// ("cv::superres::SuperResolution::setBlurSigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_superres_SuperResolution_setBlurSigma_double(cv::superres::SuperResolution* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setBlurSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTemporalAreaRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:157
	// ("cv::superres::SuperResolution::getTemporalAreaRadius", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getTemporalAreaRadius_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTemporalAreaRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTemporalAreaRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:159
	// ("cv::superres::SuperResolution::setTemporalAreaRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_superres_SuperResolution_setTemporalAreaRadius_int(cv::superres::SuperResolution* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTemporalAreaRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOpticalFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:163
	// ("cv::superres::SuperResolution::getOpticalFlow", vec![(pred!(const, [], []), _)]),
	void cv_superres_SuperResolution_getOpticalFlow_const(const cv::superres::SuperResolution* instance, Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DenseOpticalFlowExt> ret = instance->getOpticalFlow();
			Ok(new cv::Ptr<cv::superres::DenseOpticalFlowExt>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/superres.hpp:165
	// ("cv::superres::SuperResolution::setOpticalFlow", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::superres::DenseOpticalFlowExt>*"]), _)]),
	void cv_superres_SuperResolution_setOpticalFlow_const_PtrLDenseOpticalFlowExtGR(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::DenseOpticalFlowExt>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setOpticalFlow(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::superres::SuperResolution::to_Algorithm() generated
	// ("cv::superres::SuperResolution::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_superres_SuperResolution_to_Algorithm(cv::superres::SuperResolution* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::superres::SuperResolution::to_SuperRes_FrameSource() generated
	// ("cv::superres::SuperResolution::to_SuperRes_FrameSource", vec![(pred!(mut, [], []), _)]),
	cv::superres::FrameSource* cv_superres_SuperResolution_to_SuperRes_FrameSource(cv::superres::SuperResolution* instance) {
			return dynamic_cast<cv::superres::FrameSource*>(instance);
	}

	// cv::superres::SuperResolution::delete() generated
	// ("cv::superres::SuperResolution::delete", vec![(pred!(mut, [], []), _)]),
	void cv_superres_SuperResolution_delete(cv::superres::SuperResolution* instance) {
			delete instance;
	}

}
