#include "ocvrs_common.hpp"
#include <opencv2/rapid.hpp>
#include "rapid_types.hpp"

extern "C" {
	Result_void cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* cols, const cv::_InputArray* srcLocations, const cv::_OutputArray* pts2d, const cv::_InputOutputArray* pts3d, const cv::_InputArray* mask) {
		try {
			cv::rapid::convertCorrespondencies(*cols, *srcLocations, *pts2d, *pts3d, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* bundle, const cv::_InputArray* cols, const cv::_InputArray* colors) {
		try {
			cv::rapid::drawCorrespondencies(*bundle, *cols, *colors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* locations, const cv::Scalar* color) {
		try {
			cv::rapid::drawSearchLines(*img, *locations, *color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(const cv::_InputOutputArray* img, const cv::_InputArray* pts2d, const cv::_InputArray* tris, const cv::Scalar* color, int type, bool cullBackface) {
		try {
			cv::rapid::drawWireframe(*img, *pts2d, *tris, *color, type, cullBackface);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::Size* imsize, const cv::_InputArray* tris, const cv::_OutputArray* ctl2d, const cv::_OutputArray* ctl3d) {
		try {
			cv::rapid::extractControlPoints(num, len, *pts3d, *rvec, *tvec, *K, *imsize, *tris, *ctl2d, *ctl3d);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int len, const cv::_InputArray* ctl2d, const cv::_InputArray* img, const cv::_OutputArray* bundle, const cv::_OutputArray* srcLocations) {
		try {
			cv::rapid::extractLineBundle(len, *ctl2d, *img, *bundle, *srcLocations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* bundle, const cv::_OutputArray* cols, const cv::_OutputArray* response) {
		try {
			cv::rapid::findCorrespondencies(*bundle, *cols, *response);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(const cv::_InputArray* img, int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* tris, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, double* rmsd) {
		try {
			float ret = cv::rapid::rapid(*img, num, len, *pts3d, *tris, *K, *rvec, *tvec, rmsd);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<cv::Ptr<cv::rapid::OLSTracker>*> cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(const cv::_InputArray* pts3d, const cv::_InputArray* tris, int histBins, unsigned char sobelThesh) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::OLSTracker::create(*pts3d, *tris, histBins, sobelThesh);
			return Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rapid::OLSTracker>*>))
	}
	
	Result<cv::Ptr<cv::rapid::Rapid>*> cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pts3d, const cv::_InputArray* tris) {
		try {
			cv::Ptr<cv::rapid::Rapid> ret = cv::rapid::Rapid::create(*pts3d, *tris);
			return Ok(new cv::Ptr<cv::rapid::Rapid>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rapid::Rapid>*>))
	}
	
	Result<float> cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(cv::rapid::Tracker* instance, const cv::_InputArray* img, int num, int len, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, const cv::TermCriteria* termcrit) {
		try {
			float ret = instance->compute(*img, num, len, *K, *rvec, *tvec, *termcrit);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_rapid_Tracker_clearState(cv::rapid::Tracker* instance) {
		try {
			instance->clearState();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
