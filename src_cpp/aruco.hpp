#include "common.hpp"
#include <opencv2/aruco.hpp>
#include <opencv2/aruco/charuco.hpp>

namespace cv {
	// needed for estimatePoseSingleMarkers()
	OCVRS_ONLY_DEPENDENT_TYPES void dummy(
		std::vector<cv::Vec3f>,
		std::vector<cv::Vec3d>
	) {}
}
