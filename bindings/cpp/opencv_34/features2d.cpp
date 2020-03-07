#include "common.hpp"
#include <opencv2/features2d.hpp>
#include "features2d_types.hpp"

extern "C" {
	Result_void cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool(void* image, void* keypoints, int threshold, bool nonmaxSuppression) {
		try {
			cv::AGAST(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), threshold, nonmaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool_int(void* image, void* keypoints, int threshold, bool nonmaxSuppression, int type) {
		try {
			cv::AGAST(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool(void* image, void* keypoints, int threshold, bool nonmaxSuppression) {
		try {
			cv::FAST(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), threshold, nonmaxSuppression);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool_int(void* image, void* keypoints, int threshold, bool nonmaxSuppression, int type) {
		try {
			cv::FAST(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__X_const_vector_vector_unsigned_char__X_vector_Point2f_X(void* matches1to2, void* correctMatches1to2Mask, void* recallPrecisionCurve) {
		try {
			cv::computeRecallPrecisionCurve(*reinterpret_cast<const std::vector<std::vector<cv::DMatch>>*>(matches1to2), *reinterpret_cast<const std::vector<std::vector<unsigned char>>*>(correctMatches1to2Mask), *reinterpret_cast<std::vector<cv::Point2f>*>(recallPrecisionCurve));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawKeypoints_const__InputArrayX_const_vector_KeyPoint_X_const__InputOutputArrayX_const_ScalarX_int(void* image, void* keypoints, void* outImage, const cv::Scalar* color, int flags) {
		try {
			cv::drawKeypoints(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_InputOutputArray*>(outImage), *color, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMatches_const__InputArrayX_const_vector_KeyPoint_X_const__InputArrayX_const_vector_KeyPoint_X_const_vector_DMatch_X_const__InputOutputArrayX_const_ScalarX_const_ScalarX_const_vector_char_X_int(void* img1, void* keypoints1, void* img2, void* keypoints2, void* matches1to2, void* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, void* matchesMask, int flags) {
		try {
			cv::drawMatches(*reinterpret_cast<const cv::_InputArray*>(img1), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints1), *reinterpret_cast<const cv::_InputArray*>(img2), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints2), *reinterpret_cast<const std::vector<cv::DMatch>*>(matches1to2), *reinterpret_cast<const cv::_InputOutputArray*>(outImg), *matchColor, *singlePointColor, *reinterpret_cast<const std::vector<char>*>(matchesMask), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMatches_const__InputArrayX_const_vector_KeyPoint_X_const__InputArrayX_const_vector_KeyPoint_X_const_vector_vector_DMatch__X_const__InputOutputArrayX_const_ScalarX_const_ScalarX_const_vector_vector_char__X_int(void* img1, void* keypoints1, void* img2, void* keypoints2, void* matches1to2, void* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, void* matchesMask, int flags) {
		try {
			cv::drawMatches(*reinterpret_cast<const cv::_InputArray*>(img1), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints1), *reinterpret_cast<const cv::_InputArray*>(img2), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints2), *reinterpret_cast<const std::vector<std::vector<cv::DMatch>>*>(matches1to2), *reinterpret_cast<const cv::_InputOutputArray*>(outImg), *matchColor, *singlePointColor, *reinterpret_cast<const std::vector<std::vector<char>>*>(matchesMask), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_evaluateFeatureDetector_const_MatX_const_MatX_const_MatX_vector_KeyPoint_X_vector_KeyPoint_X_floatX_intX_const_Ptr_FeatureDetector_X(void* img1, void* img2, void* H1to2, void* keypoints1, void* keypoints2, float* repeatability, int* correspCount, void* fdetector) {
		try {
			cv::evaluateFeatureDetector(*reinterpret_cast<const cv::Mat*>(img1), *reinterpret_cast<const cv::Mat*>(img2), *reinterpret_cast<const cv::Mat*>(H1to2), reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints1), reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints2), *repeatability, *correspCount, *reinterpret_cast<const cv::Ptr<cv::FeatureDetector>*>(fdetector));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_getNearestPoint_const_vector_Point2f_X_float(void* recallPrecisionCurve, float l_precision) {
		try {
			int ret = cv::getNearestPoint(*reinterpret_cast<const std::vector<cv::Point2f>*>(recallPrecisionCurve), l_precision);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_getRecall_const_vector_Point2f_X_float(void* recallPrecisionCurve, float l_precision) {
		try {
			float ret = cv::getRecall(*reinterpret_cast<const std::vector<cv::Point2f>*>(recallPrecisionCurve), l_precision);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_AKAZE_create_int_int_int_float_int_int_int(int descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, int diffusivity) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok<void*>(new cv::Ptr<cv::AKAZE>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AKAZE_setDescriptorType_int(void* instance, int dtype) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setDescriptorType(dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDescriptorType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getDescriptorType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setDescriptorSize_int(void* instance, int dsize) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setDescriptorSize(dsize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDescriptorSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getDescriptorSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setDescriptorChannels_int(void* instance, int dch) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setDescriptorChannels(dch);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDescriptorChannels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getDescriptorChannels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setThreshold_double(void* instance, double threshold) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_AKAZE_getThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::AKAZE*>(instance)->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_AKAZE_setNOctaves_int(void* instance, int octaves) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setNOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getNOctaves_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getNOctaves();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setNOctaveLayers_int(void* instance, int octaveLayers) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setNOctaveLayers(octaveLayers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getNOctaveLayers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getNOctaveLayers();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AKAZE_setDiffusivity_int(void* instance, int diff) {
		try {
			reinterpret_cast<cv::AKAZE*>(instance)->setDiffusivity(diff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AKAZE_getDiffusivity_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AKAZE*>(instance)->getDiffusivity();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_AKAZE_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::AKAZE*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_AgastFeatureDetector_create_int_bool_int(int threshold, bool nonmaxSuppression, int type) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			return Ok<void*>(new cv::Ptr<cv::AgastFeatureDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AgastFeatureDetector_setThreshold_int(void* instance, int threshold) {
		try {
			reinterpret_cast<cv::AgastFeatureDetector*>(instance)->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AgastFeatureDetector_getThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AgastFeatureDetector*>(instance)->getThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_AgastFeatureDetector_setNonmaxSuppression_bool(void* instance, bool f) {
		try {
			reinterpret_cast<cv::AgastFeatureDetector*>(instance)->setNonmaxSuppression(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AgastFeatureDetector_getNonmaxSuppression_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::AgastFeatureDetector*>(instance)->getNonmaxSuppression();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_AgastFeatureDetector_setType_int(void* instance, int type) {
		try {
			reinterpret_cast<cv::AgastFeatureDetector*>(instance)->setType(type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_AgastFeatureDetector_getType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::AgastFeatureDetector*>(instance)->getType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_AgastFeatureDetector_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::AgastFeatureDetector*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BFMatcher_delete(cv::BFMatcher* instance) {
		delete instance;
	}
	Result<void*> cv_BFMatcher_BFMatcher_int_bool(int normType, bool crossCheck) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher(normType, crossCheck);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_BFMatcher_isMaskSupported_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::BFMatcher*>(instance)->isMaskSupported();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_BFMatcher_create_int_bool(int normType, bool crossCheck) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create(normType, crossCheck);
			return Ok<void*>(new cv::Ptr<cv::BFMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BFMatcher_clone_const_bool(void* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = reinterpret_cast<cv::BFMatcher*>(instance)->clone(emptyTrainData);
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BOWImgDescriptorExtractor_delete(cv::BOWImgDescriptorExtractor* instance) {
		delete instance;
	}
	Result<void*> cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorExtractor_X_const_Ptr_DescriptorMatcher_X(void* dextractor, void* dmatcher) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*reinterpret_cast<const cv::Ptr<cv::DescriptorExtractor>*>(dextractor), *reinterpret_cast<const cv::Ptr<cv::DescriptorMatcher>*>(dmatcher));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_X(void* dmatcher) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*reinterpret_cast<const cv::Ptr<cv::DescriptorMatcher>*>(dmatcher));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_setVocabulary_const_MatX(void* instance, void* vocabulary) {
		try {
			reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->setVocabulary(*reinterpret_cast<const cv::Mat*>(vocabulary));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_BOWImgDescriptorExtractor_getVocabulary_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->getVocabulary();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX_vector_vector_int__X_MatX(void* instance, void* image, void* keypoints, void* imgDescriptor, void* pointIdxsOfClusters, void* descriptors) {
		try {
			reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(imgDescriptor), reinterpret_cast<std::vector<std::vector<int>>*>(pointIdxsOfClusters), reinterpret_cast<cv::Mat*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute_const__InputArrayX_const__OutputArrayX_vector_vector_int__X(void* instance, void* keypointDescriptors, void* imgDescriptor, void* pointIdxsOfClusters) {
		try {
			reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(keypointDescriptors), *reinterpret_cast<const cv::_OutputArray*>(imgDescriptor), reinterpret_cast<std::vector<std::vector<int>>*>(pointIdxsOfClusters));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BOWImgDescriptorExtractor_compute2_const_MatX_vector_KeyPoint_X_MatX(void* instance, void* image, void* keypoints, void* imgDescriptor) {
		try {
			reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->compute2(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<cv::Mat*>(imgDescriptor));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_BOWImgDescriptorExtractor_descriptorSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->descriptorSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_BOWImgDescriptorExtractor_descriptorType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BOWImgDescriptorExtractor*>(instance)->descriptorType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_BOWKMeansTrainer_delete(cv::BOWKMeansTrainer* instance) {
		delete instance;
	}
	Result<void*> cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaX_int_int(int clusterCount, void* termcrit, int attempts, int flags) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount, *reinterpret_cast<const cv::TermCriteria*>(termcrit), attempts, flags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BOWKMeansTrainer_cluster_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BOWKMeansTrainer*>(instance)->cluster();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BOWKMeansTrainer_cluster_const_const_MatX(void* instance, void* descriptors) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BOWKMeansTrainer*>(instance)->cluster(*reinterpret_cast<const cv::Mat*>(descriptors));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BOWTrainer_add_const_MatX(void* instance, void* descriptors) {
		try {
			reinterpret_cast<cv::BOWTrainer*>(instance)->add(*reinterpret_cast<const cv::Mat*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_BOWTrainer_getDescriptors_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::BOWTrainer*>(instance)->getDescriptors();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_BOWTrainer_descriptorsCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BOWTrainer*>(instance)->descriptorsCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_BOWTrainer_clear(void* instance) {
		try {
			reinterpret_cast<cv::BOWTrainer*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_BOWTrainer_cluster_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BOWTrainer*>(instance)->cluster();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BOWTrainer_cluster_const_const_MatX(void* instance, void* descriptors) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BOWTrainer*>(instance)->cluster(*reinterpret_cast<const cv::Mat*>(descriptors));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BRISK_delete(cv::BRISK* instance) {
		delete instance;
	}
	Result<void*> cv_BRISK_create_int_int_float(int thresh, int octaves, float patternScale) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, patternScale);
			return Ok<void*>(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BRISK_create_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X(void* radiusList, void* numberList, float dMax, float dMin, void* indexChange) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*reinterpret_cast<const std::vector<float>*>(radiusList), *reinterpret_cast<const std::vector<int>*>(numberList), dMax, dMin, *reinterpret_cast<const std::vector<int>*>(indexChange));
			return Ok<void*>(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BRISK_create_int_int_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X(int thresh, int octaves, void* radiusList, void* numberList, float dMax, float dMin, void* indexChange) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *reinterpret_cast<const std::vector<float>*>(radiusList), *reinterpret_cast<const std::vector<int>*>(numberList), dMax, dMin, *reinterpret_cast<const std::vector<int>*>(indexChange));
			return Ok<void*>(new cv::Ptr<cv::BRISK>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BRISK_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::BRISK*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DescriptorMatcher_add_const__InputArrayX(void* instance, void* descriptors) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->add(*reinterpret_cast<const cv::_InputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DescriptorMatcher_getTrainDescriptors_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::DescriptorMatcher*>(instance)->getTrainDescriptors();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DescriptorMatcher_clear(void* instance) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_DescriptorMatcher_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::DescriptorMatcher*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_DescriptorMatcher_isMaskSupported_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::DescriptorMatcher*>(instance)->isMaskSupported();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_DescriptorMatcher_train(void* instance) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->train();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_match_const_const__InputArrayX_const__InputArrayX_vector_DMatch_X_const__InputArrayX(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, void* mask) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->match(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<const cv::_InputArray*>(trainDescriptors), *reinterpret_cast<std::vector<cv::DMatch>*>(matches), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_knnMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_int_const__InputArrayX_bool(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, int k, void* mask, bool compactResult) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->knnMatch(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<const cv::_InputArray*>(trainDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), k, *reinterpret_cast<const cv::_InputArray*>(mask), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_float_const__InputArrayX_bool(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, float maxDistance, void* mask, bool compactResult) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->radiusMatch(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<const cv::_InputArray*>(trainDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), maxDistance, *reinterpret_cast<const cv::_InputArray*>(mask), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_match_const__InputArrayX_vector_DMatch_X_const__InputArrayX(void* instance, void* queryDescriptors, void* matches, void* masks) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->match(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<std::vector<cv::DMatch>*>(matches), *reinterpret_cast<const cv::_InputArray*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_knnMatch_const__InputArrayX_vector_vector_DMatch__X_int_const__InputArrayX_bool(void* instance, void* queryDescriptors, void* matches, int k, void* masks, bool compactResult) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->knnMatch(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), k, *reinterpret_cast<const cv::_InputArray*>(masks), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_radiusMatch_const__InputArrayX_vector_vector_DMatch__X_float_const__InputArrayX_bool(void* instance, void* queryDescriptors, void* matches, float maxDistance, void* masks, bool compactResult) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->radiusMatch(*reinterpret_cast<const cv::_InputArray*>(queryDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), maxDistance, *reinterpret_cast<const cv::_InputArray*>(masks), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_write_const_const_StringX(void* instance, const char* fileName) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->write(cv::String(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_read_const_StringX(void* instance, const char* fileName) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->read(cv::String(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_read_const_FileNodeX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DescriptorMatcher_write_const_FileStorageX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DescriptorMatcher_clone_const_bool(void* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = reinterpret_cast<cv::DescriptorMatcher*>(instance)->clone(emptyTrainData);
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_DescriptorMatcher_create_const_StringX(const char* descriptorMatcherType) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(cv::String(descriptorMatcherType));
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_DescriptorMatcher_create_int(int matcherType) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(matcherType);
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_X_const_StringX(void* instance, void* fs, const char* name) {
		try {
			reinterpret_cast<cv::DescriptorMatcher*>(instance)->write(*reinterpret_cast<const cv::Ptr<cv::FileStorage>*>(fs), cv::String(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DrawMatchesFlags_delete(cv::DrawMatchesFlags* instance) {
		delete instance;
	}
	Result<void*> cv_FastFeatureDetector_create_int_bool_int(int threshold, bool nonmaxSuppression, int type) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create(threshold, nonmaxSuppression, type);
			return Ok<void*>(new cv::Ptr<cv::FastFeatureDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_FastFeatureDetector_setThreshold_int(void* instance, int threshold) {
		try {
			reinterpret_cast<cv::FastFeatureDetector*>(instance)->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FastFeatureDetector_getThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FastFeatureDetector*>(instance)->getThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FastFeatureDetector_setNonmaxSuppression_bool(void* instance, bool f) {
		try {
			reinterpret_cast<cv::FastFeatureDetector*>(instance)->setNonmaxSuppression(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_FastFeatureDetector_getNonmaxSuppression_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FastFeatureDetector*>(instance)->getNonmaxSuppression();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_FastFeatureDetector_setType_int(void* instance, int type) {
		try {
			reinterpret_cast<cv::FastFeatureDetector*>(instance)->setType(type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_FastFeatureDetector_getType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FastFeatureDetector*>(instance)->getType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_FastFeatureDetector_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::FastFeatureDetector*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Feature2D_delete(cv::Feature2D* instance) {
		delete instance;
	}
	Result_void cv_Feature2D_detect_const__InputArrayX_vector_KeyPoint_X_const__InputArrayX(void* instance, void* image, void* keypoints, void* mask) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_detect_const__InputArrayX_vector_vector_KeyPoint__X_const__InputArrayX(void* instance, void* images, void* keypoints, void* masks) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<std::vector<std::vector<cv::KeyPoint>>*>(keypoints), *reinterpret_cast<const cv::_InputArray*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX(void* instance, void* image, void* keypoints, void* descriptors) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_compute_const__InputArrayX_vector_vector_KeyPoint__X_const__OutputArrayX(void* instance, void* images, void* keypoints, void* descriptors) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<std::vector<std::vector<cv::KeyPoint>>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_detectAndCompute_const__InputArrayX_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX_bool(void* instance, void* image, void* mask, void* keypoints, void* descriptors, bool useProvidedKeypoints) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->detectAndCompute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors), useProvidedKeypoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Feature2D_descriptorSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Feature2D*>(instance)->descriptorSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Feature2D_descriptorType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Feature2D*>(instance)->descriptorType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Feature2D_defaultNorm_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Feature2D*>(instance)->defaultNorm();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Feature2D_write_const_const_StringX(void* instance, const char* fileName) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->write(cv::String(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_read_const_StringX(void* instance, const char* fileName) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->read(cv::String(fileName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_write_const_FileStorageX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Feature2D_read_const_FileNodeX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_Feature2D_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Feature2D*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_Feature2D_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Feature2D*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Feature2D_write_const_const_Ptr_FileStorage_X_const_StringX(void* instance, void* fs, const char* name) {
		try {
			reinterpret_cast<cv::Feature2D*>(instance)->write(*reinterpret_cast<const cv::Ptr<cv::FileStorage>*>(fs), cv::String(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FlannBasedMatcher_delete(cv::FlannBasedMatcher* instance) {
		delete instance;
	}
	Result<void*> cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_X_const_Ptr_SearchParams_X(void* indexParams, void* searchParams) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher(*reinterpret_cast<const cv::Ptr<cv::flann::IndexParams>*>(indexParams), *reinterpret_cast<const cv::Ptr<cv::flann::SearchParams>*>(searchParams));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_FlannBasedMatcher_add_const__InputArrayX(void* instance, void* descriptors) {
		try {
			reinterpret_cast<cv::FlannBasedMatcher*>(instance)->add(*reinterpret_cast<const cv::_InputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_clear(void* instance) {
		try {
			reinterpret_cast<cv::FlannBasedMatcher*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_read_const_FileNodeX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::FlannBasedMatcher*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_write_const_FileStorageX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::FlannBasedMatcher*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FlannBasedMatcher_train(void* instance) {
		try {
			reinterpret_cast<cv::FlannBasedMatcher*>(instance)->train();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_FlannBasedMatcher_isMaskSupported_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FlannBasedMatcher*>(instance)->isMaskSupported();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_FlannBasedMatcher_create() {
		try {
			cv::Ptr<cv::FlannBasedMatcher> ret = cv::FlannBasedMatcher::create();
			return Ok<void*>(new cv::Ptr<cv::FlannBasedMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FlannBasedMatcher_clone_const_bool(void* instance, bool emptyTrainData) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = reinterpret_cast<cv::FlannBasedMatcher*>(instance)->clone(emptyTrainData);
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_GFTTDetector_create_int_double_double_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double k) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, k);
			return Ok<void*>(new cv::Ptr<cv::GFTTDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_GFTTDetector_create_int_double_double_int_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, bool useHarrisDetector, double k) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize, useHarrisDetector, k);
			return Ok<void*>(new cv::Ptr<cv::GFTTDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_GFTTDetector_setMaxFeatures_int(void* instance, int maxFeatures) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GFTTDetector_getMaxFeatures_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getMaxFeatures();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GFTTDetector_setQualityLevel_double(void* instance, double qlevel) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setQualityLevel(qlevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getQualityLevel_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getQualityLevel();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GFTTDetector_setMinDistance_double(void* instance, double minDistance) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setMinDistance(minDistance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getMinDistance_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getMinDistance();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GFTTDetector_setBlockSize_int(void* instance, int blockSize) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GFTTDetector_getBlockSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getBlockSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GFTTDetector_setHarrisDetector_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setHarrisDetector(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_GFTTDetector_getHarrisDetector_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getHarrisDetector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_GFTTDetector_setK_double(void* instance, double k) {
		try {
			reinterpret_cast<cv::GFTTDetector*>(instance)->setK(k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GFTTDetector_getK_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getK();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_GFTTDetector_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::GFTTDetector*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_KAZE_create_bool_bool_float_int_int_int(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, int diffusivity) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok<void*>(new cv::Ptr<cv::KAZE>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KAZE_setExtended_bool(void* instance, bool extended) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setExtended(extended);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_KAZE_getExtended_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::KAZE*>(instance)->getExtended();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_KAZE_setUpright_bool(void* instance, bool upright) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setUpright(upright);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_KAZE_getUpright_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::KAZE*>(instance)->getUpright();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_KAZE_setThreshold_double(void* instance, double threshold) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_KAZE_getThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::KAZE*>(instance)->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_KAZE_setNOctaves_int(void* instance, int octaves) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setNOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_KAZE_getNOctaves_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::KAZE*>(instance)->getNOctaves();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_KAZE_setNOctaveLayers_int(void* instance, int octaveLayers) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setNOctaveLayers(octaveLayers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_KAZE_getNOctaveLayers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::KAZE*>(instance)->getNOctaveLayers();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_KAZE_setDiffusivity_int(void* instance, int diff) {
		try {
			reinterpret_cast<cv::KAZE*>(instance)->setDiffusivity(diff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_KAZE_getDiffusivity_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::KAZE*>(instance)->getDiffusivity();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_KAZE_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::KAZE*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_KeyPointsFilter_delete(cv::KeyPointsFilter* instance) {
		delete instance;
	}
	Result<void*> cv_KeyPointsFilter_KeyPointsFilter() {
		try {
			cv::KeyPointsFilter* ret = new cv::KeyPointsFilter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_X_Size_int(void* keypoints, const cv::Size* imageSize, int borderSize) {
		try {
			cv::KeyPointsFilter::runByImageBorder(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *imageSize, borderSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_X_float_float(void* keypoints, float minSize, float maxSize) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), minSize, maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_X_const_MatX(void* keypoints, void* mask) {
		try {
			cv::KeyPointsFilter::runByPixelsMask(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_X(void* keypoints) {
		try {
			cv::KeyPointsFilter::removeDuplicated(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_X(void* keypoints) {
		try {
			cv::KeyPointsFilter::removeDuplicatedSorted(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPointsFilter_retainBest_vector_KeyPoint_X_int(void* keypoints, int npoints) {
		try {
			cv::KeyPointsFilter::retainBest(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), npoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MSER_create_int_int_int_double_double_int_double_double_int(int _delta, int _min_area, int _max_area, double _max_variation, double _min_diversity, int _max_evolution, double _area_threshold, double _min_margin, int _edge_blur_size) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create(_delta, _min_area, _max_area, _max_variation, _min_diversity, _max_evolution, _area_threshold, _min_margin, _edge_blur_size);
			return Ok<void*>(new cv::Ptr<cv::MSER>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MSER_detectRegions_const__InputArrayX_vector_vector_Point__X_vector_Rect_X(void* instance, void* image, void* msers, void* bboxes) {
		try {
			reinterpret_cast<cv::MSER*>(instance)->detectRegions(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<std::vector<cv::Point>>*>(msers), *reinterpret_cast<std::vector<cv::Rect>*>(bboxes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MSER_setDelta_int(void* instance, int delta) {
		try {
			reinterpret_cast<cv::MSER*>(instance)->setDelta(delta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getDelta_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MSER*>(instance)->getDelta();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setMinArea_int(void* instance, int minArea) {
		try {
			reinterpret_cast<cv::MSER*>(instance)->setMinArea(minArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getMinArea_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MSER*>(instance)->getMinArea();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setMaxArea_int(void* instance, int maxArea) {
		try {
			reinterpret_cast<cv::MSER*>(instance)->setMaxArea(maxArea);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MSER_getMaxArea_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MSER*>(instance)->getMaxArea();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MSER_setPass2Only_bool(void* instance, bool f) {
		try {
			reinterpret_cast<cv::MSER*>(instance)->setPass2Only(f);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_MSER_getPass2Only_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::MSER*>(instance)->getPass2Only();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_MSER_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::MSER*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ORB_create_int_float_int_int_int_int_int_int_int(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, int scoreType, int patchSize, int fastThreshold) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold);
			return Ok<void*>(new cv::Ptr<cv::ORB>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ORB_setMaxFeatures_int(void* instance, int maxFeatures) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setMaxFeatures(maxFeatures);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getMaxFeatures_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getMaxFeatures();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setScaleFactor_double(void* instance, double scaleFactor) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setScaleFactor(scaleFactor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ORB_getScaleFactor_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ORB*>(instance)->getScaleFactor();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ORB_setNLevels_int(void* instance, int nlevels) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setNLevels(nlevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getNLevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getNLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setEdgeThreshold_int(void* instance, int edgeThreshold) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setEdgeThreshold(edgeThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getEdgeThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getEdgeThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setFirstLevel_int(void* instance, int firstLevel) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setFirstLevel(firstLevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getFirstLevel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getFirstLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setWTA_K_int(void* instance, int wta_k) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setWTA_K(wta_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getWTA_K_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getWTA_K();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setScoreType_int(void* instance, int scoreType) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setScoreType(scoreType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getScoreType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getScoreType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setPatchSize_int(void* instance, int patchSize) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setPatchSize(patchSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getPatchSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getPatchSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ORB_setFastThreshold_int(void* instance, int fastThreshold) {
		try {
			reinterpret_cast<cv::ORB*>(instance)->setFastThreshold(fastThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ORB_getFastThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ORB*>(instance)->getFastThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ORB_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ORB*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SimpleBlobDetector_delete(cv::SimpleBlobDetector* instance) {
		delete instance;
	}
	Result<void*> cv_SimpleBlobDetector_create_const_ParamsX(const cv::SimpleBlobDetector::Params* parameters) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create(*parameters);
			return Ok<void*>(new cv::Ptr<cv::SimpleBlobDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SimpleBlobDetector_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::SimpleBlobDetector*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::SimpleBlobDetector::Params> cv_SimpleBlobDetector_Params_Params() {
		try {
			cv::SimpleBlobDetector::Params ret;
			return Ok<cv::SimpleBlobDetector::Params>(ret);
		} OCVRS_CATCH(Result<cv::SimpleBlobDetector::Params>)
	}
	
	Result_void cv_SimpleBlobDetector_Params_read_const_FileNodeX(cv::SimpleBlobDetector::Params instance, void* fn) {
		try {
			reinterpret_cast<cv::SimpleBlobDetector::Params*>(&instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SimpleBlobDetector_Params_write_const_FileStorageX(cv::SimpleBlobDetector::Params instance, void* fs) {
		try {
			reinterpret_cast<cv::SimpleBlobDetector::Params*>(&instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
