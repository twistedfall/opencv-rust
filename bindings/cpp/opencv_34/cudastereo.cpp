#include "ocvrs_common.hpp"
#include <opencv2/cudastereo.hpp>
#include "cudastereo_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*> cv_cuda_createDisparityBilateralFilter_int_int_int(int ndisp, int radius, int iters) {
		try {
			cv::Ptr<cv::cuda::DisparityBilateralFilter> ret = cv::cuda::createDisparityBilateralFilter(ndisp, radius, iters);
			return Ok(new cv::Ptr<cv::cuda::DisparityBilateralFilter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::StereoBM>*> cv_cuda_createStereoBM_int_int(int numDisparities, int blockSize) {
		try {
			cv::Ptr<cv::cuda::StereoBM> ret = cv::cuda::createStereoBM(numDisparities, blockSize);
			return Ok(new cv::Ptr<cv::cuda::StereoBM>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::StereoBM>*>))
	}
	
	Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*> cv_cuda_createStereoBeliefPropagation_int_int_int_int(int ndisp, int iters, int levels, int msg_type) {
		try {
			cv::Ptr<cv::cuda::StereoBeliefPropagation> ret = cv::cuda::createStereoBeliefPropagation(ndisp, iters, levels, msg_type);
			return Ok(new cv::Ptr<cv::cuda::StereoBeliefPropagation>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*>))
	}
	
	Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*> cv_cuda_createStereoConstantSpaceBP_int_int_int_int_int(int ndisp, int iters, int levels, int nr_plane, int msg_type) {
		try {
			cv::Ptr<cv::cuda::StereoConstantSpaceBP> ret = cv::cuda::createStereoConstantSpaceBP(ndisp, iters, levels, nr_plane, msg_type);
			return Ok(new cv::Ptr<cv::cuda::StereoConstantSpaceBP>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*>))
	}
	
	Result_void cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src_disp, const cv::_OutputArray* dst_disp, int ndisp, cv::cuda::Stream* stream) {
		try {
			cv::cuda::drawColorDisp(*src_disp, *dst_disp, ndisp, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* disp, const cv::_OutputArray* xyzw, const cv::_InputArray* Q, int dst_cn, cv::cuda::Stream* stream) {
		try {
			cv::cuda::reprojectImageTo3D(*disp, *xyzw, *Q, dst_cn, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::DisparityBilateralFilter* instance, const cv::_InputArray* disparity, const cv::_InputArray* image, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->apply(*disparity, *image, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_DisparityBilateralFilter_getNumDisparities_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			int ret = instance->getNumDisparities();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setNumDisparities_int(cv::cuda::DisparityBilateralFilter* instance, int numDisparities) {
		try {
			instance->setNumDisparities(numDisparities);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_DisparityBilateralFilter_getRadius_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			int ret = instance->getRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setRadius_int(cv::cuda::DisparityBilateralFilter* instance, int radius) {
		try {
			instance->setRadius(radius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_DisparityBilateralFilter_getNumIters_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setNumIters_int(cv::cuda::DisparityBilateralFilter* instance, int iters) {
		try {
			instance->setNumIters(iters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_DisparityBilateralFilter_getEdgeThreshold_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			double ret = instance->getEdgeThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setEdgeThreshold_double(cv::cuda::DisparityBilateralFilter* instance, double edge_threshold) {
		try {
			instance->setEdgeThreshold(edge_threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_DisparityBilateralFilter_getMaxDiscThreshold_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			double ret = instance->getMaxDiscThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setMaxDiscThreshold_double(cv::cuda::DisparityBilateralFilter* instance, double max_disc_threshold) {
		try {
			instance->setMaxDiscThreshold(max_disc_threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_DisparityBilateralFilter_getSigmaRange_const(const cv::cuda::DisparityBilateralFilter* instance) {
		try {
			double ret = instance->getSigmaRange();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_DisparityBilateralFilter_setSigmaRange_double(cv::cuda::DisparityBilateralFilter* instance, double sigma_range) {
		try {
			instance->setSigmaRange(sigma_range);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_StereoBM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBM* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, cv::cuda::Stream* stream) {
		try {
			instance->compute(*left, *right, *disparity, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBeliefPropagation* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, cv::cuda::Stream* stream) {
		try {
			instance->compute(*left, *right, *disparity, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBeliefPropagation* instance, const cv::_InputArray* data, const cv::_OutputArray* disparity, cv::cuda::Stream* stream) {
		try {
			instance->compute(*data, *disparity, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_StereoBeliefPropagation_getNumIters_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			int ret = instance->getNumIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setNumIters_int(cv::cuda::StereoBeliefPropagation* instance, int iters) {
		try {
			instance->setNumIters(iters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_StereoBeliefPropagation_getNumLevels_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			int ret = instance->getNumLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setNumLevels_int(cv::cuda::StereoBeliefPropagation* instance, int levels) {
		try {
			instance->setNumLevels(levels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_StereoBeliefPropagation_getMaxDataTerm_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			double ret = instance->getMaxDataTerm();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setMaxDataTerm_double(cv::cuda::StereoBeliefPropagation* instance, double max_data_term) {
		try {
			instance->setMaxDataTerm(max_data_term);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_StereoBeliefPropagation_getDataWeight_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			double ret = instance->getDataWeight();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setDataWeight_double(cv::cuda::StereoBeliefPropagation* instance, double data_weight) {
		try {
			instance->setDataWeight(data_weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_StereoBeliefPropagation_getMaxDiscTerm_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			double ret = instance->getMaxDiscTerm();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setMaxDiscTerm_double(cv::cuda::StereoBeliefPropagation* instance, double max_disc_term) {
		try {
			instance->setMaxDiscTerm(max_disc_term);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_StereoBeliefPropagation_getDiscSingleJump_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			double ret = instance->getDiscSingleJump();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setDiscSingleJump_double(cv::cuda::StereoBeliefPropagation* instance, double disc_single_jump) {
		try {
			instance->setDiscSingleJump(disc_single_jump);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_StereoBeliefPropagation_getMsgType_const(const cv::cuda::StereoBeliefPropagation* instance) {
		try {
			int ret = instance->getMsgType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_setMsgType_int(cv::cuda::StereoBeliefPropagation* instance, int msg_type) {
		try {
			instance->setMsgType(msg_type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_StereoBeliefPropagation_estimateRecommendedParams_int_int_intR_intR_intR(int width, int height, int* ndisp, int* iters, int* levels) {
		try {
			cv::cuda::StereoBeliefPropagation::estimateRecommendedParams(width, height, *ndisp, *iters, *levels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_StereoConstantSpaceBP_getNrPlane_const(const cv::cuda::StereoConstantSpaceBP* instance) {
		try {
			int ret = instance->getNrPlane();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_StereoConstantSpaceBP_setNrPlane_int(cv::cuda::StereoConstantSpaceBP* instance, int nr_plane) {
		try {
			instance->setNrPlane(nr_plane);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_StereoConstantSpaceBP_getUseLocalInitDataCost_const(const cv::cuda::StereoConstantSpaceBP* instance) {
		try {
			bool ret = instance->getUseLocalInitDataCost();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_StereoConstantSpaceBP_setUseLocalInitDataCost_bool(cv::cuda::StereoConstantSpaceBP* instance, bool use_local_init_data_cost) {
		try {
			instance->setUseLocalInitDataCost(use_local_init_data_cost);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_StereoConstantSpaceBP_estimateRecommendedParams_int_int_intR_intR_intR_intR(int width, int height, int* ndisp, int* iters, int* levels, int* nr_plane) {
		try {
			cv::cuda::StereoConstantSpaceBP::estimateRecommendedParams(width, height, *ndisp, *iters, *levels, *nr_plane);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
