#include "ocvrs_common.hpp"
#include <opencv2/calib.hpp>
#include "calib_types.hpp"

extern "C" {
	// cv::calibrateCameraRO(InputArray, InputArray, SimpleClass, Primitive, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:951
	// ("cv::calibrateCameraRO", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "iFixedPoint", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "newObjPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateCameraRO(InputArray, InputArray, SimpleClass, Primitive, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:938
	// ("cv::calibrateCameraRO", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "iFixedPoint", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "newObjPoints", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "stdDeviationsObjPoints", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* stdDeviationsObjPoints, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *stdDeviationsObjPoints, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraRO(InputArrayOfArrays, InputArrayOfArrays, Size, int, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, SimpleClass, Primitive, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:938
	// ("cv::calibrateCameraRO", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "iFixedPoint", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "newObjPoints", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "stdDeviationsObjPoints", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* stdDeviationsObjPoints, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *stdDeviationsObjPoints, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraRO(InputArrayOfArrays, InputArrayOfArrays, Size, int, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, int, TermCriteria)(InputArray, InputArray, SimpleClass, Primitive, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:951
	// ("cv::calibrateCameraRO", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "iFixedPoint", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "newObjPoints", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:874
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:863
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:863
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:874
	// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateHandEye(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1455
	// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R_gripper2base, const cv::_InputArray* t_gripper2base, const cv::_InputArray* R_target2cam, const cv::_InputArray* t_target2cam, const cv::_OutputArray* R_cam2gripper, const cv::_OutputArray* t_cam2gripper, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateHandEye(*R_gripper2base, *t_gripper2base, *R_target2cam, *t_target2cam, *R_cam2gripper, *t_cam2gripper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateHandEye(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, HandEyeCalibrationMethod)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1455
	// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::HandEyeCalibrationMethod"]), _)]),
	void cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_HandEyeCalibrationMethod(const cv::_InputArray* R_gripper2base, const cv::_InputArray* t_gripper2base, const cv::_InputArray* R_target2cam, const cv::_InputArray* t_target2cam, const cv::_OutputArray* R_cam2gripper, const cv::_OutputArray* t_cam2gripper, cv::HandEyeCalibrationMethod method, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateHandEye(*R_gripper2base, *t_gripper2base, *R_target2cam, *t_target2cam, *R_cam2gripper, *t_cam2gripper, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateMultiview(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1301
	// ("cv::calibrateMultiview", vec![(pred!(mut, ["objPoints", "imagePoints", "imageSize", "detectionMask", "models", "Ks", "distortions", "Rs", "Ts"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::Size>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* objPoints, const std::vector<std::vector<cv::Mat>>* imagePoints, const std::vector<cv::Size>* imageSize, const cv::_InputArray* detectionMask, const cv::_InputArray* models, const cv::_InputOutputArray* Ks, const cv::_InputOutputArray* distortions, const cv::_InputOutputArray* Rs, const cv::_InputOutputArray* Ts, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateMultiview(*objPoints, *imagePoints, *imageSize, *detectionMask, *models, *Ks, *distortions, *Rs, *Ts);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateMultiview(InputArrayOfArrays, const std::vector<std::vector<Mat>> &, const std::vector<cv::Size> &, InputArray, InputArray, InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputArray, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1301
	// ("cv::calibrateMultiview", vec![(pred!(mut, ["objPoints", "imagePoints", "imageSize", "detectionMask", "models", "Ks", "distortions", "Rs", "Ts", "flagsForIntrinsics", "flags"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::Size>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_int(const cv::_InputArray* objPoints, const std::vector<std::vector<cv::Mat>>* imagePoints, const std::vector<cv::Size>* imageSize, const cv::_InputArray* detectionMask, const cv::_InputArray* models, const cv::_InputOutputArray* Ks, const cv::_InputOutputArray* distortions, const cv::_InputOutputArray* Rs, const cv::_InputOutputArray* Ts, const cv::_InputArray* flagsForIntrinsics, int flags, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateMultiview(*objPoints, *imagePoints, *imageSize, *detectionMask, *models, *Ks, *distortions, *Rs, *Ts, *flagsForIntrinsics, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateMultiview(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1291
	// ("cv::calibrateMultiview", vec![(pred!(mut, ["objPoints", "imagePoints", "imageSize", "detectionMask", "models", "Ks", "distortions", "Rs", "Ts", "initializationPairs", "rvecs0", "tvecs0", "perFrameErrors"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::Size>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objPoints, const std::vector<std::vector<cv::Mat>>* imagePoints, const std::vector<cv::Size>* imageSize, const cv::_InputArray* detectionMask, const cv::_InputArray* models, const cv::_InputOutputArray* Ks, const cv::_InputOutputArray* distortions, const cv::_InputOutputArray* Rs, const cv::_InputOutputArray* Ts, const cv::_OutputArray* initializationPairs, const cv::_OutputArray* rvecs0, const cv::_OutputArray* tvecs0, const cv::_OutputArray* perFrameErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateMultiview(*objPoints, *imagePoints, *imageSize, *detectionMask, *models, *Ks, *distortions, *Rs, *Ts, *initializationPairs, *rvecs0, *tvecs0, *perFrameErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateMultiview(InputArrayOfArrays, const std::vector<std::vector<Mat>> &, const std::vector<cv::Size> &, InputArray, InputArray, InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, InputArray, int)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1291
	// ("cv::calibrateMultiview", vec![(pred!(mut, ["objPoints", "imagePoints", "imageSize", "detectionMask", "models", "Ks", "distortions", "Rs", "Ts", "initializationPairs", "rvecs0", "tvecs0", "perFrameErrors", "flagsForIntrinsics", "flags"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::Size>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* objPoints, const std::vector<std::vector<cv::Mat>>* imagePoints, const std::vector<cv::Size>* imageSize, const cv::_InputArray* detectionMask, const cv::_InputArray* models, const cv::_InputOutputArray* Ks, const cv::_InputOutputArray* distortions, const cv::_InputOutputArray* Rs, const cv::_InputOutputArray* Ts, const cv::_OutputArray* initializationPairs, const cv::_OutputArray* rvecs0, const cv::_OutputArray* tvecs0, const cv::_OutputArray* perFrameErrors, const cv::_InputArray* flagsForIntrinsics, int flags, Result<double>* ocvrs_return) {
		try {
			double ret = cv::calibrateMultiview(*objPoints, *imagePoints, *imageSize, *detectionMask, *models, *Ks, *distortions, *Rs, *Ts, *initializationPairs, *rvecs0, *tvecs0, *perFrameErrors, *flagsForIntrinsics, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calibrateRobotWorldHandEye(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1598
	// ("cv::calibrateRobotWorldHandEye", vec![(pred!(mut, ["R_world2cam", "t_world2cam", "R_base2gripper", "t_base2gripper", "R_base2world", "t_base2world", "R_gripper2cam", "t_gripper2cam"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calibrateRobotWorldHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R_world2cam, const cv::_InputArray* t_world2cam, const cv::_InputArray* R_base2gripper, const cv::_InputArray* t_base2gripper, const cv::_OutputArray* R_base2world, const cv::_OutputArray* t_base2world, const cv::_OutputArray* R_gripper2cam, const cv::_OutputArray* t_gripper2cam, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateRobotWorldHandEye(*R_world2cam, *t_world2cam, *R_base2gripper, *t_base2gripper, *R_base2world, *t_base2world, *R_gripper2cam, *t_gripper2cam);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateRobotWorldHandEye(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, OutputArray, OutputArray, RobotWorldHandEyeCalibrationMethod)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1598
	// ("cv::calibrateRobotWorldHandEye", vec![(pred!(mut, ["R_world2cam", "t_world2cam", "R_base2gripper", "t_base2gripper", "R_base2world", "t_base2world", "R_gripper2cam", "t_gripper2cam", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::RobotWorldHandEyeCalibrationMethod"]), _)]),
	void cv_calibrateRobotWorldHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_RobotWorldHandEyeCalibrationMethod(const cv::_InputArray* R_world2cam, const cv::_InputArray* t_world2cam, const cv::_InputArray* R_base2gripper, const cv::_InputArray* t_base2gripper, const cv::_OutputArray* R_base2world, const cv::_OutputArray* t_base2world, const cv::_OutputArray* R_gripper2cam, const cv::_OutputArray* t_gripper2cam, cv::RobotWorldHandEyeCalibrationMethod method, ResultVoid* ocvrs_return) {
		try {
			cv::calibrateRobotWorldHandEye(*R_world2cam, *t_world2cam, *R_base2gripper, *t_base2gripper, *R_base2world, *t_base2world, *R_gripper2cam, *t_gripper2cam, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrationMatrixValues(InputArray, Size, double, double, double &, double &, double &, Point2d &, double &)(InputArray, SimpleClass, Primitive, Primitive, Indirect, Indirect, Indirect, SimpleClass, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:979
	// ("cv::calibrationMatrixValues", vec![(pred!(mut, ["cameraMatrix", "imageSize", "apertureWidth", "apertureHeight", "fovx", "fovy", "focalLength", "principalPoint", "aspectRatio"], ["const cv::_InputArray*", "cv::Size", "double", "double", "double*", "double*", "double*", "cv::Point2d*", "double*"]), _)]),
	void cv_calibrationMatrixValues_const__InputArrayR_Size_double_double_doubleR_doubleR_doubleR_Point2dR_doubleR(const cv::_InputArray* cameraMatrix, cv::Size* imageSize, double apertureWidth, double apertureHeight, double* fovx, double* fovy, double* focalLength, cv::Point2d* principalPoint, double* aspectRatio, ResultVoid* ocvrs_return) {
		try {
			cv::calibrationMatrixValues(*cameraMatrix, *imageSize, apertureWidth, apertureHeight, *fovx, *fovy, *focalLength, *principalPoint, *aspectRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkChessboard(InputArray, Size)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:556
	// ("cv::checkChessboard", vec![(pred!(mut, ["img", "size"], ["const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_checkChessboard_const__InputArrayR_Size(const cv::_InputArray* img, cv::Size* size, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkChessboard(*img, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawChessboardCorners(InputOutputArray, Size, InputArray, bool)(InputOutputArray, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:666
	// ("cv::drawChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "patternWasFound"], ["const cv::_InputOutputArray*", "cv::Size", "const cv::_InputArray*", "bool"]), _)]),
	void cv_drawChessboardCorners_const__InputOutputArrayR_Size_const__InputArrayR_bool(const cv::_InputOutputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, bool patternWasFound, ResultVoid* ocvrs_return) {
		try {
			cv::drawChessboardCorners(*image, *patternSize, *corners, patternWasFound);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateChessboardSharpness(InputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:646
	// ("cv::estimateChessboardSharpness", vec![(pred!(mut, ["image", "patternSize", "corners"], ["const cv::_InputArray*", "cv::Size", "const cv::_InputArray*"]), _)]),
	void cv_estimateChessboardSharpness_const__InputArrayR_Size_const__InputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::estimateChessboardSharpness(*image, *patternSize, *corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateChessboardSharpness(InputArray, Size, InputArray, float, bool, OutputArray)(InputArray, SimpleClass, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:646
	// ("cv::estimateChessboardSharpness", vec![(pred!(mut, ["image", "patternSize", "corners", "rise_distance", "vertical", "sharpness"], ["const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "float", "bool", "const cv::_OutputArray*"]), _)]),
	void cv_estimateChessboardSharpness_const__InputArrayR_Size_const__InputArrayR_float_bool_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, float rise_distance, bool vertical, const cv::_OutputArray* sharpness, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::estimateChessboardSharpness(*image, *patternSize, *corners, rise_distance, vertical, *sharpness);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// find4QuadCornerSubpix(InputArray, InputOutputArray, Size)(InputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:652
	// ("cv::find4QuadCornerSubpix", vec![(pred!(mut, ["img", "corners", "region_size"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Size"]), _)]),
	void cv_find4QuadCornerSubpix_const__InputArrayR_const__InputOutputArrayR_Size(const cv::_InputArray* img, const cv::_InputOutputArray* corners, cv::Size* region_size, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::find4QuadCornerSubpix(*img, *corners, *region_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findChessboardCornersSB(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:613
	// ("cv::findChessboardCornersSB", vec![(pred!(mut, ["image", "patternSize", "corners"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCornersSB(*image, *patternSize, *corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findChessboardCornersSB(InputArray, Size, OutputArray, int)(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:613
	// ("cv::findChessboardCornersSB", vec![(pred!(mut, ["image", "patternSize", "corners", "flags"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int"]), _)]),
	void cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCornersSB(*image, *patternSize, *corners, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findChessboardCornersSB(InputArray, Size, OutputArray, int, OutputArray)(InputArray, SimpleClass, OutputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:609
	// ("cv::findChessboardCornersSB", vec![(pred!(mut, ["image", "patternSize", "corners", "flags", "meta"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags, const cv::_OutputArray* meta, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCornersSB(*image, *patternSize, *corners, flags, *meta);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findChessboardCorners(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:549
	// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCorners(*image, *patternSize, *corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findChessboardCorners(InputArray, Size, OutputArray, int)(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:549
	// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "flags"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int"]), _)]),
	void cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findChessboardCorners(*image, *patternSize, *corners, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findCirclesGrid(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:739
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:739
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &, const CirclesGridFinderParameters &)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:733
	// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector", "parameters"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*", "const cv::CirclesGridFinderParameters*"]), _)]),
	void cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_const_CirclesGridFinderParametersR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, const cv::CirclesGridFinderParameters* parameters, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector, *parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1662
	// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* image_size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::calibrate(*objectPoints, *imagePoints, *image_size, *K, *D, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrate(InputArrayOfArrays, InputArrayOfArrays, const Size &, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1662
	// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* image_size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::calibrate(*objectPoints, *imagePoints, *image_size, *K, *D, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1713
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1707
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1707
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1713
	// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::initCameraMatrix2D(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:490
	// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*objectPoints, *imagePoints, *imageSize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initCameraMatrix2D(InputArrayOfArrays, InputArrayOfArrays, Size, double)(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:490
	// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
	void cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, double aspectRatio, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*objectPoints, *imagePoints, *imageSize, aspectRatio);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::registerCameras(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum, InputArray, InputArray, Enum, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1222
	// ("cv::registerCameras", vec![(pred!(mut, ["objectPoints1", "objectPoints2", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraModel1", "cameraMatrix2", "distCoeffs2", "cameraModel2", "R", "T", "E", "F", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints1, const cv::_InputArray* objectPoints2, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, cv::CameraModel cameraModel1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::CameraModel cameraModel2, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::registerCameras(*objectPoints1, *objectPoints2, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, cameraModel1, *cameraMatrix2, *distCoeffs2, cameraModel2, *R, *T, *E, *F, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::registerCameras(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum, InputArray, InputArray, Enum, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1208
	// ("cv::registerCameras", vec![(pred!(mut, ["objectPoints1", "objectPoints2", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraModel1", "cameraMatrix2", "distCoeffs2", "cameraModel2", "R", "T", "E", "F", "rvecs", "tvecs", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints1, const cv::_InputArray* objectPoints2, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, cv::CameraModel cameraModel1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::CameraModel cameraModel2, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::registerCameras(*objectPoints1, *objectPoints2, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, cameraModel1, *cameraMatrix2, *distCoeffs2, cameraModel2, *R, *T, *E, *F, *rvecs, *tvecs, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerCameras(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, CameraModel, InputArray, InputArray, CameraModel, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum, InputArray, InputArray, Enum, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1208
	// ("cv::registerCameras", vec![(pred!(mut, ["objectPoints1", "objectPoints2", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraModel1", "cameraMatrix2", "distCoeffs2", "cameraModel2", "R", "T", "E", "F", "rvecs", "tvecs", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints1, const cv::_InputArray* objectPoints2, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, cv::CameraModel cameraModel1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::CameraModel cameraModel2, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::registerCameras(*objectPoints1, *objectPoints2, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, cameraModel1, *cameraMatrix2, *distCoeffs2, cameraModel2, *R, *T, *E, *F, *rvecs, *tvecs, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerCameras(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, CameraModel, InputArray, InputArray, CameraModel, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum, InputArray, InputArray, Enum, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1222
	// ("cv::registerCameras", vec![(pred!(mut, ["objectPoints1", "objectPoints2", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraModel1", "cameraMatrix2", "distCoeffs2", "cameraModel2", "R", "T", "E", "F", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputArray*", "const cv::_InputArray*", "cv::CameraModel", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints1, const cv::_InputArray* objectPoints2, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, cv::CameraModel cameraModel1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::CameraModel cameraModel2, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::registerCameras(*objectPoints1, *objectPoints2, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, cameraModel1, *cameraMatrix2, *distCoeffs2, cameraModel2, *R, *T, *E, *F, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1135
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1116
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "rvecs", "tvecs", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *rvecs, *tvecs, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1116
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "rvecs", "tvecs", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *rvecs, *tvecs, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1135
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1126
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:1126
	// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CirclesGridFinderParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/calib.hpp:671
	// ("cv::CirclesGridFinderParameters::CirclesGridFinderParameters", vec![(pred!(mut, [], []), _)]),
	void cv_CirclesGridFinderParameters_CirclesGridFinderParameters(Result<cv::CirclesGridFinderParameters>* ocvrs_return) {
		try {
			cv::CirclesGridFinderParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
