#include "common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	Result_void cv_xobjdetect_WBDetector_read_const_FileNodeX(void* instance, void* node) {
		try {
			reinterpret_cast<cv::xobjdetect::WBDetector*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(node));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::xobjdetect::WBDetector*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_train_const_stringX_const_stringX(void* instance, const char* pos_samples, const char* neg_imgs) {
		try {
			reinterpret_cast<cv::xobjdetect::WBDetector*>(instance)->train(std::string(pos_samples), std::string(neg_imgs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xobjdetect_WBDetector_detect_const_MatX_vector_Rect_X_vector_double_X(void* instance, void* img, void* bboxes, void* confidences) {
		try {
			reinterpret_cast<cv::xobjdetect::WBDetector*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<std::vector<cv::Rect>*>(bboxes), *reinterpret_cast<std::vector<double>*>(confidences));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xobjdetect_WBDetector_create() {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			return Ok<void*>(new cv::Ptr<cv::xobjdetect::WBDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
