#include "common.hpp"
#include <opencv2/dpm.hpp>
#include "dpm_types.hpp"

extern "C" {
	Result<bool> cv_dpm_DPMDetector_isEmpty_const(const cv::dpm::DPMDetector* instance) {
		try {
			bool ret = instance->isEmpty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dpm_DPMDetector_detect_MatX_vector_ObjectDetection_X(cv::dpm::DPMDetector* instance, cv::Mat* image, std::vector<cv::dpm::DPMDetector::ObjectDetection>* objects) {
		try {
			instance->detect(*image, *objects);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<std::string>*> cv_dpm_DPMDetector_getClassNames_const(const cv::dpm::DPMDetector* instance) {
		try {
			std::vector<std::string> ret = instance->getClassNames();
			return Ok(new std::vector<std::string>(ret));
		} OCVRS_CATCH(Result<std::vector<std::string>*>)
	}
	
	Result<size_t> cv_dpm_DPMDetector_getClassCount_const(const cv::dpm::DPMDetector* instance) {
		try {
			size_t ret = instance->getClassCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<cv::Ptr<cv::dpm::DPMDetector>*> cv_dpm_DPMDetector_create_const_vector_string_X_const_vector_string_X(const std::vector<std::string>* filenames, const std::vector<std::string>* classNames) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*filenames, *classNames);
			return Ok(new cv::Ptr<cv::dpm::DPMDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dpm::DPMDetector>*>)
	}
	
	Result<cv::Rect> cv_dpm_DPMDetector_ObjectDetection_rect_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			cv::Rect ret = instance->rect;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setRect_Rect(cv::dpm::DPMDetector::ObjectDetection* instance, const cv::Rect* val) {
		try {
			instance->rect = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dpm_DPMDetector_ObjectDetection_score_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			float ret = instance->score;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setScore_float(cv::dpm::DPMDetector::ObjectDetection* instance, float val) {
		try {
			instance->score = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dpm_DPMDetector_ObjectDetection_classID_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
		try {
			int ret = instance->classID;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setClassID_int(cv::dpm::DPMDetector::ObjectDetection* instance, int val) {
		try {
			instance->classID = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DPMDetector_ObjectDetection_delete(cv::dpm::DPMDetector::ObjectDetection* instance) {
		delete instance;
	}
	Result<cv::dpm::DPMDetector::ObjectDetection*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection() {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dpm::DPMDetector::ObjectDetection*>)
	}
	
	Result<cv::dpm::DPMDetector::ObjectDetection*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectX_float_int(const cv::Rect* rect, float score, int classID) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score, classID);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dpm::DPMDetector::ObjectDetection*>)
	}
	
}
