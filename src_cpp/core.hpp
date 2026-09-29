#include "ocvrs_common.hpp"
#include <opencv2/core/affine.hpp>
#ifdef HAVE_OPENCL
	#include <opencv2/core/ocl.hpp>
	// opengl.hpp, va_intel.hpp and directx.hpp unconditionally include ocl.hpp thus it needs to be within ifdef HAVE_OPENCL
	#ifdef HAVE_OPENGL
		#include <opencv2/core/opengl.hpp>
	#endif
	#include <opencv2/core/va_intel.hpp>
	#include <opencv2/core/directx.hpp>
#endif
#ifdef HAVE_CUDA
	#include <opencv2/core/cuda.hpp>
#endif
#if __has_include(<opencv2/core/bindings_utils.hpp>) /* 3.4.4+ */
	#include <opencv2/core/bindings_utils.hpp>
#endif
#include <opencv2/core/utils/logger.hpp>
#if __has_include(<opencv2/core/async.hpp>) /* 3.4.7+, 4.1.1+ */
	#include <opencv2/core/async.hpp>
#endif

namespace cv {
	[[maybe_unused]] static const char *CV_VERSION_OCVRS_OVERRIDE = CV_VERSION;
}
