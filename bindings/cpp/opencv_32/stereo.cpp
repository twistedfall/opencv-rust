#include "common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	Result_void cv_stereo_censusTransform_const_MatX_const_MatX_int_MatX_MatX_int(const cv::Mat* image1, const cv::Mat* image2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, int type) {
		try {
			cv::stereo::censusTransform(*image1, *image2, kernelSize, *dist1, *dist2, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_censusTransform_const_MatX_int_MatX_int(const cv::Mat* image1, int kernelSize, cv::Mat* dist1, int type) {
		try {
			cv::stereo::censusTransform(*image1, kernelSize, *dist1, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_modifiedCensusTransform_const_MatX_const_MatX_int_MatX_MatX_int_int_const_MatX_const_MatX(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, int type, int t, const cv::Mat* IntegralImage1, const cv::Mat* IntegralImage2) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type, t, *IntegralImage1, *IntegralImage2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_modifiedCensusTransform_const_MatX_int_MatX_int_int_const_MatX(const cv::Mat* img1, int kernelSize, cv::Mat* dist, int type, int t, const cv::Mat* IntegralImage) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type, t, *IntegralImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_starCensusTransform_const_MatX_const_MatX_int_MatX_MatX(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2) {
		try {
			cv::stereo::starCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_starCensusTransform_const_MatX_int_MatX(const cv::Mat* img1, int kernelSize, cv::Mat* dist) {
		try {
			cv::stereo::starCensusTransform(*img1, kernelSize, *dist);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_symetricCensusTransform_const_MatX_const_MatX_int_MatX_MatX_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, int type) {
		try {
			cv::stereo::symetricCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_stereo_symetricCensusTransform_const_MatX_int_MatX_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist1, int type) {
		try {
			cv::stereo::symetricCensusTransform(*img1, kernelSize, *dist1, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
