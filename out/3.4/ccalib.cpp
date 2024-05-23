#include "ccalib.hpp"
#include "ccalib_types.hpp"

extern "C" {
	// cv::omnidir::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:176
	// ("cv::omnidir::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "size", "K", "xi", "D", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* xi, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::calibrate(*objectPoints, *imagePoints, *size, *K, *xi, *D, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrate(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria, OutputArray)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:176
	// ("cv::omnidir::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "size", "K", "xi", "D", "rvecs", "tvecs", "flags", "criteria", "idx"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* xi, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, const cv::_OutputArray* idx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::calibrate(*objectPoints, *imagePoints, *size, *K, *xi, *D, *rvecs, *tvecs, flags, *criteria, *idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:141
	// ("cv::omnidir::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "xi", "R", "P", "size", "m1type", "map1", "map2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::initUndistortRectifyMap(*K, *D, *xi, *R, *P, *size, m1type, *map1, *map2, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::omnidir::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:107
	// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "xi", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "double", "const cv::_InputArray*"]), _)]),
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, double xi, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *affine, *K, xi, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, double, InputArray, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:107
	// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "xi", "D", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, double xi, const cv::_InputArray* D, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *affine, *K, xi, *D, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::omnidir::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:103
	// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "xi", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*"]), _)]),
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, double xi, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, xi, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, double, InputArray, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:103
	// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "xi", "D", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, double xi, const cv::_InputArray* D, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, xi, *D, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::omnidir::stereoCalibrate(InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:207
	// ("cv::omnidir::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "imageSize1", "imageSize2", "K1", "xi1", "D1", "K2", "xi2", "D2", "rvec", "tvec", "rvecsL", "tvecsL", "flags", "criteria"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::Size*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputOutputArray* objectPoints, const cv::_InputOutputArray* imagePoints1, const cv::_InputOutputArray* imagePoints2, const cv::Size* imageSize1, const cv::Size* imageSize2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* xi1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* xi2, const cv::_InputOutputArray* D2, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* rvecsL, const cv::_OutputArray* tvecsL, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *imageSize1, *imageSize2, *K1, *xi1, *D1, *K2, *xi2, *D2, *rvec, *tvec, *rvecsL, *tvecsL, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, const Size &, const Size &, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria, OutputArray)(InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:207
	// ("cv::omnidir::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "imageSize1", "imageSize2", "K1", "xi1", "D1", "K2", "xi2", "D2", "rvec", "tvec", "rvecsL", "tvecsL", "flags", "criteria", "idx"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::Size*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputOutputArray* objectPoints, const cv::_InputOutputArray* imagePoints1, const cv::_InputOutputArray* imagePoints2, const cv::Size* imageSize1, const cv::Size* imageSize2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* xi1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* xi2, const cv::_InputOutputArray* D2, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* rvecsL, const cv::_OutputArray* tvecsL, int flags, cv::TermCriteria* criteria, const cv::_OutputArray* idx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *imageSize1, *imageSize2, *K1, *xi1, *D1, *K2, *xi2, *D2, *rvec, *tvec, *rvecsL, *tvecsL, flags, *criteria, *idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::omnidir::stereoReconstruct(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:243
	// ("cv::omnidir::stereoReconstruct", vec![(pred!(mut, ["image1", "image2", "K1", "D1", "xi1", "K2", "D2", "xi2", "R", "T", "flag", "numDisparities", "SADWindowSize", "disparity", "image1Rec", "image2Rec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image1, const cv::_InputArray* image2, const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* xi1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::_InputArray* xi2, const cv::_InputArray* R, const cv::_InputArray* T, int flag, int numDisparities, int SADWindowSize, const cv::_OutputArray* disparity, const cv::_OutputArray* image1Rec, const cv::_OutputArray* image2Rec, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::stereoReconstruct(*image1, *image2, *K1, *D1, *xi1, *K2, *D2, *xi2, *R, *T, flag, numDisparities, SADWindowSize, *disparity, *image1Rec, *image2Rec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoReconstruct(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, int, int, int, OutputArray, OutputArray, OutputArray, const Size &, InputArray, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray, SimpleClass, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:243
	// ("cv::omnidir::stereoReconstruct", vec![(pred!(mut, ["image1", "image2", "K1", "D1", "xi1", "K2", "D2", "xi2", "R", "T", "flag", "numDisparities", "SADWindowSize", "disparity", "image1Rec", "image2Rec", "newSize", "Knew", "pointCloud", "pointType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* image1, const cv::_InputArray* image2, const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* xi1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::_InputArray* xi2, const cv::_InputArray* R, const cv::_InputArray* T, int flag, int numDisparities, int SADWindowSize, const cv::_OutputArray* disparity, const cv::_OutputArray* image1Rec, const cv::_OutputArray* image2Rec, const cv::Size* newSize, const cv::_InputArray* Knew, const cv::_OutputArray* pointCloud, int pointType, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::stereoReconstruct(*image1, *image2, *K1, *D1, *xi1, *K2, *D2, *xi2, *R, *T, flag, numDisparities, SADWindowSize, *disparity, *image1Rec, *image2Rec, *newSize, *Knew, *pointCloud, pointType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectify(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:218
	// ("cv::omnidir::stereoRectify", vec![(pred!(mut, ["R", "T", "R1", "R2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::stereoRectify(*R, *T, *R1, *R2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::omnidir::undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:156
	// ("cv::omnidir::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::undistortImage(*distorted, *undistorted, *K, *D, *xi, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, int, InputArray, const Size &, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:156
	// ("cv::omnidir::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "flags", "Knew", "new_size", "R"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*"]), _)]),
	void cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, int flags, const cv::_InputArray* Knew, const cv::Size* new_size, const cv::_InputArray* R, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::undistortImage(*distorted, *undistorted, *K, *D, *xi, flags, *Knew, *new_size, *R);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/omnidir.hpp:122
	// ("cv::omnidir::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "R"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R, ResultVoid* ocvrs_return) {
		try {
			cv::omnidir::undistortPoints(*distorted, *undistorted, *K, *D, *xi, *R);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CustomPattern()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:63
	// ("cv::ccalib::CustomPattern::CustomPattern", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_CustomPattern(Result<cv::ccalib::CustomPattern*>* ocvrs_return) {
		try {
			cv::ccalib::CustomPattern* ret = new cv::ccalib::CustomPattern();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray, const Size2f, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:66
	// ("cv::ccalib::CustomPattern::create", vec![(pred!(mut, ["pattern", "boardSize", "output"], ["const cv::_InputArray*", "const cv::Size2f", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* pattern, const cv::Size2f* boardSize, const cv::_OutputArray* output, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(*pattern, *boardSize, *output);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::create(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:66
	// ("cv::ccalib::CustomPattern::create", vec![(pred!(mut, ["pattern", "boardSize"], ["const cv::_InputArray*", "const cv::Size2f"]), _)]),
	void cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f(cv::ccalib::CustomPattern* instance, const cv::_InputArray* pattern, const cv::Size2f* boardSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(*pattern, *boardSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findPattern(InputArray, OutputArray, OutputArray, const double, const double, const bool, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:68
	// ("cv::ccalib::CustomPattern::findPattern", vec![(pred!(mut, ["image", "matched_features", "pattern_points", "ratio", "proj_error", "refine_position", "out", "H", "pattern_corners"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const double", "const double", "const bool", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const_double_const_double_const_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_OutputArray* matched_features, const cv::_OutputArray* pattern_points, const double ratio, const double proj_error, const bool refine_position, const cv::_OutputArray* out, const cv::_OutputArray* H, const cv::_OutputArray* pattern_corners, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findPattern(*image, *matched_features, *pattern_points, ratio, proj_error, refine_position, *out, *H, *pattern_corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::findPattern(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:68
	// ("cv::ccalib::CustomPattern::findPattern", vec![(pred!(mut, ["image", "matched_features", "pattern_points"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_OutputArray* matched_features, const cv::_OutputArray* pattern_points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findPattern(*image, *matched_features, *pattern_points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isInitialized()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:72
	// ("cv::ccalib::CustomPattern::isInitialized", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_isInitialized(cv::ccalib::CustomPattern* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInitialized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatternPoints(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:74
	// ("cv::ccalib::CustomPattern::getPatternPoints", vec![(pred!(mut, ["original_points"], ["std::vector<cv::KeyPoint>*"]), _)]),
	void cv_ccalib_CustomPattern_getPatternPoints_vectorLKeyPointGR(cv::ccalib::CustomPattern* instance, std::vector<cv::KeyPoint>* original_points, ResultVoid* ocvrs_return) {
		try {
			instance->getPatternPoints(*original_points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPixelSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:78
	// ("cv::ccalib::CustomPattern::getPixelSize", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_getPixelSize(cv::ccalib::CustomPattern* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFeatureDetector(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:83
	// ("cv::ccalib::CustomPattern::setFeatureDetector", vec![(pred!(mut, ["featureDetector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_ccalib_CustomPattern_setFeatureDetector_PtrLFeature2DG(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* featureDetector, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setFeatureDetector(*featureDetector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorExtractor(Ptr<DescriptorExtractor>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:84
	// ("cv::ccalib::CustomPattern::setDescriptorExtractor", vec![(pred!(mut, ["extractor"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_ccalib_CustomPattern_setDescriptorExtractor_PtrLFeature2DG(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* extractor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setDescriptorExtractor(*extractor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorMatcher(Ptr<DescriptorMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:85
	// ("cv::ccalib::CustomPattern::setDescriptorMatcher", vec![(pred!(mut, ["matcher"], ["cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	void cv_ccalib_CustomPattern_setDescriptorMatcher_PtrLDescriptorMatcherG(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setDescriptorMatcher(*matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:87
	// ("cv::ccalib::CustomPattern::getFeatureDetector", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_getFeatureDetector(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getFeatureDetector();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorExtractor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:88
	// ("cv::ccalib::CustomPattern::getDescriptorExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_getDescriptorExtractor(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getDescriptorExtractor();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:89
	// ("cv::ccalib::CustomPattern::getDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_getDescriptorMatcher(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->getDescriptorMatcher();
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrate(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:91
	// ("cv::ccalib::CustomPattern::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calibrate(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:91
	// ("cv::ccalib::CustomPattern::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calibrate(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findRt(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:99
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::findRt(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:99
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findRt(InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:101
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::findRt(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:101
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findRtRANSAC(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, int, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:108
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "minInliersCount", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::findRtRANSAC(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:108
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findRtRANSAC(InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, int, OutputArray, int)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:111
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "minInliersCount", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::findRtRANSAC(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:111
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawOrientation(InputOutputArray, InputArray, InputArray, InputArray, InputArray, double, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:118
	// ("cv::ccalib::CustomPattern::drawOrientation", vec![(pred!(mut, ["image", "tvec", "rvec", "cameraMatrix", "distCoeffs", "axis_length", "axis_width"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "int"]), _)]),
	void cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(cv::ccalib::CustomPattern* instance, const cv::_InputOutputArray* image, const cv::_InputArray* tvec, const cv::_InputArray* rvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, double axis_length, int axis_width, ResultVoid* ocvrs_return) {
		try {
			instance->drawOrientation(*image, *tvec, *rvec, *cameraMatrix, *distCoeffs, axis_length, axis_width);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::drawOrientation(InputOutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib.hpp:118
	// ("cv::ccalib::CustomPattern::drawOrientation", vec![(pred!(mut, ["image", "tvec", "rvec", "cameraMatrix", "distCoeffs"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputOutputArray* image, const cv::_InputArray* tvec, const cv::_InputArray* rvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, ResultVoid* ocvrs_return) {
		try {
			instance->drawOrientation(*image, *tvec, *rvec, *cameraMatrix, *distCoeffs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccalib::CustomPattern::to_Algorithm() generated
	// ("cv::ccalib::CustomPattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ccalib_CustomPattern_to_Algorithm(cv::ccalib::CustomPattern* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ccalib::CustomPattern::delete() generated
	// ("cv::ccalib::CustomPattern::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ccalib_CustomPattern_delete(cv::ccalib::CustomPattern* instance) {
			delete instance;
	}

	// MultiCameraCalibration(int, int, const std::string &, float, float, int, int, int, int, TermCriteria, Ptr<FeatureDetector>, Ptr<DescriptorExtractor>, Ptr<DescriptorMatcher>)(Primitive, Primitive, InString, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:132
	// ("cv::multicalib::MultiCameraCalibration::MultiCameraCalibration", vec![(pred!(mut, ["cameraType", "nCameras", "fileName", "patternWidth", "patternHeight", "verbose", "showExtration", "nMiniMatches", "flags", "criteria", "detector", "descriptor", "matcher"], ["int", "int", "const std::string*", "float", "float", "int", "int", "int", "int", "cv::TermCriteria", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	void cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_PtrLFeature2DG_PtrLFeature2DG_PtrLDescriptorMatcherG(int cameraType, int nCameras, const char* fileName, float patternWidth, float patternHeight, int verbose, int showExtration, int nMiniMatches, int flags, cv::TermCriteria* criteria, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<cv::multicalib::MultiCameraCalibration*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration* ret = new cv::multicalib::MultiCameraCalibration(cameraType, nCameras, std::string(fileName), patternWidth, patternHeight, verbose, showExtration, nMiniMatches, flags, *criteria, *detector, *descriptor, *matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::multicalib::MultiCameraCalibration::MultiCameraCalibration(Primitive, Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:132
	// ("cv::multicalib::MultiCameraCalibration::MultiCameraCalibration", vec![(pred!(mut, ["cameraType", "nCameras", "fileName", "patternWidth", "patternHeight"], ["int", "int", "const std::string*", "float", "float"]), _)]),
	void cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float(int cameraType, int nCameras, const char* fileName, float patternWidth, float patternHeight, Result<cv::multicalib::MultiCameraCalibration*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration* ret = new cv::multicalib::MultiCameraCalibration(cameraType, nCameras, std::string(fileName), patternWidth, patternHeight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadImages()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:141
	// ("cv::multicalib::MultiCameraCalibration::loadImages", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_loadImages(cv::multicalib::MultiCameraCalibration* instance, ResultVoid* ocvrs_return) {
		try {
			instance->loadImages();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:145
	// ("cv::multicalib::MultiCameraCalibration::initialize", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_initialize(cv::multicalib::MultiCameraCalibration* instance, ResultVoid* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// optimizeExtrinsics()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:149
	// ("cv::multicalib::MultiCameraCalibration::optimizeExtrinsics", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(cv::multicalib::MultiCameraCalibration* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->optimizeExtrinsics();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:153
	// ("cv::multicalib::MultiCameraCalibration::run", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_run(cv::multicalib::MultiCameraCalibration* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeParameters(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:157
	// ("cv::multicalib::MultiCameraCalibration::writeParameters", vec![(pred!(mut, ["filename"], ["const std::string*"]), _)]),
	void cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(cv::multicalib::MultiCameraCalibration* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->writeParameters(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::multicalib::MultiCameraCalibration::delete() generated
	// ("cv::multicalib::MultiCameraCalibration::delete", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_delete(cv::multicalib::MultiCameraCalibration* instance) {
			delete instance;
	}

	// edge(int, int, int, Mat)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:89
	// ("cv::multicalib::MultiCameraCalibration::edge::edge", vec![(pred!(mut, ["cv", "pv", "pi", "trans"], ["int", "int", "int", "cv::Mat"]), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(int cv, int pv, int pi, cv::Mat* trans, Result<cv::multicalib::MultiCameraCalibration::edge*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::edge* ret = new cv::multicalib::MultiCameraCalibration::edge(cv, pv, pi, *trans);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::multicalib::MultiCameraCalibration::edge::cameraVertex() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:84
	// ("cv::multicalib::MultiCameraCalibration::edge::cameraVertex", vec![(pred!(const, [], []), _)]),
	int cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->cameraVertex;
			return ret;
	}

	// cv::multicalib::MultiCameraCalibration::edge::setCameraVertex(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:84
	// ("cv::multicalib::MultiCameraCalibration::edge::setCameraVertex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_const_int(cv::multicalib::MultiCameraCalibration::edge* instance, const int val) {
			instance->cameraVertex = val;
	}

	// cv::multicalib::MultiCameraCalibration::edge::photoVertex() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:85
	// ("cv::multicalib::MultiCameraCalibration::edge::photoVertex", vec![(pred!(const, [], []), _)]),
	int cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->photoVertex;
			return ret;
	}

	// cv::multicalib::MultiCameraCalibration::edge::setPhotoVertex(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:85
	// ("cv::multicalib::MultiCameraCalibration::edge::setPhotoVertex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_const_int(cv::multicalib::MultiCameraCalibration::edge* instance, const int val) {
			instance->photoVertex = val;
	}

	// cv::multicalib::MultiCameraCalibration::edge::photoIndex() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:86
	// ("cv::multicalib::MultiCameraCalibration::edge::photoIndex", vec![(pred!(const, [], []), _)]),
	int cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->photoIndex;
			return ret;
	}

	// cv::multicalib::MultiCameraCalibration::edge::setPhotoIndex(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:86
	// ("cv::multicalib::MultiCameraCalibration::edge::setPhotoIndex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_const_int(cv::multicalib::MultiCameraCalibration::edge* instance, const int val) {
			instance->photoIndex = val;
	}

	// cv::multicalib::MultiCameraCalibration::edge::transform() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:87
	// ("cv::multicalib::MultiCameraCalibration::edge::transform", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_multicalib_MultiCameraCalibration_edge_propTransform_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			cv::Mat ret = instance->transform;
			return new cv::Mat(ret);
	}

	// cv::multicalib::MultiCameraCalibration::edge::setTransform(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:87
	// ("cv::multicalib::MultiCameraCalibration::edge::setTransform", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_propTransform_const_Mat(cv::multicalib::MultiCameraCalibration::edge* instance, const cv::Mat* val) {
			instance->transform = *val;
	}

	// cv::multicalib::MultiCameraCalibration::edge::delete() generated
	// ("cv::multicalib::MultiCameraCalibration::edge::delete", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_edge_delete(cv::multicalib::MultiCameraCalibration::edge* instance) {
			delete instance;
	}

	// vertex(Mat, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:105
	// ("cv::multicalib::MultiCameraCalibration::vertex::vertex", vec![(pred!(mut, ["po", "ts"], ["cv::Mat", "int"]), _)]),
	void cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(cv::Mat* po, int ts, Result<cv::multicalib::MultiCameraCalibration::vertex*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex(*po, ts);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vertex()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:111
	// ("cv::multicalib::MultiCameraCalibration::vertex::vertex", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_vertex_vertex(Result<cv::multicalib::MultiCameraCalibration::vertex*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::multicalib::MultiCameraCalibration::vertex::pose() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:100
	// ("cv::multicalib::MultiCameraCalibration::vertex::pose", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_multicalib_MultiCameraCalibration_vertex_propPose_const(const cv::multicalib::MultiCameraCalibration::vertex* instance) {
			cv::Mat ret = instance->pose;
			return new cv::Mat(ret);
	}

	// cv::multicalib::MultiCameraCalibration::vertex::setPose(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:100
	// ("cv::multicalib::MultiCameraCalibration::vertex::setPose", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_multicalib_MultiCameraCalibration_vertex_propPose_const_Mat(cv::multicalib::MultiCameraCalibration::vertex* instance, const cv::Mat* val) {
			instance->pose = *val;
	}

	// cv::multicalib::MultiCameraCalibration::vertex::timestamp() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:103
	// ("cv::multicalib::MultiCameraCalibration::vertex::timestamp", vec![(pred!(const, [], []), _)]),
	int cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_const(const cv::multicalib::MultiCameraCalibration::vertex* instance) {
			int ret = instance->timestamp;
			return ret;
	}

	// cv::multicalib::MultiCameraCalibration::vertex::setTimestamp(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/multicalib.hpp:103
	// ("cv::multicalib::MultiCameraCalibration::vertex::setTimestamp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_const_int(cv::multicalib::MultiCameraCalibration::vertex* instance, const int val) {
			instance->timestamp = val;
	}

	// cv::multicalib::MultiCameraCalibration::vertex::delete() generated
	// ("cv::multicalib::MultiCameraCalibration::vertex::delete", vec![(pred!(mut, [], []), _)]),
	void cv_multicalib_MultiCameraCalibration_vertex_delete(cv::multicalib::MultiCameraCalibration::vertex* instance) {
			delete instance;
	}

	// RandomPatternCornerFinder(float, float, int, int, int, int, Ptr<FeatureDetector>, Ptr<DescriptorExtractor>, Ptr<DescriptorMatcher>)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:80
	// ("cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder", vec![(pred!(mut, ["patternWidth", "patternHeight", "nminiMatch", "depth", "verbose", "showExtraction", "detector", "descriptor", "matcher"], ["float", "float", "int", "int", "int", "int", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_PtrLFeature2DG_PtrLFeature2DG_PtrLDescriptorMatcherG(float patternWidth, float patternHeight, int nminiMatch, int depth, int verbose, int showExtraction, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<cv::randpattern::RandomPatternCornerFinder*>* ocvrs_return) {
		try {
			cv::randpattern::RandomPatternCornerFinder* ret = new cv::randpattern::RandomPatternCornerFinder(patternWidth, patternHeight, nminiMatch, depth, verbose, showExtraction, *detector, *descriptor, *matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:80
	// ("cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder", vec![(pred!(mut, ["patternWidth", "patternHeight"], ["float", "float"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float(float patternWidth, float patternHeight, Result<cv::randpattern::RandomPatternCornerFinder*>* ocvrs_return) {
		try {
			cv::randpattern::RandomPatternCornerFinder* ret = new cv::randpattern::RandomPatternCornerFinder(patternWidth, patternHeight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadPattern(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:89
	// ("cv::randpattern::RandomPatternCornerFinder::loadPattern", vec![(pred!(mut, ["patternImage"], ["const cv::Mat*"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(cv::randpattern::RandomPatternCornerFinder* instance, const cv::Mat* patternImage, ResultVoid* ocvrs_return) {
		try {
			instance->loadPattern(*patternImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadPattern(const cv::Mat &, const std::vector<cv::KeyPoint> &, const cv::Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:96
	// ("cv::randpattern::RandomPatternCornerFinder::loadPattern", vec![(pred!(mut, ["patternImage", "patternKeyPoints", "patternDescriptors"], ["const cv::Mat*", "const std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vectorLKeyPointGR_const_MatR(cv::randpattern::RandomPatternCornerFinder* instance, const cv::Mat* patternImage, const std::vector<cv::KeyPoint>* patternKeyPoints, const cv::Mat* patternDescriptors, ResultVoid* ocvrs_return) {
		try {
			instance->loadPattern(*patternImage, *patternKeyPoints, *patternDescriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeObjectImagePoints(std::vector<cv::Mat>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:105
	// ("cv::randpattern::RandomPatternCornerFinder::computeObjectImagePoints", vec![(pred!(mut, ["inputImages"], ["std::vector<cv::Mat>"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vectorLMatG(cv::randpattern::RandomPatternCornerFinder* instance, std::vector<cv::Mat>* inputImages, ResultVoid* ocvrs_return) {
		try {
			instance->computeObjectImagePoints(*inputImages);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeObjectImagePointsForSingle(cv::Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:114
	// ("cv::randpattern::RandomPatternCornerFinder::computeObjectImagePointsForSingle", vec![(pred!(mut, ["inputImage"], ["cv::Mat"]), _)]),
	void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(cv::randpattern::RandomPatternCornerFinder* instance, cv::Mat* inputImage, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->computeObjectImagePointsForSingle(*inputImage);
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjectPoints()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:118
	// ("cv::randpattern::RandomPatternCornerFinder::getObjectPoints", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternCornerFinder_getObjectPoints(cv::randpattern::RandomPatternCornerFinder* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getObjectPoints();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImagePoints()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:122
	// ("cv::randpattern::RandomPatternCornerFinder::getImagePoints", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternCornerFinder_getImagePoints(cv::randpattern::RandomPatternCornerFinder* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getImagePoints();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randpattern::RandomPatternCornerFinder::delete() generated
	// ("cv::randpattern::RandomPatternCornerFinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternCornerFinder_delete(cv::randpattern::RandomPatternCornerFinder* instance) {
			delete instance;
	}

	// RandomPatternGenerator(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:168
	// ("cv::randpattern::RandomPatternGenerator::RandomPatternGenerator", vec![(pred!(mut, ["imageWidth", "imageHeight"], ["int", "int"]), _)]),
	void cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(int imageWidth, int imageHeight, Result<cv::randpattern::RandomPatternGenerator*>* ocvrs_return) {
		try {
			cv::randpattern::RandomPatternGenerator* ret = new cv::randpattern::RandomPatternGenerator(imageWidth, imageHeight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generatePattern()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:172
	// ("cv::randpattern::RandomPatternGenerator::generatePattern", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternGenerator_generatePattern(cv::randpattern::RandomPatternGenerator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->generatePattern();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPattern()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ccalib/randpattern.hpp:175
	// ("cv::randpattern::RandomPatternGenerator::getPattern", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternGenerator_getPattern(cv::randpattern::RandomPatternGenerator* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPattern();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randpattern::RandomPatternGenerator::delete() generated
	// ("cv::randpattern::RandomPatternGenerator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_randpattern_RandomPatternGenerator_delete(cv::randpattern::RandomPatternGenerator* instance) {
			delete instance;
	}

}
