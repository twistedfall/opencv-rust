#include "ocvrs_common.hpp"
#include <opencv2/video.hpp>
// this is needed to support the legacy tracking module (see tracking.hpp)
#if (CV_VERSION_MAJOR >= 5) /* 5+ */ \
	|| (CV_VERSION_MAJOR == 4 && CV_VERSION_MINOR >= 6) /* 4.6.0+ */ \
	|| (CV_VERSION_MAJOR == 4 && CV_VERSION_MINOR == 5 && CV_VERSION_REVISION >= 1) /* 4.5.1+ */
	#if (CV_VERSION_MAJOR == 4 && CV_VERSION_MINOR == 5 && CV_VERSION_REVISION == 1) /* 4.5.1, header had a different name */
		#include <opencv2/video/detail/tracking.private.hpp>
	#else
		#include <opencv2/video/detail/tracking.detail.hpp>
	#endif
#endif
