#include "common.hpp"
#include <opencv2/img_hash.hpp>
#include "img_hash_types.hpp"

extern "C" {
	Result_void cv_img_hash_averageHash_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr) {
		try {
			cv::img_hash::averageHash(*inputArr, *outputArr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_blockMeanHash_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, int mode) {
		try {
			cv::img_hash::blockMeanHash(*inputArr, *outputArr, mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_colorMomentHash_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr) {
		try {
			cv::img_hash::colorMomentHash(*inputArr, *outputArr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_marrHildrethHash_const__InputArrayX_const__OutputArrayX_float_float(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, float alpha, float scale) {
		try {
			cv::img_hash::marrHildrethHash(*inputArr, *outputArr, alpha, scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_pHash_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr) {
		try {
			cv::img_hash::pHash(*inputArr, *outputArr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_radialVarianceHash_const__InputArrayX_const__OutputArrayX_double_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, double sigma, int numOfAngleLine) {
		try {
			cv::img_hash::radialVarianceHash(*inputArr, *outputArr, sigma, numOfAngleLine);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_AverageHash_delete(cv::img_hash::AverageHash* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::img_hash::AverageHash>*> cv_img_hash_AverageHash_create() {
		try {
			cv::Ptr<cv::img_hash::AverageHash> ret = cv::img_hash::AverageHash::create();
			return Ok(new cv::Ptr<cv::img_hash::AverageHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::AverageHash>*>)
	}
	
	void cv_BlockMeanHash_delete(cv::img_hash::BlockMeanHash* instance) {
		delete instance;
	}
	Result_void cv_img_hash_BlockMeanHash_setMode_int(cv::img_hash::BlockMeanHash* instance, int mode) {
		try {
			instance->setMode(mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<double>*> cv_img_hash_BlockMeanHash_getMean_const(const cv::img_hash::BlockMeanHash* instance) {
		try {
			std::vector<double> ret = instance->getMean();
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<std::vector<double>*>)
	}
	
	Result<cv::Ptr<cv::img_hash::BlockMeanHash>*> cv_img_hash_BlockMeanHash_create_int(int mode) {
		try {
			cv::Ptr<cv::img_hash::BlockMeanHash> ret = cv::img_hash::BlockMeanHash::create(mode);
			return Ok(new cv::Ptr<cv::img_hash::BlockMeanHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>)
	}
	
	void cv_ColorMomentHash_delete(cv::img_hash::ColorMomentHash* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::img_hash::ColorMomentHash>*> cv_img_hash_ColorMomentHash_create() {
		try {
			cv::Ptr<cv::img_hash::ColorMomentHash> ret = cv::img_hash::ColorMomentHash::create();
			return Ok(new cv::Ptr<cv::img_hash::ColorMomentHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::ColorMomentHash>*>)
	}
	
	void cv_ImgHashBase_delete(cv::img_hash::ImgHashBase* instance) {
		delete instance;
	}
	Result_void cv_img_hash_ImgHashBase_compute_const__InputArrayX_const__OutputArrayX(cv::img_hash::ImgHashBase* instance, const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr) {
		try {
			instance->compute(*inputArr, *outputArr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_img_hash_ImgHashBase_compare_const_const__InputArrayX_const__InputArrayX(const cv::img_hash::ImgHashBase* instance, const cv::_InputArray* hashOne, const cv::_InputArray* hashTwo) {
		try {
			double ret = instance->compare(*hashOne, *hashTwo);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_MarrHildrethHash_delete(cv::img_hash::MarrHildrethHash* instance) {
		delete instance;
	}
	Result<float> cv_img_hash_MarrHildrethHash_getAlpha_const(const cv::img_hash::MarrHildrethHash* instance) {
		try {
			float ret = instance->getAlpha();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_img_hash_MarrHildrethHash_getScale_const(const cv::img_hash::MarrHildrethHash* instance) {
		try {
			float ret = instance->getScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_img_hash_MarrHildrethHash_setKernelParam_float_float(cv::img_hash::MarrHildrethHash* instance, float alpha, float scale) {
		try {
			instance->setKernelParam(alpha, scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*> cv_img_hash_MarrHildrethHash_create_float_float(float alpha, float scale) {
		try {
			cv::Ptr<cv::img_hash::MarrHildrethHash> ret = cv::img_hash::MarrHildrethHash::create(alpha, scale);
			return Ok(new cv::Ptr<cv::img_hash::MarrHildrethHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>)
	}
	
	void cv_PHash_delete(cv::img_hash::PHash* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::img_hash::PHash>*> cv_img_hash_PHash_create() {
		try {
			cv::Ptr<cv::img_hash::PHash> ret = cv::img_hash::PHash::create();
			return Ok(new cv::Ptr<cv::img_hash::PHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::PHash>*>)
	}
	
	void cv_RadialVarianceHash_delete(cv::img_hash::RadialVarianceHash* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*> cv_img_hash_RadialVarianceHash_create_double_int(double sigma, int numOfAngleLine) {
		try {
			cv::Ptr<cv::img_hash::RadialVarianceHash> ret = cv::img_hash::RadialVarianceHash::create(sigma, numOfAngleLine);
			return Ok(new cv::Ptr<cv::img_hash::RadialVarianceHash>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>)
	}
	
	Result<int> cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(const cv::img_hash::RadialVarianceHash* instance) {
		try {
			int ret = instance->getNumOfAngleLine();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_img_hash_RadialVarianceHash_getSigma_const(const cv::img_hash::RadialVarianceHash* instance) {
		try {
			double ret = instance->getSigma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(cv::img_hash::RadialVarianceHash* instance, int value) {
		try {
			instance->setNumOfAngleLine(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_RadialVarianceHash_setSigma_double(cv::img_hash::RadialVarianceHash* instance, double value) {
		try {
			instance->setSigma(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<double>*> cv_img_hash_RadialVarianceHash_getFeatures(cv::img_hash::RadialVarianceHash* instance) {
		try {
			std::vector<double> ret = instance->getFeatures();
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<std::vector<double>*>)
	}
	
	Result<cv::Mat*> cv_img_hash_RadialVarianceHash_getHash(cv::img_hash::RadialVarianceHash* instance) {
		try {
			cv::Mat ret = instance->getHash();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatX(cv::img_hash::RadialVarianceHash* instance, const cv::Mat* input) {
		try {
			cv::Mat ret = instance->getPixPerLine(*input);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_img_hash_RadialVarianceHash_getProjection(cv::img_hash::RadialVarianceHash* instance) {
		try {
			cv::Mat ret = instance->getProjection();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
}
