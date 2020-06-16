#include "common.hpp"
#include <opencv2/quality.hpp>
#include "quality_types.hpp"

extern "C" {
	void cv_QualityBRISQUE_delete(cv::quality::QualityBRISQUE* instance) {
		delete instance;
	}
	Result<cv::Scalar> cv_quality_QualityBRISQUE_compute_const__InputArrayR(cv::quality::QualityBRISQUE* instance, const cv::_InputArray* img) {
		try {
			cv::Scalar ret = instance->compute(*img);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<cv::Ptr<cv::quality::QualityBRISQUE>*> cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(const char* model_file_path, const char* range_file_path) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(std::string(model_file_path), std::string(range_file_path));
			return Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityBRISQUE>*>))
	}
	
	Result<cv::Ptr<cv::quality::QualityBRISQUE>*> cv_quality_QualityBRISQUE_create_const_Ptr_SVM_R_const_MatR(const cv::Ptr<cv::ml::SVM>* model, const cv::Mat* range) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(*model, *range);
			return Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityBRISQUE>*>))
	}
	
	Result<cv::Scalar> cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(const cv::_InputArray* img, const char* model_file_path, const char* range_file_path) {
		try {
			cv::Scalar ret = cv::quality::QualityBRISQUE::compute(*img, std::string(model_file_path), std::string(range_file_path));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_OutputArray* features) {
		try {
			cv::quality::QualityBRISQUE::computeFeatures(*img, *features);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_quality_QualityBase_compute_const__InputArrayR(cv::quality::QualityBase* instance, const cv::_InputArray* img) {
		try {
			cv::Scalar ret = instance->compute(*img);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(const cv::quality::QualityBase* instance, const cv::_OutputArray* dst) {
		try {
			instance->getQualityMap(*dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_quality_QualityBase_clear(cv::quality::QualityBase* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_quality_QualityBase_empty_const(const cv::quality::QualityBase* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_QualityGMSD_delete(cv::quality::QualityGMSD* instance) {
		delete instance;
	}
	Result<cv::Scalar> cv_quality_QualityGMSD_compute_const__InputArrayR(cv::quality::QualityGMSD* instance, const cv::_InputArray* cmp) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<bool> cv_quality_QualityGMSD_empty_const(const cv::quality::QualityGMSD* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_quality_QualityGMSD_clear(cv::quality::QualityGMSD* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::quality::QualityGMSD>*> cv_quality_QualityGMSD_create_const__InputArrayR(const cv::_InputArray* ref) {
		try {
			cv::Ptr<cv::quality::QualityGMSD> ret = cv::quality::QualityGMSD::create(*ref);
			return Ok(new cv::Ptr<cv::quality::QualityGMSD>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityGMSD>*>))
	}
	
	Result<cv::Scalar> cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap) {
		try {
			cv::Scalar ret = cv::quality::QualityGMSD::compute(*ref, *cmp, *qualityMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	void cv_QualityMSE_delete(cv::quality::QualityMSE* instance) {
		delete instance;
	}
	Result<cv::Scalar> cv_quality_QualityMSE_compute_const__InputArrayR(cv::quality::QualityMSE* instance, const cv::_InputArray* cmpImgs) {
		try {
			cv::Scalar ret = instance->compute(*cmpImgs);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<bool> cv_quality_QualityMSE_empty_const(const cv::quality::QualityMSE* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_quality_QualityMSE_clear(cv::quality::QualityMSE* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::quality::QualityMSE>*> cv_quality_QualityMSE_create_const__InputArrayR(const cv::_InputArray* ref) {
		try {
			cv::Ptr<cv::quality::QualityMSE> ret = cv::quality::QualityMSE::create(*ref);
			return Ok(new cv::Ptr<cv::quality::QualityMSE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityMSE>*>))
	}
	
	Result<cv::Scalar> cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap) {
		try {
			cv::Scalar ret = cv::quality::QualityMSE::compute(*ref, *cmp, *qualityMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	void cv_QualityPSNR_delete(cv::quality::QualityPSNR* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::quality::QualityPSNR>*> cv_quality_QualityPSNR_create_const__InputArrayR_double(const cv::_InputArray* ref, double maxPixelValue) {
		try {
			cv::Ptr<cv::quality::QualityPSNR> ret = cv::quality::QualityPSNR::create(*ref, maxPixelValue);
			return Ok(new cv::Ptr<cv::quality::QualityPSNR>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityPSNR>*>))
	}
	
	Result<cv::Scalar> cv_quality_QualityPSNR_compute_const__InputArrayR(cv::quality::QualityPSNR* instance, const cv::_InputArray* cmp) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<bool> cv_quality_QualityPSNR_empty_const(const cv::quality::QualityPSNR* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_quality_QualityPSNR_clear(cv::quality::QualityPSNR* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, double maxPixelValue) {
		try {
			cv::Scalar ret = cv::quality::QualityPSNR::compute(*ref, *cmp, *qualityMap, maxPixelValue);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<double> cv_quality_QualityPSNR_getMaxPixelValue_const(const cv::quality::QualityPSNR* instance) {
		try {
			double ret = instance->getMaxPixelValue();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_quality_QualityPSNR_setMaxPixelValue_double(cv::quality::QualityPSNR* instance, double val) {
		try {
			instance->setMaxPixelValue(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_QualitySSIM_delete(cv::quality::QualitySSIM* instance) {
		delete instance;
	}
	Result<cv::Scalar> cv_quality_QualitySSIM_compute_const__InputArrayR(cv::quality::QualitySSIM* instance, const cv::_InputArray* cmp) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<bool> cv_quality_QualitySSIM_empty_const(const cv::quality::QualitySSIM* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_quality_QualitySSIM_clear(cv::quality::QualitySSIM* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::quality::QualitySSIM>*> cv_quality_QualitySSIM_create_const__InputArrayR(const cv::_InputArray* ref) {
		try {
			cv::Ptr<cv::quality::QualitySSIM> ret = cv::quality::QualitySSIM::create(*ref);
			return Ok(new cv::Ptr<cv::quality::QualitySSIM>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualitySSIM>*>))
	}
	
	Result<cv::Scalar> cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap) {
		try {
			cv::Scalar ret = cv::quality::QualitySSIM::compute(*ref, *cmp, *qualityMap);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
}
