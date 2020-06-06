#include "common.hpp"
#include <opencv2/optflow.hpp>
#include "optflow_types.hpp"

extern "C" {
	Result<double> cv_motempl_calcGlobalOrientation_const__InputArrayX_const__InputArrayX_const__InputArrayX_double_double(const cv::_InputArray* orientation, const cv::_InputArray* mask, const cv::_InputArray* mhi, double timestamp, double duration) {
		try {
			double ret = cv::motempl::calcGlobalOrientation(*orientation, *mask, *mhi, timestamp, duration);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_motempl_calcMotionGradient_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_double_int(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, int apertureSize) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2, apertureSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_motempl_segmentMotion_const__InputArrayX_const__OutputArrayX_vector_Rect_X_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* segmask, std::vector<cv::Rect>* boundingRects, double timestamp, double segThresh) {
		try {
			cv::motempl::segmentMotion(*mhi, *segmask, *boundingRects, timestamp, segThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_motempl_updateMotionHistory_const__InputArrayX_const__InputOutputArrayX_double_double(const cv::_InputArray* silhouette, const cv::_InputOutputArray* mhi, double timestamp, double duration) {
		try {
			cv::motempl::updateMotionHistory(*silhouette, *mhi, timestamp, duration);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, const cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement) {
		try {
			cv::optflow::calcOpticalFlowDenseRLOF(*I0, *I1, *flow, *rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSF_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSF_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int_int_double_double_int_double_double_double_int_double_double_double(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, double sigma_dist, double sigma_color, int postprocess_window, double sigma_dist_fix, double sigma_color_fix, double occ_thr, int upscale_averaging_radius, double upscale_sigma_dist, double upscale_sigma_color, double speed_up_thr) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_Ptr_RLOFOpticalFlowParameter__float(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold) {
		try {
			cv::optflow::calcOpticalFlowSparseRLOF(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *rlofParam, forwardBackwardThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int_float_bool_float_float(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int grid_step, int k, float sigma, bool use_post_proc, float fgs_lambda, float fgs_sigma) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow, grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_DeepFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DeepFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_DenseRLOF() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DenseRLOF();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*> cv_optflow_createOptFlow_DualTVL1() {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::createOptFlow_DualTVL1();
			return Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_Farneback() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_Farneback();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_PCAFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_PCAFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_SimpleFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SimpleFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::SparseOpticalFlow>*> cv_optflow_createOptFlow_SparseRLOF() {
		try {
			cv::Ptr<cv::SparseOpticalFlow> ret = cv::optflow::createOptFlow_SparseRLOF();
			return Ok(new cv::Ptr<cv::SparseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SparseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_SparseToDense() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SparseToDense();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result_void cv_read_const_FileNodeX_NodeX_Node(const cv::FileNode* fn, cv::optflow::GPCTree::Node* node, const cv::optflow::GPCTree::Node* unnamed) {
		try {
			cv::read(*fn, *node, *unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_NodeX(cv::FileStorage* fs, const char* name, const cv::optflow::GPCTree::Node* node) {
		try {
			cv::write(*fs, std::string(name), *node);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(cv::optflow::DenseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*> cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			return Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setForwardBackward(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getForwardBackward();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<cv::Size> cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			cv::Size ret = instance->getGridStep();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(cv::optflow::DenseRLOFOpticalFlow* instance, const cv::Size* val) {
		try {
			instance->setGridStep(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(cv::optflow::DenseRLOFOpticalFlow* instance, cv::optflow::InterpolationType val) {
		try {
			instance->setInterpolation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::InterpolationType> cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			cv::optflow::InterpolationType ret = instance->getInterpolation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::InterpolationType>))
	}
	
	Result<int> cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			int ret = instance->getEPICK();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val) {
		try {
			instance->setEPICK(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getEPICSigma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setEPICSigma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getEPICLambda();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setEPICLambda(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getFgsLambda();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setFgsLambda(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getFgsSigma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setFgsSigma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val) {
		try {
			instance->setUsePostProc(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			bool ret = instance->getUsePostProc();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val) {
		try {
			instance->setUseVariationalRefinement(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val) {
		try {
			instance->setRICSPSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			int ret = instance->getRICSPSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val) {
		try {
			instance->setRICSLICType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(const cv::optflow::DenseRLOFOpticalFlow* instance) {
		try {
			int ret = instance->getRICSLICType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*> cv_optflow_DenseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, const cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement) {
		try {
			cv::Ptr<cv::optflow::DenseRLOFOpticalFlow> ret = cv::optflow::DenseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			return Ok(new cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getTau_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTau();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setTau_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTau(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getLambda_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getLambda();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setLambda_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setLambda(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getTheta_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getTheta();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setTheta_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setTheta(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getGamma_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setGamma_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setGamma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getScalesNumber();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setScalesNumber(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getWarpingsNumber();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setWarpingsNumber(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getEpsilon();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setEpsilon(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getInnerIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setInnerIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getOuterIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setOuterIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::optflow::DualTVL1OpticalFlow* instance, bool val) {
		try {
			instance->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			double ret = instance->getScaleStep();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(cv::optflow::DualTVL1OpticalFlow* instance, double val) {
		try {
			instance->setScaleStep(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(const cv::optflow::DualTVL1OpticalFlow* instance) {
		try {
			int ret = instance->getMedianFiltering();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(cv::optflow::DualTVL1OpticalFlow* instance, int val) {
		try {
			instance->setMedianFiltering(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*> cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			return Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>))
	}
	
	void cv_GPCDetails_delete(cv::optflow::GPCDetails* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_X_const_GPCMatchingParamsX_int(const cv::Mat* imgCh, std::vector<cv::optflow::GPCPatchDescriptor>* descr, const cv::optflow::GPCMatchingParams* mp, int type) {
		try {
			cv::optflow::GPCDetails::getAllDescriptorsForImage(imgCh, *descr, *mp, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(size_t index, const cv::Size* sz, int* x, int* y) {
		try {
			cv::optflow::GPCDetails::getCoordinatesFromIndex(index, *sz, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCMatchingParams> cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(bool _useOpenCL) {
		try {
			cv::optflow::GPCMatchingParams ret(_useOpenCL);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	Result<cv::optflow::GPCMatchingParams> cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsX(const cv::optflow::GPCMatchingParams* params) {
		try {
			cv::optflow::GPCMatchingParams ret(*params);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	Result<cv::Vec<double, 18>> cv_optflow_GPCPatchDescriptor_getPropFeature_const(const cv::optflow::GPCPatchDescriptor* instance) {
		try {
			cv::Vec<double, 18> ret = instance->feature;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec<double, 18>>))
	}
	
	Result_void cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* val) {
		try {
			instance->feature = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GPCPatchDescriptor_delete(cv::optflow::GPCPatchDescriptor* instance) {
		delete instance;
	}
	Result<double> cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_X(const cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* coef) {
		try {
			double ret = instance->dot(*coef);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_GPCPatchDescriptor_markAsSeparated(cv::optflow::GPCPatchDescriptor* instance) {
		try {
			instance->markAsSeparated();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_GPCPatchDescriptor_isSeparated_const(const cv::optflow::GPCPatchDescriptor* instance) {
		try {
			bool ret = instance->isSeparated();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropRef(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->ref;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->ref = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropPos(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->pos;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->pos = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropNeg(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->neg;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->neg = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GPCPatchSample_delete(cv::optflow::GPCPatchSample* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_X_double(const cv::optflow::GPCPatchSample* instance, bool* refdir, bool* posdir, bool* negdir, const cv::Vec<double, 18>* coef, double rhs) {
		try {
			instance->getDirections(*refdir, *posdir, *negdir, *coef, rhs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCTrainingParams> cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(unsigned int _maxTreeDepth, int _minNumberOfSamples, cv::optflow::GPCDescType _descriptorType, bool _printProgress) {
		try {
			cv::optflow::GPCTrainingParams ret(_maxTreeDepth, _minNumberOfSamples, _descriptorType, _printProgress);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCTrainingParams>))
	}
	
	Result<bool> cv_optflow_GPCTrainingParams_check_const(const cv::optflow::GPCTrainingParams instance) {
		try {
			bool ret = instance.check();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_GPCTrainingSamples_delete(cv::optflow::GPCTrainingSamples* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*> cv_optflow_GPCTrainingSamples_create_const_vector_String_X_const_vector_String_X_const_vector_String_X_int(const std::vector<cv::String>* imagesFrom, const std::vector<cv::String>* imagesTo, const std::vector<cv::String>* gt, int descriptorType) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			return Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*> cv_optflow_GPCTrainingSamples_create_const__InputArrayX_const__InputArrayX_const__InputArrayX_int(const cv::_InputArray* imagesFrom, const cv::_InputArray* imagesTo, const cv::_InputArray* gt, int descriptorType) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			return Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	Result<size_t> cv_optflow_GPCTrainingSamples_size_const(const cv::optflow::GPCTrainingSamples* instance) {
		try {
			size_t ret = instance->size();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_optflow_GPCTrainingSamples_type_const(const cv::optflow::GPCTrainingSamples* instance) {
		try {
			int ret = instance->type();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_GPCTree_delete(cv::optflow::GPCTree* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCTree_train_GPCTrainingSamplesX_GPCTrainingParams(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, const cv::optflow::GPCTrainingParams* params) {
		try {
			instance->train(*samples, *params);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCTree_write_const_FileStorageX(const cv::optflow::GPCTree* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCTree_read_const_FileNodeX(cv::optflow::GPCTree* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorX(const cv::optflow::GPCTree* instance, const cv::optflow::GPCPatchDescriptor* descr) {
		try {
			unsigned int ret = instance->findLeafForPatch(*descr);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<cv::Ptr<cv::optflow::GPCTree>*> cv_optflow_GPCTree_create() {
		try {
			cv::Ptr<cv::optflow::GPCTree> ret = cv::optflow::GPCTree::create();
			return Ok(new cv::Ptr<cv::optflow::GPCTree>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTree>*>))
	}
	
	Result<int> cv_optflow_GPCTree_getDescriptorType_const(const cv::optflow::GPCTree* instance) {
		try {
			int ret = instance->getDescriptorType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_OpticalFlowPCAFlow_delete(cv::optflow::OpticalFlowPCAFlow* instance) {
		delete instance;
	}
	Result<cv::optflow::OpticalFlowPCAFlow*> cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_PCAPrior__Size_float_float_float_float_float(const cv::Ptr<cv::optflow::PCAPrior>* _prior, const cv::Size* _basisSize, float _sparseRate, float _retainedCornersFraction, float _occlusionsThreshold, float _dampingFactor, float _claheClip) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow(*_prior, *_basisSize, _sparseRate, _retainedCornersFraction, _occlusionsThreshold, _dampingFactor, _claheClip);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::OpticalFlowPCAFlow*>))
	}
	
	Result_void cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(cv::optflow::OpticalFlowPCAFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow) {
		try {
			instance->calc(*I0, *I1, *flow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_OpticalFlowPCAFlow_collectGarbage(cv::optflow::OpticalFlowPCAFlow* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PCAPrior_delete(cv::optflow::PCAPrior* instance) {
		delete instance;
	}
	Result<cv::optflow::PCAPrior*> cv_optflow_PCAPrior_PCAPrior_const_charX(const char* pathToPrior) {
		try {
			cv::optflow::PCAPrior* ret = new cv::optflow::PCAPrior(pathToPrior);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::PCAPrior*>))
	}
	
	Result<int> cv_optflow_PCAPrior_getPadding_const(const cv::optflow::PCAPrior* instance) {
		try {
			int ret = instance->getPadding();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_optflow_PCAPrior_getBasisSize_const(const cv::optflow::PCAPrior* instance) {
		try {
			int ret = instance->getBasisSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(const cv::optflow::PCAPrior* instance, float* A1, float* A2, float* b1, float* b2) {
		try {
			instance->fillConstraints(A1, A2, b1, b2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::SolverType> cv_optflow_RLOFOpticalFlowParameter_getPropSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			cv::optflow::SolverType ret = instance->solverType;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SolverType>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropSolverType_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType val) {
		try {
			instance->solverType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::SupportRegionType> cv_optflow_RLOFOpticalFlowParameter_getPropSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			cv::optflow::SupportRegionType ret = instance->supportRegionType;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SupportRegionType>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropSupportRegionType_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType val) {
		try {
			instance->supportRegionType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->normSigma0;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma0_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->normSigma0 = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->normSigma1;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma1_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->normSigma1 = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getPropSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->smallWinSize;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropSmallWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->smallWinSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getPropLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->largeWinSize;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropLargeWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->largeWinSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getPropCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->crossSegmentationThreshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropCrossSegmentationThreshold_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->crossSegmentationThreshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getPropMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->maxLevel;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropMaxLevel_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->maxLevel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getPropUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->useInitialFlow;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropUseInitialFlow_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->useInitialFlow = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getPropUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->useIlluminationModel;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropUseIlluminationModel_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->useIlluminationModel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getPropUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->useGlobalMotionPrior;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropUseGlobalMotionPrior_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->useGlobalMotionPrior = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getPropMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->maxIteration;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropMaxIteration_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->maxIteration = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getPropMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->minEigenValue;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropMinEigenValue_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->minEigenValue = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getPropGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->globalMotionRansacThreshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setPropGlobalMotionRansacThreshold_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->globalMotionRansacThreshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RLOFOpticalFlowParameter_delete(cv::optflow::RLOFOpticalFlowParameter* instance) {
		delete instance;
	}
	Result<cv::optflow::RLOFOpticalFlowParameter*> cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter() {
		try {
			cv::optflow::RLOFOpticalFlowParameter* ret = new cv::optflow::RLOFOpticalFlowParameter();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::RLOFOpticalFlowParameter*>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->setUseMEstimator(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType val) {
		try {
			instance->setSolverType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::SolverType> cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			cv::optflow::SolverType ret = instance->getSolverType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SolverType>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType val) {
		try {
			instance->setSupportRegionType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::SupportRegionType> cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			cv::optflow::SupportRegionType ret = instance->getSupportRegionType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SupportRegionType>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->setNormSigma0(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->getNormSigma0();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->setNormSigma1(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->getNormSigma1();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->setSmallWinSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->getSmallWinSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->setLargeWinSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->getLargeWinSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->setCrossSegmentationThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->getCrossSegmentationThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->setMaxLevel(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->getMaxLevel();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->setUseInitialFlow(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->getUseInitialFlow();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->setUseIlluminationModel(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->getUseIlluminationModel();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
		try {
			instance->setUseGlobalMotionPrior(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			bool ret = instance->getUseGlobalMotionPrior();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
		try {
			instance->setMaxIteration(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			int ret = instance->getMaxIteration();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->setMinEigenValue(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->getMinEigenValue();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
		try {
			instance->setGlobalMotionRansacThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
		try {
			float ret = instance->getGlobalMotionRansacThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*> cv_optflow_RLOFOpticalFlowParameter_create() {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = cv::optflow::RLOFOpticalFlowParameter::create();
			return Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	Result_void cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(cv::optflow::SparseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*> cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::SparseRLOFOpticalFlow* instance) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			return Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	Result_void cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::SparseRLOFOpticalFlow* instance, float val) {
		try {
			instance->setForwardBackward(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::SparseRLOFOpticalFlow* instance) {
		try {
			float ret = instance->getForwardBackward();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*> cv_optflow_SparseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold) {
		try {
			cv::Ptr<cv::optflow::SparseRLOFOpticalFlow> ret = cv::optflow::SparseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold);
			return Ok(new cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>))
	}
	
}
