#include "common.hpp"
#include <opencv2/xfeatures2d.hpp>
#include "xfeatures2d_types.hpp"

extern "C" {
	Result_void cv_xfeatures2d_FASTForPointSet_const__InputArrayX_vector_KeyPoint_X_int_bool_DetectorType(void* image, void* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type) {
		try {
			cv::xfeatures2d::FASTForPointSet(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), threshold, nonmaxSuppression, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_matchGMS_const_SizeX_const_SizeX_const_vector_KeyPoint_X_const_vector_KeyPoint_X_const_vector_DMatch_X_vector_DMatch_X_bool_bool_double(const cv::Size* size1, const cv::Size* size2, void* keypoints1, void* keypoints2, void* matches1to2, void* matchesGMS, bool withRotation, bool withScale, double thresholdFactor) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints1), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints2), *reinterpret_cast<const std::vector<cv::DMatch>*>(matches1to2), *reinterpret_cast<std::vector<cv::DMatch>*>(matchesGMS), withRotation, withScale, thresholdFactor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_matchLOGOS_const_vector_KeyPoint_X_const_vector_KeyPoint_X_const_vector_int_X_const_vector_int_X_vector_DMatch_X(void* keypoints1, void* keypoints2, void* nn1, void* nn2, void* matches1to2) {
		try {
			cv::xfeatures2d::matchLOGOS(*reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints1), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints2), *reinterpret_cast<const std::vector<int>*>(nn1), *reinterpret_cast<const std::vector<int>*>(nn2), *reinterpret_cast<std::vector<cv::DMatch>*>(matches1to2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xfeatures2d_AffineFeature2D_create_Ptr_FeatureDetector__Ptr_DescriptorExtractor_(void* keypoint_detector, void* descriptor_extractor) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(keypoint_detector), *reinterpret_cast<cv::Ptr<cv::DescriptorExtractor>*>(descriptor_extractor));
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_AffineFeature2D_create_Ptr_FeatureDetector_(void* keypoint_detector) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(keypoint_detector));
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayX_vector_Elliptic_KeyPoint_X_const__InputArrayX(void* instance, void* image, void* keypoints, void* mask) {
		try {
			reinterpret_cast<cv::xfeatures2d::AffineFeature2D*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayX_const__InputArrayX_vector_Elliptic_KeyPoint_X_const__OutputArrayX_bool(void* instance, void* image, void* mask, void* keypoints, void* descriptors, bool useProvidedKeypoints) {
		try {
			reinterpret_cast<cv::xfeatures2d::AffineFeature2D*>(instance)->detectAndCompute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors), useProvidedKeypoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xfeatures2d_BoostDesc_create_int_bool_float(int desc, bool use_scale_orientation, float scale_factor) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create(desc, use_scale_orientation, scale_factor);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_BoostDesc_setUseScaleOrientation_bool(void* instance, bool use_scale_orientation) {
		try {
			reinterpret_cast<cv::xfeatures2d::BoostDesc*>(instance)->setUseScaleOrientation(use_scale_orientation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::BoostDesc*>(instance)->getUseScaleOrientation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_xfeatures2d_BoostDesc_setScaleFactor_float(void* instance, float scale_factor) {
		try {
			reinterpret_cast<cv::xfeatures2d::BoostDesc*>(instance)->setScaleFactor(scale_factor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_BoostDesc_getScaleFactor_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::BoostDesc*>(instance)->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	void cv_BriefDescriptorExtractor_delete(cv::xfeatures2d::BriefDescriptorExtractor* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(int bytes, bool use_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create(bytes, use_orientation);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayX_bool_bool(float radius, int q_radius, int q_theta, int q_hist, cv::xfeatures2d::DAISY::NormalizationType norm, void* H, bool interpolation, bool use_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create(radius, q_radius, q_theta, q_hist, norm, *reinterpret_cast<const cv::_InputArray*>(H), interpolation, use_orientation);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::DAISY>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX(void* instance, void* image, void* keypoints, void* descriptors) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayX_vector_vector_KeyPoint__X_const__OutputArrayX(void* instance, void* images, void* keypoints, void* descriptors) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<std::vector<std::vector<cv::KeyPoint>>*>(keypoints), *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayX_Rect_const__OutputArrayX(void* instance, void* image, cv::Rect roi, void* descriptors) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), roi, *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_DAISY_compute_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* descriptors) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(void* instance, double y, double x, int orientation, float* descriptor) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->GetDescriptor(y, x, orientation, descriptor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(void* instance, double y, double x, int orientation, float* descriptor, double* H) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->GetDescriptor(y, x, orientation, descriptor, H);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(void* instance, double y, double x, int orientation, float* descriptor) {
		try {
			reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->GetUnnormalizedDescriptor(y, x, orientation, descriptor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(void* instance, double y, double x, int orientation, float* descriptor, double* H) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::DAISY*>(instance)->GetUnnormalizedDescriptor(y, x, orientation, descriptor, H);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Size_<float>> cv_xfeatures2d_Elliptic_KeyPoint_axes_const(void* instance) {
		try {
			cv::Size_<float> ret = reinterpret_cast<cv::xfeatures2d::Elliptic_KeyPoint*>(instance)->axes;
			return Ok<cv::Size_<float>>(ret);
		} OCVRS_CATCH(Result<cv::Size_<float>>)
	}
	
	Result_void cv_xfeatures2d_Elliptic_KeyPoint_setAxes_Size__float_(void* instance, cv::Size_<float> val) {
		try {
			reinterpret_cast<cv::xfeatures2d::Elliptic_KeyPoint*>(instance)->axes = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_Elliptic_KeyPoint_si_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::Elliptic_KeyPoint*>(instance)->si;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_Elliptic_KeyPoint_setSi_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::xfeatures2d::Elliptic_KeyPoint*>(instance)->si = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Elliptic_KeyPoint_delete(cv::xfeatures2d::Elliptic_KeyPoint* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint() {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(cv::Point2f pt, float angle, cv::Size axes, float size, float si) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint(pt, angle, axes, size, si);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_xfeatures2d_FREAK_NB_SCALES_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::FREAK*>(instance)->NB_SCALES;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_xfeatures2d_FREAK_NB_PAIRS_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::FREAK*>(instance)->NB_PAIRS;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_xfeatures2d_FREAK_NB_ORIENPAIRS_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::FREAK*>(instance)->NB_ORIENPAIRS;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_FREAK_delete(cv::xfeatures2d::FREAK* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vector_int_X(bool orientationNormalized, bool scaleNormalized, float patternScale, int nOctaves, void* selectedPairs) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create(orientationNormalized, scaleNormalized, patternScale, nOctaves, *reinterpret_cast<const std::vector<int>*>(selectedPairs));
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::FREAK>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_HarrisLaplaceFeatureDetector_delete(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(int numOctaves, float corn_thresh, float DOG_thresh, int maxCorners, int num_layers) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create(numOctaves, corn_thresh, DOG_thresh, maxCorners, num_layers);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LATCH_delete(cv::xfeatures2d::LATCH* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_LATCH_create_int_bool_int_double(int bytes, bool rotationInvariance, int half_ssd_size, double sigma) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create(bytes, rotationInvariance, half_ssd_size, sigma);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::LATCH>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LUCID_delete(cv::xfeatures2d::LUCID* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_LUCID_create_int_int(int lucid_kernel, int blur_kernel) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create(lucid_kernel, blur_kernel);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::LUCID>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_MSDDetector_delete(cv::xfeatures2d::MSDDetector* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(int m_patch_radius, int m_search_area_radius, int m_nms_radius, int m_nms_scale_radius, float m_th_saliency, int m_kNN, float m_scale_factor, int m_n_scales, bool m_compute_orientation) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_kNN, m_scale_factor, m_n_scales, m_compute_orientation);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignatures_create_int_int_int(int initSampleCount, int initSeedCount, int pointDistribution) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(initSampleCount, initSeedCount, pointDistribution);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_X_int(void* initSamplingPoints, int initSeedCount) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*reinterpret_cast<const std::vector<cv::Point2f>*>(initSamplingPoints), initSeedCount);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_X_const_vector_int_X(void* initSamplingPoints, void* initClusterSeedIndexes) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*reinterpret_cast<const std::vector<cv::Point2f>*>(initSamplingPoints), *reinterpret_cast<const std::vector<int>*>(initClusterSeedIndexes));
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* signature) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->computeSignature(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(signature));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vector_Mat_X_vector_Mat_X(void* instance, void* images, void* signatures) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->computeSignatures(*reinterpret_cast<const std::vector<cv::Mat>*>(images), *reinterpret_cast<std::vector<cv::Mat>*>(signatures));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayX_const__InputArrayX_const__OutputArrayX_float_int(void* source, void* signature, void* result, float radiusToShorterSideRatio, int borderThickness) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*reinterpret_cast<const cv::_InputArray*>(source), *reinterpret_cast<const cv::_InputArray*>(signature), *reinterpret_cast<const cv::_OutputArray*>(result), radiusToShorterSideRatio, borderThickness);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_generateInitPoints_vector_Point2f_X_int_int(void* initPoints, int count, int pointDistribution) {
		try {
			cv::xfeatures2d::PCTSignatures::generateInitPoints(*reinterpret_cast<std::vector<cv::Point2f>*>(initPoints), count, pointDistribution);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getSampleCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getSampleCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getGrayscaleBits();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(void* instance, int grayscaleBits) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setGrayscaleBits(grayscaleBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getWindowRadius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWindowRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWindowRadius_int(void* instance, int radius) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWindowRadius(radius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightX_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightX();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightX_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightX(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightY_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightY();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightY_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightY(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightL_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightL();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightL_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightL(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightA_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightA();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightA_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightA(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightB_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightB();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightB_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightB(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightContrast_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightContrast();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightContrast_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightContrast(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getWeightEntropy();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(void* instance, float weight) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeightEntropy(weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(void* instance) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getSamplingPoints();
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeight_int_float(void* instance, int idx, float value) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeight(idx, value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setWeights_const_vector_float_X(void* instance, void* weights) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setWeights(*reinterpret_cast<const std::vector<float>*>(weights));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setTranslation_int_float(void* instance, int idx, float value) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setTranslation(idx, value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setTranslations_const_vector_float_X(void* instance, void* translations) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setTranslations(*reinterpret_cast<const std::vector<float>*>(translations));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setSamplingPoints_vector_Point2f_(void* instance, void* samplingPoints) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setSamplingPoints(*reinterpret_cast<std::vector<cv::Point2f>*>(samplingPoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getInitSeedIndexes();
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vector_int_(void* instance, void* initSeedIndexes) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setInitSeedIndexes(*reinterpret_cast<std::vector<int>*>(initSeedIndexes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getInitSeedCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getIterationCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getIterationCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setIterationCount_int(void* instance, int iterationCount) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setIterationCount(iterationCount);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getMaxClustersCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(void* instance, int maxClustersCount) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setMaxClustersCount(maxClustersCount);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getClusterMinSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(void* instance, int clusterMinSize) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setClusterMinSize(clusterMinSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getJoiningDistance();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(void* instance, float joiningDistance) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setJoiningDistance(joiningDistance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_PCTSignatures_getDropThreshold_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getDropThreshold();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setDropThreshold_float(void* instance, float dropThreshold) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setDropThreshold(dropThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->getDistanceFunction();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(void* instance, int distanceFunction) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignatures*>(instance)->setDistanceFunction(distanceFunction);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_xfeatures2d_PCTSignaturesSQFD_create_int_int_float(int distanceFunction, int similarityFunction, float similarityParameter) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create(distanceFunction, similarityFunction, similarityParameter);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayX_const__InputArrayX(void* instance, void* _signature0, void* _signature1) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::PCTSignaturesSQFD*>(instance)->computeQuadraticFormDistance(*reinterpret_cast<const cv::_InputArray*>(_signature0), *reinterpret_cast<const cv::_InputArray*>(_signature1));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatX_const_vector_Mat_X_vector_float_X(void* instance, void* sourceSignature, void* imageSignatures, void* distances) {
		try {
			reinterpret_cast<cv::xfeatures2d::PCTSignaturesSQFD*>(instance)->computeQuadraticFormDistances(*reinterpret_cast<const cv::Mat*>(sourceSignature), *reinterpret_cast<const std::vector<cv::Mat>*>(imageSignatures), *reinterpret_cast<std::vector<float>*>(distances));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SIFT_delete(cv::xfeatures2d::SIFT* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_SIFT_create_int_int_double_double_double(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
		try {
			cv::Ptr<cv::xfeatures2d::SIFT> ret = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::SIFT>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_SURF_create_double_int_int_bool_bool(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::SURF>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_SURF_setHessianThreshold_double(void* instance, double hessianThreshold) {
		try {
			reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->setHessianThreshold(hessianThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_xfeatures2d_SURF_getHessianThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->getHessianThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_xfeatures2d_SURF_setNOctaves_int(void* instance, int nOctaves) {
		try {
			reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->setNOctaves(nOctaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_SURF_getNOctaves_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->getNOctaves();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_SURF_setNOctaveLayers_int(void* instance, int nOctaveLayers) {
		try {
			reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->setNOctaveLayers(nOctaveLayers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_xfeatures2d_SURF_getNOctaveLayers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->getNOctaveLayers();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_xfeatures2d_SURF_setExtended_bool(void* instance, bool extended) {
		try {
			reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->setExtended(extended);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_SURF_getExtended_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->getExtended();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_xfeatures2d_SURF_setUpright_bool(void* instance, bool upright) {
		try {
			reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->setUpright(upright);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_SURF_getUpright_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::SURF*>(instance)->getUpright();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_StarDetector_delete(cv::xfeatures2d::StarDetector* instance) {
		delete instance;
	}
	Result<void*> cv_xfeatures2d_StarDetector_create_int_int_int_int_int(int maxSize, int responseThreshold, int lineThresholdProjected, int lineThresholdBinarized, int suppressNonmaxSize) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create(maxSize, responseThreshold, lineThresholdProjected, lineThresholdBinarized, suppressNonmaxSize);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(int desc, float isigma, bool img_normalize, bool use_scale_orientation, float scale_factor, bool dsc_normalize) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize);
			return Ok<void*>(new cv::Ptr<cv::xfeatures2d::VGG>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_xfeatures2d_VGG_setSigma_float(void* instance, float isigma) {
		try {
			reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->setSigma(isigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_VGG_getSigma_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->getSigma();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_VGG_setUseNormalizeImage_bool(void* instance, bool img_normalize) {
		try {
			reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->setUseNormalizeImage(img_normalize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseNormalizeImage_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->getUseNormalizeImage();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_xfeatures2d_VGG_setUseScaleOrientation_bool(void* instance, bool use_scale_orientation) {
		try {
			reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->setUseScaleOrientation(use_scale_orientation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseScaleOrientation_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->getUseScaleOrientation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_xfeatures2d_VGG_setScaleFactor_float(void* instance, float scale_factor) {
		try {
			reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->setScaleFactor(scale_factor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_xfeatures2d_VGG_getScaleFactor_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_xfeatures2d_VGG_setUseNormalizeDescriptor_bool(void* instance, bool dsc_normalize) {
		try {
			reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->setUseNormalizeDescriptor(dsc_normalize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::xfeatures2d::VGG*>(instance)->getUseNormalizeDescriptor();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}
