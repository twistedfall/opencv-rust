// autoDetectWaveCorrectKind(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:342
// ("cv::detail::autoDetectWaveCorrectKind", vec![(pred!(mut, ["rmats"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_autoDetectWaveCorrectKind_const_vectorLMatGR(rmats: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_WaveCorrectKind>);
// cv::detail::computeImageFeatures(CppPassByVoidPtr, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:86
// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "image", "features"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "cv::detail::ImageFeatures*"]), _)]),
pub fn cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR(features_finder: *const c_void, image: *const c_void, features: *mut c_void, ocvrs_return: *mut Result<()>);
// computeImageFeatures(const Ptr<Feature2D> &, InputArray, ImageFeatures &, InputArray)(CppPassByVoidPtr, InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:86
// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "image", "features", "mask"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "cv::detail::ImageFeatures*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR_const__InputArrayR(features_finder: *const c_void, image: *const c_void, features: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::computeImageFeatures(CppPassByVoidPtr, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:73
// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "images", "features"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*"]), _)]),
pub fn cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR(features_finder: *const c_void, images: *const c_void, features: *mut c_void, ocvrs_return: *mut Result<()>);
// computeImageFeatures(const Ptr<Feature2D> &, InputArrayOfArrays, std::vector<ImageFeatures> &, InputArrayOfArrays)(CppPassByVoidPtr, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:73
// ("cv::detail::computeImageFeatures", vec![(pred!(mut, ["featuresFinder", "images", "features", "masks"], ["const cv::Ptr<cv::Feature2D>*", "const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR_const__InputArrayR(features_finder: *const c_void, images: *const c_void, features: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// createLaplacePyrGpu(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:173
// ("cv::detail::createLaplacePyrGpu", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vectorLUMatGR(img: *const c_void, num_levels: i32, pyr: *mut c_void, ocvrs_return: *mut Result<()>);
// createLaplacePyr(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:172
// ("cv::detail::createLaplacePyr", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_createLaplacePyr_const__InputArrayR_int_vectorLUMatGR(img: *const c_void, num_levels: i32, pyr: *mut c_void, ocvrs_return: *mut Result<()>);
// createWeightMap(InputArray, float, InputOutputArray)(InputArray, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:170
// ("cv::detail::createWeightMap", vec![(pred!(mut, ["mask", "sharpness", "weight"], ["const cv::_InputArray*", "float", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(mask: *const c_void, sharpness: f32, weight: *const c_void, ocvrs_return: *mut Result<()>);
// findMaxSpanningTree(int, const std::vector<MatchesInfo> &, Graph &, std::vector<int> &)(Primitive, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:364
// ("cv::detail::findMaxSpanningTree", vec![(pred!(mut, ["num_images", "pairwise_matches", "span_tree", "centers"], ["int", "const std::vector<cv::detail::MatchesInfo>*", "cv::detail::Graph*", "std::vector<int>*"]), _)]),
pub fn cv_detail_findMaxSpanningTree_int_const_vectorLMatchesInfoGR_GraphR_vectorLintGR(num_images: i32, pairwise_matches: *const c_void, span_tree: *mut c_void, centers: *mut c_void, ocvrs_return: *mut Result<()>);
// leaveBiggestComponent(std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:359
// ("cv::detail::leaveBiggestComponent", vec![(pred!(mut, ["features", "pairwise_matches", "conf_threshold"], ["std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
pub fn cv_detail_leaveBiggestComponent_vectorLImageFeaturesGR_vectorLMatchesInfoGR_float(features: *mut c_void, pairwise_matches: *mut c_void, conf_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// matchesGraphAsString(std::vector<String> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:356
// ("cv::detail::matchesGraphAsString", vec![(pred!(mut, ["paths", "pairwise_matches", "conf_threshold"], ["std::vector<cv::String>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
pub fn cv_detail_matchesGraphAsString_vectorLStringGR_vectorLMatchesInfoGR_float(paths: *mut c_void, pairwise_matches: *mut c_void, conf_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// normalizeUsingWeightMap(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:168
// ("cv::detail::normalizeUsingWeightMap", vec![(pred!(mut, ["weight", "src"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(weight: *const c_void, src: *const c_void, ocvrs_return: *mut Result<()>);
// overlapRoi(Point, Point, Size, Size, Rect &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:103
// ("cv::detail::overlapRoi", vec![(pred!(mut, ["tl1", "tl2", "sz1", "sz2", "roi"], ["cv::Point", "cv::Point", "cv::Size", "cv::Size", "cv::Rect*"]), _)]),
pub fn cv_detail_overlapRoi_Point_Point_Size_Size_RectR(tl1: *const core::Point, tl2: *const core::Point, sz1: *const core::Size, sz2: *const core::Size, roi: *mut core::Rect, ocvrs_return: *mut Result<bool>);
// restoreImageFromLaplacePyrGpu(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:177
// ("cv::detail::restoreImageFromLaplacePyrGpu", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_restoreImageFromLaplacePyrGpu_vectorLUMatGR(pyr: *mut c_void, ocvrs_return: *mut Result<()>);
// restoreImageFromLaplacePyr(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:176
// ("cv::detail::restoreImageFromLaplacePyr", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_restoreImageFromLaplacePyr_vectorLUMatGR(pyr: *mut c_void, ocvrs_return: *mut Result<()>);
// resultRoiIntersection(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:106
// ("cv::detail::resultRoiIntersection", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
pub fn cv_detail_resultRoiIntersection_const_vectorLPointGR_const_vectorLSizeGR(corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// resultRoi(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:105
// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
pub fn cv_detail_resultRoi_const_vectorLPointGR_const_vectorLSizeGR(corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// resultRoi(const std::vector<Point> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:104
// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "images"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_resultRoi_const_vectorLPointGR_const_vectorLUMatGR(corners: *const c_void, images: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// resultTl(const std::vector<Point> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:107
// ("cv::detail::resultTl", vec![(pred!(mut, ["corners"], ["const std::vector<cv::Point>*"]), _)]),
pub fn cv_detail_resultTl_const_vectorLPointGR(corners: *const c_void, ocvrs_return: *mut Result<core::Point>);
// selectRandomSubset(int, int, std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:110
// ("cv::detail::selectRandomSubset", vec![(pred!(mut, ["count", "size", "subset"], ["int", "int", "std::vector<int>*"]), _)]),
pub fn cv_detail_selectRandomSubset_int_int_vectorLintGR(count: i32, size: i32, subset: *mut c_void, ocvrs_return: *mut Result<()>);
// stitchingLogLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:112
// ("cv::detail::stitchingLogLevel", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_stitchingLogLevel(ocvrs_return: *mut Result<i32>);
// waveCorrect(std::vector<Mat> &, WaveCorrectKind)(CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:349
// ("cv::detail::waveCorrect", vec![(pred!(mut, ["rmats", "kind"], ["std::vector<cv::Mat>*", "cv::detail::WaveCorrectKind"]), _)]),
pub fn cv_detail_waveCorrect_vectorLMatGR_WaveCorrectKind(rmats: *mut c_void, kind: crate::stitching::Detail_WaveCorrectKind, ocvrs_return: *mut Result<()>);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:155
// ("cv::AffineWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_AffineWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::AffineWarper::defaultNew() generated
// ("cv::AffineWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_AffineWarper_defaultNew_const() -> *mut c_void;
// cv::AffineWarper::to_WarperCreator() generated
// ("cv::AffineWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::AffineWarper::delete() generated
// ("cv::AffineWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineWarper_delete(instance: *mut c_void);
// CompressedRectilinearPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:201
// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
pub fn cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:201
// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper(ocvrs_return: *mut Result<*mut c_void>);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:205
// ("cv::CompressedRectilinearPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_CompressedRectilinearPortraitWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CompressedRectilinearPortraitWarper::to_WarperCreator() generated
// ("cv::CompressedRectilinearPortraitWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearPortraitWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::CompressedRectilinearPortraitWarper::delete() generated
// ("cv::CompressedRectilinearPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearPortraitWarper_delete(instance: *mut c_void);
// CompressedRectilinearWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:190
// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
pub fn cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CompressedRectilinearWarper::CompressedRectilinearWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:190
// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearWarper_CompressedRectilinearWarper(ocvrs_return: *mut Result<*mut c_void>);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:194
// ("cv::CompressedRectilinearWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_CompressedRectilinearWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CompressedRectilinearWarper::to_WarperCreator() generated
// ("cv::CompressedRectilinearWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::CompressedRectilinearWarper::delete() generated
// ("cv::CompressedRectilinearWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CompressedRectilinearWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:164
// ("cv::CylindricalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_CylindricalWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CylindricalWarper::defaultNew() generated
// ("cv::CylindricalWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_CylindricalWarper_defaultNew_const() -> *mut c_void;
// cv::CylindricalWarper::to_WarperCreator() generated
// ("cv::CylindricalWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CylindricalWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::CylindricalWarper::delete() generated
// ("cv::CylindricalWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CylindricalWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:255
// ("cv::CylindricalWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_CylindricalWarperGpu_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CylindricalWarperGpu::defaultNew() generated
// ("cv::CylindricalWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_CylindricalWarperGpu_defaultNew_const() -> *mut c_void;
// cv::CylindricalWarperGpu::to_WarperCreator() generated
// ("cv::CylindricalWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_CylindricalWarperGpu_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::CylindricalWarperGpu::delete() generated
// ("cv::CylindricalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CylindricalWarperGpu_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:177
// ("cv::FisheyeWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_FisheyeWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FisheyeWarper::defaultNew() generated
// ("cv::FisheyeWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_FisheyeWarper_defaultNew_const() -> *mut c_void;
// cv::FisheyeWarper::to_WarperCreator() generated
// ("cv::FisheyeWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_FisheyeWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::FisheyeWarper::delete() generated
// ("cv::FisheyeWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FisheyeWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:233
// ("cv::MercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_MercatorWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MercatorWarper::defaultNew() generated
// ("cv::MercatorWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_MercatorWarper_defaultNew_const() -> *mut c_void;
// cv::MercatorWarper::to_WarperCreator() generated
// ("cv::MercatorWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_MercatorWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::MercatorWarper::delete() generated
// ("cv::MercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MercatorWarper_delete(instance: *mut c_void);
// PaniniPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:223
// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
pub fn cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PaniniPortraitWarper::PaniniPortraitWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:223
// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniPortraitWarper_PaniniPortraitWarper(ocvrs_return: *mut Result<*mut c_void>);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:227
// ("cv::PaniniPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_PaniniPortraitWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PaniniPortraitWarper::to_WarperCreator() generated
// ("cv::PaniniPortraitWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniPortraitWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::PaniniPortraitWarper::delete() generated
// ("cv::PaniniPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniPortraitWarper_delete(instance: *mut c_void);
// PaniniWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:212
// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
pub fn cv_PaniniWarper_PaniniWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PaniniWarper::PaniniWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:212
// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniWarper_PaniniWarper(ocvrs_return: *mut Result<*mut c_void>);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:216
// ("cv::PaniniWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_PaniniWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PaniniWarper::to_WarperCreator() generated
// ("cv::PaniniWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::PaniniWarper::delete() generated
// ("cv::PaniniWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PaniniWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:146
// ("cv::PlaneWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_PlaneWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PlaneWarper::defaultNew() generated
// ("cv::PlaneWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_PlaneWarper_defaultNew_const() -> *mut c_void;
// cv::PlaneWarper::to_WarperCreator() generated
// ("cv::PlaneWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_PlaneWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::PlaneWarper::delete() generated
// ("cv::PlaneWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PlaneWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:248
// ("cv::PlaneWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_PlaneWarperGpu_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PlaneWarperGpu::defaultNew() generated
// ("cv::PlaneWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_PlaneWarperGpu_defaultNew_const() -> *mut c_void;
// cv::PlaneWarperGpu::to_WarperCreator() generated
// ("cv::PlaneWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_PlaneWarperGpu_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::PlaneWarperGpu::delete() generated
// ("cv::PlaneWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PlaneWarperGpu_delete(instance: *mut c_void);
// PyRotationWarper(String, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:55
// ("cv::PyRotationWarper::PyRotationWarper", vec![(pred!(mut, ["type", "scale"], ["cv::String", "float"]), _)]),
pub fn cv_PyRotationWarper_PyRotationWarper_String_float(typ: *const c_char, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// PyRotationWarper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:56
// ("cv::PyRotationWarper::PyRotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_PyRotationWarper_PyRotationWarper(ocvrs_return: *mut Result<*mut c_void>);
// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:66
// ("cv::PyRotationWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_PyRotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:75
// ("cv::PyRotationWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_PyRotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:86
// ("cv::PyRotationWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PyRotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:98
// ("cv::PyRotationWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_PyRotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warpBackward(InputArray, InputArray, InputArray, int, int, Size, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:111
// ("cv::PyRotationWarper::warpBackward", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst_size", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Size", "const cv::_OutputArray*"]), _)]),
pub fn cv_PyRotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst_size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result<()>);
// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:120
// ("cv::PyRotationWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_PyRotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:122
// ("cv::PyRotationWarper::getScale", vec![(pred!(const, [], []), _)]),
pub fn cv_PyRotationWarper_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:123
// ("cv::PyRotationWarper::setScale", vec![(pred!(mut, ["unnamed"], ["float"]), _)]),
pub fn cv_PyRotationWarper_setScale_float(instance: *mut c_void, unnamed: f32, ocvrs_return: *mut Result<()>);
// cv::PyRotationWarper::delete() generated
// ("cv::PyRotationWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PyRotationWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:171
// ("cv::SphericalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_SphericalWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::SphericalWarper::defaultNew() generated
// ("cv::SphericalWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_SphericalWarper_defaultNew_const() -> *mut c_void;
// cv::SphericalWarper::to_WarperCreator() generated
// ("cv::SphericalWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_SphericalWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::SphericalWarper::delete() generated
// ("cv::SphericalWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SphericalWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:262
// ("cv::SphericalWarperGpu::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_SphericalWarperGpu_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::SphericalWarperGpu::defaultNew() generated
// ("cv::SphericalWarperGpu::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_SphericalWarperGpu_defaultNew_const() -> *mut c_void;
// cv::SphericalWarperGpu::to_WarperCreator() generated
// ("cv::SphericalWarperGpu::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_SphericalWarperGpu_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::SphericalWarperGpu::delete() generated
// ("cv::SphericalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SphericalWarperGpu_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:183
// ("cv::StereographicWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_StereographicWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::StereographicWarper::defaultNew() generated
// ("cv::StereographicWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_StereographicWarper_defaultNew_const() -> *mut c_void;
// cv::StereographicWarper::to_WarperCreator() generated
// ("cv::StereographicWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereographicWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::StereographicWarper::delete() generated
// ("cv::StereographicWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_StereographicWarper_delete(instance: *mut c_void);
// create(Mode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:184
// ("cv::Stitcher::create", vec![(pred!(mut, ["mode"], ["cv::Stitcher::Mode"]), _)]),
pub fn cv_Stitcher_create_Mode(mode: crate::stitching::Stitcher_Mode, ocvrs_return: *mut Result<*mut c_void>);
// cv::Stitcher::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:184
// ("cv::Stitcher::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_create(ocvrs_return: *mut Result<*mut c_void>);
// registrationResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:186
// ("cv::Stitcher::registrationResol", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_registrationResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRegistrationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:187
// ("cv::Stitcher::setRegistrationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
pub fn cv_Stitcher_setRegistrationResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result<()>);
// seamEstimationResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:189
// ("cv::Stitcher::seamEstimationResol", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_seamEstimationResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSeamEstimationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:190
// ("cv::Stitcher::setSeamEstimationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
pub fn cv_Stitcher_setSeamEstimationResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result<()>);
// compositingResol()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:192
// ("cv::Stitcher::compositingResol", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_compositingResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setCompositingResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:193
// ("cv::Stitcher::setCompositingResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
pub fn cv_Stitcher_setCompositingResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result<()>);
// panoConfidenceThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:195
// ("cv::Stitcher::panoConfidenceThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_panoConfidenceThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPanoConfidenceThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:196
// ("cv::Stitcher::setPanoConfidenceThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
pub fn cv_Stitcher_setPanoConfidenceThresh_double(instance: *mut c_void, conf_thresh: f64, ocvrs_return: *mut Result<()>);
// waveCorrection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:198
// ("cv::Stitcher::waveCorrection", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_waveCorrection_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setWaveCorrection(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:199
// ("cv::Stitcher::setWaveCorrection", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_Stitcher_setWaveCorrection_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// interpolationFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:201
// ("cv::Stitcher::interpolationFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_interpolationFlags_const(instance: *const c_void, ocvrs_return: *mut Result<crate::imgproc::InterpolationFlags>);
// setInterpolationFlags(InterpolationFlags)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:202
// ("cv::Stitcher::setInterpolationFlags", vec![(pred!(mut, ["interp_flags"], ["cv::InterpolationFlags"]), _)]),
pub fn cv_Stitcher_setInterpolationFlags_InterpolationFlags(instance: *mut c_void, interp_flags: crate::imgproc::InterpolationFlags, ocvrs_return: *mut Result<()>);
// waveCorrectKind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:204
// ("cv::Stitcher::waveCorrectKind", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_waveCorrectKind_const(instance: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_WaveCorrectKind>);
// setWaveCorrectKind(detail::WaveCorrectKind)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:205
// ("cv::Stitcher::setWaveCorrectKind", vec![(pred!(mut, ["kind"], ["cv::detail::WaveCorrectKind"]), _)]),
pub fn cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(instance: *mut c_void, kind: crate::stitching::Detail_WaveCorrectKind, ocvrs_return: *mut Result<()>);
// featuresFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:207
// ("cv::Stitcher::featuresFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_featuresFinder(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// featuresFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:208
// ("cv::Stitcher::featuresFinder", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_featuresFinder_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setFeaturesFinder(Ptr<Feature2D>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:209
// ("cv::Stitcher::setFeaturesFinder", vec![(pred!(mut, ["features_finder"], ["cv::Ptr<cv::Feature2D>"]), _)]),
pub fn cv_Stitcher_setFeaturesFinder_PtrLFeature2DG(instance: *mut c_void, features_finder: *mut c_void, ocvrs_return: *mut Result<()>);
// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:212
// ("cv::Stitcher::featuresMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_featuresMatcher(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:213
// ("cv::Stitcher::featuresMatcher", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_featuresMatcher_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setFeaturesMatcher(Ptr<detail::FeaturesMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:214
// ("cv::Stitcher::setFeaturesMatcher", vec![(pred!(mut, ["features_matcher"], ["cv::Ptr<cv::detail::FeaturesMatcher>"]), _)]),
pub fn cv_Stitcher_setFeaturesMatcher_PtrLFeaturesMatcherG(instance: *mut c_void, features_matcher: *mut c_void, ocvrs_return: *mut Result<()>);
// matchingMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:217
// ("cv::Stitcher::matchingMask", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_matchingMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMatchingMask(const cv::UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:218
// ("cv::Stitcher::setMatchingMask", vec![(pred!(mut, ["mask"], ["const cv::UMat*"]), _)]),
pub fn cv_Stitcher_setMatchingMask_const_UMatR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:224
// ("cv::Stitcher::bundleAdjuster", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_bundleAdjuster(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:225
// ("cv::Stitcher::bundleAdjuster", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_bundleAdjuster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setBundleAdjuster(Ptr<detail::BundleAdjusterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:226
// ("cv::Stitcher::setBundleAdjuster", vec![(pred!(mut, ["bundle_adjuster"], ["cv::Ptr<cv::detail::BundleAdjusterBase>"]), _)]),
pub fn cv_Stitcher_setBundleAdjuster_PtrLBundleAdjusterBaseG(instance: *mut c_void, bundle_adjuster: *mut c_void, ocvrs_return: *mut Result<()>);
// estimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:229
// ("cv::Stitcher::estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_estimator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:230
// ("cv::Stitcher::estimator", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_estimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setEstimator(Ptr<detail::Estimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:231
// ("cv::Stitcher::setEstimator", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::detail::Estimator>"]), _)]),
pub fn cv_Stitcher_setEstimator_PtrLEstimatorG(instance: *mut c_void, estimator: *mut c_void, ocvrs_return: *mut Result<()>);
// warper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:234
// ("cv::Stitcher::warper", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_warper(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// warper()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:235
// ("cv::Stitcher::warper", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_warper_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setWarper(Ptr<WarperCreator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:236
// ("cv::Stitcher::setWarper", vec![(pred!(mut, ["creator"], ["cv::Ptr<cv::WarperCreator>"]), _)]),
pub fn cv_Stitcher_setWarper_PtrLWarperCreatorG(instance: *mut c_void, creator: *mut c_void, ocvrs_return: *mut Result<()>);
// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:238
// ("cv::Stitcher::exposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_exposureCompensator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:239
// ("cv::Stitcher::exposureCompensator", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_exposureCompensator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setExposureCompensator(Ptr<detail::ExposureCompensator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:240
// ("cv::Stitcher::setExposureCompensator", vec![(pred!(mut, ["exposure_comp"], ["cv::Ptr<cv::detail::ExposureCompensator>"]), _)]),
pub fn cv_Stitcher_setExposureCompensator_PtrLExposureCompensatorG(instance: *mut c_void, exposure_comp: *mut c_void, ocvrs_return: *mut Result<()>);
// seamFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:243
// ("cv::Stitcher::seamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_seamFinder(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// seamFinder()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:244
// ("cv::Stitcher::seamFinder", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_seamFinder_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setSeamFinder(Ptr<detail::SeamFinder>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:245
// ("cv::Stitcher::setSeamFinder", vec![(pred!(mut, ["seam_finder"], ["cv::Ptr<cv::detail::SeamFinder>"]), _)]),
pub fn cv_Stitcher_setSeamFinder_PtrLSeamFinderG(instance: *mut c_void, seam_finder: *mut c_void, ocvrs_return: *mut Result<()>);
// blender()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:247
// ("cv::Stitcher::blender", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_blender(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// blender()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:248
// ("cv::Stitcher::blender", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_blender_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setBlender(Ptr<detail::Blender>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:249
// ("cv::Stitcher::setBlender", vec![(pred!(mut, ["b"], ["cv::Ptr<cv::detail::Blender>"]), _)]),
pub fn cv_Stitcher_setBlender_PtrLBlenderG(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result<()>);
// estimateTransform(InputArrayOfArrays, InputArrayOfArrays)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:260
// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images", "masks"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Stitcher_estimateTransform_const__InputArrayR_const__InputArrayR(instance: *mut c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// cv::Stitcher::estimateTransform(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:260
// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
pub fn cv_Stitcher_estimateTransform_const__InputArrayR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// setTransform(InputArrayOfArrays, const std::vector<detail::CameraParams> &, const std::vector<int> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:270
// ("cv::Stitcher::setTransform", vec![(pred!(mut, ["images", "cameras", "component"], ["const cv::_InputArray*", "const std::vector<cv::detail::CameraParams>*", "const std::vector<int>*"]), _)]),
pub fn cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR_const_vectorLintGR(instance: *mut c_void, images: *const c_void, cameras: *const c_void, component: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// setTransform(InputArrayOfArrays, const std::vector<detail::CameraParams> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:274
// ("cv::Stitcher::setTransform", vec![(pred!(mut, ["images", "cameras"], ["const cv::_InputArray*", "const std::vector<cv::detail::CameraParams>*"]), _)]),
pub fn cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR(instance: *mut c_void, images: *const c_void, cameras: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// composePanorama(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:277
// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["pano"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_Stitcher_composePanorama_const__OutputArrayR(instance: *mut c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// composePanorama(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:289
// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// stitch(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:292
// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// stitch(InputArrayOfArrays, InputArrayOfArrays, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:300
// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "masks", "pano"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, masks: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
// component()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:304
// ("cv::Stitcher::component", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_component_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cameras()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:308
// ("cv::Stitcher::cameras", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_cameras_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// workScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:309
// ("cv::Stitcher::workScale", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_workScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// resultMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching.hpp:318
// ("cv::Stitcher::resultMask", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_resultMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Stitcher::defaultNew() generated
// ("cv::Stitcher::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_Stitcher_defaultNew_const() -> *mut c_void;
// cv::Stitcher::delete() generated
// ("cv::Stitcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Stitcher_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:239
// ("cv::TransverseMercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_TransverseMercatorWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::TransverseMercatorWarper::defaultNew() generated
// ("cv::TransverseMercatorWarper::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_TransverseMercatorWarper_defaultNew_const() -> *mut c_void;
// cv::TransverseMercatorWarper::to_WarperCreator() generated
// ("cv::TransverseMercatorWarper::to_WarperCreator", vec![(pred!(mut, [], []), _)]),
pub fn cv_TransverseMercatorWarper_to_WarperCreator(instance: *mut c_void) -> *mut c_void;
// cv::TransverseMercatorWarper::delete() generated
// ("cv::TransverseMercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TransverseMercatorWarper_delete(instance: *mut c_void);
// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/warpers.hpp:136
// ("cv::WarperCreator::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
pub fn cv_WarperCreator_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::WarperCreator::to_AffineWarper() generated
// ("cv::WarperCreator::to_AffineWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_AffineWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_CompressedRectilinearPortraitWarper() generated
// ("cv::WarperCreator::to_CompressedRectilinearPortraitWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_CompressedRectilinearPortraitWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_CompressedRectilinearWarper() generated
// ("cv::WarperCreator::to_CompressedRectilinearWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_CompressedRectilinearWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_CylindricalWarper() generated
// ("cv::WarperCreator::to_CylindricalWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_CylindricalWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_CylindricalWarperGpu() generated
// ("cv::WarperCreator::to_CylindricalWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_CylindricalWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_FisheyeWarper() generated
// ("cv::WarperCreator::to_FisheyeWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_FisheyeWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_MercatorWarper() generated
// ("cv::WarperCreator::to_MercatorWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_MercatorWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_PaniniPortraitWarper() generated
// ("cv::WarperCreator::to_PaniniPortraitWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_PaniniPortraitWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_PaniniWarper() generated
// ("cv::WarperCreator::to_PaniniWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_PaniniWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_PlaneWarper() generated
// ("cv::WarperCreator::to_PlaneWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_PlaneWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_PlaneWarperGpu() generated
// ("cv::WarperCreator::to_PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_PlaneWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_SphericalWarper() generated
// ("cv::WarperCreator::to_SphericalWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_SphericalWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_SphericalWarperGpu() generated
// ("cv::WarperCreator::to_SphericalWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_SphericalWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_StereographicWarper() generated
// ("cv::WarperCreator::to_StereographicWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_StereographicWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::to_TransverseMercatorWarper() generated
// ("cv::WarperCreator::to_TransverseMercatorWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_to_TransverseMercatorWarper(instance: *mut c_void) -> *mut c_void;
// cv::WarperCreator::delete() generated
// ("cv::WarperCreator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_WarperCreator_delete(instance: *mut c_void);
// AffineBasedEstimator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:124
// ("cv::detail::AffineBasedEstimator::AffineBasedEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBasedEstimator_AffineBasedEstimator(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::AffineBasedEstimator::to_Detail_Estimator() generated
// ("cv::detail::AffineBasedEstimator::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBasedEstimator_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::AffineBasedEstimator::delete() generated
// ("cv::detail::AffineBasedEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBasedEstimator_delete(instance: *mut c_void);
// AffineBestOf2NearestMatcher(bool, bool, float, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:251
// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, ["full_affine", "try_use_gpu", "match_conf", "num_matches_thresh1"], ["bool", "bool", "float", "int"]), _)]),
pub fn cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(full_affine: bool, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:251
// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::AffineBestOf2NearestMatcher::to_Detail_BestOf2NearestMatcher() generated
// ("cv::detail::AffineBestOf2NearestMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBestOf2NearestMatcher_to_Detail_BestOf2NearestMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::AffineBestOf2NearestMatcher::to_Detail_FeaturesMatcher() generated
// ("cv::detail::AffineBestOf2NearestMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBestOf2NearestMatcher_to_Detail_FeaturesMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::AffineBestOf2NearestMatcher::delete() generated
// ("cv::detail::AffineBestOf2NearestMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineBestOf2NearestMatcher_delete(instance: *mut c_void);
// AffineWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:234
// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_AffineWarper_AffineWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::AffineWarper::AffineWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:234
// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineWarper_AffineWarper(ocvrs_return: *mut Result<*mut c_void>);
// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:243
// ("cv::detail::AffineWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "H"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:252
// ("cv::detail::AffineWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "H"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_AffineWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:263
// ("cv::detail::AffineWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "H", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, h: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:275
// ("cv::detail::AffineWarper::warp", vec![(pred!(mut, ["src", "K", "H", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, h: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:284
// ("cv::detail::AffineWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "H"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::detail::AffineWarper::to_Detail_PlaneWarper() generated
// ("cv::detail::AffineWarper::to_Detail_PlaneWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineWarper_to_Detail_PlaneWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::AffineWarper::to_Detail_RotationWarper() generated
// ("cv::detail::AffineWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::AffineWarper::delete() generated
// ("cv::detail::AffineWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_AffineWarper_delete(instance: *mut c_void);
// BestOf2NearestMatcher(bool, float, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:196
// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, ["try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2", "matches_confidence_thresh"], ["bool", "float", "int", "int", "double"]), _)]),
pub fn cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int_double(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, matches_confidence_thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:196
// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher(ocvrs_return: *mut Result<*mut c_void>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:199
// ("cv::detail::BestOf2NearestMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(bool, float, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:200
// ("cv::detail::BestOf2NearestMatcher::create", vec![(pred!(mut, ["try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2", "matches_confidence_thresh"], ["bool", "float", "int", "int", "double"]), _)]),
pub fn cv_detail_BestOf2NearestMatcher_create_bool_float_int_int_double(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, matches_confidence_thresh: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BestOf2NearestMatcher::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:200
// ("cv::detail::BestOf2NearestMatcher::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BestOf2NearestMatcher::to_Detail_AffineBestOf2NearestMatcher() generated
// ("cv::detail::BestOf2NearestMatcher::to_Detail_AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_to_Detail_AffineBestOf2NearestMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::BestOf2NearestMatcher::to_Detail_BestOf2NearestRangeMatcher() generated
// ("cv::detail::BestOf2NearestMatcher::to_Detail_BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_to_Detail_BestOf2NearestRangeMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::BestOf2NearestMatcher::to_Detail_FeaturesMatcher() generated
// ("cv::detail::BestOf2NearestMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_to_Detail_FeaturesMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::BestOf2NearestMatcher::delete() generated
// ("cv::detail::BestOf2NearestMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestMatcher_delete(instance: *mut c_void);
// BestOf2NearestRangeMatcher(int, bool, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:215
// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, ["range_width", "try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2"], ["int", "bool", "float", "int", "int"]), _)]),
pub fn cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(range_width: i32, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:215
// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BestOf2NearestRangeMatcher::to_Detail_BestOf2NearestMatcher() generated
// ("cv::detail::BestOf2NearestRangeMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestRangeMatcher_to_Detail_BestOf2NearestMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::BestOf2NearestRangeMatcher::to_Detail_FeaturesMatcher() generated
// ("cv::detail::BestOf2NearestRangeMatcher::to_Detail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestRangeMatcher_to_Detail_FeaturesMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::BestOf2NearestRangeMatcher::delete() generated
// ("cv::detail::BestOf2NearestRangeMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BestOf2NearestRangeMatcher_delete(instance: *mut c_void);
// createDefault(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:69
// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type", "try_gpu"], ["int", "bool"]), _)]),
pub fn cv_detail_Blender_createDefault_int_bool(typ: i32, try_gpu: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::Blender::createDefault(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:69
// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_detail_Blender_createDefault_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// prepare(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:76
// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
pub fn cv_detail_Blender_prepare_const_vectorLPointGR_const_vectorLSizeGR(instance: *mut c_void, corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<()>);
// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:78
// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
pub fn cv_detail_Blender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result<()>);
// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:85
// ("cv::detail::Blender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
pub fn cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result<()>);
// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:91
// ("cv::detail::Blender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::Blender::defaultNew() generated
// ("cv::detail::Blender::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_Blender_defaultNew_const() -> *mut c_void;
// cv::detail::Blender::to_Detail_FeatherBlender() generated
// ("cv::detail::Blender::to_Detail_FeatherBlender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Blender_to_Detail_FeatherBlender(instance: *mut c_void) -> *mut c_void;
// cv::detail::Blender::to_Detail_MultiBandBlender() generated
// ("cv::detail::Blender::to_Detail_MultiBandBlender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Blender_to_Detail_MultiBandBlender(instance: *mut c_void) -> *mut c_void;
// cv::detail::Blender::delete() generated
// ("cv::detail::Blender::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Blender_delete(instance: *mut c_void);
// BlocksChannelsCompensator(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:235
// ("cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator", vec![(pred!(mut, ["bl_width", "bl_height", "nr_feeds"], ["int", "int", "int"]), _)]),
pub fn cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(bl_width: i32, bl_height: i32, nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:235
// ("cv::detail::BlocksChannelsCompensator::BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator(ocvrs_return: *mut Result<*mut c_void>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:238
// ("cv::detail::BlocksChannelsCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_BlocksChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::BlocksChannelsCompensator::to_Detail_BlocksCompensator() generated
// ("cv::detail::BlocksChannelsCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksChannelsCompensator_to_Detail_BlocksCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksChannelsCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::BlocksChannelsCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksChannelsCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksChannelsCompensator::delete() generated
// ("cv::detail::BlocksChannelsCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksChannelsCompensator_delete(instance: *mut c_void);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:176
// ("cv::detail::BlocksCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:177
// ("cv::detail::BlocksCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_BlocksCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:178
// ("cv::detail::BlocksCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_BlocksCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:179
// ("cv::detail::BlocksCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
pub fn cv_detail_BlocksCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result<()>);
// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:180
// ("cv::detail::BlocksCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:181
// ("cv::detail::BlocksCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
pub fn cv_detail_BlocksCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result<()>);
// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:182
// ("cv::detail::BlocksCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_BlocksCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBlockSize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:183
// ("cv::detail::BlocksCompensator::setBlockSize", vec![(pred!(mut, ["width", "height"], ["int", "int"]), _)]),
pub fn cv_detail_BlocksCompensator_setBlockSize_int_int(instance: *mut c_void, width: i32, height: i32, ocvrs_return: *mut Result<()>);
// setBlockSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:184
// ("cv::detail::BlocksCompensator::setBlockSize", vec![(pred!(mut, ["size"], ["cv::Size"]), _)]),
pub fn cv_detail_BlocksCompensator_setBlockSize_Size(instance: *mut c_void, size: *const core::Size, ocvrs_return: *mut Result<()>);
// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:185
// ("cv::detail::BlocksCompensator::getBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_BlocksCompensator_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setNrGainsFilteringIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:186
// ("cv::detail::BlocksCompensator::setNrGainsFilteringIterations", vec![(pred!(mut, ["nr_iterations"], ["int"]), _)]),
pub fn cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(instance: *mut c_void, nr_iterations: i32, ocvrs_return: *mut Result<()>);
// getNrGainsFilteringIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:187
// ("cv::detail::BlocksCompensator::getNrGainsFilteringIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::detail::BlocksCompensator::to_Detail_BlocksChannelsCompensator() generated
// ("cv::detail::BlocksCompensator::to_Detail_BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksCompensator_to_Detail_BlocksChannelsCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksCompensator::to_Detail_BlocksGainCompensator() generated
// ("cv::detail::BlocksCompensator::to_Detail_BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksCompensator_to_Detail_BlocksGainCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::BlocksCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksCompensator::delete() generated
// ("cv::detail::BlocksCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksCompensator_delete(instance: *mut c_void);
// BlocksGainCompensator(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:212
// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, ["bl_width", "bl_height"], ["int", "int"]), _)]),
pub fn cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(bl_width: i32, bl_height: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BlocksGainCompensator::BlocksGainCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:212
// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksGainCompensator_BlocksGainCompensator(ocvrs_return: *mut Result<*mut c_void>);
// BlocksGainCompensator(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:214
// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, ["bl_width", "bl_height", "nr_feeds"], ["int", "int", "int"]), _)]),
pub fn cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(bl_width: i32, bl_height: i32, nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:217
// ("cv::detail::BlocksGainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_BlocksGainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:221
// ("cv::detail::BlocksGainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:224
// ("cv::detail::BlocksGainCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_BlocksGainCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:226
// ("cv::detail::BlocksGainCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_BlocksGainCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::BlocksGainCompensator::to_Detail_BlocksCompensator() generated
// ("cv::detail::BlocksGainCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksGainCompensator_to_Detail_BlocksCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksGainCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::BlocksGainCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksGainCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BlocksGainCompensator::delete() generated
// ("cv::detail::BlocksGainCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BlocksGainCompensator_delete(instance: *mut c_void);
// BundleAdjusterAffine()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:292
// ("cv::detail::BundleAdjusterAffine::BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffine_BundleAdjusterAffine(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BundleAdjusterAffine::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::BundleAdjusterAffine::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffine_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterAffine::to_Detail_Estimator() generated
// ("cv::detail::BundleAdjusterAffine::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffine_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterAffine::delete() generated
// ("cv::detail::BundleAdjusterAffine::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffine_delete(instance: *mut c_void);
// BundleAdjusterAffinePartial()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:316
// ("cv::detail::BundleAdjusterAffinePartial::BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BundleAdjusterAffinePartial::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::BundleAdjusterAffinePartial::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffinePartial_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterAffinePartial::to_Detail_Estimator() generated
// ("cv::detail::BundleAdjusterAffinePartial::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffinePartial_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterAffinePartial::delete() generated
// ("cv::detail::BundleAdjusterAffinePartial::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterAffinePartial_delete(instance: *mut c_void);
// refinementMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:136
// ("cv::detail::BundleAdjusterBase::refinementMask", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_refinementMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setRefinementMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:137
// ("cv::detail::BundleAdjusterBase::setRefinementMask", vec![(pred!(mut, ["mask"], ["const cv::Mat*"]), _)]),
pub fn cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// confThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:143
// ("cv::detail::BundleAdjusterBase::confThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_confThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setConfThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:144
// ("cv::detail::BundleAdjusterBase::setConfThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
pub fn cv_detail_BundleAdjusterBase_setConfThresh_double(instance: *mut c_void, conf_thresh: f64, ocvrs_return: *mut Result<()>);
// termCriteria()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:146
// ("cv::detail::BundleAdjusterBase::termCriteria", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_termCriteria(instance: *mut c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:147
// ("cv::detail::BundleAdjusterBase::setTermCriteria", vec![(pred!(mut, ["term_criteria"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, term_criteria: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffine() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffine(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffinePartial() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffinePartial(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterRay() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterRay(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterReproj() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterReproj(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::to_Detail_NoBundleAdjuster() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_NoBundleAdjuster(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::to_Detail_Estimator() generated
// ("cv::detail::BundleAdjusterBase::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterBase::delete() generated
// ("cv::detail::BundleAdjusterBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterBase_delete(instance: *mut c_void);
// BundleAdjusterRay()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:268
// ("cv::detail::BundleAdjusterRay::BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterRay_BundleAdjusterRay(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BundleAdjusterRay::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::BundleAdjusterRay::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterRay_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterRay::to_Detail_Estimator() generated
// ("cv::detail::BundleAdjusterRay::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterRay_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterRay::delete() generated
// ("cv::detail::BundleAdjusterRay::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterRay_delete(instance: *mut c_void);
// BundleAdjusterReproj()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:248
// ("cv::detail::BundleAdjusterReproj::BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterReproj_BundleAdjusterReproj(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::BundleAdjusterReproj::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::BundleAdjusterReproj::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterReproj_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterReproj::to_Detail_Estimator() generated
// ("cv::detail::BundleAdjusterReproj::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterReproj_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::BundleAdjusterReproj::delete() generated
// ("cv::detail::BundleAdjusterReproj::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_BundleAdjusterReproj_delete(instance: *mut c_void);
// CameraParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:60
// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CameraParams_CameraParams(ocvrs_return: *mut Result<*mut c_void>);
// CameraParams(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:61
// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
pub fn cv_detail_CameraParams_CameraParams_const_CameraParamsR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:62
// ("cv::detail::CameraParams::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
pub fn cv_detail_CameraParams_operatorST_const_CameraParamsR(instance: *mut c_void, other: *const c_void, ocvrs_return: *mut Result<()>);
// K()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:63
// ("cv::detail::CameraParams::K", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_K_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CameraParams::implicitClone() generated
// ("cv::detail::CameraParams::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CameraParams::focal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:65
// ("cv::detail::CameraParams::focal", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propFocal_const(instance: *const c_void) -> f64;
// cv::detail::CameraParams::setFocal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:65
// ("cv::detail::CameraParams::setFocal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_detail_CameraParams_propFocal_const_double(instance: *mut c_void, val: f64);
// cv::detail::CameraParams::aspect() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:66
// ("cv::detail::CameraParams::aspect", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propAspect_const(instance: *const c_void) -> f64;
// cv::detail::CameraParams::setAspect(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:66
// ("cv::detail::CameraParams::setAspect", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_detail_CameraParams_propAspect_const_double(instance: *mut c_void, val: f64);
// cv::detail::CameraParams::ppx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:67
// ("cv::detail::CameraParams::ppx", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propPpx_const(instance: *const c_void) -> f64;
// cv::detail::CameraParams::setPpx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:67
// ("cv::detail::CameraParams::setPpx", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_detail_CameraParams_propPpx_const_double(instance: *mut c_void, val: f64);
// cv::detail::CameraParams::ppy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:68
// ("cv::detail::CameraParams::ppy", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propPpy_const(instance: *const c_void) -> f64;
// cv::detail::CameraParams::setPpy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:68
// ("cv::detail::CameraParams::setPpy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_detail_CameraParams_propPpy_const_double(instance: *mut c_void, val: f64);
// cv::detail::CameraParams::R() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:69
// ("cv::detail::CameraParams::R", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propR_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CameraParams::setR(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:69
// ("cv::detail::CameraParams::setR", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_detail_CameraParams_propR_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::detail::CameraParams::t() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:70
// ("cv::detail::CameraParams::t", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CameraParams_propT_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CameraParams::setT(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/camera.hpp:70
// ("cv::detail::CameraParams::setT", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_detail_CameraParams_propT_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::detail::CameraParams::delete() generated
// ("cv::detail::CameraParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CameraParams_delete(instance: *mut c_void);
// ChannelsCompensator(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:149
// ("cv::detail::ChannelsCompensator::ChannelsCompensator", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
pub fn cv_detail_ChannelsCompensator_ChannelsCompensator_int(nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::ChannelsCompensator::ChannelsCompensator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:149
// ("cv::detail::ChannelsCompensator::ChannelsCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_ChannelsCompensator(ocvrs_return: *mut Result<*mut c_void>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:151
// ("cv::detail::ChannelsCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_ChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:153
// ("cv::detail::ChannelsCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:154
// ("cv::detail::ChannelsCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_ChannelsCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:155
// ("cv::detail::ChannelsCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_ChannelsCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:156
// ("cv::detail::ChannelsCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
pub fn cv_detail_ChannelsCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result<()>);
// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:157
// ("cv::detail::ChannelsCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:158
// ("cv::detail::ChannelsCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
pub fn cv_detail_ChannelsCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result<()>);
// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:159
// ("cv::detail::ChannelsCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// gains()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:160
// ("cv::detail::ChannelsCompensator::gains", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_gains_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::ChannelsCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::ChannelsCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ChannelsCompensator::delete() generated
// ("cv::detail::ChannelsCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ChannelsCompensator_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:405
// ("cv::detail::CompressedRectilinearPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:406
// ("cv::detail::CompressedRectilinearPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::CompressedRectilinearPortraitProjector::defaultNew() generated
// ("cv::detail::CompressedRectilinearPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_defaultNew_const() -> *mut c_void;
// cv::detail::CompressedRectilinearPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
// ("cv::detail::CompressedRectilinearPortraitProjector::a", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_propA_const(instance: *const c_void) -> f32;
// cv::detail::CompressedRectilinearPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
// ("cv::detail::CompressedRectilinearPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_propA_const_float(instance: *mut c_void, val: f32);
// cv::detail::CompressedRectilinearPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
// ("cv::detail::CompressedRectilinearPortraitProjector::b", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_propB_const(instance: *const c_void) -> f32;
// cv::detail::CompressedRectilinearPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:403
// ("cv::detail::CompressedRectilinearPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_propB_const_float(instance: *mut c_void, val: f32);
// cv::detail::CompressedRectilinearPortraitProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::CompressedRectilinearPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::CompressedRectilinearPortraitProjector::delete() generated
// ("cv::detail::CompressedRectilinearPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitProjector_delete(instance: *mut c_void);
// CompressedRectilinearPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:413
// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:413
// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CompressedRectilinearPortraitWarper::to_Detail_RotationWarper() generated
// ("cv::detail::CompressedRectilinearPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CompressedRectilinearPortraitWarper::delete() generated
// ("cv::detail::CompressedRectilinearPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearPortraitWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:384
// ("cv::detail::CompressedRectilinearProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:385
// ("cv::detail::CompressedRectilinearProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::CompressedRectilinearProjector::defaultNew() generated
// ("cv::detail::CompressedRectilinearProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearProjector_defaultNew_const() -> *mut c_void;
// cv::detail::CompressedRectilinearProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
// ("cv::detail::CompressedRectilinearProjector::a", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearProjector_propA_const(instance: *const c_void) -> f32;
// cv::detail::CompressedRectilinearProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
// ("cv::detail::CompressedRectilinearProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_CompressedRectilinearProjector_propA_const_float(instance: *mut c_void, val: f32);
// cv::detail::CompressedRectilinearProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
// ("cv::detail::CompressedRectilinearProjector::b", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CompressedRectilinearProjector_propB_const(instance: *const c_void) -> f32;
// cv::detail::CompressedRectilinearProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:382
// ("cv::detail::CompressedRectilinearProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_CompressedRectilinearProjector_propB_const_float(instance: *mut c_void, val: f32);
// cv::detail::CompressedRectilinearProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::CompressedRectilinearProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::CompressedRectilinearProjector::delete() generated
// ("cv::detail::CompressedRectilinearProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearProjector_delete(instance: *mut c_void);
// CompressedRectilinearWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:392
// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
pub fn cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:392
// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CompressedRectilinearWarper::to_Detail_RotationWarper() generated
// ("cv::detail::CompressedRectilinearWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CompressedRectilinearWarper::delete() generated
// ("cv::detail::CompressedRectilinearWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CompressedRectilinearWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:655
// ("cv::detail::CylindricalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:656
// ("cv::detail::CylindricalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::CylindricalPortraitProjector::defaultNew() generated
// ("cv::detail::CylindricalPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CylindricalPortraitProjector_defaultNew_const() -> *mut c_void;
// cv::detail::CylindricalPortraitProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::CylindricalPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalPortraitProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalPortraitProjector::delete() generated
// ("cv::detail::CylindricalPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalPortraitProjector_delete(instance: *mut c_void);
// CylindricalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:663
// ("cv::detail::CylindricalPortraitWarper::CylindricalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::CylindricalPortraitWarper::to_Detail_RotationWarper() generated
// ("cv::detail::CylindricalPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalPortraitWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalPortraitWarper::delete() generated
// ("cv::detail::CylindricalPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalPortraitWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:326
// ("cv::detail::CylindricalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:327
// ("cv::detail::CylindricalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::CylindricalProjector::defaultNew() generated
// ("cv::detail::CylindricalProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CylindricalProjector_defaultNew_const() -> *mut c_void;
// cv::detail::CylindricalProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::CylindricalProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalProjector::delete() generated
// ("cv::detail::CylindricalProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalProjector_delete(instance: *mut c_void);
// CylindricalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:340
// ("cv::detail::CylindricalWarper::CylindricalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_CylindricalWarper_CylindricalWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:342
// ("cv::detail::CylindricalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:343
// ("cv::detail::CylindricalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// cv::detail::CylindricalWarper::to_Detail_CylindricalWarperGpu() generated
// ("cv::detail::CylindricalWarper::to_Detail_CylindricalWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarper_to_Detail_CylindricalWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalWarper::to_Detail_RotationWarper() generated
// ("cv::detail::CylindricalWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalWarper::delete() generated
// ("cv::detail::CylindricalWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarper_delete(instance: *mut c_void);
// CylindricalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:598
// ("cv::detail::CylindricalWarperGpu::CylindricalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:605
// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:613
// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:625
// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:627
// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
// cv::detail::CylindricalWarperGpu::to_Detail_CylindricalWarper() generated
// ("cv::detail::CylindricalWarperGpu::to_Detail_CylindricalWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarperGpu_to_Detail_CylindricalWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalWarperGpu::to_Detail_RotationWarper() generated
// ("cv::detail::CylindricalWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarperGpu_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::CylindricalWarperGpu::delete() generated
// ("cv::detail::CylindricalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CylindricalWarperGpu_delete(instance: *mut c_void);
// DisjointSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:58
// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
pub fn cv_detail_DisjointSets_DisjointSets_int(elem_count: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::DisjointSets::DisjointSets() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:58
// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_DisjointSets_DisjointSets(ocvrs_return: *mut Result<*mut c_void>);
// createOneElemSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:60
// ("cv::detail::DisjointSets::createOneElemSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
pub fn cv_detail_DisjointSets_createOneElemSets_int(instance: *mut c_void, elem_count: i32, ocvrs_return: *mut Result<()>);
// findSetByElem(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:61
// ("cv::detail::DisjointSets::findSetByElem", vec![(pred!(mut, ["elem"], ["int"]), _)]),
pub fn cv_detail_DisjointSets_findSetByElem_int(instance: *mut c_void, elem: i32, ocvrs_return: *mut Result<i32>);
// mergeSets(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:62
// ("cv::detail::DisjointSets::mergeSets", vec![(pred!(mut, ["set1", "set2"], ["int", "int"]), _)]),
pub fn cv_detail_DisjointSets_mergeSets_int_int(instance: *mut c_void, set1: i32, set2: i32, ocvrs_return: *mut Result<i32>);
// cv::detail::DisjointSets::parent() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:64
// ("cv::detail::DisjointSets::parent", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_DisjointSets_propParent_const(instance: *const c_void) -> *mut c_void;
// cv::detail::DisjointSets::setParent(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:64
// ("cv::detail::DisjointSets::setParent", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_detail_DisjointSets_propParent_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::detail::DisjointSets::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:65
// ("cv::detail::DisjointSets::size", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_DisjointSets_propSize_const(instance: *const c_void) -> *mut c_void;
// cv::detail::DisjointSets::setSize(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:65
// ("cv::detail::DisjointSets::setSize", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_detail_DisjointSets_propSize_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::detail::DisjointSets::delete() generated
// ("cv::detail::DisjointSets::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_DisjointSets_delete(instance: *mut c_void);
// DpSeamFinder(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:125
// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, ["costFunc"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
pub fn cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cost_func: crate::stitching::Detail_DpSeamFinder_CostFunction, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::DpSeamFinder::DpSeamFinder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:125
// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_DpSeamFinder_DpSeamFinder(ocvrs_return: *mut Result<*mut c_void>);
// DpSeamFinder(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:126
// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, ["costFunc"], ["cv::String"]), _)]),
pub fn cv_detail_DpSeamFinder_DpSeamFinder_String(cost_func: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// costFunction()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:128
// ("cv::detail::DpSeamFinder::costFunction", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_DpSeamFinder_costFunction_const(instance: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_DpSeamFinder_CostFunction>);
// setCostFunction(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:129
// ("cv::detail::DpSeamFinder::setCostFunction", vec![(pred!(mut, ["val"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
pub fn cv_detail_DpSeamFinder_setCostFunction_CostFunction(instance: *mut c_void, val: crate::stitching::Detail_DpSeamFinder_CostFunction, ocvrs_return: *mut Result<()>);
// setCostFunction(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:130
// ("cv::detail::DpSeamFinder::setCostFunction", vec![(pred!(mut, ["val"], ["cv::String"]), _)]),
pub fn cv_detail_DpSeamFinder_setCostFunction_String(instance: *mut c_void, val: *const c_char, ocvrs_return: *mut Result<()>);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:132
// ("cv::detail::DpSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_DpSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::DpSeamFinder::to_Detail_SeamFinder() generated
// ("cv::detail::DpSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_DpSeamFinder_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::DpSeamFinder::delete() generated
// ("cv::detail::DpSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_DpSeamFinder_delete(instance: *mut c_void);
// operator()(const std::vector<ImageFeatures> &, const std::vector<MatchesInfo> &, std::vector<CameraParams> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:77
// ("cv::detail::Estimator::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "cameras"], ["const std::vector<cv::detail::ImageFeatures>*", "const std::vector<cv::detail::MatchesInfo>*", "std::vector<cv::detail::CameraParams>*"]), _)]),
pub fn cv_detail_Estimator_operator___const_vectorLImageFeaturesGR_const_vectorLMatchesInfoGR_vectorLCameraParamsGR(instance: *mut c_void, features: *const c_void, pairwise_matches: *const c_void, cameras: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::detail::Estimator::to_Detail_AffineBasedEstimator() generated
// ("cv::detail::Estimator::to_Detail_AffineBasedEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_AffineBasedEstimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_BundleAdjusterAffine() generated
// ("cv::detail::Estimator::to_Detail_BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_BundleAdjusterAffine(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_BundleAdjusterAffinePartial() generated
// ("cv::detail::Estimator::to_Detail_BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_BundleAdjusterAffinePartial(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::Estimator::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_BundleAdjusterRay() generated
// ("cv::detail::Estimator::to_Detail_BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_BundleAdjusterRay(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_BundleAdjusterReproj() generated
// ("cv::detail::Estimator::to_Detail_BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_BundleAdjusterReproj(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_HomographyBasedEstimator() generated
// ("cv::detail::Estimator::to_Detail_HomographyBasedEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_HomographyBasedEstimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::to_Detail_NoBundleAdjuster() generated
// ("cv::detail::Estimator::to_Detail_NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_to_Detail_NoBundleAdjuster(instance: *mut c_void) -> *mut c_void;
// cv::detail::Estimator::delete() generated
// ("cv::detail::Estimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Estimator_delete(instance: *mut c_void);
// createDefault(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:68
// ("cv::detail::ExposureCompensator::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_detail_ExposureCompensator_createDefault_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:76
// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLUMatGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:79
// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:88
// ("cv::detail::ExposureCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:89
// ("cv::detail::ExposureCompensator::getMatGains", vec![(pred!(mut, ["unnamed"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_ExposureCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:90
// ("cv::detail::ExposureCompensator::setMatGains", vec![(pred!(mut, ["unnamed"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_ExposureCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// setUpdateGain(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:91
// ("cv::detail::ExposureCompensator::setUpdateGain", vec![(pred!(mut, ["b"], ["bool"]), _)]),
pub fn cv_detail_ExposureCompensator_setUpdateGain_bool(instance: *mut c_void, b: bool, ocvrs_return: *mut Result<()>);
// getUpdateGain()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:92
// ("cv::detail::ExposureCompensator::getUpdateGain", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_getUpdateGain(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::detail::ExposureCompensator::to_Detail_BlocksChannelsCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_BlocksChannelsCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_BlocksChannelsCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::to_Detail_BlocksCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_BlocksCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::to_Detail_BlocksGainCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_BlocksGainCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::to_Detail_ChannelsCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_ChannelsCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_ChannelsCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::to_Detail_GainCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_GainCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_GainCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::to_Detail_NoExposureCompensator() generated
// ("cv::detail::ExposureCompensator::to_Detail_NoExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_to_Detail_NoExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::ExposureCompensator::delete() generated
// ("cv::detail::ExposureCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExposureCompensator_delete(instance: *mut c_void);
// FeatherBlender(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:103
// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, ["sharpness"], ["float"]), _)]),
pub fn cv_detail_FeatherBlender_FeatherBlender_float(sharpness: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::FeatherBlender::FeatherBlender() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:103
// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeatherBlender_FeatherBlender(ocvrs_return: *mut Result<*mut c_void>);
// sharpness()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:105
// ("cv::detail::FeatherBlender::sharpness", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_FeatherBlender_sharpness_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSharpness(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:106
// ("cv::detail::FeatherBlender::setSharpness", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_detail_FeatherBlender_setSharpness_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:108
// ("cv::detail::FeatherBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
pub fn cv_detail_FeatherBlender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result<()>);
// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:109
// ("cv::detail::FeatherBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
pub fn cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result<()>);
// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:110
// ("cv::detail::FeatherBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result<()>);
// createWeightMaps(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:114
// ("cv::detail::FeatherBlender::createWeightMaps", vec![(pred!(mut, ["masks", "corners", "weight_maps"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_FeatherBlender_createWeightMaps_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, masks: *const c_void, corners: *const c_void, weight_maps: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::detail::FeatherBlender::to_Detail_Blender() generated
// ("cv::detail::FeatherBlender::to_Detail_Blender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeatherBlender_to_Detail_Blender(instance: *mut c_void) -> *mut c_void;
// cv::detail::FeatherBlender::delete() generated
// ("cv::detail::FeatherBlender::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeatherBlender_delete(instance: *mut c_void);
// operator()(const ImageFeatures &, const ImageFeatures &, MatchesInfo &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:127
// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features1", "features2", "matches_info"], ["const cv::detail::ImageFeatures*", "const cv::detail::ImageFeatures*", "cv::detail::MatchesInfo*"]), _)]),
pub fn cv_detail_FeaturesMatcher_operator___const_ImageFeaturesR_const_ImageFeaturesR_MatchesInfoR(instance: *mut c_void, features1: *const c_void, features2: *const c_void, matches_info: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(const std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, const cv::UMat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:140
// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "mask"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "const cv::UMat*"]), _)]),
pub fn cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR_const_UMatR(instance: *mut c_void, features: *const c_void, pairwise_matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::FeaturesMatcher::operator()(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:140
// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*"]), _)]),
pub fn cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR(instance: *mut c_void, features: *const c_void, pairwise_matches: *mut c_void, ocvrs_return: *mut Result<()>);
// isThreadSafe()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:145
// ("cv::detail::FeaturesMatcher::isThreadSafe", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_isThreadSafe_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:149
// ("cv::detail::FeaturesMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::FeaturesMatcher::to_Detail_AffineBestOf2NearestMatcher() generated
// ("cv::detail::FeaturesMatcher::to_Detail_AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_to_Detail_AffineBestOf2NearestMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestMatcher() generated
// ("cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestRangeMatcher() generated
// ("cv::detail::FeaturesMatcher::to_Detail_BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestRangeMatcher(instance: *mut c_void) -> *mut c_void;
// cv::detail::FeaturesMatcher::delete() generated
// ("cv::detail::FeaturesMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FeaturesMatcher_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:354
// ("cv::detail::FisheyeProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:355
// ("cv::detail::FisheyeProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::FisheyeProjector::defaultNew() generated
// ("cv::detail::FisheyeProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_FisheyeProjector_defaultNew_const() -> *mut c_void;
// cv::detail::FisheyeProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::FisheyeProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FisheyeProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::FisheyeProjector::delete() generated
// ("cv::detail::FisheyeProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FisheyeProjector_delete(instance: *mut c_void);
// FisheyeWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:362
// ("cv::detail::FisheyeWarper::FisheyeWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_FisheyeWarper_FisheyeWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::FisheyeWarper::to_Detail_RotationWarper() generated
// ("cv::detail::FisheyeWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FisheyeWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::FisheyeWarper::delete() generated
// ("cv::detail::FisheyeWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_FisheyeWarper_delete(instance: *mut c_void);
// GainCompensator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:116
// ("cv::detail::GainCompensator::GainCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GainCompensator_GainCompensator(ocvrs_return: *mut Result<*mut c_void>);
// GainCompensator(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:118
// ("cv::detail::GainCompensator::GainCompensator", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
pub fn cv_detail_GainCompensator_GainCompensator_int(nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:120
// ("cv::detail::GainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_GainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// singleFeed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:122
// ("cv::detail::GainCompensator::singleFeed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_GainCompensator_singleFeed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:124
// ("cv::detail::GainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:125
// ("cv::detail::GainCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_GainCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:126
// ("cv::detail::GainCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_GainCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setNrFeeds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:127
// ("cv::detail::GainCompensator::setNrFeeds", vec![(pred!(mut, ["nr_feeds"], ["int"]), _)]),
pub fn cv_detail_GainCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result<()>);
// getNrFeeds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:128
// ("cv::detail::GainCompensator::getNrFeeds", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GainCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setSimilarityThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:129
// ("cv::detail::GainCompensator::setSimilarityThreshold", vec![(pred!(mut, ["similarity_threshold"], ["double"]), _)]),
pub fn cv_detail_GainCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result<()>);
// getSimilarityThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:130
// ("cv::detail::GainCompensator::getSimilarityThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GainCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// prepareSimilarityMask(const std::vector<Point> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:131
// ("cv::detail::GainCompensator::prepareSimilarityMask", vec![(pred!(mut, ["corners", "images"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_GainCompensator_prepareSimilarityMask_const_vectorLPointGR_const_vectorLUMatGR(instance: *mut c_void, corners: *const c_void, images: *const c_void, ocvrs_return: *mut Result<()>);
// gains()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:132
// ("cv::detail::GainCompensator::gains", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GainCompensator_gains_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::GainCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::GainCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GainCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::GainCompensator::delete() generated
// ("cv::detail::GainCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GainCompensator_delete(instance: *mut c_void);
// Graph(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:88
// ("cv::detail::Graph::Graph", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
pub fn cv_detail_Graph_Graph_int(num_vertices: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::Graph::Graph() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:88
// ("cv::detail::Graph::Graph", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Graph_Graph(ocvrs_return: *mut Result<*mut c_void>);
// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:89
// ("cv::detail::Graph::create", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
pub fn cv_detail_Graph_create_int(instance: *mut c_void, num_vertices: i32, ocvrs_return: *mut Result<()>);
// numVertices()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:90
// ("cv::detail::Graph::numVertices", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_Graph_numVertices_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// addEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:91
// ("cv::detail::Graph::addEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
pub fn cv_detail_Graph_addEdge_int_int_float(instance: *mut c_void, from: i32, to: i32, weight: f32, ocvrs_return: *mut Result<()>);
// cv::detail::Graph::delete() generated
// ("cv::detail::Graph::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_Graph_delete(instance: *mut c_void);
// GraphCutSeamFinder(int, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:243
// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["int", "float", "float"]), _)]),
pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::GraphCutSeamFinder::GraphCutSeamFinder() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:243
// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder(ocvrs_return: *mut Result<*mut c_void>);
// GraphCutSeamFinder(String, float, float)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:245
// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["cv::String", "float", "float"]), _)]),
pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(cost_type: *const c_char, terminal_cost: f32, bad_region_penalty: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::GraphCutSeamFinder::GraphCutSeamFinder(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:245
// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type"], ["cv::String"]), _)]),
pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String(cost_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:250
// ("cv::detail::GraphCutSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_GraphCutSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::GraphCutSeamFinder::to_Detail_GraphCutSeamFinderBase() generated
// ("cv::detail::GraphCutSeamFinder::to_Detail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinder_to_Detail_GraphCutSeamFinderBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::GraphCutSeamFinder::to_Detail_SeamFinder() generated
// ("cv::detail::GraphCutSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinder_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::GraphCutSeamFinder::delete() generated
// ("cv::detail::GraphCutSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinder_delete(instance: *mut c_void);
// cv::detail::GraphCutSeamFinderBase::defaultNew() generated
// ("cv::detail::GraphCutSeamFinderBase::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderBase_defaultNew_const() -> *mut c_void;
// cv::detail::GraphCutSeamFinderBase::delete() generated
// ("cv::detail::GraphCutSeamFinderBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderBase_delete(instance: *mut c_void);
// GraphCutSeamFinderGpu(int, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:264
// ("cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["int", "float", "float"]), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu_int_float_float(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:264
// ("cv::detail::GraphCutSeamFinderGpu::GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu(ocvrs_return: *mut Result<*mut c_void>);
// find(const std::vector<cv::UMat> &, const std::vector<cv::Point> &, std::vector<cv::UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:269
// ("cv::detail::GraphCutSeamFinderGpu::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// findInPair(size_t, size_t, Rect)(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:271
// ("cv::detail::GraphCutSeamFinderGpu::findInPair", vec![(pred!(mut, ["first", "second", "roi"], ["size_t", "size_t", "cv::Rect"]), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_findInPair_size_t_size_t_Rect(instance: *mut c_void, first: size_t, second: size_t, roi: *const core::Rect, ocvrs_return: *mut Result<()>);
// cv::detail::GraphCutSeamFinderGpu::to_Detail_GraphCutSeamFinderBase() generated
// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_to_Detail_GraphCutSeamFinderBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::GraphCutSeamFinderGpu::to_Detail_PairwiseSeamFinder() generated
// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_to_Detail_PairwiseSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::GraphCutSeamFinderGpu::to_Detail_SeamFinder() generated
// ("cv::detail::GraphCutSeamFinderGpu::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::GraphCutSeamFinderGpu::delete() generated
// ("cv::detail::GraphCutSeamFinderGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphCutSeamFinderGpu_delete(instance: *mut c_void);
// GraphEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:74
// ("cv::detail::GraphEdge::GraphEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
pub fn cv_detail_GraphEdge_GraphEdge_int_int_float(from: i32, to: i32, weight: f32, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:75
// ("cv::detail::GraphEdge::operator<", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
pub fn cv_detail_GraphEdge_operatorL_const_const_GraphEdgeR(instance: *const c_void, other: *const c_void, ocvrs_return: *mut Result<bool>);
// operator>(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:76
// ("cv::detail::GraphEdge::operator>", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
pub fn cv_detail_GraphEdge_operatorG_const_const_GraphEdgeR(instance: *const c_void, other: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::detail::GraphEdge::from() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
// ("cv::detail::GraphEdge::from", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GraphEdge_propFrom_const(instance: *const c_void) -> i32;
// cv::detail::GraphEdge::setFrom(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
// ("cv::detail::GraphEdge::setFrom", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_GraphEdge_propFrom_const_int(instance: *mut c_void, val: i32);
// cv::detail::GraphEdge::to() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
// ("cv::detail::GraphEdge::to", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GraphEdge_propTo_const(instance: *const c_void) -> i32;
// cv::detail::GraphEdge::setTo(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:78
// ("cv::detail::GraphEdge::setTo", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_GraphEdge_propTo_const_int(instance: *mut c_void, val: i32);
// cv::detail::GraphEdge::weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:79
// ("cv::detail::GraphEdge::weight", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_GraphEdge_propWeight_const(instance: *const c_void) -> f32;
// cv::detail::GraphEdge::setWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/util.hpp:79
// ("cv::detail::GraphEdge::setWeight", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_GraphEdge_propWeight_const_float(instance: *mut c_void, val: f32);
// cv::detail::GraphEdge::delete() generated
// ("cv::detail::GraphEdge::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GraphEdge_delete(instance: *mut c_void);
// HomographyBasedEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:103
// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, ["is_focals_estimated"], ["bool"]), _)]),
pub fn cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(is_focals_estimated: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::HomographyBasedEstimator::HomographyBasedEstimator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:103
// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_HomographyBasedEstimator_HomographyBasedEstimator(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::HomographyBasedEstimator::to_Detail_Estimator() generated
// ("cv::detail::HomographyBasedEstimator::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_HomographyBasedEstimator_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::HomographyBasedEstimator::delete() generated
// ("cv::detail::HomographyBasedEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_HomographyBasedEstimator_delete(instance: *mut c_void);
// getKeypoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:64
// ("cv::detail::ImageFeatures::getKeypoints", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ImageFeatures_getKeypoints(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::ImageFeatures::implicitClone() generated
// ("cv::detail::ImageFeatures::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::ImageFeatures::defaultNew() generated
// ("cv::detail::ImageFeatures::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_defaultNew_const() -> *mut c_void;
// cv::detail::ImageFeatures::img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:60
// ("cv::detail::ImageFeatures::img_idx", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_propImg_idx_const(instance: *const c_void) -> i32;
// cv::detail::ImageFeatures::setImg_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:60
// ("cv::detail::ImageFeatures::setImg_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_ImageFeatures_propImg_idx_const_int(instance: *mut c_void, val: i32);
// cv::detail::ImageFeatures::img_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:61
// ("cv::detail::ImageFeatures::img_size", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_propImg_size_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::detail::ImageFeatures::setImg_size(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:61
// ("cv::detail::ImageFeatures::setImg_size", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_detail_ImageFeatures_propImg_size_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::detail::ImageFeatures::keypoints() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:62
// ("cv::detail::ImageFeatures::keypoints", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_propKeypoints_const(instance: *const c_void) -> *mut c_void;
// cv::detail::ImageFeatures::setKeypoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:62
// ("cv::detail::ImageFeatures::setKeypoints", vec![(pred!(mut, ["val"], ["const std::vector<cv::KeyPoint>"]), _)]),
pub fn cv_detail_ImageFeatures_propKeypoints_const_vectorLKeyPointG(instance: *mut c_void, val: *const c_void);
// cv::detail::ImageFeatures::descriptors() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:63
// ("cv::detail::ImageFeatures::descriptors", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ImageFeatures_propDescriptors_const(instance: *const c_void) -> *mut c_void;
// cv::detail::ImageFeatures::setDescriptors(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:63
// ("cv::detail::ImageFeatures::setDescriptors", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
pub fn cv_detail_ImageFeatures_propDescriptors_const_UMat(instance: *mut c_void, val: *const c_void);
// cv::detail::ImageFeatures::delete() generated
// ("cv::detail::ImageFeatures::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ImageFeatures_delete(instance: *mut c_void);
// MatchesInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:101
// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MatchesInfo_MatchesInfo(ocvrs_return: *mut Result<*mut c_void>);
// MatchesInfo(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:102
// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
pub fn cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:103
// ("cv::detail::MatchesInfo::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
pub fn cv_detail_MatchesInfo_operatorST_const_MatchesInfoR(instance: *mut c_void, other: *const c_void, ocvrs_return: *mut Result<()>);
// getMatches()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:112
// ("cv::detail::MatchesInfo::getMatches", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MatchesInfo_getMatches(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getInliers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:113
// ("cv::detail::MatchesInfo::getInliers", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MatchesInfo_getInliers(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::MatchesInfo::implicitClone() generated
// ("cv::detail::MatchesInfo::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::MatchesInfo::src_img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:105
// ("cv::detail::MatchesInfo::src_img_idx", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propSrc_img_idx_const(instance: *const c_void) -> i32;
// cv::detail::MatchesInfo::setSrc_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:105
// ("cv::detail::MatchesInfo::setSrc_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_MatchesInfo_propSrc_img_idx_const_int(instance: *mut c_void, val: i32);
// cv::detail::MatchesInfo::dst_img_idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:106
// ("cv::detail::MatchesInfo::dst_img_idx", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propDst_img_idx_const(instance: *const c_void) -> i32;
// cv::detail::MatchesInfo::setDst_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:106
// ("cv::detail::MatchesInfo::setDst_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_MatchesInfo_propDst_img_idx_const_int(instance: *mut c_void, val: i32);
// cv::detail::MatchesInfo::matches() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:107
// ("cv::detail::MatchesInfo::matches", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propMatches_const(instance: *const c_void) -> *mut c_void;
// cv::detail::MatchesInfo::setMatches(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:107
// ("cv::detail::MatchesInfo::setMatches", vec![(pred!(mut, ["val"], ["const std::vector<cv::DMatch>"]), _)]),
pub fn cv_detail_MatchesInfo_propMatches_const_vectorLDMatchG(instance: *mut c_void, val: *const c_void);
// cv::detail::MatchesInfo::inliers_mask() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:108
// ("cv::detail::MatchesInfo::inliers_mask", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propInliers_mask_const(instance: *const c_void) -> *mut c_void;
// cv::detail::MatchesInfo::setInliers_mask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:108
// ("cv::detail::MatchesInfo::setInliers_mask", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
pub fn cv_detail_MatchesInfo_propInliers_mask_const_vectorLunsigned_charG(instance: *mut c_void, val: *const c_void);
// cv::detail::MatchesInfo::num_inliers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:109
// ("cv::detail::MatchesInfo::num_inliers", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propNum_inliers_const(instance: *const c_void) -> i32;
// cv::detail::MatchesInfo::setNum_inliers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:109
// ("cv::detail::MatchesInfo::setNum_inliers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_MatchesInfo_propNum_inliers_const_int(instance: *mut c_void, val: i32);
// cv::detail::MatchesInfo::H() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:110
// ("cv::detail::MatchesInfo::H", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propH_const(instance: *const c_void) -> *mut c_void;
// cv::detail::MatchesInfo::setH(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:110
// ("cv::detail::MatchesInfo::setH", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_detail_MatchesInfo_propH_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::detail::MatchesInfo::confidence() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:111
// ("cv::detail::MatchesInfo::confidence", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MatchesInfo_propConfidence_const(instance: *const c_void) -> f64;
// cv::detail::MatchesInfo::setConfidence(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/matchers.hpp:111
// ("cv::detail::MatchesInfo::setConfidence", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_detail_MatchesInfo_propConfidence_const_double(instance: *mut c_void, val: f64);
// cv::detail::MatchesInfo::delete() generated
// ("cv::detail::MatchesInfo::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MatchesInfo_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:467
// ("cv::detail::MercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:468
// ("cv::detail::MercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::MercatorProjector::defaultNew() generated
// ("cv::detail::MercatorProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MercatorProjector_defaultNew_const() -> *mut c_void;
// cv::detail::MercatorProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::MercatorProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MercatorProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::MercatorProjector::delete() generated
// ("cv::detail::MercatorProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MercatorProjector_delete(instance: *mut c_void);
// MercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:475
// ("cv::detail::MercatorWarper::MercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_MercatorWarper_MercatorWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::MercatorWarper::to_Detail_RotationWarper() generated
// ("cv::detail::MercatorWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MercatorWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::MercatorWarper::delete() generated
// ("cv::detail::MercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MercatorWarper_delete(instance: *mut c_void);
// MultiBandBlender(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:130
// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, ["try_gpu", "num_bands", "weight_type"], ["int", "int", "int"]), _)]),
pub fn cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(try_gpu: i32, num_bands: i32, weight_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::MultiBandBlender::MultiBandBlender() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:130
// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MultiBandBlender_MultiBandBlender(ocvrs_return: *mut Result<*mut c_void>);
// numBands()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:132
// ("cv::detail::MultiBandBlender::numBands", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_MultiBandBlender_numBands_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumBands(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:133
// ("cv::detail::MultiBandBlender::setNumBands", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_detail_MultiBandBlender_setNumBands_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:135
// ("cv::detail::MultiBandBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
pub fn cv_detail_MultiBandBlender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result<()>);
// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:136
// ("cv::detail::MultiBandBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
pub fn cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result<()>);
// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/blenders.hpp:137
// ("cv::detail::MultiBandBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::MultiBandBlender::to_Detail_Blender() generated
// ("cv::detail::MultiBandBlender::to_Detail_Blender", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MultiBandBlender_to_Detail_Blender(instance: *mut c_void) -> *mut c_void;
// cv::detail::MultiBandBlender::delete() generated
// ("cv::detail::MultiBandBlender::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_MultiBandBlender_delete(instance: *mut c_void);
// NoBundleAdjuster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/motion_estimators.hpp:224
// ("cv::detail::NoBundleAdjuster::NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoBundleAdjuster_NoBundleAdjuster(ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::NoBundleAdjuster::to_Detail_BundleAdjusterBase() generated
// ("cv::detail::NoBundleAdjuster::to_Detail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoBundleAdjuster_to_Detail_BundleAdjusterBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::NoBundleAdjuster::to_Detail_Estimator() generated
// ("cv::detail::NoBundleAdjuster::to_Detail_Estimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoBundleAdjuster_to_Detail_Estimator(instance: *mut c_void) -> *mut c_void;
// cv::detail::NoBundleAdjuster::delete() generated
// ("cv::detail::NoBundleAdjuster::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoBundleAdjuster_delete(instance: *mut c_void);
// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:102
// ("cv::detail::NoExposureCompensator::feed", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
pub fn cv_detail_NoExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(instance: *mut c_void, unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *const c_void, ocvrs_return: *mut Result<()>);
// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:104
// ("cv::detail::NoExposureCompensator::apply", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed", "unnamed"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, unnamed: i32, unnamed_1: *const core::Point, unnamed_2: *const c_void, unnamed_3: *const c_void, ocvrs_return: *mut Result<()>);
// getMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:105
// ("cv::detail::NoExposureCompensator::getMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_NoExposureCompensator_getMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// setMatGains(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/exposure_compensate.hpp:106
// ("cv::detail::NoExposureCompensator::setMatGains", vec![(pred!(mut, ["umv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_detail_NoExposureCompensator_setMatGains_vectorLMatGR(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::NoExposureCompensator::defaultNew() generated
// ("cv::detail::NoExposureCompensator::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_NoExposureCompensator_defaultNew_const() -> *mut c_void;
// cv::detail::NoExposureCompensator::to_Detail_ExposureCompensator() generated
// ("cv::detail::NoExposureCompensator::to_Detail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoExposureCompensator_to_Detail_ExposureCompensator(instance: *mut c_void) -> *mut c_void;
// cv::detail::NoExposureCompensator::delete() generated
// ("cv::detail::NoExposureCompensator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoExposureCompensator_delete(instance: *mut c_void);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:79
// ("cv::detail::NoSeamFinder::find", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_NoSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::NoSeamFinder::defaultNew() generated
// ("cv::detail::NoSeamFinder::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_NoSeamFinder_defaultNew_const() -> *mut c_void;
// cv::detail::NoSeamFinder::to_Detail_SeamFinder() generated
// ("cv::detail::NoSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoSeamFinder_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::NoSeamFinder::delete() generated
// ("cv::detail::NoSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_NoSeamFinder_delete(instance: *mut c_void);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:87
// ("cv::detail::PairwiseSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_PairwiseSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::PairwiseSeamFinder::to_Detail_GraphCutSeamFinderGpu() generated
// ("cv::detail::PairwiseSeamFinder::to_Detail_GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PairwiseSeamFinder_to_Detail_GraphCutSeamFinderGpu(instance: *mut c_void) -> *mut c_void;
// cv::detail::PairwiseSeamFinder::to_Detail_VoronoiSeamFinder() generated
// ("cv::detail::PairwiseSeamFinder::to_Detail_VoronoiSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PairwiseSeamFinder_to_Detail_VoronoiSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::PairwiseSeamFinder::to_Detail_SeamFinder() generated
// ("cv::detail::PairwiseSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PairwiseSeamFinder_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::PairwiseSeamFinder::delete() generated
// ("cv::detail::PairwiseSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PairwiseSeamFinder_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:447
// ("cv::detail::PaniniPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:448
// ("cv::detail::PaniniPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::PaniniPortraitProjector::defaultNew() generated
// ("cv::detail::PaniniPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniPortraitProjector_defaultNew_const() -> *mut c_void;
// cv::detail::PaniniPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
// ("cv::detail::PaniniPortraitProjector::a", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniPortraitProjector_propA_const(instance: *const c_void) -> f32;
// cv::detail::PaniniPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
// ("cv::detail::PaniniPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_PaniniPortraitProjector_propA_const_float(instance: *mut c_void, val: f32);
// cv::detail::PaniniPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
// ("cv::detail::PaniniPortraitProjector::b", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniPortraitProjector_propB_const(instance: *const c_void) -> f32;
// cv::detail::PaniniPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:445
// ("cv::detail::PaniniPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_PaniniPortraitProjector_propB_const_float(instance: *mut c_void, val: f32);
// cv::detail::PaniniPortraitProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::PaniniPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniPortraitProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::PaniniPortraitProjector::delete() generated
// ("cv::detail::PaniniPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniPortraitProjector_delete(instance: *mut c_void);
// PaniniPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:455
// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
pub fn cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PaniniPortraitWarper::PaniniPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:455
// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PaniniPortraitWarper::to_Detail_RotationWarper() generated
// ("cv::detail::PaniniPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniPortraitWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PaniniPortraitWarper::delete() generated
// ("cv::detail::PaniniPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniPortraitWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:426
// ("cv::detail::PaniniProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:427
// ("cv::detail::PaniniProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::PaniniProjector::defaultNew() generated
// ("cv::detail::PaniniProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniProjector_defaultNew_const() -> *mut c_void;
// cv::detail::PaniniProjector::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
// ("cv::detail::PaniniProjector::a", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniProjector_propA_const(instance: *const c_void) -> f32;
// cv::detail::PaniniProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
// ("cv::detail::PaniniProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_PaniniProjector_propA_const_float(instance: *mut c_void, val: f32);
// cv::detail::PaniniProjector::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
// ("cv::detail::PaniniProjector::b", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PaniniProjector_propB_const(instance: *const c_void) -> f32;
// cv::detail::PaniniProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:424
// ("cv::detail::PaniniProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_PaniniProjector_propB_const_float(instance: *mut c_void, val: f32);
// cv::detail::PaniniProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::PaniniProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::PaniniProjector::delete() generated
// ("cv::detail::PaniniProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniProjector_delete(instance: *mut c_void);
// PaniniWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:434
// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
pub fn cv_detail_PaniniWarper_PaniniWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PaniniWarper::PaniniWarper(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:434
// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_PaniniWarper_PaniniWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PaniniWarper::to_Detail_RotationWarper() generated
// ("cv::detail::PaniniWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PaniniWarper::delete() generated
// ("cv::detail::PaniniWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PaniniWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:674
// ("cv::detail::PlanePortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:675
// ("cv::detail::PlanePortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::PlanePortraitProjector::defaultNew() generated
// ("cv::detail::PlanePortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PlanePortraitProjector_defaultNew_const() -> *mut c_void;
// cv::detail::PlanePortraitProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::PlanePortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlanePortraitProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlanePortraitProjector::delete() generated
// ("cv::detail::PlanePortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlanePortraitProjector_delete(instance: *mut c_void);
// PlanePortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:682
// ("cv::detail::PlanePortraitWarper::PlanePortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PlanePortraitWarper::to_Detail_RotationWarper() generated
// ("cv::detail::PlanePortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlanePortraitWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlanePortraitWarper::delete() generated
// ("cv::detail::PlanePortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlanePortraitWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:185
// ("cv::detail::PlaneProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:186
// ("cv::detail::PlaneProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::PlaneProjector::defaultNew() generated
// ("cv::detail::PlaneProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_PlaneProjector_defaultNew_const() -> *mut c_void;
// cv::detail::PlaneProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::PlaneProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneProjector::delete() generated
// ("cv::detail::PlaneProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneProjector_delete(instance: *mut c_void);
// PlaneWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:198
// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_PlaneWarper_PlaneWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PlaneWarper::PlaneWarper() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:198
// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarper_PlaneWarper(ocvrs_return: *mut Result<*mut c_void>);
// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:200
// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPoint(const Point2f &, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:201
// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R", "T"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:203
// ("cv::detail::PlaneWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPointBackward(const Point2f &, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:204
// ("cv::detail::PlaneWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R", "T"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:206
// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:207
// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:209
// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:211
// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:214
// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warpRoi(Size, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:215
// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R", "T"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::detail::PlaneWarper::to_Detail_AffineWarper() generated
// ("cv::detail::PlaneWarper::to_Detail_AffineWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarper_to_Detail_AffineWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneWarper::to_Detail_PlaneWarperGpu() generated
// ("cv::detail::PlaneWarper::to_Detail_PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarper_to_Detail_PlaneWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneWarper::to_Detail_RotationWarper() generated
// ("cv::detail::PlaneWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneWarper::delete() generated
// ("cv::detail::PlaneWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarper_delete(instance: *mut c_void);
// PlaneWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:496
// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::PlaneWarperGpu::PlaneWarperGpu() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:496
// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarperGpu_PlaneWarperGpu(ocvrs_return: *mut Result<*mut c_void>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:503
// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:511
// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:519
// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:528
// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:540
// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// buildMaps(Size, InputArray, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:542
// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:544
// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
// warp(const cuda::GpuMat &, InputArray, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:547
// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
// cv::detail::PlaneWarperGpu::to_Detail_PlaneWarper() generated
// ("cv::detail::PlaneWarperGpu::to_Detail_PlaneWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarperGpu_to_Detail_PlaneWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneWarperGpu::to_Detail_RotationWarper() generated
// ("cv::detail::PlaneWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarperGpu_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::PlaneWarperGpu::delete() generated
// ("cv::detail::PlaneWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_PlaneWarperGpu_delete(instance: *mut c_void);
// setCameraParams(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:135
// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, ["K", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// cv::detail::ProjectorBase::setCameraParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:135
// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_setCameraParams(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::ProjectorBase::implicitClone() generated
// ("cv::detail::ProjectorBase::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::ProjectorBase::defaultNew() generated
// ("cv::detail::ProjectorBase::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_defaultNew_const() -> *mut c_void;
// cv::detail::ProjectorBase::scale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:139
// ("cv::detail::ProjectorBase::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propScale_const(instance: *const c_void) -> f32;
// cv::detail::ProjectorBase::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:139
// ("cv::detail::ProjectorBase::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_detail_ProjectorBase_propScale_const_float(instance: *mut c_void, val: f32);
// cv::detail::ProjectorBase::k() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:140
// ("cv::detail::ProjectorBase::k", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propK_const(instance: *const c_void) -> *const [f32; 9];
// cv::detail::ProjectorBase::kMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:140
// ("cv::detail::ProjectorBase::kMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_propK(instance: *mut c_void) -> *mut [f32; 9];
// cv::detail::ProjectorBase::rinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:141
// ("cv::detail::ProjectorBase::rinv", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propRinv_const(instance: *const c_void) -> *const [f32; 9];
// cv::detail::ProjectorBase::rinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:141
// ("cv::detail::ProjectorBase::rinvMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_propRinv(instance: *mut c_void) -> *mut [f32; 9];
// cv::detail::ProjectorBase::r_kinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:142
// ("cv::detail::ProjectorBase::r_kinv", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propR_kinv_const(instance: *const c_void) -> *const [f32; 9];
// cv::detail::ProjectorBase::r_kinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:142
// ("cv::detail::ProjectorBase::r_kinvMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_propR_kinv(instance: *mut c_void) -> *mut [f32; 9];
// cv::detail::ProjectorBase::k_rinv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:143
// ("cv::detail::ProjectorBase::k_rinv", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propK_rinv_const(instance: *const c_void) -> *const [f32; 9];
// cv::detail::ProjectorBase::k_rinvMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:143
// ("cv::detail::ProjectorBase::k_rinvMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_propK_rinv(instance: *mut c_void) -> *mut [f32; 9];
// cv::detail::ProjectorBase::t() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:144
// ("cv::detail::ProjectorBase::t", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ProjectorBase_propT_const(instance: *const c_void) -> *const [f32; 3];
// cv::detail::ProjectorBase::tMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:144
// ("cv::detail::ProjectorBase::tMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_propT(instance: *mut c_void) -> *mut [f32; 3];
// cv::detail::ProjectorBase::delete() generated
// ("cv::detail::ProjectorBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ProjectorBase_delete(instance: *mut c_void);
// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:71
// ("cv::detail::RotationWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// warpPointBackward(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:80
// ("cv::detail::RotationWarper::warpPointBackward", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_RotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:91
// ("cv::detail::RotationWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:103
// ("cv::detail::RotationWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// warpBackward(InputArray, InputArray, InputArray, int, int, Size, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:116
// ("cv::detail::RotationWarper::warpBackward", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst_size", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Size", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst_size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result<()>);
// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:125
// ("cv::detail::RotationWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// getScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:127
// ("cv::detail::RotationWarper::getScale", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_RotationWarper_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:128
// ("cv::detail::RotationWarper::setScale", vec![(pred!(mut, ["unnamed"], ["float"]), _)]),
pub fn cv_detail_RotationWarper_setScale_float(instance: *mut c_void, unnamed: f32, ocvrs_return: *mut Result<()>);
// cv::detail::RotationWarper::delete() generated
// ("cv::detail::RotationWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_RotationWarper_delete(instance: *mut c_void);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:69
// ("cv::detail::SeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_SeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// createDefault(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:71
// ("cv::detail::SeamFinder::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_detail_SeamFinder_createDefault_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::SeamFinder::to_Detail_DpSeamFinder() generated
// ("cv::detail::SeamFinder::to_Detail_DpSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_DpSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::to_Detail_GraphCutSeamFinder() generated
// ("cv::detail::SeamFinder::to_Detail_GraphCutSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_GraphCutSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::to_Detail_GraphCutSeamFinderGpu() generated
// ("cv::detail::SeamFinder::to_Detail_GraphCutSeamFinderGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_GraphCutSeamFinderGpu(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::to_Detail_NoSeamFinder() generated
// ("cv::detail::SeamFinder::to_Detail_NoSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_NoSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::to_Detail_PairwiseSeamFinder() generated
// ("cv::detail::SeamFinder::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_PairwiseSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::to_Detail_VoronoiSeamFinder() generated
// ("cv::detail::SeamFinder::to_Detail_VoronoiSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_to_Detail_VoronoiSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::SeamFinder::delete() generated
// ("cv::detail::SeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SeamFinder_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:637
// ("cv::detail::SphericalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:638
// ("cv::detail::SphericalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::SphericalPortraitProjector::defaultNew() generated
// ("cv::detail::SphericalPortraitProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_SphericalPortraitProjector_defaultNew_const() -> *mut c_void;
// cv::detail::SphericalPortraitProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::SphericalPortraitProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalPortraitProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalPortraitProjector::delete() generated
// ("cv::detail::SphericalPortraitProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalPortraitProjector_delete(instance: *mut c_void);
// SphericalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:647
// ("cv::detail::SphericalPortraitWarper::SphericalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::SphericalPortraitWarper::to_Detail_RotationWarper() generated
// ("cv::detail::SphericalPortraitWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalPortraitWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalPortraitWarper::delete() generated
// ("cv::detail::SphericalPortraitWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalPortraitWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:296
// ("cv::detail::SphericalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:297
// ("cv::detail::SphericalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::SphericalProjector::implicitClone() generated
// ("cv::detail::SphericalProjector::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_SphericalProjector_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::SphericalProjector::defaultNew() generated
// ("cv::detail::SphericalProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_SphericalProjector_defaultNew_const() -> *mut c_void;
// cv::detail::SphericalProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::SphericalProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalProjector::delete() generated
// ("cv::detail::SphericalProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalProjector_delete(instance: *mut c_void);
// SphericalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:315
// ("cv::detail::SphericalWarper::SphericalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_SphericalWarper_SphericalWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:317
// ("cv::detail::SphericalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:318
// ("cv::detail::SphericalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// cv::detail::SphericalWarper::to_Detail_SphericalWarperGpu() generated
// ("cv::detail::SphericalWarper::to_Detail_SphericalWarperGpu", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarper_to_Detail_SphericalWarperGpu(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalWarper::to_Detail_RotationWarper() generated
// ("cv::detail::SphericalWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalWarper::delete() generated
// ("cv::detail::SphericalWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarper_delete(instance: *mut c_void);
// SphericalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:558
// ("cv::detail::SphericalWarperGpu::SphericalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:565
// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:573
// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:585
// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:587
// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
// cv::detail::SphericalWarperGpu::to_Detail_RotationWarper() generated
// ("cv::detail::SphericalWarperGpu::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarperGpu_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalWarperGpu::to_Detail_SphericalWarper() generated
// ("cv::detail::SphericalWarperGpu::to_Detail_SphericalWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarperGpu_to_Detail_SphericalWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::SphericalWarperGpu::delete() generated
// ("cv::detail::SphericalWarperGpu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_SphericalWarperGpu_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:368
// ("cv::detail::StereographicProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:369
// ("cv::detail::StereographicProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::StereographicProjector::defaultNew() generated
// ("cv::detail::StereographicProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_StereographicProjector_defaultNew_const() -> *mut c_void;
// cv::detail::StereographicProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::StereographicProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_StereographicProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::StereographicProjector::delete() generated
// ("cv::detail::StereographicProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_StereographicProjector_delete(instance: *mut c_void);
// StereographicWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:376
// ("cv::detail::StereographicWarper::StereographicWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_StereographicWarper_StereographicWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::StereographicWarper::to_Detail_RotationWarper() generated
// ("cv::detail::StereographicWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_StereographicWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::StereographicWarper::delete() generated
// ("cv::detail::StereographicWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_StereographicWarper_delete(instance: *mut c_void);
// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:481
// ("cv::detail::TransverseMercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result<()>);
// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:482
// ("cv::detail::TransverseMercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
pub fn cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result<()>);
// cv::detail::TransverseMercatorProjector::defaultNew() generated
// ("cv::detail::TransverseMercatorProjector::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_TransverseMercatorProjector_defaultNew_const() -> *mut c_void;
// cv::detail::TransverseMercatorProjector::to_Detail_ProjectorBase() generated
// ("cv::detail::TransverseMercatorProjector::to_Detail_ProjectorBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_TransverseMercatorProjector_to_Detail_ProjectorBase(instance: *mut c_void) -> *mut c_void;
// cv::detail::TransverseMercatorProjector::delete() generated
// ("cv::detail::TransverseMercatorProjector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_TransverseMercatorProjector_delete(instance: *mut c_void);
// TransverseMercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/warpers.hpp:489
// ("cv::detail::TransverseMercatorWarper::TransverseMercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
pub fn cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::TransverseMercatorWarper::to_Detail_RotationWarper() generated
// ("cv::detail::TransverseMercatorWarper::to_Detail_RotationWarper", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_TransverseMercatorWarper_to_Detail_RotationWarper(instance: *mut c_void) -> *mut c_void;
// cv::detail::TransverseMercatorWarper::delete() generated
// ("cv::detail::TransverseMercatorWarper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_TransverseMercatorWarper_delete(instance: *mut c_void);
// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:111
// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_VoronoiSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// find(const std::vector<Size> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stitching/detail/seam_finders.hpp:113
// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["size", "corners", "masks"], ["const std::vector<cv::Size>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
pub fn cv_detail_VoronoiSeamFinder_find_const_vectorLSizeGR_const_vectorLPointGR_vectorLUMatGR(instance: *mut c_void, size: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::detail::VoronoiSeamFinder::defaultNew() generated
// ("cv::detail::VoronoiSeamFinder::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_VoronoiSeamFinder_defaultNew_const() -> *mut c_void;
// cv::detail::VoronoiSeamFinder::to_Detail_PairwiseSeamFinder() generated
// ("cv::detail::VoronoiSeamFinder::to_Detail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_VoronoiSeamFinder_to_Detail_PairwiseSeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::VoronoiSeamFinder::to_Detail_SeamFinder() generated
// ("cv::detail::VoronoiSeamFinder::to_Detail_SeamFinder", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_VoronoiSeamFinder_to_Detail_SeamFinder(instance: *mut c_void) -> *mut c_void;
// cv::detail::VoronoiSeamFinder::delete() generated
// ("cv::detail::VoronoiSeamFinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_VoronoiSeamFinder_delete(instance: *mut c_void);
