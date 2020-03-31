#include "common.hpp"
#include <opencv2/intensity_transform.hpp>
#include "intensity_transform_types.hpp"

extern "C" {
	Result_void cv_intensity_transform_autoscaling_Mat_MatX(const cv::Mat* input, cv::Mat* output) {
		try {
			cv::intensity_transform::autoscaling(*input, *output);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_intensity_transform_contrastStretching_Mat_MatX_int_int_int_int(const cv::Mat* input, cv::Mat* output, int r1, int s1, int r2, int s2) {
		try {
			cv::intensity_transform::contrastStretching(*input, *output, r1, s1, r2, s2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_intensity_transform_gammaCorrection_Mat_MatX_float(const cv::Mat* input, cv::Mat* output, float gamma) {
		try {
			cv::intensity_transform::gammaCorrection(*input, *output, gamma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_intensity_transform_logTransform_Mat_MatX(const cv::Mat* input, cv::Mat* output) {
		try {
			cv::intensity_transform::logTransform(*input, *output);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
