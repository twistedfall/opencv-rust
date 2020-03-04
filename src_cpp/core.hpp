#define CV_COLLECT_IMPL_DATA

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/core/affine.hpp>
#include <opencv2/core/directx.hpp>
#include <opencv2/core/ocl.hpp>
#include <opencv2/core/va_intel.hpp>
//#include <opencv2/core/cuda.hpp> // todo
//#include <opencv2/core/opengl.hpp> // todo
#if !(CV_VERSION_MAJOR == 3 && CV_VERSION_MINOR == 2)
	#include <opencv2/core/async.hpp>
	#include <opencv2/core/bindings_utils.hpp>
	#include <opencv2/core/utils/logger.hpp>
#endif

namespace cv {
	static const char *CV_VERSION_OCVRS_OVERRIDE = CV_VERSION;
}
