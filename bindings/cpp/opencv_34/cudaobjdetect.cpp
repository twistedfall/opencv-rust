#include "common.hpp"
#include <opencv2/cudaobjdetect.hpp>
#include "cudaobjdetect_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cuda::CascadeClassifier>*> cv_cuda_CascadeClassifier_create_const_StringR(const char* filename) {
		try {
			cv::Ptr<cv::cuda::CascadeClassifier> ret = cv::cuda::CascadeClassifier::create(cv::String(filename));
			return Ok(new cv::Ptr<cv::cuda::CascadeClassifier>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CascadeClassifier>*>))
	}
	
	Result<cv::Ptr<cv::cuda::CascadeClassifier>*> cv_cuda_CascadeClassifier_create_const_FileStorageR(const cv::FileStorage* file) {
		try {
			cv::Ptr<cv::cuda::CascadeClassifier> ret = cv::cuda::CascadeClassifier::create(*file);
			return Ok(new cv::Ptr<cv::cuda::CascadeClassifier>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CascadeClassifier>*>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setMaxObjectSize_Size(cv::cuda::CascadeClassifier* instance, const cv::Size* maxObjectSize) {
		try {
			instance->setMaxObjectSize(*maxObjectSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_cuda_CascadeClassifier_getMaxObjectSize_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setMinObjectSize_Size(cv::cuda::CascadeClassifier* instance, const cv::Size* minSize) {
		try {
			instance->setMinObjectSize(*minSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_cuda_CascadeClassifier_getMinObjectSize_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setScaleFactor_double(cv::cuda::CascadeClassifier* instance, double scaleFactor) {
		try {
			instance->setScaleFactor(scaleFactor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_CascadeClassifier_getScaleFactor_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			double ret = instance->getScaleFactor();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setMinNeighbors_int(cv::cuda::CascadeClassifier* instance, int minNeighbors) {
		try {
			instance->setMinNeighbors(minNeighbors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_CascadeClassifier_getMinNeighbors_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			int ret = instance->getMinNeighbors();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setFindLargestObject_bool(cv::cuda::CascadeClassifier* instance, bool findLargestObject) {
		try {
			instance->setFindLargestObject(findLargestObject);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_CascadeClassifier_getFindLargestObject(cv::cuda::CascadeClassifier* instance) {
		try {
			bool ret = instance->getFindLargestObject();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_CascadeClassifier_setMaxNumObjects_int(cv::cuda::CascadeClassifier* instance, int maxNumObjects) {
		try {
			instance->setMaxNumObjects(maxNumObjects);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_CascadeClassifier_getMaxNumObjects_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			int ret = instance->getMaxNumObjects();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Size> cv_cuda_CascadeClassifier_getClassifierSize_const(const cv::cuda::CascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getClassifierSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CascadeClassifier* instance, const cv::_InputArray* image, const cv::_OutputArray* objects, cv::cuda::Stream* stream) {
		try {
			instance->detectMultiScale(*image, *objects, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CascadeClassifier_convert_const__OutputArrayR_vector_Rect_R(cv::cuda::CascadeClassifier* instance, const cv::_OutputArray* gpu_objects, std::vector<cv::Rect>* objects) {
		try {
			instance->convert(*gpu_objects, *objects);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::HOG>*> cv_cuda_HOG_create_Size_Size_Size_Size_int(const cv::Size* win_size, const cv::Size* block_size, const cv::Size* block_stride, const cv::Size* cell_size, int nbins) {
		try {
			cv::Ptr<cv::cuda::HOG> ret = cv::cuda::HOG::create(*win_size, *block_size, *block_stride, *cell_size, nbins);
			return Ok(new cv::Ptr<cv::cuda::HOG>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::HOG>*>))
	}
	
	Result_void cv_cuda_HOG_setWinSigma_double(cv::cuda::HOG* instance, double win_sigma) {
		try {
			instance->setWinSigma(win_sigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_HOG_getWinSigma_const(const cv::cuda::HOG* instance) {
		try {
			double ret = instance->getWinSigma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_HOG_setL2HysThreshold_double(cv::cuda::HOG* instance, double threshold_L2hys) {
		try {
			instance->setL2HysThreshold(threshold_L2hys);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_HOG_getL2HysThreshold_const(const cv::cuda::HOG* instance) {
		try {
			double ret = instance->getL2HysThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_HOG_setGammaCorrection_bool(cv::cuda::HOG* instance, bool gamma_correction) {
		try {
			instance->setGammaCorrection(gamma_correction);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_HOG_getGammaCorrection_const(const cv::cuda::HOG* instance) {
		try {
			bool ret = instance->getGammaCorrection();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_HOG_setNumLevels_int(cv::cuda::HOG* instance, int nlevels) {
		try {
			instance->setNumLevels(nlevels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HOG_getNumLevels_const(const cv::cuda::HOG* instance) {
		try {
			int ret = instance->getNumLevels();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HOG_setHitThreshold_double(cv::cuda::HOG* instance, double hit_threshold) {
		try {
			instance->setHitThreshold(hit_threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_HOG_getHitThreshold_const(const cv::cuda::HOG* instance) {
		try {
			double ret = instance->getHitThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_HOG_setWinStride_Size(cv::cuda::HOG* instance, const cv::Size* win_stride) {
		try {
			instance->setWinStride(*win_stride);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_cuda_HOG_getWinStride_const(const cv::cuda::HOG* instance) {
		try {
			cv::Size ret = instance->getWinStride();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_cuda_HOG_setScaleFactor_double(cv::cuda::HOG* instance, double scale0) {
		try {
			instance->setScaleFactor(scale0);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_HOG_getScaleFactor_const(const cv::cuda::HOG* instance) {
		try {
			double ret = instance->getScaleFactor();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_HOG_setGroupThreshold_int(cv::cuda::HOG* instance, int group_threshold) {
		try {
			instance->setGroupThreshold(group_threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HOG_getGroupThreshold_const(const cv::cuda::HOG* instance) {
		try {
			int ret = instance->getGroupThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HOG_setDescriptorFormat_int(cv::cuda::HOG* instance, int descr_format) {
		try {
			instance->setDescriptorFormat(descr_format);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HOG_getDescriptorFormat_const(const cv::cuda::HOG* instance) {
		try {
			int ret = instance->getDescriptorFormat();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_HOG_getDescriptorSize_const(const cv::cuda::HOG* instance) {
		try {
			size_t ret = instance->getDescriptorSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_HOG_getBlockHistogramSize_const(const cv::cuda::HOG* instance) {
		try {
			size_t ret = instance->getBlockHistogramSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_cuda_HOG_setSVMDetector_const__InputArrayR(cv::cuda::HOG* instance, const cv::_InputArray* detector) {
		try {
			instance->setSVMDetector(*detector);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_cuda_HOG_getDefaultPeopleDetector_const(const cv::cuda::HOG* instance) {
		try {
			cv::Mat ret = instance->getDefaultPeopleDetector();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_cuda_HOG_detect_const__InputArrayR_vector_Point_R_vector_double_X(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Point>* found_locations, std::vector<double>* confidences) {
		try {
			instance->detect(*img, *found_locations, confidences);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HOG_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_double_X(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Rect>* found_locations, std::vector<double>* confidences) {
		try {
			instance->detectMultiScale(*img, *found_locations, confidences);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HOG* instance, const cv::_InputArray* img, const cv::_OutputArray* descriptors, cv::cuda::Stream* stream) {
		try {
			instance->compute(*img, *descriptors, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
