#include "common.hpp"
#include <opencv2/rapid.hpp>
#include "rapid_types.hpp"

extern "C" {
	Result_void cv_rapid_drawCorrespondencies_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(const cv::_InputOutputArray* bundle, const cv::_InputArray* srcLocations, const cv::_InputArray* newLocations, const cv::_InputArray* colors) {
		try {
			cv::rapid::drawCorrespondencies(*bundle, *srcLocations, *newLocations, *colors);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_drawSearchLines_const__InputOutputArrayX_const__InputArrayX_const_ScalarX(const cv::_InputOutputArray* img, const cv::_InputArray* locations, const cv::Scalar* color) {
		try {
			cv::rapid::drawSearchLines(*img, *locations, *color);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_drawWireframe_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const_ScalarX_int_bool(const cv::_InputOutputArray* img, const cv::_InputArray* pts2d, const cv::_InputArray* tris, const cv::Scalar* color, int type, bool cullBackface) {
		try {
			cv::rapid::drawWireframe(*img, *pts2d, *tris, *color, type, cullBackface);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_extractControlPoints_int_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::Size* imsize, const cv::_InputArray* tris, const cv::_OutputArray* ctl2d, const cv::_OutputArray* ctl3d) {
		try {
			cv::rapid::extractControlPoints(num, len, *pts3d, *rvec, *tvec, *K, *imsize, *tris, *ctl2d, *ctl3d);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_extractLineBundle_int_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(int len, const cv::_InputArray* ctl2d, const cv::_InputArray* img, const cv::_OutputArray* bundle, const cv::_OutputArray* srcLocations) {
		try {
			cv::rapid::extractLineBundle(len, *ctl2d, *img, *bundle, *srcLocations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_filterCorrespondencies_const__InputOutputArrayX_const__InputOutputArrayX_const__InputArrayX(const cv::_InputOutputArray* pts2d, const cv::_InputOutputArray* pts3d, const cv::_InputArray* mask) {
		try {
			cv::rapid::filterCorrespondencies(*pts2d, *pts3d, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rapid_findCorrespondencies_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::_InputArray* bundle, const cv::_InputArray* srcLocations, const cv::_OutputArray* newLocations, const cv::_OutputArray* response) {
		try {
			cv::rapid::findCorrespondencies(*bundle, *srcLocations, *newLocations, *response);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_rapid_rapid_const__InputArrayX_int_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX(const cv::_InputArray* img, int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* tris, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec) {
		try {
			float ret = cv::rapid::rapid(*img, num, len, *pts3d, *tris, *K, *rvec, *tvec);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
}
