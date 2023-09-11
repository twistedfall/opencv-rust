use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::ExportConfig;

/// Manual export config adjustments in form of "cpp_name(Reference)" => tweak function. If the export config is not
/// detected from the sources an `ExportConfig::default()` is passed to the function.
#[allow(clippy::type_complexity)]
pub static ELEMENT_EXPORT_TWEAK: Lazy<HashMap<&str, fn(ExportConfig) -> Option<ExportConfig>>> = Lazy::new(|| {
	HashMap::from([
		("VADisplay", ExportConfig::export as _),
		("VASurfaceID", ExportConfig::export as _),
		("cv::AffineWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::CompressedRectilinearPortraitWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::CompressedRectilinearWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::CylindricalWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::CylindricalWarperGpu", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::FisheyeWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::MercatorWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::PaniniPortraitWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::PaniniWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::PlaneWarper", ExportConfig::export as _),  // 3.2 3.4 stitching warpers
		("cv::PlaneWarperGpu", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::SphericalWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::SphericalWarperGpu", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::StereographicWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::TransverseMercatorWarper", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::WarperCreator", ExportConfig::export as _), // 3.2 3.4 stitching warpers
		("cv::ConfidenceMap", ExportConfig::export as _),
		("cv::CvFeatureParams", ExportConfig::export as _),
		("cv::CvHaarEvaluator", ExportConfig::export as _),
		("cv::DetectionBasedTracker::ExtObject", ExportConfig::force_boxed as _),
		("cv::DetectionBasedTracker::IDetector", ExportConfig::force_boxed as _),
		("cv::DetectionROI", ExportConfig::export as _),
		("cv::FileNodeIterator::SeqReader", ExportConfig::export as _),
		("cv::KeyPoint", ExportConfig::force_boxed as _), // has descendants in xfeatures2d
		("cv::Mat_", ExportConfig::export as _),
		("cv::String", ExportConfig::no_export as _),
		("cv::QtFont", ExportConfig::export as _),
		("cv::TermCriteria", ExportConfig::simple as _),
		("cv::RotatedRect", ExportConfig::simple as _), // marked simple since 4.8.0
		("cv::aruco::DetectorParameters", ExportConfig::force_boxed as _), // used in Ptr
		("cv::aruco::EstimateParameters", ExportConfig::force_boxed as _), // used in Ptr
		("cv::bioinspired::RetinaParameters", ExportConfig::force_boxed as _),
		("cv::bioinspired::SegmentationParameters", ExportConfig::simple as _),
		("cv::bioinspired::createRetina_OCL", ExportConfig::export as _), // 3.2 not exported
		("cv::cudacodec::FormatInfo", ExportConfig::simple as _),
		("cv::detail::CheckContext", ExportConfig::force_boxed as _),
		("cv::detail::ImageFeatures", ExportConfig::force_boxed as _), // 3.4 marked as boxed, but has no constructors
		("cv::dnn::BackendNode", ExportConfig::export as _),
		("cv::dnn::BackendWrapper", ExportConfig::export as _),
		("cv::dnn::DictValue", ExportConfig::export as _), // 3.2 not exported
		("cv::dnn::MatShape", ExportConfig::export as _),
		("cv::dnn::Net", ExportConfig::force_boxed as _), // incorrectly marked as simple
		("cv::dnn::_Range", ExportConfig::export as _),   // dnn shape_utils
		("cv::dnn::clamp", ExportConfig::export as _),    // dnn shape_utils
		("cv::dnn::concat", ExportConfig::export as _),   // dnn shape_utils
		("cv::dnn::getPlane", ExportConfig::export as _), // dnn shape_utils
		("cv::dnn::print", ExportConfig::export as _),    // dnn shape_utils
		("cv::dnn::shape", ExportConfig::export as _),    // dnn shape_utils
		("cv::dnn::slice", ExportConfig::export as _),    // dnn shape_utils
		("cv::dnn::toString", ExportConfig::export as _), // dnn shape_utils
		("cv::dnn::total", ExportConfig::export as _),    // dnn shape_utils
		("cv::face::CParams", ExportConfig::export as _),
		("cv::face::FacemarkAAM::Model::Texture", ExportConfig::export as _),
		("cv::getElemSize", ExportConfig::export as _),
		("cv::kinfu::Intr", ExportConfig::simple as _),
		("cv::linemod::QuantizedPyramid", ExportConfig::export as _), // missing in 3.2
		("cv::morphologyDefaultBorderValue", ExportConfig::export as _),
		("cv::ocl::Device", ExportConfig::force_boxed as _),
		("cv::optflow::GPCMatchingParams", ExportConfig::simple as _),
		("cv::optflow::GPCTrainingParams", ExportConfig::simple as _),
		("cv::ppf_match_3d::Pose3DPtr", ExportConfig::export as _),
		("cv::superres::PyrLKOpticalFlow", ExportConfig::export as _),
		("cv::utils::FunctionParams", ExportConfig::export as _), // missing in 4.8
		("cv::utils::logging::LogTag", ExportConfig::export as _),
		("cv::videostab::MaskFrameSource", ExportConfig::export as _),
		("cv::viz::Color", ExportConfig::export as _),
		("cv::ximgproc::Box", ExportConfig::simple as _), // used by Boxes typedef
		("cvv::impl::CallMetaData", ExportConfig::force_boxed as _),
		// gapi
		("cv::GCompileArg", ExportConfig::export as _),
		("cv::GCompileArgs", ExportConfig::export as _),
		("cv::GKinds", ExportConfig::export as _),
		("cv::GRunArgs", ExportConfig::export as _),
		("cv::GShapes", ExportConfig::export as _),
		("cv::GTypeInfo", ExportConfig::force_boxed as _),
		("cv::GTypesInfo", ExportConfig::export as _),
		("cv::RMat::IAdapter", ExportConfig::export as _),
		("cv::detail::ExtractArgsCallback", ExportConfig::export as _),
		("cv::detail::ExtractMetaCallback", ExportConfig::export as _),
		("cv::gapi::GFunctor", ExportConfig::export as _),
		("cv::util::any", ExportConfig::export as _),
		// force pure boxed
		("cv::dnn::TextDetectionModel", ExportConfig::override_boxed as _), // marked as simple, can be simple, but has protected constructor
	])
});
