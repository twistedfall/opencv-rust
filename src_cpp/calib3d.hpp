#include "ocvrs_common.hpp"
#include <opencv2/calib3d.hpp>

namespace cv {
	// needed for calibrateCamera()
	OCVRS_ONLY_DEPENDENT_TYPES void dummy(
		std::vector<cv::Point3i>,
		std::vector<std::vector<cv::Point3i>>,
		std::vector<cv::Point3d>,
		std::vector<std::vector<cv::Point3d>>
	) {}
}
