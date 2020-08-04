#include "ocvrs_common.hpp"
#include <opencv2/videoio.hpp>
#if !(CV_VERSION_MAJOR == 3 && CV_VERSION_MINOR == 2)
	#include <opencv2/videoio/registry.hpp>
#endif
