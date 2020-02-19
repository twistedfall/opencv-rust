#include "common.hpp"
#include <opencv2/videostab.hpp>
#include "videostab_types.hpp"

extern "C" {
	Result<float> cv_videostab_calcBlurriness_const_MatX(void* frame) {
		try {
			float ret = cv::videostab::calcBlurriness(*reinterpret_cast<const cv::Mat*>(frame));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_calcFlowMask_const_MatX_const_MatX_const_MatX_float_const_MatX_const_MatX_MatX(void* flowX, void* flowY, void* errors, float maxError, void* mask0, void* mask1, void* flowMask) {
		try {
			cv::videostab::calcFlowMask(*reinterpret_cast<const cv::Mat*>(flowX), *reinterpret_cast<const cv::Mat*>(flowY), *reinterpret_cast<const cv::Mat*>(errors), maxError, *reinterpret_cast<const cv::Mat*>(mask0), *reinterpret_cast<const cv::Mat*>(mask1), *reinterpret_cast<cv::Mat*>(flowMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_completeFrameAccordingToFlow_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_float_MatX_MatX(void* flowMask, void* flowX, void* flowY, void* frame1, void* mask1, float distThresh, void* frame0, void* mask0) {
		try {
			cv::videostab::completeFrameAccordingToFlow(*reinterpret_cast<const cv::Mat*>(flowMask), *reinterpret_cast<const cv::Mat*>(flowX), *reinterpret_cast<const cv::Mat*>(flowY), *reinterpret_cast<const cv::Mat*>(frame1), *reinterpret_cast<const cv::Mat*>(mask1), distThresh, *reinterpret_cast<cv::Mat*>(frame0), *reinterpret_cast<cv::Mat*>(mask0));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_ensureInclusionConstraint_const_MatX_Size_float(void* M, cv::Size size, float trimRatio) {
		try {
			cv::Mat ret = cv::videostab::ensureInclusionConstraint(*reinterpret_cast<const cv::Mat*>(M), size, trimRatio);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayX_const__InputOutputArrayX_int_floatX(void* points0, void* points1, int model, float* rmse) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionLeastSquares(*reinterpret_cast<const cv::_InputOutputArray*>(points0), *reinterpret_cast<const cv::_InputOutputArray*>(points1), model, rmse);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_estimateGlobalMotionRansac_const__InputArrayX_const__InputArrayX_int_const_RansacParamsX_floatX_intX(void* points0, void* points1, int model, void* params, float* rmse, int* ninliers) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionRansac(*reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), model, *reinterpret_cast<const cv::videostab::RansacParams*>(params), rmse, ninliers);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_videostab_estimateOptimalTrimRatio_const_MatX_Size(void* M, cv::Size size) {
		try {
			float ret = cv::videostab::estimateOptimalTrimRatio(*reinterpret_cast<const cv::Mat*>(M), size);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_videostab_getMotion_int_int_const_vector_Mat_X(int from, int to, void* motions) {
		try {
			cv::Mat ret = cv::videostab::getMotion(from, to, *reinterpret_cast<const std::vector<cv::Mat>*>(motions));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ColorAverageInpainter_delete(cv::videostab::ColorAverageInpainter* instance) {
		delete instance;
	}
	Result_void cv_videostab_ColorAverageInpainter_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::ColorAverageInpainter*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ColorInpainter_delete(cv::videostab::ColorInpainter* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_ColorInpainter_ColorInpainter_int_double(int method, double radius) {
		try {
			cv::videostab::ColorInpainter* ret = new cv::videostab::ColorInpainter(method, radius);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_ColorInpainter_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::ColorInpainter*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ConsistentMosaicInpainter_delete(cv::videostab::ConsistentMosaicInpainter* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter() {
		try {
			cv::videostab::ConsistentMosaicInpainter* ret = new cv::videostab::ConsistentMosaicInpainter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::ConsistentMosaicInpainter*>(instance)->setStdevThresh(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::ConsistentMosaicInpainter*>(instance)->stdevThresh();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::ConsistentMosaicInpainter*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_DeblurerBase_setRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_DeblurerBase_radius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->radius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_DeblurerBase_deblur_int_MatX(void* instance, int idx, void* frame) {
		try {
			reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->deblur(idx, *reinterpret_cast<cv::Mat*>(frame));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_DeblurerBase_setFrames_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->setFrames(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_DeblurerBase_frames_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->frames();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_DeblurerBase_setMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->setMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_DeblurerBase_motions_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->motions();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_DeblurerBase_setBlurrinessRates_const_vector_float_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->setBlurrinessRates(*reinterpret_cast<const std::vector<float>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_DeblurerBase_blurrinessRates_const(void* instance) {
		try {
			std::vector<float> ret = reinterpret_cast<cv::videostab::DeblurerBase*>(instance)->blurrinessRates();
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_FastMarchingMethod_delete(cv::videostab::FastMarchingMethod* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_FastMarchingMethod_FastMarchingMethod() {
		try {
			cv::videostab::FastMarchingMethod* ret = new cv::videostab::FastMarchingMethod();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_FastMarchingMethod_distanceMap_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::FastMarchingMethod*>(instance)->distanceMap();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_FromFileMotionReader_delete(cv::videostab::FromFileMotionReader* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringX(const char* path) {
		try {
			cv::videostab::FromFileMotionReader* ret = new cv::videostab::FromFileMotionReader(cv::String(path));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_FromFileMotionReader_estimate_const_MatX_const_MatX_boolX(void* instance, void* frame0, void* frame1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::FromFileMotionReader*>(instance)->estimate(*reinterpret_cast<const cv::Mat*>(frame0), *reinterpret_cast<const cv::Mat*>(frame1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_GaussianMotionFilter_delete(cv::videostab::GaussianMotionFilter* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(int radius, float stdev) {
		try {
			cv::videostab::GaussianMotionFilter* ret = new cv::videostab::GaussianMotionFilter(radius, stdev);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_GaussianMotionFilter_setParams_int_float(void* instance, int radius, float stdev) {
		try {
			reinterpret_cast<cv::videostab::GaussianMotionFilter*>(instance)->setParams(radius, stdev);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_GaussianMotionFilter_radius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::GaussianMotionFilter*>(instance)->radius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_videostab_GaussianMotionFilter_stdev_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::GaussianMotionFilter*>(instance)->stdev();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX(void* instance, void* frame0, void* frame1, void* flowX, void* flowY, void* errors) {
		try {
			reinterpret_cast<cv::videostab::IDenseOptFlowEstimator*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(frame0), *reinterpret_cast<const cv::_InputArray*>(frame1), *reinterpret_cast<const cv::_InputOutputArray*>(flowX), *reinterpret_cast<const cv::_InputOutputArray*>(flowY), *reinterpret_cast<const cv::_OutputArray*>(errors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_IFrameSource_reset(void* instance) {
		try {
			reinterpret_cast<cv::videostab::IFrameSource*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_IFrameSource_nextFrame(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::IFrameSource*>(instance)->nextFrame();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_ILog_print_const_charX(void* instance, const char* format) {
		try {
			reinterpret_cast<cv::videostab::ILog*>(instance)->print(format);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_IOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, cv::Size frameSize, void* points0, void* points1, void* mask) {
		try {
			reinterpret_cast<cv::videostab::IOutlierRejector*>(instance)->process(frameSize, *reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* frame0, void* frame1, void* points0, void* points1, void* status, void* errors) {
		try {
			reinterpret_cast<cv::videostab::ISparseOptFlowEstimator*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(frame0), *reinterpret_cast<const cv::_InputArray*>(frame1), *reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputOutputArray*>(points1), *reinterpret_cast<const cv::_OutputArray*>(status), *reinterpret_cast<const cv::_OutputArray*>(errors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::ImageMotionEstimatorBase*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_ImageMotionEstimatorBase_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::ImageMotionEstimatorBase*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<void*> cv_videostab_ImageMotionEstimatorBase_estimate_const_MatX_const_MatX_boolX(void* instance, void* frame0, void* frame1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::ImageMotionEstimatorBase*>(instance)->estimate(*reinterpret_cast<const cv::Mat*>(frame0), *reinterpret_cast<const cv::Mat*>(frame1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_InpainterBase_setRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_InpainterBase_radius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->radius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_InpainterBase_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_InpainterBase_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_InpainterBase_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpainterBase_setFrames_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setFrames(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_InpainterBase_frames_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->frames();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_InpainterBase_setMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_InpainterBase_motions_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->motions();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_InpainterBase_setStabilizedFrames_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setStabilizedFrames(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_InpainterBase_stabilizedFrames_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->stabilizedFrames();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_InpainterBase_setStabilizationMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpainterBase*>(instance)->setStabilizationMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_InpainterBase_stabilizationMotions_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::InpainterBase*>(instance)->stabilizationMotions();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_InpaintingPipeline_delete(cv::videostab::InpaintingPipeline* instance) {
		delete instance;
	}
	Result_void cv_videostab_InpaintingPipeline_pushBack_Ptr_InpainterBase_(void* instance, void* inpainter) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->pushBack(*reinterpret_cast<cv::Ptr<cv::videostab::InpainterBase>*>(inpainter));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_InpaintingPipeline_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setFrames_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setFrames(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setStabilizedFrames(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->setStabilizationMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_InpaintingPipeline_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::InpaintingPipeline*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KeypointBasedMotionEstimator_delete(cv::videostab::KeypointBasedMotionEstimator* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_Ptr_MotionEstimatorBase_(void* estimator) {
		try {
			cv::videostab::KeypointBasedMotionEstimator* ret = new cv::videostab::KeypointBasedMotionEstimator(*reinterpret_cast<cv::Ptr<cv::videostab::MotionEstimatorBase>*>(estimator));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_KeypointBasedMotionEstimator_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setDetector_Ptr_FeatureDetector_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->setDetector(*reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_detector_const(void* instance) {
		try {
			cv::Ptr<cv::FeatureDetector> ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->detector();
			return Ok<void*>(new cv::Ptr<cv::FeatureDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_Ptr_ISparseOptFlowEstimator_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->setOpticalFlowEstimator(*reinterpret_cast<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::ISparseOptFlowEstimator> ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->opticalFlowEstimator();
			return Ok<void*>(new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_Ptr_IOutlierRejector_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->setOutlierRejector(*reinterpret_cast<cv::Ptr<cv::videostab::IOutlierRejector>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::IOutlierRejector> ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->outlierRejector();
			return Ok<void*>(new cv::Ptr<cv::videostab::IOutlierRejector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatX_const_MatX_boolX(void* instance, void* frame0, void* frame1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->estimate(*reinterpret_cast<const cv::Mat*>(frame0), *reinterpret_cast<const cv::Mat*>(frame1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayX_const__InputArrayX_boolX(void* instance, void* frame0, void* frame1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance)->estimate(*reinterpret_cast<const cv::_InputArray*>(frame0), *reinterpret_cast<const cv::_InputArray*>(frame1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LogToStdout_delete(cv::videostab::LogToStdout* instance) {
		delete instance;
	}
	Result_void cv_videostab_LogToStdout_print_const_charX(void* instance, const char* format) {
		try {
			reinterpret_cast<cv::videostab::LogToStdout*>(instance)->print(format);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LpMotionStabilizer_delete(cv::videostab::LpMotionStabilizer* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::LpMotionStabilizer* ret = new cv::videostab::LpMotionStabilizer(model);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_LpMotionStabilizer_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setFrameSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setFrameSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_LpMotionStabilizer_frameSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->frameSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setTrimRatio_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_trimRatio_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->trimRatio();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight1_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setWeight1(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight1_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->weight1();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight2_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setWeight2(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight2_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->weight2();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight3_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setWeight3(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight3_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->weight3();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_LpMotionStabilizer_setWeight4_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->setWeight4(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_LpMotionStabilizer_weight4_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::LpMotionStabilizer*>(instance)->weight4();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	void cv_MoreAccurateMotionWobbleSuppressor_delete(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
		delete instance;
	}
	Result_void cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatX_MatX(void* instance, int idx, void* frame, void* result) {
		try {
			reinterpret_cast<cv::videostab::MoreAccurateMotionWobbleSuppressor*>(instance)->suppress(idx, *reinterpret_cast<const cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase*>(instance)->setPeriod(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase*>(instance)->period();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::MotionEstimatorBase*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_MotionEstimatorBase_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::MotionEstimatorBase*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<void*> cv_videostab_MotionEstimatorBase_estimate_const__InputArrayX_const__InputArrayX_boolX(void* instance, void* points0, void* points1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::MotionEstimatorBase*>(instance)->estimate(*reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_MotionEstimatorL1_delete(cv::videostab::MotionEstimatorL1* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::MotionEstimatorL1* ret = new cv::videostab::MotionEstimatorL1(model);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_MotionEstimatorL1_estimate_const__InputArrayX_const__InputArrayX_boolX(void* instance, void* points0, void* points1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::MotionEstimatorL1*>(instance)->estimate(*reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_MotionEstimatorRansacL2_delete(cv::videostab::MotionEstimatorRansacL2* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::MotionEstimatorRansacL2* ret = new cv::videostab::MotionEstimatorRansacL2(model);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::MotionEstimatorRansacL2*>(instance)->setRansacParams(*reinterpret_cast<const cv::videostab::RansacParams*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_MotionEstimatorRansacL2_ransacParams_const(void* instance) {
		try {
			cv::videostab::RansacParams ret = reinterpret_cast<cv::videostab::MotionEstimatorRansacL2*>(instance)->ransacParams();
			return Ok<void*>(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::MotionEstimatorRansacL2*>(instance)->setMinInlierRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::MotionEstimatorRansacL2*>(instance)->minInlierRatio();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayX_const__InputArrayX_boolX(void* instance, void* points0, void* points1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::MotionEstimatorRansacL2*>(instance)->estimate(*reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_MotionInpainter_delete(cv::videostab::MotionInpainter* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_MotionInpainter_MotionInpainter() {
		try {
			cv::videostab::MotionInpainter* ret = new cv::videostab::MotionInpainter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_MotionInpainter_setOptFlowEstimator_Ptr_IDenseOptFlowEstimator_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->setOptFlowEstimator(*reinterpret_cast<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_MotionInpainter_optFlowEstimator_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::IDenseOptFlowEstimator> ret = reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->optFlowEstimator();
			return Ok<void*>(new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_MotionInpainter_setFlowErrorThreshold_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->setFlowErrorThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionInpainter_flowErrorThreshold_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->flowErrorThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_MotionInpainter_setDistThreshold_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->setDistThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_MotionInpainter_distThresh_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->distThresh();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_MotionInpainter_setBorderMode_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->setBorderMode(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_MotionInpainter_borderMode_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->borderMode();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_MotionInpainter_inpaint_int_MatX_MatX(void* instance, int idx, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::videostab::MotionInpainter*>(instance)->inpaint(idx, *reinterpret_cast<cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MotionStabilizationPipeline_delete(cv::videostab::MotionStabilizationPipeline* instance) {
		delete instance;
	}
	Result_void cv_videostab_MotionStabilizationPipeline_pushBack_Ptr_IMotionStabilizer_(void* instance, void* stabilizer) {
		try {
			reinterpret_cast<cv::videostab::MotionStabilizationPipeline*>(instance)->pushBack(*reinterpret_cast<cv::Ptr<cv::videostab::IMotionStabilizer>*>(stabilizer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_MotionStabilizationPipeline_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::videostab::MotionStabilizationPipeline*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_NullDeblurer_delete(cv::videostab::NullDeblurer* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullDeblurer_deblur_int_MatX(void* instance, int unnamed, void* unnamed_1) {
		try {
			reinterpret_cast<cv::videostab::NullDeblurer*>(instance)->deblur(unnamed, *reinterpret_cast<cv::Mat*>(unnamed_1));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullFrameSource_delete(cv::videostab::NullFrameSource* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullFrameSource_reset(void* instance) {
		try {
			reinterpret_cast<cv::videostab::NullFrameSource*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_NullFrameSource_nextFrame(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::NullFrameSource*>(instance)->nextFrame();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_NullInpainter_delete(cv::videostab::NullInpainter* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullInpainter_inpaint_int_MatX_MatX(void* instance, int unnamed, void* unnamed_1, void* unnamed_2) {
		try {
			reinterpret_cast<cv::videostab::NullInpainter*>(instance)->inpaint(unnamed, *reinterpret_cast<cv::Mat*>(unnamed_1), *reinterpret_cast<cv::Mat*>(unnamed_2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullLog_delete(cv::videostab::NullLog* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullLog_print_const_charX(void* instance, const char* unnamed) {
		try {
			reinterpret_cast<cv::videostab::NullLog*>(instance)->print(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullOutlierRejector_delete(cv::videostab::NullOutlierRejector* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, cv::Size frameSize, void* points0, void* points1, void* mask) {
		try {
			reinterpret_cast<cv::videostab::NullOutlierRejector*>(instance)->process(frameSize, *reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NullWobbleSuppressor_delete(cv::videostab::NullWobbleSuppressor* instance) {
		delete instance;
	}
	Result_void cv_videostab_NullWobbleSuppressor_suppress_int_const_MatX_MatX(void* instance, int idx, void* frame, void* result) {
		try {
			reinterpret_cast<cv::videostab::NullWobbleSuppressor*>(instance)->suppress(idx, *reinterpret_cast<const cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_OnePassStabilizer_delete(cv::videostab::OnePassStabilizer* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_OnePassStabilizer_OnePassStabilizer() {
		try {
			cv::videostab::OnePassStabilizer* ret = new cv::videostab::OnePassStabilizer();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_OnePassStabilizer_setMotionFilter_Ptr_MotionFilterBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::OnePassStabilizer*>(instance)->setMotionFilter(*reinterpret_cast<cv::Ptr<cv::videostab::MotionFilterBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_OnePassStabilizer_motionFilter_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::MotionFilterBase> ret = reinterpret_cast<cv::videostab::OnePassStabilizer*>(instance)->motionFilter();
			return Ok<void*>(new cv::Ptr<cv::videostab::MotionFilterBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_OnePassStabilizer_reset(void* instance) {
		try {
			reinterpret_cast<cv::videostab::OnePassStabilizer*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_OnePassStabilizer_nextFrame(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::OnePassStabilizer*>(instance)->nextFrame();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PyrLkOptFlowEstimatorBase_delete(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase() {
		try {
			cv::videostab::PyrLkOptFlowEstimatorBase* ret = new cv::videostab::PyrLkOptFlowEstimatorBase();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance)->setWinSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance)->winSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance)->setMaxLevel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance)->maxLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_RansacParams_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::RansacParams*>(instance)->size;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_RansacParams_setSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::RansacParams*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_thresh_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::RansacParams*>(instance)->thresh;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setThresh_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::RansacParams*>(instance)->thresh = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_eps_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::RansacParams*>(instance)->eps;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setEps_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::RansacParams*>(instance)->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_RansacParams_prob_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::RansacParams*>(instance)->prob;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_RansacParams_setProb_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::RansacParams*>(instance)->prob = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RansacParams_delete(cv::videostab::RansacParams* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_RansacParams_RansacParams() {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videostab_RansacParams_RansacParams_int_float_float_float(int size, float thresh, float eps, float prob) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams(size, thresh, eps, prob);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_videostab_RansacParams_niters_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::RansacParams*>(instance)->niters();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_videostab_RansacParams_default2dMotion_MotionModel(cv::videostab::MotionModel model) {
		try {
			cv::videostab::RansacParams ret = cv::videostab::RansacParams::default2dMotion(model);
			return Ok<void*>(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SparsePyrLkOptFlowEstimator_delete(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
		delete instance;
	}
	Result_void cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* frame0, void* frame1, void* points0, void* points1, void* status, void* errors) {
		try {
			reinterpret_cast<cv::videostab::SparsePyrLkOptFlowEstimator*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(frame0), *reinterpret_cast<const cv::_InputArray*>(frame1), *reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputOutputArray*>(points1), *reinterpret_cast<const cv::_OutputArray*>(status), *reinterpret_cast<const cv::_OutputArray*>(errors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_StabilizerBase_setLog_Ptr_ILog_(void* instance, void* ilog) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setLog(*reinterpret_cast<cv::Ptr<cv::videostab::ILog>*>(ilog));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_StabilizerBase_log_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::ILog> ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->log();
			return Ok<void*>(new cv::Ptr<cv::videostab::ILog>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_StabilizerBase_radius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->radius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_StabilizerBase_setFrameSource_Ptr_IFrameSource_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setFrameSource(*reinterpret_cast<cv::Ptr<cv::videostab::IFrameSource>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_StabilizerBase_frameSource_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::IFrameSource> ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->frameSource();
			return Ok<void*>(new cv::Ptr<cv::videostab::IFrameSource>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setMotionEstimator(*reinterpret_cast<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_StabilizerBase_motionEstimator_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->motionEstimator();
			return Ok<void*>(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setDeblurer_Ptr_DeblurerBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setDeblurer(*reinterpret_cast<cv::Ptr<cv::videostab::DeblurerBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_StabilizerBase_deblurrer_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::DeblurerBase> ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->deblurrer();
			return Ok<void*>(new cv::Ptr<cv::videostab::DeblurerBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_StabilizerBase_setTrimRatio_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_StabilizerBase_trimRatio_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->trimRatio();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setCorrectionForInclusion(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_StabilizerBase_doCorrectionForInclusion_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->doCorrectionForInclusion();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_StabilizerBase_setBorderMode_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setBorderMode(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_StabilizerBase_borderMode_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->borderMode();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_StabilizerBase_setInpainter_Ptr_InpainterBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->setInpainter(*reinterpret_cast<cv::Ptr<cv::videostab::InpainterBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_StabilizerBase_inpainter_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::InpainterBase> ret = reinterpret_cast<cv::videostab::StabilizerBase*>(instance)->inpainter();
			return Ok<void*>(new cv::Ptr<cv::videostab::InpainterBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ToFileMotionWriter_delete(cv::videostab::ToFileMotionWriter* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringX_Ptr_ImageMotionEstimatorBase_(const char* path, void* estimator) {
		try {
			cv::videostab::ToFileMotionWriter* ret = new cv::videostab::ToFileMotionWriter(cv::String(path), *reinterpret_cast<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>(estimator));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(void* instance, cv::videostab::MotionModel val) {
		try {
			reinterpret_cast<cv::videostab::ToFileMotionWriter*>(instance)->setMotionModel(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::videostab::MotionModel> cv_videostab_ToFileMotionWriter_motionModel_const(void* instance) {
		try {
			cv::videostab::MotionModel ret = reinterpret_cast<cv::videostab::ToFileMotionWriter*>(instance)->motionModel();
			return Ok<cv::videostab::MotionModel>(ret);
		} OCVRS_CATCH(Result<cv::videostab::MotionModel>)
	}
	
	Result<void*> cv_videostab_ToFileMotionWriter_estimate_const_MatX_const_MatX_boolX(void* instance, void* frame0, void* frame1, bool* ok) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::ToFileMotionWriter*>(instance)->estimate(*reinterpret_cast<const cv::Mat*>(frame0), *reinterpret_cast<const cv::Mat*>(frame1), ok);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_TranslationBasedLocalOutlierRejector_delete(cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector() {
		try {
			cv::videostab::TranslationBasedLocalOutlierRejector* ret = new cv::videostab::TranslationBasedLocalOutlierRejector();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance)->setCellSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance)->cellSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance)->setRansacParams(*reinterpret_cast<cv::videostab::RansacParams*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(void* instance) {
		try {
			cv::videostab::RansacParams ret = reinterpret_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance)->ransacParams();
			return Ok<void*>(new cv::videostab::RansacParams(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, cv::Size frameSize, void* points0, void* points1, void* mask) {
		try {
			reinterpret_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance)->process(frameSize, *reinterpret_cast<const cv::_InputArray*>(points0), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_TwoPassStabilizer_delete(cv::videostab::TwoPassStabilizer* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_TwoPassStabilizer_TwoPassStabilizer() {
		try {
			cv::videostab::TwoPassStabilizer* ret = new cv::videostab::TwoPassStabilizer();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setMotionStabilizer_Ptr_IMotionStabilizer_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->setMotionStabilizer(*reinterpret_cast<cv::Ptr<cv::videostab::IMotionStabilizer>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_TwoPassStabilizer_motionStabilizer_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::IMotionStabilizer> ret = reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->motionStabilizer();
			return Ok<void*>(new cv::Ptr<cv::videostab::IMotionStabilizer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setWobbleSuppressor_Ptr_WobbleSuppressorBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->setWobbleSuppressor(*reinterpret_cast<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::WobbleSuppressorBase> ret = reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->wobbleSuppressor();
			return Ok<void*>(new cv::Ptr<cv::videostab::WobbleSuppressorBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->setEstimateTrimRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->mustEstimateTrimaRatio();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_videostab_TwoPassStabilizer_reset(void* instance) {
		try {
			reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_TwoPassStabilizer_nextFrame(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::TwoPassStabilizer*>(instance)->nextFrame();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_VideoFileSource_delete(cv::videostab::VideoFileSource* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_VideoFileSource_VideoFileSource_const_StringX_bool(const char* path, bool volatileFrame) {
		try {
			cv::videostab::VideoFileSource* ret = new cv::videostab::VideoFileSource(cv::String(path), volatileFrame);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_VideoFileSource_reset(void* instance) {
		try {
			reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_VideoFileSource_nextFrame(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->nextFrame();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_videostab_VideoFileSource_width(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->width();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_VideoFileSource_height(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->height();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_videostab_VideoFileSource_count(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->count();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_videostab_VideoFileSource_fps(void* instance) {
		try {
			double ret = reinterpret_cast<cv::videostab::VideoFileSource*>(instance)->fps();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_WeightingDeblurer_delete(cv::videostab::WeightingDeblurer* instance) {
		delete instance;
	}
	Result<void*> cv_videostab_WeightingDeblurer_WeightingDeblurer() {
		try {
			cv::videostab::WeightingDeblurer* ret = new cv::videostab::WeightingDeblurer();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_WeightingDeblurer_setSensitivity_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::videostab::WeightingDeblurer*>(instance)->setSensitivity(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_videostab_WeightingDeblurer_sensitivity_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::videostab::WeightingDeblurer*>(instance)->sensitivity();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_videostab_WeightingDeblurer_deblur_int_MatX(void* instance, int idx, void* frame) {
		try {
			reinterpret_cast<cv::videostab::WeightingDeblurer*>(instance)->deblur(idx, *reinterpret_cast<cv::Mat*>(frame));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->setMotionEstimator(*reinterpret_cast<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_WobbleSuppressorBase_motionEstimator_const(void* instance) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->motionEstimator();
			return Ok<void*>(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_suppress_int_const_MatX_MatX(void* instance, int idx, void* frame, void* result) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->suppress(idx, *reinterpret_cast<const cv::Mat*>(frame), *reinterpret_cast<cv::Mat*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setFrameCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->setFrameCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_videostab_WobbleSuppressorBase_frameCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->frameCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->setMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_WobbleSuppressorBase_motions_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->motions();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setMotions2_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->setMotions2(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_WobbleSuppressorBase_motions2_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->motions2();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vector_Mat_X(void* instance, void* val) {
		try {
			reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->setStabilizationMotions(*reinterpret_cast<const std::vector<cv::Mat>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::videostab::WobbleSuppressorBase*>(instance)->stabilizationMotions();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
