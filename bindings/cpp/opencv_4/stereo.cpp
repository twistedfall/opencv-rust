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
	
	Result<cv::Point2i> cv_stereo_Match_getPropP0_const(const cv::stereo::Match* instance) {
		try {
			cv::Point2i ret = instance->p0;
			return Ok<cv::Point2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2i>))
	}
	
	Result_void cv_stereo_Match_setPropP0_Point2i(cv::stereo::Match* instance, cv::Point2i* val) {
		try {
			instance->p0 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2i> cv_stereo_Match_getPropP1_const(const cv::stereo::Match* instance) {
		try {
			cv::Point2i ret = instance->p1;
			return Ok<cv::Point2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2i>))
	}
	
	Result_void cv_stereo_Match_setPropP1_Point2i(cv::stereo::Match* instance, cv::Point2i* val) {
		try {
			instance->p1 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_stereo_Match_getPropCorr_const(const cv::stereo::Match* instance) {
		try {
			float ret = instance->corr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_stereo_Match_setPropCorr_float(cv::stereo::Match* instance, float val) {
		try {
			instance->corr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Match_delete(cv::stereo::Match* instance) {
		delete instance;
	}
	Result<int> cv_stereo_PropagationParameters_getPropCorrWinSizeX_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->corrWinSizeX;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropCorrWinSizeX_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->corrWinSizeX = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropCorrWinSizeY_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->corrWinSizeY;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropCorrWinSizeY_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->corrWinSizeY = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropBorderX_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->borderX;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropBorderX_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->borderX = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropBorderY_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->borderY;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropBorderY_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->borderY = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_stereo_PropagationParameters_getPropCorrelationThreshold_const(const cv::stereo::PropagationParameters* instance) {
		try {
			float ret = instance->correlationThreshold;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropCorrelationThreshold_float(cv::stereo::PropagationParameters* instance, float val) {
		try {
			instance->correlationThreshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_stereo_PropagationParameters_getPropTextrureThreshold_const(const cv::stereo::PropagationParameters* instance) {
		try {
			float ret = instance->textrureThreshold;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropTextrureThreshold_float(cv::stereo::PropagationParameters* instance, float val) {
		try {
			instance->textrureThreshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropNeighborhoodSize_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->neighborhoodSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropNeighborhoodSize_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->neighborhoodSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropDisparityGradient_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->disparityGradient;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropDisparityGradient_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->disparityGradient = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropLkTemplateSize_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->lkTemplateSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropLkTemplateSize_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->lkTemplateSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropLkPyrLvl_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->lkPyrLvl;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropLkPyrLvl_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->lkPyrLvl = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropLkTermParam1_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->lkTermParam1;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropLkTermParam1_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->lkTermParam1 = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_stereo_PropagationParameters_getPropLkTermParam2_const(const cv::stereo::PropagationParameters* instance) {
		try {
			float ret = instance->lkTermParam2;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropLkTermParam2_float(cv::stereo::PropagationParameters* instance, float val) {
		try {
			instance->lkTermParam2 = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_stereo_PropagationParameters_getPropGftQualityThres_const(const cv::stereo::PropagationParameters* instance) {
		try {
			float ret = instance->gftQualityThres;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropGftQualityThres_float(cv::stereo::PropagationParameters* instance, float val) {
		try {
			instance->gftQualityThres = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropGftMinSeperationDist_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->gftMinSeperationDist;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropGftMinSeperationDist_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->gftMinSeperationDist = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_stereo_PropagationParameters_getPropGftMaxNumFeatures_const(const cv::stereo::PropagationParameters* instance) {
		try {
			int ret = instance->gftMaxNumFeatures;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stereo_PropagationParameters_setPropGftMaxNumFeatures_int(cv::stereo::PropagationParameters* instance, int val) {
		try {
			instance->gftMaxNumFeatures = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PropagationParameters_delete(cv::stereo::PropagationParameters* instance) {
		delete instance;
	}
	Result<cv::stereo::PropagationParameters*> cv_stereo_QuasiDenseStereo_getPropParam(cv::stereo::QuasiDenseStereo* instance) {
		try {
			cv::stereo::PropagationParameters ret = instance->Param;
			return Ok(new cv::stereo::PropagationParameters(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::stereo::PropagationParameters*>))
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
	
	Result_void cv_stereo_QuasiDenseStereo_getSparseMatches_vector_Match_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::Match>* sMatches) {
		try {
			instance->getSparseMatches(*sMatches);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_QuasiDenseStereo_getDenseMatches_vector_Match_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::Match>* denseMatches) {
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
	
	Result<cv::Mat*> cv_stereo_QuasiDenseStereo_getDisparity_uint8_t(cv::stereo::QuasiDenseStereo* instance, uint8_t disparityLvls) {
		try {
			cv::Mat ret = instance->getDisparity(disparityLvls);
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
