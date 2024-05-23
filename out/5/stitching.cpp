#include "ocvrs_common.hpp"
#include <opencv2/stitching.hpp>
#include "stitching_types.hpp"

extern "C" {
	// autoDetectWaveCorrectKind(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:342
	// ("cv::detail::autoDetectWaveCorrectKind", vec![(pred!(mut, ["rmats"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_detail_autoDetectWaveCorrectKind_const_vectorLMatGR(const std::vector<cv::Mat>* rmats, Result<cv::detail::WaveCorrectKind>* ocvrs_return) {
		try {
			cv::detail::WaveCorrectKind ret = cv::detail::autoDetectWaveCorrectKind(*rmats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::computeImageFeatures(CppPassByVoidPtr, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:86
	// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "image", "features"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "cv::detail::ImageFeatures*"]), _)]),
	void cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* image, cv::detail::ImageFeatures* features, ResultVoid* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *image, *features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeImageFeatures(const Ptr<Feature2D> &, InputArray, ImageFeatures &, InputArray)(CppPassByVoidPtr, InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:86
	// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "image", "features", "mask"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "cv::detail::ImageFeatures*", "const cv::_InputArray*"]), _)]),
	void cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR_const__InputArrayR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* image, cv::detail::ImageFeatures* features, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *image, *features, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::computeImageFeatures(CppPassByVoidPtr, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:73
	// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "images", "features"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*"]), _)]),
	void cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* images, std::vector<cv::detail::ImageFeatures>* features, ResultVoid* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *images, *features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeImageFeatures(const Ptr<Feature2D> &, InputArrayOfArrays, std::vector<ImageFeatures> &, InputArrayOfArrays)(CppPassByVoidPtr, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:73
	// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "images", "features", "masks"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*", "const cv::_InputArray*"]), _)]),
	void cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR_const__InputArrayR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* images, std::vector<cv::detail::ImageFeatures>* features, const cv::_InputArray* masks, ResultVoid* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *images, *features, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLaplacePyrGpu(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:173
	// ("cv::detail::createLaplacePyrGpu", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vectorLUMatGR(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr, ResultVoid* ocvrs_return) {
		try {
			cv::detail::createLaplacePyrGpu(*img, num_levels, *pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLaplacePyr(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:172
	// ("cv::detail::createLaplacePyr", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_createLaplacePyr_const__InputArrayR_int_vectorLUMatGR(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr, ResultVoid* ocvrs_return) {
		try {
			cv::detail::createLaplacePyr(*img, num_levels, *pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWeightMap(InputArray, float, InputOutputArray)(InputArray, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:170
	// ("cv::detail::createWeightMap", vec![(pred!(mut, ["mask", "sharpness", "weight"], ["const cv::_InputArray*", "float", "const cv::_InputOutputArray*"]), _)]),
	void cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(const cv::_InputArray* mask, float sharpness, const cv::_InputOutputArray* weight, ResultVoid* ocvrs_return) {
		try {
			cv::detail::createWeightMap(*mask, sharpness, *weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findMaxSpanningTree(int, const std::vector<MatchesInfo> &, Graph &, std::vector<int> &)(Primitive, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:364
	// ("cv::detail::findMaxSpanningTree", vec![(pred!(mut, ["num_images", "pairwise_matches", "span_tree", "centers"], ["int", "const std::vector<cv::detail::MatchesInfo>*", "cv::detail::Graph*", "std::vector<int>*"]), _)]),
	void cv_detail_findMaxSpanningTree_int_const_vectorLMatchesInfoGR_GraphR_vectorLintGR(int num_images, const std::vector<cv::detail::MatchesInfo>* pairwise_matches, cv::detail::Graph* span_tree, std::vector<int>* centers, ResultVoid* ocvrs_return) {
		try {
			cv::detail::findMaxSpanningTree(num_images, *pairwise_matches, *span_tree, *centers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// leaveBiggestComponent(std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:359
	// ("cv::detail::leaveBiggestComponent", vec![(pred!(mut, ["features", "pairwise_matches", "conf_threshold"], ["std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
	void cv_detail_leaveBiggestComponent_vectorLImageFeaturesGR_vectorLMatchesInfoGR_float(std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = cv::detail::leaveBiggestComponent(*features, *pairwise_matches, conf_threshold);
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchesGraphAsString(std::vector<String> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:356
	// ("cv::detail::matchesGraphAsString", vec![(pred!(mut, ["paths", "pairwise_matches", "conf_threshold"], ["std::vector<cv::String>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
	void cv_detail_matchesGraphAsString_vectorLStringGR_vectorLMatchesInfoGR_float(std::vector<cv::String>* paths, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::detail::matchesGraphAsString(*paths, *pairwise_matches, conf_threshold);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalizeUsingWeightMap(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:168
	// ("cv::detail::normalizeUsingWeightMap", vec![(pred!(mut, ["weight", "src"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* weight, const cv::_InputOutputArray* src, ResultVoid* ocvrs_return) {
		try {
			cv::detail::normalizeUsingWeightMap(*weight, *src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// overlapRoi(Point, Point, Size, Size, Rect &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:103
	// ("cv::detail::overlapRoi", vec![(pred!(mut, ["tl1", "tl2", "sz1", "sz2", "roi"], ["cv::Point", "cv::Point", "cv::Size", "cv::Size", "cv::Rect*"]), _)]),
	void cv_detail_overlapRoi_Point_Point_Size_Size_RectR(cv::Point* tl1, cv::Point* tl2, cv::Size* sz1, cv::Size* sz2, cv::Rect* roi, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::detail::overlapRoi(*tl1, *tl2, *sz1, *sz2, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// restoreImageFromLaplacePyrGpu(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:177
	// ("cv::detail::restoreImageFromLaplacePyrGpu", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
	void cv_detail_restoreImageFromLaplacePyrGpu_vectorLUMatGR(std::vector<cv::UMat>* pyr, ResultVoid* ocvrs_return) {
		try {
			cv::detail::restoreImageFromLaplacePyrGpu(*pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// restoreImageFromLaplacePyr(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:176
	// ("cv::detail::restoreImageFromLaplacePyr", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
	void cv_detail_restoreImageFromLaplacePyr_vectorLUMatGR(std::vector<cv::UMat>* pyr, ResultVoid* ocvrs_return) {
		try {
			cv::detail::restoreImageFromLaplacePyr(*pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resultRoiIntersection(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:106
	// ("cv::detail::resultRoiIntersection", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
	void cv_detail_resultRoiIntersection_const_vectorLPointGR_const_vectorLSizeGR(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoiIntersection(*corners, *sizes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resultRoi(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:105
	// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
	void cv_detail_resultRoi_const_vectorLPointGR_const_vectorLSizeGR(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *sizes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resultRoi(const std::vector<Point> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:104
	// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "images"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*"]), _)]),
	void cv_detail_resultRoi_const_vectorLPointGR_const_vectorLUMatGR(const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *images);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resultTl(const std::vector<Point> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:107
	// ("cv::detail::resultTl", vec![(pred!(mut, ["corners"], ["const std::vector<cv::Point>*"]), _)]),
	void cv_detail_resultTl_const_vectorLPointGR(const std::vector<cv::Point>* corners, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = cv::detail::resultTl(*corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selectRandomSubset(int, int, std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:110
	// ("cv::detail::selectRandomSubset", vec![(pred!(mut, ["count", "size", "subset"], ["int", "int", "std::vector<int>*"]), _)]),
	void cv_detail_selectRandomSubset_int_int_vectorLintGR(int count, int size, std::vector<int>* subset, ResultVoid* ocvrs_return) {
		try {
			cv::detail::selectRandomSubset(count, size, *subset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stitchingLogLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:112
	// ("cv::detail::stitchingLogLevel", vec![(pred!(mut, [], []), _)]),
	void cv_detail_stitchingLogLevel(Result<int>* ocvrs_return) {
		try {
			int ret = cv::detail::stitchingLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waveCorrect(std::vector<Mat> &, WaveCorrectKind)(CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:349
	// ("cv::detail::waveCorrect", vec![(pred!(mut, ["rmats", "kind"], ["std::vector<cv::Mat>*", "cv::detail::WaveCorrectKind"]), _)]),
	void cv_detail_waveCorrect_vectorLMatGR_WaveCorrectKind(std::vector<cv::Mat>* rmats, cv::detail::WaveCorrectKind kind, ResultVoid* ocvrs_return) {
		try {
			cv::detail::waveCorrect(*rmats, kind);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:155
	// ("cv::AffineWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_AffineWarper_create_const_float(const cv::AffineWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AffineWarper::defaultNew() generated
	// ("cv::AffineWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::AffineWarper* cv_AffineWarper_defaultNew_const() {
			cv::AffineWarper* ret = new cv::AffineWarper();
			return ret;
	}

	// cv::AffineWarper::to_WarperCreator() generated
	// ("cv::AffineWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_AffineWarper_to_WarperCreator(cv::AffineWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::AffineWarper::delete() generated
	// ("cv::AffineWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AffineWarper_delete(cv::AffineWarper* instance) {
			delete instance;
	}

	// CompressedRectilinearPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:201
	// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	void cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(float A, float B, Result<cv::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:201
	// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, [], []), _)]),
	void cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper(Result<cv::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:205
	// ("cv::CompressedRectilinearPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_CompressedRectilinearPortraitWarper_create_const_float(const cv::CompressedRectilinearPortraitWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CompressedRectilinearPortraitWarper::to_WarperCreator() generated
	// ("cv::CompressedRectilinearPortraitWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_CompressedRectilinearPortraitWarper_to_WarperCreator(cv::CompressedRectilinearPortraitWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::CompressedRectilinearPortraitWarper::delete() generated
	// ("cv::CompressedRectilinearPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CompressedRectilinearPortraitWarper_delete(cv::CompressedRectilinearPortraitWarper* instance) {
			delete instance;
	}

	// CompressedRectilinearWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:190
	// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	void cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(float A, float B, Result<cv::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CompressedRectilinearWarper::CompressedRectilinearWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:190
	// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, [], []), _)]),
	void cv_CompressedRectilinearWarper_CompressedRectilinearWarper(Result<cv::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:194
	// ("cv::CompressedRectilinearWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_CompressedRectilinearWarper_create_const_float(const cv::CompressedRectilinearWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CompressedRectilinearWarper::to_WarperCreator() generated
	// ("cv::CompressedRectilinearWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_CompressedRectilinearWarper_to_WarperCreator(cv::CompressedRectilinearWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::CompressedRectilinearWarper::delete() generated
	// ("cv::CompressedRectilinearWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CompressedRectilinearWarper_delete(cv::CompressedRectilinearWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:164
	// ("cv::CylindricalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_CylindricalWarper_create_const_float(const cv::CylindricalWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CylindricalWarper::defaultNew() generated
	// ("cv::CylindricalWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::CylindricalWarper* cv_CylindricalWarper_defaultNew_const() {
			cv::CylindricalWarper* ret = new cv::CylindricalWarper();
			return ret;
	}

	// cv::CylindricalWarper::to_WarperCreator() generated
	// ("cv::CylindricalWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_CylindricalWarper_to_WarperCreator(cv::CylindricalWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::CylindricalWarper::delete() generated
	// ("cv::CylindricalWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CylindricalWarper_delete(cv::CylindricalWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:255
	// ("cv::CylindricalWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_CylindricalWarperGpu_create_const_float(const cv::CylindricalWarperGpu* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CylindricalWarperGpu::defaultNew() generated
	// ("cv::CylindricalWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::CylindricalWarperGpu* cv_CylindricalWarperGpu_defaultNew_const() {
			cv::CylindricalWarperGpu* ret = new cv::CylindricalWarperGpu();
			return ret;
	}

	// cv::CylindricalWarperGpu::to_WarperCreator() generated
	// ("cv::CylindricalWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_CylindricalWarperGpu_to_WarperCreator(cv::CylindricalWarperGpu* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::CylindricalWarperGpu::delete() generated
	// ("cv::CylindricalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CylindricalWarperGpu_delete(cv::CylindricalWarperGpu* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:177
	// ("cv::FisheyeWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_FisheyeWarper_create_const_float(const cv::FisheyeWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FisheyeWarper::defaultNew() generated
	// ("cv::FisheyeWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::FisheyeWarper* cv_FisheyeWarper_defaultNew_const() {
			cv::FisheyeWarper* ret = new cv::FisheyeWarper();
			return ret;
	}

	// cv::FisheyeWarper::to_WarperCreator() generated
	// ("cv::FisheyeWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_FisheyeWarper_to_WarperCreator(cv::FisheyeWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::FisheyeWarper::delete() generated
	// ("cv::FisheyeWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FisheyeWarper_delete(cv::FisheyeWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:233
	// ("cv::MercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_MercatorWarper_create_const_float(const cv::MercatorWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MercatorWarper::defaultNew() generated
	// ("cv::MercatorWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::MercatorWarper* cv_MercatorWarper_defaultNew_const() {
			cv::MercatorWarper* ret = new cv::MercatorWarper();
			return ret;
	}

	// cv::MercatorWarper::to_WarperCreator() generated
	// ("cv::MercatorWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_MercatorWarper_to_WarperCreator(cv::MercatorWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::MercatorWarper::delete() generated
	// ("cv::MercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MercatorWarper_delete(cv::MercatorWarper* instance) {
			delete instance;
	}

	// PaniniPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:223
	// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	void cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(float A, float B, Result<cv::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PaniniPortraitWarper::PaniniPortraitWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:223
	// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, [], []), _)]),
	void cv_PaniniPortraitWarper_PaniniPortraitWarper(Result<cv::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:227
	// ("cv::PaniniPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_PaniniPortraitWarper_create_const_float(const cv::PaniniPortraitWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PaniniPortraitWarper::to_WarperCreator() generated
	// ("cv::PaniniPortraitWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_PaniniPortraitWarper_to_WarperCreator(cv::PaniniPortraitWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::PaniniPortraitWarper::delete() generated
	// ("cv::PaniniPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PaniniPortraitWarper_delete(cv::PaniniPortraitWarper* instance) {
			delete instance;
	}

	// PaniniWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:212
	// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	void cv_PaniniWarper_PaniniWarper_float_float(float A, float B, Result<cv::PaniniWarper*>* ocvrs_return) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PaniniWarper::PaniniWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:212
	// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, [], []), _)]),
	void cv_PaniniWarper_PaniniWarper(Result<cv::PaniniWarper*>* ocvrs_return) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:216
	// ("cv::PaniniWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_PaniniWarper_create_const_float(const cv::PaniniWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PaniniWarper::to_WarperCreator() generated
	// ("cv::PaniniWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_PaniniWarper_to_WarperCreator(cv::PaniniWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::PaniniWarper::delete() generated
	// ("cv::PaniniWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PaniniWarper_delete(cv::PaniniWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:146
	// ("cv::PlaneWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_PlaneWarper_create_const_float(const cv::PlaneWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PlaneWarper::defaultNew() generated
	// ("cv::PlaneWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::PlaneWarper* cv_PlaneWarper_defaultNew_const() {
			cv::PlaneWarper* ret = new cv::PlaneWarper();
			return ret;
	}

	// cv::PlaneWarper::to_WarperCreator() generated
	// ("cv::PlaneWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_PlaneWarper_to_WarperCreator(cv::PlaneWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::PlaneWarper::delete() generated
	// ("cv::PlaneWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PlaneWarper_delete(cv::PlaneWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:248
	// ("cv::PlaneWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_PlaneWarperGpu_create_const_float(const cv::PlaneWarperGpu* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PlaneWarperGpu::defaultNew() generated
	// ("cv::PlaneWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::PlaneWarperGpu* cv_PlaneWarperGpu_defaultNew_const() {
			cv::PlaneWarperGpu* ret = new cv::PlaneWarperGpu();
			return ret;
	}

	// cv::PlaneWarperGpu::to_WarperCreator() generated
	// ("cv::PlaneWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_PlaneWarperGpu_to_WarperCreator(cv::PlaneWarperGpu* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::PlaneWarperGpu::delete() generated
	// ("cv::PlaneWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PlaneWarperGpu_delete(cv::PlaneWarperGpu* instance) {
			delete instance;
	}

	// PyRotationWarper(String, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:55
	// ("cv::PyRotationWarper::PyRotationWarper", vec![(pred!(mut, ["type", "scale"], ["cv::String", "float"]), _)]),
	void cv_PyRotationWarper_PyRotationWarper_String_float(const char* type, float scale, Result<cv::PyRotationWarper*>* ocvrs_return) {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper(std::string(type), scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PyRotationWarper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:56
	// ("cv::PyRotationWarper::PyRotationWarper", vec![(pred!(mut, [], []), _)]),
	void cv_PyRotationWarper_PyRotationWarper(Result<cv::PyRotationWarper*>* ocvrs_return) {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:66
	// ("cv::PyRotationWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_PyRotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:75
	// ("cv::PyRotationWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_PyRotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:86
	// ("cv::PyRotationWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PyRotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::PyRotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:98
	// ("cv::PyRotationWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_PyRotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpBackward(InputArray, InputArray, InputArray, int, int, Size, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:111
	// ("cv::PyRotationWarper::warpBackward", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst_size", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_PyRotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::Size* dst_size, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:120
	// ("cv::PyRotationWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_PyRotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:122
	// ("cv::PyRotationWarper::getScale", vec![(pred!(const, [], []), _)]),
	void cv_PyRotationWarper_getScale_const(const cv::PyRotationWarper* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:123
	// ("cv::PyRotationWarper::setScale", vec![(pred!(mut, ["unnamed"], ["float"]), _)]),
	void cv_PyRotationWarper_setScale_float(cv::PyRotationWarper* instance, float unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->setScale(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PyRotationWarper::delete() generated
	// ("cv::PyRotationWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PyRotationWarper_delete(cv::PyRotationWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:171
	// ("cv::SphericalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_SphericalWarper_create_const_float(const cv::SphericalWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SphericalWarper::defaultNew() generated
	// ("cv::SphericalWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::SphericalWarper* cv_SphericalWarper_defaultNew_const() {
			cv::SphericalWarper* ret = new cv::SphericalWarper();
			return ret;
	}

	// cv::SphericalWarper::to_WarperCreator() generated
	// ("cv::SphericalWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_SphericalWarper_to_WarperCreator(cv::SphericalWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::SphericalWarper::delete() generated
	// ("cv::SphericalWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SphericalWarper_delete(cv::SphericalWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:262
	// ("cv::SphericalWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_SphericalWarperGpu_create_const_float(const cv::SphericalWarperGpu* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SphericalWarperGpu::defaultNew() generated
	// ("cv::SphericalWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::SphericalWarperGpu* cv_SphericalWarperGpu_defaultNew_const() {
			cv::SphericalWarperGpu* ret = new cv::SphericalWarperGpu();
			return ret;
	}

	// cv::SphericalWarperGpu::to_WarperCreator() generated
	// ("cv::SphericalWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_SphericalWarperGpu_to_WarperCreator(cv::SphericalWarperGpu* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::SphericalWarperGpu::delete() generated
	// ("cv::SphericalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SphericalWarperGpu_delete(cv::SphericalWarperGpu* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:183
	// ("cv::StereographicWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_StereographicWarper_create_const_float(const cv::StereographicWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereographicWarper::defaultNew() generated
	// ("cv::StereographicWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::StereographicWarper* cv_StereographicWarper_defaultNew_const() {
			cv::StereographicWarper* ret = new cv::StereographicWarper();
			return ret;
	}

	// cv::StereographicWarper::to_WarperCreator() generated
	// ("cv::StereographicWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_StereographicWarper_to_WarperCreator(cv::StereographicWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::StereographicWarper::delete() generated
	// ("cv::StereographicWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereographicWarper_delete(cv::StereographicWarper* instance) {
			delete instance;
	}

	// create(Mode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:184
	// ("cv::Stitcher::create", vec![(pred!(mut, ["mode"], ["cv::Stitcher::Mode"]), _)]),
	void cv_Stitcher_create_Mode(cv::Stitcher::Mode mode, Result<cv::Ptr<cv::Stitcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create(mode);
			Ok(new cv::Ptr<cv::Stitcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Stitcher::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:184
	// ("cv::Stitcher::create", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_create(Result<cv::Ptr<cv::Stitcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create();
			Ok(new cv::Ptr<cv::Stitcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registrationResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:186
	// ("cv::Stitcher::registrationResol", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_registrationResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->registrationResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRegistrationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:187
	// ("cv::Stitcher::setRegistrationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	void cv_Stitcher_setRegistrationResol_double(cv::Stitcher* instance, double resol_mpx, ResultVoid* ocvrs_return) {
		try {
			instance->setRegistrationResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seamEstimationResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:189
	// ("cv::Stitcher::seamEstimationResol", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_seamEstimationResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->seamEstimationResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSeamEstimationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:190
	// ("cv::Stitcher::setSeamEstimationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	void cv_Stitcher_setSeamEstimationResol_double(cv::Stitcher* instance, double resol_mpx, ResultVoid* ocvrs_return) {
		try {
			instance->setSeamEstimationResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compositingResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:192
	// ("cv::Stitcher::compositingResol", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_compositingResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->compositingResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCompositingResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:193
	// ("cv::Stitcher::setCompositingResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	void cv_Stitcher_setCompositingResol_double(cv::Stitcher* instance, double resol_mpx, ResultVoid* ocvrs_return) {
		try {
			instance->setCompositingResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// panoConfidenceThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:195
	// ("cv::Stitcher::panoConfidenceThresh", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_panoConfidenceThresh_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->panoConfidenceThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPanoConfidenceThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:196
	// ("cv::Stitcher::setPanoConfidenceThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
	void cv_Stitcher_setPanoConfidenceThresh_double(cv::Stitcher* instance, double conf_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setPanoConfidenceThresh(conf_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waveCorrection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:198
	// ("cv::Stitcher::waveCorrection", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_waveCorrection_const(const cv::Stitcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->waveCorrection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWaveCorrection(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:199
	// ("cv::Stitcher::setWaveCorrection", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_Stitcher_setWaveCorrection_bool(cv::Stitcher* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->setWaveCorrection(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// interpolationFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:201
	// ("cv::Stitcher::interpolationFlags", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_interpolationFlags_const(const cv::Stitcher* instance, Result<cv::InterpolationFlags>* ocvrs_return) {
		try {
			cv::InterpolationFlags ret = instance->interpolationFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInterpolationFlags(InterpolationFlags)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:202
	// ("cv::Stitcher::setInterpolationFlags", vec![(pred!(mut, ["interp_flags"], ["cv::InterpolationFlags"]), _)]),
	void cv_Stitcher_setInterpolationFlags_InterpolationFlags(cv::Stitcher* instance, cv::InterpolationFlags interp_flags, ResultVoid* ocvrs_return) {
		try {
			instance->setInterpolationFlags(interp_flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waveCorrectKind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:204
	// ("cv::Stitcher::waveCorrectKind", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_waveCorrectKind_const(const cv::Stitcher* instance, Result<cv::detail::WaveCorrectKind>* ocvrs_return) {
		try {
			cv::detail::WaveCorrectKind ret = instance->waveCorrectKind();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWaveCorrectKind(detail::WaveCorrectKind)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:205
	// ("cv::Stitcher::setWaveCorrectKind", vec![(pred!(mut, ["kind"], ["cv::detail::WaveCorrectKind"]), _)]),
	void cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(cv::Stitcher* instance, cv::detail::WaveCorrectKind kind, ResultVoid* ocvrs_return) {
		try {
			instance->setWaveCorrectKind(kind);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// featuresFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:207
	// ("cv::Stitcher::featuresFinder", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_featuresFinder(cv::Stitcher* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// featuresFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:208
	// ("cv::Stitcher::featuresFinder", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_featuresFinder_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFeaturesFinder(Ptr<Feature2D>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:209
	// ("cv::Stitcher::setFeaturesFinder", vec![(pred!(mut, ["features_finder"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_Stitcher_setFeaturesFinder_PtrLFeature2DG(cv::Stitcher* instance, cv::Ptr<cv::Feature2D>* features_finder, ResultVoid* ocvrs_return) {
		try {
			instance->setFeaturesFinder(*features_finder);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:212
	// ("cv::Stitcher::featuresMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_featuresMatcher(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::FeaturesMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:213
	// ("cv::Stitcher::featuresMatcher", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_featuresMatcher_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::FeaturesMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFeaturesMatcher(Ptr<detail::FeaturesMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:214
	// ("cv::Stitcher::setFeaturesMatcher", vec![(pred!(mut, ["features_matcher"], ["cv::Ptr<cv::detail::FeaturesMatcher>"]), _)]),
	void cv_Stitcher_setFeaturesMatcher_PtrLFeaturesMatcherG(cv::Stitcher* instance, cv::Ptr<cv::detail::FeaturesMatcher>* features_matcher, ResultVoid* ocvrs_return) {
		try {
			instance->setFeaturesMatcher(*features_matcher);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchingMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:217
	// ("cv::Stitcher::matchingMask", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_matchingMask_const(const cv::Stitcher* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->matchingMask();
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatchingMask(const cv::UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:218
	// ("cv::Stitcher::setMatchingMask", vec![(pred!(mut, ["mask"], ["const cv::UMat*"]), _)]),
	void cv_Stitcher_setMatchingMask_const_UMatR(cv::Stitcher* instance, const cv::UMat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->setMatchingMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:224
	// ("cv::Stitcher::bundleAdjuster", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_bundleAdjuster(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			Ok(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:225
	// ("cv::Stitcher::bundleAdjuster", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_bundleAdjuster_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			Ok(new const cv::Ptr<cv::detail::BundleAdjusterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBundleAdjuster(Ptr<detail::BundleAdjusterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:226
	// ("cv::Stitcher::setBundleAdjuster", vec![(pred!(mut, ["bundle_adjuster"], ["cv::Ptr<cv::detail::BundleAdjusterBase>"]), _)]),
	void cv_Stitcher_setBundleAdjuster_PtrLBundleAdjusterBaseG(cv::Stitcher* instance, cv::Ptr<cv::detail::BundleAdjusterBase>* bundle_adjuster, ResultVoid* ocvrs_return) {
		try {
			instance->setBundleAdjuster(*bundle_adjuster);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:229
	// ("cv::Stitcher::estimator", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_estimator(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Estimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			Ok(new cv::Ptr<cv::detail::Estimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:230
	// ("cv::Stitcher::estimator", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_estimator_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Estimator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			Ok(new const cv::Ptr<cv::detail::Estimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEstimator(Ptr<detail::Estimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:231
	// ("cv::Stitcher::setEstimator", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::detail::Estimator>"]), _)]),
	void cv_Stitcher_setEstimator_PtrLEstimatorG(cv::Stitcher* instance, cv::Ptr<cv::detail::Estimator>* estimator, ResultVoid* ocvrs_return) {
		try {
			instance->setEstimator(*estimator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:234
	// ("cv::Stitcher::warper", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_warper(cv::Stitcher* instance, Result<cv::Ptr<cv::WarperCreator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::WarperCreator> ret = instance->warper();
			Ok(new cv::Ptr<cv::WarperCreator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:235
	// ("cv::Stitcher::warper", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_warper_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::WarperCreator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::WarperCreator> ret = instance->warper();
			Ok(new const cv::Ptr<cv::WarperCreator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWarper(Ptr<WarperCreator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:236
	// ("cv::Stitcher::setWarper", vec![(pred!(mut, ["creator"], ["cv::Ptr<cv::WarperCreator>"]), _)]),
	void cv_Stitcher_setWarper_PtrLWarperCreatorG(cv::Stitcher* instance, cv::Ptr<cv::WarperCreator>* creator, ResultVoid* ocvrs_return) {
		try {
			instance->setWarper(*creator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:238
	// ("cv::Stitcher::exposureCompensator", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_exposureCompensator(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:239
	// ("cv::Stitcher::exposureCompensator", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_exposureCompensator_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			Ok(new const cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExposureCompensator(Ptr<detail::ExposureCompensator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:240
	// ("cv::Stitcher::setExposureCompensator", vec![(pred!(mut, ["exposure_comp"], ["cv::Ptr<cv::detail::ExposureCompensator>"]), _)]),
	void cv_Stitcher_setExposureCompensator_PtrLExposureCompensatorG(cv::Stitcher* instance, cv::Ptr<cv::detail::ExposureCompensator>* exposure_comp, ResultVoid* ocvrs_return) {
		try {
			instance->setExposureCompensator(*exposure_comp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seamFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:243
	// ("cv::Stitcher::seamFinder", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_seamFinder(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			Ok(new cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seamFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:244
	// ("cv::Stitcher::seamFinder", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_seamFinder_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			Ok(new const cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSeamFinder(Ptr<detail::SeamFinder>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:245
	// ("cv::Stitcher::setSeamFinder", vec![(pred!(mut, ["seam_finder"], ["cv::Ptr<cv::detail::SeamFinder>"]), _)]),
	void cv_Stitcher_setSeamFinder_PtrLSeamFinderG(cv::Stitcher* instance, cv::Ptr<cv::detail::SeamFinder>* seam_finder, ResultVoid* ocvrs_return) {
		try {
			instance->setSeamFinder(*seam_finder);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blender()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:247
	// ("cv::Stitcher::blender", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_blender(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Blender> ret = instance->blender();
			Ok(new cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blender()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:248
	// ("cv::Stitcher::blender", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_blender_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::Blender> ret = instance->blender();
			Ok(new const cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlender(Ptr<detail::Blender>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:249
	// ("cv::Stitcher::setBlender", vec![(pred!(mut, ["b"], ["cv::Ptr<cv::detail::Blender>"]), _)]),
	void cv_Stitcher_setBlender_PtrLBlenderG(cv::Stitcher* instance, cv::Ptr<cv::detail::Blender>* b, ResultVoid* ocvrs_return) {
		try {
			instance->setBlender(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateTransform(InputArrayOfArrays, InputArrayOfArrays)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:260
	// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images", "masks"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Stitcher_estimateTransform_const__InputArrayR_const__InputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images, *masks);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Stitcher::estimateTransform(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:260
	// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	void cv_Stitcher_estimateTransform_const__InputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransform(InputArrayOfArrays, const std::vector<detail::CameraParams> &, const std::vector<int> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:270
	// ("cv::Stitcher::setTransform", vec![(pred!(mut, ["images", "cameras", "component"], ["const cv::_InputArray*", "const std::vector<cv::detail::CameraParams>*", "const std::vector<int>*"]), _)]),
	void cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR_const_vectorLintGR(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<cv::detail::CameraParams>* cameras, const std::vector<int>* component, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->setTransform(*images, *cameras, *component);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransform(InputArrayOfArrays, const std::vector<detail::CameraParams> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:274
	// ("cv::Stitcher::setTransform", vec![(pred!(mut, ["images", "cameras"], ["const cv::_InputArray*", "const std::vector<cv::detail::CameraParams>*"]), _)]),
	void cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<cv::detail::CameraParams>* cameras, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->setTransform(*images, *cameras);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// composePanorama(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:277
	// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["pano"], ["const cv::_OutputArray*"]), _)]),
	void cv_Stitcher_composePanorama_const__OutputArrayR(cv::Stitcher* instance, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// composePanorama(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:289
	// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*images, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stitch(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:292
	// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stitch(InputArrayOfArrays, InputArrayOfArrays, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:300
	// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "masks", "pano"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *masks, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// component()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:304
	// ("cv::Stitcher::component", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_component_const(const cv::Stitcher* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->component();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cameras()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:308
	// ("cv::Stitcher::cameras", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_cameras_const(const cv::Stitcher* instance, Result<std::vector<cv::detail::CameraParams>*>* ocvrs_return) {
		try {
			std::vector<cv::detail::CameraParams> ret = instance->cameras();
			Ok(new std::vector<cv::detail::CameraParams>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// workScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:309
	// ("cv::Stitcher::workScale", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_workScale_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->workScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resultMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:318
	// ("cv::Stitcher::resultMask", vec![(pred!(const, [], []), _)]),
	void cv_Stitcher_resultMask_const(const cv::Stitcher* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->resultMask();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Stitcher::defaultNew() generated
	// ("cv::Stitcher::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::Stitcher* cv_Stitcher_defaultNew_const() {
			cv::Stitcher* ret = new cv::Stitcher();
			return ret;
	}

	// cv::Stitcher::delete() generated
	// ("cv::Stitcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Stitcher_delete(cv::Stitcher* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:239
	// ("cv::TransverseMercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_TransverseMercatorWarper_create_const_float(const cv::TransverseMercatorWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TransverseMercatorWarper::defaultNew() generated
	// ("cv::TransverseMercatorWarper::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::TransverseMercatorWarper* cv_TransverseMercatorWarper_defaultNew_const() {
			cv::TransverseMercatorWarper* ret = new cv::TransverseMercatorWarper();
			return ret;
	}

	// cv::TransverseMercatorWarper::to_WarperCreator() generated
	// ("cv::TransverseMercatorWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_TransverseMercatorWarper_to_WarperCreator(cv::TransverseMercatorWarper* instance) {
			return dynamic_cast<cv::WarperCreator*>(instance);
	}

	// cv::TransverseMercatorWarper::delete() generated
	// ("cv::TransverseMercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TransverseMercatorWarper_delete(cv::TransverseMercatorWarper* instance) {
			delete instance;
	}

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:136
	// ("cv::WarperCreator::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	void cv_WarperCreator_create_const_float(const cv::WarperCreator* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::WarperCreator::to_AffineWarper() generated
	// ("cv::WarperCreator::to_AffineWarper", vec![(pred!(mut, [], []), _)]),
	cv::AffineWarper* cv_WarperCreator_to_AffineWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::AffineWarper*>(instance);
	}

	// cv::WarperCreator::to_CompressedRectilinearPortraitWarper() generated
	// ("cv::WarperCreator::to_CompressedRectilinearPortraitWarper", vec![(pred!(mut, [], []), _)]),
	cv::CompressedRectilinearPortraitWarper* cv_WarperCreator_to_CompressedRectilinearPortraitWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::CompressedRectilinearPortraitWarper*>(instance);
	}

	// cv::WarperCreator::to_CompressedRectilinearWarper() generated
	// ("cv::WarperCreator::to_CompressedRectilinearWarper", vec![(pred!(mut, [], []), _)]),
	cv::CompressedRectilinearWarper* cv_WarperCreator_to_CompressedRectilinearWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::CompressedRectilinearWarper*>(instance);
	}

	// cv::WarperCreator::to_CylindricalWarper() generated
	// ("cv::WarperCreator::to_CylindricalWarper", vec![(pred!(mut, [], []), _)]),
	cv::CylindricalWarper* cv_WarperCreator_to_CylindricalWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::CylindricalWarper*>(instance);
	}

	// cv::WarperCreator::to_CylindricalWarperGpu() generated
	// ("cv::WarperCreator::to_CylindricalWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::CylindricalWarperGpu* cv_WarperCreator_to_CylindricalWarperGpu(cv::WarperCreator* instance) {
			return dynamic_cast<cv::CylindricalWarperGpu*>(instance);
	}

	// cv::WarperCreator::to_FisheyeWarper() generated
	// ("cv::WarperCreator::to_FisheyeWarper", vec![(pred!(mut, [], []), _)]),
	cv::FisheyeWarper* cv_WarperCreator_to_FisheyeWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::FisheyeWarper*>(instance);
	}

	// cv::WarperCreator::to_MercatorWarper() generated
	// ("cv::WarperCreator::to_MercatorWarper", vec![(pred!(mut, [], []), _)]),
	cv::MercatorWarper* cv_WarperCreator_to_MercatorWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::MercatorWarper*>(instance);
	}

	// cv::WarperCreator::to_PaniniPortraitWarper() generated
	// ("cv::WarperCreator::to_PaniniPortraitWarper", vec![(pred!(mut, [], []), _)]),
	cv::PaniniPortraitWarper* cv_WarperCreator_to_PaniniPortraitWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::PaniniPortraitWarper*>(instance);
	}

	// cv::WarperCreator::to_PaniniWarper() generated
	// ("cv::WarperCreator::to_PaniniWarper", vec![(pred!(mut, [], []), _)]),
	cv::PaniniWarper* cv_WarperCreator_to_PaniniWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::PaniniWarper*>(instance);
	}

	// cv::WarperCreator::to_PlaneWarper() generated
	// ("cv::WarperCreator::to_PlaneWarper", vec![(pred!(mut, [], []), _)]),
	cv::PlaneWarper* cv_WarperCreator_to_PlaneWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::PlaneWarper*>(instance);
	}

	// cv::WarperCreator::to_PlaneWarperGpu() generated
	// ("cv::WarperCreator::to_PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::PlaneWarperGpu* cv_WarperCreator_to_PlaneWarperGpu(cv::WarperCreator* instance) {
			return dynamic_cast<cv::PlaneWarperGpu*>(instance);
	}

	// cv::WarperCreator::to_SphericalWarper() generated
	// ("cv::WarperCreator::to_SphericalWarper", vec![(pred!(mut, [], []), _)]),
	cv::SphericalWarper* cv_WarperCreator_to_SphericalWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::SphericalWarper*>(instance);
	}

	// cv::WarperCreator::to_SphericalWarperGpu() generated
	// ("cv::WarperCreator::to_SphericalWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::SphericalWarperGpu* cv_WarperCreator_to_SphericalWarperGpu(cv::WarperCreator* instance) {
			return dynamic_cast<cv::SphericalWarperGpu*>(instance);
	}

	// cv::WarperCreator::to_StereographicWarper() generated
	// ("cv::WarperCreator::to_StereographicWarper", vec![(pred!(mut, [], []), _)]),
	cv::StereographicWarper* cv_WarperCreator_to_StereographicWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::StereographicWarper*>(instance);
	}

	// cv::WarperCreator::to_TransverseMercatorWarper() generated
	// ("cv::WarperCreator::to_TransverseMercatorWarper", vec![(pred!(mut, [], []), _)]),
	cv::TransverseMercatorWarper* cv_WarperCreator_to_TransverseMercatorWarper(cv::WarperCreator* instance) {
			return dynamic_cast<cv::TransverseMercatorWarper*>(instance);
	}

	// cv::WarperCreator::delete() generated
	// ("cv::WarperCreator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_WarperCreator_delete(cv::WarperCreator* instance) {
			delete instance;
	}

	// AffineBasedEstimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:124
	// ("cv::detail::AffineBasedEstimator::AffineBasedEstimator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineBasedEstimator_AffineBasedEstimator(Result<cv::detail::AffineBasedEstimator*>* ocvrs_return) {
		try {
			cv::detail::AffineBasedEstimator* ret = new cv::detail::AffineBasedEstimator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::AffineBasedEstimator::to_Detail_Estimator() generated
	// ("cv::detail::AffineBasedEstimator::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_AffineBasedEstimator_to_Detail_Estimator(cv::detail::AffineBasedEstimator* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::AffineBasedEstimator::delete() generated
	// ("cv::detail::AffineBasedEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineBasedEstimator_delete(cv::detail::AffineBasedEstimator* instance) {
			delete instance;
	}

	// AffineBestOf2NearestMatcher(bool, bool, float, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:251
	// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, ["full_affine", "try_use_gpu", "match_conf", "num_matches_thresh1"], ["bool", "bool", "float", "int"]), _)]),
	void cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(bool full_affine, bool try_use_gpu, float match_conf, int num_matches_thresh1, Result<cv::detail::AffineBestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher(full_affine, try_use_gpu, match_conf, num_matches_thresh1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:251
	// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher(Result<cv::detail::AffineBestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::AffineBestOf2NearestMatcher::to_Detail_BestOf2NearestMatcher() generated
	// ("cv::detail::AffineBestOf2NearestMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestMatcher* cv_detail_AffineBestOf2NearestMatcher_to_Detail_BestOf2NearestMatcher(cv::detail::AffineBestOf2NearestMatcher* instance) {
			return dynamic_cast<cv::detail::BestOf2NearestMatcher*>(instance);
	}

	// cv::detail::AffineBestOf2NearestMatcher::to_Detail_FeaturesMatcher() generated
	// ("cv::detail::AffineBestOf2NearestMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::FeaturesMatcher* cv_detail_AffineBestOf2NearestMatcher_to_Detail_FeaturesMatcher(cv::detail::AffineBestOf2NearestMatcher* instance) {
			return dynamic_cast<cv::detail::FeaturesMatcher*>(instance);
	}

	// cv::detail::AffineBestOf2NearestMatcher::delete() generated
	// ("cv::detail::AffineBestOf2NearestMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineBestOf2NearestMatcher_delete(cv::detail::AffineBestOf2NearestMatcher* instance) {
			delete instance;
	}

	// AffineWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:234
	// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_AffineWarper_AffineWarper_float(float scale, Result<cv::detail::AffineWarper*>* ocvrs_return) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::AffineWarper::AffineWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:234
	// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineWarper_AffineWarper(Result<cv::detail::AffineWarper*>* ocvrs_return) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:243
	// ("cv::detail::AffineWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "H"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:252
	// ("cv::detail::AffineWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "H"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_AffineWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:263
	// ("cv::detail::AffineWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "H", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *H, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:275
	// ("cv::detail::AffineWarper::warp", vec![(pred!(mut, ["src", "K", "H", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::AffineWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* H, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *H, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:284
	// ("cv::detail::AffineWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "H"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::AffineWarper::to_Detail_PlaneWarper() generated
	// ("cv::detail::AffineWarper::to_Detail_PlaneWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::PlaneWarper* cv_detail_AffineWarper_to_Detail_PlaneWarper(cv::detail::AffineWarper* instance) {
			return dynamic_cast<cv::detail::PlaneWarper*>(instance);
	}

	// cv::detail::AffineWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::AffineWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_AffineWarper_to_Detail_RotationWarper(cv::detail::AffineWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::AffineWarper::delete() generated
	// ("cv::detail::AffineWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_AffineWarper_delete(cv::detail::AffineWarper* instance) {
			delete instance;
	}

	// BestOf2NearestMatcher(bool, float, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:196
	// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, ["try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2", "matches_confidence_thresh"], ["bool", "float", "int", "int", "double"]), _)]),
	void cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int_double(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, double matches_confidence_thresh, Result<cv::detail::BestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, matches_confidence_thresh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:196
	// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher(Result<cv::detail::BestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:199
	// ("cv::detail::BestOf2NearestMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestMatcher_collectGarbage(cv::detail::BestOf2NearestMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(bool, float, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:200
	// ("cv::detail::BestOf2NearestMatcher::create", vec![(pred!(mut, ["try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2", "matches_confidence_thresh"], ["bool", "float", "int", "int", "double"]), _)]),
	void cv_detail_BestOf2NearestMatcher_create_bool_float_int_int_double(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, double matches_confidence_thresh, Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::BestOf2NearestMatcher> ret = cv::detail::BestOf2NearestMatcher::create(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, matches_confidence_thresh);
			Ok(new cv::Ptr<cv::detail::BestOf2NearestMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BestOf2NearestMatcher::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:200
	// ("cv::detail::BestOf2NearestMatcher::create", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestMatcher_create(Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::BestOf2NearestMatcher> ret = cv::detail::BestOf2NearestMatcher::create();
			Ok(new cv::Ptr<cv::detail::BestOf2NearestMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BestOf2NearestMatcher::to_Detail_AffineBestOf2NearestMatcher() generated
	// ("cv::detail::BestOf2NearestMatcher::to_Detail_AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineBestOf2NearestMatcher* cv_detail_BestOf2NearestMatcher_to_Detail_AffineBestOf2NearestMatcher(cv::detail::BestOf2NearestMatcher* instance) {
			return dynamic_cast<cv::detail::AffineBestOf2NearestMatcher*>(instance);
	}

	// cv::detail::BestOf2NearestMatcher::to_Detail_BestOf2NearestRangeMatcher() generated
	// ("cv::detail::BestOf2NearestMatcher::to_Detail_BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestRangeMatcher* cv_detail_BestOf2NearestMatcher_to_Detail_BestOf2NearestRangeMatcher(cv::detail::BestOf2NearestMatcher* instance) {
			return dynamic_cast<cv::detail::BestOf2NearestRangeMatcher*>(instance);
	}

	// cv::detail::BestOf2NearestMatcher::to_Detail_FeaturesMatcher() generated
	// ("cv::detail::BestOf2NearestMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::FeaturesMatcher* cv_detail_BestOf2NearestMatcher_to_Detail_FeaturesMatcher(cv::detail::BestOf2NearestMatcher* instance) {
			return dynamic_cast<cv::detail::FeaturesMatcher*>(instance);
	}

	// cv::detail::BestOf2NearestMatcher::delete() generated
	// ("cv::detail::BestOf2NearestMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestMatcher_delete(cv::detail::BestOf2NearestMatcher* instance) {
			delete instance;
	}

	// BestOf2NearestRangeMatcher(int, bool, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:215
	// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, ["range_width", "try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2"], ["int", "bool", "float", "int", "int"]), _)]),
	void cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(int range_width, bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, Result<cv::detail::BestOf2NearestRangeMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:215
	// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher(Result<cv::detail::BestOf2NearestRangeMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BestOf2NearestRangeMatcher::to_Detail_BestOf2NearestMatcher() generated
	// ("cv::detail::BestOf2NearestRangeMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestMatcher* cv_detail_BestOf2NearestRangeMatcher_to_Detail_BestOf2NearestMatcher(cv::detail::BestOf2NearestRangeMatcher* instance) {
			return dynamic_cast<cv::detail::BestOf2NearestMatcher*>(instance);
	}

	// cv::detail::BestOf2NearestRangeMatcher::to_Detail_FeaturesMatcher() generated
	// ("cv::detail::BestOf2NearestRangeMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::FeaturesMatcher* cv_detail_BestOf2NearestRangeMatcher_to_Detail_FeaturesMatcher(cv::detail::BestOf2NearestRangeMatcher* instance) {
			return dynamic_cast<cv::detail::FeaturesMatcher*>(instance);
	}

	// cv::detail::BestOf2NearestRangeMatcher::delete() generated
	// ("cv::detail::BestOf2NearestRangeMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BestOf2NearestRangeMatcher_delete(cv::detail::BestOf2NearestRangeMatcher* instance) {
			delete instance;
	}

	// createDefault(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:69
	// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type", "try_gpu"], ["int", "bool"]), _)]),
	void cv_detail_Blender_createDefault_int_bool(int type, bool try_gpu, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type, try_gpu);
			Ok(new cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::Blender::createDefault(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:69
	// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_detail_Blender_createDefault_int(int type, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type);
			Ok(new cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepare(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:76
	// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
	void cv_detail_Blender_prepare_const_vectorLPointGR_const_vectorLSizeGR(cv::detail::Blender* instance, const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, ResultVoid* ocvrs_return) {
		try {
			instance->prepare(*corners, *sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:78
	// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	void cv_detail_Blender_prepare_Rect(cv::detail::Blender* instance, cv::Rect* dst_roi, ResultVoid* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:85
	// ("cv::detail::Blender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::Blender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:91
	// ("cv::detail::Blender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::Blender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, ResultVoid* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::Blender::defaultNew() generated
	// ("cv::detail::Blender::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::Blender* cv_detail_Blender_defaultNew_const() {
			cv::detail::Blender* ret = new cv::detail::Blender();
			return ret;
	}

	// cv::detail::Blender::to_Detail_FeatherBlender() generated
	// ("cv::detail::Blender::to_Detail_FeatherBlender", vec![(pred!(mut, [], []), _)]),
	cv::detail::FeatherBlender* cv_detail_Blender_to_Detail_FeatherBlender(cv::detail::Blender* instance) {
			return dynamic_cast<cv::detail::FeatherBlender*>(instance);
	}

	// cv::detail::Blender::to_Detail_MultiBandBlender() generated
	// ("cv::detail::Blender::to_Detail_MultiBandBlender", vec![(pred!(mut, [], []), _)]),
	cv::detail::MultiBandBlender* cv_detail_Blender_to_Detail_MultiBandBlender(cv::detail::Blender* instance) {
			return dynamic_cast<cv::detail::MultiBandBlender*>(instance);
	}

	// cv::detail::Blender::delete() generated
	// ("cv::detail::Blender::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_Blender_delete(cv::detail::Blender* instance) {
			delete instance;
	}

	// BlocksChannelsCompensator(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:235
	// ("cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator", vec![(pred!(mut, ["bl_width", "bl_height", "nr_feeds"], ["int", "int", "int"]), _)]),
	void cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds, Result<cv::detail::BlocksChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksChannelsCompensator* ret = new cv::detail::BlocksChannelsCompensator(bl_width, bl_height, nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:235
	// ("cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator(Result<cv::detail::BlocksChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksChannelsCompensator* ret = new cv::detail::BlocksChannelsCompensator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:238
	// ("cv::detail::BlocksChannelsCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_BlocksChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::BlocksChannelsCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BlocksChannelsCompensator::to_Detail_BlocksCompensator() generated
	// ("cv::detail::BlocksChannelsCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksCompensator* cv_detail_BlocksChannelsCompensator_to_Detail_BlocksCompensator(cv::detail::BlocksChannelsCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksCompensator*>(instance);
	}

	// cv::detail::BlocksChannelsCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::BlocksChannelsCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_BlocksChannelsCompensator_to_Detail_ExposureCompensator(cv::detail::BlocksChannelsCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::BlocksChannelsCompensator::delete() generated
	// ("cv::detail::BlocksChannelsCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksChannelsCompensator_delete(cv::detail::BlocksChannelsCompensator* instance) {
			delete instance;
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:176
	// ("cv::detail::BlocksCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::BlocksCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:177
	// ("cv::detail::BlocksCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_BlocksCompensator_getMatGains_vectorLMatGR(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:178
	// ("cv::detail::BlocksCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_BlocksCompensator_setMatGains_vectorLMatGR(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:179
	// ("cv::detail::BlocksCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
	void cv_detail_BlocksCompensator_setNrFeeds_int(cv::detail::BlocksCompensator* instance, int nr_feeds, ResultVoid* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:180
	// ("cv::detail::BlocksCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksCompensator_getNrFeeds(cv::detail::BlocksCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:181
	// ("cv::detail::BlocksCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
	void cv_detail_BlocksCompensator_setSimilarityThreshold_double(cv::detail::BlocksCompensator* instance, double similarity_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:182
	// ("cv::detail::BlocksCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
	void cv_detail_BlocksCompensator_getSimilarityThreshold_const(const cv::detail::BlocksCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:183
	// ("cv::detail::BlocksCompensator::setBlockSize", vec![(pred!(mut, ["width", "height"], ["int", "int"]), _)]),
	void cv_detail_BlocksCompensator_setBlockSize_int_int(cv::detail::BlocksCompensator* instance, int width, int height, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:184
	// ("cv::detail::BlocksCompensator::setBlockSize", vec![(pred!(mut, ["size"], ["cv::Size"]), _)]),
	void cv_detail_BlocksCompensator_setBlockSize_Size(cv::detail::BlocksCompensator* instance, cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(*size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:185
	// ("cv::detail::BlocksCompensator::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_detail_BlocksCompensator_getBlockSize_const(const cv::detail::BlocksCompensator* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNrGainsFilteringIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:186
	// ("cv::detail::BlocksCompensator::setNrGainsFilteringIterations", vec![(pred!(mut, ["nr_iterations"], ["int"]), _)]),
	void cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(cv::detail::BlocksCompensator* instance, int nr_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setNrGainsFilteringIterations(nr_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNrGainsFilteringIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:187
	// ("cv::detail::BlocksCompensator::getNrGainsFilteringIterations", vec![(pred!(const, [], []), _)]),
	void cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(const cv::detail::BlocksCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrGainsFilteringIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BlocksCompensator::to_Detail_BlocksChannelsCompensator() generated
	// ("cv::detail::BlocksCompensator::to_Detail_BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksChannelsCompensator* cv_detail_BlocksCompensator_to_Detail_BlocksChannelsCompensator(cv::detail::BlocksCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksChannelsCompensator*>(instance);
	}

	// cv::detail::BlocksCompensator::to_Detail_BlocksGainCompensator() generated
	// ("cv::detail::BlocksCompensator::to_Detail_BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksGainCompensator* cv_detail_BlocksCompensator_to_Detail_BlocksGainCompensator(cv::detail::BlocksCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksGainCompensator*>(instance);
	}

	// cv::detail::BlocksCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::BlocksCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_BlocksCompensator_to_Detail_ExposureCompensator(cv::detail::BlocksCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::BlocksCompensator::delete() generated
	// ("cv::detail::BlocksCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksCompensator_delete(cv::detail::BlocksCompensator* instance) {
			delete instance;
	}

	// BlocksGainCompensator(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:212
	// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, ["bl_width", "bl_height"], ["int", "int"]), _)]),
	void cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(int bl_width, int bl_height, Result<cv::detail::BlocksGainCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BlocksGainCompensator::BlocksGainCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:212
	// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksGainCompensator_BlocksGainCompensator(Result<cv::detail::BlocksGainCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BlocksGainCompensator(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:214
	// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, ["bl_width", "bl_height", "nr_feeds"], ["int", "int", "int"]), _)]),
	void cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds, Result<cv::detail::BlocksGainCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height, nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:217
	// ("cv::detail::BlocksGainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_BlocksGainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::BlocksGainCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:221
	// ("cv::detail::BlocksGainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::BlocksGainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:224
	// ("cv::detail::BlocksGainCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_BlocksGainCompensator_getMatGains_vectorLMatGR(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:226
	// ("cv::detail::BlocksGainCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_BlocksGainCompensator_setMatGains_vectorLMatGR(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BlocksGainCompensator::to_Detail_BlocksCompensator() generated
	// ("cv::detail::BlocksGainCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksCompensator* cv_detail_BlocksGainCompensator_to_Detail_BlocksCompensator(cv::detail::BlocksGainCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksCompensator*>(instance);
	}

	// cv::detail::BlocksGainCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::BlocksGainCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_BlocksGainCompensator_to_Detail_ExposureCompensator(cv::detail::BlocksGainCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::BlocksGainCompensator::delete() generated
	// ("cv::detail::BlocksGainCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BlocksGainCompensator_delete(cv::detail::BlocksGainCompensator* instance) {
			delete instance;
	}

	// BundleAdjusterAffine()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:292
	// ("cv::detail::BundleAdjusterAffine::BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterAffine_BundleAdjusterAffine(Result<cv::detail::BundleAdjusterAffine*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterAffine* ret = new cv::detail::BundleAdjusterAffine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BundleAdjusterAffine::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::BundleAdjusterAffine::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_BundleAdjusterAffine_to_Detail_BundleAdjusterBase(cv::detail::BundleAdjusterAffine* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::BundleAdjusterAffine::to_Detail_Estimator() generated
	// ("cv::detail::BundleAdjusterAffine::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_BundleAdjusterAffine_to_Detail_Estimator(cv::detail::BundleAdjusterAffine* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::BundleAdjusterAffine::delete() generated
	// ("cv::detail::BundleAdjusterAffine::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterAffine_delete(cv::detail::BundleAdjusterAffine* instance) {
			delete instance;
	}

	// BundleAdjusterAffinePartial()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:316
	// ("cv::detail::BundleAdjusterAffinePartial::BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial(Result<cv::detail::BundleAdjusterAffinePartial*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterAffinePartial* ret = new cv::detail::BundleAdjusterAffinePartial();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BundleAdjusterAffinePartial::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::BundleAdjusterAffinePartial::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_BundleAdjusterAffinePartial_to_Detail_BundleAdjusterBase(cv::detail::BundleAdjusterAffinePartial* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::BundleAdjusterAffinePartial::to_Detail_Estimator() generated
	// ("cv::detail::BundleAdjusterAffinePartial::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_BundleAdjusterAffinePartial_to_Detail_Estimator(cv::detail::BundleAdjusterAffinePartial* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::BundleAdjusterAffinePartial::delete() generated
	// ("cv::detail::BundleAdjusterAffinePartial::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterAffinePartial_delete(cv::detail::BundleAdjusterAffinePartial* instance) {
			delete instance;
	}

	// refinementMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:136
	// ("cv::detail::BundleAdjusterBase::refinementMask", vec![(pred!(const, [], []), _)]),
	void cv_detail_BundleAdjusterBase_refinementMask_const(const cv::detail::BundleAdjusterBase* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->refinementMask();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefinementMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:137
	// ("cv::detail::BundleAdjusterBase::setRefinementMask", vec![(pred!(mut, ["mask"], ["const cv::Mat*"]), _)]),
	void cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(cv::detail::BundleAdjusterBase* instance, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->setRefinementMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// confThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:143
	// ("cv::detail::BundleAdjusterBase::confThresh", vec![(pred!(const, [], []), _)]),
	void cv_detail_BundleAdjusterBase_confThresh_const(const cv::detail::BundleAdjusterBase* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->confThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setConfThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:144
	// ("cv::detail::BundleAdjusterBase::setConfThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
	void cv_detail_BundleAdjusterBase_setConfThresh_double(cv::detail::BundleAdjusterBase* instance, double conf_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setConfThresh(conf_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// termCriteria()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:146
	// ("cv::detail::BundleAdjusterBase::termCriteria", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterBase_termCriteria(cv::detail::BundleAdjusterBase* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->termCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:147
	// ("cv::detail::BundleAdjusterBase::setTermCriteria", vec![(pred!(mut, ["term_criteria"], ["const cv::TermCriteria*"]), _)]),
	void cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(cv::detail::BundleAdjusterBase* instance, const cv::TermCriteria* term_criteria, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*term_criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffine() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffine* cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffine(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterAffine*>(instance);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffinePartial() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffinePartial* cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffinePartial(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterAffinePartial*>(instance);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterRay() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterRay* cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterRay(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterRay*>(instance);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterReproj() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterReproj* cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterReproj(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterReproj*>(instance);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_NoBundleAdjuster() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoBundleAdjuster* cv_detail_BundleAdjusterBase_to_Detail_NoBundleAdjuster(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::NoBundleAdjuster*>(instance);
	}

	// cv::detail::BundleAdjusterBase::to_Detail_Estimator() generated
	// ("cv::detail::BundleAdjusterBase::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_BundleAdjusterBase_to_Detail_Estimator(cv::detail::BundleAdjusterBase* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::BundleAdjusterBase::delete() generated
	// ("cv::detail::BundleAdjusterBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterBase_delete(cv::detail::BundleAdjusterBase* instance) {
			delete instance;
	}

	// BundleAdjusterRay()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:268
	// ("cv::detail::BundleAdjusterRay::BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterRay_BundleAdjusterRay(Result<cv::detail::BundleAdjusterRay*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterRay* ret = new cv::detail::BundleAdjusterRay();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BundleAdjusterRay::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::BundleAdjusterRay::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_BundleAdjusterRay_to_Detail_BundleAdjusterBase(cv::detail::BundleAdjusterRay* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::BundleAdjusterRay::to_Detail_Estimator() generated
	// ("cv::detail::BundleAdjusterRay::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_BundleAdjusterRay_to_Detail_Estimator(cv::detail::BundleAdjusterRay* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::BundleAdjusterRay::delete() generated
	// ("cv::detail::BundleAdjusterRay::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterRay_delete(cv::detail::BundleAdjusterRay* instance) {
			delete instance;
	}

	// BundleAdjusterReproj()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:248
	// ("cv::detail::BundleAdjusterReproj::BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterReproj_BundleAdjusterReproj(Result<cv::detail::BundleAdjusterReproj*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterReproj* ret = new cv::detail::BundleAdjusterReproj();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::BundleAdjusterReproj::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::BundleAdjusterReproj::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_BundleAdjusterReproj_to_Detail_BundleAdjusterBase(cv::detail::BundleAdjusterReproj* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::BundleAdjusterReproj::to_Detail_Estimator() generated
	// ("cv::detail::BundleAdjusterReproj::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_BundleAdjusterReproj_to_Detail_Estimator(cv::detail::BundleAdjusterReproj* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::BundleAdjusterReproj::delete() generated
	// ("cv::detail::BundleAdjusterReproj::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_BundleAdjusterReproj_delete(cv::detail::BundleAdjusterReproj* instance) {
			delete instance;
	}

	// CameraParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:60
	// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CameraParams_CameraParams(Result<cv::detail::CameraParams*>* ocvrs_return) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CameraParams(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:61
	// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
	void cv_detail_CameraParams_CameraParams_const_CameraParamsR(const cv::detail::CameraParams* other, Result<cv::detail::CameraParams*>* ocvrs_return) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:62
	// ("cv::detail::CameraParams::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
	void cv_detail_CameraParams_operatorST_const_CameraParamsR(cv::detail::CameraParams* instance, const cv::detail::CameraParams* other, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*other);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// K()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:63
	// ("cv::detail::CameraParams::K", vec![(pred!(const, [], []), _)]),
	void cv_detail_CameraParams_K_const(const cv::detail::CameraParams* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->K();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CameraParams::implicitClone() generated
	// ("cv::detail::CameraParams::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::CameraParams* cv_detail_CameraParams_implicitClone_const(const cv::detail::CameraParams* instance) {
			return new cv::detail::CameraParams(*instance);
	}

	// cv::detail::CameraParams::focal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:65
	// ("cv::detail::CameraParams::focal", vec![(pred!(const, [], []), _)]),
	double cv_detail_CameraParams_propFocal_const(const cv::detail::CameraParams* instance) {
			double ret = instance->focal;
			return ret;
	}

	// cv::detail::CameraParams::setFocal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:65
	// ("cv::detail::CameraParams::setFocal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_detail_CameraParams_propFocal_const_double(cv::detail::CameraParams* instance, const double val) {
			instance->focal = val;
	}

	// cv::detail::CameraParams::aspect() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:66
	// ("cv::detail::CameraParams::aspect", vec![(pred!(const, [], []), _)]),
	double cv_detail_CameraParams_propAspect_const(const cv::detail::CameraParams* instance) {
			double ret = instance->aspect;
			return ret;
	}

	// cv::detail::CameraParams::setAspect(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:66
	// ("cv::detail::CameraParams::setAspect", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_detail_CameraParams_propAspect_const_double(cv::detail::CameraParams* instance, const double val) {
			instance->aspect = val;
	}

	// cv::detail::CameraParams::ppx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:67
	// ("cv::detail::CameraParams::ppx", vec![(pred!(const, [], []), _)]),
	double cv_detail_CameraParams_propPpx_const(const cv::detail::CameraParams* instance) {
			double ret = instance->ppx;
			return ret;
	}

	// cv::detail::CameraParams::setPpx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:67
	// ("cv::detail::CameraParams::setPpx", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_detail_CameraParams_propPpx_const_double(cv::detail::CameraParams* instance, const double val) {
			instance->ppx = val;
	}

	// cv::detail::CameraParams::ppy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:68
	// ("cv::detail::CameraParams::ppy", vec![(pred!(const, [], []), _)]),
	double cv_detail_CameraParams_propPpy_const(const cv::detail::CameraParams* instance) {
			double ret = instance->ppy;
			return ret;
	}

	// cv::detail::CameraParams::setPpy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:68
	// ("cv::detail::CameraParams::setPpy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_detail_CameraParams_propPpy_const_double(cv::detail::CameraParams* instance, const double val) {
			instance->ppy = val;
	}

	// cv::detail::CameraParams::R() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:69
	// ("cv::detail::CameraParams::R", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_detail_CameraParams_propR_const(const cv::detail::CameraParams* instance) {
			cv::Mat ret = instance->R;
			return new cv::Mat(ret);
	}

	// cv::detail::CameraParams::setR(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:69
	// ("cv::detail::CameraParams::setR", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_detail_CameraParams_propR_const_Mat(cv::detail::CameraParams* instance, const cv::Mat* val) {
			instance->R = *val;
	}

	// cv::detail::CameraParams::t() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:70
	// ("cv::detail::CameraParams::t", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_detail_CameraParams_propT_const(const cv::detail::CameraParams* instance) {
			cv::Mat ret = instance->t;
			return new cv::Mat(ret);
	}

	// cv::detail::CameraParams::setT(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:70
	// ("cv::detail::CameraParams::setT", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_detail_CameraParams_propT_const_Mat(cv::detail::CameraParams* instance, const cv::Mat* val) {
			instance->t = *val;
	}

	// cv::detail::CameraParams::delete() generated
	// ("cv::detail::CameraParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CameraParams_delete(cv::detail::CameraParams* instance) {
			delete instance;
	}

	// ChannelsCompensator(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:149
	// ("cv::detail::ChannelsCompensator::ChannelsCompensator", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
	void cv_detail_ChannelsCompensator_ChannelsCompensator_int(int nr_feeds, Result<cv::detail::ChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::ChannelsCompensator* ret = new cv::detail::ChannelsCompensator(nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ChannelsCompensator::ChannelsCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:149
	// ("cv::detail::ChannelsCompensator::ChannelsCompensator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ChannelsCompensator_ChannelsCompensator(Result<cv::detail::ChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::ChannelsCompensator* ret = new cv::detail::ChannelsCompensator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:151
	// ("cv::detail::ChannelsCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_ChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::ChannelsCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:153
	// ("cv::detail::ChannelsCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::ChannelsCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:154
	// ("cv::detail::ChannelsCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_ChannelsCompensator_getMatGains_vectorLMatGR(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:155
	// ("cv::detail::ChannelsCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_ChannelsCompensator_setMatGains_vectorLMatGR(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:156
	// ("cv::detail::ChannelsCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
	void cv_detail_ChannelsCompensator_setNrFeeds_int(cv::detail::ChannelsCompensator* instance, int nr_feeds, ResultVoid* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:157
	// ("cv::detail::ChannelsCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ChannelsCompensator_getNrFeeds(cv::detail::ChannelsCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:158
	// ("cv::detail::ChannelsCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
	void cv_detail_ChannelsCompensator_setSimilarityThreshold_double(cv::detail::ChannelsCompensator* instance, double similarity_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:159
	// ("cv::detail::ChannelsCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
	void cv_detail_ChannelsCompensator_getSimilarityThreshold_const(const cv::detail::ChannelsCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gains()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:160
	// ("cv::detail::ChannelsCompensator::gains", vec![(pred!(const, [], []), _)]),
	void cv_detail_ChannelsCompensator_gains_const(const cv::detail::ChannelsCompensator* instance, Result<std::vector<cv::Scalar>*>* ocvrs_return) {
		try {
			std::vector<cv::Scalar> ret = instance->gains();
			Ok(new std::vector<cv::Scalar>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ChannelsCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::ChannelsCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_ChannelsCompensator_to_Detail_ExposureCompensator(cv::detail::ChannelsCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::ChannelsCompensator::delete() generated
	// ("cv::detail::ChannelsCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ChannelsCompensator_delete(cv::detail::ChannelsCompensator* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:405
	// ("cv::detail::CompressedRectilinearPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:406
	// ("cv::detail::CompressedRectilinearPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearPortraitProjector::defaultNew() generated
	// ("cv::detail::CompressedRectilinearPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::CompressedRectilinearPortraitProjector* cv_detail_CompressedRectilinearPortraitProjector_defaultNew_const() {
			cv::detail::CompressedRectilinearPortraitProjector* ret = new cv::detail::CompressedRectilinearPortraitProjector();
			return ret;
	}

	// cv::detail::CompressedRectilinearPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
	// ("cv::detail::CompressedRectilinearPortraitProjector::a", vec![(pred!(const, [], []), _)]),
	float cv_detail_CompressedRectilinearPortraitProjector_propA_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
			float ret = instance->a;
			return ret;
	}

	// cv::detail::CompressedRectilinearPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
	// ("cv::detail::CompressedRectilinearPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_CompressedRectilinearPortraitProjector_propA_const_float(cv::detail::CompressedRectilinearPortraitProjector* instance, const float val) {
			instance->a = val;
	}

	// cv::detail::CompressedRectilinearPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
	// ("cv::detail::CompressedRectilinearPortraitProjector::b", vec![(pred!(const, [], []), _)]),
	float cv_detail_CompressedRectilinearPortraitProjector_propB_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
			float ret = instance->b;
			return ret;
	}

	// cv::detail::CompressedRectilinearPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
	// ("cv::detail::CompressedRectilinearPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_CompressedRectilinearPortraitProjector_propB_const_float(cv::detail::CompressedRectilinearPortraitProjector* instance, const float val) {
			instance->b = val;
	}

	// cv::detail::CompressedRectilinearPortraitProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::CompressedRectilinearPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_CompressedRectilinearPortraitProjector_to_Detail_ProjectorBase(cv::detail::CompressedRectilinearPortraitProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::CompressedRectilinearPortraitProjector::delete() generated
	// ("cv::detail::CompressedRectilinearPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CompressedRectilinearPortraitProjector_delete(cv::detail::CompressedRectilinearPortraitProjector* instance) {
			delete instance;
	}

	// CompressedRectilinearPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:413
	// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	void cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(float scale, float A, float B, Result<cv::detail::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:413
	// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float(float scale, Result<cv::detail::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearPortraitWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::CompressedRectilinearPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_CompressedRectilinearPortraitWarper_to_Detail_RotationWarper(cv::detail::CompressedRectilinearPortraitWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::CompressedRectilinearPortraitWarper::delete() generated
	// ("cv::detail::CompressedRectilinearPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CompressedRectilinearPortraitWarper_delete(cv::detail::CompressedRectilinearPortraitWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:384
	// ("cv::detail::CompressedRectilinearProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:385
	// ("cv::detail::CompressedRectilinearProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearProjector::defaultNew() generated
	// ("cv::detail::CompressedRectilinearProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::CompressedRectilinearProjector* cv_detail_CompressedRectilinearProjector_defaultNew_const() {
			cv::detail::CompressedRectilinearProjector* ret = new cv::detail::CompressedRectilinearProjector();
			return ret;
	}

	// cv::detail::CompressedRectilinearProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
	// ("cv::detail::CompressedRectilinearProjector::a", vec![(pred!(const, [], []), _)]),
	float cv_detail_CompressedRectilinearProjector_propA_const(const cv::detail::CompressedRectilinearProjector* instance) {
			float ret = instance->a;
			return ret;
	}

	// cv::detail::CompressedRectilinearProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
	// ("cv::detail::CompressedRectilinearProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_CompressedRectilinearProjector_propA_const_float(cv::detail::CompressedRectilinearProjector* instance, const float val) {
			instance->a = val;
	}

	// cv::detail::CompressedRectilinearProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
	// ("cv::detail::CompressedRectilinearProjector::b", vec![(pred!(const, [], []), _)]),
	float cv_detail_CompressedRectilinearProjector_propB_const(const cv::detail::CompressedRectilinearProjector* instance) {
			float ret = instance->b;
			return ret;
	}

	// cv::detail::CompressedRectilinearProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
	// ("cv::detail::CompressedRectilinearProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_CompressedRectilinearProjector_propB_const_float(cv::detail::CompressedRectilinearProjector* instance, const float val) {
			instance->b = val;
	}

	// cv::detail::CompressedRectilinearProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::CompressedRectilinearProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_CompressedRectilinearProjector_to_Detail_ProjectorBase(cv::detail::CompressedRectilinearProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::CompressedRectilinearProjector::delete() generated
	// ("cv::detail::CompressedRectilinearProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CompressedRectilinearProjector_delete(cv::detail::CompressedRectilinearProjector* instance) {
			delete instance;
	}

	// CompressedRectilinearWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:392
	// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	void cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(float scale, float A, float B, Result<cv::detail::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:392
	// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float(float scale, Result<cv::detail::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CompressedRectilinearWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::CompressedRectilinearWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_CompressedRectilinearWarper_to_Detail_RotationWarper(cv::detail::CompressedRectilinearWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::CompressedRectilinearWarper::delete() generated
	// ("cv::detail::CompressedRectilinearWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CompressedRectilinearWarper_delete(cv::detail::CompressedRectilinearWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:655
	// ("cv::detail::CylindricalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:656
	// ("cv::detail::CylindricalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CylindricalPortraitProjector::defaultNew() generated
	// ("cv::detail::CylindricalPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::CylindricalPortraitProjector* cv_detail_CylindricalPortraitProjector_defaultNew_const() {
			cv::detail::CylindricalPortraitProjector* ret = new cv::detail::CylindricalPortraitProjector();
			return ret;
	}

	// cv::detail::CylindricalPortraitProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::CylindricalPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_CylindricalPortraitProjector_to_Detail_ProjectorBase(cv::detail::CylindricalPortraitProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::CylindricalPortraitProjector::delete() generated
	// ("cv::detail::CylindricalPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CylindricalPortraitProjector_delete(cv::detail::CylindricalPortraitProjector* instance) {
			delete instance;
	}

	// CylindricalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:663
	// ("cv::detail::CylindricalPortraitWarper::CylindricalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(float scale, Result<cv::detail::CylindricalPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::CylindricalPortraitWarper* ret = new cv::detail::CylindricalPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CylindricalPortraitWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::CylindricalPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_CylindricalPortraitWarper_to_Detail_RotationWarper(cv::detail::CylindricalPortraitWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::CylindricalPortraitWarper::delete() generated
	// ("cv::detail::CylindricalPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CylindricalPortraitWarper_delete(cv::detail::CylindricalPortraitWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:326
	// ("cv::detail::CylindricalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:327
	// ("cv::detail::CylindricalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CylindricalProjector::defaultNew() generated
	// ("cv::detail::CylindricalProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::CylindricalProjector* cv_detail_CylindricalProjector_defaultNew_const() {
			cv::detail::CylindricalProjector* ret = new cv::detail::CylindricalProjector();
			return ret;
	}

	// cv::detail::CylindricalProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::CylindricalProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_CylindricalProjector_to_Detail_ProjectorBase(cv::detail::CylindricalProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::CylindricalProjector::delete() generated
	// ("cv::detail::CylindricalProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CylindricalProjector_delete(cv::detail::CylindricalProjector* instance) {
			delete instance;
	}

	// CylindricalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:340
	// ("cv::detail::CylindricalWarper::CylindricalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_CylindricalWarper_CylindricalWarper_float(float scale, Result<cv::detail::CylindricalWarper*>* ocvrs_return) {
		try {
			cv::detail::CylindricalWarper* ret = new cv::detail::CylindricalWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:342
	// ("cv::detail::CylindricalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:343
	// ("cv::detail::CylindricalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CylindricalWarper::to_Detail_CylindricalWarperGpu() generated
	// ("cv::detail::CylindricalWarper::to_Detail_CylindricalWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::detail::CylindricalWarperGpu* cv_detail_CylindricalWarper_to_Detail_CylindricalWarperGpu(cv::detail::CylindricalWarper* instance) {
			return dynamic_cast<cv::detail::CylindricalWarperGpu*>(instance);
	}

	// cv::detail::CylindricalWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::CylindricalWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_CylindricalWarper_to_Detail_RotationWarper(cv::detail::CylindricalWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::CylindricalWarper::delete() generated
	// ("cv::detail::CylindricalWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CylindricalWarper_delete(cv::detail::CylindricalWarper* instance) {
			delete instance;
	}

	// CylindricalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:598
	// ("cv::detail::CylindricalWarperGpu::CylindricalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(float scale, Result<cv::detail::CylindricalWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::CylindricalWarperGpu* ret = new cv::detail::CylindricalWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:605
	// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:613
	// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:625
	// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:627
	// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::CylindricalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::CylindricalWarperGpu::to_Detail_CylindricalWarper() generated
	// ("cv::detail::CylindricalWarperGpu::to_Detail_CylindricalWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::CylindricalWarper* cv_detail_CylindricalWarperGpu_to_Detail_CylindricalWarper(cv::detail::CylindricalWarperGpu* instance) {
			return dynamic_cast<cv::detail::CylindricalWarper*>(instance);
	}

	// cv::detail::CylindricalWarperGpu::to_Detail_RotationWarper() generated
	// ("cv::detail::CylindricalWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_CylindricalWarperGpu_to_Detail_RotationWarper(cv::detail::CylindricalWarperGpu* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::CylindricalWarperGpu::delete() generated
	// ("cv::detail::CylindricalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CylindricalWarperGpu_delete(cv::detail::CylindricalWarperGpu* instance) {
			delete instance;
	}

	// DisjointSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:58
	// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
	void cv_detail_DisjointSets_DisjointSets_int(int elem_count, Result<cv::detail::DisjointSets*>* ocvrs_return) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets(elem_count);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::DisjointSets::DisjointSets() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:58
	// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, [], []), _)]),
	void cv_detail_DisjointSets_DisjointSets(Result<cv::detail::DisjointSets*>* ocvrs_return) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOneElemSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:60
	// ("cv::detail::DisjointSets::createOneElemSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
	void cv_detail_DisjointSets_createOneElemSets_int(cv::detail::DisjointSets* instance, int elem_count, ResultVoid* ocvrs_return) {
		try {
			instance->createOneElemSets(elem_count);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findSetByElem(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:61
	// ("cv::detail::DisjointSets::findSetByElem", vec![(pred!(mut, ["elem"], ["int"]), _)]),
	void cv_detail_DisjointSets_findSetByElem_int(cv::detail::DisjointSets* instance, int elem, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findSetByElem(elem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mergeSets(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:62
	// ("cv::detail::DisjointSets::mergeSets", vec![(pred!(mut, ["set1", "set2"], ["int", "int"]), _)]),
	void cv_detail_DisjointSets_mergeSets_int_int(cv::detail::DisjointSets* instance, int set1, int set2, Result<int>* ocvrs_return) {
		try {
			int ret = instance->mergeSets(set1, set2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::DisjointSets::parent() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:64
	// ("cv::detail::DisjointSets::parent", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_detail_DisjointSets_propParent_const(const cv::detail::DisjointSets* instance) {
			std::vector<int> ret = instance->parent;
			return new std::vector<int>(ret);
	}

	// cv::detail::DisjointSets::setParent(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:64
	// ("cv::detail::DisjointSets::setParent", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_detail_DisjointSets_propParent_const_vectorLintG(cv::detail::DisjointSets* instance, const std::vector<int>* val) {
			instance->parent = *val;
	}

	// cv::detail::DisjointSets::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:65
	// ("cv::detail::DisjointSets::size", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_detail_DisjointSets_propSize_const(const cv::detail::DisjointSets* instance) {
			std::vector<int> ret = instance->size;
			return new std::vector<int>(ret);
	}

	// cv::detail::DisjointSets::setSize(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:65
	// ("cv::detail::DisjointSets::setSize", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_detail_DisjointSets_propSize_const_vectorLintG(cv::detail::DisjointSets* instance, const std::vector<int>* val) {
			instance->size = *val;
	}

	// cv::detail::DisjointSets::delete() generated
	// ("cv::detail::DisjointSets::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_DisjointSets_delete(cv::detail::DisjointSets* instance) {
			delete instance;
	}

	// DpSeamFinder(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:125
	// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, ["costFunc"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
	void cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cv::detail::DpSeamFinder::CostFunction costFunc, Result<cv::detail::DpSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(costFunc);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::DpSeamFinder::DpSeamFinder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:125
	// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, [], []), _)]),
	void cv_detail_DpSeamFinder_DpSeamFinder(Result<cv::detail::DpSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DpSeamFinder(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:126
	// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, ["costFunc"], ["cv::String"]), _)]),
	void cv_detail_DpSeamFinder_DpSeamFinder_String(const char* costFunc, Result<cv::detail::DpSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(std::string(costFunc));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// costFunction()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:128
	// ("cv::detail::DpSeamFinder::costFunction", vec![(pred!(const, [], []), _)]),
	void cv_detail_DpSeamFinder_costFunction_const(const cv::detail::DpSeamFinder* instance, Result<cv::detail::DpSeamFinder::CostFunction>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder::CostFunction ret = instance->costFunction();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCostFunction(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:129
	// ("cv::detail::DpSeamFinder::setCostFunction", vec![(pred!(mut, ["val"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
	void cv_detail_DpSeamFinder_setCostFunction_CostFunction(cv::detail::DpSeamFinder* instance, cv::detail::DpSeamFinder::CostFunction val, ResultVoid* ocvrs_return) {
		try {
			instance->setCostFunction(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCostFunction(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:130
	// ("cv::detail::DpSeamFinder::setCostFunction", vec![(pred!(mut, ["val"], ["cv::String"]), _)]),
	void cv_detail_DpSeamFinder_setCostFunction_String(cv::detail::DpSeamFinder* instance, const char* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCostFunction(std::string(val));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:132
	// ("cv::detail::DpSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_DpSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::DpSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::DpSeamFinder::to_Detail_SeamFinder() generated
	// ("cv::detail::DpSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_DpSeamFinder_to_Detail_SeamFinder(cv::detail::DpSeamFinder* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::DpSeamFinder::delete() generated
	// ("cv::detail::DpSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_DpSeamFinder_delete(cv::detail::DpSeamFinder* instance) {
			delete instance;
	}

	// operator()(const std::vector<ImageFeatures> &, const std::vector<MatchesInfo> &, std::vector<CameraParams> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:77
	// ("cv::detail::Estimator::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "cameras"], ["const std::vector<cv::detail::ImageFeatures>*", "const std::vector<cv::detail::MatchesInfo>*", "std::vector<cv::detail::CameraParams>*"]), _)]),
	void cv_detail_Estimator_operator___const_vectorLImageFeaturesGR_const_vectorLMatchesInfoGR_vectorLCameraParamsGR(cv::detail::Estimator* instance, const std::vector<cv::detail::ImageFeatures>* features, const std::vector<cv::detail::MatchesInfo>* pairwise_matches, std::vector<cv::detail::CameraParams>* cameras, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator()(*features, *pairwise_matches, *cameras);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::Estimator::to_Detail_AffineBasedEstimator() generated
	// ("cv::detail::Estimator::to_Detail_AffineBasedEstimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineBasedEstimator* cv_detail_Estimator_to_Detail_AffineBasedEstimator(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::AffineBasedEstimator*>(instance);
	}

	// cv::detail::Estimator::to_Detail_BundleAdjusterAffine() generated
	// ("cv::detail::Estimator::to_Detail_BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffine* cv_detail_Estimator_to_Detail_BundleAdjusterAffine(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterAffine*>(instance);
	}

	// cv::detail::Estimator::to_Detail_BundleAdjusterAffinePartial() generated
	// ("cv::detail::Estimator::to_Detail_BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffinePartial* cv_detail_Estimator_to_Detail_BundleAdjusterAffinePartial(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterAffinePartial*>(instance);
	}

	// cv::detail::Estimator::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::Estimator::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_Estimator_to_Detail_BundleAdjusterBase(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::Estimator::to_Detail_BundleAdjusterRay() generated
	// ("cv::detail::Estimator::to_Detail_BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterRay* cv_detail_Estimator_to_Detail_BundleAdjusterRay(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterRay*>(instance);
	}

	// cv::detail::Estimator::to_Detail_BundleAdjusterReproj() generated
	// ("cv::detail::Estimator::to_Detail_BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterReproj* cv_detail_Estimator_to_Detail_BundleAdjusterReproj(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterReproj*>(instance);
	}

	// cv::detail::Estimator::to_Detail_HomographyBasedEstimator() generated
	// ("cv::detail::Estimator::to_Detail_HomographyBasedEstimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::HomographyBasedEstimator* cv_detail_Estimator_to_Detail_HomographyBasedEstimator(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::HomographyBasedEstimator*>(instance);
	}

	// cv::detail::Estimator::to_Detail_NoBundleAdjuster() generated
	// ("cv::detail::Estimator::to_Detail_NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoBundleAdjuster* cv_detail_Estimator_to_Detail_NoBundleAdjuster(cv::detail::Estimator* instance) {
			return dynamic_cast<cv::detail::NoBundleAdjuster*>(instance);
	}

	// cv::detail::Estimator::delete() generated
	// ("cv::detail::Estimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_Estimator_delete(cv::detail::Estimator* instance) {
			delete instance;
	}

	// createDefault(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:68
	// ("cv::detail::ExposureCompensator::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_detail_ExposureCompensator_createDefault_int(int type, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = cv::detail::ExposureCompensator::createDefault(type);
			Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:76
	// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<cv::UMat>*"]), _)]),
	void cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLUMatGR(cv::detail::ExposureCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:79
	// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::ExposureCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:88
	// ("cv::detail::ExposureCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::ExposureCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:89
	// ("cv::detail::ExposureCompensator::getMatGains", vec![(pred!(mut, ["unnamed"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_ExposureCompensator_getMatGains_vectorLMatGR(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:90
	// ("cv::detail::ExposureCompensator::setMatGains", vec![(pred!(mut, ["unnamed"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_ExposureCompensator_setMatGains_vectorLMatGR(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpdateGain(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:91
	// ("cv::detail::ExposureCompensator::setUpdateGain", vec![(pred!(mut, ["b"], ["bool"]), _)]),
	void cv_detail_ExposureCompensator_setUpdateGain_bool(cv::detail::ExposureCompensator* instance, bool b, ResultVoid* ocvrs_return) {
		try {
			instance->setUpdateGain(b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpdateGain()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:92
	// ("cv::detail::ExposureCompensator::getUpdateGain", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ExposureCompensator_getUpdateGain(cv::detail::ExposureCompensator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpdateGain();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ExposureCompensator::to_Detail_BlocksChannelsCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksChannelsCompensator* cv_detail_ExposureCompensator_to_Detail_BlocksChannelsCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksChannelsCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::to_Detail_BlocksCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksCompensator* cv_detail_ExposureCompensator_to_Detail_BlocksCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::to_Detail_BlocksGainCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksGainCompensator* cv_detail_ExposureCompensator_to_Detail_BlocksGainCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::BlocksGainCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::to_Detail_ChannelsCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_ChannelsCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ChannelsCompensator* cv_detail_ExposureCompensator_to_Detail_ChannelsCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::ChannelsCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::to_Detail_GainCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_GainCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::GainCompensator* cv_detail_ExposureCompensator_to_Detail_GainCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::GainCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::to_Detail_NoExposureCompensator() generated
	// ("cv::detail::ExposureCompensator::to_Detail_NoExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoExposureCompensator* cv_detail_ExposureCompensator_to_Detail_NoExposureCompensator(cv::detail::ExposureCompensator* instance) {
			return dynamic_cast<cv::detail::NoExposureCompensator*>(instance);
	}

	// cv::detail::ExposureCompensator::delete() generated
	// ("cv::detail::ExposureCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ExposureCompensator_delete(cv::detail::ExposureCompensator* instance) {
			delete instance;
	}

	// FeatherBlender(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:103
	// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, ["sharpness"], ["float"]), _)]),
	void cv_detail_FeatherBlender_FeatherBlender_float(float sharpness, Result<cv::detail::FeatherBlender*>* ocvrs_return) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender(sharpness);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FeatherBlender::FeatherBlender() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:103
	// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FeatherBlender_FeatherBlender(Result<cv::detail::FeatherBlender*>* ocvrs_return) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sharpness()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:105
	// ("cv::detail::FeatherBlender::sharpness", vec![(pred!(const, [], []), _)]),
	void cv_detail_FeatherBlender_sharpness_const(const cv::detail::FeatherBlender* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->sharpness();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSharpness(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:106
	// ("cv::detail::FeatherBlender::setSharpness", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_detail_FeatherBlender_setSharpness_float(cv::detail::FeatherBlender* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSharpness(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:108
	// ("cv::detail::FeatherBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	void cv_detail_FeatherBlender_prepare_Rect(cv::detail::FeatherBlender* instance, cv::Rect* dst_roi, ResultVoid* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:109
	// ("cv::detail::FeatherBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::FeatherBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:110
	// ("cv::detail::FeatherBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::FeatherBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, ResultVoid* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWeightMaps(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:114
	// ("cv::detail::FeatherBlender::createWeightMaps", vec![(pred!(mut, ["masks", "corners", "weight_maps"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_FeatherBlender_createWeightMaps_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::FeatherBlender* instance, const std::vector<cv::UMat>* masks, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* weight_maps, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->createWeightMaps(*masks, *corners, *weight_maps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FeatherBlender::to_Detail_Blender() generated
	// ("cv::detail::FeatherBlender::to_Detail_Blender", vec![(pred!(mut, [], []), _)]),
	cv::detail::Blender* cv_detail_FeatherBlender_to_Detail_Blender(cv::detail::FeatherBlender* instance) {
			return dynamic_cast<cv::detail::Blender*>(instance);
	}

	// cv::detail::FeatherBlender::delete() generated
	// ("cv::detail::FeatherBlender::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FeatherBlender_delete(cv::detail::FeatherBlender* instance) {
			delete instance;
	}

	// operator()(const ImageFeatures &, const ImageFeatures &, MatchesInfo &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:127
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features1", "features2", "matches_info"], ["const cv::detail::ImageFeatures*", "const cv::detail::ImageFeatures*", "cv::detail::MatchesInfo*"]), _)]),
	void cv_detail_FeaturesMatcher_operator___const_ImageFeaturesR_const_ImageFeaturesR_MatchesInfoR(cv::detail::FeaturesMatcher* instance, const cv::detail::ImageFeatures* features1, const cv::detail::ImageFeatures* features2, cv::detail::MatchesInfo* matches_info, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*features1, *features2, *matches_info);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, const cv::UMat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:140
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "mask"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "const cv::UMat*"]), _)]),
	void cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR_const_UMatR(cv::detail::FeaturesMatcher* instance, const std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, const cv::UMat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*features, *pairwise_matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FeaturesMatcher::operator()(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:140
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*"]), _)]),
	void cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR(cv::detail::FeaturesMatcher* instance, const std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*features, *pairwise_matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isThreadSafe()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:145
	// ("cv::detail::FeaturesMatcher::isThreadSafe", vec![(pred!(const, [], []), _)]),
	void cv_detail_FeaturesMatcher_isThreadSafe_const(const cv::detail::FeaturesMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isThreadSafe();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:149
	// ("cv::detail::FeaturesMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FeaturesMatcher_collectGarbage(cv::detail::FeaturesMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FeaturesMatcher::to_Detail_AffineBestOf2NearestMatcher() generated
	// ("cv::detail::FeaturesMatcher::to_Detail_AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineBestOf2NearestMatcher* cv_detail_FeaturesMatcher_to_Detail_AffineBestOf2NearestMatcher(cv::detail::FeaturesMatcher* instance) {
			return dynamic_cast<cv::detail::AffineBestOf2NearestMatcher*>(instance);
	}

	// cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestMatcher() generated
	// ("cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestMatcher* cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestMatcher(cv::detail::FeaturesMatcher* instance) {
			return dynamic_cast<cv::detail::BestOf2NearestMatcher*>(instance);
	}

	// cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestRangeMatcher() generated
	// ("cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestRangeMatcher* cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestRangeMatcher(cv::detail::FeaturesMatcher* instance) {
			return dynamic_cast<cv::detail::BestOf2NearestRangeMatcher*>(instance);
	}

	// cv::detail::FeaturesMatcher::delete() generated
	// ("cv::detail::FeaturesMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FeaturesMatcher_delete(cv::detail::FeaturesMatcher* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:354
	// ("cv::detail::FisheyeProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:355
	// ("cv::detail::FisheyeProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FisheyeProjector::defaultNew() generated
	// ("cv::detail::FisheyeProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::FisheyeProjector* cv_detail_FisheyeProjector_defaultNew_const() {
			cv::detail::FisheyeProjector* ret = new cv::detail::FisheyeProjector();
			return ret;
	}

	// cv::detail::FisheyeProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::FisheyeProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_FisheyeProjector_to_Detail_ProjectorBase(cv::detail::FisheyeProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::FisheyeProjector::delete() generated
	// ("cv::detail::FisheyeProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FisheyeProjector_delete(cv::detail::FisheyeProjector* instance) {
			delete instance;
	}

	// FisheyeWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:362
	// ("cv::detail::FisheyeWarper::FisheyeWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_FisheyeWarper_FisheyeWarper_float(float scale, Result<cv::detail::FisheyeWarper*>* ocvrs_return) {
		try {
			cv::detail::FisheyeWarper* ret = new cv::detail::FisheyeWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::FisheyeWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::FisheyeWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_FisheyeWarper_to_Detail_RotationWarper(cv::detail::FisheyeWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::FisheyeWarper::delete() generated
	// ("cv::detail::FisheyeWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_FisheyeWarper_delete(cv::detail::FisheyeWarper* instance) {
			delete instance;
	}

	// GainCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:116
	// ("cv::detail::GainCompensator::GainCompensator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GainCompensator_GainCompensator(Result<cv::detail::GainCompensator*>* ocvrs_return) {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GainCompensator(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:118
	// ("cv::detail::GainCompensator::GainCompensator", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
	void cv_detail_GainCompensator_GainCompensator_int(int nr_feeds, Result<cv::detail::GainCompensator*>* ocvrs_return) {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator(nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:120
	// ("cv::detail::GainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_GainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::GainCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// singleFeed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:122
	// ("cv::detail::GainCompensator::singleFeed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_GainCompensator_singleFeed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::GainCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<std::pair<cv::UMat, unsigned char>>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->singleFeed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:124
	// ("cv::detail::GainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::GainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:125
	// ("cv::detail::GainCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_GainCompensator_getMatGains_vectorLMatGR(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:126
	// ("cv::detail::GainCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_GainCompensator_setMatGains_vectorLMatGR(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:127
	// ("cv::detail::GainCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
	void cv_detail_GainCompensator_setNrFeeds_int(cv::detail::GainCompensator* instance, int nr_feeds, ResultVoid* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:128
	// ("cv::detail::GainCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GainCompensator_getNrFeeds(cv::detail::GainCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:129
	// ("cv::detail::GainCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
	void cv_detail_GainCompensator_setSimilarityThreshold_double(cv::detail::GainCompensator* instance, double similarity_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:130
	// ("cv::detail::GainCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
	void cv_detail_GainCompensator_getSimilarityThreshold_const(const cv::detail::GainCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareSimilarityMask(const std::vector<Point> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:131
	// ("cv::detail::GainCompensator::prepareSimilarityMask", vec![(pred!(mut, ["corners", "images"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*"]), _)]),
	void cv_detail_GainCompensator_prepareSimilarityMask_const_vectorLPointGR_const_vectorLUMatGR(cv::detail::GainCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, ResultVoid* ocvrs_return) {
		try {
			instance->prepareSimilarityMask(*corners, *images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gains()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:132
	// ("cv::detail::GainCompensator::gains", vec![(pred!(const, [], []), _)]),
	void cv_detail_GainCompensator_gains_const(const cv::detail::GainCompensator* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->gains();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GainCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::GainCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_GainCompensator_to_Detail_ExposureCompensator(cv::detail::GainCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::GainCompensator::delete() generated
	// ("cv::detail::GainCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GainCompensator_delete(cv::detail::GainCompensator* instance) {
			delete instance;
	}

	// Graph(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:88
	// ("cv::detail::Graph::Graph", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
	void cv_detail_Graph_Graph_int(int num_vertices, Result<cv::detail::Graph*>* ocvrs_return) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph(num_vertices);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::Graph::Graph() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:88
	// ("cv::detail::Graph::Graph", vec![(pred!(mut, [], []), _)]),
	void cv_detail_Graph_Graph(Result<cv::detail::Graph*>* ocvrs_return) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:89
	// ("cv::detail::Graph::create", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
	void cv_detail_Graph_create_int(cv::detail::Graph* instance, int num_vertices, ResultVoid* ocvrs_return) {
		try {
			instance->create(num_vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numVertices()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:90
	// ("cv::detail::Graph::numVertices", vec![(pred!(const, [], []), _)]),
	void cv_detail_Graph_numVertices_const(const cv::detail::Graph* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numVertices();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:91
	// ("cv::detail::Graph::addEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
	void cv_detail_Graph_addEdge_int_int_float(cv::detail::Graph* instance, int from, int to, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->addEdge(from, to, weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::Graph::delete() generated
	// ("cv::detail::Graph::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_Graph_delete(cv::detail::Graph* instance) {
			delete instance;
	}

	// GraphCutSeamFinder(int, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:243
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["int", "float", "float"]), _)]),
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty, Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(cost_type, terminal_cost, bad_region_penalty);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphCutSeamFinder::GraphCutSeamFinder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:243
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder(Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GraphCutSeamFinder(String, float, float)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:245
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["cv::String", "float", "float"]), _)]),
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(const char* cost_type, float terminal_cost, float bad_region_penalty, Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(std::string(cost_type), terminal_cost, bad_region_penalty);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphCutSeamFinder::GraphCutSeamFinder(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:245
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type"], ["cv::String"]), _)]),
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String(const char* cost_type, Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(std::string(cost_type));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:250
	// ("cv::detail::GraphCutSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_GraphCutSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::GraphCutSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphCutSeamFinder::to_Detail_GraphCutSeamFinderBase() generated
	// ("cv::detail::GraphCutSeamFinder::to_Detail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinderBase* cv_detail_GraphCutSeamFinder_to_Detail_GraphCutSeamFinderBase(cv::detail::GraphCutSeamFinder* instance) {
			return dynamic_cast<cv::detail::GraphCutSeamFinderBase*>(instance);
	}

	// cv::detail::GraphCutSeamFinder::to_Detail_SeamFinder() generated
	// ("cv::detail::GraphCutSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_GraphCutSeamFinder_to_Detail_SeamFinder(cv::detail::GraphCutSeamFinder* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::GraphCutSeamFinder::delete() generated
	// ("cv::detail::GraphCutSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphCutSeamFinder_delete(cv::detail::GraphCutSeamFinder* instance) {
			delete instance;
	}

	// cv::detail::GraphCutSeamFinderBase::defaultNew() generated
	// ("cv::detail::GraphCutSeamFinderBase::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::GraphCutSeamFinderBase* cv_detail_GraphCutSeamFinderBase_defaultNew_const() {
			cv::detail::GraphCutSeamFinderBase* ret = new cv::detail::GraphCutSeamFinderBase();
			return ret;
	}

	// cv::detail::GraphCutSeamFinderBase::delete() generated
	// ("cv::detail::GraphCutSeamFinderBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphCutSeamFinderBase_delete(cv::detail::GraphCutSeamFinderBase* instance) {
			delete instance;
	}

	// GraphCutSeamFinderGpu(int, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:264
	// ("cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["int", "float", "float"]), _)]),
	void cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty, Result<cv::detail::GraphCutSeamFinderGpu*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinderGpu* ret = new cv::detail::GraphCutSeamFinderGpu(cost_type, terminal_cost, bad_region_penalty);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:264
	// ("cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu(Result<cv::detail::GraphCutSeamFinderGpu*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinderGpu* ret = new cv::detail::GraphCutSeamFinderGpu();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find(const std::vector<cv::UMat> &, const std::vector<cv::Point> &, std::vector<cv::UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:269
	// ("cv::detail::GraphCutSeamFinderGpu::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_GraphCutSeamFinderGpu_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::GraphCutSeamFinderGpu* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findInPair(size_t, size_t, Rect)(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:271
	// ("cv::detail::GraphCutSeamFinderGpu::findInPair", vec![(pred!(mut, ["first", "second", "roi"], ["size_t", "size_t", "cv::Rect"]), _)]),
	void cv_detail_GraphCutSeamFinderGpu_findInPair_size_t_size_t_Rect(cv::detail::GraphCutSeamFinderGpu* instance, size_t first, size_t second, cv::Rect* roi, ResultVoid* ocvrs_return) {
		try {
			instance->findInPair(first, second, *roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphCutSeamFinderGpu::to_Detail_GraphCutSeamFinderBase() generated
	// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinderBase* cv_detail_GraphCutSeamFinderGpu_to_Detail_GraphCutSeamFinderBase(cv::detail::GraphCutSeamFinderGpu* instance) {
			return dynamic_cast<cv::detail::GraphCutSeamFinderBase*>(instance);
	}

	// cv::detail::GraphCutSeamFinderGpu::to_Detail_PairwiseSeamFinder() generated
	// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::PairwiseSeamFinder* cv_detail_GraphCutSeamFinderGpu_to_Detail_PairwiseSeamFinder(cv::detail::GraphCutSeamFinderGpu* instance) {
			return dynamic_cast<cv::detail::PairwiseSeamFinder*>(instance);
	}

	// cv::detail::GraphCutSeamFinderGpu::to_Detail_SeamFinder() generated
	// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_GraphCutSeamFinderGpu_to_Detail_SeamFinder(cv::detail::GraphCutSeamFinderGpu* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::GraphCutSeamFinderGpu::delete() generated
	// ("cv::detail::GraphCutSeamFinderGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphCutSeamFinderGpu_delete(cv::detail::GraphCutSeamFinderGpu* instance) {
			delete instance;
	}

	// GraphEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:74
	// ("cv::detail::GraphEdge::GraphEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
	void cv_detail_GraphEdge_GraphEdge_int_int_float(int from, int to, float weight, Result<cv::detail::GraphEdge*>* ocvrs_return) {
		try {
			cv::detail::GraphEdge* ret = new cv::detail::GraphEdge(from, to, weight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:75
	// ("cv::detail::GraphEdge::operator<", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
	void cv_detail_GraphEdge_operatorL_const_const_GraphEdgeR(const cv::detail::GraphEdge* instance, const cv::detail::GraphEdge* other, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:76
	// ("cv::detail::GraphEdge::operator>", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
	void cv_detail_GraphEdge_operatorG_const_const_GraphEdgeR(const cv::detail::GraphEdge* instance, const cv::detail::GraphEdge* other, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator>(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::GraphEdge::from() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::from", vec![(pred!(const, [], []), _)]),
	int cv_detail_GraphEdge_propFrom_const(const cv::detail::GraphEdge* instance) {
			int ret = instance->from;
			return ret;
	}

	// cv::detail::GraphEdge::setFrom(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::setFrom", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_GraphEdge_propFrom_const_int(cv::detail::GraphEdge* instance, const int val) {
			instance->from = val;
	}

	// cv::detail::GraphEdge::to() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::to", vec![(pred!(const, [], []), _)]),
	int cv_detail_GraphEdge_propTo_const(const cv::detail::GraphEdge* instance) {
			int ret = instance->to;
			return ret;
	}

	// cv::detail::GraphEdge::setTo(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::setTo", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_GraphEdge_propTo_const_int(cv::detail::GraphEdge* instance, const int val) {
			instance->to = val;
	}

	// cv::detail::GraphEdge::weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:79
	// ("cv::detail::GraphEdge::weight", vec![(pred!(const, [], []), _)]),
	float cv_detail_GraphEdge_propWeight_const(const cv::detail::GraphEdge* instance) {
			float ret = instance->weight;
			return ret;
	}

	// cv::detail::GraphEdge::setWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:79
	// ("cv::detail::GraphEdge::setWeight", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_GraphEdge_propWeight_const_float(cv::detail::GraphEdge* instance, const float val) {
			instance->weight = val;
	}

	// cv::detail::GraphEdge::delete() generated
	// ("cv::detail::GraphEdge::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GraphEdge_delete(cv::detail::GraphEdge* instance) {
			delete instance;
	}

	// HomographyBasedEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:103
	// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, ["is_focals_estimated"], ["bool"]), _)]),
	void cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(bool is_focals_estimated, Result<cv::detail::HomographyBasedEstimator*>* ocvrs_return) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator(is_focals_estimated);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::HomographyBasedEstimator::HomographyBasedEstimator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:103
	// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, [], []), _)]),
	void cv_detail_HomographyBasedEstimator_HomographyBasedEstimator(Result<cv::detail::HomographyBasedEstimator*>* ocvrs_return) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::HomographyBasedEstimator::to_Detail_Estimator() generated
	// ("cv::detail::HomographyBasedEstimator::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_HomographyBasedEstimator_to_Detail_Estimator(cv::detail::HomographyBasedEstimator* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::HomographyBasedEstimator::delete() generated
	// ("cv::detail::HomographyBasedEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_HomographyBasedEstimator_delete(cv::detail::HomographyBasedEstimator* instance) {
			delete instance;
	}

	// getKeypoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:64
	// ("cv::detail::ImageFeatures::getKeypoints", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ImageFeatures_getKeypoints(cv::detail::ImageFeatures* instance, Result<std::vector<cv::KeyPoint>*>* ocvrs_return) {
		try {
			std::vector<cv::KeyPoint> ret = instance->getKeypoints();
			Ok(new std::vector<cv::KeyPoint>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ImageFeatures::implicitClone() generated
	// ("cv::detail::ImageFeatures::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::ImageFeatures* cv_detail_ImageFeatures_implicitClone_const(const cv::detail::ImageFeatures* instance) {
			return new cv::detail::ImageFeatures(*instance);
	}

	// cv::detail::ImageFeatures::defaultNew() generated
	// ("cv::detail::ImageFeatures::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::ImageFeatures* cv_detail_ImageFeatures_defaultNew_const() {
			cv::detail::ImageFeatures* ret = new cv::detail::ImageFeatures();
			return ret;
	}

	// cv::detail::ImageFeatures::img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:60
	// ("cv::detail::ImageFeatures::img_idx", vec![(pred!(const, [], []), _)]),
	int cv_detail_ImageFeatures_propImg_idx_const(const cv::detail::ImageFeatures* instance) {
			int ret = instance->img_idx;
			return ret;
	}

	// cv::detail::ImageFeatures::setImg_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:60
	// ("cv::detail::ImageFeatures::setImg_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_ImageFeatures_propImg_idx_const_int(cv::detail::ImageFeatures* instance, const int val) {
			instance->img_idx = val;
	}

	// cv::detail::ImageFeatures::img_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:61
	// ("cv::detail::ImageFeatures::img_size", vec![(pred!(const, [], []), _)]),
	void cv_detail_ImageFeatures_propImg_size_const(const cv::detail::ImageFeatures* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->img_size;
			*ocvrs_return = ret;
	}

	// cv::detail::ImageFeatures::setImg_size(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:61
	// ("cv::detail::ImageFeatures::setImg_size", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_detail_ImageFeatures_propImg_size_const_Size(cv::detail::ImageFeatures* instance, const cv::Size* val) {
			instance->img_size = *val;
	}

	// cv::detail::ImageFeatures::keypoints() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:62
	// ("cv::detail::ImageFeatures::keypoints", vec![(pred!(const, [], []), _)]),
	std::vector<cv::KeyPoint>* cv_detail_ImageFeatures_propKeypoints_const(const cv::detail::ImageFeatures* instance) {
			std::vector<cv::KeyPoint> ret = instance->keypoints;
			return new std::vector<cv::KeyPoint>(ret);
	}

	// cv::detail::ImageFeatures::setKeypoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:62
	// ("cv::detail::ImageFeatures::setKeypoints", vec![(pred!(mut, ["val"], ["const std::vector<cv::KeyPoint>"]), _)]),
	void cv_detail_ImageFeatures_propKeypoints_const_vectorLKeyPointG(cv::detail::ImageFeatures* instance, const std::vector<cv::KeyPoint>* val) {
			instance->keypoints = *val;
	}

	// cv::detail::ImageFeatures::descriptors() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:63
	// ("cv::detail::ImageFeatures::descriptors", vec![(pred!(const, [], []), _)]),
	cv::UMat* cv_detail_ImageFeatures_propDescriptors_const(const cv::detail::ImageFeatures* instance) {
			cv::UMat ret = instance->descriptors;
			return new cv::UMat(ret);
	}

	// cv::detail::ImageFeatures::setDescriptors(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:63
	// ("cv::detail::ImageFeatures::setDescriptors", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	void cv_detail_ImageFeatures_propDescriptors_const_UMat(cv::detail::ImageFeatures* instance, const cv::UMat* val) {
			instance->descriptors = *val;
	}

	// cv::detail::ImageFeatures::delete() generated
	// ("cv::detail::ImageFeatures::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ImageFeatures_delete(cv::detail::ImageFeatures* instance) {
			delete instance;
	}

	// MatchesInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:101
	// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MatchesInfo_MatchesInfo(Result<cv::detail::MatchesInfo*>* ocvrs_return) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatchesInfo(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:102
	// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
	void cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(const cv::detail::MatchesInfo* other, Result<cv::detail::MatchesInfo*>* ocvrs_return) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:103
	// ("cv::detail::MatchesInfo::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
	void cv_detail_MatchesInfo_operatorST_const_MatchesInfoR(cv::detail::MatchesInfo* instance, const cv::detail::MatchesInfo* other, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*other);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatches()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:112
	// ("cv::detail::MatchesInfo::getMatches", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MatchesInfo_getMatches(cv::detail::MatchesInfo* instance, Result<std::vector<cv::DMatch>*>* ocvrs_return) {
		try {
			std::vector<cv::DMatch> ret = instance->getMatches();
			Ok(new std::vector<cv::DMatch>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInliers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:113
	// ("cv::detail::MatchesInfo::getInliers", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MatchesInfo_getInliers(cv::detail::MatchesInfo* instance, Result<std::vector<unsigned char>*>* ocvrs_return) {
		try {
			std::vector<unsigned char> ret = instance->getInliers();
			Ok(new std::vector<unsigned char>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::MatchesInfo::implicitClone() generated
	// ("cv::detail::MatchesInfo::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::MatchesInfo* cv_detail_MatchesInfo_implicitClone_const(const cv::detail::MatchesInfo* instance) {
			return new cv::detail::MatchesInfo(*instance);
	}

	// cv::detail::MatchesInfo::src_img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:105
	// ("cv::detail::MatchesInfo::src_img_idx", vec![(pred!(const, [], []), _)]),
	int cv_detail_MatchesInfo_propSrc_img_idx_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->src_img_idx;
			return ret;
	}

	// cv::detail::MatchesInfo::setSrc_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:105
	// ("cv::detail::MatchesInfo::setSrc_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_MatchesInfo_propSrc_img_idx_const_int(cv::detail::MatchesInfo* instance, const int val) {
			instance->src_img_idx = val;
	}

	// cv::detail::MatchesInfo::dst_img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:106
	// ("cv::detail::MatchesInfo::dst_img_idx", vec![(pred!(const, [], []), _)]),
	int cv_detail_MatchesInfo_propDst_img_idx_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->dst_img_idx;
			return ret;
	}

	// cv::detail::MatchesInfo::setDst_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:106
	// ("cv::detail::MatchesInfo::setDst_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_MatchesInfo_propDst_img_idx_const_int(cv::detail::MatchesInfo* instance, const int val) {
			instance->dst_img_idx = val;
	}

	// cv::detail::MatchesInfo::matches() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:107
	// ("cv::detail::MatchesInfo::matches", vec![(pred!(const, [], []), _)]),
	std::vector<cv::DMatch>* cv_detail_MatchesInfo_propMatches_const(const cv::detail::MatchesInfo* instance) {
			std::vector<cv::DMatch> ret = instance->matches;
			return new std::vector<cv::DMatch>(ret);
	}

	// cv::detail::MatchesInfo::setMatches(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:107
	// ("cv::detail::MatchesInfo::setMatches", vec![(pred!(mut, ["val"], ["const std::vector<cv::DMatch>"]), _)]),
	void cv_detail_MatchesInfo_propMatches_const_vectorLDMatchG(cv::detail::MatchesInfo* instance, const std::vector<cv::DMatch>* val) {
			instance->matches = *val;
	}

	// cv::detail::MatchesInfo::inliers_mask() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:108
	// ("cv::detail::MatchesInfo::inliers_mask", vec![(pred!(const, [], []), _)]),
	std::vector<unsigned char>* cv_detail_MatchesInfo_propInliers_mask_const(const cv::detail::MatchesInfo* instance) {
			std::vector<unsigned char> ret = instance->inliers_mask;
			return new std::vector<unsigned char>(ret);
	}

	// cv::detail::MatchesInfo::setInliers_mask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:108
	// ("cv::detail::MatchesInfo::setInliers_mask", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
	void cv_detail_MatchesInfo_propInliers_mask_const_vectorLunsigned_charG(cv::detail::MatchesInfo* instance, const std::vector<unsigned char>* val) {
			instance->inliers_mask = *val;
	}

	// cv::detail::MatchesInfo::num_inliers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:109
	// ("cv::detail::MatchesInfo::num_inliers", vec![(pred!(const, [], []), _)]),
	int cv_detail_MatchesInfo_propNum_inliers_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->num_inliers;
			return ret;
	}

	// cv::detail::MatchesInfo::setNum_inliers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:109
	// ("cv::detail::MatchesInfo::setNum_inliers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_MatchesInfo_propNum_inliers_const_int(cv::detail::MatchesInfo* instance, const int val) {
			instance->num_inliers = val;
	}

	// cv::detail::MatchesInfo::H() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:110
	// ("cv::detail::MatchesInfo::H", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_detail_MatchesInfo_propH_const(const cv::detail::MatchesInfo* instance) {
			cv::Mat ret = instance->H;
			return new cv::Mat(ret);
	}

	// cv::detail::MatchesInfo::setH(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:110
	// ("cv::detail::MatchesInfo::setH", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_detail_MatchesInfo_propH_const_Mat(cv::detail::MatchesInfo* instance, const cv::Mat* val) {
			instance->H = *val;
	}

	// cv::detail::MatchesInfo::confidence() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:111
	// ("cv::detail::MatchesInfo::confidence", vec![(pred!(const, [], []), _)]),
	double cv_detail_MatchesInfo_propConfidence_const(const cv::detail::MatchesInfo* instance) {
			double ret = instance->confidence;
			return ret;
	}

	// cv::detail::MatchesInfo::setConfidence(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:111
	// ("cv::detail::MatchesInfo::setConfidence", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_detail_MatchesInfo_propConfidence_const_double(cv::detail::MatchesInfo* instance, const double val) {
			instance->confidence = val;
	}

	// cv::detail::MatchesInfo::delete() generated
	// ("cv::detail::MatchesInfo::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MatchesInfo_delete(cv::detail::MatchesInfo* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:467
	// ("cv::detail::MercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:468
	// ("cv::detail::MercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::MercatorProjector::defaultNew() generated
	// ("cv::detail::MercatorProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::MercatorProjector* cv_detail_MercatorProjector_defaultNew_const() {
			cv::detail::MercatorProjector* ret = new cv::detail::MercatorProjector();
			return ret;
	}

	// cv::detail::MercatorProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::MercatorProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_MercatorProjector_to_Detail_ProjectorBase(cv::detail::MercatorProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::MercatorProjector::delete() generated
	// ("cv::detail::MercatorProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MercatorProjector_delete(cv::detail::MercatorProjector* instance) {
			delete instance;
	}

	// MercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:475
	// ("cv::detail::MercatorWarper::MercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_MercatorWarper_MercatorWarper_float(float scale, Result<cv::detail::MercatorWarper*>* ocvrs_return) {
		try {
			cv::detail::MercatorWarper* ret = new cv::detail::MercatorWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::MercatorWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::MercatorWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_MercatorWarper_to_Detail_RotationWarper(cv::detail::MercatorWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::MercatorWarper::delete() generated
	// ("cv::detail::MercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MercatorWarper_delete(cv::detail::MercatorWarper* instance) {
			delete instance;
	}

	// MultiBandBlender(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:130
	// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, ["try_gpu", "num_bands", "weight_type"], ["int", "int", "int"]), _)]),
	void cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(int try_gpu, int num_bands, int weight_type, Result<cv::detail::MultiBandBlender*>* ocvrs_return) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender(try_gpu, num_bands, weight_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::MultiBandBlender::MultiBandBlender() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:130
	// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MultiBandBlender_MultiBandBlender(Result<cv::detail::MultiBandBlender*>* ocvrs_return) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numBands()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:132
	// ("cv::detail::MultiBandBlender::numBands", vec![(pred!(const, [], []), _)]),
	void cv_detail_MultiBandBlender_numBands_const(const cv::detail::MultiBandBlender* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numBands();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumBands(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:133
	// ("cv::detail::MultiBandBlender::setNumBands", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_detail_MultiBandBlender_setNumBands_int(cv::detail::MultiBandBlender* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setNumBands(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:135
	// ("cv::detail::MultiBandBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	void cv_detail_MultiBandBlender_prepare_Rect(cv::detail::MultiBandBlender* instance, cv::Rect* dst_roi, ResultVoid* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:136
	// ("cv::detail::MultiBandBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::MultiBandBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:137
	// ("cv::detail::MultiBandBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::MultiBandBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, ResultVoid* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::MultiBandBlender::to_Detail_Blender() generated
	// ("cv::detail::MultiBandBlender::to_Detail_Blender", vec![(pred!(mut, [], []), _)]),
	cv::detail::Blender* cv_detail_MultiBandBlender_to_Detail_Blender(cv::detail::MultiBandBlender* instance) {
			return dynamic_cast<cv::detail::Blender*>(instance);
	}

	// cv::detail::MultiBandBlender::delete() generated
	// ("cv::detail::MultiBandBlender::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_MultiBandBlender_delete(cv::detail::MultiBandBlender* instance) {
			delete instance;
	}

	// NoBundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:224
	// ("cv::detail::NoBundleAdjuster::NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
	void cv_detail_NoBundleAdjuster_NoBundleAdjuster(Result<cv::detail::NoBundleAdjuster*>* ocvrs_return) {
		try {
			cv::detail::NoBundleAdjuster* ret = new cv::detail::NoBundleAdjuster();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::NoBundleAdjuster::to_Detail_BundleAdjusterBase() generated
	// ("cv::detail::NoBundleAdjuster::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_detail_NoBundleAdjuster_to_Detail_BundleAdjusterBase(cv::detail::NoBundleAdjuster* instance) {
			return dynamic_cast<cv::detail::BundleAdjusterBase*>(instance);
	}

	// cv::detail::NoBundleAdjuster::to_Detail_Estimator() generated
	// ("cv::detail::NoBundleAdjuster::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::detail::Estimator* cv_detail_NoBundleAdjuster_to_Detail_Estimator(cv::detail::NoBundleAdjuster* instance) {
			return dynamic_cast<cv::detail::Estimator*>(instance);
	}

	// cv::detail::NoBundleAdjuster::delete() generated
	// ("cv::detail::NoBundleAdjuster::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_NoBundleAdjuster_delete(cv::detail::NoBundleAdjuster* instance) {
			delete instance;
	}

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:102
	// ("cv::detail::NoExposureCompensator::feed", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	void cv_detail_NoExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(cv::detail::NoExposureCompensator* instance, const std::vector<cv::Point>* unnamed, const std::vector<cv::UMat>* unnamed_1, const std::vector<std::pair<cv::UMat, unsigned char>>* unnamed_2, ResultVoid* ocvrs_return) {
		try {
			instance->feed(*unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:104
	// ("cv::detail::NoExposureCompensator::apply", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed", "unnamed"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::NoExposureCompensator* instance, int unnamed, cv::Point* unnamed_1, const cv::_InputOutputArray* unnamed_2, const cv::_InputArray* unnamed_3, ResultVoid* ocvrs_return) {
		try {
			instance->apply(unnamed, *unnamed_1, *unnamed_2, *unnamed_3);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:105
	// ("cv::detail::NoExposureCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_NoExposureCompensator_getMatGains_vectorLMatGR(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:106
	// ("cv::detail::NoExposureCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_detail_NoExposureCompensator_setMatGains_vectorLMatGR(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::NoExposureCompensator::defaultNew() generated
	// ("cv::detail::NoExposureCompensator::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::NoExposureCompensator* cv_detail_NoExposureCompensator_defaultNew_const() {
			cv::detail::NoExposureCompensator* ret = new cv::detail::NoExposureCompensator();
			return ret;
	}

	// cv::detail::NoExposureCompensator::to_Detail_ExposureCompensator() generated
	// ("cv::detail::NoExposureCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::detail::ExposureCompensator* cv_detail_NoExposureCompensator_to_Detail_ExposureCompensator(cv::detail::NoExposureCompensator* instance) {
			return dynamic_cast<cv::detail::ExposureCompensator*>(instance);
	}

	// cv::detail::NoExposureCompensator::delete() generated
	// ("cv::detail::NoExposureCompensator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_NoExposureCompensator_delete(cv::detail::NoExposureCompensator* instance) {
			delete instance;
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:79
	// ("cv::detail::NoSeamFinder::find", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_NoSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::NoSeamFinder* instance, const std::vector<cv::UMat>* unnamed, const std::vector<cv::Point>* unnamed_1, std::vector<cv::UMat>* unnamed_2, ResultVoid* ocvrs_return) {
		try {
			instance->find(*unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::NoSeamFinder::defaultNew() generated
	// ("cv::detail::NoSeamFinder::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::NoSeamFinder* cv_detail_NoSeamFinder_defaultNew_const() {
			cv::detail::NoSeamFinder* ret = new cv::detail::NoSeamFinder();
			return ret;
	}

	// cv::detail::NoSeamFinder::to_Detail_SeamFinder() generated
	// ("cv::detail::NoSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_NoSeamFinder_to_Detail_SeamFinder(cv::detail::NoSeamFinder* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::NoSeamFinder::delete() generated
	// ("cv::detail::NoSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_NoSeamFinder_delete(cv::detail::NoSeamFinder* instance) {
			delete instance;
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:87
	// ("cv::detail::PairwiseSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_PairwiseSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::PairwiseSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PairwiseSeamFinder::to_Detail_GraphCutSeamFinderGpu() generated
	// ("cv::detail::PairwiseSeamFinder::to_Detail_GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinderGpu* cv_detail_PairwiseSeamFinder_to_Detail_GraphCutSeamFinderGpu(cv::detail::PairwiseSeamFinder* instance) {
			return dynamic_cast<cv::detail::GraphCutSeamFinderGpu*>(instance);
	}

	// cv::detail::PairwiseSeamFinder::to_Detail_VoronoiSeamFinder() generated
	// ("cv::detail::PairwiseSeamFinder::to_Detail_VoronoiSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::VoronoiSeamFinder* cv_detail_PairwiseSeamFinder_to_Detail_VoronoiSeamFinder(cv::detail::PairwiseSeamFinder* instance) {
			return dynamic_cast<cv::detail::VoronoiSeamFinder*>(instance);
	}

	// cv::detail::PairwiseSeamFinder::to_Detail_SeamFinder() generated
	// ("cv::detail::PairwiseSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_PairwiseSeamFinder_to_Detail_SeamFinder(cv::detail::PairwiseSeamFinder* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::PairwiseSeamFinder::delete() generated
	// ("cv::detail::PairwiseSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PairwiseSeamFinder_delete(cv::detail::PairwiseSeamFinder* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:447
	// ("cv::detail::PaniniPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:448
	// ("cv::detail::PaniniPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniPortraitProjector::defaultNew() generated
	// ("cv::detail::PaniniPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::PaniniPortraitProjector* cv_detail_PaniniPortraitProjector_defaultNew_const() {
			cv::detail::PaniniPortraitProjector* ret = new cv::detail::PaniniPortraitProjector();
			return ret;
	}

	// cv::detail::PaniniPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
	// ("cv::detail::PaniniPortraitProjector::a", vec![(pred!(const, [], []), _)]),
	float cv_detail_PaniniPortraitProjector_propA_const(const cv::detail::PaniniPortraitProjector* instance) {
			float ret = instance->a;
			return ret;
	}

	// cv::detail::PaniniPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
	// ("cv::detail::PaniniPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_PaniniPortraitProjector_propA_const_float(cv::detail::PaniniPortraitProjector* instance, const float val) {
			instance->a = val;
	}

	// cv::detail::PaniniPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
	// ("cv::detail::PaniniPortraitProjector::b", vec![(pred!(const, [], []), _)]),
	float cv_detail_PaniniPortraitProjector_propB_const(const cv::detail::PaniniPortraitProjector* instance) {
			float ret = instance->b;
			return ret;
	}

	// cv::detail::PaniniPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
	// ("cv::detail::PaniniPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_PaniniPortraitProjector_propB_const_float(cv::detail::PaniniPortraitProjector* instance, const float val) {
			instance->b = val;
	}

	// cv::detail::PaniniPortraitProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::PaniniPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_PaniniPortraitProjector_to_Detail_ProjectorBase(cv::detail::PaniniPortraitProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::PaniniPortraitProjector::delete() generated
	// ("cv::detail::PaniniPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PaniniPortraitProjector_delete(cv::detail::PaniniPortraitProjector* instance) {
			delete instance;
	}

	// PaniniPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:455
	// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	void cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(float scale, float A, float B, Result<cv::detail::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniPortraitWarper::PaniniPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:455
	// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float(float scale, Result<cv::detail::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniPortraitWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::PaniniPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_PaniniPortraitWarper_to_Detail_RotationWarper(cv::detail::PaniniPortraitWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::PaniniPortraitWarper::delete() generated
	// ("cv::detail::PaniniPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PaniniPortraitWarper_delete(cv::detail::PaniniPortraitWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:426
	// ("cv::detail::PaniniProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:427
	// ("cv::detail::PaniniProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniProjector::defaultNew() generated
	// ("cv::detail::PaniniProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::PaniniProjector* cv_detail_PaniniProjector_defaultNew_const() {
			cv::detail::PaniniProjector* ret = new cv::detail::PaniniProjector();
			return ret;
	}

	// cv::detail::PaniniProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
	// ("cv::detail::PaniniProjector::a", vec![(pred!(const, [], []), _)]),
	float cv_detail_PaniniProjector_propA_const(const cv::detail::PaniniProjector* instance) {
			float ret = instance->a;
			return ret;
	}

	// cv::detail::PaniniProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
	// ("cv::detail::PaniniProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_PaniniProjector_propA_const_float(cv::detail::PaniniProjector* instance, const float val) {
			instance->a = val;
	}

	// cv::detail::PaniniProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
	// ("cv::detail::PaniniProjector::b", vec![(pred!(const, [], []), _)]),
	float cv_detail_PaniniProjector_propB_const(const cv::detail::PaniniProjector* instance) {
			float ret = instance->b;
			return ret;
	}

	// cv::detail::PaniniProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
	// ("cv::detail::PaniniProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_PaniniProjector_propB_const_float(cv::detail::PaniniProjector* instance, const float val) {
			instance->b = val;
	}

	// cv::detail::PaniniProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::PaniniProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_PaniniProjector_to_Detail_ProjectorBase(cv::detail::PaniniProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::PaniniProjector::delete() generated
	// ("cv::detail::PaniniProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PaniniProjector_delete(cv::detail::PaniniProjector* instance) {
			delete instance;
	}

	// PaniniWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:434
	// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	void cv_detail_PaniniWarper_PaniniWarper_float_float_float(float scale, float A, float B, Result<cv::detail::PaniniWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniWarper::PaniniWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:434
	// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_PaniniWarper_PaniniWarper_float(float scale, Result<cv::detail::PaniniWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PaniniWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::PaniniWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_PaniniWarper_to_Detail_RotationWarper(cv::detail::PaniniWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::PaniniWarper::delete() generated
	// ("cv::detail::PaniniWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PaniniWarper_delete(cv::detail::PaniniWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:674
	// ("cv::detail::PlanePortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:675
	// ("cv::detail::PlanePortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlanePortraitProjector::defaultNew() generated
	// ("cv::detail::PlanePortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::PlanePortraitProjector* cv_detail_PlanePortraitProjector_defaultNew_const() {
			cv::detail::PlanePortraitProjector* ret = new cv::detail::PlanePortraitProjector();
			return ret;
	}

	// cv::detail::PlanePortraitProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::PlanePortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_PlanePortraitProjector_to_Detail_ProjectorBase(cv::detail::PlanePortraitProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::PlanePortraitProjector::delete() generated
	// ("cv::detail::PlanePortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlanePortraitProjector_delete(cv::detail::PlanePortraitProjector* instance) {
			delete instance;
	}

	// PlanePortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:682
	// ("cv::detail::PlanePortraitWarper::PlanePortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(float scale, Result<cv::detail::PlanePortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::PlanePortraitWarper* ret = new cv::detail::PlanePortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlanePortraitWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::PlanePortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_PlanePortraitWarper_to_Detail_RotationWarper(cv::detail::PlanePortraitWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::PlanePortraitWarper::delete() generated
	// ("cv::detail::PlanePortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlanePortraitWarper_delete(cv::detail::PlanePortraitWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:185
	// ("cv::detail::PlaneProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:186
	// ("cv::detail::PlaneProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlaneProjector::defaultNew() generated
	// ("cv::detail::PlaneProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::PlaneProjector* cv_detail_PlaneProjector_defaultNew_const() {
			cv::detail::PlaneProjector* ret = new cv::detail::PlaneProjector();
			return ret;
	}

	// cv::detail::PlaneProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::PlaneProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_PlaneProjector_to_Detail_ProjectorBase(cv::detail::PlaneProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::PlaneProjector::delete() generated
	// ("cv::detail::PlaneProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlaneProjector_delete(cv::detail::PlaneProjector* instance) {
			delete instance;
	}

	// PlaneWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:198
	// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_PlaneWarper_PlaneWarper_float(float scale, Result<cv::detail::PlaneWarper*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlaneWarper::PlaneWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:198
	// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlaneWarper_PlaneWarper(Result<cv::detail::PlaneWarper*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:200
	// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPoint(const Point2f &, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:201
	// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R", "T"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:203
	// ("cv::detail::PlaneWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPointBackward(const Point2f &, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:204
	// ("cv::detail::PlaneWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R", "T"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:206
	// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:207
	// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:209
	// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:211
	// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:214
	// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpRoi(Size, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:215
	// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R", "T"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlaneWarper::to_Detail_AffineWarper() generated
	// ("cv::detail::PlaneWarper::to_Detail_AffineWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineWarper* cv_detail_PlaneWarper_to_Detail_AffineWarper(cv::detail::PlaneWarper* instance) {
			return dynamic_cast<cv::detail::AffineWarper*>(instance);
	}

	// cv::detail::PlaneWarper::to_Detail_PlaneWarperGpu() generated
	// ("cv::detail::PlaneWarper::to_Detail_PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::detail::PlaneWarperGpu* cv_detail_PlaneWarper_to_Detail_PlaneWarperGpu(cv::detail::PlaneWarper* instance) {
			return dynamic_cast<cv::detail::PlaneWarperGpu*>(instance);
	}

	// cv::detail::PlaneWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::PlaneWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_PlaneWarper_to_Detail_RotationWarper(cv::detail::PlaneWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::PlaneWarper::delete() generated
	// ("cv::detail::PlaneWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlaneWarper_delete(cv::detail::PlaneWarper* instance) {
			delete instance;
	}

	// PlaneWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:496
	// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(float scale, Result<cv::detail::PlaneWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlaneWarperGpu::PlaneWarperGpu() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:496
	// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlaneWarperGpu_PlaneWarperGpu(Result<cv::detail::PlaneWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:503
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:511
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:519
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:528
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:540
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:542
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:544
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:547
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::PlaneWarperGpu::to_Detail_PlaneWarper() generated
	// ("cv::detail::PlaneWarperGpu::to_Detail_PlaneWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::PlaneWarper* cv_detail_PlaneWarperGpu_to_Detail_PlaneWarper(cv::detail::PlaneWarperGpu* instance) {
			return dynamic_cast<cv::detail::PlaneWarper*>(instance);
	}

	// cv::detail::PlaneWarperGpu::to_Detail_RotationWarper() generated
	// ("cv::detail::PlaneWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_PlaneWarperGpu_to_Detail_RotationWarper(cv::detail::PlaneWarperGpu* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::PlaneWarperGpu::delete() generated
	// ("cv::detail::PlaneWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_PlaneWarperGpu_delete(cv::detail::PlaneWarperGpu* instance) {
			delete instance;
	}

	// setCameraParams(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:135
	// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, ["K", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::ProjectorBase* instance, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraParams(*K, *R, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ProjectorBase::setCameraParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:135
	// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ProjectorBase_setCameraParams(cv::detail::ProjectorBase* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraParams();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ProjectorBase::implicitClone() generated
	// ("cv::detail::ProjectorBase::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_ProjectorBase_implicitClone_const(const cv::detail::ProjectorBase* instance) {
			return new cv::detail::ProjectorBase(*instance);
	}

	// cv::detail::ProjectorBase::defaultNew() generated
	// ("cv::detail::ProjectorBase::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_ProjectorBase_defaultNew_const() {
			cv::detail::ProjectorBase* ret = new cv::detail::ProjectorBase();
			return ret;
	}

	// cv::detail::ProjectorBase::scale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:139
	// ("cv::detail::ProjectorBase::scale", vec![(pred!(const, [], []), _)]),
	float cv_detail_ProjectorBase_propScale_const(const cv::detail::ProjectorBase* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::detail::ProjectorBase::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:139
	// ("cv::detail::ProjectorBase::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_detail_ProjectorBase_propScale_const_float(cv::detail::ProjectorBase* instance, const float val) {
			instance->scale = val;
	}

	// cv::detail::ProjectorBase::k() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:140
	// ("cv::detail::ProjectorBase::k", vec![(pred!(const, [], []), _)]),
	const float** cv_detail_ProjectorBase_propK_const(const cv::detail::ProjectorBase* instance) {
			const float(*ret)[9] = &instance->k;
			return (const float**)ret;
	}

	// cv::detail::ProjectorBase::kMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:140
	// ("cv::detail::ProjectorBase::kMut", vec![(pred!(mut, [], []), _)]),
	float** cv_detail_ProjectorBase_propK(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->k;
			return (float**)ret;
	}

	// cv::detail::ProjectorBase::rinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:141
	// ("cv::detail::ProjectorBase::rinv", vec![(pred!(const, [], []), _)]),
	const float** cv_detail_ProjectorBase_propRinv_const(const cv::detail::ProjectorBase* instance) {
			const float(*ret)[9] = &instance->rinv;
			return (const float**)ret;
	}

	// cv::detail::ProjectorBase::rinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:141
	// ("cv::detail::ProjectorBase::rinvMut", vec![(pred!(mut, [], []), _)]),
	float** cv_detail_ProjectorBase_propRinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->rinv;
			return (float**)ret;
	}

	// cv::detail::ProjectorBase::r_kinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:142
	// ("cv::detail::ProjectorBase::r_kinv", vec![(pred!(const, [], []), _)]),
	const float** cv_detail_ProjectorBase_propR_kinv_const(const cv::detail::ProjectorBase* instance) {
			const float(*ret)[9] = &instance->r_kinv;
			return (const float**)ret;
	}

	// cv::detail::ProjectorBase::r_kinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:142
	// ("cv::detail::ProjectorBase::r_kinvMut", vec![(pred!(mut, [], []), _)]),
	float** cv_detail_ProjectorBase_propR_kinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->r_kinv;
			return (float**)ret;
	}

	// cv::detail::ProjectorBase::k_rinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:143
	// ("cv::detail::ProjectorBase::k_rinv", vec![(pred!(const, [], []), _)]),
	const float** cv_detail_ProjectorBase_propK_rinv_const(const cv::detail::ProjectorBase* instance) {
			const float(*ret)[9] = &instance->k_rinv;
			return (const float**)ret;
	}

	// cv::detail::ProjectorBase::k_rinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:143
	// ("cv::detail::ProjectorBase::k_rinvMut", vec![(pred!(mut, [], []), _)]),
	float** cv_detail_ProjectorBase_propK_rinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->k_rinv;
			return (float**)ret;
	}

	// cv::detail::ProjectorBase::t() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:144
	// ("cv::detail::ProjectorBase::t", vec![(pred!(const, [], []), _)]),
	const float** cv_detail_ProjectorBase_propT_const(const cv::detail::ProjectorBase* instance) {
			const float(*ret)[3] = &instance->t;
			return (const float**)ret;
	}

	// cv::detail::ProjectorBase::tMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:144
	// ("cv::detail::ProjectorBase::tMut", vec![(pred!(mut, [], []), _)]),
	float** cv_detail_ProjectorBase_propT(cv::detail::ProjectorBase* instance) {
			float(*ret)[3] = &instance->t;
			return (float**)ret;
	}

	// cv::detail::ProjectorBase::delete() generated
	// ("cv::detail::ProjectorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ProjectorBase_delete(cv::detail::ProjectorBase* instance) {
			delete instance;
	}

	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:71
	// ("cv::detail::RotationWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:80
	// ("cv::detail::RotationWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_RotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:91
	// ("cv::detail::RotationWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:103
	// ("cv::detail::RotationWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpBackward(InputArray, InputArray, InputArray, int, int, Size, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:116
	// ("cv::detail::RotationWarper::warpBackward", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst_size", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::Size* dst_size, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:125
	// ("cv::detail::RotationWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:127
	// ("cv::detail::RotationWarper::getScale", vec![(pred!(const, [], []), _)]),
	void cv_detail_RotationWarper_getScale_const(const cv::detail::RotationWarper* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:128
	// ("cv::detail::RotationWarper::setScale", vec![(pred!(mut, ["unnamed"], ["float"]), _)]),
	void cv_detail_RotationWarper_setScale_float(cv::detail::RotationWarper* instance, float unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->setScale(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::RotationWarper::delete() generated
	// ("cv::detail::RotationWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_RotationWarper_delete(cv::detail::RotationWarper* instance) {
			delete instance;
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:69
	// ("cv::detail::SeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_SeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::SeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDefault(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:71
	// ("cv::detail::SeamFinder::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_detail_SeamFinder_createDefault_int(int type, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = cv::detail::SeamFinder::createDefault(type);
			Ok(new cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SeamFinder::to_Detail_DpSeamFinder() generated
	// ("cv::detail::SeamFinder::to_Detail_DpSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::DpSeamFinder* cv_detail_SeamFinder_to_Detail_DpSeamFinder(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::DpSeamFinder*>(instance);
	}

	// cv::detail::SeamFinder::to_Detail_GraphCutSeamFinder() generated
	// ("cv::detail::SeamFinder::to_Detail_GraphCutSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinder* cv_detail_SeamFinder_to_Detail_GraphCutSeamFinder(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::GraphCutSeamFinder*>(instance);
	}

	// cv::detail::SeamFinder::to_Detail_GraphCutSeamFinderGpu() generated
	// ("cv::detail::SeamFinder::to_Detail_GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinderGpu* cv_detail_SeamFinder_to_Detail_GraphCutSeamFinderGpu(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::GraphCutSeamFinderGpu*>(instance);
	}

	// cv::detail::SeamFinder::to_Detail_NoSeamFinder() generated
	// ("cv::detail::SeamFinder::to_Detail_NoSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoSeamFinder* cv_detail_SeamFinder_to_Detail_NoSeamFinder(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::NoSeamFinder*>(instance);
	}

	// cv::detail::SeamFinder::to_Detail_PairwiseSeamFinder() generated
	// ("cv::detail::SeamFinder::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::PairwiseSeamFinder* cv_detail_SeamFinder_to_Detail_PairwiseSeamFinder(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::PairwiseSeamFinder*>(instance);
	}

	// cv::detail::SeamFinder::to_Detail_VoronoiSeamFinder() generated
	// ("cv::detail::SeamFinder::to_Detail_VoronoiSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::VoronoiSeamFinder* cv_detail_SeamFinder_to_Detail_VoronoiSeamFinder(cv::detail::SeamFinder* instance) {
			return dynamic_cast<cv::detail::VoronoiSeamFinder*>(instance);
	}

	// cv::detail::SeamFinder::delete() generated
	// ("cv::detail::SeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SeamFinder_delete(cv::detail::SeamFinder* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:637
	// ("cv::detail::SphericalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:638
	// ("cv::detail::SphericalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SphericalPortraitProjector::defaultNew() generated
	// ("cv::detail::SphericalPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::SphericalPortraitProjector* cv_detail_SphericalPortraitProjector_defaultNew_const() {
			cv::detail::SphericalPortraitProjector* ret = new cv::detail::SphericalPortraitProjector();
			return ret;
	}

	// cv::detail::SphericalPortraitProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::SphericalPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_SphericalPortraitProjector_to_Detail_ProjectorBase(cv::detail::SphericalPortraitProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::SphericalPortraitProjector::delete() generated
	// ("cv::detail::SphericalPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SphericalPortraitProjector_delete(cv::detail::SphericalPortraitProjector* instance) {
			delete instance;
	}

	// SphericalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:647
	// ("cv::detail::SphericalPortraitWarper::SphericalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(float scale, Result<cv::detail::SphericalPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::SphericalPortraitWarper* ret = new cv::detail::SphericalPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SphericalPortraitWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::SphericalPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_SphericalPortraitWarper_to_Detail_RotationWarper(cv::detail::SphericalPortraitWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::SphericalPortraitWarper::delete() generated
	// ("cv::detail::SphericalPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SphericalPortraitWarper_delete(cv::detail::SphericalPortraitWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:296
	// ("cv::detail::SphericalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:297
	// ("cv::detail::SphericalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SphericalProjector::implicitClone() generated
	// ("cv::detail::SphericalProjector::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::SphericalProjector* cv_detail_SphericalProjector_implicitClone_const(const cv::detail::SphericalProjector* instance) {
			return new cv::detail::SphericalProjector(*instance);
	}

	// cv::detail::SphericalProjector::defaultNew() generated
	// ("cv::detail::SphericalProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::SphericalProjector* cv_detail_SphericalProjector_defaultNew_const() {
			cv::detail::SphericalProjector* ret = new cv::detail::SphericalProjector();
			return ret;
	}

	// cv::detail::SphericalProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::SphericalProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_SphericalProjector_to_Detail_ProjectorBase(cv::detail::SphericalProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::SphericalProjector::delete() generated
	// ("cv::detail::SphericalProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SphericalProjector_delete(cv::detail::SphericalProjector* instance) {
			delete instance;
	}

	// SphericalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:315
	// ("cv::detail::SphericalWarper::SphericalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_SphericalWarper_SphericalWarper_float(float scale, Result<cv::detail::SphericalWarper*>* ocvrs_return) {
		try {
			cv::detail::SphericalWarper* ret = new cv::detail::SphericalWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:317
	// ("cv::detail::SphericalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:318
	// ("cv::detail::SphericalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SphericalWarper::to_Detail_SphericalWarperGpu() generated
	// ("cv::detail::SphericalWarper::to_Detail_SphericalWarperGpu", vec![(pred!(mut, [], []), _)]),
	cv::detail::SphericalWarperGpu* cv_detail_SphericalWarper_to_Detail_SphericalWarperGpu(cv::detail::SphericalWarper* instance) {
			return dynamic_cast<cv::detail::SphericalWarperGpu*>(instance);
	}

	// cv::detail::SphericalWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::SphericalWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_SphericalWarper_to_Detail_RotationWarper(cv::detail::SphericalWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::SphericalWarper::delete() generated
	// ("cv::detail::SphericalWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SphericalWarper_delete(cv::detail::SphericalWarper* instance) {
			delete instance;
	}

	// SphericalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:558
	// ("cv::detail::SphericalWarperGpu::SphericalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(float scale, Result<cv::detail::SphericalWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::SphericalWarperGpu* ret = new cv::detail::SphericalWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:565
	// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:573
	// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:585
	// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:587
	// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	void cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::SphericalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::SphericalWarperGpu::to_Detail_RotationWarper() generated
	// ("cv::detail::SphericalWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_SphericalWarperGpu_to_Detail_RotationWarper(cv::detail::SphericalWarperGpu* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::SphericalWarperGpu::to_Detail_SphericalWarper() generated
	// ("cv::detail::SphericalWarperGpu::to_Detail_SphericalWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::SphericalWarper* cv_detail_SphericalWarperGpu_to_Detail_SphericalWarper(cv::detail::SphericalWarperGpu* instance) {
			return dynamic_cast<cv::detail::SphericalWarper*>(instance);
	}

	// cv::detail::SphericalWarperGpu::delete() generated
	// ("cv::detail::SphericalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_SphericalWarperGpu_delete(cv::detail::SphericalWarperGpu* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:368
	// ("cv::detail::StereographicProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:369
	// ("cv::detail::StereographicProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::StereographicProjector::defaultNew() generated
	// ("cv::detail::StereographicProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::StereographicProjector* cv_detail_StereographicProjector_defaultNew_const() {
			cv::detail::StereographicProjector* ret = new cv::detail::StereographicProjector();
			return ret;
	}

	// cv::detail::StereographicProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::StereographicProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_StereographicProjector_to_Detail_ProjectorBase(cv::detail::StereographicProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::StereographicProjector::delete() generated
	// ("cv::detail::StereographicProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_StereographicProjector_delete(cv::detail::StereographicProjector* instance) {
			delete instance;
	}

	// StereographicWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:376
	// ("cv::detail::StereographicWarper::StereographicWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_StereographicWarper_StereographicWarper_float(float scale, Result<cv::detail::StereographicWarper*>* ocvrs_return) {
		try {
			cv::detail::StereographicWarper* ret = new cv::detail::StereographicWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::StereographicWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::StereographicWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_StereographicWarper_to_Detail_RotationWarper(cv::detail::StereographicWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::StereographicWarper::delete() generated
	// ("cv::detail::StereographicWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_StereographicWarper_delete(cv::detail::StereographicWarper* instance) {
			delete instance;
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:481
	// ("cv::detail::TransverseMercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float x, float y, float* u, float* v, ResultVoid* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:482
	// ("cv::detail::TransverseMercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	void cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float u, float v, float* x, float* y, ResultVoid* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::TransverseMercatorProjector::defaultNew() generated
	// ("cv::detail::TransverseMercatorProjector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::TransverseMercatorProjector* cv_detail_TransverseMercatorProjector_defaultNew_const() {
			cv::detail::TransverseMercatorProjector* ret = new cv::detail::TransverseMercatorProjector();
			return ret;
	}

	// cv::detail::TransverseMercatorProjector::to_Detail_ProjectorBase() generated
	// ("cv::detail::TransverseMercatorProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
	cv::detail::ProjectorBase* cv_detail_TransverseMercatorProjector_to_Detail_ProjectorBase(cv::detail::TransverseMercatorProjector* instance) {
			return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}

	// cv::detail::TransverseMercatorProjector::delete() generated
	// ("cv::detail::TransverseMercatorProjector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_TransverseMercatorProjector_delete(cv::detail::TransverseMercatorProjector* instance) {
			delete instance;
	}

	// TransverseMercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:489
	// ("cv::detail::TransverseMercatorWarper::TransverseMercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(float scale, Result<cv::detail::TransverseMercatorWarper*>* ocvrs_return) {
		try {
			cv::detail::TransverseMercatorWarper* ret = new cv::detail::TransverseMercatorWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::TransverseMercatorWarper::to_Detail_RotationWarper() generated
	// ("cv::detail::TransverseMercatorWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
	cv::detail::RotationWarper* cv_detail_TransverseMercatorWarper_to_Detail_RotationWarper(cv::detail::TransverseMercatorWarper* instance) {
			return dynamic_cast<cv::detail::RotationWarper*>(instance);
	}

	// cv::detail::TransverseMercatorWarper::delete() generated
	// ("cv::detail::TransverseMercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_TransverseMercatorWarper_delete(cv::detail::TransverseMercatorWarper* instance) {
			delete instance;
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:111
	// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_VoronoiSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find(const std::vector<Size> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:113
	// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["size", "corners", "masks"], ["const std::vector<cv::Size>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	void cv_detail_VoronoiSeamFinder_find_const_vectorLSizeGR_const_vectorLPointGR_vectorLUMatGR(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::Size>* size, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->find(*size, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::VoronoiSeamFinder::defaultNew() generated
	// ("cv::detail::VoronoiSeamFinder::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::VoronoiSeamFinder* cv_detail_VoronoiSeamFinder_defaultNew_const() {
			cv::detail::VoronoiSeamFinder* ret = new cv::detail::VoronoiSeamFinder();
			return ret;
	}

	// cv::detail::VoronoiSeamFinder::to_Detail_PairwiseSeamFinder() generated
	// ("cv::detail::VoronoiSeamFinder::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::PairwiseSeamFinder* cv_detail_VoronoiSeamFinder_to_Detail_PairwiseSeamFinder(cv::detail::VoronoiSeamFinder* instance) {
			return dynamic_cast<cv::detail::PairwiseSeamFinder*>(instance);
	}

	// cv::detail::VoronoiSeamFinder::to_Detail_SeamFinder() generated
	// ("cv::detail::VoronoiSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::detail::SeamFinder* cv_detail_VoronoiSeamFinder_to_Detail_SeamFinder(cv::detail::VoronoiSeamFinder* instance) {
			return dynamic_cast<cv::detail::SeamFinder*>(instance);
	}

	// cv::detail::VoronoiSeamFinder::delete() generated
	// ("cv::detail::VoronoiSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_VoronoiSeamFinder_delete(cv::detail::VoronoiSeamFinder* instance) {
			delete instance;
	}

}
