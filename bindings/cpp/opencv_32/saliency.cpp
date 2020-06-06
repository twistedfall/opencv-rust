#include "common.hpp"
#include <opencv2/saliency.hpp>
#include "saliency_types.hpp"

extern "C" {
	void cv_MotionSaliencyBinWangApr2014_delete(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
		delete instance;
	}
	Result<cv::saliency::MotionSaliencyBinWangApr2014*> cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014() {
		try {
			cv::saliency::MotionSaliencyBinWangApr2014* ret = new cv::saliency::MotionSaliencyBinWangApr2014();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::saliency::MotionSaliencyBinWangApr2014*>))
	}
	
	Result<cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>*> cv_saliency_MotionSaliencyBinWangApr2014_create() {
		try {
			cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014> ret = cv::saliency::MotionSaliencyBinWangApr2014::create();
			return Ok(new cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>*>))
	}
	
	Result<bool> cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayX_const__OutputArrayX(cv::saliency::MotionSaliencyBinWangApr2014* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int W, int H) {
		try {
			instance->setImagesize(W, H);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_saliency_MotionSaliencyBinWangApr2014_init(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
		try {
			bool ret = instance->init();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(const cv::saliency::MotionSaliencyBinWangApr2014* instance) {
		try {
			int ret = instance->getImageWidth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int val) {
		try {
			instance->setImageWidth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(const cv::saliency::MotionSaliencyBinWangApr2014* instance) {
		try {
			int ret = instance->getImageHeight();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int val) {
		try {
			instance->setImageHeight(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ObjectnessBING_delete(cv::saliency::ObjectnessBING* instance) {
		delete instance;
	}
	Result<cv::saliency::ObjectnessBING*> cv_saliency_ObjectnessBING_ObjectnessBING() {
		try {
			cv::saliency::ObjectnessBING* ret = new cv::saliency::ObjectnessBING();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::saliency::ObjectnessBING*>))
	}
	
	Result<cv::Ptr<cv::saliency::ObjectnessBING>*> cv_saliency_ObjectnessBING_create() {
		try {
			cv::Ptr<cv::saliency::ObjectnessBING> ret = cv::saliency::ObjectnessBING::create();
			return Ok(new cv::Ptr<cv::saliency::ObjectnessBING>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::saliency::ObjectnessBING>*>))
	}
	
	Result<bool> cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayX_const__OutputArrayX(cv::saliency::ObjectnessBING* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_saliency_ObjectnessBING_read(cv::saliency::ObjectnessBING* instance) {
		try {
			instance->read();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_saliency_ObjectnessBING_write_const(const cv::saliency::ObjectnessBING* instance) {
		try {
			instance->write();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<float>*> cv_saliency_ObjectnessBING_getobjectnessValues(cv::saliency::ObjectnessBING* instance) {
		try {
			std::vector<float> ret = instance->getobjectnessValues();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result_void cv_saliency_ObjectnessBING_setTrainingPath_const_StringX(cv::saliency::ObjectnessBING* instance, const char* trainingPath) {
		try {
			instance->setTrainingPath(cv::String(trainingPath));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_saliency_ObjectnessBING_setBBResDir_const_StringX(cv::saliency::ObjectnessBING* instance, const char* resultsDir) {
		try {
			instance->setBBResDir(cv::String(resultsDir));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_saliency_ObjectnessBING_getBase_const(const cv::saliency::ObjectnessBING* instance) {
		try {
			double ret = instance->getBase();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_saliency_ObjectnessBING_setBase_double(cv::saliency::ObjectnessBING* instance, double val) {
		try {
			instance->setBase(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_saliency_ObjectnessBING_getNSS_const(const cv::saliency::ObjectnessBING* instance) {
		try {
			int ret = instance->getNSS();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_ObjectnessBING_setNSS_int(cv::saliency::ObjectnessBING* instance, int val) {
		try {
			instance->setNSS(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_saliency_ObjectnessBING_getW_const(const cv::saliency::ObjectnessBING* instance) {
		try {
			int ret = instance->getW();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_ObjectnessBING_setW_int(cv::saliency::ObjectnessBING* instance, int val) {
		try {
			instance->setW(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::saliency::Saliency>*> cv_saliency_Saliency_create_const_StringX(const char* saliencyType) {
		try {
			cv::Ptr<cv::saliency::Saliency> ret = cv::saliency::Saliency::create(cv::String(saliencyType));
			return Ok(new cv::Ptr<cv::saliency::Saliency>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::saliency::Saliency>*>))
	}
	
	Result<bool> cv_saliency_Saliency_computeSaliency_const__InputArrayX_const__OutputArrayX(cv::saliency::Saliency* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_saliency_Saliency_getClassName_const(const cv::saliency::Saliency* instance) {
		try {
			cv::String ret = instance->getClassName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayX_const__OutputArrayX(cv::saliency::StaticSaliency* instance, const cv::_InputArray* _saliencyMap, const cv::_OutputArray* _binaryMap) {
		try {
			bool ret = instance->computeBinaryMap(*_saliencyMap, *_binaryMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_StaticSaliencyFineGrained_delete(cv::saliency::StaticSaliencyFineGrained* instance) {
		delete instance;
	}
	Result<cv::saliency::StaticSaliencyFineGrained*> cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained() {
		try {
			cv::saliency::StaticSaliencyFineGrained* ret = new cv::saliency::StaticSaliencyFineGrained();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::saliency::StaticSaliencyFineGrained*>))
	}
	
	Result<cv::Ptr<cv::saliency::StaticSaliencyFineGrained>*> cv_saliency_StaticSaliencyFineGrained_create() {
		try {
			cv::Ptr<cv::saliency::StaticSaliencyFineGrained> ret = cv::saliency::StaticSaliencyFineGrained::create();
			return Ok(new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::saliency::StaticSaliencyFineGrained>*>))
	}
	
	Result<bool> cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayX_const__OutputArrayX(cv::saliency::StaticSaliencyFineGrained* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_StaticSaliencySpectralResidual_delete(cv::saliency::StaticSaliencySpectralResidual* instance) {
		delete instance;
	}
	Result<cv::saliency::StaticSaliencySpectralResidual*> cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual() {
		try {
			cv::saliency::StaticSaliencySpectralResidual* ret = new cv::saliency::StaticSaliencySpectralResidual();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::saliency::StaticSaliencySpectralResidual*>))
	}
	
	Result<cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>*> cv_saliency_StaticSaliencySpectralResidual_create() {
		try {
			cv::Ptr<cv::saliency::StaticSaliencySpectralResidual> ret = cv::saliency::StaticSaliencySpectralResidual::create();
			return Ok(new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>*>))
	}
	
	Result<bool> cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayX_const__OutputArrayX(cv::saliency::StaticSaliencySpectralResidual* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeX(cv::saliency::StaticSaliencySpectralResidual* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageX(const cv::saliency::StaticSaliencySpectralResidual* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(const cv::saliency::StaticSaliencySpectralResidual* instance) {
		try {
			int ret = instance->getImageWidth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(cv::saliency::StaticSaliencySpectralResidual* instance, int val) {
		try {
			instance->setImageWidth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(const cv::saliency::StaticSaliencySpectralResidual* instance) {
		try {
			int ret = instance->getImageHeight();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(cv::saliency::StaticSaliencySpectralResidual* instance, int val) {
		try {
			instance->setImageHeight(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
