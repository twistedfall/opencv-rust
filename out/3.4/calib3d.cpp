#include "ocvrs_common.hpp"
#include <opencv2/calib3d.hpp>
#include "calib3d_types.hpp"

extern "C" {
	// cv::RQDecomp3x3(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:654
	// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mtxR, const cv::_OutputArray* mtxQ, Result<cv::Vec3d>* ocvrs_return) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*src, *mtxR, *mtxQ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RQDecomp3x3(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:654
	// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ", "Qx", "Qy", "Qz"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mtxR, const cv::_OutputArray* mtxQ, const cv::_OutputArray* Qx, const cv::_OutputArray* Qy, const cv::_OutputArray* Qz, Result<cv::Vec3d>* ocvrs_return) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*src, *mtxR, *mtxQ, *Qx, *Qy, *Qz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Rodrigues(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:559
	// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Rodrigues_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::Rodrigues(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Rodrigues(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:559
	// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::Rodrigues(*src, *dst, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1400
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1382
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1382
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1400
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateHandEye(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1905
	// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R_gripper2base, const cv::_InputArray* t_gripper2base, const cv::_InputArray* R_target2cam, const cv::_InputArray* t_target2cam, const cv::_OutputArray* R_cam2gripper, const cv::_OutputArray* t_cam2gripper, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateHandEye(*R_gripper2base, *t_gripper2base, *R_target2cam, *t_target2cam, *R_cam2gripper, *t_cam2gripper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateHandEye(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, HandEyeCalibrationMethod)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1905
	// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::HandEyeCalibrationMethod"]), _)]),
	void cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_HandEyeCalibrationMethod(const cv::_InputArray* R_gripper2base, const cv::_InputArray* t_gripper2base, const cv::_InputArray* R_target2cam, const cv::_InputArray* t_target2cam, const cv::_OutputArray* R_cam2gripper, const cv::_OutputArray* t_cam2gripper, cv::HandEyeCalibrationMethod method, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateHandEye(*R_gripper2base, *t_gripper2base, *R_target2cam, *t_target2cam, *R_cam2gripper, *t_cam2gripper, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrationMatrixValues(InputArray, Size, double, double, double &, double &, double &, Point2d &, double &)(InputArray, SimpleClass, Primitive, Primitive, Indirect, Indirect, Indirect, SimpleClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1427
	// ("cv::calibrationMatrixValues", vec![(pred!(mut, ["cameraMatrix", "imageSize", "apertureWidth", "apertureHeight", "fovx", "fovy", "focalLength", "principalPoint", "aspectRatio"], ["const cv::_InputArray*", "cv::Size", "double", "double", "double*", "double*", "double*", "cv::Point2d*", "double*"]), _)]),
	void cv_calibrationMatrixValues_const__InputArrayR_Size_double_double_doubleR_doubleR_doubleR_Point2dR_doubleR(const cv::_InputArray* cameraMatrix, cv::Size* imageSize, double apertureWidth, double apertureHeight, double* fovx, double* fovy, double* focalLength, cv::Point2d* principalPoint, double* aspectRatio, ResultVoid* ocvrs_return) {
		try {
			cv::calibrationMatrixValues(*cameraMatrix, *imageSize, apertureWidth, apertureHeight, *fovx, *fovy, *focalLength, *principalPoint, *aspectRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:732
	// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* rvec1, const cv::_InputArray* tvec1, const cv::_InputArray* rvec2, const cv::_InputArray* tvec2, const cv::_OutputArray* rvec3, const cv::_OutputArray* tvec3, ResultVoid* ocvrs_return) {
		try {
			cv::composeRT(*rvec1, *tvec1, *rvec2, *tvec2, *rvec3, *tvec3);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:732
	// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3", "dr3dr1", "dr3dt1", "dr3dr2", "dr3dt2", "dt3dr1", "dt3dt1", "dt3dr2", "dt3dt2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* rvec1, const cv::_InputArray* tvec1, const cv::_InputArray* rvec2, const cv::_InputArray* tvec2, const cv::_OutputArray* rvec3, const cv::_OutputArray* tvec3, const cv::_OutputArray* dr3dr1, const cv::_OutputArray* dr3dt1, const cv::_OutputArray* dr3dr2, const cv::_OutputArray* dr3dt2, const cv::_OutputArray* dt3dr1, const cv::_OutputArray* dt3dt1, const cv::_OutputArray* dt3dr2, const cv::_OutputArray* dt3dt2, ResultVoid* ocvrs_return) {
		try {
			cv::composeRT(*rvec1, *tvec1, *rvec2, *tvec2, *rvec3, *tvec3, *dr3dr1, *dr3dt1, *dr3dr2, *dr3dt2, *dt3dr1, *dt3dt1, *dt3dr2, *dt3dt2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeCorrespondEpilines(InputArray, int, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2255
	// ("cv::computeCorrespondEpilines", vec![(pred!(mut, ["points", "whichImage", "F", "lines"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, int whichImage, const cv::_InputArray* F, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			cv::computeCorrespondEpilines(*points, whichImage, *F, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsFromHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1929
	// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsFromHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1941
	// ("cv::convertPointsHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsToHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1918
	// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsToHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// correctMatches(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2301
	// ("cv::correctMatches", vec![(pred!(mut, ["F", "points1", "points2", "newPoints1", "newPoints2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* newPoints1, const cv::_OutputArray* newPoints2, ResultVoid* ocvrs_return) {
		try {
			cv::correctMatches(*F, *points1, *points2, *newPoints1, *newPoints2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeEssentialMat(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2113
	// ("cv::decomposeEssentialMat", vec![(pred!(mut, ["E", "R1", "R2", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* t, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeEssentialMat(*E, *R1, *R2, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeHomographyMat(InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2588
	// ("cv::decomposeHomographyMat", vec![(pred!(mut, ["H", "K", "rotations", "translations", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* H, const cv::_InputArray* K, const cv::_OutputArray* rotations, const cv::_OutputArray* translations, const cv::_OutputArray* normals, Result<int>* ocvrs_return) {
		try {
			int ret = cv::decomposeHomographyMat(*H, *K, *rotations, *translations, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:681
	// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* projMatrix, const cv::_OutputArray* cameraMatrix, const cv::_OutputArray* rotMatrix, const cv::_OutputArray* transVect, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeProjectionMatrix(*projMatrix, *cameraMatrix, *rotMatrix, *transVect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:681
	// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect", "rotMatrixX", "rotMatrixY", "rotMatrixZ", "eulerAngles"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* projMatrix, const cv::_OutputArray* cameraMatrix, const cv::_OutputArray* rotMatrix, const cv::_OutputArray* transVect, const cv::_OutputArray* rotMatrixX, const cv::_OutputArray* rotMatrixY, const cv::_OutputArray* rotMatrixZ, const cv::_OutputArray* eulerAngles, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeProjectionMatrix(*projMatrix, *cameraMatrix, *rotMatrix, *transVect, *rotMatrixX, *rotMatrixY, *rotMatrixZ, *eulerAngles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawChessboardCorners(InputOutputArray, Size, InputArray, bool)(InputOutputArray, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1164
	// ("cv::drawChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "patternWasFound"], ["const cv::_InputOutputArray*", "cv::Size", "const cv::_InputArray*", "bool"]), _)]),
	void cv_drawChessboardCorners_const__InputOutputArrayR_Size_const__InputArrayR_bool(const cv::_InputOutputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, bool patternWasFound, ResultVoid* ocvrs_return) {
		try {
			cv::drawChessboardCorners(*image, *patternSize, *corners, patternWasFound);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1183
	// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float"]), _)]),
	void cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length, ResultVoid* ocvrs_return) {
		try {
			cv::drawFrameAxes(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, float, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1183
	// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length", "thickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "int"]), _)]),
	void cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_int(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length, int thickness, ResultVoid* ocvrs_return) {
		try {
			cv::drawFrameAxes(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length, thickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffine2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2507
	// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_estimateAffine2D_const__InputArrayR_const__InputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*from, *to);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2507
	// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
	void cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2441
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateAffine3D(*src, *dst, *out, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2441
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, double ransacThreshold, double confidence, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateAffine3D(*src, *dst, *out, *inliers, ransacThreshold, confidence);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffinePartial2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2555
	// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*from, *to);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffinePartial2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2555
	// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
	void cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::filterHomographyDecompByVisibleRefpoints(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2612
	// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* rotations, const cv::_InputArray* normals, const cv::_InputArray* beforePoints, const cv::_InputArray* afterPoints, const cv::_OutputArray* possibleSolutions, ResultVoid* ocvrs_return) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*rotations, *normals, *beforePoints, *afterPoints, *possibleSolutions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filterHomographyDecompByVisibleRefpoints(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2612
	// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions", "pointsMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* rotations, const cv::_InputArray* normals, const cv::_InputArray* beforePoints, const cv::_InputArray* afterPoints, const cv::_OutputArray* possibleSolutions, const cv::_InputArray* pointsMask, ResultVoid* ocvrs_return) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*rotations, *normals, *beforePoints, *afterPoints, *possibleSolutions, *pointsMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::filterSpeckles(InputOutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2316
	// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff"], ["const cv::_InputOutputArray*", "double", "int", "double"]), _)]),
	void cv_filterSpeckles_const__InputOutputArrayR_double_int_double(const cv::_InputOutputArray* img, double newVal, int maxSpeckleSize, double maxDiff, ResultVoid* ocvrs_return) {
		try {
			cv::filterSpeckles(*img, newVal, maxSpeckleSize, maxDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filterSpeckles(InputOutputArray, double, int, double, InputOutputArray)(InputOutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2316
	// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff", "buf"], ["const cv::_InputOutputArray*", "double", "int", "double", "const cv::_InputOutputArray*"]), _)]),
	void cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(const cv::_InputOutputArray* img, double newVal, int maxSpeckleSize, double maxDiff, const cv::_InputOutputArray* buf, ResultVoid* ocvrs_return) {
		try {
			cv::filterSpeckles(*img, newVal, maxSpeckleSize, maxDiff, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find4QuadCornerSubpix(InputArray, InputOutputArray, Size)(InputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1150
	// ("cv::find4QuadCornerSubpix", vec![(pred!(mut, ["img", "corners", "region_size"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Size"]), _)]),
	void cv_find4QuadCornerSubpix_const__InputArrayR_const__InputOutputArrayR_Size(const cv::_InputArray* img, const cv::_InputOutputArray* corners, cv::Size* region_size, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::find4QuadCornerSubpix(*img, *corners, *region_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findChessboardCorners(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1146
	// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCorners(*image, *patternSize, *corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findChessboardCorners(InputArray, Size, OutputArray, int)(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1146
	// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "flags"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int"]), _)]),
	void cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCorners(*image, *patternSize, *corners, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCirclesGrid2(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &, CirclesGridFinderParameters2)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1257
	// ("cv::findCirclesGrid2", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector", "parameters"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*", "cv::CirclesGridFinderParameters2"]), _)]),
	void cv_findCirclesGrid2_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_CirclesGridFinderParameters2(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, cv::CirclesGridFinderParameters2* parameters, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid2(*image, *patternSize, *centers, flags, *blobDetector, *parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findCirclesGrid(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1263
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1263
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &, CirclesGridFinderParameters)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1251
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector", "parameters"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*", "cv::CirclesGridFinderParameters"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_CirclesGridFinderParameters(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, cv::CirclesGridFinderParameters* parameters, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector, *parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2090
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2048
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2048
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, int method, double prob, double threshold, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix, method, prob, threshold, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2042
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, int method, double prob, double threshold, int maxIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix, method, prob, threshold, maxIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2042
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, int method, double prob, double threshold, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix, method, prob, threshold, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2090
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, double focal, cv::Point2d* pp, int method, double prob, double threshold, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, focal, *pp, method, prob, threshold, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2083
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int(const cv::_InputArray* points1, const cv::_InputArray* points2, double focal, cv::Point2d* pp, int method, double prob, double threshold, int maxIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, focal, *pp, method, prob, threshold, maxIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2083
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, double focal, cv::Point2d* pp, int method, double prob, double threshold, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, focal, *pp, method, prob, threshold, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1999
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2005
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, OutputArray, int, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2005
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "method", "ransacReprojThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, int method, double ransacReprojThreshold, double confidence, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask, method, ransacReprojThreshold, confidence);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1999
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1994
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, int maxIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, maxIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1994
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findHomography(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:626
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findHomography(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:632
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findHomography(InputArray, InputArray, OutputArray, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:632
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "method", "ransacReprojThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, int method, double ransacReprojThreshold, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask, method, ransacReprojThreshold);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findHomography(InputArray, InputArray, int, double, OutputArray, const int, const double)(InputArray, InputArray, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:626
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "method", "ransacReprojThreshold", "mask", "maxIters", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::_OutputArray*", "const int", "const double"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, int method, double ransacReprojThreshold, const cv::_OutputArray* mask, const int maxIters, const double confidence, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, method, ransacReprojThreshold, *mask, maxIters, confidence);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2974
	// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* image_size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::calibrate(*objectPoints, *imagePoints, *image_size, *K, *D, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrate(InputArrayOfArrays, InputArrayOfArrays, const Size &, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2974
	// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* image_size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::calibrate(*objectPoints, *imagePoints, *image_size, *K, *D, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2861
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distortPoints(InputArray, OutputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2861
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *K, *D, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, SimpleClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2938
	// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* D, const cv::Size* image_size, const cv::_InputArray* R, const cv::_OutputArray* P, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*K, *D, *image_size, *R, *P);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, const Size &, InputArray, OutputArray, double, const Size &, double)(InputArray, InputArray, SimpleClass, InputArray, OutputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2938
	// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P", "balance", "new_size", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "double"]), _)]),
	void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(const cv::_InputArray* K, const cv::_InputArray* D, const cv::Size* image_size, const cv::_InputArray* R, const cv::_OutputArray* P, double balance, const cv::Size* new_size, double fov_scale, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*K, *D, *image_size, *R, *P, balance, *new_size, fov_scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2891
	// ("cv::fisheye::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "R", "P", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::initUndistortRectifyMap(*K, *D, *R, *P, *size, m1type, *map1, *map2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2841
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *affine, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2841
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *affine, *K, *D, alpha, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2845
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2845
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, *D, alpha, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3044
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3044
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3008
	// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* tvec, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::stereoRectify(*K1, *D1, *K2, *D2, *imageSize, *R, *tvec, *R1, *R2, *P1, *P2, *Q, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectify(InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, const Size &, double, double)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3008
	// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags", "newImageSize", "balance", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::Size*", "double", "double"]), _)]),
	void cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* tvec, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, const cv::Size* newImageSize, double balance, double fov_scale, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::stereoRectify(*K1, *D1, *K2, *D2, *imageSize, *R, *tvec, *R1, *R2, *P1, *P2, *Q, flags, *newImageSize, balance, fov_scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::undistortImage(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2922
	// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortImage(*distorted, *undistorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, const Size &)(InputArray, OutputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2922
	// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "Knew", "new_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
	void cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* Knew, const cv::Size* new_size, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortImage(*distorted, *undistorted, *K, *D, *Knew, *new_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2874
	// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortPoints(*distorted, *undistorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2874
	// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "R", "P"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortPoints(*distorted, *undistorted, *K, *D, *R, *P);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getOptimalNewCameraMatrix(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1772
	// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
	void cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, double alpha, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*cameraMatrix, *distCoeffs, *imageSize, alpha);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOptimalNewCameraMatrix(InputArray, InputArray, Size, double, Size, Rect *, bool)(InputArray, InputArray, SimpleClass, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1772
	// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha", "newImgSize", "validPixROI", "centerPrincipalPoint"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double", "cv::Size", "cv::Rect*", "bool"]), _)]),
	void cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, double alpha, cv::Size* newImgSize, cv::Rect* validPixROI, bool centerPrincipalPoint, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*cameraMatrix, *distCoeffs, *imageSize, alpha, *newImgSize, validPixROI, centerPrincipalPoint);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getValidDisparityROI(Rect, Rect, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2321
	// ("cv::getValidDisparityROI", vec![(pred!(mut, ["roi1", "roi2", "minDisparity", "numberOfDisparities", "blockSize"], ["cv::Rect", "cv::Rect", "int", "int", "int"]), _)]),
	void cv_getValidDisparityROI_Rect_Rect_int_int_int(cv::Rect* roi1, cv::Rect* roi2, int minDisparity, int numberOfDisparities, int blockSize, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::getValidDisparityROI(*roi1, *roi2, minDisparity, numberOfDisparities, blockSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::initCameraMatrix2D(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1093
	// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*objectPoints, *imagePoints, *imageSize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initCameraMatrix2D(InputArrayOfArrays, InputArrayOfArrays, Size, double)(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1093
	// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
	void cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, double aspectRatio, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*objectPoints, *imagePoints, *imageSize, aspectRatio);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matMulDeriv(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:701
	// ("cv::matMulDeriv", vec![(pred!(mut, ["A", "B", "dABdA", "dABdB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* dABdA, const cv::_OutputArray* dABdB, ResultVoid* ocvrs_return) {
		try {
			cv::matMulDeriv(*A, *B, *dABdA, *dABdB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:773
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:773
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "jacobian", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, const cv::_OutputArray* jacobian, double aspectRatio, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints, *jacobian, aspectRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2165
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2165
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, const cv::_InputOutputArray* mask, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2228
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, double distanceThresh, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, distanceThresh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double, InputOutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2228
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh", "mask", "triangulatedPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, double distanceThresh, const cv::_InputOutputArray* mask, const cv::_OutputArray* triangulatedPoints, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, distanceThresh, *mask, *triangulatedPoints);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2198
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* R, const cv::_OutputArray* t, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *R, *t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray, double, Point2d, InputOutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, SimpleClass, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2198
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t", "focal", "pp", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Point2d", "const cv::_InputOutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* R, const cv::_OutputArray* t, double focal, cv::Point2d* pp, const cv::_InputOutputArray* mask, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *R, *t, focal, *pp, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rectify3Collinear(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArrayOfArrays, InputArrayOfArrays, Size, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double, Size, Rect *, Rect *, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1735
	// ("cv::rectify3Collinear", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "cameraMatrix3", "distCoeffs3", "imgpt1", "imgpt3", "imageSize", "R12", "T12", "R13", "T13", "R1", "R2", "R3", "P1", "P2", "P3", "Q", "alpha", "newImgSize", "roi1", "roi2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Size", "cv::Rect*", "cv::Rect*", "int"]), _)]),
	void cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, const cv::_InputArray* cameraMatrix3, const cv::_InputArray* distCoeffs3, const cv::_InputArray* imgpt1, const cv::_InputArray* imgpt3, cv::Size* imageSize, const cv::_InputArray* R12, const cv::_InputArray* T12, const cv::_InputArray* R13, const cv::_InputArray* T13, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* R3, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* P3, const cv::_OutputArray* Q, double alpha, cv::Size* newImgSize, cv::Rect* roi1, cv::Rect* roi2, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = cv::rectify3Collinear(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *cameraMatrix3, *distCoeffs3, *imgpt1, *imgpt3, *imageSize, *R12, *T12, *R13, *T13, *R1, *R2, *R3, *P1, *P2, *P3, *Q, alpha, *newImgSize, roi1, roi2, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2369
	// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* disparity, const cv::_OutputArray* _3dImage, const cv::_InputArray* Q, ResultVoid* ocvrs_return) {
		try {
			cv::reprojectImageTo3D(*disparity, *_3dImage, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reprojectImageTo3D(InputArray, OutputArray, InputArray, bool, int)(InputArray, OutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2369
	// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q", "handleMissingValues", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "int"]), _)]),
	void cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(const cv::_InputArray* disparity, const cv::_OutputArray* _3dImage, const cv::_InputArray* Q, bool handleMissingValues, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::reprojectImageTo3D(*disparity, *_3dImage, *Q, handleMissingValues, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sampsonDistance(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2391
	// ("cv::sampsonDistance", vec![(pred!(mut, ["pt1", "pt2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pt1, const cv::_InputArray* pt2, const cv::_InputArray* F, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sampsonDistance(*pt1, *pt2, *F);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveP3P(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:933
	// ("cv::solveP3P", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveP3P(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1071
	// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solvePnPGeneric(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, bool, SolvePnPMethod, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Enum, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1071
	// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "useExtrinsicGuess", "flags", "rvec", "tvec", "reprojectionError"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::SolvePnPMethod", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_SolvePnPMethod_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, bool useExtrinsicGuess, cv::SolvePnPMethod flags, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_OutputArray* reprojectionError, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solvePnPGeneric(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, useExtrinsicGuess, flags, *rvec, *tvec, *reprojectionError);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:899
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, double, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:899
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "confidence", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "double", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, double confidence, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, confidence, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:962
	// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineLM(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:962
	// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria"]), _)]),
	void cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineLM(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:993
	// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineVVS(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria, double)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:993
	// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria", "VVSlambda"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria", "double"]), _)]),
	void cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria, double VVSlambda, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineVVS(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria, VVSlambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:851
	// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:851
	// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
	void cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1555
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1555
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1564
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1564
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoRectifyUncalibrated(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1729
	// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* F, cv::Size* imgSize, const cv::_OutputArray* H1, const cv::_OutputArray* H2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*points1, *points2, *F, *imgSize, *H1, *H2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectifyUncalibrated(InputArray, InputArray, InputArray, Size, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1729
	// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2", "threshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* F, cv::Size* imgSize, const cv::_OutputArray* H1, const cv::_OutputArray* H2, double threshold, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*points1, *points2, *F, *imgSize, *H1, *H2, threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1691
	// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, ResultVoid* ocvrs_return) {
		try {
			cv::stereoRectify(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *R1, *R2, *P1, *P2, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectify(InputArray, InputArray, InputArray, InputArray, Size, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, double, Size, Rect *, Rect *)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1691
	// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q", "flags", "alpha", "newImageSize", "validPixROI1", "validPixROI2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "cv::Size", "cv::Rect*", "cv::Rect*"]), _)]),
	void cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, double alpha, cv::Size* newImageSize, cv::Rect* validPixROI1, cv::Rect* validPixROI2, ResultVoid* ocvrs_return) {
		try {
			cv::stereoRectify(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *R1, *R2, *P1, *P2, *Q, flags, alpha, *newImageSize, validPixROI1, validPixROI2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangulatePoints(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2282
	// ("cv::triangulatePoints", vec![(pred!(mut, ["projMatr1", "projMatr2", "projPoints1", "projPoints2", "points4D"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* projMatr1, const cv::_InputArray* projMatr2, const cv::_InputArray* projPoints1, const cv::_InputArray* projPoints2, const cv::_OutputArray* points4D, ResultVoid* ocvrs_return) {
		try {
			cv::triangulatePoints(*projMatr1, *projMatr2, *projPoints1, *projPoints2, *points4D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::validateDisparity(InputOutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2326
	// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int(const cv::_InputOutputArray* disparity, const cv::_InputArray* cost, int minDisparity, int numberOfDisparities, ResultVoid* ocvrs_return) {
		try {
			cv::validateDisparity(*disparity, *cost, minDisparity, numberOfDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// validateDisparity(InputOutputArray, InputArray, int, int, int)(InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2326
	// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities", "disp12MaxDisp"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
	void cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(const cv::_InputOutputArray* disparity, const cv::_InputArray* cost, int minDisparity, int numberOfDisparities, int disp12MaxDisp, ResultVoid* ocvrs_return) {
		try {
			cv::validateDisparity(*disparity, *cost, minDisparity, numberOfDisparities, disp12MaxDisp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CirclesGridFinderParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1188
	// ("cv::CirclesGridFinderParameters::CirclesGridFinderParameters", vec![(pred!(mut, [], []), _)]),
	void cv_CirclesGridFinderParameters_CirclesGridFinderParameters(Result<cv::CirclesGridFinderParameters*>* ocvrs_return) {
		try {
			cv::CirclesGridFinderParameters* ret = new cv::CirclesGridFinderParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CirclesGridFinderParameters::implicitClone() generated
	// ("cv::CirclesGridFinderParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::CirclesGridFinderParameters* cv_CirclesGridFinderParameters_implicitClone_const(const cv::CirclesGridFinderParameters* instance) {
			return new cv::CirclesGridFinderParameters(*instance);
	}

	// cv::CirclesGridFinderParameters::densityNeighborhoodSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1189
	// ("cv::CirclesGridFinderParameters::densityNeighborhoodSize", vec![(pred!(const, [], []), _)]),
	void cv_CirclesGridFinderParameters_propDensityNeighborhoodSize_const(const cv::CirclesGridFinderParameters* instance, cv::Size2f* ocvrs_return) {
			cv::Size2f ret = instance->densityNeighborhoodSize;
			*ocvrs_return = ret;
	}

	// cv::CirclesGridFinderParameters::setDensityNeighborhoodSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1189
	// ("cv::CirclesGridFinderParameters::setDensityNeighborhoodSize", vec![(pred!(mut, ["val"], ["const cv::Size2f"]), _)]),
	void cv_CirclesGridFinderParameters_propDensityNeighborhoodSize_const_Size2f(cv::CirclesGridFinderParameters* instance, const cv::Size2f* val) {
			instance->densityNeighborhoodSize = *val;
	}

	// cv::CirclesGridFinderParameters::minDensity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1190
	// ("cv::CirclesGridFinderParameters::minDensity", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propMinDensity_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->minDensity;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setMinDensity(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1190
	// ("cv::CirclesGridFinderParameters::setMinDensity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propMinDensity_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->minDensity = val;
	}

	// cv::CirclesGridFinderParameters::kmeansAttempts() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1191
	// ("cv::CirclesGridFinderParameters::kmeansAttempts", vec![(pred!(const, [], []), _)]),
	int cv_CirclesGridFinderParameters_propKmeansAttempts_const(const cv::CirclesGridFinderParameters* instance) {
			int ret = instance->kmeansAttempts;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setKmeansAttempts(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1191
	// ("cv::CirclesGridFinderParameters::setKmeansAttempts", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CirclesGridFinderParameters_propKmeansAttempts_const_int(cv::CirclesGridFinderParameters* instance, const int val) {
			instance->kmeansAttempts = val;
	}

	// cv::CirclesGridFinderParameters::minDistanceToAddKeypoint() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1192
	// ("cv::CirclesGridFinderParameters::minDistanceToAddKeypoint", vec![(pred!(const, [], []), _)]),
	int cv_CirclesGridFinderParameters_propMinDistanceToAddKeypoint_const(const cv::CirclesGridFinderParameters* instance) {
			int ret = instance->minDistanceToAddKeypoint;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setMinDistanceToAddKeypoint(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1192
	// ("cv::CirclesGridFinderParameters::setMinDistanceToAddKeypoint", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CirclesGridFinderParameters_propMinDistanceToAddKeypoint_const_int(cv::CirclesGridFinderParameters* instance, const int val) {
			instance->minDistanceToAddKeypoint = val;
	}

	// cv::CirclesGridFinderParameters::keypointScale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1193
	// ("cv::CirclesGridFinderParameters::keypointScale", vec![(pred!(const, [], []), _)]),
	int cv_CirclesGridFinderParameters_propKeypointScale_const(const cv::CirclesGridFinderParameters* instance) {
			int ret = instance->keypointScale;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setKeypointScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1193
	// ("cv::CirclesGridFinderParameters::setKeypointScale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CirclesGridFinderParameters_propKeypointScale_const_int(cv::CirclesGridFinderParameters* instance, const int val) {
			instance->keypointScale = val;
	}

	// cv::CirclesGridFinderParameters::minGraphConfidence() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1194
	// ("cv::CirclesGridFinderParameters::minGraphConfidence", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propMinGraphConfidence_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->minGraphConfidence;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setMinGraphConfidence(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1194
	// ("cv::CirclesGridFinderParameters::setMinGraphConfidence", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propMinGraphConfidence_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->minGraphConfidence = val;
	}

	// cv::CirclesGridFinderParameters::vertexGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1195
	// ("cv::CirclesGridFinderParameters::vertexGain", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propVertexGain_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->vertexGain;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setVertexGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1195
	// ("cv::CirclesGridFinderParameters::setVertexGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propVertexGain_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->vertexGain = val;
	}

	// cv::CirclesGridFinderParameters::vertexPenalty() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1196
	// ("cv::CirclesGridFinderParameters::vertexPenalty", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propVertexPenalty_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->vertexPenalty;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setVertexPenalty(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1196
	// ("cv::CirclesGridFinderParameters::setVertexPenalty", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propVertexPenalty_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->vertexPenalty = val;
	}

	// cv::CirclesGridFinderParameters::existingVertexGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1197
	// ("cv::CirclesGridFinderParameters::existingVertexGain", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propExistingVertexGain_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->existingVertexGain;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setExistingVertexGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1197
	// ("cv::CirclesGridFinderParameters::setExistingVertexGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propExistingVertexGain_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->existingVertexGain = val;
	}

	// cv::CirclesGridFinderParameters::edgeGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1198
	// ("cv::CirclesGridFinderParameters::edgeGain", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propEdgeGain_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->edgeGain;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setEdgeGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1198
	// ("cv::CirclesGridFinderParameters::setEdgeGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propEdgeGain_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->edgeGain = val;
	}

	// cv::CirclesGridFinderParameters::edgePenalty() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1199
	// ("cv::CirclesGridFinderParameters::edgePenalty", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propEdgePenalty_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->edgePenalty;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setEdgePenalty(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1199
	// ("cv::CirclesGridFinderParameters::setEdgePenalty", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propEdgePenalty_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->edgePenalty = val;
	}

	// cv::CirclesGridFinderParameters::convexHullFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1200
	// ("cv::CirclesGridFinderParameters::convexHullFactor", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propConvexHullFactor_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->convexHullFactor;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setConvexHullFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1200
	// ("cv::CirclesGridFinderParameters::setConvexHullFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propConvexHullFactor_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->convexHullFactor = val;
	}

	// cv::CirclesGridFinderParameters::minRNGEdgeSwitchDist() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1201
	// ("cv::CirclesGridFinderParameters::minRNGEdgeSwitchDist", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters_propMinRNGEdgeSwitchDist_const(const cv::CirclesGridFinderParameters* instance) {
			float ret = instance->minRNGEdgeSwitchDist;
			return ret;
	}

	// cv::CirclesGridFinderParameters::setMinRNGEdgeSwitchDist(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1201
	// ("cv::CirclesGridFinderParameters::setMinRNGEdgeSwitchDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters_propMinRNGEdgeSwitchDist_const_float(cv::CirclesGridFinderParameters* instance, const float val) {
			instance->minRNGEdgeSwitchDist = val;
	}

	// cv::CirclesGridFinderParameters::gridType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1207
	// ("cv::CirclesGridFinderParameters::gridType", vec![(pred!(const, [], []), _)]),
	void cv_CirclesGridFinderParameters_propGridType_const(const cv::CirclesGridFinderParameters* instance, cv::CirclesGridFinderParameters::GridType* ocvrs_return) {
			cv::CirclesGridFinderParameters::GridType ret = instance->gridType;
			*ocvrs_return = ret;
	}

	// cv::CirclesGridFinderParameters::setGridType(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1207
	// ("cv::CirclesGridFinderParameters::setGridType", vec![(pred!(mut, ["val"], ["const cv::CirclesGridFinderParameters::GridType"]), _)]),
	void cv_CirclesGridFinderParameters_propGridType_const_GridType(cv::CirclesGridFinderParameters* instance, const cv::CirclesGridFinderParameters::GridType val) {
			instance->gridType = val;
	}

	// cv::CirclesGridFinderParameters::delete() generated
	// ("cv::CirclesGridFinderParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CirclesGridFinderParameters_delete(cv::CirclesGridFinderParameters* instance) {
			delete instance;
	}

	// CirclesGridFinderParameters2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1212
	// ("cv::CirclesGridFinderParameters2::CirclesGridFinderParameters2", vec![(pred!(mut, [], []), _)]),
	void cv_CirclesGridFinderParameters2_CirclesGridFinderParameters2(Result<cv::CirclesGridFinderParameters2*>* ocvrs_return) {
		try {
			cv::CirclesGridFinderParameters2* ret = new cv::CirclesGridFinderParameters2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CirclesGridFinderParameters2::implicitClone() generated
	// ("cv::CirclesGridFinderParameters2::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::CirclesGridFinderParameters2* cv_CirclesGridFinderParameters2_implicitClone_const(const cv::CirclesGridFinderParameters2* instance) {
			return new cv::CirclesGridFinderParameters2(*instance);
	}

	// cv::CirclesGridFinderParameters2::squareSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1214
	// ("cv::CirclesGridFinderParameters2::squareSize", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters2_propSquareSize_const(const cv::CirclesGridFinderParameters2* instance) {
			float ret = instance->squareSize;
			return ret;
	}

	// cv::CirclesGridFinderParameters2::setSquareSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1214
	// ("cv::CirclesGridFinderParameters2::setSquareSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters2_propSquareSize_const_float(cv::CirclesGridFinderParameters2* instance, const float val) {
			instance->squareSize = val;
	}

	// cv::CirclesGridFinderParameters2::maxRectifiedDistance() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1215
	// ("cv::CirclesGridFinderParameters2::maxRectifiedDistance", vec![(pred!(const, [], []), _)]),
	float cv_CirclesGridFinderParameters2_propMaxRectifiedDistance_const(const cv::CirclesGridFinderParameters2* instance) {
			float ret = instance->maxRectifiedDistance;
			return ret;
	}

	// cv::CirclesGridFinderParameters2::setMaxRectifiedDistance(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1215
	// ("cv::CirclesGridFinderParameters2::setMaxRectifiedDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_CirclesGridFinderParameters2_propMaxRectifiedDistance_const_float(cv::CirclesGridFinderParameters2* instance, const float val) {
			instance->maxRectifiedDistance = val;
	}

	// cv::CirclesGridFinderParameters2::to_CirclesGridFinderParameters() generated
	// ("cv::CirclesGridFinderParameters2::to_CirclesGridFinderParameters", vec![(pred!(mut, [], []), _)]),
	cv::CirclesGridFinderParameters* cv_CirclesGridFinderParameters2_to_CirclesGridFinderParameters(cv::CirclesGridFinderParameters2* instance) {
			return dynamic_cast<cv::CirclesGridFinderParameters*>(instance);
	}

	// cv::CirclesGridFinderParameters2::delete() generated
	// ("cv::CirclesGridFinderParameters2::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CirclesGridFinderParameters2_delete(cv::CirclesGridFinderParameters2* instance) {
			delete instance;
	}

	// getPreFilterType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2669
	// ("cv::StereoBM::getPreFilterType", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterType_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2670
	// ("cv::StereoBM::setPreFilterType", vec![(pred!(mut, ["preFilterType"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterType_int(cv::StereoBM* instance, int preFilterType, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterType(preFilterType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPreFilterSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2672
	// ("cv::StereoBM::getPreFilterSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2673
	// ("cv::StereoBM::setPreFilterSize", vec![(pred!(mut, ["preFilterSize"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterSize_int(cv::StereoBM* instance, int preFilterSize, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterSize(preFilterSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2675
	// ("cv::StereoBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterCap_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2676
	// ("cv::StereoBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterCap_int(cv::StereoBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTextureThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2678
	// ("cv::StereoBM::getTextureThreshold", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getTextureThreshold_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTextureThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTextureThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2679
	// ("cv::StereoBM::setTextureThreshold", vec![(pred!(mut, ["textureThreshold"], ["int"]), _)]),
	void cv_StereoBM_setTextureThreshold_int(cv::StereoBM* instance, int textureThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setTextureThreshold(textureThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2681
	// ("cv::StereoBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getUniquenessRatio_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2682
	// ("cv::StereoBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	void cv_StereoBM_setUniquenessRatio_int(cv::StereoBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmallerBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2684
	// ("cv::StereoBM::getSmallerBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getSmallerBlockSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmallerBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmallerBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2685
	// ("cv::StereoBM::setSmallerBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	void cv_StereoBM_setSmallerBlockSize_int(cv::StereoBM* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSmallerBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2687
	// ("cv::StereoBM::getROI1", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getROI1_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setROI1(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2688
	// ("cv::StereoBM::setROI1", vec![(pred!(mut, ["roi1"], ["cv::Rect"]), _)]),
	void cv_StereoBM_setROI1_Rect(cv::StereoBM* instance, cv::Rect* roi1, ResultVoid* ocvrs_return) {
		try {
			instance->setROI1(*roi1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2690
	// ("cv::StereoBM::getROI2", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getROI2_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setROI2(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2691
	// ("cv::StereoBM::setROI2", vec![(pred!(mut, ["roi2"], ["cv::Rect"]), _)]),
	void cv_StereoBM_setROI2_Rect(cv::StereoBM* instance, cv::Rect* roi2, ResultVoid* ocvrs_return) {
		try {
			instance->setROI2(*roi2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2706
	// ("cv::StereoBM::create", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
	void cv_StereoBM_create_int_int(int numDisparities, int blockSize, Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create(numDisparities, blockSize);
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoBM::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2706
	// ("cv::StereoBM::create", vec![(pred!(mut, [], []), _)]),
	void cv_StereoBM_create(Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create();
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoBM::to_Algorithm() generated
	// ("cv::StereoBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoBM_to_Algorithm(cv::StereoBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoBM::to_StereoMatcher() generated
	// ("cv::StereoBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_StereoBM_to_StereoMatcher(cv::StereoBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::StereoBM::delete() generated
	// ("cv::StereoBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoBM_delete(cv::StereoBM* instance) {
			delete instance;
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2636
	// ("cv::StereoMatcher::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::StereoMatcher* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDisparity()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2639
	// ("cv::StereoMatcher::getMinDisparity", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getMinDisparity_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinDisparity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDisparity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2640
	// ("cv::StereoMatcher::setMinDisparity", vec![(pred!(mut, ["minDisparity"], ["int"]), _)]),
	void cv_StereoMatcher_setMinDisparity_int(cv::StereoMatcher* instance, int minDisparity, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDisparity(minDisparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2642
	// ("cv::StereoMatcher::getNumDisparities", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getNumDisparities_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumDisparities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2643
	// ("cv::StereoMatcher::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
	void cv_StereoMatcher_setNumDisparities_int(cv::StereoMatcher* instance, int numDisparities, ResultVoid* ocvrs_return) {
		try {
			instance->setNumDisparities(numDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2645
	// ("cv::StereoMatcher::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getBlockSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2646
	// ("cv::StereoMatcher::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	void cv_StereoMatcher_setBlockSize_int(cv::StereoMatcher* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSpeckleWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2648
	// ("cv::StereoMatcher::getSpeckleWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getSpeckleWindowSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSpeckleWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2649
	// ("cv::StereoMatcher::setSpeckleWindowSize", vec![(pred!(mut, ["speckleWindowSize"], ["int"]), _)]),
	void cv_StereoMatcher_setSpeckleWindowSize_int(cv::StereoMatcher* instance, int speckleWindowSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleWindowSize(speckleWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSpeckleRange()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2651
	// ("cv::StereoMatcher::getSpeckleRange", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getSpeckleRange_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSpeckleRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2652
	// ("cv::StereoMatcher::setSpeckleRange", vec![(pred!(mut, ["speckleRange"], ["int"]), _)]),
	void cv_StereoMatcher_setSpeckleRange_int(cv::StereoMatcher* instance, int speckleRange, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleRange(speckleRange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDisp12MaxDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2654
	// ("cv::StereoMatcher::getDisp12MaxDiff", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getDisp12MaxDiff_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDisp12MaxDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDisp12MaxDiff(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2655
	// ("cv::StereoMatcher::setDisp12MaxDiff", vec![(pred!(mut, ["disp12MaxDiff"], ["int"]), _)]),
	void cv_StereoMatcher_setDisp12MaxDiff_int(cv::StereoMatcher* instance, int disp12MaxDiff, ResultVoid* ocvrs_return) {
		try {
			instance->setDisp12MaxDiff(disp12MaxDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoMatcher::to_StereoBM() generated
	// ("cv::StereoMatcher::to_StereoBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoBM* cv_StereoMatcher_to_StereoBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoBM*>(instance);
	}

	// cv::StereoMatcher::to_StereoSGBM() generated
	// ("cv::StereoMatcher::to_StereoSGBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoSGBM* cv_StereoMatcher_to_StereoSGBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoSGBM*>(instance);
	}

	// cv::StereoMatcher::to_Algorithm() generated
	// ("cv::StereoMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoMatcher_to_Algorithm(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoMatcher::delete() generated
	// ("cv::StereoMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoMatcher_delete(cv::StereoMatcher* instance) {
			delete instance;
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2738
	// ("cv::StereoSGBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getPreFilterCap_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2739
	// ("cv::StereoSGBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	void cv_StereoSGBM_setPreFilterCap_int(cv::StereoSGBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2741
	// ("cv::StereoSGBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getUniquenessRatio_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2742
	// ("cv::StereoSGBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	void cv_StereoSGBM_setUniquenessRatio_int(cv::StereoSGBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2744
	// ("cv::StereoSGBM::getP1", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getP1_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2745
	// ("cv::StereoSGBM::setP1", vec![(pred!(mut, ["P1"], ["int"]), _)]),
	void cv_StereoSGBM_setP1_int(cv::StereoSGBM* instance, int P1, ResultVoid* ocvrs_return) {
		try {
			instance->setP1(P1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2747
	// ("cv::StereoSGBM::getP2", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getP2_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP2(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2748
	// ("cv::StereoSGBM::setP2", vec![(pred!(mut, ["P2"], ["int"]), _)]),
	void cv_StereoSGBM_setP2_int(cv::StereoSGBM* instance, int P2, ResultVoid* ocvrs_return) {
		try {
			instance->setP2(P2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2750
	// ("cv::StereoSGBM::getMode", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getMode_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2751
	// ("cv::StereoSGBM::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	void cv_StereoSGBM_setMode_int(cv::StereoSGBM* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, int, int, int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2790
	// ("cv::StereoSGBM::create", vec![(pred!(mut, ["minDisparity", "numDisparities", "blockSize", "P1", "P2", "disp12MaxDiff", "preFilterCap", "uniquenessRatio", "speckleWindowSize", "speckleRange", "mode"], ["int", "int", "int", "int", "int", "int", "int", "int", "int", "int", "int"]), _)]),
	void cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(int minDisparity, int numDisparities, int blockSize, int P1, int P2, int disp12MaxDiff, int preFilterCap, int uniquenessRatio, int speckleWindowSize, int speckleRange, int mode, Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create(minDisparity, numDisparities, blockSize, P1, P2, disp12MaxDiff, preFilterCap, uniquenessRatio, speckleWindowSize, speckleRange, mode);
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoSGBM::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2790
	// ("cv::StereoSGBM::create", vec![(pred!(mut, [], []), _)]),
	void cv_StereoSGBM_create(Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create();
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoSGBM::to_Algorithm() generated
	// ("cv::StereoSGBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoSGBM_to_Algorithm(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoSGBM::to_StereoMatcher() generated
	// ("cv::StereoSGBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_StereoSGBM_to_StereoMatcher(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::StereoSGBM::delete() generated
	// ("cv::StereoSGBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoSGBM_delete(cv::StereoSGBM* instance) {
			delete instance;
	}

}
