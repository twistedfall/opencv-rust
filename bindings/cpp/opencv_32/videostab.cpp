#include "common.hpp"
#include <opencv2/videostab.hpp>
#include "videostab_types.hpp"

extern "C" {
	Result<float> cv_videostab_calcBlurriness_const_MatX(const cv::Mat* frame) {
		try {
			float ret = cv::videostab::calcBlurriness(*frame);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_calcFlowMask_const_MatX_const_MatX_const_MatX_float_const_MatX_const_MatX_MatX(const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* errors, float maxError, const cv::Mat* mask0, const cv::Mat* mask1, cv::Mat* flowMask) {
		try {
			cv::videostab::calcFlowMask(*flowX, *flowY, *errors, maxError, *mask0, *mask1, *flowMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_completeFrameAccordingToFlow_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_float_MatX_MatX(const cv::Mat* flowMask, const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* frame1, const cv::Mat* mask1, float distThresh, cv::Mat* frame0, cv::Mat* mask0) {
		try {
			cv::videostab::completeFrameAccordingToFlow(*flowMask, *flowX, *flowY, *frame1, *mask1, distThresh, *frame0, *mask0);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_ensureInclusionConstraint_const_MatX_Size_float(const cv::Mat* M, const cv::Size* size, float trimRatio) {
		try {
			cv::Mat ret = cv::videostab::ensureInclusionConstraint(*M, *size, trimRatio);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayX_const__InputOutputArrayX_int_floatX(const cv::_InputOutputArray* points0, const cv::_InputOutputArray* points1, int model, float* rmse) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionLeastSquares(*points0, *points1, model, rmse);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_videostab_estimateGlobalMotionRansac_const__InputArrayX_const__InputArrayX_int_const_RansacParamsX_floatX_intX(const cv::_InputArray* points0, const cv::_InputArray* points1, int model, const cv::videostab::RansacParams* params, float* rmse, int* ninliers) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionRansac(*points0, *points1, model, *params, rmse, ninliers);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<float> cv_videostab_estimateOptimalTrimRatio_const_MatX_Size(const cv::Mat* M, const cv::Size* size) {
		try {
			float ret = cv::videostab::estimateOptimalTrimRatio(*M, *size);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Mat*> cv_videostab_getMotion_int_int_const_vector_Mat_X(int from, int to, const std::vector<cv::Mat>* motions) {
		try {
			cv::Mat ret = cv::videostab::getMotion(from, to, *motions);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_ColorAverageInpainter_delete(cv::videostab::ColorAverageInpainter* instance) {
		delete instance;
	}
	Result_void cv_videostab_ColorAverageInpainter_inpaint_int_MatX_MatX(cv::videostab::ColorAverageInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ColorInpainter_delete(cv::videostab::ColorInpainter* instance) {
		delete instance;
	}
	Result<cv::videostab::ColorInpainter*> cv_videostab_ColorInpainter_ColorInpainter_int_double(int method, double radius) {
		try {
			cv::videostab::ColorInpainter* ret = new cv::videostab::ColorInpainter(method, radius);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::ColorInpainter*>)
	}
	
	Result_void cv_videostab_ColorInpainter_inpaint_int_MatX_MatX(cv::videostab::ColorInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ConsistentMosaicInpainter_delete(cv::videostab::ConsistentMosaicInpainter* instance) {
		delete instance;
	}
	Result<cv::videostab::ConsistentMosaicInpainter*> cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter() {
		try {
			cv::videostab::ConsistentMosaicInpainter* ret = new cv::videostab::ConsistentMosaicInpainter();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::ConsistentMosaicInpainter*>)
	}
	
	Result_void cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(cv::videostab::ConsistentMosaicInpainter* instance, float val) {
		try {
			instance->setStdevThresh(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(const cv::videostab::ConsistentMosaicInpainter* instance) {
		try {
			float ret = instance->stdevThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatX_MatX(cv::videostab::ConsistentMosaicInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_DeblurerBase_setRadius_int(cv::videostab::DeblurerBase* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_DeblurerBase_radius_const(const cv::videostab::DeblurerBase* instance) {
		try {
			int ret = instance->radius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_DeblurerBase_deblur_int_MatX(cv::videostab::DeblurerBase* instance, int idx, cv::Mat* frame) {
		try {
			instance->deblur(idx, *frame);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_DeblurerBase_setFrames_const_vector_Mat_X(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setFrames(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_DeblurerBase_frames_const(const cv::videostab::DeblurerBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->frames();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_DeblurerBase_setMotions_const_vector_Mat_X(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_DeblurerBase_motions_const(const cv::videostab::DeblurerBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->motions();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_DeblurerBase_setBlurrinessRates_const_vector_float_X(cv::videostab::DeblurerBase* instance, const std::vector<float>* val) {
		try {
			instance->setBlurrinessRates(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<float>*> cv_videostab_DeblurerBase_blurrinessRates_const(const cv::videostab::DeblurerBase* instance) {
		try {
			std::vector<float> ret = instance->blurrinessRates();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	void cv_FastMarchingMethod_delete(cv::videostab::FastMarchingMethod* instance) {
		delete instance;
	}
	Result<cv::videostab::FastMarchingMethod*> cv_videostab_FastMarchingMethod_FastMarchingMethod() {
		try {
			cv::videostab::FastMarchingMethod* ret = new cv::videostab::FastMarchingMethod();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::FastMarchingMethod*>)
	}
	
	Result<cv::Mat*> cv_videostab_FastMarchingMethod_distanceMap_const(const cv::videostab::FastMarchingMethod* instance) {
		try {
			cv::Mat ret = instance->distanceMap();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_FromFileMotionReader_delete(cv::videostab::FromFileMotionReader* instance) {
		delete instance;
	}
	Result<cv::videostab::FromFileMotionReader*> cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringX(const char* path) {
		try {
			cv::videostab::FromFileMotionReader* ret = new cv::videostab::FromFileMotionReader(cv::String(path));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::FromFileMotionReader*>)
	}
	
	Result<cv::Mat*> cv_videostab_FromFileMotionReader_estimate_const_MatX_const_MatX_boolX(cv::videostab::FromFileMotionReader* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_GaussianMotionFilter_delete(cv::videostab::GaussianMotionFilter* instance) {
		delete instance;
	}
	Result<cv::videostab::GaussianMotionFilter*> cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(int radius, float stdev) {
		try {
			cv::videostab::GaussianMotionFilter* ret = new cv::videostab::GaussianMotionFilter(radius, stdev);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::GaussianMotionFilter*>)
	}
	
	Result_void cv_videostab_GaussianMotionFilter_setParams_int_float(cv::videostab::GaussianMotionFilter* instance, int radius, float stdev) {
		try {
			instance->setParams(radius, stdev);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_GaussianMotionFilter_radius_const(const cv::videostab::GaussianMotionFilter* instance) {
		try {
			int ret = instance->radius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_videostab_GaussianMotionFilter_stdev_const(const cv::videostab::GaussianMotionFilter* instance) {
		try {
			float ret = instance->stdev();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX(cv::videostab::IDenseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputOutputArray* flowX, const cv::_InputOutputArray* flowY, const cv::_OutputArray* errors) {
		try {
			instance->run(*frame0, *frame1, *flowX, *flowY, *errors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_IFrameSource_reset(cv::videostab::IFrameSource* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_IFrameSource_nextFrame(cv::videostab::IFrameSource* instance) {
		try {
			cv::Mat ret = instance->nextFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_videostab_ILog_print_const_charX(cv::videostab::ILog* instance, const char* format) {
		try {
			instance->print(format);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_IOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::videostab::IOutlierRejector* instance, const cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::videostab::ISparseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::ImageMotionEstimatorBase* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_ImageMotionEstimatorBase_motionModel_const(const cv::videostab::ImageMotionEstimatorBase* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<cv::Mat*> cv_videostab_ImageMotionEstimatorBase_estimate_const_MatX_const_MatX_boolX(cv::videostab::ImageMotionEstimatorBase* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_videostab_InpainterBase_setRadius_int(cv::videostab::InpainterBase* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_InpainterBase_radius_const(const cv::videostab::InpainterBase* instance) {
		try {
			int ret = instance->radius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_InpainterBase_setMotionModel_MotionModel(cv::videostab::InpainterBase* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_InpainterBase_motionModel_const(const cv::videostab::InpainterBase* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_InpainterBase_inpaint_int_MatX_MatX(cv::videostab::InpainterBase* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpainterBase_setFrames_const_vector_Mat_X(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setFrames(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_InpainterBase_frames_const(const cv::videostab::InpainterBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->frames();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_InpainterBase_setMotions_const_vector_Mat_X(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_InpainterBase_motions_const(const cv::videostab::InpainterBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->motions();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_InpainterBase_setStabilizedFrames_const_vector_Mat_X(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setStabilizedFrames(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_InpainterBase_stabilizedFrames_const(const cv::videostab::InpainterBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->stabilizedFrames();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_InpainterBase_setStabilizationMotions_const_vector_Mat_X(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setStabilizationMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_InpainterBase_stabilizationMotions_const(const cv::videostab::InpainterBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->stabilizationMotions();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	void cv_InpaintingPipeline_delete(cv::videostab::InpaintingPipeline* instance) {
		delete instance;
	}
	Result_void cv_videostab_InpaintingPipeline_pushBack_Ptr_InpainterBase_(cv::videostab::InpaintingPipeline* instance, cv::Ptr<cv::videostab::InpainterBase>* inpainter) {
		try {
			instance->pushBack(*inpainter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_InpaintingPipeline_empty_const(const cv::videostab::InpaintingPipeline* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setRadius_int(cv::videostab::InpaintingPipeline* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(cv::videostab::InpaintingPipeline* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setFrames_const_vector_Mat_X(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setFrames(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setMotions_const_vector_Mat_X(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vector_Mat_X(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setStabilizedFrames(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vector_Mat_X(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setStabilizationMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_inpaint_int_MatX_MatX(cv::videostab::InpaintingPipeline* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KeypointBasedMotionEstimator_delete(cv::videostab::KeypointBasedMotionEstimator* instance) {
		delete instance;
	}
	Result<cv::videostab::KeypointBasedMotionEstimator*> cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_Ptr_MotionEstimatorBase_(cv::Ptr<cv::videostab::MotionEstimatorBase>* estimator) {
		try {
			cv::videostab::KeypointBasedMotionEstimator* ret = new cv::videostab::KeypointBasedMotionEstimator(*estimator);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::KeypointBasedMotionEstimator*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(cv::videostab::KeypointBasedMotionEstimator* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_KeypointBasedMotionEstimator_motionModel_const(const cv::videostab::KeypointBasedMotionEstimator* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setDetector_Ptr_Feature2D_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::Feature2D>* val) {
		try {
			instance->setDetector(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::Feature2D>*> cv_videostab_KeypointBasedMotionEstimator_detector_const(const cv::videostab::KeypointBasedMotionEstimator* instance) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->detector();
			return Ok(new cv::Ptr<cv::Feature2D>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Feature2D>*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_Ptr_ISparseOptFlowEstimator_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* val) {
		try {
			instance->setOpticalFlowEstimator(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*> cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(const cv::videostab::KeypointBasedMotionEstimator* instance) {
		try {
			cv::Ptr<cv::videostab::ISparseOptFlowEstimator> ret = instance->opticalFlowEstimator();
			return Ok(new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_Ptr_IOutlierRejector_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::IOutlierRejector>* val) {
		try {
			instance->setOutlierRejector(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::IOutlierRejector>*> cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(const cv::videostab::KeypointBasedMotionEstimator* instance) {
		try {
			cv::Ptr<cv::videostab::IOutlierRejector> ret = instance->outlierRejector();
			return Ok(new cv::Ptr<cv::videostab::IOutlierRejector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::IOutlierRejector>*>)
	}
	
	Result<cv::Mat*> cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatX_const_MatX_boolX(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_LogToStdout_delete(cv::videostab::LogToStdout* instance) {
		delete instance;
	}
	Result_void cv_videostab_LogToStdout_print_const_charX(cv::videostab::LogToStdout* instance, const char* format) {
		try {
			instance->print(format);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LpMotionStabilizer_delete(cv::videostab::LpMotionStabilizer* instance) {
		delete instance;
	}
	Result<cv::videostab::LpMotionStabilizer*> cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::LpMotionStabilizer* ret = new cv::videostab::LpMotionStabilizer(model);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::LpMotionStabilizer*>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(cv::videostab::LpMotionStabilizer* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_LpMotionStabilizer_motionModel_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setFrameSize_Size(cv::videostab::LpMotionStabilizer* instance, const cv::Size* val) {
		try {
			instance->setFrameSize(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_LpMotionStabilizer_frameSize_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			cv::Size ret = instance->frameSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setTrimRatio_float(cv::videostab::LpMotionStabilizer* instance, float val) {
		try {
			instance->setTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_trimRatio_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			float ret = instance->trimRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight1_float(cv::videostab::LpMotionStabilizer* instance, float val) {
		try {
			instance->setWeight1(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight1_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			float ret = instance->weight1();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight2_float(cv::videostab::LpMotionStabilizer* instance, float val) {
		try {
			instance->setWeight2(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight2_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			float ret = instance->weight2();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight3_float(cv::videostab::LpMotionStabilizer* instance, float val) {
		try {
			instance->setWeight3(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight3_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			float ret = instance->weight3();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight4_float(cv::videostab::LpMotionStabilizer* instance, float val) {
		try {
			instance->setWeight4(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight4_const(const cv::videostab::LpMotionStabilizer* instance) {
		try {
			float ret = instance->weight4();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	void cv_MoreAccurateMotionWobbleSuppressor_delete(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
		delete instance;
	}
	Result_void cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatX_MatX(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result) {
		try {
			instance->suppress(idx, *frame, *result);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance, int val) {
		try {
			instance->setPeriod(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance) {
		try {
			int ret = instance->period();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::MotionEstimatorBase* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_MotionEstimatorBase_motionModel_const(const cv::videostab::MotionEstimatorBase* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<cv::Mat*> cv_videostab_MotionEstimatorBase_estimate_const__InputArrayX_const__InputArrayX_boolX(cv::videostab::MotionEstimatorBase* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_MotionEstimatorL1_delete(cv::videostab::MotionEstimatorL1* instance) {
		delete instance;
	}
	Result<cv::videostab::MotionEstimatorL1*> cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::MotionEstimatorL1* ret = new cv::videostab::MotionEstimatorL1(model);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionEstimatorL1*>)
	}
	
	Result<cv::Mat*> cv_videostab_MotionEstimatorL1_estimate_const__InputArrayX_const__InputArrayX_boolX(cv::videostab::MotionEstimatorL1* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_MotionEstimatorRansacL2_delete(cv::videostab::MotionEstimatorRansacL2* instance) {
		delete instance;
	}
	Result<cv::videostab::MotionEstimatorRansacL2*> cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::MotionEstimatorRansacL2* ret = new cv::videostab::MotionEstimatorRansacL2(model);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionEstimatorRansacL2*>)
	}
	
	Result_void cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsX(cv::videostab::MotionEstimatorRansacL2* instance, const cv::videostab::RansacParams* val) {
		try {
			instance->setRansacParams(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::RansacParams*> cv_videostab_MotionEstimatorRansacL2_ransacParams_const(const cv::videostab::MotionEstimatorRansacL2* instance) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			return Ok(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<cv::videostab::RansacParams*>)
	}
	
	Result_void cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(cv::videostab::MotionEstimatorRansacL2* instance, float val) {
		try {
			instance->setMinInlierRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(const cv::videostab::MotionEstimatorRansacL2* instance) {
		try {
			float ret = instance->minInlierRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Mat*> cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayX_const__InputArrayX_boolX(cv::videostab::MotionEstimatorRansacL2* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_MotionInpainter_delete(cv::videostab::MotionInpainter* instance) {
		delete instance;
	}
	Result<cv::videostab::MotionInpainter*> cv_videostab_MotionInpainter_MotionInpainter() {
		try {
			cv::videostab::MotionInpainter* ret = new cv::videostab::MotionInpainter();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionInpainter*>)
	}
	
	Result_void cv_videostab_MotionInpainter_setOptFlowEstimator_Ptr_IDenseOptFlowEstimator_(cv::videostab::MotionInpainter* instance, cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* val) {
		try {
			instance->setOptFlowEstimator(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*> cv_videostab_MotionInpainter_optFlowEstimator_const(const cv::videostab::MotionInpainter* instance) {
		try {
			cv::Ptr<cv::videostab::IDenseOptFlowEstimator> ret = instance->optFlowEstimator();
			return Ok(new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>)
	}
	
	Result_void cv_videostab_MotionInpainter_setFlowErrorThreshold_float(cv::videostab::MotionInpainter* instance, float val) {
		try {
			instance->setFlowErrorThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionInpainter_flowErrorThreshold_const(const cv::videostab::MotionInpainter* instance) {
		try {
			float ret = instance->flowErrorThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_MotionInpainter_setDistThreshold_float(cv::videostab::MotionInpainter* instance, float val) {
		try {
			instance->setDistThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionInpainter_distThresh_const(const cv::videostab::MotionInpainter* instance) {
		try {
			float ret = instance->distThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_MotionInpainter_setBorderMode_int(cv::videostab::MotionInpainter* instance, int val) {
		try {
			instance->setBorderMode(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_MotionInpainter_borderMode_const(const cv::videostab::MotionInpainter* instance) {
		try {
			int ret = instance->borderMode();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_MotionInpainter_inpaint_int_MatX_MatX(cv::videostab::MotionInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask) {
		try {
			instance->inpaint(idx, *frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MotionStabilizationPipeline_delete(cv::videostab::MotionStabilizationPipeline* instance) {
		delete instance;
	}
	Result_void cv_videostab_MotionStabilizationPipeline_pushBack_Ptr_IMotionStabilizer_(cv::videostab::MotionStabilizationPipeline* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* stabilizer) {
		try {
			instance->pushBack(*stabilizer);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_MotionStabilizationPipeline_empty_const(const cv::videostab::MotionStabilizationPipeline* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_NullDeblurer_delete(cv::videostab::NullDeblurer* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullDeblurer_deblur_int_MatX(cv::videostab::NullDeblurer* instance, int unnamed, cv::Mat* unnamed_1) {
		try {
			instance->deblur(unnamed, *unnamed_1);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullFrameSource_delete(cv::videostab::NullFrameSource* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullFrameSource_reset(cv::videostab::NullFrameSource* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_NullFrameSource_nextFrame(cv::videostab::NullFrameSource* instance) {
		try {
			cv::Mat ret = instance->nextFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_NullInpainter_delete(cv::videostab::NullInpainter* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullInpainter_inpaint_int_MatX_MatX(cv::videostab::NullInpainter* instance, int unnamed, cv::Mat* unnamed_1, cv::Mat* unnamed_2) {
		try {
			instance->inpaint(unnamed, *unnamed_1, *unnamed_2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullLog_delete(cv::videostab::NullLog* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullLog_print_const_charX(cv::videostab::NullLog* instance, const char* unnamed) {
		try {
			instance->print(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullOutlierRejector_delete(cv::videostab::NullOutlierRejector* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::videostab::NullOutlierRejector* instance, const cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullWobbleSuppressor_delete(cv::videostab::NullWobbleSuppressor* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullWobbleSuppressor_suppress_int_const_MatX_MatX(cv::videostab::NullWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result) {
		try {
			instance->suppress(idx, *frame, *result);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_OnePassStabilizer_delete(cv::videostab::OnePassStabilizer* instance) {
		delete instance;
	}
	Result<cv::videostab::OnePassStabilizer*> cv_videostab_OnePassStabilizer_OnePassStabilizer() {
		try {
			cv::videostab::OnePassStabilizer* ret = new cv::videostab::OnePassStabilizer();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::OnePassStabilizer*>)
	}
	
	Result_void cv_videostab_OnePassStabilizer_setMotionFilter_Ptr_MotionFilterBase_(cv::videostab::OnePassStabilizer* instance, cv::Ptr<cv::videostab::MotionFilterBase>* val) {
		try {
			instance->setMotionFilter(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::MotionFilterBase>*> cv_videostab_OnePassStabilizer_motionFilter_const(const cv::videostab::OnePassStabilizer* instance) {
		try {
			cv::Ptr<cv::videostab::MotionFilterBase> ret = instance->motionFilter();
			return Ok(new cv::Ptr<cv::videostab::MotionFilterBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::MotionFilterBase>*>)
	}
	
	Result_void cv_videostab_OnePassStabilizer_reset(cv::videostab::OnePassStabilizer* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_OnePassStabilizer_nextFrame(cv::videostab::OnePassStabilizer* instance) {
		try {
			cv::Mat ret = instance->nextFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_PyrLkOptFlowEstimatorBase_delete(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		delete instance;
	}
	Result<cv::videostab::PyrLkOptFlowEstimatorBase*> cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase() {
		try {
			cv::videostab::PyrLkOptFlowEstimatorBase* ret = new cv::videostab::PyrLkOptFlowEstimatorBase();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::PyrLkOptFlowEstimatorBase*>)
	}
	
	Result_void cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(cv::videostab::PyrLkOptFlowEstimatorBase* instance, const cv::Size* val) {
		try {
			instance->setWinSize(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		try {
			cv::Size ret = instance->winSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(cv::videostab::PyrLkOptFlowEstimatorBase* instance, int val) {
		try {
			instance->setMaxLevel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		try {
			int ret = instance->maxLevel();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_RansacParams_size_const(const cv::videostab::RansacParams* instance) {
		try {
			int ret = instance->size;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_RansacParams_setSize_int(cv::videostab::RansacParams* instance, int val) {
		try {
			instance->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_thresh_const(const cv::videostab::RansacParams* instance) {
		try {
			float ret = instance->thresh;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setThresh_float(cv::videostab::RansacParams* instance, float val) {
		try {
			instance->thresh = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_eps_const(const cv::videostab::RansacParams* instance) {
		try {
			float ret = instance->eps;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setEps_float(cv::videostab::RansacParams* instance, float val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_prob_const(const cv::videostab::RansacParams* instance) {
		try {
			float ret = instance->prob;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setProb_float(cv::videostab::RansacParams* instance, float val) {
		try {
			instance->prob = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RansacParams_delete(cv::videostab::RansacParams* instance) {
		delete instance;
	}
	Result<cv::videostab::RansacParams*> cv_videostab_RansacParams_RansacParams() {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::RansacParams*>)
	}
	
	Result<cv::videostab::RansacParams*> cv_videostab_RansacParams_RansacParams_int_float_float_float(int size, float thresh, float eps, float prob) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams(size, thresh, eps, prob);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::RansacParams*>)
	}
	
	Result<int> cv_videostab_RansacParams_niters_const(const cv::videostab::RansacParams* instance) {
		try {
			int ret = instance->niters();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::videostab::RansacParams*> cv_videostab_RansacParams_default2dMotion_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::RansacParams ret = cv::videostab::RansacParams::default2dMotion(model);
			return Ok(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<cv::videostab::RansacParams*>)
	}
	
	void cv_SparsePyrLkOptFlowEstimator_delete(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
		delete instance;
	}
	Result_void cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::videostab::SparsePyrLkOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_StabilizerBase_setLog_Ptr_ILog_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ILog>* ilog) {
		try {
			instance->setLog(*ilog);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::ILog>*> cv_videostab_StabilizerBase_log_const(const cv::videostab::StabilizerBase* instance) {
		try {
			cv::Ptr<cv::videostab::ILog> ret = instance->log();
			return Ok(new cv::Ptr<cv::videostab::ILog>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::ILog>*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setRadius_int(cv::videostab::StabilizerBase* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_StabilizerBase_radius_const(const cv::videostab::StabilizerBase* instance) {
		try {
			int ret = instance->radius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_StabilizerBase_setFrameSource_Ptr_IFrameSource_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::IFrameSource>* val) {
		try {
			instance->setFrameSource(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::IFrameSource>*> cv_videostab_StabilizerBase_frameSource_const(const cv::videostab::StabilizerBase* instance) {
		try {
			cv::Ptr<cv::videostab::IFrameSource> ret = instance->frameSource();
			return Ok(new cv::Ptr<cv::videostab::IFrameSource>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::IFrameSource>*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val) {
		try {
			instance->setMotionEstimator(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*> cv_videostab_StabilizerBase_motionEstimator_const(const cv::videostab::StabilizerBase* instance) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			return Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setDeblurer_Ptr_DeblurerBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::DeblurerBase>* val) {
		try {
			instance->setDeblurer(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::DeblurerBase>*> cv_videostab_StabilizerBase_deblurrer_const(const cv::videostab::StabilizerBase* instance) {
		try {
			cv::Ptr<cv::videostab::DeblurerBase> ret = instance->deblurrer();
			return Ok(new cv::Ptr<cv::videostab::DeblurerBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::DeblurerBase>*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setTrimRatio_float(cv::videostab::StabilizerBase* instance, float val) {
		try {
			instance->setTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_StabilizerBase_trimRatio_const(const cv::videostab::StabilizerBase* instance) {
		try {
			float ret = instance->trimRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(cv::videostab::StabilizerBase* instance, bool val) {
		try {
			instance->setCorrectionForInclusion(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_StabilizerBase_doCorrectionForInclusion_const(const cv::videostab::StabilizerBase* instance) {
		try {
			bool ret = instance->doCorrectionForInclusion();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_StabilizerBase_setBorderMode_int(cv::videostab::StabilizerBase* instance, int val) {
		try {
			instance->setBorderMode(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_StabilizerBase_borderMode_const(const cv::videostab::StabilizerBase* instance) {
		try {
			int ret = instance->borderMode();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_StabilizerBase_setInpainter_Ptr_InpainterBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::InpainterBase>* val) {
		try {
			instance->setInpainter(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::InpainterBase>*> cv_videostab_StabilizerBase_inpainter_const(const cv::videostab::StabilizerBase* instance) {
		try {
			cv::Ptr<cv::videostab::InpainterBase> ret = instance->inpainter();
			return Ok(new cv::Ptr<cv::videostab::InpainterBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::InpainterBase>*>)
	}
	
	void cv_ToFileMotionWriter_delete(cv::videostab::ToFileMotionWriter* instance) {
		delete instance;
	}
	Result<cv::videostab::ToFileMotionWriter*> cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringX_Ptr_ImageMotionEstimatorBase_(const char* path, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* estimator) {
		try {
			cv::videostab::ToFileMotionWriter* ret = new cv::videostab::ToFileMotionWriter(cv::String(path), *estimator);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::ToFileMotionWriter*>)
	}
	
	Result_void cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(cv::videostab::ToFileMotionWriter* instance, cv::videostab::MotionModel val) {
		try {
			instance->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_ToFileMotionWriter_motionModel_const(const cv::videostab::ToFileMotionWriter* instance) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<cv::Mat*> cv_videostab_ToFileMotionWriter_estimate_const_MatX_const_MatX_boolX(cv::videostab::ToFileMotionWriter* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_TranslationBasedLocalOutlierRejector_delete(cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
		delete instance;
	}
	Result<cv::videostab::TranslationBasedLocalOutlierRejector*> cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector() {
		try {
			cv::videostab::TranslationBasedLocalOutlierRejector* ret = new cv::videostab::TranslationBasedLocalOutlierRejector();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::TranslationBasedLocalOutlierRejector*>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(cv::videostab::TranslationBasedLocalOutlierRejector* instance, const cv::Size* val) {
		try {
			instance->setCellSize(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
		try {
			cv::Size ret = instance->cellSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::videostab::RansacParams* val) {
		try {
			instance->setRansacParams(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::RansacParams*> cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			return Ok(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<cv::videostab::RansacParams*>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::videostab::TranslationBasedLocalOutlierRejector* instance, const cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_TwoPassStabilizer_delete(cv::videostab::TwoPassStabilizer* instance) {
		delete instance;
	}
	Result<cv::videostab::TwoPassStabilizer*> cv_videostab_TwoPassStabilizer_TwoPassStabilizer() {
		try {
			cv::videostab::TwoPassStabilizer* ret = new cv::videostab::TwoPassStabilizer();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::TwoPassStabilizer*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setMotionStabilizer_Ptr_IMotionStabilizer_(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* val) {
		try {
			instance->setMotionStabilizer(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::IMotionStabilizer>*> cv_videostab_TwoPassStabilizer_motionStabilizer_const(const cv::videostab::TwoPassStabilizer* instance) {
		try {
			cv::Ptr<cv::videostab::IMotionStabilizer> ret = instance->motionStabilizer();
			return Ok(new cv::Ptr<cv::videostab::IMotionStabilizer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setWobbleSuppressor_Ptr_WobbleSuppressorBase_(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::WobbleSuppressorBase>* val) {
		try {
			instance->setWobbleSuppressor(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*> cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(const cv::videostab::TwoPassStabilizer* instance) {
		try {
			cv::Ptr<cv::videostab::WobbleSuppressorBase> ret = instance->wobbleSuppressor();
			return Ok(new cv::Ptr<cv::videostab::WobbleSuppressorBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(cv::videostab::TwoPassStabilizer* instance, bool val) {
		try {
			instance->setEstimateTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(const cv::videostab::TwoPassStabilizer* instance) {
		try {
			bool ret = instance->mustEstimateTrimaRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_reset(cv::videostab::TwoPassStabilizer* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_TwoPassStabilizer_nextFrame(cv::videostab::TwoPassStabilizer* instance) {
		try {
			cv::Mat ret = instance->nextFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_VideoFileSource_delete(cv::videostab::VideoFileSource* instance) {
		delete instance;
	}
	Result<cv::videostab::VideoFileSource*> cv_videostab_VideoFileSource_VideoFileSource_const_StringX_bool(const char* path, bool volatileFrame) {
		try {
			cv::videostab::VideoFileSource* ret = new cv::videostab::VideoFileSource(cv::String(path), volatileFrame);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::VideoFileSource*>)
	}
	
	Result_void cv_videostab_VideoFileSource_reset(cv::videostab::VideoFileSource* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_videostab_VideoFileSource_nextFrame(cv::videostab::VideoFileSource* instance) {
		try {
			cv::Mat ret = instance->nextFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<int> cv_videostab_VideoFileSource_width(cv::videostab::VideoFileSource* instance) {
		try {
			int ret = instance->width();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_VideoFileSource_height(cv::videostab::VideoFileSource* instance) {
		try {
			int ret = instance->height();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_VideoFileSource_count(cv::videostab::VideoFileSource* instance) {
		try {
			int ret = instance->count();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_videostab_VideoFileSource_fps(cv::videostab::VideoFileSource* instance) {
		try {
			double ret = instance->fps();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_WeightingDeblurer_delete(cv::videostab::WeightingDeblurer* instance) {
		delete instance;
	}
	Result<cv::videostab::WeightingDeblurer*> cv_videostab_WeightingDeblurer_WeightingDeblurer() {
		try {
			cv::videostab::WeightingDeblurer* ret = new cv::videostab::WeightingDeblurer();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::videostab::WeightingDeblurer*>)
	}
	
	Result_void cv_videostab_WeightingDeblurer_setSensitivity_float(cv::videostab::WeightingDeblurer* instance, float val) {
		try {
			instance->setSensitivity(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_WeightingDeblurer_sensitivity_const(const cv::videostab::WeightingDeblurer* instance) {
		try {
			float ret = instance->sensitivity();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_WeightingDeblurer_deblur_int_MatX(cv::videostab::WeightingDeblurer* instance, int idx, cv::Mat* frame) {
		try {
			instance->deblur(idx, *frame);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(cv::videostab::WobbleSuppressorBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val) {
		try {
			instance->setMotionEstimator(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*> cv_videostab_WobbleSuppressorBase_motionEstimator_const(const cv::videostab::WobbleSuppressorBase* instance) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			return Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_suppress_int_const_MatX_MatX(cv::videostab::WobbleSuppressorBase* instance, int idx, const cv::Mat* frame, cv::Mat* result) {
		try {
			instance->suppress(idx, *frame, *result);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setFrameCount_int(cv::videostab::WobbleSuppressorBase* instance, int val) {
		try {
			instance->setFrameCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_WobbleSuppressorBase_frameCount_const(const cv::videostab::WobbleSuppressorBase* instance) {
		try {
			int ret = instance->frameCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotions_const_vector_Mat_X(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_WobbleSuppressorBase_motions_const(const cv::videostab::WobbleSuppressorBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->motions();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotions2_const_vector_Mat_X(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setMotions2(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_WobbleSuppressorBase_motions2_const(const cv::videostab::WobbleSuppressorBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->motions2();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vector_Mat_X(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val) {
		try {
			instance->setStabilizationMotions(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(const cv::videostab::WobbleSuppressorBase* instance) {
		try {
			std::vector<cv::Mat> ret = instance->stabilizationMotions();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
}
