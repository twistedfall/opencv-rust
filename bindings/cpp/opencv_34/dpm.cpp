#include "ocvrs_common.hpp"
#include <opencv2/dpm.hpp>
#include "dpm_types.hpp"

extern "C" {
	Result<bool> cv_dpm_DPMDetector_isEmpty_const(const cv::dpm::DPMDetector* instance) {
		try {
			bool ret = instance->isEmpty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dpm_DPMDetector_detect_MatR_vector_ObjectDetection_R(cv::dpm::DPMDetector* instance, cv::Mat* image, std::vector<cv::dpm::DPMDetector::ObjectDetection>* objects) {
		try {
			instance->detect(*image, *objects);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const std::vector<std::string>*> cv_dpm_DPMDetector_getClassNames_const(const cv::dpm::DPMDetector* instance) {
		try {
			const std::vector<std::string> ret = instance->getClassNames();
			return Ok(new const std::vector<std::string>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const std::vector<std::string>*>))
	}
	
	Result<size_t> cv_dpm_DPMDetector_getClassCount_const(const cv::dpm::DPMDetector* instance) {
		try {
			size_t ret = instance->getClassCount();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<cv::Ptr<cv::dpm::DPMDetector>*> cv_dpm_DPMDetector_create_const_vector_string_R_const_vector_string_R(const std::vector<std::string>* filenames, const std::vector<std::string>* classNames) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*filenames, *classNames);
			return Ok(new cv::Ptr<cv::dpm::DPMDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dpm::DPMDetector>*>))
	}
	
	Result<cv::Rect> cv_dpm_DPMDetector_ObjectDetection_getPropRect_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			cv::Rect ret = instance->rect;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setPropRect_Rect(cv::dpm::DPMDetector::ObjectDetection* instance, cv::Rect* val) {
		try {
			instance->rect = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dpm_DPMDetector_ObjectDetection_getPropScore_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			float ret = instance->score;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setPropScore_float(cv::dpm::DPMDetector::ObjectDetection* instance, float val) {
		try {
			instance->score = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dpm_DPMDetector_ObjectDetection_getPropClassID_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			int ret = instance->classID;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setPropClassID_int(cv::dpm::DPMDetector::ObjectDetection* instance, int val) {
		try {
			instance->classID = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DPMDetector_ObjectDetection_delete(cv::dpm::DPMDetector::ObjectDetection* instance) {
		delete instance;
	}
	Result<cv::dpm::DPMDetector::ObjectDetection*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection() {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection();
			return Ok<cv::dpm::DPMDetector::ObjectDetection*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dpm::DPMDetector::ObjectDetection*>))
	}
	
	Result<cv::dpm::DPMDetector::ObjectDetection*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(const cv::Rect* rect, float score, int classID) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score, classID);
			return Ok<cv::dpm::DPMDetector::ObjectDetection*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dpm::DPMDetector::ObjectDetection*>))
	}
	
}
