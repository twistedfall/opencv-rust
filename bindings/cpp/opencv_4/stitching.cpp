#include "common.hpp"
#include <opencv2/stitching.hpp>
#include "stitching_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::Stitcher>*> cv_createStitcherScans_bool(bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::createStitcherScans(try_use_gpu);
			return Ok(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Stitcher>*>)
	}
	
	Result<cv::Ptr<cv::Stitcher>*> cv_createStitcher_bool(bool try_use_gpu) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::createStitcher(try_use_gpu);
			return Ok(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Stitcher>*>)
	}
	
	Result_void cv_detail_computeImageFeatures_const_Ptr_Feature2D_X_const__InputArrayX_ImageFeaturesX_const__InputArrayX(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* image, cv::detail::ImageFeatures* features, const cv::_InputArray* mask) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *image, *features, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_computeImageFeatures_const_Ptr_Feature2D_X_const__InputArrayX_vector_ImageFeatures_X_const__InputArrayX(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* images, std::vector<cv::detail::ImageFeatures>* features, const cv::_InputArray* masks) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *images, *features, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_createLaplacePyrGpu_const__InputArrayX_int_vector_UMat_X(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::createLaplacePyrGpu(*img, num_levels, *pyr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_createLaplacePyr_const__InputArrayX_int_vector_UMat_X(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::createLaplacePyr(*img, num_levels, *pyr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_createWeightMap_const__InputArrayX_float_const__InputOutputArrayX(const cv::_InputArray* mask, float sharpness, const cv::_InputOutputArray* weight) {
		try {
			cv::detail::createWeightMap(*mask, sharpness, *weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_X_GraphX_vector_int_X(int num_images, const std::vector<cv::detail::MatchesInfo>* pairwise_matches, cv::detail::Graph* span_tree, std::vector<int>* centers) {
		try {
			cv::detail::findMaxSpanningTree(num_images, *pairwise_matches, *span_tree, *centers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_detail_leaveBiggestComponent_vector_ImageFeatures_X_vector_MatchesInfo_X_float(std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold) {
		try {
			std::vector<int> ret = cv::detail::leaveBiggestComponent(*features, *pairwise_matches, conf_threshold);
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<void*> cv_detail_matchesGraphAsString_vector_String_X_vector_MatchesInfo_X_float(std::vector<cv::String>* pathes, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold) {
		try {
			cv::String ret = cv::detail::matchesGraphAsString(*pathes, *pairwise_matches, conf_threshold);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_normalizeUsingWeightMap_const__InputArrayX_const__InputOutputArrayX(const cv::_InputArray* weight, const cv::_InputOutputArray* src) {
		try {
			cv::detail::normalizeUsingWeightMap(*weight, *src);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_detail_overlapRoi_Point_Point_Size_Size_RectX(const cv::Point* tl1, const cv::Point* tl2, const cv::Size* sz1, const cv::Size* sz2, cv::Rect* roi) {
		try {
			bool ret = cv::detail::overlapRoi(*tl1, *tl2, *sz1, *sz2, *roi);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_X(std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyrGpu(*pyr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_restoreImageFromLaplacePyr_vector_UMat_X(std::vector<cv::UMat>* pyr) {
		try {
			cv::detail::restoreImageFromLaplacePyr(*pyr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_resultRoiIntersection_const_vector_Point_X_const_vector_Size_X(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoiIntersection(*corners, *sizes);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_X_const_vector_Size_X(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *sizes);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_resultRoi_const_vector_Point_X_const_vector_UMat_X(const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *images);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_resultTl_const_vector_Point_X(const std::vector<cv::Point>* corners) {
		try {
			cv::Point ret = cv::detail::resultTl(*corners);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_detail_selectRandomSubset_int_int_vector_int_X(int count, int size, std::vector<int>* subset) {
		try {
			cv::detail::selectRandomSubset(count, size, *subset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_stitchingLogLevel() {
		try {
			int ret = cv::detail::stitchingLogLevel();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_waveCorrect_vector_Mat_X_WaveCorrectKind(std::vector<cv::Mat>* rmats, cv::detail::WaveCorrectKind kind) {
		try {
			cv::detail::waveCorrect(*rmats, kind);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_AffineWarper_delete(cv::AffineWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_AffineWarper_create_const_float(const cv::AffineWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_CompressedRectilinearPortraitWarper_delete(cv::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::CompressedRectilinearPortraitWarper*> cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper(A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::CompressedRectilinearPortraitWarper*>)
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CompressedRectilinearPortraitWarper_create_const_float(const cv::CompressedRectilinearPortraitWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_CompressedRectilinearWarper_delete(cv::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<cv::CompressedRectilinearWarper*> cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(float A, float B) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper(A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::CompressedRectilinearWarper*>)
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CompressedRectilinearWarper_create_const_float(const cv::CompressedRectilinearWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_CylindricalWarper_delete(cv::CylindricalWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_CylindricalWarper_create_const_float(const cv::CylindricalWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_FisheyeWarper_delete(cv::FisheyeWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_FisheyeWarper_create_const_float(const cv::FisheyeWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_MercatorWarper_delete(cv::MercatorWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_MercatorWarper_create_const_float(const cv::MercatorWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_PaniniPortraitWarper_delete(cv::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::PaniniPortraitWarper*> cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(float A, float B) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper(A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::PaniniPortraitWarper*>)
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PaniniPortraitWarper_create_const_float(const cv::PaniniPortraitWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_PaniniWarper_delete(cv::PaniniWarper* instance) {
		delete instance;
	}
	Result<cv::PaniniWarper*> cv_PaniniWarper_PaniniWarper_float_float(float A, float B) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper(A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::PaniniWarper*>)
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PaniniWarper_create_const_float(const cv::PaniniWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_PlaneWarper_delete(cv::PlaneWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_PlaneWarper_create_const_float(const cv::PlaneWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_PyRotationWarper_delete(cv::PyRotationWarper* instance) {
		delete instance;
	}
	Result<cv::PyRotationWarper*> cv_PyRotationWarper_PyRotationWarper_String_float(char* type, float scale) {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper(std::string(type), scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::PyRotationWarper*>)
	}
	
	Result<cv::PyRotationWarper*> cv_PyRotationWarper_PyRotationWarper() {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::PyRotationWarper*>)
	}
	
	Result<cv::Point2f> cv_PyRotationWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(cv::PyRotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_PyRotationWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::PyRotationWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_PyRotationWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_PyRotationWarper_warpBackward_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_Size_const__OutputArrayX(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::Size* dst_size, const cv::_OutputArray* dst) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_PyRotationWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(cv::PyRotationWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<float> cv_PyRotationWarper_getScale_const(const cv::PyRotationWarper* instance) {
		try {
			float ret = instance->getScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_PyRotationWarper_setScale_float(cv::PyRotationWarper* instance, float unnamed) {
		try {
			instance->setScale(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SphericalWarper_delete(cv::SphericalWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_SphericalWarper_create_const_float(const cv::SphericalWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_StereographicWarper_delete(cv::StereographicWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_StereographicWarper_create_const_float(const cv::StereographicWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	Result<double> cv_Stitcher_ORIG_RESOL_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->ORIG_RESOL;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_Stitcher_delete(cv::Stitcher* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::Stitcher>*> cv_Stitcher_create_Mode(cv::Stitcher::Mode mode) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create(mode);
			return Ok(new cv::Ptr<cv::Stitcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Stitcher>*>)
	}
	
	Result<double> cv_Stitcher_registrationResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->registrationResol();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setRegistrationResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setRegistrationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_seamEstimationResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->seamEstimationResol();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setSeamEstimationResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setSeamEstimationResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_compositingResol_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->compositingResol();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setCompositingResol_double(cv::Stitcher* instance, double resol_mpx) {
		try {
			instance->setCompositingResol(resol_mpx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_Stitcher_panoConfidenceThresh_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->panoConfidenceThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_Stitcher_setPanoConfidenceThresh_double(cv::Stitcher* instance, double conf_thresh) {
		try {
			instance->setPanoConfidenceThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_Stitcher_waveCorrection_const(const cv::Stitcher* instance) {
		try {
			bool ret = instance->waveCorrection();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_Stitcher_setWaveCorrection_bool(cv::Stitcher* instance, bool flag) {
		try {
			instance->setWaveCorrection(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::InterpolationFlags> cv_Stitcher_interpolationFlags_const(const cv::Stitcher* instance) {
		try {
			cv::InterpolationFlags ret = instance->interpolationFlags();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::InterpolationFlags>)
	}
	
	Result_void cv_Stitcher_setInterpolationFlags_InterpolationFlags(cv::Stitcher* instance, cv::InterpolationFlags interp_flags) {
		try {
			instance->setInterpolationFlags(interp_flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::detail::WaveCorrectKind> cv_Stitcher_waveCorrectKind_const(const cv::Stitcher* instance) {
		try {
			cv::detail::WaveCorrectKind ret = instance->waveCorrectKind();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::WaveCorrectKind>)
	}
	
	Result_void cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(cv::Stitcher* instance, cv::detail::WaveCorrectKind kind) {
		try {
			instance->setWaveCorrectKind(kind);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::Feature2D>*> cv_Stitcher_featuresFinder(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			return Ok(new cv::Ptr<cv::Feature2D>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Feature2D>*>)
	}
	
	Result<cv::Ptr<cv::Feature2D>*> cv_Stitcher_featuresFinder_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			return Ok(new cv::Ptr<cv::Feature2D>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::Feature2D>*>)
	}
	
	Result_void cv_Stitcher_setFeaturesFinder_Ptr_Feature2D_(cv::Stitcher* instance, cv::Ptr<cv::Feature2D>* features_finder) {
		try {
			instance->setFeaturesFinder(*features_finder);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::FeaturesMatcher>*> cv_Stitcher_featuresMatcher(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			return Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::FeaturesMatcher>*>)
	}
	
	Result<cv::Ptr<cv::detail::FeaturesMatcher>*> cv_Stitcher_featuresMatcher_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			return Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::FeaturesMatcher>*>)
	}
	
	Result_void cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(cv::Stitcher* instance, cv::Ptr<cv::detail::FeaturesMatcher>* features_matcher) {
		try {
			instance->setFeaturesMatcher(*features_matcher);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::UMat*> cv_Stitcher_matchingMask_const(const cv::Stitcher* instance) {
		try {
			cv::UMat ret = instance->matchingMask();
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(Result<cv::UMat*>)
	}
	
	Result_void cv_Stitcher_setMatchingMask_const_UMatX(cv::Stitcher* instance, const cv::UMat* mask) {
		try {
			instance->setMatchingMask(*mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::BundleAdjusterBase>*> cv_Stitcher_bundleAdjuster(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			return Ok(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>)
	}
	
	Result<cv::Ptr<cv::detail::BundleAdjusterBase>*> cv_Stitcher_bundleAdjuster_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			return Ok(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>)
	}
	
	Result_void cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(cv::Stitcher* instance, cv::Ptr<cv::detail::BundleAdjusterBase>* bundle_adjuster) {
		try {
			instance->setBundleAdjuster(*bundle_adjuster);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::Estimator>*> cv_Stitcher_estimator(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			return Ok(new cv::Ptr<cv::detail::Estimator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::Estimator>*>)
	}
	
	Result<cv::Ptr<cv::detail::Estimator>*> cv_Stitcher_estimator_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			return Ok(new cv::Ptr<cv::detail::Estimator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::Estimator>*>)
	}
	
	Result_void cv_Stitcher_setEstimator_Ptr_Estimator_(cv::Stitcher* instance, cv::Ptr<cv::detail::Estimator>* estimator) {
		try {
			instance->setEstimator(*estimator);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::WarperCreator>*> cv_Stitcher_warper(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::WarperCreator> ret = instance->warper();
			return Ok(new cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::WarperCreator>*>)
	}
	
	Result<cv::Ptr<cv::WarperCreator>*> cv_Stitcher_warper_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::WarperCreator> ret = instance->warper();
			return Ok(new cv::Ptr<cv::WarperCreator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::WarperCreator>*>)
	}
	
	Result_void cv_Stitcher_setWarper_Ptr_WarperCreator_(cv::Stitcher* instance, cv::Ptr<cv::WarperCreator>* creator) {
		try {
			instance->setWarper(*creator);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::ExposureCompensator>*> cv_Stitcher_exposureCompensator(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			return Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::ExposureCompensator>*>)
	}
	
	Result<cv::Ptr<cv::detail::ExposureCompensator>*> cv_Stitcher_exposureCompensator_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			return Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::ExposureCompensator>*>)
	}
	
	Result_void cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(cv::Stitcher* instance, cv::Ptr<cv::detail::ExposureCompensator>* exposure_comp) {
		try {
			instance->setExposureCompensator(*exposure_comp);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::SeamFinder>*> cv_Stitcher_seamFinder(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			return Ok(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::SeamFinder>*>)
	}
	
	Result<cv::Ptr<cv::detail::SeamFinder>*> cv_Stitcher_seamFinder_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			return Ok(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::SeamFinder>*>)
	}
	
	Result_void cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(cv::Stitcher* instance, cv::Ptr<cv::detail::SeamFinder>* seam_finder) {
		try {
			instance->setSeamFinder(*seam_finder);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::Blender>*> cv_Stitcher_blender(cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::Blender> ret = instance->blender();
			return Ok(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::Blender>*>)
	}
	
	Result<cv::Ptr<cv::detail::Blender>*> cv_Stitcher_blender_const(const cv::Stitcher* instance) {
		try {
			cv::Ptr<cv::detail::Blender> ret = instance->blender();
			return Ok(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::Blender>*>)
	}
	
	Result_void cv_Stitcher_setBlender_Ptr_Blender_(cv::Stitcher* instance, cv::Ptr<cv::detail::Blender>* b) {
		try {
			instance->setBlender(*b);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_estimateTransform_const__InputArrayX_const__InputArrayX(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images, *masks);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__OutputArrayX(cv::Stitcher* instance, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*pano);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_composePanorama_const__InputArrayX_const__OutputArrayX(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*images, *pano);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayX_const__OutputArrayX(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *pano);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<cv::Stitcher::Status> cv_Stitcher_stitch_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks, const cv::_OutputArray* pano) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *masks, *pano);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Stitcher::Status>)
	}
	
	Result<std::vector<int>*> cv_Stitcher_component_const(const cv::Stitcher* instance) {
		try {
			std::vector<int> ret = instance->component();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<std::vector<cv::detail::CameraParams>*> cv_Stitcher_cameras_const(const cv::Stitcher* instance) {
		try {
			std::vector<cv::detail::CameraParams> ret = instance->cameras();
			return Ok(new std::vector<cv::detail::CameraParams>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::detail::CameraParams>*>)
	}
	
	Result<double> cv_Stitcher_workScale_const(const cv::Stitcher* instance) {
		try {
			double ret = instance->workScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::UMat*> cv_Stitcher_resultMask_const(const cv::Stitcher* instance) {
		try {
			cv::UMat ret = instance->resultMask();
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(Result<cv::UMat*>)
	}
	
	void cv_TransverseMercatorWarper_delete(cv::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_TransverseMercatorWarper_create_const_float(const cv::TransverseMercatorWarper* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	Result<cv::Ptr<cv::detail::RotationWarper>*> cv_WarperCreator_create_const_float(const cv::WarperCreator* instance, float scale) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			return Ok(new cv::Ptr<cv::detail::RotationWarper>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::RotationWarper>*>)
	}
	
	void cv_Detail_AffineBasedEstimator_delete(cv::detail::AffineBasedEstimator* instance) {
		delete instance;
	}
	Result<cv::detail::AffineBasedEstimator*> cv_detail_AffineBasedEstimator_AffineBasedEstimator() {
		try {
			cv::detail::AffineBasedEstimator* ret = new cv::detail::AffineBasedEstimator();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::AffineBasedEstimator*>)
	}
	
	void cv_Detail_AffineBestOf2NearestMatcher_delete(cv::detail::AffineBestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::AffineBestOf2NearestMatcher*> cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(bool full_affine, bool try_use_gpu, float match_conf, int num_matches_thresh1) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher(full_affine, try_use_gpu, match_conf, num_matches_thresh1);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::AffineBestOf2NearestMatcher*>)
	}
	
	void cv_Detail_AffineWarper_delete(cv::detail::AffineWarper* instance) {
		delete instance;
	}
	Result<cv::detail::AffineWarper*> cv_detail_AffineWarper_AffineWarper_float(float scale) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::AffineWarper*>)
	}
	
	Result<cv::Point2f> cv_detail_AffineWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *H);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::AffineWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *H, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_AffineWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::AffineWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* H, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *H, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Rect> cv_detail_AffineWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(cv::detail::AffineWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *H);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	void cv_Detail_BestOf2NearestMatcher_delete(cv::detail::BestOf2NearestMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::BestOf2NearestMatcher*> cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BestOf2NearestMatcher*>)
	}
	
	Result_void cv_detail_BestOf2NearestMatcher_collectGarbage(cv::detail::BestOf2NearestMatcher* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*> cv_detail_BestOf2NearestMatcher_create_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::Ptr<cv::detail::BestOf2NearestMatcher> ret = cv::detail::BestOf2NearestMatcher::create(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok(new cv::Ptr<cv::detail::BestOf2NearestMatcher>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>)
	}
	
	void cv_Detail_BestOf2NearestRangeMatcher_delete(cv::detail::BestOf2NearestRangeMatcher* instance) {
		delete instance;
	}
	Result<cv::detail::BestOf2NearestRangeMatcher*> cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(int range_width, bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BestOf2NearestRangeMatcher*>)
	}
	
	void cv_Detail_Blender_delete(cv::detail::Blender* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::detail::Blender>*> cv_detail_Blender_createDefault_int_bool(int type, bool try_gpu) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type, try_gpu);
			return Ok(new cv::Ptr<cv::detail::Blender>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::Blender>*>)
	}
	
	Result_void cv_detail_Blender_prepare_const_vector_Point_X_const_vector_Size_X(cv::detail::Blender* instance, const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes) {
		try {
			instance->prepare(*corners, *sizes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_prepare_Rect(cv::detail::Blender* instance, const cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_feed_const__InputArrayX_const__InputArrayX_Point(cv::detail::Blender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, const cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_Blender_blend_const__InputOutputArrayX_const__InputOutputArrayX(cv::detail::Blender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BlocksChannelsCompensator_delete(cv::detail::BlocksChannelsCompensator* instance) {
		delete instance;
	}
	Result<cv::detail::BlocksChannelsCompensator*> cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds) {
		try {
			cv::detail::BlocksChannelsCompensator* ret = new cv::detail::BlocksChannelsCompensator(bl_width, bl_height, nr_feeds);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BlocksChannelsCompensator*>)
	}
	
	Result_void cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::BlocksCompensator* instance, int index, const cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksCompensator_getMatGains_vector_Mat_X(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->getMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksCompensator_setMatGains_vector_Mat_X(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->setMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksCompensator_setNrFeeds_int(cv::detail::BlocksCompensator* instance, int nr_feeds) {
		try {
			instance->setNrFeeds(nr_feeds);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_BlocksCompensator_getNrFeeds(cv::detail::BlocksCompensator* instance) {
		try {
			int ret = instance->getNrFeeds();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_BlocksCompensator_setBlockSize_int_int(cv::detail::BlocksCompensator* instance, int width, int height) {
		try {
			instance->setBlockSize(width, height);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksCompensator_setBlockSize_Size(cv::detail::BlocksCompensator* instance, const cv::Size* size) {
		try {
			instance->setBlockSize(*size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_detail_BlocksCompensator_getBlockSize_const(const cv::detail::BlocksCompensator* instance) {
		try {
			cv::Size ret = instance->getBlockSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(cv::detail::BlocksCompensator* instance, int nr_iterations) {
		try {
			instance->setNrGainsFilteringIterations(nr_iterations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(const cv::detail::BlocksCompensator* instance) {
		try {
			int ret = instance->getNrGainsFilteringIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_Detail_BlocksGainCompensator_delete(cv::detail::BlocksGainCompensator* instance) {
		delete instance;
	}
	Result<cv::detail::BlocksGainCompensator*> cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(int bl_width, int bl_height) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BlocksGainCompensator*>)
	}
	
	Result<cv::detail::BlocksGainCompensator*> cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height, nr_feeds);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BlocksGainCompensator*>)
	}
	
	Result_void cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::BlocksGainCompensator* instance, int index, const cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksGainCompensator_getMatGains_vector_Mat_X(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->getMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_BlocksGainCompensator_setMatGains_vector_Mat_X(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->setMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BundleAdjusterAffine_delete(cv::detail::BundleAdjusterAffine* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterAffine*> cv_detail_BundleAdjusterAffine_BundleAdjusterAffine() {
		try {
			cv::detail::BundleAdjusterAffine* ret = new cv::detail::BundleAdjusterAffine();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BundleAdjusterAffine*>)
	}
	
	void cv_Detail_BundleAdjusterAffinePartial_delete(cv::detail::BundleAdjusterAffinePartial* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterAffinePartial*> cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial() {
		try {
			cv::detail::BundleAdjusterAffinePartial* ret = new cv::detail::BundleAdjusterAffinePartial();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BundleAdjusterAffinePartial*>)
	}
	
	Result<cv::Mat*> cv_detail_BundleAdjusterBase_refinementMask_const(const cv::detail::BundleAdjusterBase* instance) {
		try {
			cv::Mat ret = instance->refinementMask();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setRefinementMask_const_MatX(cv::detail::BundleAdjusterBase* instance, const cv::Mat* mask) {
		try {
			instance->setRefinementMask(*mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_BundleAdjusterBase_confThresh_const(const cv::detail::BundleAdjusterBase* instance) {
		try {
			double ret = instance->confThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setConfThresh_double(cv::detail::BundleAdjusterBase* instance, double conf_thresh) {
		try {
			instance->setConfThresh(conf_thresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_detail_BundleAdjusterBase_termCriteria(cv::detail::BundleAdjusterBase* instance) {
		try {
			cv::TermCriteria ret = instance->termCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaX(cv::detail::BundleAdjusterBase* instance, const cv::TermCriteria* term_criteria) {
		try {
			instance->setTermCriteria(*term_criteria);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_BundleAdjusterRay_delete(cv::detail::BundleAdjusterRay* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterRay*> cv_detail_BundleAdjusterRay_BundleAdjusterRay() {
		try {
			cv::detail::BundleAdjusterRay* ret = new cv::detail::BundleAdjusterRay();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BundleAdjusterRay*>)
	}
	
	void cv_Detail_BundleAdjusterReproj_delete(cv::detail::BundleAdjusterReproj* instance) {
		delete instance;
	}
	Result<cv::detail::BundleAdjusterReproj*> cv_detail_BundleAdjusterReproj_BundleAdjusterReproj() {
		try {
			cv::detail::BundleAdjusterReproj* ret = new cv::detail::BundleAdjusterReproj();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::BundleAdjusterReproj*>)
	}
	
	Result<double> cv_detail_CameraParams_focal_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->focal;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setFocal_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->focal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_aspect_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->aspect;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setAspect_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->aspect = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_ppx_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->ppx;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setPpx_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->ppx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_CameraParams_ppy_const(const cv::detail::CameraParams* instance) {
		try {
			double ret = instance->ppy;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_CameraParams_setPpy_double(cv::detail::CameraParams* instance, double val) {
		try {
			instance->ppy = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_R(cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->R;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_detail_CameraParams_setR_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
		try {
			instance->R = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_t(cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->t;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_detail_CameraParams_setT_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
		try {
			instance->t = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CameraParams_delete(cv::detail::CameraParams* instance) {
		delete instance;
	}
	Result<cv::detail::CameraParams*> cv_detail_CameraParams_CameraParams() {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CameraParams*>)
	}
	
	Result<cv::detail::CameraParams*> cv_detail_CameraParams_CameraParams_const_CameraParamsX(const cv::detail::CameraParams* other) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams(*other);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CameraParams*>)
	}
	
	Result<cv::Mat*> cv_detail_CameraParams_K_const(const cv::detail::CameraParams* instance) {
		try {
			cv::Mat ret = instance->K();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	void cv_Detail_ChannelsCompensator_delete(cv::detail::ChannelsCompensator* instance) {
		delete instance;
	}
	Result<cv::detail::ChannelsCompensator*> cv_detail_ChannelsCompensator_ChannelsCompensator_int(int nr_feeds) {
		try {
			cv::detail::ChannelsCompensator* ret = new cv::detail::ChannelsCompensator(nr_feeds);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::ChannelsCompensator*>)
	}
	
	Result_void cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::ChannelsCompensator* instance, int index, const cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ChannelsCompensator_getMatGains_vector_Mat_X(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->getMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ChannelsCompensator_setMatGains_vector_Mat_X(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->setMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ChannelsCompensator_setNrFeeds_int(cv::detail::ChannelsCompensator* instance, int nr_feeds) {
		try {
			instance->setNrFeeds(nr_feeds);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_ChannelsCompensator_getNrFeeds(cv::detail::ChannelsCompensator* instance) {
		try {
			int ret = instance->getNrFeeds();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<std::vector<cv::Scalar>*> cv_detail_ChannelsCompensator_gains_const(const cv::detail::ChannelsCompensator* instance) {
		try {
			std::vector<cv::Scalar> ret = instance->gains();
			return Ok(new std::vector<cv::Scalar>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Scalar>*>)
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_a_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
		try {
			float ret = instance->a;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setA_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_CompressedRectilinearPortraitProjector_b_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
		try {
			float ret = instance->b;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_setB_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearPortraitProjector_delete(cv::detail::CompressedRectilinearPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatX_floatX(cv::detail::CompressedRectilinearPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatX_floatX(cv::detail::CompressedRectilinearPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearPortraitWarper_delete(cv::detail::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CompressedRectilinearPortraitWarper*> cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale, A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CompressedRectilinearPortraitWarper*>)
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_a_const(const cv::detail::CompressedRectilinearProjector* instance) {
		try {
			float ret = instance->a;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setA_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_CompressedRectilinearProjector_b_const(const cv::detail::CompressedRectilinearProjector* instance) {
		try {
			float ret = instance->b;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_setB_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearProjector_delete(cv::detail::CompressedRectilinearProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatX_floatX(cv::detail::CompressedRectilinearProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatX_floatX(cv::detail::CompressedRectilinearProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CompressedRectilinearWarper_delete(cv::detail::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CompressedRectilinearWarper*> cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale, A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CompressedRectilinearWarper*>)
	}
	
	void cv_Detail_CylindricalPortraitProjector_delete(cv::detail::CylindricalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatX_floatX(cv::detail::CylindricalPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatX_floatX(cv::detail::CylindricalPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CylindricalPortraitWarper_delete(cv::detail::CylindricalPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalPortraitWarper*> cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(float scale) {
		try {
			cv::detail::CylindricalPortraitWarper* ret = new cv::detail::CylindricalPortraitWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CylindricalPortraitWarper*>)
	}
	
	void cv_Detail_CylindricalProjector_delete(cv::detail::CylindricalProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_CylindricalProjector_mapForward_float_float_floatX_floatX(cv::detail::CylindricalProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_CylindricalProjector_mapBackward_float_float_floatX_floatX(cv::detail::CylindricalProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_CylindricalWarper_delete(cv::detail::CylindricalWarper* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalWarper*> cv_detail_CylindricalWarper_CylindricalWarper_float(float scale) {
		try {
			cv::detail::CylindricalWarper* ret = new cv::detail::CylindricalWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CylindricalWarper*>)
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::CylindricalWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_CylindricalWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::CylindricalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_CylindricalWarperGpu_delete(cv::detail::CylindricalWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::CylindricalWarperGpu*> cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(float scale) {
		try {
			cv::detail::CylindricalWarperGpu* ret = new cv::detail::CylindricalWarperGpu(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::CylindricalWarperGpu*>)
	}
	
	Result<cv::Rect> cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::CylindricalWarperGpu* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_CylindricalWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::CylindricalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<std::vector<int>*> cv_detail_DisjointSets_parent(cv::detail::DisjointSets* instance) {
		try {
			std::vector<int> ret = instance->parent;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result_void cv_detail_DisjointSets_setParent_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
		try {
			instance->parent = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_detail_DisjointSets_size(cv::detail::DisjointSets* instance) {
		try {
			std::vector<int> ret = instance->size;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result_void cv_detail_DisjointSets_setSize_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
		try {
			instance->size = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_DisjointSets_delete(cv::detail::DisjointSets* instance) {
		delete instance;
	}
	Result<cv::detail::DisjointSets*> cv_detail_DisjointSets_DisjointSets_int(int elem_count) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets(elem_count);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::DisjointSets*>)
	}
	
	Result_void cv_detail_DisjointSets_createOneElemSets_int(cv::detail::DisjointSets* instance, int elem_count) {
		try {
			instance->createOneElemSets(elem_count);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_DisjointSets_findSetByElem_int(cv::detail::DisjointSets* instance, int elem) {
		try {
			int ret = instance->findSetByElem(elem);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_detail_DisjointSets_mergeSets_int_int(cv::detail::DisjointSets* instance, int set1, int set2) {
		try {
			int ret = instance->mergeSets(set1, set2);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_Detail_DpSeamFinder_delete(cv::detail::DpSeamFinder* instance) {
		delete instance;
	}
	Result<cv::detail::DpSeamFinder*> cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cv::detail::DpSeamFinder::CostFunction costFunc) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(costFunc);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::DpSeamFinder*>)
	}
	
	Result<cv::detail::DpSeamFinder*> cv_detail_DpSeamFinder_DpSeamFinder_String(char* costFunc) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(std::string(costFunc));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::DpSeamFinder*>)
	}
	
	Result<cv::detail::DpSeamFinder::CostFunction> cv_detail_DpSeamFinder_costFunction_const(const cv::detail::DpSeamFinder* instance) {
		try {
			cv::detail::DpSeamFinder::CostFunction ret = instance->costFunction();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::DpSeamFinder::CostFunction>)
	}
	
	Result_void cv_detail_DpSeamFinder_setCostFunction_CostFunction(cv::detail::DpSeamFinder* instance, cv::detail::DpSeamFinder::CostFunction val) {
		try {
			instance->setCostFunction(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_DpSeamFinder_setCostFunction_String(cv::detail::DpSeamFinder* instance, char* val) {
		try {
			instance->setCostFunction(std::string(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_DpSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::DpSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::ExposureCompensator>*> cv_detail_ExposureCompensator_createDefault_int(int type) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = cv::detail::ExposureCompensator::createDefault(type);
			return Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::ExposureCompensator>*>)
	}
	
	Result_void cv_detail_ExposureCompensator_feed_const_vector_Point_X_const_vector_UMat_X_const_vector_UMat_X(cv::detail::ExposureCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<cv::UMat>* masks) {
		try {
			instance->feed(*corners, *images, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::ExposureCompensator* instance, int index, const cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ExposureCompensator_getMatGains_vector_Mat_X(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed) {
		try {
			instance->getMatGains(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ExposureCompensator_setMatGains_vector_Mat_X(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed) {
		try {
			instance->setMatGains(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_ExposureCompensator_setUpdateGain_bool(cv::detail::ExposureCompensator* instance, bool b) {
		try {
			instance->setUpdateGain(b);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_detail_ExposureCompensator_getUpdateGain(cv::detail::ExposureCompensator* instance) {
		try {
			bool ret = instance->getUpdateGain();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_Detail_FeatherBlender_delete(cv::detail::FeatherBlender* instance) {
		delete instance;
	}
	Result<cv::detail::FeatherBlender*> cv_detail_FeatherBlender_FeatherBlender_float(float sharpness) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender(sharpness);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::FeatherBlender*>)
	}
	
	Result<float> cv_detail_FeatherBlender_sharpness_const(const cv::detail::FeatherBlender* instance) {
		try {
			float ret = instance->sharpness();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_FeatherBlender_setSharpness_float(cv::detail::FeatherBlender* instance, float val) {
		try {
			instance->setSharpness(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_prepare_Rect(cv::detail::FeatherBlender* instance, const cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_feed_const__InputArrayX_const__InputArrayX_Point(cv::detail::FeatherBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, const cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FeatherBlender_blend_const__InputOutputArrayX_const__InputOutputArrayX(cv::detail::FeatherBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::FeatherBlender* instance, const std::vector<cv::UMat>* masks, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* weight_maps) {
		try {
			cv::Rect ret = instance->createWeightMaps(*masks, *corners, *weight_maps);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<bool> cv_detail_FeaturesMatcher_isThreadSafe_const(const cv::detail::FeaturesMatcher* instance) {
		try {
			bool ret = instance->isThreadSafe();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_detail_FeaturesMatcher_collectGarbage(cv::detail::FeaturesMatcher* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_FisheyeProjector_delete(cv::detail::FisheyeProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_FisheyeProjector_mapForward_float_float_floatX_floatX(cv::detail::FisheyeProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_FisheyeProjector_mapBackward_float_float_floatX_floatX(cv::detail::FisheyeProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_FisheyeWarper_delete(cv::detail::FisheyeWarper* instance) {
		delete instance;
	}
	Result<cv::detail::FisheyeWarper*> cv_detail_FisheyeWarper_FisheyeWarper_float(float scale) {
		try {
			cv::detail::FisheyeWarper* ret = new cv::detail::FisheyeWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::FisheyeWarper*>)
	}
	
	void cv_Detail_GainCompensator_delete(cv::detail::GainCompensator* instance) {
		delete instance;
	}
	Result<cv::detail::GainCompensator*> cv_detail_GainCompensator_GainCompensator() {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::GainCompensator*>)
	}
	
	Result<cv::detail::GainCompensator*> cv_detail_GainCompensator_GainCompensator_int(int nr_feeds) {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator(nr_feeds);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::GainCompensator*>)
	}
	
	Result_void cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::GainCompensator* instance, int index, const cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask) {
		try {
			instance->apply(index, *corner, *image, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_GainCompensator_getMatGains_vector_Mat_X(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->getMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_GainCompensator_setMatGains_vector_Mat_X(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->setMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_GainCompensator_setNrFeeds_int(cv::detail::GainCompensator* instance, int nr_feeds) {
		try {
			instance->setNrFeeds(nr_feeds);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_GainCompensator_getNrFeeds(cv::detail::GainCompensator* instance) {
		try {
			int ret = instance->getNrFeeds();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<std::vector<double>*> cv_detail_GainCompensator_gains_const(const cv::detail::GainCompensator* instance) {
		try {
			std::vector<double> ret = instance->gains();
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<std::vector<double>*>)
	}
	
	void cv_Detail_Graph_delete(cv::detail::Graph* instance) {
		delete instance;
	}
	Result<cv::detail::Graph*> cv_detail_Graph_Graph_int(int num_vertices) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph(num_vertices);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::Graph*>)
	}
	
	Result_void cv_detail_Graph_create_int(cv::detail::Graph* instance, int num_vertices) {
		try {
			instance->create(num_vertices);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_Graph_numVertices_const(const cv::detail::Graph* instance) {
		try {
			int ret = instance->numVertices();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_Graph_addEdge_int_int_float(cv::detail::Graph* instance, int from, int to, float weight) {
		try {
			instance->addEdge(from, to, weight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphCutSeamFinder_delete(cv::detail::GraphCutSeamFinder* instance) {
		delete instance;
	}
	Result<cv::detail::GraphCutSeamFinder*> cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(cost_type, terminal_cost, bad_region_penalty);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::GraphCutSeamFinder*>)
	}
	
	Result<cv::detail::GraphCutSeamFinder*> cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(char* cost_type, float terminal_cost, float bad_region_penalty) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(std::string(cost_type), terminal_cost, bad_region_penalty);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::GraphCutSeamFinder*>)
	}
	
	Result_void cv_detail_GraphCutSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::GraphCutSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphCutSeamFinderBase_delete(cv::detail::GraphCutSeamFinderBase* instance) {
		delete instance;
	}
	Result<int> cv_detail_GraphEdge_from_const(const cv::detail::GraphEdge* instance) {
		try {
			int ret = instance->from;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_GraphEdge_setFrom_int(cv::detail::GraphEdge* instance, int val) {
		try {
			instance->from = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_GraphEdge_to_const(const cv::detail::GraphEdge* instance) {
		try {
			int ret = instance->to;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_GraphEdge_setTo_int(cv::detail::GraphEdge* instance, int val) {
		try {
			instance->to = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_GraphEdge_weight_const(const cv::detail::GraphEdge* instance) {
		try {
			float ret = instance->weight;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_GraphEdge_setWeight_float(cv::detail::GraphEdge* instance, float val) {
		try {
			instance->weight = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_GraphEdge_delete(cv::detail::GraphEdge* instance) {
		delete instance;
	}
	Result<cv::detail::GraphEdge*> cv_detail_GraphEdge_GraphEdge_int_int_float(int from, int to, float weight) {
		try {
			cv::detail::GraphEdge* ret = new cv::detail::GraphEdge(from, to, weight);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::GraphEdge*>)
	}
	
	void cv_Detail_HomographyBasedEstimator_delete(cv::detail::HomographyBasedEstimator* instance) {
		delete instance;
	}
	Result<cv::detail::HomographyBasedEstimator*> cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(bool is_focals_estimated) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator(is_focals_estimated);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::HomographyBasedEstimator*>)
	}
	
	Result<int> cv_detail_ImageFeatures_img_idx_const(const cv::detail::ImageFeatures* instance) {
		try {
			int ret = instance->img_idx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_ImageFeatures_setImg_idx_int(cv::detail::ImageFeatures* instance, int val) {
		try {
			instance->img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_detail_ImageFeatures_img_size_const(const cv::detail::ImageFeatures* instance) {
		try {
			cv::Size ret = instance->img_size;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_detail_ImageFeatures_setImg_size_Size(cv::detail::ImageFeatures* instance, const cv::Size* val) {
		try {
			instance->img_size = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::KeyPoint>*> cv_detail_ImageFeatures_keypoints(cv::detail::ImageFeatures* instance) {
		try {
			std::vector<cv::KeyPoint> ret = instance->keypoints;
			return Ok(new std::vector<cv::KeyPoint>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::KeyPoint>*>)
	}
	
	Result_void cv_detail_ImageFeatures_setKeypoints_vector_KeyPoint_(cv::detail::ImageFeatures* instance, std::vector<cv::KeyPoint>* val) {
		try {
			instance->keypoints = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::UMat*> cv_detail_ImageFeatures_descriptors(cv::detail::ImageFeatures* instance) {
		try {
			cv::UMat ret = instance->descriptors;
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(Result<cv::UMat*>)
	}
	
	Result_void cv_detail_ImageFeatures_setDescriptors_UMat(cv::detail::ImageFeatures* instance, cv::UMat* val) {
		try {
			instance->descriptors = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_ImageFeatures_delete(cv::detail::ImageFeatures* instance) {
		delete instance;
	}
	Result<std::vector<cv::KeyPoint>*> cv_detail_ImageFeatures_getKeypoints(cv::detail::ImageFeatures* instance) {
		try {
			std::vector<cv::KeyPoint> ret = instance->getKeypoints();
			return Ok(new std::vector<cv::KeyPoint>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::KeyPoint>*>)
	}
	
	Result<int> cv_detail_MatchesInfo_src_img_idx_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->src_img_idx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setSrc_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->src_img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_MatchesInfo_dst_img_idx_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->dst_img_idx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setDst_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->dst_img_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::DMatch>*> cv_detail_MatchesInfo_matches(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<cv::DMatch> ret = instance->matches;
			return Ok(new std::vector<cv::DMatch>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::DMatch>*>)
	}
	
	Result_void cv_detail_MatchesInfo_setMatches_vector_DMatch_(cv::detail::MatchesInfo* instance, std::vector<cv::DMatch>* val) {
		try {
			instance->matches = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<unsigned char>*> cv_detail_MatchesInfo_inliers_mask(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<unsigned char> ret = instance->inliers_mask;
			return Ok(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(Result<std::vector<unsigned char>*>)
	}
	
	Result_void cv_detail_MatchesInfo_setInliers_mask_vector_unsigned_char_(cv::detail::MatchesInfo* instance, std::vector<unsigned char>* val) {
		try {
			instance->inliers_mask = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_detail_MatchesInfo_num_inliers_const(const cv::detail::MatchesInfo* instance) {
		try {
			int ret = instance->num_inliers;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MatchesInfo_setNum_inliers_int(cv::detail::MatchesInfo* instance, int val) {
		try {
			instance->num_inliers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_detail_MatchesInfo_H(cv::detail::MatchesInfo* instance) {
		try {
			cv::Mat ret = instance->H;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_detail_MatchesInfo_setH_Mat(cv::detail::MatchesInfo* instance, cv::Mat* val) {
		try {
			instance->H = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_detail_MatchesInfo_confidence_const(const cv::detail::MatchesInfo* instance) {
		try {
			double ret = instance->confidence;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_detail_MatchesInfo_setConfidence_double(cv::detail::MatchesInfo* instance, double val) {
		try {
			instance->confidence = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_MatchesInfo_delete(cv::detail::MatchesInfo* instance) {
		delete instance;
	}
	Result<cv::detail::MatchesInfo*> cv_detail_MatchesInfo_MatchesInfo() {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::MatchesInfo*>)
	}
	
	Result<cv::detail::MatchesInfo*> cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoX(const cv::detail::MatchesInfo* other) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo(*other);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::MatchesInfo*>)
	}
	
	Result<std::vector<cv::DMatch>*> cv_detail_MatchesInfo_getMatches(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<cv::DMatch> ret = instance->getMatches();
			return Ok(new std::vector<cv::DMatch>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::DMatch>*>)
	}
	
	Result<std::vector<unsigned char>*> cv_detail_MatchesInfo_getInliers(cv::detail::MatchesInfo* instance) {
		try {
			std::vector<unsigned char> ret = instance->getInliers();
			return Ok(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(Result<std::vector<unsigned char>*>)
	}
	
	void cv_Detail_MercatorProjector_delete(cv::detail::MercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_MercatorProjector_mapForward_float_float_floatX_floatX(cv::detail::MercatorProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MercatorProjector_mapBackward_float_float_floatX_floatX(cv::detail::MercatorProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_MercatorWarper_delete(cv::detail::MercatorWarper* instance) {
		delete instance;
	}
	Result<cv::detail::MercatorWarper*> cv_detail_MercatorWarper_MercatorWarper_float(float scale) {
		try {
			cv::detail::MercatorWarper* ret = new cv::detail::MercatorWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::MercatorWarper*>)
	}
	
	void cv_Detail_MultiBandBlender_delete(cv::detail::MultiBandBlender* instance) {
		delete instance;
	}
	Result<cv::detail::MultiBandBlender*> cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(int try_gpu, int num_bands, int weight_type) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender(try_gpu, num_bands, weight_type);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::MultiBandBlender*>)
	}
	
	Result<int> cv_detail_MultiBandBlender_numBands_const(const cv::detail::MultiBandBlender* instance) {
		try {
			int ret = instance->numBands();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_MultiBandBlender_setNumBands_int(cv::detail::MultiBandBlender* instance, int val) {
		try {
			instance->setNumBands(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_prepare_Rect(cv::detail::MultiBandBlender* instance, const cv::Rect* dst_roi) {
		try {
			instance->prepare(*dst_roi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_feed_const__InputArrayX_const__InputArrayX_Point(cv::detail::MultiBandBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, const cv::Point* tl) {
		try {
			instance->feed(*img, *mask, *tl);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_MultiBandBlender_blend_const__InputOutputArrayX_const__InputOutputArrayX(cv::detail::MultiBandBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask) {
		try {
			instance->blend(*dst, *dst_mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_NoBundleAdjuster_delete(cv::detail::NoBundleAdjuster* instance) {
		delete instance;
	}
	Result<cv::detail::NoBundleAdjuster*> cv_detail_NoBundleAdjuster_NoBundleAdjuster() {
		try {
			cv::detail::NoBundleAdjuster* ret = new cv::detail::NoBundleAdjuster();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::NoBundleAdjuster*>)
	}
	
	void cv_Detail_NoExposureCompensator_delete(cv::detail::NoExposureCompensator* instance) {
		delete instance;
	}
	Result_void cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayX_const__InputArrayX(cv::detail::NoExposureCompensator* instance, int unnamed, const cv::Point* unnamed_1, const cv::_InputOutputArray* unnamed_2, const cv::_InputArray* unnamed_3) {
		try {
			instance->apply(unnamed, *unnamed_1, *unnamed_2, *unnamed_3);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_NoExposureCompensator_getMatGains_vector_Mat_X(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->getMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_NoExposureCompensator_setMatGains_vector_Mat_X(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv) {
		try {
			instance->setMatGains(*umv);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_NoSeamFinder_delete(cv::detail::NoSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_NoSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::NoSeamFinder* instance, const std::vector<cv::UMat>* unnamed, const std::vector<cv::Point>* unnamed_1, std::vector<cv::UMat>* unnamed_2) {
		try {
			instance->find(*unnamed, *unnamed_1, *unnamed_2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PairwiseSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::PairwiseSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_a_const(const cv::detail::PaniniPortraitProjector* instance) {
		try {
			float ret = instance->a;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setA_float(cv::detail::PaniniPortraitProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniPortraitProjector_b_const(const cv::detail::PaniniPortraitProjector* instance) {
		try {
			float ret = instance->b;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_setB_float(cv::detail::PaniniPortraitProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniPortraitProjector_delete(cv::detail::PaniniPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniPortraitProjector_mapForward_float_float_floatX_floatX(cv::detail::PaniniPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatX_floatX(cv::detail::PaniniPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniPortraitWarper_delete(cv::detail::PaniniPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PaniniPortraitWarper*> cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale, A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::PaniniPortraitWarper*>)
	}
	
	Result<float> cv_detail_PaniniProjector_a_const(const cv::detail::PaniniProjector* instance) {
		try {
			float ret = instance->a;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniProjector_setA_float(cv::detail::PaniniProjector* instance, float val) {
		try {
			instance->a = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_detail_PaniniProjector_b_const(const cv::detail::PaniniProjector* instance) {
		try {
			float ret = instance->b;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_PaniniProjector_setB_float(cv::detail::PaniniProjector* instance, float val) {
		try {
			instance->b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniProjector_delete(cv::detail::PaniniProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PaniniProjector_mapForward_float_float_floatX_floatX(cv::detail::PaniniProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PaniniProjector_mapBackward_float_float_floatX_floatX(cv::detail::PaniniProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PaniniWarper_delete(cv::detail::PaniniWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PaniniWarper*> cv_detail_PaniniWarper_PaniniWarper_float_float_float(float scale, float A, float B) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale, A, B);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::PaniniWarper*>)
	}
	
	void cv_Detail_PlanePortraitProjector_delete(cv::detail::PlanePortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlanePortraitProjector_mapForward_float_float_floatX_floatX(cv::detail::PlanePortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PlanePortraitProjector_mapBackward_float_float_floatX_floatX(cv::detail::PlanePortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PlanePortraitWarper_delete(cv::detail::PlanePortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PlanePortraitWarper*> cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(float scale) {
		try {
			cv::detail::PlanePortraitWarper* ret = new cv::detail::PlanePortraitWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::PlanePortraitWarper*>)
	}
	
	void cv_Detail_PlaneProjector_delete(cv::detail::PlaneProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_PlaneProjector_mapForward_float_float_floatX_floatX(cv::detail::PlaneProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_PlaneProjector_mapBackward_float_float_floatX_floatX(cv::detail::PlaneProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_PlaneWarper_delete(cv::detail::PlaneWarper* instance) {
		delete instance;
	}
	Result<cv::detail::PlaneWarper*> cv_detail_PlaneWarper_PlaneWarper_float(float scale) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::PlaneWarper*>)
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Point2f> cv_detail_PlaneWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX_const__InputArrayX(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R, *T);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::PlaneWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::PlaneWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(cv::detail::PlaneWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX(cv::detail::PlaneWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R, *T);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	void cv_Detail_PlaneWarperGpu_delete(cv::detail::PlaneWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::PlaneWarperGpu*> cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(float scale) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::PlaneWarperGpu*>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::PlaneWarperGpu* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::PlaneWarperGpu* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<cv::Point> cv_detail_PlaneWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result<float> cv_detail_ProjectorBase_scale_const(const cv::detail::ProjectorBase* instance) {
		try {
			float ret = instance->scale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_ProjectorBase_setScale_float(cv::detail::ProjectorBase* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_k(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->k;
			return Ok(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_rinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->rinv;
			return Ok(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_r_kinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->r_kinv;
			return Ok(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[9]> cv_detail_ProjectorBase_k_rinv(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[9] = &instance->k_rinv;
			return Ok(ret);
		} OCVRS_CATCH(Result<float(*)[9]>)
	}
	
	Result<float(*)[3]> cv_detail_ProjectorBase_t(cv::detail::ProjectorBase* instance) {
		try {
			float(*ret)[3] = &instance->t;
			return Ok(ret);
		} OCVRS_CATCH(Result<float(*)[3]>)
	}
	
	void cv_Detail_ProjectorBase_delete(cv::detail::ProjectorBase* instance) {
		delete instance;
	}
	Result_void cv_detail_ProjectorBase_setCameraParams_const__InputArrayX_const__InputArrayX_const__InputArrayX(cv::detail::ProjectorBase* instance, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T) {
		try {
			instance->setCameraParams(*K, *R, *T);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_detail_RotationWarper_warpPoint_const_Point2fX_const__InputArrayX_const__InputArrayX(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::RotationWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_RotationWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_detail_RotationWarper_warpBackward_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_Size_const__OutputArrayX(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::Size* dst_size, const cv::_OutputArray* dst) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_detail_RotationWarper_warpRoi_Size_const__InputArrayX_const__InputArrayX(cv::detail::RotationWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<float> cv_detail_RotationWarper_getScale_const(const cv::detail::RotationWarper* instance) {
		try {
			float ret = instance->getScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_detail_RotationWarper_setScale_float(cv::detail::RotationWarper* instance, float unnamed) {
		try {
			instance->setScale(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::SeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::detail::SeamFinder>*> cv_detail_SeamFinder_createDefault_int(int type) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = cv::detail::SeamFinder::createDefault(type);
			return Ok(new cv::Ptr<cv::detail::SeamFinder>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::detail::SeamFinder>*>)
	}
	
	void cv_Detail_SphericalPortraitProjector_delete(cv::detail::SphericalPortraitProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_SphericalPortraitProjector_mapForward_float_float_floatX_floatX(cv::detail::SphericalPortraitProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatX_floatX(cv::detail::SphericalPortraitProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_SphericalPortraitWarper_delete(cv::detail::SphericalPortraitWarper* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalPortraitWarper*> cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(float scale) {
		try {
			cv::detail::SphericalPortraitWarper* ret = new cv::detail::SphericalPortraitWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::SphericalPortraitWarper*>)
	}
	
	Result_void cv_detail_SphericalProjector_mapForward_float_float_floatX_floatX(cv::detail::SphericalProjector instance, float x, float y, float* u, float* v) {
		try {
			instance.mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_SphericalProjector_mapBackward_float_float_floatX_floatX(cv::detail::SphericalProjector instance, float u, float v, float* x, float* y) {
		try {
			instance.mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_SphericalWarper_delete(cv::detail::SphericalWarper* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalWarper*> cv_detail_SphericalWarper_SphericalWarper_float(float scale) {
		try {
			cv::detail::SphericalWarper* ret = new cv::detail::SphericalWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::SphericalWarper*>)
	}
	
	Result<cv::Rect> cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::SphericalWarper* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_SphericalWarper_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::SphericalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_SphericalWarperGpu_delete(cv::detail::SphericalWarperGpu* instance) {
		delete instance;
	}
	Result<cv::detail::SphericalWarperGpu*> cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(float scale) {
		try {
			cv::detail::SphericalWarperGpu* ret = new cv::detail::SphericalWarperGpu(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::SphericalWarperGpu*>)
	}
	
	Result<cv::Rect> cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::detail::SphericalWarperGpu* instance, const cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Point> cv_detail_SphericalWarperGpu_warp_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_const__OutputArrayX(cv::detail::SphericalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	void cv_Detail_StereographicProjector_delete(cv::detail::StereographicProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_StereographicProjector_mapForward_float_float_floatX_floatX(cv::detail::StereographicProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_StereographicProjector_mapBackward_float_float_floatX_floatX(cv::detail::StereographicProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_StereographicWarper_delete(cv::detail::StereographicWarper* instance) {
		delete instance;
	}
	Result<cv::detail::StereographicWarper*> cv_detail_StereographicWarper_StereographicWarper_float(float scale) {
		try {
			cv::detail::StereographicWarper* ret = new cv::detail::StereographicWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::StereographicWarper*>)
	}
	
	void cv_Detail_TransverseMercatorProjector_delete(cv::detail::TransverseMercatorProjector* instance) {
		delete instance;
	}
	Result_void cv_detail_TransverseMercatorProjector_mapForward_float_float_floatX_floatX(cv::detail::TransverseMercatorProjector* instance, float x, float y, float* u, float* v) {
		try {
			instance->mapForward(x, y, *u, *v);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatX_floatX(cv::detail::TransverseMercatorProjector* instance, float u, float v, float* x, float* y) {
		try {
			instance->mapBackward(u, v, *x, *y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Detail_TransverseMercatorWarper_delete(cv::detail::TransverseMercatorWarper* instance) {
		delete instance;
	}
	Result<cv::detail::TransverseMercatorWarper*> cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(float scale) {
		try {
			cv::detail::TransverseMercatorWarper* ret = new cv::detail::TransverseMercatorWarper(scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::detail::TransverseMercatorWarper*>)
	}
	
	void cv_Detail_VoronoiSeamFinder_delete(cv::detail::VoronoiSeamFinder* instance) {
		delete instance;
	}
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_UMat_X_const_vector_Point_X_vector_UMat_X(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*src, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_VoronoiSeamFinder_find_const_vector_Size_X_const_vector_Point_X_vector_UMat_X(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::Size>* size, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks) {
		try {
			instance->find(*size, *corners, *masks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
