#include "ocvrs_common.hpp"

#ifndef HAVE_CUDA
	// <opencv2/stitching.hpp> unconditionally includes a bunch of CUDA stuff without a proper way to disable it. That creates an
	// issue as we don't generate GpuMat when HAVE_CUDA is not defined. A workaround is a couple of hacks that prevent the forced
	// inclusion of <opencv2/core/cuda.hpp> and generation of functions that have GpuMat arguments.

	// prevent forced inclusion of the <opencv2/core/cuda.hpp> header
	#define OPENCV_CORE_CUDA_HPP

	// generate a minimal stub for GpuMat so that C++ code compiles, but because of the missing export macro the bindings for the
	// functions using this type will not be generated
	namespace cv {
		namespace cuda {
			class GpuMat {
				public:
				void upload(InputArray arr);
				void download(OutputArray dst) const;
			};
		}
	}
#endif

#include <opencv2/stitching.hpp>
