#include "common.hpp"
#include <opencv2/img_hash.hpp>
#include "img_hash_types.hpp"

extern "C" {
	Result_void cv_img_hash_averageHash_const__InputArrayX_const__OutputArrayX(void* inputArr, void* outputArr) {
		try {
			cv::img_hash::averageHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_blockMeanHash_const__InputArrayX_const__OutputArrayX_int(void* inputArr, void* outputArr, int mode) {
		try {
			cv::img_hash::blockMeanHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr), mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_colorMomentHash_const__InputArrayX_const__OutputArrayX(void* inputArr, void* outputArr) {
		try {
			cv::img_hash::colorMomentHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_marrHildrethHash_const__InputArrayX_const__OutputArrayX_float_float(void* inputArr, void* outputArr, float alpha, float scale) {
		try {
			cv::img_hash::marrHildrethHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr), alpha, scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_pHash_const__InputArrayX_const__OutputArrayX(void* inputArr, void* outputArr) {
		try {
			cv::img_hash::pHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_radialVarianceHash_const__InputArrayX_const__OutputArrayX_double_int(void* inputArr, void* outputArr, double sigma, int numOfAngleLine) {
		try {
			cv::img_hash::radialVarianceHash(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr), sigma, numOfAngleLine);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_AverageHash_delete(cv::img_hash::AverageHash* instance) {
		delete instance;
	}
	Result<void*> cv_img_hash_AverageHash_create() {
		try {
			cv::Ptr<cv::img_hash::AverageHash> ret = cv::img_hash::AverageHash::create();
			return Ok<void*>(new cv::Ptr<cv::img_hash::AverageHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BlockMeanHash_delete(cv::img_hash::BlockMeanHash* instance) {
		delete instance;
	}
	Result_void cv_img_hash_BlockMeanHash_setMode_int(void* instance, int mode) {
		try {
			reinterpret_cast<cv::img_hash::BlockMeanHash*>(instance)->setMode(mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_img_hash_BlockMeanHash_getMean_const(void* instance) {
		try {
			std::vector<double> ret = reinterpret_cast<cv::img_hash::BlockMeanHash*>(instance)->getMean();
			return Ok<void*>(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_img_hash_BlockMeanHash_create_int(int mode) {
		try {
			cv::Ptr<cv::img_hash::BlockMeanHash> ret = cv::img_hash::BlockMeanHash::create(mode);
			return Ok<void*>(new cv::Ptr<cv::img_hash::BlockMeanHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ColorMomentHash_delete(cv::img_hash::ColorMomentHash* instance) {
		delete instance;
	}
	Result<void*> cv_img_hash_ColorMomentHash_create() {
		try {
			cv::Ptr<cv::img_hash::ColorMomentHash> ret = cv::img_hash::ColorMomentHash::create();
			return Ok<void*>(new cv::Ptr<cv::img_hash::ColorMomentHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ImgHashBase_delete(cv::img_hash::ImgHashBase* instance) {
		delete instance;
	}
	Result_void cv_img_hash_ImgHashBase_compute_const__InputArrayX_const__OutputArrayX(void* instance, void* inputArr, void* outputArr) {
		try {
			reinterpret_cast<cv::img_hash::ImgHashBase*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(inputArr), *reinterpret_cast<const cv::_OutputArray*>(outputArr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_img_hash_ImgHashBase_compare_const_const__InputArrayX_const__InputArrayX(void* instance, void* hashOne, void* hashTwo) {
		try {
			double ret = reinterpret_cast<cv::img_hash::ImgHashBase*>(instance)->compare(*reinterpret_cast<const cv::_InputArray*>(hashOne), *reinterpret_cast<const cv::_InputArray*>(hashTwo));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_MarrHildrethHash_delete(cv::img_hash::MarrHildrethHash* instance) {
		delete instance;
	}
	Result<float> cv_img_hash_MarrHildrethHash_getAlpha_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::img_hash::MarrHildrethHash*>(instance)->getAlpha();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_img_hash_MarrHildrethHash_getScale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::img_hash::MarrHildrethHash*>(instance)->getScale();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_img_hash_MarrHildrethHash_setKernelParam_float_float(void* instance, float alpha, float scale) {
		try {
			reinterpret_cast<cv::img_hash::MarrHildrethHash*>(instance)->setKernelParam(alpha, scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_img_hash_MarrHildrethHash_create_float_float(float alpha, float scale) {
		try {
			cv::Ptr<cv::img_hash::MarrHildrethHash> ret = cv::img_hash::MarrHildrethHash::create(alpha, scale);
			return Ok<void*>(new cv::Ptr<cv::img_hash::MarrHildrethHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PHash_delete(cv::img_hash::PHash* instance) {
		delete instance;
	}
	Result<void*> cv_img_hash_PHash_create() {
		try {
			cv::Ptr<cv::img_hash::PHash> ret = cv::img_hash::PHash::create();
			return Ok<void*>(new cv::Ptr<cv::img_hash::PHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_RadialVarianceHash_delete(cv::img_hash::RadialVarianceHash* instance) {
		delete instance;
	}
	Result<void*> cv_img_hash_RadialVarianceHash_create_double_int(double sigma, int numOfAngleLine) {
		try {
			cv::Ptr<cv::img_hash::RadialVarianceHash> ret = cv::img_hash::RadialVarianceHash::create(sigma, numOfAngleLine);
			return Ok<void*>(new cv::Ptr<cv::img_hash::RadialVarianceHash>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getNumOfAngleLine();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_img_hash_RadialVarianceHash_getSigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getSigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->setNumOfAngleLine(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_img_hash_RadialVarianceHash_setSigma_double(void* instance, double value) {
		try {
			reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->setSigma(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_img_hash_RadialVarianceHash_getFeatures(void* instance) {
		try {
			std::vector<double> ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getFeatures();
			return Ok<void*>(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_img_hash_RadialVarianceHash_getHash(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getHash();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatX(void* instance, void* input) {
		try {
			cv::Mat ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getPixPerLine(*reinterpret_cast<const cv::Mat*>(input));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_img_hash_RadialVarianceHash_getProjection(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::img_hash::RadialVarianceHash*>(instance)->getProjection();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
