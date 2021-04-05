#include "ocvrs_common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	Result_void cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* image1, const cv::Mat* image2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type) {
		try {
			cv::stereo::censusTransform(*image1, *image2, kernelSize, *dist1, *dist2, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_censusTransform_const_MatR_int_MatR_const_int(const cv::Mat* image1, int kernelSize, cv::Mat* dist1, const int type) {
		try {
			cv::stereo::censusTransform(*image1, kernelSize, *dist1, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, int t, const cv::Mat* integralImage1, const cv::Mat* integralImage2) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type, t, *integralImage1, *integralImage2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, int t, const cv::Mat* integralImage) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type, t, *integralImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2) {
		try {
			cv::stereo::starCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_starCensusTransform_const_MatR_int_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist) {
		try {
			cv::stereo::starCensusTransform(*img1, kernelSize, *dist);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type) {
		try {
			cv::stereo::symetricCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist1, const int type) {
		try {
			cv::stereo::symetricCensusTransform(*img1, kernelSize, *dist1, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::stereo::MatchQuasiDense> cv_stereo_MatchQuasiDense_MatchQuasiDense() {
		try {
			cv::stereo::MatchQuasiDense ret;
			return Ok<cv::stereo::MatchQuasiDense>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::stereo::MatchQuasiDense>))
	}
	
	Result<cv::stereo::PropagationParameters> cv_stereo_QuasiDenseStereo_getPropParam_const(const cv::stereo::QuasiDenseStereo* instance) {
		try {
			cv::stereo::PropagationParameters ret = instance->Param;
			return Ok<cv::stereo::PropagationParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::stereo::PropagationParameters>))
	}
	
	Result_void cv_stereo_QuasiDenseStereo_setPropParam_PropagationParameters(cv::stereo::QuasiDenseStereo* instance, cv::stereo::PropagationParameters* val) {
		try {
			instance->Param = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_QuasiDenseStereo_loadParameters_String(cv::stereo::QuasiDenseStereo* instance, char* filepath) {
		try {
			int ret = instance->loadParameters(std::string(filepath));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_stereo_QuasiDenseStereo_saveParameters_String(cv::stereo::QuasiDenseStereo* instance, char* filepath) {
		try {
			int ret = instance->saveParameters(std::string(filepath));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_QuasiDenseStereo_getSparseMatches_vector_MatchQuasiDense_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* sMatches) {
		try {
			instance->getSparseMatches(*sMatches);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_QuasiDenseStereo_getDenseMatches_vector_MatchQuasiDense_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* denseMatches) {
		try {
			instance->getDenseMatches(*denseMatches);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(cv::stereo::QuasiDenseStereo* instance, const cv::Mat* imgLeft, const cv::Mat* imgRight) {
		try {
			instance->process(*imgLeft, *imgRight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2f> cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(cv::stereo::QuasiDenseStereo* instance, const int x, const int y) {
		try {
			cv::Point2f ret = instance->getMatch(x, y);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<cv::Mat*> cv_stereo_QuasiDenseStereo_getDisparity(cv::stereo::QuasiDenseStereo* instance) {
		try {
			cv::Mat ret = instance->getDisparity();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*> cv_stereo_QuasiDenseStereo_create_Size_String(cv::Size* monoImgSize, char* paramFilepath) {
		try {
			cv::Ptr<cv::stereo::QuasiDenseStereo> ret = cv::stereo::QuasiDenseStereo::create(*monoImgSize, std::string(paramFilepath));
			return Ok(new cv::Ptr<cv::stereo::QuasiDenseStereo>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>))
	}
	
}
