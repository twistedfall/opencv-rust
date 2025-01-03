use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::element::ExcludeKind;

/// cpp_name(Reference)
pub static ELEMENT_EXCLUDE_KIND: Lazy<HashMap<&str, ExcludeKind>> = Lazy::new(|| {
	HashMap::from([
		("cv::face::FacemarkLBF::BBox", ExcludeKind::Excluded), // not used, not exported in windows dll
		("CV_DEPRECATED", ExcludeKind::Ignored),
		("CV_EXPORTS", ExcludeKind::Ignored),
		("CV_IMPL", ExcludeKind::Ignored),
		("CV_MAKE_TYPE", ExcludeKind::Ignored),
		("FILE", ExcludeKind::Ignored),
		("HG_AUTOSIZE", ExcludeKind::Ignored), // 3.4
		("cv::ErrorCallback", ExcludeKind::Ignored),
		("cv::MatAllocator", ExcludeKind::Ignored),         // doesn't handle cpp part too well
		("cv::MatExpr::op", ExcludeKind::Ignored),          // fixme implement PointerOf types
		("cv::NAryMatIterator", ExcludeKind::Ignored),      // uses pointers of pointers
		("cv::Node", ExcludeKind::Ignored),                 // template class
		("cv::gapi::own::Mat", ExcludeKind::Ignored),       // internal alias to Mat
		("cv::text::ERStat::pixels", ExcludeKind::Ignored), // fixme: reference to a vector, we don't handle it too well yet
		("std::exception_ptr", ExcludeKind::Ignored),
		("std::random_access_iterator_tag", ExcludeKind::Ignored),
	])
});
