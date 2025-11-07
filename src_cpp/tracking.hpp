#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#if (CV_VERSION_MAJOR >= 5) /* 5+ */ \
	|| (CV_VERSION_MAJOR == 4 && CV_VERSION_MINOR >= 6) /* 4.6.0+ */ \
	|| (CV_VERSION_MAJOR == 4 && CV_VERSION_MINOR == 5 && CV_VERSION_REVISION >= 1) /* 4.5.1+ */
	#include <opencv2/tracking/tracking_legacy.hpp>
#endif
