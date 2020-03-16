#include "common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	Result_void cv_xobjdetect_WBDetector_read_const_FileNodeX(cv::xobjdetect::WBDetector* instance, const cv::FileNode* node) {
		try {
			instance->read(*node);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_write_const_FileStorageX(const cv::xobjdetect::WBDetector* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_train_const_stringX_const_stringX(cv::xobjdetect::WBDetector* instance, const char* pos_samples, const char* neg_imgs) {
		try {
			instance->train(std::string(pos_samples), std::string(neg_imgs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_detect_const_MatX_vector_Rect_X_vector_double_X(cv::xobjdetect::WBDetector* instance, const cv::Mat* img, std::vector<cv::Rect>* bboxes, std::vector<double>* confidences) {
		try {
			instance->detect(*img, *bboxes, *confidences);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::xobjdetect::WBDetector>*> cv_xobjdetect_WBDetector_create() {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			return Ok(new cv::Ptr<cv::xobjdetect::WBDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>)
	}
	
}
