use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::element::ExcludeKind;

/// cpp_name(Reference)
pub static ELEMENT_EXCLUDE_KIND: Lazy<HashMap<&str, ExcludeKind>> = Lazy::new(|| {
	HashMap::from([
		("cv::internal::format", ExcludeKind::Excluded),        // 3.2 duplicate definition
		("cv::face::FacemarkLBF::BBox", ExcludeKind::Excluded), // not used, not exported in windows dll
		("CV_DEPRECATED", ExcludeKind::Ignored),
		("CV_EXPORTS", ExcludeKind::Ignored),
		("CV_IMPL", ExcludeKind::Ignored), // 3.2
		("CV_MAKE_TYPE", ExcludeKind::Ignored),
		("CvFileNode", ExcludeKind::Ignored), // 3.2 3.4 C struct
		("CvSeq", ExcludeKind::Ignored),      // 3.2 C struct
		("FILE", ExcludeKind::Ignored),
		("HG_AUTOSIZE", ExcludeKind::Ignored), // 3.2
		("cv::ErrorCallback", ExcludeKind::Ignored),
		("cv::MatAllocator", ExcludeKind::Ignored),    // doesn't handle cpp part too well
		("cv::NAryMatIterator", ExcludeKind::Ignored), // uses pointers of pointers
		("cv::Node", ExcludeKind::Ignored),            // template class
		("cv::gapi::own::Mat", ExcludeKind::Ignored),  // internal alias to Mat
		("std::exception_ptr", ExcludeKind::Ignored),
		("std::random_access_iterator_tag", ExcludeKind::Ignored),
	])
});
