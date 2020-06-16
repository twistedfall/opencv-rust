#include "common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	Result_void cv_xobjdetect_WBDetector_read_const_FileNodeR(cv::xobjdetect::WBDetector* instance, const cv::FileNode* node) {
		try {
			instance->read(*node);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xobjdetect_WBDetector_write_const_FileStorageR(const cv::xobjdetect::WBDetector* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(cv::xobjdetect::WBDetector* instance, const char* pos_samples, const char* neg_imgs) {
		try {
			instance->train(std::string(pos_samples), std::string(neg_imgs));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xobjdetect_WBDetector_detect_const_MatR_vector_Rect_R_vector_double_R(cv::xobjdetect::WBDetector* instance, const cv::Mat* img, std::vector<cv::Rect>* bboxes, std::vector<double>* confidences) {
		try {
			instance->detect(*img, *bboxes, *confidences);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xobjdetect::WBDetector>*> cv_xobjdetect_WBDetector_create() {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			return Ok(new cv::Ptr<cv::xobjdetect::WBDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>))
	}
	
}
