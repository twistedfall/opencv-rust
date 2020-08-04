#include "ocvrs_common.hpp"
#include <opencv2/cudafeatures2d.hpp>
#include "cudafeatures2d_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cuda::DescriptorMatcher>*> cv_cuda_DescriptorMatcher_createBFMatcher_int(int normType) {
		try {
			cv::Ptr<cv::cuda::DescriptorMatcher> ret = cv::cuda::DescriptorMatcher::createBFMatcher(normType);
			return Ok(new cv::Ptr<cv::cuda::DescriptorMatcher>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::DescriptorMatcher>*>))
	}
	
	Result<bool> cv_cuda_DescriptorMatcher_isMaskSupported_const(const cv::cuda::DescriptorMatcher* instance) {
		try {
			bool ret = instance->isMaskSupported();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_DescriptorMatcher_add_const_vector_GpuMat_R(cv::cuda::DescriptorMatcher* instance, const std::vector<cv::cuda::GpuMat>* descriptors) {
		try {
			instance->add(*descriptors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const std::vector<cv::cuda::GpuMat>*> cv_cuda_DescriptorMatcher_getTrainDescriptors_const(const cv::cuda::DescriptorMatcher* instance) {
		try {
			const std::vector<cv::cuda::GpuMat> ret = instance->getTrainDescriptors();
			return Ok(new const std::vector<cv::cuda::GpuMat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const std::vector<cv::cuda::GpuMat>*>))
	}
	
	Result_void cv_cuda_DescriptorMatcher_clear(cv::cuda::DescriptorMatcher* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_DescriptorMatcher_empty_const(const cv::cuda::DescriptorMatcher* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_DescriptorMatcher_train(cv::cuda::DescriptorMatcher* instance) {
		try {
			instance->train();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const_vector_GpuMat_R(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const std::vector<cv::cuda::GpuMat>* masks) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->matchAsync(*queryDescriptors, *trainDescriptors, *matches, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR_const_vector_GpuMat_R_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream) {
		try {
			instance->matchAsync(*queryDescriptors, *matches, *masks, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_matchConvert_const__InputArrayR_vector_DMatch_R(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<cv::DMatch>* matches) {
		try {
			instance->matchConvert(*gpu_matches, *matches);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const_vector_GpuMat_R_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const std::vector<cv::cuda::GpuMat>* masks, bool compactResult) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, int k, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *trainDescriptors, *matches, k, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int_const_vector_GpuMat_R_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, int k, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *matches, k, *masks, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vector_vector_DMatch__R_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, bool compactResult) {
		try {
			instance->knnMatchConvert(*gpu_matches, *matches, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const_vector_GpuMat_R_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const std::vector<cv::cuda::GpuMat>* masks, bool compactResult) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, float maxDistance, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float_const_vector_GpuMat_R_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, float maxDistance, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *matches, maxDistance, *masks, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vector_vector_DMatch__R_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, bool compactResult) {
		try {
			instance->radiusMatchConvert(*gpu_matches, *matches, compactResult);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::FastFeatureDetector>*> cv_cuda_FastFeatureDetector_create_int_bool_int_int(int threshold, bool nonmaxSuppression, int type, int max_npoints) {
		try {
			cv::Ptr<cv::cuda::FastFeatureDetector> ret = cv::cuda::FastFeatureDetector::create(threshold, nonmaxSuppression, type, max_npoints);
			return Ok(new cv::Ptr<cv::cuda::FastFeatureDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::FastFeatureDetector>*>))
	}
	
	Result_void cv_cuda_FastFeatureDetector_setThreshold_int(cv::cuda::FastFeatureDetector* instance, int threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_FastFeatureDetector_setMaxNumPoints_int(cv::cuda::FastFeatureDetector* instance, int max_npoints) {
		try {
			instance->setMaxNumPoints(max_npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_FastFeatureDetector_getMaxNumPoints_const(const cv::cuda::FastFeatureDetector* instance) {
		try {
			int ret = instance->getMaxNumPoints();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->detectAsync(*image, *keypoints, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, cv::cuda::Stream* stream) {
		try {
			instance->computeAsync(*image, *keypoints, *descriptors, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, cv::cuda::Stream* stream) {
		try {
			instance->detectAndComputeAsync(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Feature2DAsync_convert_const__InputArrayR_vector_KeyPoint_R(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* gpu_keypoints, std::vector<cv::KeyPoint>* keypoints) {
		try {
			instance->convert(*gpu_keypoints, *keypoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::ORB>*> cv_cuda_ORB_create_int_float_int_int_int_int_int_int_int_bool(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, int scoreType, int patchSize, int fastThreshold, bool blurForDescriptor) {
		try {
			cv::Ptr<cv::cuda::ORB> ret = cv::cuda::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold, blurForDescriptor);
			return Ok(new cv::Ptr<cv::cuda::ORB>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::ORB>*>))
	}
	
	Result_void cv_cuda_ORB_setBlurForDescriptor_bool(cv::cuda::ORB* instance, bool blurForDescriptor) {
		try {
			instance->setBlurForDescriptor(blurForDescriptor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_ORB_getBlurForDescriptor_const(const cv::cuda::ORB* instance) {
		try {
			bool ret = instance->getBlurForDescriptor();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_ORB_setFastThreshold_int(cv::cuda::ORB* instance, int fastThreshold) {
		try {
			instance->setFastThreshold(fastThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_ORB_getFastThreshold_const(const cv::cuda::ORB* instance) {
		try {
			int ret = instance->getFastThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
}
