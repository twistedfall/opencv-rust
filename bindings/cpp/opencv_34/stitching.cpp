#include "ocvrs_common.hpp"
#include <opencv2/stitching.hpp>
#include "stitching_types.hpp"

extern "C" {
	Result_void cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vector_UMat_R(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::createLaplacePyrGpu(*img, num_levels, *pyr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_createLaplacePyr_const__InputArrayR_int_vector_UMat_R(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::createLaplacePyr(*img, num_levels, *pyr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(const cv::_InputArray* mask, float sharpness, const cv::_InputOutputArray* weight) {
		try {
			cv::detail::createWeightMap(*mask, sharpness, *weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_R_GraphR_vector_int_R(int num_images, const std::vector<cv::detail::MatchesInfo>* pairwise_matches, cv::detail::Graph* span_tree, std::vector<int>* centers) {
		try {
			cv::detail::findMaxSpanningTree(num_images, *pairwise_matches, *span_tree, *centers);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_detail_leaveBiggestComponent_vector_ImageFeatures_R_vector_MatchesInfo_R_float(std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold) {
		try {
			std::vector<int> ret = cv::detail::leaveBiggestComponent(*features, *pairwise_matches, conf_threshold);
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<void*> cv_detail_matchesGraphAsString_vector_String_R_vector_MatchesInfo_R_float(std::vector<cv::String>* pathes, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold) {
		try {
			cv::String ret = cv::detail::matchesGraphAsString(*pathes, *pairwise_matches, conf_threshold);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* weight, const cv::_InputOutputArray* src) {
		try {
			cv::detail::normalizeUsingWeightMap(*weight, *src);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_detail_overlapRoi_Point_Point_Size_Size_RectR(cv::Point* tl1, cv::Point* tl2, cv::Size* sz1, cv::Size* sz2, cv::Rect* roi) {
		try {
			bool ret = cv::detail::overlapRoi(*tl1, *tl2, *sz1, *sz2, *roi);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_R(std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyrGpu(*pyr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyr_vector_UMat_R(std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyr(*pyr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_detail_resultRoiIntersection_const_vector_Point_R_const_vector_Size_R(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoiIntersection(*corners, *sizes);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_R_const_vector_Size_R(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *sizes);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_R_const_vector_UMat_R(const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *images);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_resultTl_const_vector_Point_R(const std::vector<cv::Point>* corners) {
		try {
			cv::Point ret = cv::detail::resultTl(*corners);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_detail_selectRandomSubset_int_int_vector_int_R(int count, int size, std::vector<int>* subset) {
		try {
			cv::detail::selectRandomSubset(count, size, *subset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_stitchingLogLevel() {
		try {
			int ret = cv::detail::stitchingLogLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_waveCorrect_vector_Mat_R_WaveCorrectKind(std::vector<cv::Mat>* rmats, cv::detail::WaveCorrectKind kind) {
		try {
			cv::detail::waveCorrect(*rmats, kind);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_AffineWarper_delete(cv::AffineWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_AffineWarper_create_const_float(const cv::AffineWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CompressedRectilinearPortraitWarper_delete(cv::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::CompressedRectilinearPortraitWarper*> cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper(A, B);
			return Ok<cv::CompressedRectilinearPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CompressedRectilinearPortraitWarper*>))
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CompressedRectilinearPortraitWarper_create_const_float(const cv::CompressedRectilinearPortraitWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CompressedRectilinearWarper_delete(cv::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<cv::CompressedRectilinearWarper*> cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper(A, B);
			return Ok<cv::CompressedRectilinearWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CompressedRectilinearWarper*>))
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CompressedRectilinearWarper_create_const_float(const cv::CompressedRectilinearWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CylindricalWarper_delete(cv::CylindricalWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CylindricalWarper_create_const_float(const cv::CylindricalWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_FisheyeWarper_delete(cv::FisheyeWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_FisheyeWarper_create_const_float(const cv::FisheyeWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_MercatorWarper_delete(cv::MercatorWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_MercatorWarper_create_const_float(const cv::MercatorWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PaniniPortraitWarper_delete(cv::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::PaniniPortraitWarper*> cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(float A, float B) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper(A, B);
			return Ok<cv::PaniniPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PaniniPortraitWarper*>))
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PaniniPortraitWarper_create_const_float(const cv::PaniniPortraitWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PaniniWarper_delete(cv::PaniniWarper* instance) {
		delete instance;
	}
	Result<cv::PaniniWarper*> cv_PaniniWarper_PaniniWarper_float_float(float A, float B) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper(A, B);
			return Ok<cv::PaniniWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PaniniWarper*>))
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PaniniWarper_create_const_float(const cv::PaniniWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PlaneWarper_delete(cv::PlaneWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PlaneWarper_create_const_float(const cv::PlaneWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_SphericalWarper_delete(cv::SphericalWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_SphericalWarper_create_const_float(const cv::SphericalWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_StereographicWarper_delete(cv::StereographicWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_StereographicWarper_create_const_float(const cv::StereographicWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_Stitcher_delete(cv::Stitcher* instance) {
		delete instance;
	}
	Result<cv::Stitcher*> cv_Stitcher_createDefault_bool(bool try_use_gpu) {
		try {
			cv::Stitcher ret = cv::Stitcher::createDefault(try_use_gpu);
			return Ok(new cv::Stitcher(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher*>))
	}
	
	Result<cv::Ptr<cv::Stitcher>*> cv_Stitcher_create_Mode_bool(cv::Stitcher::Mode mode, bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create(mode, try_use_gpu);
			return Ok(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Stitcher>*>))
	}
	
	Result<double> cv_Stitcher_registrationResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->registrationResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_Stitcher_setRegistrationResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setRegistrationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_Stitcher_seamEstimationResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->seamEstimationResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_Stitcher_setSeamEstimationResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setSeamEstimationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_Stitcher_compositingResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->compositingResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_Stitcher_setCompositingResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setCompositingResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_Stitcher_panoConfidenceThresh_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->panoConfidenceThresh();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_Stitcher_setPanoConfidenceThresh_double(cv::Stitcher* instance, double conf_thresh) {
		try {
			instance->setPanoConfidenceThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_Stitcher_waveCorrection_const(const cv::Stitcher* instance) {
		try {
			bool ret = instance->waveCorrection();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_Stitcher_setWaveCorrection_bool(cv::Stitcher* instance, bool flag) {
		try {
			instance->setWaveCorrection(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::detail::WaveCorrectKind> cv_Stitcher_waveCorrectKind_const(const cv::Stitcher* instance) {
		try {
			cv::detail::WaveCorrectKind ret = instance->waveCorrectKind();
			return Ok<cv::detail::WaveCorrectKind>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::WaveCorrectKind>))
	}
	
	Result_void cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(cv::Stitcher* instance, cv::detail::WaveCorrectKind kind) {
		try {
			instance->setWaveCorrectKind(kind);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::FeaturesFinder>*> cv_Stitcher_featuresFinder(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesFinder> ret = instance->featuresFinder();
			return Ok(new cv::Ptr<cv::detail::FeaturesFinder>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::FeaturesFinder>*>))
	}
	
	Result<const cv::Ptr<cv::detail::FeaturesFinder>*> cv_Stitcher_featuresFinder_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::FeaturesFinder> ret = instance->featuresFinder();
			return Ok(new const cv::Ptr<cv::detail::FeaturesFinder>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::FeaturesFinder>*>))
	}
	
	Result_void cv_Stitcher_setFeaturesFinder_Ptr_FeaturesFinder_(cv::Stitcher* instance, cv::Ptr<cv::detail::FeaturesFinder>* features_finder) {
		try {
			instance->setFeaturesFinder(*features_finder);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::FeaturesMatcher>*> cv_Stitcher_featuresMatcher(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			return Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::FeaturesMatcher>*>))
	}
	
	Result<const cv::Ptr<cv::detail::FeaturesMatcher>*> cv_Stitcher_featuresMatcher_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			return Ok(new const cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::FeaturesMatcher>*>))
	}
	
	Result_void cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(cv::Stitcher* instance, cv::Ptr<cv::detail::FeaturesMatcher>* features_matcher) {
		try {
			instance->setFeaturesMatcher(*features_matcher);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::UMat*> cv_Stitcher_matchingMask_const(const cv::Stitcher* instance) {
		try {
			const cv::UMat ret = instance->matchingMask();
			return Ok(new const cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::UMat*>))
	}
	
	Result_void cv_Stitcher_setMatchingMask_const_UMatR(cv::Stitcher* instance, const cv::UMat* mask) {
		try {
			instance->setMatchingMask(*mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::BundleAdjusterBase>*> cv_Stitcher_bundleAdjuster(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			return Ok(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>))
	}
	
	Result<const cv::Ptr<cv::detail::BundleAdjusterBase>*> cv_Stitcher_bundleAdjuster_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			return Ok(new const cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::BundleAdjusterBase>*>))
	}
	
	Result_void cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(cv::Stitcher* instance, cv::Ptr<cv::detail::BundleAdjusterBase>* bundle_adjuster) {
		try {
			instance->setBundleAdjuster(*bundle_adjuster);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::WarperCreator>*> cv_Stitcher_warper(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::WarperCreator> ret = instance->warper();
			return Ok(new cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::WarperCreator>*>))
	}
	
	Result<const cv::Ptr<cv::WarperCreator>*> cv_Stitcher_warper_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::WarperCreator> ret = instance->warper();
			return Ok(new const cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::WarperCreator>*>))
	}
	
	Result_void cv_Stitcher_setWarper_Ptr_WarperCreator_(cv::Stitcher* instance, cv::Ptr<cv::WarperCreator>* creator) {
		try {
			instance->setWarper(*creator);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::ExposureCompensator>*> cv_Stitcher_exposureCompensator(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			return Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	Result<const cv::Ptr<cv::detail::ExposureCompensator>*> cv_Stitcher_exposureCompensator_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			return Ok(new const cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	Result_void cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(cv::Stitcher* instance, cv::Ptr<cv::detail::ExposureCompensator>* exposure_comp) {
		try {
			instance->setExposureCompensator(*exposure_comp);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::SeamFinder>*> cv_Stitcher_seamFinder(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			return Ok(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::SeamFinder>*>))
	}
	
	Result<const cv::Ptr<cv::detail::SeamFinder>*> cv_Stitcher_seamFinder_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			return Ok(new const cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::SeamFinder>*>))
	}
	
	Result_void cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(cv::Stitcher* instance, cv::Ptr<cv::detail::SeamFinder>* seam_finder) {
		try {
			instance->setSeamFinder(*seam_finder);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::Blender>*> cv_Stitcher_blender(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::Blender> ret = instance->blender();
			return Ok(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Blender>*>))
	}
	
	Result<const cv::Ptr<cv::detail::Blender>*> cv_Stitcher_blender_const(const cv::Stitcher* instance) {
		try {
			const cv::Ptr<cv::detail::Blender> ret = instance->blender();
			return Ok(new const cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Ptr<cv::detail::Blender>*>))
	}
	
	Result_void cv_Stitcher_setBlender_Ptr_Blender_(cv::Stitcher* instance, cv::Ptr<cv::detail::Blender>* b) {
		try {
			instance->setBlender(*b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_estimateTransform_const__InputArrayR(cv::Stitcher* instance, const cv::_InputArray* images) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_estimateTransform_const__InputArrayR_const_vector_vector_Rect__R(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<std::vector<cv::Rect>>* rois) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images, *rois);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__OutputArrayR(cv::Stitcher* instance, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*pano);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*images, *pano);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *pano);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayR_const_vector_vector_Rect__R_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<std::vector<cv::Rect>>* rois, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *rois, *pano);
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	Result<std::vector<int>*> cv_Stitcher_component_const(const cv::Stitcher* instance) {
		try {
			std::vector<int> ret = instance->component();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<std::vector<cv::detail::CameraParams>*> cv_Stitcher_cameras_const(const cv::Stitcher* instance) {
		try {
			std::vector<cv::detail::CameraParams> ret = instance->cameras();
			return Ok(new std::vector<cv::detail::CameraParams>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::detail::CameraParams>*>))
	}
	
	Result<double> cv_Stitcher_workScale_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->workScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_TransverseMercatorWarper_delete(cv::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_TransverseMercatorWarper_create_const_float(const cv::TransverseMercatorWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_WarperCreator_create_const_float(const cv::WarperCreator* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_Detail_AKAZEFeaturesFinder_delete(cv::detail::AKAZEFeaturesFinder* instance) {
		delete instance;
	}
	Result<cv::detail::AKAZEFeaturesFinder*> cv_detail_AKAZEFeaturesFinder_AKAZEFeaturesFinder_int_int_int_float_int_int_int(int descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, int diffusivity) {
		try {
			cv::detail::AKAZEFeaturesFinder* ret = new cv::detail::AKAZEFeaturesFinder(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok<cv::detail::AKAZEFeaturesFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AKAZEFeaturesFinder*>))
	}
	
	void cv_Detail_AffineBasedEstimator_delete(cv::detail::AffineBasedEstimator* instance) {
		delete instance;
	}
	void cv_Detail_AffineBestOf2NearestMatcher_delete(cv::detail::AffineBestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::AffineBestOf2NearestMatcher*> cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(bool full_affine, bool try_use_gpu, float match_conf, int num_matches_thresh1) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher(full_affine, try_use_gpu, match_conf, num_matches_thresh1);
			return Ok<cv::detail::AffineBestOf2NearestMatcher*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AffineBestOf2NearestMatcher*>))
	}
	
	void cv_Detail_AffineWarper_delete(cv::detail::AffineWarper* instance) {
		delete instance;
	}
	Result<cv::detail::AffineWarper*> cv_detail_AffineWarper_AffineWarper_float(float scale) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper(scale);
			return Ok<cv::detail::AffineWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AffineWarper*>))
	}
	
	Result<cv::Point2f> cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *H);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *H, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::AffineWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* H, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *H, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *H);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	void cv_Detail_BestOf2NearestMatcher_delete(cv::detail::BestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::BestOf2NearestMatcher*> cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok<cv::detail::BestOf2NearestMatcher*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BestOf2NearestMatcher*>))
	}
	
	Result_void cv_detail_BestOf2NearestMatcher_collectGarbage(cv::detail::BestOf2NearestMatcher* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BestOf2NearestRangeMatcher_delete(cv::detail::BestOf2NearestRangeMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::BestOf2NearestRangeMatcher*> cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(int range_width, bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok<cv::detail::BestOf2NearestRangeMatcher*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BestOf2NearestRangeMatcher*>))
	}
	
	void cv_Detail_Blender_delete(cv::detail::Blender* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::Blender>*> cv_detail_Blender_createDefault_int_bool(int type, bool try_gpu) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type, try_gpu);
			return Ok(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Blender>*>))
	}
	
	Result_void cv_detail_Blender_prepare_const_vector_Point_R_const_vector_Size_R(cv::detail::Blender* instance, const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			instance->prepare(*corners, *sizes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_Blender_prepare_Rect(cv::detail::Blender* instance, cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::Blender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::Blender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BlocksGainCompensator_delete(cv::detail::BlocksGainCompensator* instance) {
		delete instance;
	}
	Result<cv::detail::BlocksGainCompensator*> cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(int bl_width, int bl_height) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height);
			return Ok<cv::detail::BlocksGainCompensator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BlocksGainCompensator*>))
	}
	
	Result_void cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::BlocksGainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BundleAdjusterAffine_delete(cv::detail::BundleAdjusterAffine* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterAffine*> cv_detail_BundleAdjusterAffine_BundleAdjusterAffine() {
		try {
			cv::detail::BundleAdjusterAffine* ret = new cv::detail::BundleAdjusterAffine();
			return Ok<cv::detail::BundleAdjusterAffine*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterAffine*>))
	}
	
	void cv_Detail_BundleAdjusterAffinePartial_delete(cv::detail::BundleAdjusterAffinePartial* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterAffinePartial*> cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial() {
		try {
			cv::detail::BundleAdjusterAffinePartial* ret = new cv::detail::BundleAdjusterAffinePartial();
			return Ok<cv::detail::BundleAdjusterAffinePartial*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterAffinePartial*>))
	}
	
	Result<const cv::Mat*> cv_detail_BundleAdjusterBase_refinementMask_const(const cv::detail::BundleAdjusterBase* instance) {
		try {
			const cv::Mat ret = instance->refinementMask();
			return Ok(new const cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Mat*>))
	}
	
	Result_void cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(cv::detail::BundleAdjusterBase* instance, const cv::Mat* mask) {
		try {
			instance->setRefinementMask(*mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_detail_BundleAdjusterBase_confThresh_const(const cv::detail::BundleAdjusterBase* instance) {
		try {
			double ret = instance->confThresh();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_BundleAdjusterBase_setConfThresh_double(cv::detail::BundleAdjusterBase* instance, double conf_thresh) {
		try {
			instance->setConfThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::TermCriteria> cv_detail_BundleAdjusterBase_termCriteria(cv::detail::BundleAdjusterBase* instance) {
		try {
			cv::TermCriteria ret = instance->termCriteria();
			return Ok<cv::TermCriteria>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	Result_void cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(cv::detail::BundleAdjusterBase* instance, const cv::TermCriteria* term_criteria) {
		try {
			instance->setTermCriteria(*term_criteria);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BundleAdjusterRay_delete(cv::detail::BundleAdjusterRay* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterRay*> cv_detail_BundleAdjusterRay_BundleAdjusterRay() {
		try {
			cv::detail::BundleAdjusterRay* ret = new cv::detail::BundleAdjusterRay();
			return Ok<cv::detail::BundleAdjusterRay*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterRay*>))
	}
	
	void cv_Detail_BundleAdjusterReproj_delete(cv::detail::BundleAdjusterReproj* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterReproj*> cv_detail_BundleAdjusterReproj_BundleAdjusterReproj() {
		try {
			cv::detail::BundleAdjusterReproj* ret = new cv::detail::BundleAdjusterReproj();
			return Ok<cv::detail::BundleAdjusterReproj*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterReproj*>))
	}
	
	Result<double> cv_detail_CameraParams_getPropFocal_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->focal;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_CameraParams_setPropFocal_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->focal = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_detail_CameraParams_getPropAspect_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->aspect;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_CameraParams_setPropAspect_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->aspect = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_detail_CameraParams_getPropPpx_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->ppx;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_CameraParams_setPropPpx_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->ppx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_detail_CameraParams_getPropPpy_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->ppy;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_CameraParams_setPropPpy_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->ppy = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_getPropR(cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->R;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_detail_CameraParams_setPropR_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
		try {
			instance->R = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_getPropT(cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->t;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_detail_CameraParams_setPropT_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
		try {
			instance->t = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CameraParams_delete(cv::detail::CameraParams* instance) {
		delete instance;
	}
	Result<cv::detail::CameraParams*> cv_detail_CameraParams_CameraParams() {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams();
			return Ok<cv::detail::CameraParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CameraParams*>))
	}
	
	Result<cv::detail::CameraParams*> cv_detail_CameraParams_CameraParams_const_CameraParamsR(const cv::detail::CameraParams* other) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams(*other);
			return Ok<cv::detail::CameraParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CameraParams*>))
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_K_const(const cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->K();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_getPropA_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
		try {
			float ret = instance->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setPropA_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_getPropB_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
		try {
			float ret = instance->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setPropB_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearPortraitProjector_delete(cv::detail::CompressedRectilinearPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearPortraitWarper_delete(cv::detail::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CompressedRectilinearPortraitWarper*> cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale, A, B);
			return Ok<cv::detail::CompressedRectilinearPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CompressedRectilinearPortraitWarper*>))
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_getPropA_const(const cv::detail::CompressedRectilinearProjector* instance) {
		try {
			float ret = instance->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setPropA_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_getPropB_const(const cv::detail::CompressedRectilinearProjector* instance) {
		try {
			float ret = instance->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setPropB_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearProjector_delete(cv::detail::CompressedRectilinearProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearWarper_delete(cv::detail::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CompressedRectilinearWarper*> cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale, A, B);
			return Ok<cv::detail::CompressedRectilinearWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CompressedRectilinearWarper*>))
	}
	
	void cv_Detail_CylindricalPortraitProjector_delete(cv::detail::CylindricalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CylindricalPortraitWarper_delete(cv::detail::CylindricalPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalPortraitWarper*> cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(float scale) {
		try {
			cv::detail::CylindricalPortraitWarper* ret = new cv::detail::CylindricalPortraitWarper(scale);
			return Ok<cv::detail::CylindricalPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalPortraitWarper*>))
	}
	
	void cv_Detail_CylindricalProjector_delete(cv::detail::CylindricalProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CylindricalWarper_delete(cv::detail::CylindricalWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalWarper*> cv_detail_CylindricalWarper_CylindricalWarper_float(float scale) {
		try {
			cv::detail::CylindricalWarper* ret = new cv::detail::CylindricalWarper(scale);
			return Ok<cv::detail::CylindricalWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalWarper*>))
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	void cv_Detail_CylindricalWarperGpu_delete(cv::detail::CylindricalWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalWarperGpu*> cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(float scale) {
		try {
			cv::detail::CylindricalWarperGpu* ret = new cv::detail::CylindricalWarperGpu(scale);
			return Ok<cv::detail::CylindricalWarperGpu*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalWarperGpu*>))
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::CylindricalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<std::vector<int>*> cv_detail_DisjointSets_getPropParent(cv::detail::DisjointSets* instance) {
		try {
			std::vector<int> ret = instance->parent;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_detail_DisjointSets_setPropParent_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
		try {
			instance->parent = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_detail_DisjointSets_getPropSize(cv::detail::DisjointSets* instance) {
		try {
			std::vector<int> ret = instance->size;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_detail_DisjointSets_setPropSize_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
		try {
			instance->size = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_DisjointSets_delete(cv::detail::DisjointSets* instance) {
		delete instance;
	}
	Result<cv::detail::DisjointSets*> cv_detail_DisjointSets_DisjointSets_int(int elem_count) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets(elem_count);
			return Ok<cv::detail::DisjointSets*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DisjointSets*>))
	}
	
	Result_void cv_detail_DisjointSets_createOneElemSets_int(cv::detail::DisjointSets* instance, int elem_count) {
		try {
			instance->createOneElemSets(elem_count);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_DisjointSets_findSetByElem_int(cv::detail::DisjointSets* instance, int elem) {
		try {
			int ret = instance->findSetByElem(elem);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_detail_DisjointSets_mergeSets_int_int(cv::detail::DisjointSets* instance, int set1, int set2) {
		try {
			int ret = instance->mergeSets(set1, set2);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_Detail_DpSeamFinder_delete(cv::detail::DpSeamFinder* instance) {
		delete instance;
	}
	Result<cv::detail::DpSeamFinder*> cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cv::detail::DpSeamFinder::CostFunction costFunc) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(costFunc);
			return Ok<cv::detail::DpSeamFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DpSeamFinder*>))
	}
	
	Result<cv::detail::DpSeamFinder::CostFunction> cv_detail_DpSeamFinder_costFunction_const(const cv::detail::DpSeamFinder* instance) {
		try {
			cv::detail::DpSeamFinder::CostFunction ret = instance->costFunction();
			return Ok<cv::detail::DpSeamFinder::CostFunction>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DpSeamFinder::CostFunction>))
	}
	
	Result_void cv_detail_DpSeamFinder_setCostFunction_CostFunction(cv::detail::DpSeamFinder* instance, cv::detail::DpSeamFinder::CostFunction val) {
		try {
			instance->setCostFunction(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_DpSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::DpSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::detail::ExposureCompensator>*> cv_detail_ExposureCompensator_createDefault_int(int type) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = cv::detail::ExposureCompensator::createDefault(type);
			return Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	Result_void cv_detail_ExposureCompensator_feed_const_vector_Point_R_const_vector_UMat_R_const_vector_UMat_R(cv::detail::ExposureCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<cv::UMat>* masks) {
		try {
			instance->feed(*corners, *images, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::ExposureCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_FeatherBlender_delete(cv::detail::FeatherBlender* instance) {
		delete instance;
	}
	Result<cv::detail::FeatherBlender*> cv_detail_FeatherBlender_FeatherBlender_float(float sharpness) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender(sharpness);
			return Ok<cv::detail::FeatherBlender*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::FeatherBlender*>))
	}
	
	Result<float> cv_detail_FeatherBlender_sharpness_const(const cv::detail::FeatherBlender* instance) {
		try {
			float ret = instance->sharpness();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_FeatherBlender_setSharpness_float(cv::detail::FeatherBlender* instance, float val) {
		try {
			instance->setSharpness(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_FeatherBlender_prepare_Rect(cv::detail::FeatherBlender* instance, cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::FeatherBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::FeatherBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::FeatherBlender* instance, const std::vector<cv::UMat>* masks, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* weight_maps) {
		try {
			cv::Rect ret = instance->createWeightMaps(*masks, *corners, *weight_maps);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_detail_FeaturesFinder_collectGarbage(cv::detail::FeaturesFinder* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_detail_FeaturesMatcher_isThreadSafe_const(const cv::detail::FeaturesMatcher* instance) {
		try {
			bool ret = instance->isThreadSafe();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_detail_FeaturesMatcher_collectGarbage(cv::detail::FeaturesMatcher* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_FisheyeProjector_delete(cv::detail::FisheyeProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_FisheyeWarper_delete(cv::detail::FisheyeWarper* instance) {
		delete instance;
	}
	Result<cv::detail::FisheyeWarper*> cv_detail_FisheyeWarper_FisheyeWarper_float(float scale) {
		try {
			cv::detail::FisheyeWarper* ret = new cv::detail::FisheyeWarper(scale);
			return Ok<cv::detail::FisheyeWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::FisheyeWarper*>))
	}
	
	void cv_Detail_GainCompensator_delete(cv::detail::GainCompensator* instance) {
		delete instance;
	}
	Result_void cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::GainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<double>*> cv_detail_GainCompensator_gains_const(const cv::detail::GainCompensator* instance) {
		try {
			std::vector<double> ret = instance->gains();
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	void cv_Detail_Graph_delete(cv::detail::Graph* instance) {
		delete instance;
	}
	Result<cv::detail::Graph*> cv_detail_Graph_Graph_int(int num_vertices) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph(num_vertices);
			return Ok<cv::detail::Graph*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::Graph*>))
	}
	
	Result_void cv_detail_Graph_create_int(cv::detail::Graph* instance, int num_vertices) {
		try {
			instance->create(num_vertices);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_Graph_numVertices_const(const cv::detail::Graph* instance) {
		try {
			int ret = instance->numVertices();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_Graph_addEdge_int_int_float(cv::detail::Graph* instance, int from, int to, float weight) {
		try {
			instance->addEdge(from, to, weight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_GraphCutSeamFinder_delete(cv::detail::GraphCutSeamFinder* instance) {
		delete instance;
	}
	Result<cv::detail::GraphCutSeamFinder*> cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(cost_type, terminal_cost, bad_region_penalty);
			return Ok<cv::detail::GraphCutSeamFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphCutSeamFinder*>))
	}
	
	Result_void cv_detail_GraphCutSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::GraphCutSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_GraphCutSeamFinderBase_delete(cv::detail::GraphCutSeamFinderBase* instance) {
		delete instance;
	}
	void cv_Detail_GraphCutSeamFinderGpu_delete(cv::detail::GraphCutSeamFinderGpu* instance) {
		delete instance;
	}
	Result<cv::detail::GraphCutSeamFinderGpu*> cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty) {
		try {
			cv::detail::GraphCutSeamFinderGpu* ret = new cv::detail::GraphCutSeamFinderGpu(cost_type, terminal_cost, bad_region_penalty);
			return Ok<cv::detail::GraphCutSeamFinderGpu*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphCutSeamFinderGpu*>))
	}
	
	Result_void cv_detail_GraphCutSeamFinderGpu_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::GraphCutSeamFinderGpu* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_GraphCutSeamFinderGpu_findInPair_size_t_size_t_Rect(cv::detail::GraphCutSeamFinderGpu* instance, size_t first, size_t second, cv::Rect* roi) {
		try {
			instance->findInPair(first, second, *roi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_GraphEdge_getPropFrom_const(const cv::detail::GraphEdge* instance) {
		try {
			int ret = instance->from;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_GraphEdge_setPropFrom_int(cv::detail::GraphEdge* instance, int val) {
		try {
			instance->from = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_GraphEdge_getPropTo_const(const cv::detail::GraphEdge* instance) {
		try {
			int ret = instance->to;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_GraphEdge_setPropTo_int(cv::detail::GraphEdge* instance, int val) {
		try {
			instance->to = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_GraphEdge_getPropWeight_const(const cv::detail::GraphEdge* instance) {
		try {
			float ret = instance->weight;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_GraphEdge_setPropWeight_float(cv::detail::GraphEdge* instance, float val) {
		try {
			instance->weight = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_GraphEdge_delete(cv::detail::GraphEdge* instance) {
		delete instance;
	}
	Result<cv::detail::GraphEdge*> cv_detail_GraphEdge_GraphEdge_int_int_float(int from, int to, float weight) {
		try {
			cv::detail::GraphEdge* ret = new cv::detail::GraphEdge(from, to, weight);
			return Ok<cv::detail::GraphEdge*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphEdge*>))
	}
	
	void cv_Detail_HomographyBasedEstimator_delete(cv::detail::HomographyBasedEstimator* instance) {
		delete instance;
	}
	Result<cv::detail::HomographyBasedEstimator*> cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(bool is_focals_estimated) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator(is_focals_estimated);
			return Ok<cv::detail::HomographyBasedEstimator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::HomographyBasedEstimator*>))
	}
	
	Result<int> cv_detail_ImageFeatures_getPropImg_idx_const(const cv::detail::ImageFeatures* instance) {
		try {
			int ret = instance->img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_ImageFeatures_setPropImg_idx_int(cv::detail::ImageFeatures* instance, int val) {
		try {
			instance->img_idx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_detail_ImageFeatures_getPropImg_size_const(const cv::detail::ImageFeatures* instance) {
		try {
			cv::Size ret = instance->img_size;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_detail_ImageFeatures_setPropImg_size_Size(cv::detail::ImageFeatures* instance, cv::Size* val) {
		try {
			instance->img_size = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::KeyPoint>*> cv_detail_ImageFeatures_getPropKeypoints(cv::detail::ImageFeatures* instance) {
		try {
			std::vector<cv::KeyPoint> ret = instance->keypoints;
			return Ok(new std::vector<cv::KeyPoint>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::KeyPoint>*>))
	}
	
	Result_void cv_detail_ImageFeatures_setPropKeypoints_vector_KeyPoint_(cv::detail::ImageFeatures* instance, std::vector<cv::KeyPoint>* val) {
		try {
			instance->keypoints = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMat*> cv_detail_ImageFeatures_getPropDescriptors(cv::detail::ImageFeatures* instance) {
		try {
			cv::UMat ret = instance->descriptors;
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv_detail_ImageFeatures_setPropDescriptors_UMat(cv::detail::ImageFeatures* instance, cv::UMat* val) {
		try {
			instance->descriptors = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_ImageFeatures_delete(cv::detail::ImageFeatures* instance) {
		delete instance;
	}
	Result<int> cv_detail_MatchesInfo_getPropSrc_img_idx_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->src_img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropSrc_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->src_img_idx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_MatchesInfo_getPropDst_img_idx_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->dst_img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropDst_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->dst_img_idx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::DMatch>*> cv_detail_MatchesInfo_getPropMatches(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<cv::DMatch> ret = instance->matches;
			return Ok(new std::vector<cv::DMatch>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::DMatch>*>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropMatches_vector_DMatch_(cv::detail::MatchesInfo* instance, std::vector<cv::DMatch>* val) {
		try {
			instance->matches = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<unsigned char>*> cv_detail_MatchesInfo_getPropInliers_mask(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<unsigned char> ret = instance->inliers_mask;
			return Ok(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<unsigned char>*>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropInliers_mask_vector_unsigned_char_(cv::detail::MatchesInfo* instance, std::vector<unsigned char>* val) {
		try {
			instance->inliers_mask = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_detail_MatchesInfo_getPropNum_inliers_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->num_inliers;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropNum_inliers_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->num_inliers = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_detail_MatchesInfo_getPropH(cv::detail::MatchesInfo* instance) {
		try {
			cv::Mat ret = instance->H;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropH_Mat(cv::detail::MatchesInfo* instance, cv::Mat* val) {
		try {
			instance->H = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_detail_MatchesInfo_getPropConfidence_const(const cv::detail::MatchesInfo* instance) {
		try {
			double ret = instance->confidence;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_detail_MatchesInfo_setPropConfidence_double(cv::detail::MatchesInfo* instance, double val) {
		try {
			instance->confidence = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_MatchesInfo_delete(cv::detail::MatchesInfo* instance) {
		delete instance;
	}
	Result<cv::detail::MatchesInfo*> cv_detail_MatchesInfo_MatchesInfo() {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo();
			return Ok<cv::detail::MatchesInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MatchesInfo*>))
	}
	
	Result<cv::detail::MatchesInfo*> cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(const cv::detail::MatchesInfo* other) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo(*other);
			return Ok<cv::detail::MatchesInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MatchesInfo*>))
	}
	
	void cv_Detail_MercatorProjector_delete(cv::detail::MercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_MercatorWarper_delete(cv::detail::MercatorWarper* instance) {
		delete instance;
	}
	Result<cv::detail::MercatorWarper*> cv_detail_MercatorWarper_MercatorWarper_float(float scale) {
		try {
			cv::detail::MercatorWarper* ret = new cv::detail::MercatorWarper(scale);
			return Ok<cv::detail::MercatorWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MercatorWarper*>))
	}
	
	void cv_Detail_MultiBandBlender_delete(cv::detail::MultiBandBlender* instance) {
		delete instance;
	}
	Result<cv::detail::MultiBandBlender*> cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(int try_gpu, int num_bands, int weight_type) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender(try_gpu, num_bands, weight_type);
			return Ok<cv::detail::MultiBandBlender*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MultiBandBlender*>))
	}
	
	Result<int> cv_detail_MultiBandBlender_numBands_const(const cv::detail::MultiBandBlender* instance) {
		try {
			int ret = instance->numBands();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_detail_MultiBandBlender_setNumBands_int(cv::detail::MultiBandBlender* instance, int val) {
		try {
			instance->setNumBands(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_MultiBandBlender_prepare_Rect(cv::detail::MultiBandBlender* instance, cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::MultiBandBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::MultiBandBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_NoBundleAdjuster_delete(cv::detail::NoBundleAdjuster* instance) {
		delete instance;
	}
	Result<cv::detail::NoBundleAdjuster*> cv_detail_NoBundleAdjuster_NoBundleAdjuster() {
		try {
			cv::detail::NoBundleAdjuster* ret = new cv::detail::NoBundleAdjuster();
			return Ok<cv::detail::NoBundleAdjuster*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::NoBundleAdjuster*>))
	}
	
	void cv_Detail_NoExposureCompensator_delete(cv::detail::NoExposureCompensator* instance) {
		delete instance;
	}
	Result_void cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::NoExposureCompensator* instance, int unnamed, cv::Point* unnamed_1, const cv::_InputOutputArray* unnamed_2, const cv::_InputArray* unnamed_3) {
		try {
			instance->apply(unnamed, *unnamed_1, *unnamed_2, *unnamed_3);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_NoSeamFinder_delete(cv::detail::NoSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_NoSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::NoSeamFinder* instance, const std::vector<cv::UMat>* unnamed, const std::vector<cv::Point>* unnamed_1, std::vector<cv::UMat>* unnamed_2) {
		try {
			instance->find(*unnamed, *unnamed_1, *unnamed_2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_OrbFeaturesFinder_delete(cv::detail::OrbFeaturesFinder* instance) {
		delete instance;
	}
	Result<cv::detail::OrbFeaturesFinder*> cv_detail_OrbFeaturesFinder_OrbFeaturesFinder_Size_int_float_int(cv::Size* _grid_size, int nfeatures, float scaleFactor, int nlevels) {
		try {
			cv::detail::OrbFeaturesFinder* ret = new cv::detail::OrbFeaturesFinder(*_grid_size, nfeatures, scaleFactor, nlevels);
			return Ok<cv::detail::OrbFeaturesFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::OrbFeaturesFinder*>))
	}
	
	Result_void cv_detail_PairwiseSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::PairwiseSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_getPropA_const(const cv::detail::PaniniPortraitProjector* instance) {
		try {
			float ret = instance->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setPropA_float(cv::detail::PaniniPortraitProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_getPropB_const(const cv::detail::PaniniPortraitProjector* instance) {
		try {
			float ret = instance->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setPropB_float(cv::detail::PaniniPortraitProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniPortraitProjector_delete(cv::detail::PaniniPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniPortraitWarper_delete(cv::detail::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PaniniPortraitWarper*> cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale, A, B);
			return Ok<cv::detail::PaniniPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PaniniPortraitWarper*>))
	}
	
	Result<float> cv_detail_PaniniProjector_getPropA_const(const cv::detail::PaniniProjector* instance) {
		try {
			float ret = instance->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_PaniniProjector_setPropA_float(cv::detail::PaniniProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_detail_PaniniProjector_getPropB_const(const cv::detail::PaniniProjector* instance) {
		try {
			float ret = instance->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_PaniniProjector_setPropB_float(cv::detail::PaniniProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniProjector_delete(cv::detail::PaniniProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniWarper_delete(cv::detail::PaniniWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PaniniWarper*> cv_detail_PaniniWarper_PaniniWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale, A, B);
			return Ok<cv::detail::PaniniWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PaniniWarper*>))
	}
	
	void cv_Detail_PlanePortraitProjector_delete(cv::detail::PlanePortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PlanePortraitWarper_delete(cv::detail::PlanePortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PlanePortraitWarper*> cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(float scale) {
		try {
			cv::detail::PlanePortraitWarper* ret = new cv::detail::PlanePortraitWarper(scale);
			return Ok<cv::detail::PlanePortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlanePortraitWarper*>))
	}
	
	void cv_Detail_PlaneProjector_delete(cv::detail::PlaneProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PlaneWarper_delete(cv::detail::PlaneWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PlaneWarper*> cv_detail_PlaneWarper_PlaneWarper_float(float scale) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper(scale);
			return Ok<cv::detail::PlaneWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlaneWarper*>))
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R, *T);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R, *T);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	void cv_Detail_PlaneWarperGpu_delete(cv::detail::PlaneWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::PlaneWarperGpu*> cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(float scale) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu(scale);
			return Ok<cv::detail::PlaneWarperGpu*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlaneWarperGpu*>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, cv::cuda::GpuMat* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<float> cv_detail_ProjectorBase_getPropScale_const(const cv::detail::ProjectorBase* instance) {
		try {
			float ret = instance->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_ProjectorBase_setPropScale_float(cv::detail::ProjectorBase* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_getPropK(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->k;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float(*)[9]>))
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_getPropRinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->rinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float(*)[9]>))
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_getPropR_kinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->r_kinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float(*)[9]>))
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_getPropK_rinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->k_rinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float(*)[9]>))
	}
	
	Result<float(*)[3]> cv_detail_ProjectorBase_getPropT(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[3] = &instance->t;
			return Ok<float(*)[3]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float(*)[3]>))
	}
	
	void cv_Detail_ProjectorBase_delete(cv::detail::ProjectorBase* instance) {
		delete instance;
	}
	Result_void cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::ProjectorBase* instance, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			instance->setCameraParams(*K, *R, *T);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2f> cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::Size* dst_size, const cv::_OutputArray* dst) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<float> cv_detail_RotationWarper_getScale_const(const cv::detail::RotationWarper* instance) {
		try {
			float ret = instance->getScale();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_detail_RotationWarper_setScale_float(cv::detail::RotationWarper* instance, float unnamed) {
		try {
			instance->setScale(unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_SeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::SeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_SiftFeaturesFinder_delete(cv::detail::SiftFeaturesFinder* instance) {
		delete instance;
	}
	Result<cv::detail::SiftFeaturesFinder*> cv_detail_SiftFeaturesFinder_SiftFeaturesFinder() {
		try {
			cv::detail::SiftFeaturesFinder* ret = new cv::detail::SiftFeaturesFinder();
			return Ok<cv::detail::SiftFeaturesFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SiftFeaturesFinder*>))
	}
	
	void cv_Detail_SphericalPortraitProjector_delete(cv::detail::SphericalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_SphericalPortraitWarper_delete(cv::detail::SphericalPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalPortraitWarper*> cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(float scale) {
		try {
			cv::detail::SphericalPortraitWarper* ret = new cv::detail::SphericalPortraitWarper(scale);
			return Ok<cv::detail::SphericalPortraitWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalPortraitWarper*>))
	}
	
	void cv_Detail_SphericalProjector_delete(cv::detail::SphericalProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_SphericalWarper_delete(cv::detail::SphericalWarper* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalWarper*> cv_detail_SphericalWarper_SphericalWarper_float(float scale) {
		try {
			cv::detail::SphericalWarper* ret = new cv::detail::SphericalWarper(scale);
			return Ok<cv::detail::SphericalWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalWarper*>))
	}
	
	Result<cv::Rect> cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	void cv_Detail_SphericalWarperGpu_delete(cv::detail::SphericalWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalWarperGpu*> cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(float scale) {
		try {
			cv::detail::SphericalWarperGpu* ret = new cv::detail::SphericalWarperGpu(scale);
			return Ok<cv::detail::SphericalWarperGpu*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalWarperGpu*>))
	}
	
	Result<cv::Rect> cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result<cv::Rect> cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Point> cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::SphericalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	void cv_Detail_StereographicProjector_delete(cv::detail::StereographicProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_StereographicWarper_delete(cv::detail::StereographicWarper* instance) {
		delete instance;
	}
	Result<cv::detail::StereographicWarper*> cv_detail_StereographicWarper_StereographicWarper_float(float scale) {
		try {
			cv::detail::StereographicWarper* ret = new cv::detail::StereographicWarper(scale);
			return Ok<cv::detail::StereographicWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::StereographicWarper*>))
	}
	
	void cv_Detail_SurfFeaturesFinder_delete(cv::detail::SurfFeaturesFinder* instance) {
		delete instance;
	}
	Result<cv::detail::SurfFeaturesFinder*> cv_detail_SurfFeaturesFinder_SurfFeaturesFinder_double_int_int_int_int(double hess_thresh, int num_octaves, int num_layers, int num_octaves_descr, int num_layers_descr) {
		try {
			cv::detail::SurfFeaturesFinder* ret = new cv::detail::SurfFeaturesFinder(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr);
			return Ok<cv::detail::SurfFeaturesFinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SurfFeaturesFinder*>))
	}
	
	void cv_Detail_SurfFeaturesFinderGpu_delete(cv::detail::SurfFeaturesFinderGpu* instance) {
		delete instance;
	}
	Result<cv::detail::SurfFeaturesFinderGpu*> cv_detail_SurfFeaturesFinderGpu_SurfFeaturesFinderGpu_double_int_int_int_int(double hess_thresh, int num_octaves, int num_layers, int num_octaves_descr, int num_layers_descr) {
		try {
			cv::detail::SurfFeaturesFinderGpu* ret = new cv::detail::SurfFeaturesFinderGpu(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr);
			return Ok<cv::detail::SurfFeaturesFinderGpu*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SurfFeaturesFinderGpu*>))
	}
	
	Result_void cv_detail_SurfFeaturesFinderGpu_collectGarbage(cv::detail::SurfFeaturesFinderGpu* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_TransverseMercatorProjector_delete(cv::detail::TransverseMercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_TransverseMercatorWarper_delete(cv::detail::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<cv::detail::TransverseMercatorWarper*> cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(float scale) {
		try {
			cv::detail::TransverseMercatorWarper* ret = new cv::detail::TransverseMercatorWarper(scale);
			return Ok<cv::detail::TransverseMercatorWarper*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::TransverseMercatorWarper*>))
	}
	
	void cv_Detail_VoronoiSeamFinder_delete(cv::detail::VoronoiSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_Size_R_const_vector_Point_R_vector_UMat_R(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::Size>* size, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*size, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
