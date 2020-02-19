#include "common.hpp"
#include <opencv2/imgproc.hpp>

namespace cv {
	// needed for findContours()
	OCVRS_ONLY_DEPENDENT_TYPES void dummy(
		std::vector<cv::Vec4i>
	) {}
}
