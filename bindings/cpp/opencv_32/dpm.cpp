#include "common.hpp"
#include <opencv2/dpm.hpp>
#include "dpm_types.hpp"

extern "C" {
	Result<bool> cv_dpm_DPMDetector_isEmpty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dpm::DPMDetector*>(instance)->isEmpty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dpm_DPMDetector_detect_MatX_vector_ObjectDetection_X(void* instance, void* image, void* objects) {
		try {
			reinterpret_cast<cv::dpm::DPMDetector*>(instance)->detect(*reinterpret_cast<cv::Mat*>(image), *reinterpret_cast<std::vector<cv::dpm::DPMDetector::ObjectDetection>*>(objects));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dpm_DPMDetector_getClassNames_const(void* instance) {
		try {
			std::vector<std::string> ret = reinterpret_cast<cv::dpm::DPMDetector*>(instance)->getClassNames();
			return Ok<void*>(new std::vector<std::string>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_dpm_DPMDetector_getClassCount_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::dpm::DPMDetector*>(instance)->getClassCount();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cv_dpm_DPMDetector_create_const_vector_string_X_const_vector_string_X(void* filenames, void* classNames) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*reinterpret_cast<const std::vector<std::string>*>(filenames), *reinterpret_cast<const std::vector<std::string>*>(classNames));
			return Ok<void*>(new cv::Ptr<cv::dpm::DPMDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_dpm_DPMDetector_ObjectDetection_rect_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->rect;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setRect_Rect(void* instance, const cv::Rect* val) {
		try {
			reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->rect = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dpm_DPMDetector_ObjectDetection_score_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->score;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setScore_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->score = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dpm_DPMDetector_ObjectDetection_classID_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->classID;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dpm_DPMDetector_ObjectDetection_setClassID_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dpm::DPMDetector::ObjectDetection*>(instance)->classID = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DPMDetector_ObjectDetection_delete(cv::dpm::DPMDetector::ObjectDetection* instance) {
		delete instance;
	}
	Result<void*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection() {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectX_float_int(const cv::Rect* rect, float score, int classID) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score, classID);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}
