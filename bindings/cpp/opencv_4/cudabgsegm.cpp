#include "common.hpp"
#include <opencv2/cudabgsegm.hpp>
#include "cudabgsegm_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>*> cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG2> ret = cv::cuda::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			return Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>*>))
	}
	
	Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG>*> cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG> ret = cv::cuda::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			return Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG>*>))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(cv::cuda::BackgroundSubtractorMOG* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, cv::cuda::Stream* stream) {
		try {
			instance->apply(*image, *fgmask, learningRate, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(const cv::cuda::BackgroundSubtractorMOG* instance, const cv::_OutputArray* backgroundImage, cv::cuda::Stream* stream) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_GpuMatR_StreamR(cv::cuda::BackgroundSubtractorMOG* instance, cv::cuda::GpuMat* backgroundImage, cv::cuda::Stream* stream) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_BackgroundSubtractorMOG_getHistory_const(const cv::cuda::BackgroundSubtractorMOG* instance) {
		try {
			int ret = instance->getHistory();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_setHistory_int(cv::cuda::BackgroundSubtractorMOG* instance, int nframes) {
		try {
			instance->setHistory(nframes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(const cv::cuda::BackgroundSubtractorMOG* instance) {
		try {
			int ret = instance->getNMixtures();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(cv::cuda::BackgroundSubtractorMOG* instance, int nmix) {
		try {
			instance->setNMixtures(nmix);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(const cv::cuda::BackgroundSubtractorMOG* instance) {
		try {
			double ret = instance->getBackgroundRatio();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(cv::cuda::BackgroundSubtractorMOG* instance, double backgroundRatio) {
		try {
			instance->setBackgroundRatio(backgroundRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(const cv::cuda::BackgroundSubtractorMOG* instance) {
		try {
			double ret = instance->getNoiseSigma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG_setNoiseSigma_double(cv::cuda::BackgroundSubtractorMOG* instance, double noiseSigma) {
		try {
			instance->setNoiseSigma(noiseSigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(cv::cuda::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, cv::cuda::Stream* stream) {
		try {
			instance->apply(*image, *fgmask, learningRate, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(const cv::cuda::BackgroundSubtractorMOG2* instance, const cv::_OutputArray* backgroundImage, cv::cuda::Stream* stream) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_GpuMatR_StreamR(cv::cuda::BackgroundSubtractorMOG2* instance, cv::cuda::GpuMat* backgroundImage, cv::cuda::Stream* stream) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
