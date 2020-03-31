#include "common.hpp"
#include <opencv2/features2d.hpp>
#include "features2d_types.hpp"

extern "C" {
	Result_void cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__X_const_vector_vector_unsigned_char__X_vector_Point2f_X(const std::vector<std::vector<cv::DMatch>>* matches1to2, const std::vector<std::vector<unsigned char>>* correctMatches1to2Mask, std::vector<cv::Point2f>* recallPrecisionCurve) {
		try {
			cv::computeRecallPrecisionCurve(*matches1to2, *correctMatches1to2Mask, *recallPrecisionCurve);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawKeypoints_const__InputArrayX_const_vector_KeyPoint_X_const__InputOutputArrayX_const_ScalarX_DrawMatchesFlags(const cv::_InputArray* image, const std::vector<cv::KeyPoint>* keypoints, const cv::_InputOutputArray* outImage, const cv::Scalar* color, cv::DrawMatchesFlags flags) {
		try {
			cv::drawKeypoints(*image, *keypoints, *outImage, *color, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMatches_const__InputArrayX_const_vector_KeyPoint_X_const__InputArrayX_const_vector_KeyPoint_X_const_vector_DMatch_X_const__InputOutputArrayX_const_ScalarX_const_ScalarX_const_vector_char_X_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMatches_const__InputArrayX_const_vector_KeyPoint_X_const__InputArrayX_const_vector_KeyPoint_X_const_vector_vector_DMatch__X_const__InputOutputArrayX_const_ScalarX_const_ScalarX_const_vector_vector_char__X_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<std::vector<cv::DMatch>>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<std::vector<char>>* matchesMask, cv::DrawMatchesFlags flags) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_evaluateFeatureDetector_const_MatX_const_MatX_const_MatX_vector_KeyPoint_X_vector_KeyPoint_X_floatX_intX_const_Ptr_Feature2D_X(const cv::Mat* img1, const cv::Mat* img2, const cv::Mat* H1to2, std::vector<cv::KeyPoint>* keypoints1, std::vector<cv::KeyPoint>* keypoints2, float* repeatability, int* correspCount, const cv::Ptr<cv::Feature2D>* fdetector) {
		try {
			cv::evaluateFeatureDetector(*img1, *img2, *H1to2, keypoints1, keypoints2, *repeatability, *correspCount, *fdetector);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_getNearestPoint_const_vector_Point2f_X_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision) {
		try {
			int ret = cv::getNearestPoint(*recallPrecisionCurve, l_precision);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_getRecall_const_vector_Point2f_X_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision) {
		try {
			float ret = cv::getRecall(*recallPrecisionCurve, l_precision);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Ptr<cv::AKAZE>*> cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(cv::AKAZE::DescriptorType descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok(new cv::Ptr<cv::AKAZE>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::AKAZE>*>)
	}
	
	Result_void cv_AKAZE_setDescriptorType_DescriptorType(cv::AKAZE* instance, cv::AKAZE::DescriptorType dtype) {
		try {
			instance->setDescriptorType(dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::AKAZE::DescriptorType> cv_AKAZE_getDescriptorType_const(const cv::AKAZE* instance) {
		try {
			cv::AKAZE::DescriptorType ret = instance->getDescriptorType();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::AKAZE::DescriptorType>)
	}
	
	Result_void cv_AKAZE_setDescriptorSize_int(cv::AKAZE* instance, int dsize) {
		try {
			instance->setDescriptorSize(dsize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDescriptorSize_const(const cv::AKAZE* instance) {
		try {
			int ret = instance->getDescriptorSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setDescriptorChannels_int(cv::AKAZE* instance, int dch) {
		try {
			instance->setDescriptorChannels(dch);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDescriptorChannels_const(const cv::AKAZE* instance) {
		try {
			int ret = instance->getDescriptorChannels();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setThreshold_double(cv::AKAZE* instance, double threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_AKAZE_getThreshold_const(const cv::AKAZE* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_AKAZE_setNOctaves_int(cv::AKAZE* instance, int octaves) {
		try {
			instance->setNOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getNOctaves_const(const cv::AKAZE* instance) {
		try {
			int ret = instance->getNOctaves();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setNOctaveLayers_int(cv::AKAZE* instance, int octaveLayers) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getNOctaveLayers_const(const cv::AKAZE* instance) {
		try {
			int ret = instance->getNOctaveLayers();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setDiffusivity_DiffusivityType(cv::AKAZE* instance, cv::KAZE::DiffusivityType diff) {
		try {
			instance->setDiffusivity(diff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::KAZE::DiffusivityType> cv_AKAZE_getDiffusivity_const(const cv::AKAZE* instance) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::KAZE::DiffusivityType>)
	}
	
	Result<void*> cv_AKAZE_getDefaultName_const(const cv::AKAZE* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Ptr<cv::AgastFeatureDetector>*> cv_AgastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			return Ok(new cv::Ptr<cv::AgastFeatureDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::AgastFeatureDetector>*>)
	}
	
	Result_void cv_AgastFeatureDetector_setThreshold_int(cv::AgastFeatureDetector* instance, int threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AgastFeatureDetector_getThreshold_const(const cv::AgastFeatureDetector* instance) {
		try {
			int ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AgastFeatureDetector_setNonmaxSuppression_bool(cv::AgastFeatureDetector* instance, bool f) {
		try {
			instance->setNonmaxSuppression(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AgastFeatureDetector_getNonmaxSuppression_const(const cv::AgastFeatureDetector* instance) {
		try {
			bool ret = instance->getNonmaxSuppression();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_AgastFeatureDetector_setType_DetectorType(cv::AgastFeatureDetector* instance, cv::AgastFeatureDetector::DetectorType type) {
		try {
			instance->setType(type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::AgastFeatureDetector::DetectorType> cv_AgastFeatureDetector_getType_const(const cv::AgastFeatureDetector* instance) {
		try {
			cv::AgastFeatureDetector::DetectorType ret = instance->getType();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::AgastFeatureDetector::DetectorType>)
	}
	
	Result<void*> cv_AgastFeatureDetector_getDefaultName_const(const cv::AgastFeatureDetector* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BFMatcher_delete(cv::BFMatcher* instance) {
		delete instance;
	}
	Result<cv::BFMatcher*> cv_BFMatcher_BFMatcher_int_bool(int normType, bool crossCheck) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher(normType, crossCheck);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::BFMatcher*>)
	}
	
	Result<bool> cv_BFMatcher_isMaskSupported_const(const cv::BFMatcher* instance) {
		try {
			bool ret = instance->isMaskSupported();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::BFMatcher>*> cv_BFMatcher_create_int_bool(int normType, bool crossCheck) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create(normType, crossCheck);
			return Ok(new cv::Ptr<cv::BFMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BFMatcher>*>)
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_BFMatcher_clone_const_bool(const cv::BFMatcher* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>)
	}
	
	void cv_BOWImgDescriptorExtractor_delete(cv::BOWImgDescriptorExtractor* instance) {
		delete instance;
	}
	Result<cv::BOWImgDescriptorExtractor*> cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_Feature2D_X_const_Ptr_DescriptorMatcher_X(const cv::Ptr<cv::Feature2D>* dextractor, const cv::Ptr<cv::DescriptorMatcher>* dmatcher) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dextractor, *dmatcher);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::BOWImgDescriptorExtractor*>)
	}
	
	Result<cv::BOWImgDescriptorExtractor*> cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_X(const cv::Ptr<cv::DescriptorMatcher>* dmatcher) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dmatcher);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::BOWImgDescriptorExtractor*>)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_setVocabulary_const_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* vocabulary) {
		try {
			instance->setVocabulary(*vocabulary);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_BOWImgDescriptorExtractor_getVocabulary_const(const cv::BOWImgDescriptorExtractor* instance) {
		try {
			cv::Mat ret = instance->getVocabulary();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX_vector_vector_int__X_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, cv::Mat* descriptors) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor, pointIdxsOfClusters, descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute_const__InputArrayX_const__OutputArrayX_vector_vector_int__X(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor, pointIdxsOfClusters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute2_const_MatX_vector_KeyPoint_X_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* image, std::vector<cv::KeyPoint>* keypoints, cv::Mat* imgDescriptor) {
		try {
			instance->compute2(*image, *keypoints, *imgDescriptor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BOWImgDescriptorExtractor_descriptorSize_const(const cv::BOWImgDescriptorExtractor* instance) {
		try {
			int ret = instance->descriptorSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_BOWImgDescriptorExtractor_descriptorType_const(const cv::BOWImgDescriptorExtractor* instance) {
		try {
			int ret = instance->descriptorType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_BOWKMeansTrainer_delete(cv::BOWKMeansTrainer* instance) {
		delete instance;
	}
	Result<cv::BOWKMeansTrainer*> cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaX_int_int(int clusterCount, const cv::TermCriteria* termcrit, int attempts, int flags) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount, *termcrit, attempts, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::BOWKMeansTrainer*>)
	}
	
	Result<cv::Mat*> cv_BOWKMeansTrainer_cluster_const(const cv::BOWKMeansTrainer* instance) {
		try {
			cv::Mat ret = instance->cluster();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_BOWKMeansTrainer_cluster_const_const_MatX(const cv::BOWKMeansTrainer* instance, const cv::Mat* descriptors) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_BOWTrainer_add_const_MatX(cv::BOWTrainer* instance, const cv::Mat* descriptors) {
		try {
			instance->add(*descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_BOWTrainer_getDescriptors_const(const cv::BOWTrainer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getDescriptors();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result<int> cv_BOWTrainer_descriptorsCount_const(const cv::BOWTrainer* instance) {
		try {
			int ret = instance->descriptorsCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BOWTrainer_clear(cv::BOWTrainer* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_BOWTrainer_cluster_const(const cv::BOWTrainer* instance) {
		try {
			cv::Mat ret = instance->cluster();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_BOWTrainer_cluster_const_const_MatX(const cv::BOWTrainer* instance, const cv::Mat* descriptors) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_BRISK_delete(cv::BRISK* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::BRISK>*> cv_BRISK_create_int_int_float(int thresh, int octaves, float patternScale) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, patternScale);
			return Ok(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>)
	}
	
	Result<cv::Ptr<cv::BRISK>*> cv_BRISK_create_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X(const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*radiusList, *numberList, dMax, dMin, *indexChange);
			return Ok(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>)
	}
	
	Result<cv::Ptr<cv::BRISK>*> cv_BRISK_create_int_int_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *radiusList, *numberList, dMax, dMin, *indexChange);
			return Ok(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BRISK>*>)
	}
	
	Result<void*> cv_BRISK_getDefaultName_const(const cv::BRISK* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BRISK_setThreshold_int(cv::BRISK* instance, int threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BRISK_getThreshold_const(const cv::BRISK* instance) {
		try {
			int ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BRISK_setOctaves_int(cv::BRISK* instance, int octaves) {
		try {
			instance->setOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BRISK_getOctaves_const(const cv::BRISK* instance) {
		try {
			int ret = instance->getOctaves();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DescriptorMatcher_add_const__InputArrayX(cv::DescriptorMatcher* instance, const cv::_InputArray* descriptors) {
		try {
			instance->add(*descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_DescriptorMatcher_getTrainDescriptors_const(const cv::DescriptorMatcher* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getTrainDescriptors();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_DescriptorMatcher_clear(cv::DescriptorMatcher* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_DescriptorMatcher_empty_const(const cv::DescriptorMatcher* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_DescriptorMatcher_isMaskSupported_const(const cv::DescriptorMatcher* instance) {
		try {
			bool ret = instance->isMaskSupported();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_DescriptorMatcher_train(cv::DescriptorMatcher* instance) {
		try {
			instance->train();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_match_const_const__InputArrayX_const__InputArrayX_vector_DMatch_X_const__InputArrayX(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_knnMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_int_const__InputArrayX_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_float_const__InputArrayX_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_match_const__InputArrayX_vector_DMatch_X_const__InputArrayX(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* masks) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_knnMatch_const__InputArrayX_vector_vector_DMatch__X_int_const__InputArrayX_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* masks, bool compactResult) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_radiusMatch_const__InputArrayX_vector_vector_DMatch__X_float_const__InputArrayX_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* masks, bool compactResult) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_write_const_const_StringX(const cv::DescriptorMatcher* instance, const char* fileName) {
		try {
			instance->write(std::string(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_read_const_StringX(cv::DescriptorMatcher* instance, const char* fileName) {
		try {
			instance->read(std::string(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_read_const_FileNodeX(cv::DescriptorMatcher* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_write_const_FileStorageX(const cv::DescriptorMatcher* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_DescriptorMatcher_clone_const_bool(const cv::DescriptorMatcher* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>)
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_DescriptorMatcher_create_const_StringX(const char* descriptorMatcherType) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(std::string(descriptorMatcherType));
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>)
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_DescriptorMatcher_create_const_MatcherTypeX(const cv::DescriptorMatcher::MatcherType* matcherType) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(*matcherType);
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>)
	}
	
	Result_void cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_X_const_StringX(const cv::DescriptorMatcher* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name) {
		try {
			instance->write(*fs, std::string(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::FastFeatureDetector>*> cv_FastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create(threshold, nonmaxSuppression, type);
			return Ok(new cv::Ptr<cv::FastFeatureDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::FastFeatureDetector>*>)
	}
	
	Result_void cv_FastFeatureDetector_setThreshold_int(cv::FastFeatureDetector* instance, int threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FastFeatureDetector_getThreshold_const(const cv::FastFeatureDetector* instance) {
		try {
			int ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FastFeatureDetector_setNonmaxSuppression_bool(cv::FastFeatureDetector* instance, bool f) {
		try {
			instance->setNonmaxSuppression(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_FastFeatureDetector_getNonmaxSuppression_const(const cv::FastFeatureDetector* instance) {
		try {
			bool ret = instance->getNonmaxSuppression();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_FastFeatureDetector_setType_DetectorType(cv::FastFeatureDetector* instance, cv::FastFeatureDetector::DetectorType type) {
		try {
			instance->setType(type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::FastFeatureDetector::DetectorType> cv_FastFeatureDetector_getType_const(const cv::FastFeatureDetector* instance) {
		try {
			cv::FastFeatureDetector::DetectorType ret = instance->getType();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::FastFeatureDetector::DetectorType>)
	}
	
	Result<void*> cv_FastFeatureDetector_getDefaultName_const(const cv::FastFeatureDetector* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Feature2D_delete(cv::Feature2D* instance) {
		delete instance;
	}
	Result_void cv_Feature2D_detect_const__InputArrayX_vector_KeyPoint_X_const__InputArrayX(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_InputArray* mask) {
		try {
			instance->detect(*image, *keypoints, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_detect_const__InputArrayX_vector_vector_KeyPoint__X_const__InputArrayX(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_InputArray* masks) {
		try {
			instance->detect(*images, *keypoints, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_compute_const__InputArrayX_vector_vector_KeyPoint__X_const__OutputArrayX(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_detectAndCompute_const__InputArrayX_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX_bool(cv::Feature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Feature2D_descriptorSize_const(const cv::Feature2D* instance) {
		try {
			int ret = instance->descriptorSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Feature2D_descriptorType_const(const cv::Feature2D* instance) {
		try {
			int ret = instance->descriptorType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Feature2D_defaultNorm_const(const cv::Feature2D* instance) {
		try {
			int ret = instance->defaultNorm();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Feature2D_write_const_const_StringX(const cv::Feature2D* instance, const char* fileName) {
		try {
			instance->write(std::string(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_read_const_StringX(cv::Feature2D* instance, const char* fileName) {
		try {
			instance->read(std::string(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_write_const_FileStorageX(const cv::Feature2D* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_read_const_FileNodeX(cv::Feature2D* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_Feature2D_empty_const(const cv::Feature2D* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_Feature2D_getDefaultName_const(const cv::Feature2D* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Feature2D_write_const_const_Ptr_FileStorage_X_const_StringX(const cv::Feature2D* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name) {
		try {
			instance->write(*fs, std::string(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FlannBasedMatcher_delete(cv::FlannBasedMatcher* instance) {
		delete instance;
	}
	Result<cv::FlannBasedMatcher*> cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_X_const_Ptr_SearchParams_X(const cv::Ptr<cv::flann::IndexParams>* indexParams, const cv::Ptr<cv::flann::SearchParams>* searchParams) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher(*indexParams, *searchParams);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::FlannBasedMatcher*>)
	}
	
	Result_void cv_FlannBasedMatcher_add_const__InputArrayX(cv::FlannBasedMatcher* instance, const cv::_InputArray* descriptors) {
		try {
			instance->add(*descriptors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_clear(cv::FlannBasedMatcher* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_read_const_FileNodeX(cv::FlannBasedMatcher* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_write_const_FileStorageX(const cv::FlannBasedMatcher* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_train(cv::FlannBasedMatcher* instance) {
		try {
			instance->train();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_FlannBasedMatcher_isMaskSupported_const(const cv::FlannBasedMatcher* instance) {
		try {
			bool ret = instance->isMaskSupported();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::FlannBasedMatcher>*> cv_FlannBasedMatcher_create() {
		try {
			cv::Ptr<cv::FlannBasedMatcher> ret = cv::FlannBasedMatcher::create();
			return Ok(new cv::Ptr<cv::FlannBasedMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::FlannBasedMatcher>*>)
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_FlannBasedMatcher_clone_const_bool(const cv::FlannBasedMatcher* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::DescriptorMatcher>*>)
	}
	
	Result<cv::Ptr<cv::GFTTDetector>*> cv_GFTTDetector_create_int_double_double_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double k) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, k);
			return Ok(new cv::Ptr<cv::GFTTDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::GFTTDetector>*>)
	}
	
	Result<cv::Ptr<cv::GFTTDetector>*> cv_GFTTDetector_create_int_double_double_int_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, bool useHarrisDetector, double k) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize, useHarrisDetector, k);
			return Ok(new cv::Ptr<cv::GFTTDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::GFTTDetector>*>)
	}
	
	Result_void cv_GFTTDetector_setMaxFeatures_int(cv::GFTTDetector* instance, int maxFeatures) {
		try {
			instance->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GFTTDetector_getMaxFeatures_const(const cv::GFTTDetector* instance) {
		try {
			int ret = instance->getMaxFeatures();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GFTTDetector_setQualityLevel_double(cv::GFTTDetector* instance, double qlevel) {
		try {
			instance->setQualityLevel(qlevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getQualityLevel_const(const cv::GFTTDetector* instance) {
		try {
			double ret = instance->getQualityLevel();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GFTTDetector_setMinDistance_double(cv::GFTTDetector* instance, double minDistance) {
		try {
			instance->setMinDistance(minDistance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getMinDistance_const(const cv::GFTTDetector* instance) {
		try {
			double ret = instance->getMinDistance();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GFTTDetector_setBlockSize_int(cv::GFTTDetector* instance, int blockSize) {
		try {
			instance->setBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GFTTDetector_getBlockSize_const(const cv::GFTTDetector* instance) {
		try {
			int ret = instance->getBlockSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GFTTDetector_setHarrisDetector_bool(cv::GFTTDetector* instance, bool val) {
		try {
			instance->setHarrisDetector(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_GFTTDetector_getHarrisDetector_const(const cv::GFTTDetector* instance) {
		try {
			bool ret = instance->getHarrisDetector();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_GFTTDetector_setK_double(cv::GFTTDetector* instance, double k) {
		try {
			instance->setK(k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getK_const(const cv::GFTTDetector* instance) {
		try {
			double ret = instance->getK();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_GFTTDetector_getDefaultName_const(const cv::GFTTDetector* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Ptr<cv::KAZE>*> cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok(new cv::Ptr<cv::KAZE>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::KAZE>*>)
	}
	
	Result_void cv_KAZE_setExtended_bool(cv::KAZE* instance, bool extended) {
		try {
			instance->setExtended(extended);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_KAZE_getExtended_const(const cv::KAZE* instance) {
		try {
			bool ret = instance->getExtended();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_KAZE_setUpright_bool(cv::KAZE* instance, bool upright) {
		try {
			instance->setUpright(upright);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_KAZE_getUpright_const(const cv::KAZE* instance) {
		try {
			bool ret = instance->getUpright();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_KAZE_setThreshold_double(cv::KAZE* instance, double threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_KAZE_getThreshold_const(const cv::KAZE* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_KAZE_setNOctaves_int(cv::KAZE* instance, int octaves) {
		try {
			instance->setNOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_KAZE_getNOctaves_const(const cv::KAZE* instance) {
		try {
			int ret = instance->getNOctaves();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_KAZE_setNOctaveLayers_int(cv::KAZE* instance, int octaveLayers) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_KAZE_getNOctaveLayers_const(const cv::KAZE* instance) {
		try {
			int ret = instance->getNOctaveLayers();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_KAZE_setDiffusivity_DiffusivityType(cv::KAZE* instance, cv::KAZE::DiffusivityType diff) {
		try {
			instance->setDiffusivity(diff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::KAZE::DiffusivityType> cv_KAZE_getDiffusivity_const(const cv::KAZE* instance) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::KAZE::DiffusivityType>)
	}
	
	Result<void*> cv_KAZE_getDefaultName_const(const cv::KAZE* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_KeyPointsFilter_delete(cv::KeyPointsFilter* instance) {
		delete instance;
	}
	Result<cv::KeyPointsFilter*> cv_KeyPointsFilter_KeyPointsFilter() {
		try {
			cv::KeyPointsFilter* ret = new cv::KeyPointsFilter();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::KeyPointsFilter*>)
	}
	
	Result_void cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_X_Size_int(std::vector<cv::KeyPoint>* keypoints, const cv::Size* imageSize, int borderSize) {
		try {
			cv::KeyPointsFilter::runByImageBorder(*keypoints, *imageSize, borderSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_X_float_float(std::vector<cv::KeyPoint>* keypoints, float minSize, float maxSize) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*keypoints, minSize, maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_X_const_MatX(std::vector<cv::KeyPoint>* keypoints, const cv::Mat* mask) {
		try {
			cv::KeyPointsFilter::runByPixelsMask(*keypoints, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_X(std::vector<cv::KeyPoint>* keypoints) {
		try {
			cv::KeyPointsFilter::removeDuplicated(*keypoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_X(std::vector<cv::KeyPoint>* keypoints) {
		try {
			cv::KeyPointsFilter::removeDuplicatedSorted(*keypoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_retainBest_vector_KeyPoint_X_int(std::vector<cv::KeyPoint>* keypoints, int npoints) {
		try {
			cv::KeyPointsFilter::retainBest(*keypoints, npoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::MSER>*> cv_MSER_create_int_int_int_double_double_int_double_double_int(int _delta, int _min_area, int _max_area, double _max_variation, double _min_diversity, int _max_evolution, double _area_threshold, double _min_margin, int _edge_blur_size) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create(_delta, _min_area, _max_area, _max_variation, _min_diversity, _max_evolution, _area_threshold, _min_margin, _edge_blur_size);
			return Ok(new cv::Ptr<cv::MSER>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::MSER>*>)
	}
	
	Result_void cv_MSER_detectRegions_const__InputArrayX_vector_vector_Point__X_vector_Rect_X(cv::MSER* instance, const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* msers, std::vector<cv::Rect>* bboxes) {
		try {
			instance->detectRegions(*image, *msers, *bboxes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MSER_setDelta_int(cv::MSER* instance, int delta) {
		try {
			instance->setDelta(delta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getDelta_const(const cv::MSER* instance) {
		try {
			int ret = instance->getDelta();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setMinArea_int(cv::MSER* instance, int minArea) {
		try {
			instance->setMinArea(minArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getMinArea_const(const cv::MSER* instance) {
		try {
			int ret = instance->getMinArea();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setMaxArea_int(cv::MSER* instance, int maxArea) {
		try {
			instance->setMaxArea(maxArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getMaxArea_const(const cv::MSER* instance) {
		try {
			int ret = instance->getMaxArea();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setPass2Only_bool(cv::MSER* instance, bool f) {
		try {
			instance->setPass2Only(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_MSER_getPass2Only_const(const cv::MSER* instance) {
		try {
			bool ret = instance->getPass2Only();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_MSER_getDefaultName_const(const cv::MSER* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ORB_kBytes_const(const cv::ORB* instance) {
		try {
			int ret = instance->kBytes;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Ptr<cv::ORB>*> cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, cv::ORB::ScoreType scoreType, int patchSize, int fastThreshold) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold);
			return Ok(new cv::Ptr<cv::ORB>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ORB>*>)
	}
	
	Result_void cv_ORB_setMaxFeatures_int(cv::ORB* instance, int maxFeatures) {
		try {
			instance->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getMaxFeatures_const(const cv::ORB* instance) {
		try {
			int ret = instance->getMaxFeatures();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setScaleFactor_double(cv::ORB* instance, double scaleFactor) {
		try {
			instance->setScaleFactor(scaleFactor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ORB_getScaleFactor_const(const cv::ORB* instance) {
		try {
			double ret = instance->getScaleFactor();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ORB_setNLevels_int(cv::ORB* instance, int nlevels) {
		try {
			instance->setNLevels(nlevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getNLevels_const(const cv::ORB* instance) {
		try {
			int ret = instance->getNLevels();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setEdgeThreshold_int(cv::ORB* instance, int edgeThreshold) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getEdgeThreshold_const(const cv::ORB* instance) {
		try {
			int ret = instance->getEdgeThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setFirstLevel_int(cv::ORB* instance, int firstLevel) {
		try {
			instance->setFirstLevel(firstLevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getFirstLevel_const(const cv::ORB* instance) {
		try {
			int ret = instance->getFirstLevel();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setWTA_K_int(cv::ORB* instance, int wta_k) {
		try {
			instance->setWTA_K(wta_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getWTA_K_const(const cv::ORB* instance) {
		try {
			int ret = instance->getWTA_K();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setScoreType_ScoreType(cv::ORB* instance, cv::ORB::ScoreType scoreType) {
		try {
			instance->setScoreType(scoreType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::ORB::ScoreType> cv_ORB_getScoreType_const(const cv::ORB* instance) {
		try {
			cv::ORB::ScoreType ret = instance->getScoreType();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ORB::ScoreType>)
	}
	
	Result_void cv_ORB_setPatchSize_int(cv::ORB* instance, int patchSize) {
		try {
			instance->setPatchSize(patchSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getPatchSize_const(const cv::ORB* instance) {
		try {
			int ret = instance->getPatchSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setFastThreshold_int(cv::ORB* instance, int fastThreshold) {
		try {
			instance->setFastThreshold(fastThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getFastThreshold_const(const cv::ORB* instance) {
		try {
			int ret = instance->getFastThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ORB_getDefaultName_const(const cv::ORB* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SimpleBlobDetector_delete(cv::SimpleBlobDetector* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::SimpleBlobDetector>*> cv_SimpleBlobDetector_create_const_ParamsX(const cv::SimpleBlobDetector::Params* parameters) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create(*parameters);
			return Ok(new cv::Ptr<cv::SimpleBlobDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::SimpleBlobDetector>*>)
	}
	
	Result<void*> cv_SimpleBlobDetector_getDefaultName_const(const cv::SimpleBlobDetector* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::SimpleBlobDetector::Params> cv_SimpleBlobDetector_Params_Params() {
		try {
			cv::SimpleBlobDetector::Params ret;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::SimpleBlobDetector::Params>)
	}
	
	Result_void cv_SimpleBlobDetector_Params_read_const_FileNodeX(cv::SimpleBlobDetector::Params instance, const cv::FileNode* fn) {
		try {
			instance.read(*fn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SimpleBlobDetector_Params_write_const_FileStorageX(const cv::SimpleBlobDetector::Params instance, cv::FileStorage* fs) {
		try {
			instance.write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
