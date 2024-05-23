#include "ocvrs_common.hpp"
#include <opencv2/rapid.hpp>
#include "rapid_types.hpp"

extern "C" {
	// cv::rapid::convertCorrespondencies(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:100
	// ("cv::rapid::convertCorrespondencies", vec![(pred!(mut, ["cols", "srcLocations", "pts2d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* cols, const cv::_InputArray* srcLocations, const cv::_OutputArray* pts2d, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::convertCorrespondencies(*cols, *srcLocations, *pts2d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertCorrespondencies(InputArray, InputArray, OutputArray, InputOutputArray, InputArray)(InputArray, InputArray, OutputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:100
	// ("cv::rapid::convertCorrespondencies", vec![(pred!(mut, ["cols", "srcLocations", "pts2d", "pts3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* cols, const cv::_InputArray* srcLocations, const cv::_OutputArray* pts2d, const cv::_InputOutputArray* pts3d, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::convertCorrespondencies(*cols, *srcLocations, *pts2d, *pts3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::drawCorrespondencies(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:30
	// ("cv::rapid::drawCorrespondencies", vec![(pred!(mut, ["bundle", "cols"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* bundle, const cv::_InputArray* cols, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::drawCorrespondencies(*bundle, *cols);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawCorrespondencies(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:30
	// ("cv::rapid::drawCorrespondencies", vec![(pred!(mut, ["bundle", "cols", "colors"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* bundle, const cv::_InputArray* cols, const cv::_InputArray* colors, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::drawCorrespondencies(*bundle, *cols, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawSearchLines(InputOutputArray, InputArray, const Scalar &)(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:39
	// ("cv::rapid::drawSearchLines", vec![(pred!(mut, ["img", "locations", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* locations, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::drawSearchLines(*img, *locations, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::drawWireframe(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:50
	// ("cv::rapid::drawWireframe", vec![(pred!(mut, ["img", "pts2d", "tris", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* pts2d, const cv::_InputArray* tris, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::drawWireframe(*img, *pts2d, *tris, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawWireframe(InputOutputArray, InputArray, InputArray, const Scalar &, int, bool)(InputOutputArray, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:50
	// ("cv::rapid::drawWireframe", vec![(pred!(mut, ["img", "pts2d", "tris", "color", "type", "cullBackface"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "bool"]), _)]),
	void cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(const cv::_InputOutputArray* img, const cv::_InputArray* pts2d, const cv::_InputArray* tris, const cv::Scalar* color, int type, bool cullBackface, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::drawWireframe(*img, *pts2d, *tris, *color, type, cullBackface);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractControlPoints(int, int, InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, OutputArray, OutputArray)(Primitive, Primitive, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:67
	// ("cv::rapid::extractControlPoints", vec![(pred!(mut, ["num", "len", "pts3d", "rvec", "tvec", "K", "imsize", "tris", "ctl2d", "ctl3d"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::Size* imsize, const cv::_InputArray* tris, const cv::_OutputArray* ctl2d, const cv::_OutputArray* ctl3d, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::extractControlPoints(num, len, *pts3d, *rvec, *tvec, *K, *imsize, *tris, *ctl2d, *ctl3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractLineBundle(int, InputArray, InputArray, OutputArray, OutputArray)(Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:79
	// ("cv::rapid::extractLineBundle", vec![(pred!(mut, ["len", "ctl2d", "img", "bundle", "srcLocations"], ["int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int len, const cv::_InputArray* ctl2d, const cv::_InputArray* img, const cv::_OutputArray* bundle, const cv::_OutputArray* srcLocations, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::extractLineBundle(len, *ctl2d, *img, *bundle, *srcLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::findCorrespondencies(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:89
	// ("cv::rapid::findCorrespondencies", vec![(pred!(mut, ["bundle", "cols"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* bundle, const cv::_OutputArray* cols, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::findCorrespondencies(*bundle, *cols);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCorrespondencies(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:89
	// ("cv::rapid::findCorrespondencies", vec![(pred!(mut, ["bundle", "cols", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* bundle, const cv::_OutputArray* cols, const cv::_OutputArray* response, ResultVoid* ocvrs_return) {
		try {
			cv::rapid::findCorrespondencies(*bundle, *cols, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::rapid(InputArray, Primitive, Primitive, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:123
	// ("cv::rapid::rapid", vec![(pred!(mut, ["img", "num", "len", "pts3d", "tris", "K", "rvec", "tvec"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* img, int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* tris, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, Result<float>* ocvrs_return) {
		try {
			float ret = cv::rapid::rapid(*img, num, len, *pts3d, *tris, *K, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rapid(InputArray, int, int, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, double *)(InputArray, Primitive, Primitive, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:123
	// ("cv::rapid::rapid", vec![(pred!(mut, ["img", "num", "len", "pts3d", "tris", "K", "rvec", "tvec", "rmsd"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "double*"]), _)]),
	void cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(const cv::_InputArray* img, int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* tris, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, double* rmsd, Result<float>* ocvrs_return) {
		try {
			float ret = cv::rapid::rapid(*img, num, len, *pts3d, *tris, *K, *rvec, *tvec, rmsd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray, InputArray, int, uchar)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:158
	// ("cv::rapid::GOSTracker::create", vec![(pred!(mut, ["pts3d", "tris", "histBins", "sobelThesh"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "unsigned char"]), _)]),
	void cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(const cv::_InputArray* pts3d, const cv::_InputArray* tris, int histBins, unsigned char sobelThesh, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::GOSTracker::create(*pts3d, *tris, histBins, sobelThesh);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::GOSTracker::create(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:158
	// ("cv::rapid::GOSTracker::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pts3d, const cv::_InputArray* tris, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::GOSTracker::create(*pts3d, *tris);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::GOSTracker::to_Algorithm() generated
	// ("cv::rapid::GOSTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rapid_GOSTracker_to_Algorithm(cv::rapid::GOSTracker* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rapid::GOSTracker::to_Rapid_Tracker() generated
	// ("cv::rapid::GOSTracker::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Tracker* cv_rapid_GOSTracker_to_Rapid_Tracker(cv::rapid::GOSTracker* instance) {
			return dynamic_cast<cv::rapid::Tracker*>(instance);
	}

	// cv::rapid::GOSTracker::delete() generated
	// ("cv::rapid::GOSTracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rapid_GOSTracker_delete(cv::rapid::GOSTracker* instance) {
			delete instance;
	}

	// create(InputArray, InputArray, int, uchar)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:150
	// ("cv::rapid::OLSTracker::create", vec![(pred!(mut, ["pts3d", "tris", "histBins", "sobelThesh"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "unsigned char"]), _)]),
	void cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(const cv::_InputArray* pts3d, const cv::_InputArray* tris, int histBins, unsigned char sobelThesh, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::OLSTracker::create(*pts3d, *tris, histBins, sobelThesh);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::OLSTracker::create(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:150
	// ("cv::rapid::OLSTracker::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pts3d, const cv::_InputArray* tris, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::OLSTracker::create(*pts3d, *tris);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::OLSTracker::to_Algorithm() generated
	// ("cv::rapid::OLSTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rapid_OLSTracker_to_Algorithm(cv::rapid::OLSTracker* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rapid::OLSTracker::to_Rapid_Tracker() generated
	// ("cv::rapid::OLSTracker::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Tracker* cv_rapid_OLSTracker_to_Rapid_Tracker(cv::rapid::OLSTracker* instance) {
			return dynamic_cast<cv::rapid::Tracker*>(instance);
	}

	// cv::rapid::OLSTracker::delete() generated
	// ("cv::rapid::OLSTracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rapid_OLSTracker_delete(cv::rapid::OLSTracker* instance) {
			delete instance;
	}

	// create(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:141
	// ("cv::rapid::Rapid::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pts3d, const cv::_InputArray* tris, Result<cv::Ptr<cv::rapid::Rapid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::Rapid> ret = cv::rapid::Rapid::create(*pts3d, *tris);
			Ok(new cv::Ptr<cv::rapid::Rapid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::Rapid::to_Algorithm() generated
	// ("cv::rapid::Rapid::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rapid_Rapid_to_Algorithm(cv::rapid::Rapid* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rapid::Rapid::to_Rapid_Tracker() generated
	// ("cv::rapid::Rapid::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Tracker* cv_rapid_Rapid_to_Rapid_Tracker(cv::rapid::Rapid* instance) {
			return dynamic_cast<cv::rapid::Tracker*>(instance);
	}

	// cv::rapid::Rapid::delete() generated
	// ("cv::rapid::Rapid::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rapid_Rapid_delete(cv::rapid::Rapid* instance) {
			delete instance;
	}

	// compute(InputArray, int, int, InputArray, InputOutputArray, InputOutputArray, const TermCriteria &)(InputArray, Primitive, Primitive, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:132
	// ("cv::rapid::Tracker::compute", vec![(pred!(mut, ["img", "num", "len", "K", "rvec", "tvec", "termcrit"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::TermCriteria*"]), _)]),
	void cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(cv::rapid::Tracker* instance, const cv::_InputArray* img, int num, int len, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, const cv::TermCriteria* termcrit, Result<float>* ocvrs_return) {
		try {
			float ret = instance->compute(*img, num, len, *K, *rvec, *tvec, *termcrit);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::Tracker::compute(InputArray, Primitive, Primitive, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:132
	// ("cv::rapid::Tracker::compute", vec![(pred!(mut, ["img", "num", "len", "K", "rvec", "tvec"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(cv::rapid::Tracker* instance, const cv::_InputArray* img, int num, int len, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, Result<float>* ocvrs_return) {
		try {
			float ret = instance->compute(*img, num, len, *K, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearState()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rapid.hpp:134
	// ("cv::rapid::Tracker::clearState", vec![(pred!(mut, [], []), _)]),
	void cv_rapid_Tracker_clearState(cv::rapid::Tracker* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearState();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rapid::Tracker::to_Rapid_GOSTracker() generated
	// ("cv::rapid::Tracker::to_Rapid_GOSTracker", vec![(pred!(mut, [], []), _)]),
	cv::rapid::GOSTracker* cv_rapid_Tracker_to_Rapid_GOSTracker(cv::rapid::Tracker* instance) {
			return dynamic_cast<cv::rapid::GOSTracker*>(instance);
	}

	// cv::rapid::Tracker::to_Rapid_OLSTracker() generated
	// ("cv::rapid::Tracker::to_Rapid_OLSTracker", vec![(pred!(mut, [], []), _)]),
	cv::rapid::OLSTracker* cv_rapid_Tracker_to_Rapid_OLSTracker(cv::rapid::Tracker* instance) {
			return dynamic_cast<cv::rapid::OLSTracker*>(instance);
	}

	// cv::rapid::Tracker::to_Rapid_Rapid() generated
	// ("cv::rapid::Tracker::to_Rapid_Rapid", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Rapid* cv_rapid_Tracker_to_Rapid_Rapid(cv::rapid::Tracker* instance) {
			return dynamic_cast<cv::rapid::Rapid*>(instance);
	}

	// cv::rapid::Tracker::to_Algorithm() generated
	// ("cv::rapid::Tracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rapid_Tracker_to_Algorithm(cv::rapid::Tracker* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rapid::Tracker::delete() generated
	// ("cv::rapid::Tracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rapid_Tracker_delete(cv::rapid::Tracker* instance) {
			delete instance;
	}

}
