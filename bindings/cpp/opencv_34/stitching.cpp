#include "common.hpp"
#include <opencv2/stitching.hpp>
#include "stitching_types.hpp"

extern "C" {
	Result<void*> cv_createStitcherScans_bool(bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::createStitcherScans(try_use_gpu);
			return Ok<void*>(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createStitcher_bool(bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::createStitcher(try_use_gpu);
			return Ok<void*>(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_createLaplacePyrGpu_const__InputArrayX_int_vector_UMat_X(void* img, int num_levels, void* pyr) {
		try {
			cv::detail::createLaplacePyrGpu(*reinterpret_cast<const cv::_InputArray*>(img), num_levels, *reinterpret_cast<std::vector<cv::UMat>*>(pyr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_createLaplacePyr_const__InputArrayX_int_vector_UMat_X(void* img, int num_levels, void* pyr) {
		try {
			cv::detail::createLaplacePyr(*reinterpret_cast<const cv::_InputArray*>(img), num_levels, *reinterpret_cast<std::vector<cv::UMat>*>(pyr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_createWeightMap_const__InputArrayX_float_const__InputOutputArrayX(void* mask, float sharpness, void* weight) {
		try {
			cv::detail::createWeightMap(*reinterpret_cast<const cv::_InputArray*>(mask), sharpness, *reinterpret_cast<const cv::_InputOutputArray*>(weight));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_X_GraphX_vector_int_X(int num_images, void* pairwise_matches, void* span_tree, void* centers) {
		try {
			cv::detail::findMaxSpanningTree(num_images, *reinterpret_cast<const std::vector<cv::detail::MatchesInfo>*>(pairwise_matches), *reinterpret_cast<cv::detail::Graph*>(span_tree), *reinterpret_cast<std::vector<int>*>(centers));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_leaveBiggestComponent_vector_ImageFeatures_X_vector_MatchesInfo_X_float(void* features, void* pairwise_matches, float conf_threshold) {
		try {
			std::vector<int> ret = cv::detail::leaveBiggestComponent(*reinterpret_cast<std::vector<cv::detail::ImageFeatures>*>(features), *reinterpret_cast<std::vector<cv::detail::MatchesInfo>*>(pairwise_matches), conf_threshold);
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_matchesGraphAsString_vector_String_X_vector_MatchesInfo_X_float(void* pathes, void* pairwise_matches, float conf_threshold) {
		try {
			cv::String ret = cv::detail::matchesGraphAsString(*reinterpret_cast<std::vector<cv::String>*>(pathes), *reinterpret_cast<std::vector<cv::detail::MatchesInfo>*>(pairwise_matches), conf_threshold);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_normalizeUsingWeightMap_const__InputArrayX_const__InputOutputArrayX(void* weight, void* src) {
		try {
			cv::detail::normalizeUsingWeightMap(*reinterpret_cast<const cv::_InputArray*>(weight), *reinterpret_cast<const cv::_InputOutputArray*>(src));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_detail_overlapRoi_Point_Point_Size_Size_RectX(const cv::Point* tl1, const cv::Point* tl2, const cv::Size* sz1, const cv::Size* sz2, cv::Rect* roi) {
		try {
			bool ret = cv::detail::overlapRoi(*tl1, *tl2, *sz1, *sz2, *roi);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_X(void* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyrGpu(*reinterpret_cast<std::vector<cv::UMat>*>(pyr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyr_vector_UMat_X(void* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyr(*reinterpret_cast<std::vector<cv::UMat>*>(pyr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_resultRoiIntersection_const_vector_Point_X_const_vector_Size_X(void* corners, void* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoiIntersection(*reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<const std::vector<cv::Size>*>(sizes));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_X_const_vector_Size_X(void* corners, void* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<const std::vector<cv::Size>*>(sizes));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_X_const_vector_UMat_X(void* corners, void* images) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<const std::vector<cv::UMat>*>(images));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_resultTl_const_vector_Point_X(void* corners) {
		try {
			cv::Point ret = cv::detail::resultTl(*reinterpret_cast<const std::vector<cv::Point>*>(corners));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_detail_selectRandomSubset_int_int_vector_int_X(int count, int size, void* subset) {
		try {
			cv::detail::selectRandomSubset(count, size, *reinterpret_cast<std::vector<int>*>(subset));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_stitchingLogLevel() {
		try {
			int ret = cv::detail::stitchingLogLevel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_waveCorrect_vector_Mat_X_WaveCorrectKind(void* rmats, cv::detail::WaveCorrectKind kind) {
		try {
			cv::detail::waveCorrect(*reinterpret_cast<std::vector<cv::Mat>*>(rmats), kind);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_AffineWarper_delete(cv::AffineWarper* instance) {
		delete instance;
	}
	Result<void*> cv_AffineWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::AffineWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CompressedRectilinearPortraitWarper_delete(cv::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper(A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_CompressedRectilinearPortraitWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::CompressedRectilinearPortraitWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CompressedRectilinearWarper_delete(cv::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<void*> cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper(A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_CompressedRectilinearWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::CompressedRectilinearWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CylindricalWarper_delete(cv::CylindricalWarper* instance) {
		delete instance;
	}
	Result<void*> cv_CylindricalWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::CylindricalWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_FisheyeWarper_delete(cv::FisheyeWarper* instance) {
		delete instance;
	}
	Result<void*> cv_FisheyeWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::FisheyeWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_MercatorWarper_delete(cv::MercatorWarper* instance) {
		delete instance;
	}
	Result<void*> cv_MercatorWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::MercatorWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PaniniPortraitWarper_delete(cv::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(float A, float B) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper(A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_PaniniPortraitWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::PaniniPortraitWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PaniniWarper_delete(cv::PaniniWarper* instance) {
		delete instance;
	}
	Result<void*> cv_PaniniWarper_PaniniWarper_float_float(float A, float B) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper(A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_PaniniWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::PaniniWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PlaneWarper_delete(cv::PlaneWarper* instance) {
		delete instance;
	}
	Result<void*> cv_PlaneWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::PlaneWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SphericalWarper_delete(cv::SphericalWarper* instance) {
		delete instance;
	}
	Result<void*> cv_SphericalWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::SphericalWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_StereographicWarper_delete(cv::StereographicWarper* instance) {
		delete instance;
	}
	Result<void*> cv_StereographicWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::StereographicWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Stitcher_delete(cv::Stitcher* instance) {
		delete instance;
	}
	Result<void*> cv_Stitcher_createDefault_bool(bool try_use_gpu) {
		try {
			cv::Stitcher ret = cv::Stitcher::createDefault(try_use_gpu);
			return Ok<void*>(new cv::Stitcher(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_create_Mode_bool(cv::Stitcher::Mode mode, bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create(mode, try_use_gpu);
			return Ok<void*>(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_Stitcher_registrationResol_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::Stitcher*>(instance)->registrationResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setRegistrationResol_double(void* instance, double resol_mpx) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setRegistrationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_seamEstimationResol_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::Stitcher*>(instance)->seamEstimationResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setSeamEstimationResol_double(void* instance, double resol_mpx) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setSeamEstimationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_compositingResol_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::Stitcher*>(instance)->compositingResol();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setCompositingResol_double(void* instance, double resol_mpx) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setCompositingResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_panoConfidenceThresh_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::Stitcher*>(instance)->panoConfidenceThresh();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setPanoConfidenceThresh_double(void* instance, double conf_thresh) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setPanoConfidenceThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_Stitcher_waveCorrection_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Stitcher*>(instance)->waveCorrection();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_Stitcher_setWaveCorrection_bool(void* instance, bool flag) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setWaveCorrection(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::detail::WaveCorrectKind> cv_Stitcher_waveCorrectKind_const(void* instance) {
		try {
			cv::detail::WaveCorrectKind ret = reinterpret_cast<cv::Stitcher*>(instance)->waveCorrectKind();
			return Ok<cv::detail::WaveCorrectKind>(ret);
		} OCVRS_CATCH(Result<cv::detail::WaveCorrectKind>)
	}
	
	Result_void cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(void* instance, cv::detail::WaveCorrectKind kind) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setWaveCorrectKind(kind);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_featuresFinder(void* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesFinder> ret = reinterpret_cast<cv::Stitcher*>(instance)->featuresFinder();
			return Ok<void*>(new cv::Ptr<cv::detail::FeaturesFinder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_featuresFinder_const(void* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesFinder> ret = reinterpret_cast<cv::Stitcher*>(instance)->featuresFinder();
			return Ok<void*>(new cv::Ptr<cv::detail::FeaturesFinder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setFeaturesFinder_Ptr_FeaturesFinder_(void* instance, void* features_finder) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setFeaturesFinder(*reinterpret_cast<cv::Ptr<cv::detail::FeaturesFinder>*>(features_finder));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_featuresMatcher(void* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = reinterpret_cast<cv::Stitcher*>(instance)->featuresMatcher();
			return Ok<void*>(new cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_featuresMatcher_const(void* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = reinterpret_cast<cv::Stitcher*>(instance)->featuresMatcher();
			return Ok<void*>(new cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(void* instance, void* features_matcher) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setFeaturesMatcher(*reinterpret_cast<cv::Ptr<cv::detail::FeaturesMatcher>*>(features_matcher));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_matchingMask_const(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::Stitcher*>(instance)->matchingMask();
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setMatchingMask_const_UMatX(void* instance, void* mask) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setMatchingMask(*reinterpret_cast<const cv::UMat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_bundleAdjuster(void* instance) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = reinterpret_cast<cv::Stitcher*>(instance)->bundleAdjuster();
			return Ok<void*>(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_bundleAdjuster_const(void* instance) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = reinterpret_cast<cv::Stitcher*>(instance)->bundleAdjuster();
			return Ok<void*>(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(void* instance, void* bundle_adjuster) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setBundleAdjuster(*reinterpret_cast<cv::Ptr<cv::detail::BundleAdjusterBase>*>(bundle_adjuster));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_warper(void* instance) {
		try {
			cv::Ptr<cv::WarperCreator> ret = reinterpret_cast<cv::Stitcher*>(instance)->warper();
			return Ok<void*>(new cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_warper_const(void* instance) {
		try {
			cv::Ptr<cv::WarperCreator> ret = reinterpret_cast<cv::Stitcher*>(instance)->warper();
			return Ok<void*>(new cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setWarper_Ptr_WarperCreator_(void* instance, void* creator) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setWarper(*reinterpret_cast<cv::Ptr<cv::WarperCreator>*>(creator));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_exposureCompensator(void* instance) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = reinterpret_cast<cv::Stitcher*>(instance)->exposureCompensator();
			return Ok<void*>(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_exposureCompensator_const(void* instance) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = reinterpret_cast<cv::Stitcher*>(instance)->exposureCompensator();
			return Ok<void*>(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(void* instance, void* exposure_comp) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setExposureCompensator(*reinterpret_cast<cv::Ptr<cv::detail::ExposureCompensator>*>(exposure_comp));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_seamFinder(void* instance) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = reinterpret_cast<cv::Stitcher*>(instance)->seamFinder();
			return Ok<void*>(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_seamFinder_const(void* instance) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = reinterpret_cast<cv::Stitcher*>(instance)->seamFinder();
			return Ok<void*>(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(void* instance, void* seam_finder) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setSeamFinder(*reinterpret_cast<cv::Ptr<cv::detail::SeamFinder>*>(seam_finder));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Stitcher_blender(void* instance) {
		try {
			cv::Ptr<cv::detail::Blender> ret = reinterpret_cast<cv::Stitcher*>(instance)->blender();
			return Ok<void*>(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_blender_const(void* instance) {
		try {
			cv::Ptr<cv::detail::Blender> ret = reinterpret_cast<cv::Stitcher*>(instance)->blender();
			return Ok<void*>(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Stitcher_setBlender_Ptr_Blender_(void* instance, void* b) {
		try {
			reinterpret_cast<cv::Stitcher*>(instance)->setBlender(*reinterpret_cast<cv::Ptr<cv::detail::Blender>*>(b));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_estimateTransform_const__InputArrayX(void* instance, void* images) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->estimateTransform(*reinterpret_cast<const cv::_InputArray*>(images));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_estimateTransform_const__InputArrayX_const_vector_vector_Rect__X(void* instance, void* images, void* rois) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->estimateTransform(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const std::vector<std::vector<cv::Rect>>*>(rois));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__OutputArrayX(void* instance, void* pano) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->composePanorama(*reinterpret_cast<const cv::_OutputArray*>(pano));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__InputArrayX_const__OutputArrayX(void* instance, void* images, void* pano) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->composePanorama(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const cv::_OutputArray*>(pano));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayX_const__OutputArrayX(void* instance, void* images, void* pano) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->stitch(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const cv::_OutputArray*>(pano));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayX_const_vector_vector_Rect__X_const__OutputArrayX(void* instance, void* images, void* rois, void* pano) {
		try {
			cv::Stitcher::Status ret = reinterpret_cast<cv::Stitcher*>(instance)->stitch(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const std::vector<std::vector<cv::Rect>>*>(rois), *reinterpret_cast<const cv::_OutputArray*>(pano));
			return Ok<cv::Stitcher::Status>(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<void*> cv_Stitcher_component_const(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::Stitcher*>(instance)->component();
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Stitcher_cameras_const(void* instance) {
		try {
			std::vector<cv::detail::CameraParams> ret = reinterpret_cast<cv::Stitcher*>(instance)->cameras();
			return Ok<void*>(new std::vector<cv::detail::CameraParams>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_Stitcher_workScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::Stitcher*>(instance)->workScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_TransverseMercatorWarper_delete(cv::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<void*> cv_TransverseMercatorWarper_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::TransverseMercatorWarper*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_WarperCreator_create_const_float(void* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = reinterpret_cast<cv::WarperCreator*>(instance)->create(scale);
			return Ok<void*>(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_AKAZEFeaturesFinder_delete(cv::detail::AKAZEFeaturesFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_AKAZEFeaturesFinder_AKAZEFeaturesFinder_int_int_int_float_int_int_int(int descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, int diffusivity) {
		try {
			cv::detail::AKAZEFeaturesFinder* ret = new cv::detail::AKAZEFeaturesFinder(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_AffineBasedEstimator_delete(cv::detail::AffineBasedEstimator* instance) {
		delete instance;
	}
	void cv_Detail_AffineBestOf2NearestMatcher_delete(cv::detail::AffineBestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<void*> cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(bool full_affine, bool try_use_gpu, float match_conf, int num_matches_thresh1) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher(full_affine, try_use_gpu, match_conf, num_matches_thresh1);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_AffineWarper_delete(cv::detail::AffineWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_AffineWarper_AffineWarper_float(float scale) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Point2f> cv_detail_AffineWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(void* instance, const cv::Point2f* pt, void* K, void* H) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::detail::AffineWarper*>(instance)->warpPoint(*pt, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(H));
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* H, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::AffineWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(H), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_AffineWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* H, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::AffineWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(H), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(void* instance, const cv::Size* src_size, void* K, void* H) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::AffineWarper*>(instance)->warpRoi(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(H));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	void cv_Detail_BestOf2NearestMatcher_delete(cv::detail::BestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_BestOf2NearestMatcher_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::detail::BestOf2NearestMatcher*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BestOf2NearestRangeMatcher_delete(cv::detail::BestOf2NearestRangeMatcher* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(int range_width, bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_Blender_delete(cv::detail::Blender* instance) {
		delete instance;
	}
	Result<void*> cv_detail_Blender_createDefault_int_bool(int type, bool try_gpu) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type, try_gpu);
			return Ok<void*>(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_Blender_prepare_const_vector_Point_X_const_vector_Size_X(void* instance, void* corners, void* sizes) {
		try {
			reinterpret_cast<cv::detail::Blender*>(instance)->prepare(*reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<const std::vector<cv::Size>*>(sizes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_prepare_Rect(void* instance, const cv::Rect* dst_roi) {
		try {
			reinterpret_cast<cv::detail::Blender*>(instance)->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_feed_const__InputArrayX_const__InputArrayX_Point(void* instance, void* img, void* mask, const cv::Point* tl) {
		try {
			reinterpret_cast<cv::detail::Blender*>(instance)->feed(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(mask), *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_blend_const__InputOutputArrayX_const__InputOutputArrayX(void* instance, void* dst, void* dst_mask) {
		try {
			reinterpret_cast<cv::detail::Blender*>(instance)->blend(*reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputOutputArray*>(dst_mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BlocksGainCompensator_delete(cv::detail::BlocksGainCompensator* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(int bl_width, int bl_height) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(void* instance, int index, const cv::Point* corner, void* image, void* mask) {
		try {
			reinterpret_cast<cv::detail::BlocksGainCompensator*>(instance)->apply(index, *corner, *reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BundleAdjusterAffine_delete(cv::detail::BundleAdjusterAffine* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BundleAdjusterAffine_BundleAdjusterAffine() {
		try {
			cv::detail::BundleAdjusterAffine* ret = new cv::detail::BundleAdjusterAffine();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_BundleAdjusterAffinePartial_delete(cv::detail::BundleAdjusterAffinePartial* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial() {
		try {
			cv::detail::BundleAdjusterAffinePartial* ret = new cv::detail::BundleAdjusterAffinePartial();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_BundleAdjusterBase_refinementMask_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->refinementMask();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setRefinementMask_const_MatX(void* instance, void* mask) {
		try {
			reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->setRefinementMask(*reinterpret_cast<const cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_BundleAdjusterBase_confThresh_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->confThresh();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setConfThresh_double(void* instance, double conf_thresh) {
		try {
			reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->setConfThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_BundleAdjusterBase_termCriteria(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->termCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaX(void* instance, void* term_criteria) {
		try {
			reinterpret_cast<cv::detail::BundleAdjusterBase*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(term_criteria));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BundleAdjusterRay_delete(cv::detail::BundleAdjusterRay* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BundleAdjusterRay_BundleAdjusterRay() {
		try {
			cv::detail::BundleAdjusterRay* ret = new cv::detail::BundleAdjusterRay();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_BundleAdjusterReproj_delete(cv::detail::BundleAdjusterReproj* instance) {
		delete instance;
	}
	Result<void*> cv_detail_BundleAdjusterReproj_BundleAdjusterReproj() {
		try {
			cv::detail::BundleAdjusterReproj* ret = new cv::detail::BundleAdjusterReproj();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_detail_CameraParams_focal_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->focal;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setFocal_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->focal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_aspect_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->aspect;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setAspect_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->aspect = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_ppx_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->ppx;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setPpx_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->ppx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_ppy_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->ppy;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setPpy_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->ppy = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_CameraParams_R(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->R;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_CameraParams_setR_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->R = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_CameraParams_t(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->t;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_CameraParams_setT_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::CameraParams*>(instance)->t = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CameraParams_delete(cv::detail::CameraParams* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CameraParams_CameraParams() {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_CameraParams_CameraParams_const_CameraParamsX(void* other) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams(*reinterpret_cast<const cv::detail::CameraParams*>(other));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_CameraParams_K_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::detail::CameraParams*>(instance)->K();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_a_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setA_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_b_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setB_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearPortraitProjector_delete(cv::detail::CompressedRectilinearPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearPortraitProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearPortraitWarper_delete(cv::detail::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale, A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_a_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setA_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_b_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setB_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearProjector_delete(cv::detail::CompressedRectilinearProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::CompressedRectilinearProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearWarper_delete(cv::detail::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale, A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_CylindricalPortraitProjector_delete(cv::detail::CylindricalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::CylindricalPortraitProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::CylindricalPortraitProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CylindricalPortraitWarper_delete(cv::detail::CylindricalPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(float scale) {
		try {
			cv::detail::CylindricalPortraitWarper* ret = new cv::detail::CylindricalPortraitWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_CylindricalProjector_delete(cv::detail::CylindricalProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::CylindricalProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CylindricalProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::CylindricalProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CylindricalWarper_delete(cv::detail::CylindricalWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CylindricalWarper_CylindricalWarper_float(float scale) {
		try {
			cv::detail::CylindricalWarper* ret = new cv::detail::CylindricalWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::CylindricalWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_CylindricalWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::CylindricalWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_CylindricalWarperGpu_delete(cv::detail::CylindricalWarperGpu* instance) {
		delete instance;
	}
	Result<void*> cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(float scale) {
		try {
			cv::detail::CylindricalWarperGpu* ret = new cv::detail::CylindricalWarperGpu(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::CylindricalWarperGpu*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_CylindricalWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::CylindricalWarperGpu*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<void*> cv_detail_DisjointSets_parent(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::detail::DisjointSets*>(instance)->parent;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_DisjointSets_setParent_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::DisjointSets*>(instance)->parent = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_DisjointSets_size(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::detail::DisjointSets*>(instance)->size;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_DisjointSets_setSize_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::DisjointSets*>(instance)->size = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_DisjointSets_delete(cv::detail::DisjointSets* instance) {
		delete instance;
	}
	Result<void*> cv_detail_DisjointSets_DisjointSets_int(int elem_count) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets(elem_count);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_DisjointSets_createOneElemSets_int(void* instance, int elem_count) {
		try {
			reinterpret_cast<cv::detail::DisjointSets*>(instance)->createOneElemSets(elem_count);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_DisjointSets_findSetByElem_int(void* instance, int elem) {
		try {
			int ret = reinterpret_cast<cv::detail::DisjointSets*>(instance)->findSetByElem(elem);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_detail_DisjointSets_mergeSets_int_int(void* instance, int set1, int set2) {
		try {
			int ret = reinterpret_cast<cv::detail::DisjointSets*>(instance)->mergeSets(set1, set2);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_Detail_DpSeamFinder_delete(cv::detail::DpSeamFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cv::detail::DpSeamFinder::CostFunction costFunc) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(costFunc);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::detail::DpSeamFinder::CostFunction> cv_detail_DpSeamFinder_costFunction_const(void* instance) {
		try {
			cv::detail::DpSeamFinder::CostFunction ret = reinterpret_cast<cv::detail::DpSeamFinder*>(instance)->costFunction();
			return Ok<cv::detail::DpSeamFinder::CostFunction>(ret);
		} OCVRS_CATCH(Result<cv::detail::DpSeamFinder::CostFunction>)
	}
	
	Result_void cv_detail_DpSeamFinder_setCostFunction_CostFunction(void* instance, cv::detail::DpSeamFinder::CostFunction val) {
		try {
			reinterpret_cast<cv::detail::DpSeamFinder*>(instance)->setCostFunction(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_DpSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* src, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::DpSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(src), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_ExposureCompensator_createDefault_int(int type) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = cv::detail::ExposureCompensator::createDefault(type);
			return Ok<void*>(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_ExposureCompensator_feed_const_vector_Point_X_const_vector_UMat_X_const_vector_UMat_X(void* instance, void* corners, void* images, void* masks) {
		try {
			reinterpret_cast<cv::detail::ExposureCompensator*>(instance)->feed(*reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<const std::vector<cv::UMat>*>(images), *reinterpret_cast<const std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(void* instance, int index, const cv::Point* corner, void* image, void* mask) {
		try {
			reinterpret_cast<cv::detail::ExposureCompensator*>(instance)->apply(index, *corner, *reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_FeatherBlender_delete(cv::detail::FeatherBlender* instance) {
		delete instance;
	}
	Result<void*> cv_detail_FeatherBlender_FeatherBlender_float(float sharpness) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender(sharpness);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_detail_FeatherBlender_sharpness_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::FeatherBlender*>(instance)->sharpness();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_FeatherBlender_setSharpness_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::FeatherBlender*>(instance)->setSharpness(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_prepare_Rect(void* instance, const cv::Rect* dst_roi) {
		try {
			reinterpret_cast<cv::detail::FeatherBlender*>(instance)->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_feed_const__InputArrayX_const__InputArrayX_Point(void* instance, void* img, void* mask, const cv::Point* tl) {
		try {
			reinterpret_cast<cv::detail::FeatherBlender*>(instance)->feed(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(mask), *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_blend_const__InputOutputArrayX_const__InputOutputArrayX(void* instance, void* dst, void* dst_mask) {
		try {
			reinterpret_cast<cv::detail::FeatherBlender*>(instance)->blend(*reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputOutputArray*>(dst_mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* masks, void* corners, void* weight_maps) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::FeatherBlender*>(instance)->createWeightMaps(*reinterpret_cast<const std::vector<cv::UMat>*>(masks), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(weight_maps));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_detail_FeaturesFinder_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::detail::FeaturesFinder*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_detail_FeaturesMatcher_isThreadSafe_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::detail::FeaturesMatcher*>(instance)->isThreadSafe();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_detail_FeaturesMatcher_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::detail::FeaturesMatcher*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_FisheyeProjector_delete(cv::detail::FisheyeProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_FisheyeProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::FisheyeProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FisheyeProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::FisheyeProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_FisheyeWarper_delete(cv::detail::FisheyeWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_FisheyeWarper_FisheyeWarper_float(float scale) {
		try {
			cv::detail::FisheyeWarper* ret = new cv::detail::FisheyeWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_GainCompensator_delete(cv::detail::GainCompensator* instance) {
		delete instance;
	}
	Result_void cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(void* instance, int index, const cv::Point* corner, void* image, void* mask) {
		try {
			reinterpret_cast<cv::detail::GainCompensator*>(instance)->apply(index, *corner, *reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_GainCompensator_gains_const(void* instance) {
		try {
			std::vector<double> ret = reinterpret_cast<cv::detail::GainCompensator*>(instance)->gains();
			return Ok<void*>(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_Graph_delete(cv::detail::Graph* instance) {
		delete instance;
	}
	Result<void*> cv_detail_Graph_Graph_int(int num_vertices) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph(num_vertices);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_Graph_create_int(void* instance, int num_vertices) {
		try {
			reinterpret_cast<cv::detail::Graph*>(instance)->create(num_vertices);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_Graph_numVertices_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::Graph*>(instance)->numVertices();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_Graph_addEdge_int_int_float(void* instance, int from, int to, float weight) {
		try {
			reinterpret_cast<cv::detail::Graph*>(instance)->addEdge(from, to, weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphCutSeamFinder_delete(cv::detail::GraphCutSeamFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(cost_type, terminal_cost, bad_region_penalty);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_GraphCutSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* src, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::GraphCutSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(src), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphCutSeamFinderBase_delete(cv::detail::GraphCutSeamFinderBase* instance) {
		delete instance;
	}
	Result<int> cv_detail_GraphEdge_from_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::GraphEdge*>(instance)->from;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_GraphEdge_setFrom_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::GraphEdge*>(instance)->from = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_GraphEdge_to_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::GraphEdge*>(instance)->to;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_GraphEdge_setTo_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::GraphEdge*>(instance)->to = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_GraphEdge_weight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::GraphEdge*>(instance)->weight;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_GraphEdge_setWeight_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::GraphEdge*>(instance)->weight = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphEdge_delete(cv::detail::GraphEdge* instance) {
		delete instance;
	}
	Result<void*> cv_detail_GraphEdge_GraphEdge_int_int_float(int from, int to, float weight) {
		try {
			cv::detail::GraphEdge* ret = new cv::detail::GraphEdge(from, to, weight);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_HomographyBasedEstimator_delete(cv::detail::HomographyBasedEstimator* instance) {
		delete instance;
	}
	Result<void*> cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(bool is_focals_estimated) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator(is_focals_estimated);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_detail_ImageFeatures_img_idx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::ImageFeatures*>(instance)->img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_ImageFeatures_setImg_idx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::ImageFeatures*>(instance)->img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_detail_ImageFeatures_img_size_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::detail::ImageFeatures*>(instance)->img_size;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_detail_ImageFeatures_setImg_size_Size(void* instance, const cv::Size* val) {
		try {
			reinterpret_cast<cv::detail::ImageFeatures*>(instance)->img_size = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_ImageFeatures_keypoints(void* instance) {
		try {
			std::vector<cv::KeyPoint> ret = reinterpret_cast<cv::detail::ImageFeatures*>(instance)->keypoints;
			return Ok<void*>(new std::vector<cv::KeyPoint>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_ImageFeatures_setKeypoints_vector_KeyPoint_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::ImageFeatures*>(instance)->keypoints = *reinterpret_cast<std::vector<cv::KeyPoint>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_ImageFeatures_descriptors(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::detail::ImageFeatures*>(instance)->descriptors;
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_ImageFeatures_setDescriptors_UMat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::ImageFeatures*>(instance)->descriptors = *reinterpret_cast<cv::UMat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_ImageFeatures_delete(cv::detail::ImageFeatures* instance) {
		delete instance;
	}
	Result<int> cv_detail_MatchesInfo_src_img_idx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->src_img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setSrc_img_idx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->src_img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_MatchesInfo_dst_img_idx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->dst_img_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setDst_img_idx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->dst_img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_MatchesInfo_matches(void* instance) {
		try {
			std::vector<cv::DMatch> ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->matches;
			return Ok<void*>(new std::vector<cv::DMatch>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_MatchesInfo_setMatches_vector_DMatch_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->matches = *reinterpret_cast<std::vector<cv::DMatch>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_MatchesInfo_inliers_mask(void* instance) {
		try {
			std::vector<unsigned char> ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->inliers_mask;
			return Ok<void*>(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_MatchesInfo_setInliers_mask_vector_unsigned_char_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->inliers_mask = *reinterpret_cast<std::vector<unsigned char>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_MatchesInfo_num_inliers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->num_inliers;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setNum_inliers_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->num_inliers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_MatchesInfo_H(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->H;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_MatchesInfo_setH_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->H = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_MatchesInfo_confidence_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::detail::MatchesInfo*>(instance)->confidence;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_MatchesInfo_setConfidence_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::detail::MatchesInfo*>(instance)->confidence = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_MatchesInfo_delete(cv::detail::MatchesInfo* instance) {
		delete instance;
	}
	Result<void*> cv_detail_MatchesInfo_MatchesInfo() {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoX(void* other) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo(*reinterpret_cast<const cv::detail::MatchesInfo*>(other));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_MercatorProjector_delete(cv::detail::MercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_MercatorProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::MercatorProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MercatorProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::MercatorProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_MercatorWarper_delete(cv::detail::MercatorWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_MercatorWarper_MercatorWarper_float(float scale) {
		try {
			cv::detail::MercatorWarper* ret = new cv::detail::MercatorWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_MultiBandBlender_delete(cv::detail::MultiBandBlender* instance) {
		delete instance;
	}
	Result<void*> cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(int try_gpu, int num_bands, int weight_type) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender(try_gpu, num_bands, weight_type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_detail_MultiBandBlender_numBands_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::MultiBandBlender*>(instance)->numBands();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MultiBandBlender_setNumBands_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::MultiBandBlender*>(instance)->setNumBands(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_prepare_Rect(void* instance, const cv::Rect* dst_roi) {
		try {
			reinterpret_cast<cv::detail::MultiBandBlender*>(instance)->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_feed_const__InputArrayX_const__InputArrayX_Point(void* instance, void* img, void* mask, const cv::Point* tl) {
		try {
			reinterpret_cast<cv::detail::MultiBandBlender*>(instance)->feed(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(mask), *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_blend_const__InputOutputArrayX_const__InputOutputArrayX(void* instance, void* dst, void* dst_mask) {
		try {
			reinterpret_cast<cv::detail::MultiBandBlender*>(instance)->blend(*reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputOutputArray*>(dst_mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_NoBundleAdjuster_delete(cv::detail::NoBundleAdjuster* instance) {
		delete instance;
	}
	Result<void*> cv_detail_NoBundleAdjuster_NoBundleAdjuster() {
		try {
			cv::detail::NoBundleAdjuster* ret = new cv::detail::NoBundleAdjuster();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_NoExposureCompensator_delete(cv::detail::NoExposureCompensator* instance) {
		delete instance;
	}
	Result_void cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(void* instance, int unnamed, const cv::Point* unnamed_1, void* unnamed_2, void* unnamed_3) {
		try {
			reinterpret_cast<cv::detail::NoExposureCompensator*>(instance)->apply(unnamed, *unnamed_1, *reinterpret_cast<const cv::_InputOutputArray*>(unnamed_2), *reinterpret_cast<const cv::_InputArray*>(unnamed_3));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_NoSeamFinder_delete(cv::detail::NoSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_NoSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* unnamed, void* unnamed_1, void* unnamed_2) {
		try {
			reinterpret_cast<cv::detail::NoSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(unnamed), *reinterpret_cast<const std::vector<cv::Point>*>(unnamed_1), *reinterpret_cast<std::vector<cv::UMat>*>(unnamed_2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_OrbFeaturesFinder_delete(cv::detail::OrbFeaturesFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_OrbFeaturesFinder_OrbFeaturesFinder_Size_int_float_int(const cv::Size* _grid_size, int nfeatures, float scaleFactor, int nlevels) {
		try {
			cv::detail::OrbFeaturesFinder* ret = new cv::detail::OrbFeaturesFinder(*_grid_size, nfeatures, scaleFactor, nlevels);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_PairwiseSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* src, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::PairwiseSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(src), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_a_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setA_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_b_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setB_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniPortraitProjector_delete(cv::detail::PaniniPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniPortraitProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::PaniniPortraitProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniPortraitWarper_delete(cv::detail::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale, A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_detail_PaniniProjector_a_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::PaniniProjector*>(instance)->a;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniProjector_setA_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::PaniniProjector*>(instance)->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniProjector_b_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::PaniniProjector*>(instance)->b;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniProjector_setB_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::PaniniProjector*>(instance)->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniProjector_delete(cv::detail::PaniniProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::PaniniProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PaniniProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::PaniniProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniWarper_delete(cv::detail::PaniniWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_PaniniWarper_PaniniWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale, A, B);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_PlanePortraitProjector_delete(cv::detail::PlanePortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlanePortraitProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::PlanePortraitProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PlanePortraitProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::PlanePortraitProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PlanePortraitWarper_delete(cv::detail::PlanePortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(float scale) {
		try {
			cv::detail::PlanePortraitWarper* ret = new cv::detail::PlanePortraitWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_PlaneProjector_delete(cv::detail::PlaneProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlaneProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::PlaneProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PlaneProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::PlaneProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PlaneWarper_delete(cv::detail::PlaneWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_PlaneWarper_PlaneWarper_float(float scale) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(void* instance, const cv::Point2f* pt, void* K, void* R) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warpPoint(*pt, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* instance, const cv::Point2f* pt, void* K, void* R, void* T) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warpPoint(*pt, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T));
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* T, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, void* T, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(void* instance, const cv::Size* src_size, void* K, void* R) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warpRoi(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* T) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarper*>(instance)->warpRoi(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	void cv_Detail_PlaneWarperGpu_delete(cv::detail::PlaneWarperGpu* instance) {
		delete instance;
	}
	Result<void*> cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(float scale) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarperGpu*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* T, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::PlaneWarperGpu*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::PlaneWarperGpu*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, void* T, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::PlaneWarperGpu*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<float> cv_detail_ProjectorBase_scale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::ProjectorBase*>(instance)->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_ProjectorBase_setScale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::detail::ProjectorBase*>(instance)->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_k(void* instance) {
		try {
			float(*ret)[9] = &reinterpret_cast<cv::detail::ProjectorBase*>(instance)->k;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_rinv(void* instance) {
		try {
			float(*ret)[9] = &reinterpret_cast<cv::detail::ProjectorBase*>(instance)->rinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_r_kinv(void* instance) {
		try {
			float(*ret)[9] = &reinterpret_cast<cv::detail::ProjectorBase*>(instance)->r_kinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_k_rinv(void* instance) {
		try {
			float(*ret)[9] = &reinterpret_cast<cv::detail::ProjectorBase*>(instance)->k_rinv;
			return Ok<float(*)[9]>(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[3]> cv_detail_ProjectorBase_t(void* instance) {
		try {
			float(*ret)[3] = &reinterpret_cast<cv::detail::ProjectorBase*>(instance)->t;
			return Ok<float(*)[3]>(ret);
		} OCVRS_CATCH(Result<float(*)[3]>)
	}
	
	void cv_Detail_ProjectorBase_delete(cv::detail::ProjectorBase* instance) {
		delete instance;
	}
	Result_void cv_detail_ProjectorBase_setCameraParams_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* instance, void* K, void* R, void* T) {
		try {
			reinterpret_cast<cv::detail::ProjectorBase*>(instance)->setCameraParams(*reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_detail_RotationWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(void* instance, const cv::Point2f* pt, void* K, void* R) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::detail::RotationWarper*>(instance)->warpPoint(*pt, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::RotationWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_RotationWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::RotationWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_detail_RotationWarper_warpBackward_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_Size_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, const cv::Size* dst_size, void* dst) {
		try {
			reinterpret_cast<cv::detail::RotationWarper*>(instance)->warpBackward(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *dst_size, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(void* instance, const cv::Size* src_size, void* K, void* R) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::RotationWarper*>(instance)->warpRoi(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<float> cv_detail_RotationWarper_getScale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::detail::RotationWarper*>(instance)->getScale();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_RotationWarper_setScale_float(void* instance, float unnamed) {
		try {
			reinterpret_cast<cv::detail::RotationWarper*>(instance)->setScale(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* src, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::SeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(src), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_SiftFeaturesFinder_delete(cv::detail::SiftFeaturesFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SiftFeaturesFinder_SiftFeaturesFinder() {
		try {
			cv::detail::SiftFeaturesFinder* ret = new cv::detail::SiftFeaturesFinder();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_SphericalPortraitProjector_delete(cv::detail::SphericalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_SphericalPortraitProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::SphericalPortraitProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::SphericalPortraitProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_SphericalPortraitWarper_delete(cv::detail::SphericalPortraitWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(float scale) {
		try {
			cv::detail::SphericalPortraitWarper* ret = new cv::detail::SphericalPortraitWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_SphericalProjector_delete(cv::detail::SphericalProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_SphericalProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::SphericalProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SphericalProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::SphericalProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_SphericalWarper_delete(cv::detail::SphericalWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SphericalWarper_SphericalWarper_float(float scale) {
		try {
			cv::detail::SphericalWarper* ret = new cv::detail::SphericalWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::SphericalWarper*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_SphericalWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::SphericalWarper*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_SphericalWarperGpu_delete(cv::detail::SphericalWarperGpu* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(float scale) {
		try {
			cv::detail::SphericalWarperGpu* ret = new cv::detail::SphericalWarperGpu(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, const cv::Size* src_size, void* K, void* R, void* xmap, void* ymap) {
		try {
			cv::Rect ret = reinterpret_cast<cv::detail::SphericalWarperGpu*>(instance)->buildMaps(*src_size, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(xmap), *reinterpret_cast<const cv::_OutputArray*>(ymap));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_SphericalWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(void* instance, void* src, void* K, void* R, int interp_mode, int border_mode, void* dst) {
		try {
			cv::Point ret = reinterpret_cast<cv::detail::SphericalWarperGpu*>(instance)->warp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), interp_mode, border_mode, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_StereographicProjector_delete(cv::detail::StereographicProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_StereographicProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::StereographicProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_StereographicProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::StereographicProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_StereographicWarper_delete(cv::detail::StereographicWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_StereographicWarper_StereographicWarper_float(float scale) {
		try {
			cv::detail::StereographicWarper* ret = new cv::detail::StereographicWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_SurfFeaturesFinder_delete(cv::detail::SurfFeaturesFinder* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SurfFeaturesFinder_SurfFeaturesFinder_double_int_int_int_int(double hess_thresh, int num_octaves, int num_layers, int num_octaves_descr, int num_layers_descr) {
		try {
			cv::detail::SurfFeaturesFinder* ret = new cv::detail::SurfFeaturesFinder(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_SurfFeaturesFinderGpu_delete(cv::detail::SurfFeaturesFinderGpu* instance) {
		delete instance;
	}
	Result<void*> cv_detail_SurfFeaturesFinderGpu_SurfFeaturesFinderGpu_double_int_int_int_int(double hess_thresh, int num_octaves, int num_layers, int num_octaves_descr, int num_layers_descr) {
		try {
			cv::detail::SurfFeaturesFinderGpu* ret = new cv::detail::SurfFeaturesFinderGpu(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_SurfFeaturesFinderGpu_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::detail::SurfFeaturesFinderGpu*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_TransverseMercatorProjector_delete(cv::detail::TransverseMercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_TransverseMercatorProjector_mapForward_float_float_floatX_floatX(void* instance, float x, float y, float* u, float* v) {
		try {
			reinterpret_cast<cv::detail::TransverseMercatorProjector*>(instance)->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatX_floatX(void* instance, float u, float v, float* x, float* y) {
		try {
			reinterpret_cast<cv::detail::TransverseMercatorProjector*>(instance)->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_TransverseMercatorWarper_delete(cv::detail::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<void*> cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(float scale) {
		try {
			cv::detail::TransverseMercatorWarper* ret = new cv::detail::TransverseMercatorWarper(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_VoronoiSeamFinder_delete(cv::detail::VoronoiSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(void* instance, void* src, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::VoronoiSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::UMat>*>(src), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_Size_X_const_vector_Point_X_vector_UMat_X(void* instance, void* size, void* corners, void* masks) {
		try {
			reinterpret_cast<cv::detail::VoronoiSeamFinder*>(instance)->find(*reinterpret_cast<const std::vector<cv::Size>*>(size), *reinterpret_cast<const std::vector<cv::Point>*>(corners), *reinterpret_cast<std::vector<cv::UMat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
