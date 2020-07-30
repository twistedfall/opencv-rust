#include "common.hpp"
#include <opencv2/xfeatures2d.hpp>
#include "xfeatures2d_types.hpp"

extern "C" {
	Result_void cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vector_KeyPoint_R_int_bool_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, int type) {
		try {
			cv::xfeatures2d::FASTForPointSet(*image, *keypoints, threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_DMatch_R_vector_DMatch_R_const_bool_const_bool_const_double(const cv::Size* size1, const cv::Size* size2, const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, std::vector<cv::DMatch>* matchesGMS, const bool withRotation, const bool withScale, const double thresholdFactor) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *keypoints1, *keypoints2, *matches1to2, *matchesGMS, withRotation, withScale, thresholdFactor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*> cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D__Ptr_Feature2D_(cv::Ptr<cv::Feature2D>* keypoint_detector, cv::Ptr<cv::Feature2D>* descriptor_extractor) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector, *descriptor_extractor);
			return Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*> cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D_(cv::Ptr<cv::Feature2D>* keypoint_detector) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector);
			return Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>))
	}
	
	Result_void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__InputArrayR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_InputArray* mask) {
		try {
			instance->detect(*image, *keypoints, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__OutputArrayR_bool(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*> cv_xfeatures2d_BoostDesc_create_int_bool_float(int desc, bool use_scale_orientation, float scale_factor) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create(desc, use_scale_orientation, scale_factor);
			return Ok(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>))
	}
	
	Result_void cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(cv::xfeatures2d::BoostDesc* instance, const bool use_scale_orientation) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(const cv::xfeatures2d::BoostDesc* instance) {
		try {
			bool ret = instance->getUseScaleOrientation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(cv::xfeatures2d::BoostDesc* instance, const float scale_factor) {
		try {
			instance->setScaleFactor(scale_factor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_BoostDesc_getScaleFactor_const(const cv::xfeatures2d::BoostDesc* instance) {
		try {
			float ret = instance->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	void cv_BriefDescriptorExtractor_delete(cv::xfeatures2d::BriefDescriptorExtractor* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*> cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(int bytes, bool use_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create(bytes, use_orientation);
			return Ok(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::DAISY>*> cv_xfeatures2d_DAISY_create_float_int_int_int_int_const__InputArrayR_bool_bool(float radius, int q_radius, int q_theta, int q_hist, int norm, const cv::_InputArray* H, bool interpolation, bool use_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create(radius, q_radius, q_theta, q_hist, norm, *H, interpolation, use_orientation);
			return Ok(new cv::Ptr<cv::xfeatures2d::DAISY>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::DAISY>*>))
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, cv::Rect* roi, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*image, *roi, *descriptors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, const cv::_OutputArray* descriptors) {
		try {
			instance->compute(*image, *descriptors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor) {
		try {
			instance->GetDescriptor(y, x, orientation, descriptor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, double* H) {
		try {
			bool ret = instance->GetDescriptor(y, x, orientation, descriptor, H);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor) {
		try {
			instance->GetUnnormalizedDescriptor(y, x, orientation, descriptor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, double* H) {
		try {
			bool ret = instance->GetUnnormalizedDescriptor(y, x, orientation, descriptor, H);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Size_<float>> cv_xfeatures2d_Elliptic_KeyPoint_getPropAxes_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance) {
		try {
			cv::Size_<float> ret = instance->axes;
			return Ok<cv::Size_<float>>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size_<float>>))
	}
	
	Result_void cv_xfeatures2d_Elliptic_KeyPoint_setPropAxes_Size__float_(cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Size_<float>* val) {
		try {
			instance->axes = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_Elliptic_KeyPoint_getPropSi_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance) {
		try {
			float ret = instance->si;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_Elliptic_KeyPoint_setPropSi_float(cv::xfeatures2d::Elliptic_KeyPoint* instance, float val) {
		try {
			instance->si = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Matx23f> cv_xfeatures2d_Elliptic_KeyPoint_getPropTransf_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance) {
		try {
			cv::Matx23f ret = instance->transf;
			return Ok<cv::Matx23f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx23f>))
	}
	
	Result_void cv_xfeatures2d_Elliptic_KeyPoint_setPropTransf_Matx23f(cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Matx23f* val) {
		try {
			instance->transf = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Elliptic_KeyPoint_delete(cv::xfeatures2d::Elliptic_KeyPoint* instance) {
		delete instance;
	}
	Result<cv::xfeatures2d::Elliptic_KeyPoint*> cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint() {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint();
			return Ok<cv::xfeatures2d::Elliptic_KeyPoint*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::xfeatures2d::Elliptic_KeyPoint*>))
	}
	
	Result<cv::xfeatures2d::Elliptic_KeyPoint*> cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(cv::Point2f* pt, float angle, cv::Size* axes, float size, float si) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint(*pt, angle, *axes, size, si);
			return Ok<cv::xfeatures2d::Elliptic_KeyPoint*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::xfeatures2d::Elliptic_KeyPoint*>))
	}
	
	void cv_FREAK_delete(cv::xfeatures2d::FREAK* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::FREAK>*> cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vector_int_R(bool orientationNormalized, bool scaleNormalized, float patternScale, int nOctaves, const std::vector<int>* selectedPairs) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create(orientationNormalized, scaleNormalized, patternScale, nOctaves, *selectedPairs);
			return Ok(new cv::Ptr<cv::xfeatures2d::FREAK>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::FREAK>*>))
	}
	
	void cv_HarrisLaplaceFeatureDetector_delete(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*> cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(int numOctaves, float corn_thresh, float DOG_thresh, int maxCorners, int num_layers) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create(numOctaves, corn_thresh, DOG_thresh, maxCorners, num_layers);
			return Ok(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*>))
	}
	
	void cv_LATCH_delete(cv::xfeatures2d::LATCH* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::LATCH>*> cv_xfeatures2d_LATCH_create_int_bool_int_double(int bytes, bool rotationInvariance, int half_ssd_size, double sigma) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create(bytes, rotationInvariance, half_ssd_size, sigma);
			return Ok(new cv::Ptr<cv::xfeatures2d::LATCH>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::LATCH>*>))
	}
	
	void cv_LUCID_delete(cv::xfeatures2d::LUCID* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::LUCID>*> cv_xfeatures2d_LUCID_create_const_int_const_int(const int lucid_kernel, const int blur_kernel) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create(lucid_kernel, blur_kernel);
			return Ok(new cv::Ptr<cv::xfeatures2d::LUCID>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::LUCID>*>))
	}
	
	void cv_MSDDetector_delete(cv::xfeatures2d::MSDDetector* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*> cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(int m_patch_radius, int m_search_area_radius, int m_nms_radius, int m_nms_scale_radius, float m_th_saliency, int m_kNN, float m_scale_factor, int m_n_scales, bool m_compute_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_kNN, m_scale_factor, m_n_scales, m_compute_orientation);
			return Ok(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*> cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(const int initSampleCount, const int initSeedCount, const int pointDistribution) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(initSampleCount, initSeedCount, pointDistribution);
			return Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*> cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_int(const std::vector<cv::Point2f>* initSamplingPoints, const int initSeedCount) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, initSeedCount);
			return Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*> cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_vector_int_R(const std::vector<cv::Point2f>* initSamplingPoints, const std::vector<int>* initClusterSeedIndexes) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, *initClusterSeedIndexes);
			return Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(const cv::xfeatures2d::PCTSignatures* instance, const cv::_InputArray* image, const cv::_OutputArray* signature) {
		try {
			instance->computeSignature(*image, *signature);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vector_Mat_R_vector_Mat_R(const cv::xfeatures2d::PCTSignatures* instance, const std::vector<cv::Mat>* images, std::vector<cv::Mat>* signatures) {
		try {
			instance->computeSignatures(*images, *signatures);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(const cv::_InputArray* source, const cv::_InputArray* signature, const cv::_OutputArray* result, float radiusToShorterSideRatio, int borderThickness) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*source, *signature, *result, radiusToShorterSideRatio, borderThickness);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_generateInitPoints_vector_Point2f_R_const_int_int(std::vector<cv::Point2f>* initPoints, const int count, int pointDistribution) {
		try {
			cv::xfeatures2d::PCTSignatures::generateInitPoints(*initPoints, count, pointDistribution);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getSampleCount_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getSampleCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getGrayscaleBits();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(cv::xfeatures2d::PCTSignatures* instance, int grayscaleBits) {
		try {
			instance->setGrayscaleBits(grayscaleBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getWindowRadius_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getWindowRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWindowRadius_int(cv::xfeatures2d::PCTSignatures* instance, int radius) {
		try {
			instance->setWindowRadius(radius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightX_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightX();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightX_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightX(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightY_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightY();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightY_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightY(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightL_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightL();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightL_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightL(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightA_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightA();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightA_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightA(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightB_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightB();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightB_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightB(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightContrast_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightContrast();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightContrast_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightContrast(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getWeightEntropy();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(cv::xfeatures2d::PCTSignatures* instance, float weight) {
		try {
			instance->setWeightEntropy(weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Point2f>*> cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->getSamplingPoints();
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point2f>*>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeight_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value) {
		try {
			instance->setWeight(idx, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeights_const_vector_float_R(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* weights) {
		try {
			instance->setWeights(*weights);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setTranslation_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value) {
		try {
			instance->setTranslation(idx, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setTranslations_const_vector_float_R(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* translations) {
		try {
			instance->setTranslations(*translations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setSamplingPoints_vector_Point2f_(cv::xfeatures2d::PCTSignatures* instance, std::vector<cv::Point2f>* samplingPoints) {
		try {
			instance->setSamplingPoints(*samplingPoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			std::vector<int> ret = instance->getInitSeedIndexes();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vector_int_(cv::xfeatures2d::PCTSignatures* instance, std::vector<int>* initSeedIndexes) {
		try {
			instance->setInitSeedIndexes(*initSeedIndexes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getInitSeedCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getIterationCount_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getIterationCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setIterationCount_int(cv::xfeatures2d::PCTSignatures* instance, int iterationCount) {
		try {
			instance->setIterationCount(iterationCount);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getMaxClustersCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(cv::xfeatures2d::PCTSignatures* instance, int maxClustersCount) {
		try {
			instance->setMaxClustersCount(maxClustersCount);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getClusterMinSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(cv::xfeatures2d::PCTSignatures* instance, int clusterMinSize) {
		try {
			instance->setClusterMinSize(clusterMinSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getJoiningDistance();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(cv::xfeatures2d::PCTSignatures* instance, float joiningDistance) {
		try {
			instance->setJoiningDistance(joiningDistance);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getDropThreshold_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			float ret = instance->getDropThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setDropThreshold_float(cv::xfeatures2d::PCTSignatures* instance, float dropThreshold) {
		try {
			instance->setDropThreshold(dropThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(const cv::xfeatures2d::PCTSignatures* instance) {
		try {
			int ret = instance->getDistanceFunction();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(cv::xfeatures2d::PCTSignatures* instance, int distanceFunction) {
		try {
			instance->setDistanceFunction(distanceFunction);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*> cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(const int distanceFunction, const int similarityFunction, const float similarityParameter) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create(distanceFunction, similarityFunction, similarityParameter);
			return Ok(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>))
	}
	
	Result<float> cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(const cv::xfeatures2d::PCTSignaturesSQFD* instance, const cv::_InputArray* _signature0, const cv::_InputArray* _signature1) {
		try {
			float ret = instance->computeQuadraticFormDistance(*_signature0, *_signature1);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vector_Mat_R_vector_float_R(const cv::xfeatures2d::PCTSignaturesSQFD* instance, const cv::Mat* sourceSignature, const std::vector<cv::Mat>* imageSignatures, std::vector<float>* distances) {
		try {
			instance->computeQuadraticFormDistances(*sourceSignature, *imageSignatures, *distances);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::SURF>*> cv_xfeatures2d_SURF_create_double_int_int_bool_bool(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
			return Ok(new cv::Ptr<cv::xfeatures2d::SURF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::SURF>*>))
	}
	
	Result_void cv_xfeatures2d_SURF_setHessianThreshold_double(cv::xfeatures2d::SURF* instance, double hessianThreshold) {
		try {
			instance->setHessianThreshold(hessianThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_xfeatures2d_SURF_getHessianThreshold_const(const cv::xfeatures2d::SURF* instance) {
		try {
			double ret = instance->getHessianThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_xfeatures2d_SURF_setNOctaves_int(cv::xfeatures2d::SURF* instance, int nOctaves) {
		try {
			instance->setNOctaves(nOctaves);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_SURF_getNOctaves_const(const cv::xfeatures2d::SURF* instance) {
		try {
			int ret = instance->getNOctaves();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_SURF_setNOctaveLayers_int(cv::xfeatures2d::SURF* instance, int nOctaveLayers) {
		try {
			instance->setNOctaveLayers(nOctaveLayers);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_xfeatures2d_SURF_getNOctaveLayers_const(const cv::xfeatures2d::SURF* instance) {
		try {
			int ret = instance->getNOctaveLayers();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_xfeatures2d_SURF_setExtended_bool(cv::xfeatures2d::SURF* instance, bool extended) {
		try {
			instance->setExtended(extended);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_SURF_getExtended_const(const cv::xfeatures2d::SURF* instance) {
		try {
			bool ret = instance->getExtended();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_xfeatures2d_SURF_setUpright_bool(cv::xfeatures2d::SURF* instance, bool upright) {
		try {
			instance->setUpright(upright);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_SURF_getUpright_const(const cv::xfeatures2d::SURF* instance) {
		try {
			bool ret = instance->getUpright();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_StarDetector_delete(cv::xfeatures2d::StarDetector* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::xfeatures2d::StarDetector>*> cv_xfeatures2d_StarDetector_create_int_int_int_int_int(int maxSize, int responseThreshold, int lineThresholdProjected, int lineThresholdBinarized, int suppressNonmaxSize) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create(maxSize, responseThreshold, lineThresholdProjected, lineThresholdBinarized, suppressNonmaxSize);
			return Ok(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>))
	}
	
	Result<cv::Ptr<cv::xfeatures2d::VGG>*> cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(int desc, float isigma, bool img_normalize, bool use_scale_orientation, float scale_factor, bool dsc_normalize) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize);
			return Ok(new cv::Ptr<cv::xfeatures2d::VGG>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xfeatures2d::VGG>*>))
	}
	
	Result_void cv_xfeatures2d_VGG_setSigma_const_float(cv::xfeatures2d::VGG* instance, const float isigma) {
		try {
			instance->setSigma(isigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_VGG_getSigma_const(const cv::xfeatures2d::VGG* instance) {
		try {
			float ret = instance->getSigma();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(cv::xfeatures2d::VGG* instance, const bool img_normalize) {
		try {
			instance->setUseNormalizeImage(img_normalize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseNormalizeImage_const(const cv::xfeatures2d::VGG* instance) {
		try {
			bool ret = instance->getUseNormalizeImage();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(cv::xfeatures2d::VGG* instance, const bool use_scale_orientation) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseScaleOrientation_const(const cv::xfeatures2d::VGG* instance) {
		try {
			bool ret = instance->getUseScaleOrientation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_xfeatures2d_VGG_setScaleFactor_const_float(cv::xfeatures2d::VGG* instance, const float scale_factor) {
		try {
			instance->setScaleFactor(scale_factor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_xfeatures2d_VGG_getScaleFactor_const(const cv::xfeatures2d::VGG* instance) {
		try {
			float ret = instance->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(cv::xfeatures2d::VGG* instance, const bool dsc_normalize) {
		try {
			instance->setUseNormalizeDescriptor(dsc_normalize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(const cv::xfeatures2d::VGG* instance) {
		try {
			bool ret = instance->getUseNormalizeDescriptor();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
}
