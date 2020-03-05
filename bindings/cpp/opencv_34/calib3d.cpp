#include "calib3d.hpp"
#include "calib3d_types.hpp"

extern "C" {
	Result<cv::Vec3d> cv_RQDecomp3x3_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* src, void* mtxR, void* mtxQ, void* Qx, void* Qy, void* Qz) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(mtxR), *reinterpret_cast<const cv::_OutputArray*>(mtxQ), *reinterpret_cast<const cv::_OutputArray*>(Qx), *reinterpret_cast<const cv::_OutputArray*>(Qy), *reinterpret_cast<const cv::_OutputArray*>(Qz));
			return Ok<cv::Vec3d>(ret);
		} OCVRS_CATCH(Result<cv::Vec3d>)
	}
	
	Result_void cv_Rodrigues_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* src, void* dst, void* jacobian) {
		try {
			cv::Rodrigues(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_OutputArray*>(jacobian));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_calibrateCamera_const__InputArrayX_const__InputArrayX_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints, cv::Size imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, void* stdDeviationsIntrinsics, void* stdDeviationsExtrinsics, void* perViewErrors, int flags, void* criteria) {
		try {
			double ret = cv::calibrateCamera(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsIntrinsics), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsExtrinsics), *reinterpret_cast<const cv::_OutputArray*>(perViewErrors), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_calibrateCamera_const__InputArrayX_const__InputArrayX_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints, cv::Size imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, int flags, void* criteria) {
		try {
			double ret = cv::calibrateCamera(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_calibrateHandEye_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_HandEyeCalibrationMethod(void* R_gripper2base, void* t_gripper2base, void* R_target2cam, void* t_target2cam, void* R_cam2gripper, void* t_cam2gripper, cv::HandEyeCalibrationMethod method) {
		try {
			cv::calibrateHandEye(*reinterpret_cast<const cv::_InputArray*>(R_gripper2base), *reinterpret_cast<const cv::_InputArray*>(t_gripper2base), *reinterpret_cast<const cv::_InputArray*>(R_target2cam), *reinterpret_cast<const cv::_InputArray*>(t_target2cam), *reinterpret_cast<const cv::_OutputArray*>(R_cam2gripper), *reinterpret_cast<const cv::_OutputArray*>(t_cam2gripper), method);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calibrationMatrixValues_const__InputArrayX_Size_double_double_doubleX_doubleX_doubleX_Point2dX_doubleX(void* cameraMatrix, cv::Size imageSize, double apertureWidth, double apertureHeight, double* fovx, double* fovy, double* focalLength, cv::Point2d* principalPoint, double* aspectRatio) {
		try {
			cv::calibrationMatrixValues(*reinterpret_cast<const cv::_InputArray*>(cameraMatrix), imageSize, apertureWidth, apertureHeight, *fovx, *fovy, *focalLength, *principalPoint, *aspectRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_composeRT_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* rvec1, void* tvec1, void* rvec2, void* tvec2, void* rvec3, void* tvec3, void* dr3dr1, void* dr3dt1, void* dr3dr2, void* dr3dt2, void* dt3dr1, void* dt3dt1, void* dt3dr2, void* dt3dt2) {
		try {
			cv::composeRT(*reinterpret_cast<const cv::_InputArray*>(rvec1), *reinterpret_cast<const cv::_InputArray*>(tvec1), *reinterpret_cast<const cv::_InputArray*>(rvec2), *reinterpret_cast<const cv::_InputArray*>(tvec2), *reinterpret_cast<const cv::_OutputArray*>(rvec3), *reinterpret_cast<const cv::_OutputArray*>(tvec3), *reinterpret_cast<const cv::_OutputArray*>(dr3dr1), *reinterpret_cast<const cv::_OutputArray*>(dr3dt1), *reinterpret_cast<const cv::_OutputArray*>(dr3dr2), *reinterpret_cast<const cv::_OutputArray*>(dr3dt2), *reinterpret_cast<const cv::_OutputArray*>(dt3dr1), *reinterpret_cast<const cv::_OutputArray*>(dt3dt1), *reinterpret_cast<const cv::_OutputArray*>(dt3dr2), *reinterpret_cast<const cv::_OutputArray*>(dt3dt2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_computeCorrespondEpilines_const__InputArrayX_int_const__InputArrayX_const__OutputArrayX(void* points, int whichImage, void* F, void* lines) {
		try {
			cv::computeCorrespondEpilines(*reinterpret_cast<const cv::_InputArray*>(points), whichImage, *reinterpret_cast<const cv::_InputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(lines));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convertPointsFromHomogeneous_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::convertPointsFromHomogeneous(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convertPointsHomogeneous_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::convertPointsHomogeneous(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convertPointsToHomogeneous_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::convertPointsToHomogeneous(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_correctMatches_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* F, void* points1, void* points2, void* newPoints1, void* newPoints2) {
		try {
			cv::correctMatches(*reinterpret_cast<const cv::_InputArray*>(F), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_OutputArray*>(newPoints1), *reinterpret_cast<const cv::_OutputArray*>(newPoints2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_decomposeEssentialMat_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* E, void* R1, void* R2, void* t) {
		try {
			cv::decomposeEssentialMat(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_OutputArray*>(R1), *reinterpret_cast<const cv::_OutputArray*>(R2), *reinterpret_cast<const cv::_OutputArray*>(t));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_decomposeHomographyMat_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* H, void* K, void* rotations, void* translations, void* normals) {
		try {
			int ret = cv::decomposeHomographyMat(*reinterpret_cast<const cv::_InputArray*>(H), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(rotations), *reinterpret_cast<const cv::_OutputArray*>(translations), *reinterpret_cast<const cv::_OutputArray*>(normals));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_decomposeProjectionMatrix_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* projMatrix, void* cameraMatrix, void* rotMatrix, void* transVect, void* rotMatrixX, void* rotMatrixY, void* rotMatrixZ, void* eulerAngles) {
		try {
			cv::decomposeProjectionMatrix(*reinterpret_cast<const cv::_InputArray*>(projMatrix), *reinterpret_cast<const cv::_OutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_OutputArray*>(rotMatrix), *reinterpret_cast<const cv::_OutputArray*>(transVect), *reinterpret_cast<const cv::_OutputArray*>(rotMatrixX), *reinterpret_cast<const cv::_OutputArray*>(rotMatrixY), *reinterpret_cast<const cv::_OutputArray*>(rotMatrixZ), *reinterpret_cast<const cv::_OutputArray*>(eulerAngles));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawChessboardCorners_const__InputOutputArrayX_Size_const__InputArrayX_bool(void* image, cv::Size patternSize, void* corners, bool patternWasFound) {
		try {
			cv::drawChessboardCorners(*reinterpret_cast<const cv::_InputOutputArray*>(image), patternSize, *reinterpret_cast<const cv::_InputArray*>(corners), patternWasFound);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawFrameAxes_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_float_int(void* image, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, float length, int thickness) {
		try {
			cv::drawFrameAxes(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), length, thickness);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_estimateAffine2D_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_double_size_t_double_size_t(void* from, void* to, void* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*reinterpret_cast<const cv::_InputArray*>(from), *reinterpret_cast<const cv::_InputArray*>(to), *reinterpret_cast<const cv::_OutputArray*>(inliers), method, ransacReprojThreshold, maxIters, confidence, refineIters);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_estimateAffine3D_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_double(void* src, void* dst, void* out, void* inliers, double ransacThreshold, double confidence) {
		try {
			int ret = cv::estimateAffine3D(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst), *reinterpret_cast<const cv::_OutputArray*>(out), *reinterpret_cast<const cv::_OutputArray*>(inliers), ransacThreshold, confidence);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_estimateAffinePartial2D_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_double_size_t_double_size_t(void* from, void* to, void* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*reinterpret_cast<const cv::_InputArray*>(from), *reinterpret_cast<const cv::_InputArray*>(to), *reinterpret_cast<const cv::_OutputArray*>(inliers), method, ransacReprojThreshold, maxIters, confidence, refineIters);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* rotations, void* normals, void* beforePoints, void* afterPoints, void* possibleSolutions, void* pointsMask) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*reinterpret_cast<const cv::_InputArray*>(rotations), *reinterpret_cast<const cv::_InputArray*>(normals), *reinterpret_cast<const cv::_InputArray*>(beforePoints), *reinterpret_cast<const cv::_InputArray*>(afterPoints), *reinterpret_cast<const cv::_OutputArray*>(possibleSolutions), *reinterpret_cast<const cv::_InputArray*>(pointsMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_filterSpeckles_const__InputOutputArrayX_double_int_double_const__InputOutputArrayX(void* img, double newVal, int maxSpeckleSize, double maxDiff, void* buf) {
		try {
			cv::filterSpeckles(*reinterpret_cast<const cv::_InputOutputArray*>(img), newVal, maxSpeckleSize, maxDiff, *reinterpret_cast<const cv::_InputOutputArray*>(buf));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_find4QuadCornerSubpix_const__InputArrayX_const__InputOutputArrayX_Size(void* img, void* corners, cv::Size region_size) {
		try {
			bool ret = cv::find4QuadCornerSubpix(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputOutputArray*>(corners), region_size);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_findChessboardCorners_const__InputArrayX_Size_const__OutputArrayX_int(void* image, cv::Size patternSize, void* corners, int flags) {
		try {
			bool ret = cv::findChessboardCorners(*reinterpret_cast<const cv::_InputArray*>(image), patternSize, *reinterpret_cast<const cv::_OutputArray*>(corners), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_findCirclesGrid2_const__InputArrayX_Size_const__OutputArrayX_int_const_Ptr_FeatureDetector_X_CirclesGridFinderParameters2(void* image, cv::Size patternSize, void* centers, int flags, void* blobDetector, cv::CirclesGridFinderParameters2 parameters) {
		try {
			bool ret = cv::findCirclesGrid2(*reinterpret_cast<const cv::_InputArray*>(image), patternSize, *reinterpret_cast<const cv::_OutputArray*>(centers), flags, *reinterpret_cast<const cv::Ptr<cv::FeatureDetector>*>(blobDetector), parameters);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_findCirclesGrid_const__InputArrayX_Size_const__OutputArrayX_int_const_Ptr_FeatureDetector_X(void* image, cv::Size patternSize, void* centers, int flags, void* blobDetector) {
		try {
			bool ret = cv::findCirclesGrid(*reinterpret_cast<const cv::_InputArray*>(image), patternSize, *reinterpret_cast<const cv::_OutputArray*>(centers), flags, *reinterpret_cast<const cv::Ptr<cv::FeatureDetector>*>(blobDetector));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_findCirclesGrid_const__InputArrayX_Size_const__OutputArrayX_int_const_Ptr_FeatureDetector_X_CirclesGridFinderParameters(void* image, cv::Size patternSize, void* centers, int flags, void* blobDetector, cv::CirclesGridFinderParameters parameters) {
		try {
			bool ret = cv::findCirclesGrid(*reinterpret_cast<const cv::_InputArray*>(image), patternSize, *reinterpret_cast<const cv::_OutputArray*>(centers), flags, *reinterpret_cast<const cv::Ptr<cv::FeatureDetector>*>(blobDetector), parameters);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_findEssentialMat_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_double_double_const__OutputArrayX(void* points1, void* points2, void* cameraMatrix, int method, double prob, double threshold, void* mask) {
		try {
			cv::Mat ret = cv::findEssentialMat(*reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), method, prob, threshold, *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_findEssentialMat_const__InputArrayX_const__InputArrayX_double_Point2d_int_double_double_const__OutputArrayX(void* points1, void* points2, double focal, cv::Point2d pp, int method, double prob, double threshold, void* mask) {
		try {
			cv::Mat ret = cv::findEssentialMat(*reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), focal, pp, method, prob, threshold, *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_findFundamentalMat_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_double_double(void* points1, void* points2, void* mask, int method, double ransacReprojThreshold, double confidence) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_OutputArray*>(mask), method, ransacReprojThreshold, confidence);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_findFundamentalMat_const__InputArrayX_const__InputArrayX_int_double_double_const__OutputArrayX(void* points1, void* points2, int method, double ransacReprojThreshold, double confidence, void* mask) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), method, ransacReprojThreshold, confidence, *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_findHomography_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_double(void* srcPoints, void* dstPoints, void* mask, int method, double ransacReprojThreshold) {
		try {
			cv::Mat ret = cv::findHomography(*reinterpret_cast<const cv::_InputArray*>(srcPoints), *reinterpret_cast<const cv::_InputArray*>(dstPoints), *reinterpret_cast<const cv::_OutputArray*>(mask), method, ransacReprojThreshold);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_findHomography_const__InputArrayX_const__InputArrayX_int_double_const__OutputArrayX_int_double(void* srcPoints, void* dstPoints, int method, double ransacReprojThreshold, void* mask, int maxIters, double confidence) {
		try {
			cv::Mat ret = cv::findHomography(*reinterpret_cast<const cv::_InputArray*>(srcPoints), *reinterpret_cast<const cv::_InputArray*>(dstPoints), method, ransacReprojThreshold, *reinterpret_cast<const cv::_OutputArray*>(mask), maxIters, confidence);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_fisheye_calibrate_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints, const cv::Size* image_size, void* K, void* D, void* rvecs, void* tvecs, int flags, void* criteria) {
		try {
			double ret = cv::fisheye::calibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *image_size, *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_InputOutputArray*>(D), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_fisheye_distortPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_double(void* undistorted, void* distorted, void* K, void* D, double alpha) {
		try {
			cv::fisheye::distortPoints(*reinterpret_cast<const cv::_InputArray*>(undistorted), *reinterpret_cast<const cv::_OutputArray*>(distorted), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), alpha);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputArrayX_const__OutputArrayX_double_const_SizeX_double(void* K, void* D, const cv::Size* image_size, void* R, void* P, double balance, const cv::Size* new_size, double fov_scale) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *image_size, *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(P), balance, *new_size, fov_scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_initUndistortRectifyMap_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_int_const__OutputArrayX_const__OutputArrayX(void* K, void* D, void* R, void* P, const cv::Size* size, int m1type, void* map1, void* map2) {
		try {
			cv::fisheye::initUndistortRectifyMap(*reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(P), *size, m1type, *reinterpret_cast<const cv::_OutputArray*>(map1), *reinterpret_cast<const cv::_OutputArray*>(map2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_projectPoints_const__InputArrayX_const__OutputArrayX_const_Affine3dX_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX(void* objectPoints, void* imagePoints, const cv::Affine3d* affine, void* K, void* D, double alpha, void* jacobian) {
		try {
			cv::fisheye::projectPoints(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_OutputArray*>(imagePoints), *affine, *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), alpha, *reinterpret_cast<const cv::_OutputArray*>(jacobian));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_projectPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX(void* objectPoints, void* imagePoints, void* rvec, void* tvec, void* K, void* D, double alpha, void* jacobian) {
		try {
			cv::fisheye::projectPoints(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_OutputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), alpha, *reinterpret_cast<const cv::_OutputArray*>(jacobian));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_fisheye_stereoCalibrate_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_Size_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints1, void* imagePoints2, void* K1, void* D1, void* K2, void* D2, cv::Size imageSize, void* R, void* T, int flags, void* criteria) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints1), *reinterpret_cast<const cv::_InputArray*>(imagePoints2), *reinterpret_cast<const cv::_InputOutputArray*>(K1), *reinterpret_cast<const cv::_InputOutputArray*>(D1), *reinterpret_cast<const cv::_InputOutputArray*>(K2), *reinterpret_cast<const cv::_InputOutputArray*>(D2), imageSize, *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(T), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_fisheye_stereoRectify_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_const_SizeX_double_double(void* K1, void* D1, void* K2, void* D2, const cv::Size* imageSize, void* R, void* tvec, void* R1, void* R2, void* P1, void* P2, void* Q, int flags, const cv::Size* newImageSize, double balance, double fov_scale) {
		try {
			cv::fisheye::stereoRectify(*reinterpret_cast<const cv::_InputArray*>(K1), *reinterpret_cast<const cv::_InputArray*>(D1), *reinterpret_cast<const cv::_InputArray*>(K2), *reinterpret_cast<const cv::_InputArray*>(D2), *imageSize, *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_OutputArray*>(R1), *reinterpret_cast<const cv::_OutputArray*>(R2), *reinterpret_cast<const cv::_OutputArray*>(P1), *reinterpret_cast<const cv::_OutputArray*>(P2), *reinterpret_cast<const cv::_OutputArray*>(Q), flags, *newImageSize, balance, fov_scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_undistortImage_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX(void* distorted, void* undistorted, void* K, void* D, void* Knew, const cv::Size* new_size) {
		try {
			cv::fisheye::undistortImage(*reinterpret_cast<const cv::_InputArray*>(distorted), *reinterpret_cast<const cv::_OutputArray*>(undistorted), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(Knew), *new_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fisheye_undistortPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* distorted, void* undistorted, void* K, void* D, void* R, void* P) {
		try {
			cv::fisheye::undistortPoints(*reinterpret_cast<const cv::_InputArray*>(distorted), *reinterpret_cast<const cv::_OutputArray*>(undistorted), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(P));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_getOptimalNewCameraMatrix_const__InputArrayX_const__InputArrayX_Size_double_Size_RectX_bool(void* cameraMatrix, void* distCoeffs, cv::Size imageSize, double alpha, cv::Size newImgSize, cv::Rect* validPixROI, bool centerPrincipalPoint) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), imageSize, alpha, newImgSize, validPixROI, centerPrincipalPoint);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Rect> cv_getValidDisparityROI_Rect_Rect_int_int_int(cv::Rect roi1, cv::Rect roi2, int minDisparity, int numberOfDisparities, int SADWindowSize) {
		try {
			cv::Rect ret = cv::getValidDisparityROI(roi1, roi2, minDisparity, numberOfDisparities, SADWindowSize);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<void*> cv_initCameraMatrix2D_const__InputArrayX_const__InputArrayX_Size_double(void* objectPoints, void* imagePoints, cv::Size imageSize, double aspectRatio) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), imageSize, aspectRatio);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_matMulDeriv_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* A, void* B, void* dABdA, void* dABdB) {
		try {
			cv::matMulDeriv(*reinterpret_cast<const cv::_InputArray*>(A), *reinterpret_cast<const cv::_InputArray*>(B), *reinterpret_cast<const cv::_OutputArray*>(dABdA), *reinterpret_cast<const cv::_OutputArray*>(dABdB));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_projectPoints_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double(void* objectPoints, void* rvec, void* tvec, void* cameraMatrix, void* distCoeffs, void* imagePoints, void* jacobian, double aspectRatio) {
		try {
			cv::projectPoints(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(imagePoints), *reinterpret_cast<const cv::_OutputArray*>(jacobian), aspectRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_recoverPose_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX(void* E, void* points1, void* points2, void* cameraMatrix, void* R, void* t, void* mask) {
		try {
			int ret = cv::recoverPose(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t), *reinterpret_cast<const cv::_InputOutputArray*>(mask));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_recoverPose_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_const__InputOutputArrayX_const__OutputArrayX(void* E, void* points1, void* points2, void* cameraMatrix, void* R, void* t, double distanceThresh, void* mask, void* triangulatedPoints) {
		try {
			int ret = cv::recoverPose(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t), distanceThresh, *reinterpret_cast<const cv::_InputOutputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(triangulatedPoints));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_recoverPose_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_Point2d_const__InputOutputArrayX(void* E, void* points1, void* points2, void* R, void* t, double focal, cv::Point2d pp, void* mask) {
		try {
			int ret = cv::recoverPose(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t), focal, pp, *reinterpret_cast<const cv::_InputOutputArray*>(mask));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_rectify3Collinear_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_Size_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_double_Size_RectX_RectX_int(void* cameraMatrix1, void* distCoeffs1, void* cameraMatrix2, void* distCoeffs2, void* cameraMatrix3, void* distCoeffs3, void* imgpt1, void* imgpt3, cv::Size imageSize, void* R12, void* T12, void* R13, void* T13, void* R1, void* R2, void* R3, void* P1, void* P2, void* P3, void* Q, double alpha, cv::Size newImgSize, cv::Rect* roi1, cv::Rect* roi2, int flags) {
		try {
			float ret = cv::rectify3Collinear(*reinterpret_cast<const cv::_InputArray*>(cameraMatrix1), *reinterpret_cast<const cv::_InputArray*>(distCoeffs1), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix2), *reinterpret_cast<const cv::_InputArray*>(distCoeffs2), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix3), *reinterpret_cast<const cv::_InputArray*>(distCoeffs3), *reinterpret_cast<const cv::_InputArray*>(imgpt1), *reinterpret_cast<const cv::_InputArray*>(imgpt3), imageSize, *reinterpret_cast<const cv::_InputArray*>(R12), *reinterpret_cast<const cv::_InputArray*>(T12), *reinterpret_cast<const cv::_InputArray*>(R13), *reinterpret_cast<const cv::_InputArray*>(T13), *reinterpret_cast<const cv::_OutputArray*>(R1), *reinterpret_cast<const cv::_OutputArray*>(R2), *reinterpret_cast<const cv::_OutputArray*>(R3), *reinterpret_cast<const cv::_OutputArray*>(P1), *reinterpret_cast<const cv::_OutputArray*>(P2), *reinterpret_cast<const cv::_OutputArray*>(P3), *reinterpret_cast<const cv::_OutputArray*>(Q), alpha, newImgSize, roi1, roi2, flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_reprojectImageTo3D_const__InputArrayX_const__OutputArrayX_const__InputArrayX_bool_int(void* disparity, void* _3dImage, void* Q, bool handleMissingValues, int ddepth) {
		try {
			cv::reprojectImageTo3D(*reinterpret_cast<const cv::_InputArray*>(disparity), *reinterpret_cast<const cv::_OutputArray*>(_3dImage), *reinterpret_cast<const cv::_InputArray*>(Q), handleMissingValues, ddepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_sampsonDistance_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* pt1, void* pt2, void* F) {
		try {
			double ret = cv::sampsonDistance(*reinterpret_cast<const cv::_InputArray*>(pt1), *reinterpret_cast<const cv::_InputArray*>(pt2), *reinterpret_cast<const cv::_InputArray*>(F));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_solveP3P_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, int flags) {
		try {
			int ret = cv::solveP3P(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_solvePnPGeneric_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool_SolvePnPMethod_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, bool useExtrinsicGuess, cv::SolvePnPMethod flags, void* rvec, void* tvec, void* reprojectionError) {
		try {
			int ret = cv::solvePnPGeneric(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), useExtrinsicGuess, flags, *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_OutputArray*>(reprojectionError));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_solvePnPRansac_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool_int_float_double_const__OutputArrayX_int(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, double confidence, void* inliers, int flags) {
		try {
			bool ret = cv::solvePnPRansac(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvec), *reinterpret_cast<const cv::_OutputArray*>(tvec), useExtrinsicGuess, iterationsCount, reprojectionError, confidence, *reinterpret_cast<const cv::_OutputArray*>(inliers), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_solvePnPRefineLM_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_TermCriteria(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, void* criteria) {
		try {
			cv::solvePnPRefineLM(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_solvePnPRefineVVS_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_TermCriteria_double(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, void* criteria, double VVSlambda) {
		try {
			cv::solvePnPRefineVVS(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), *reinterpret_cast<cv::TermCriteria*>(criteria), VVSlambda);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_solvePnP_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool_int(void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = cv::solvePnP(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvec), *reinterpret_cast<const cv::_OutputArray*>(tvec), useExtrinsicGuess, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_stereoCalibrate_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints1, void* imagePoints2, void* cameraMatrix1, void* distCoeffs1, void* cameraMatrix2, void* distCoeffs2, cv::Size imageSize, void* R, void* T, void* E, void* F, void* perViewErrors, int flags, void* criteria) {
		try {
			double ret = cv::stereoCalibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints1), *reinterpret_cast<const cv::_InputArray*>(imagePoints2), *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix1), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs1), *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix2), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs2), imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(R), *reinterpret_cast<const cv::_InputOutputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(E), *reinterpret_cast<const cv::_OutputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(perViewErrors), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_stereoCalibrate_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_Size_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* objectPoints, void* imagePoints1, void* imagePoints2, void* cameraMatrix1, void* distCoeffs1, void* cameraMatrix2, void* distCoeffs2, cv::Size imageSize, void* R, void* T, void* E, void* F, int flags, void* criteria) {
		try {
			double ret = cv::stereoCalibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints1), *reinterpret_cast<const cv::_InputArray*>(imagePoints2), *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix1), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs1), *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix2), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs2), imageSize, *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(E), *reinterpret_cast<const cv::_OutputArray*>(F), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<bool> cv_stereoRectifyUncalibrated_const__InputArrayX_const__InputArrayX_const__InputArrayX_Size_const__OutputArrayX_const__OutputArrayX_double(void* points1, void* points2, void* F, cv::Size imgSize, void* H1, void* H2, double threshold) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*reinterpret_cast<const cv::_InputArray*>(points1), *reinterpret_cast<const cv::_InputArray*>(points2), *reinterpret_cast<const cv::_InputArray*>(F), imgSize, *reinterpret_cast<const cv::_OutputArray*>(H1), *reinterpret_cast<const cv::_OutputArray*>(H2), threshold);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_stereoRectify_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_Size_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_double_Size_RectX_RectX(void* cameraMatrix1, void* distCoeffs1, void* cameraMatrix2, void* distCoeffs2, cv::Size imageSize, void* R, void* T, void* R1, void* R2, void* P1, void* P2, void* Q, int flags, double alpha, cv::Size newImageSize, cv::Rect* validPixROI1, cv::Rect* validPixROI2) {
		try {
			cv::stereoRectify(*reinterpret_cast<const cv::_InputArray*>(cameraMatrix1), *reinterpret_cast<const cv::_InputArray*>(distCoeffs1), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix2), *reinterpret_cast<const cv::_InputArray*>(distCoeffs2), imageSize, *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(R1), *reinterpret_cast<const cv::_OutputArray*>(R2), *reinterpret_cast<const cv::_OutputArray*>(P1), *reinterpret_cast<const cv::_OutputArray*>(P2), *reinterpret_cast<const cv::_OutputArray*>(Q), flags, alpha, newImageSize, validPixROI1, validPixROI2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_triangulatePoints_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* projMatr1, void* projMatr2, void* projPoints1, void* projPoints2, void* points4D) {
		try {
			cv::triangulatePoints(*reinterpret_cast<const cv::_InputArray*>(projMatr1), *reinterpret_cast<const cv::_InputArray*>(projMatr2), *reinterpret_cast<const cv::_InputArray*>(projPoints1), *reinterpret_cast<const cv::_InputArray*>(projPoints2), *reinterpret_cast<const cv::_OutputArray*>(points4D));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_validateDisparity_const__InputOutputArrayX_const__InputArrayX_int_int_int(void* disparity, void* cost, int minDisparity, int numberOfDisparities, int disp12MaxDisp) {
		try {
			cv::validateDisparity(*reinterpret_cast<const cv::_InputOutputArray*>(disparity), *reinterpret_cast<const cv::_InputArray*>(cost), minDisparity, numberOfDisparities, disp12MaxDisp);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::CirclesGridFinderParameters> cv_CirclesGridFinderParameters_CirclesGridFinderParameters() {
		try {
			cv::CirclesGridFinderParameters ret;
			return Ok<cv::CirclesGridFinderParameters>(ret);
		} OCVRS_CATCH(Result<cv::CirclesGridFinderParameters>)
	}
	
	Result<cv::CirclesGridFinderParameters2> cv_CirclesGridFinderParameters2_CirclesGridFinderParameters2() {
		try {
			cv::CirclesGridFinderParameters2 ret;
			return Ok<cv::CirclesGridFinderParameters2>(ret);
		} OCVRS_CATCH(Result<cv::CirclesGridFinderParameters2>)
	}
	
	Result<int> cv_StereoBM_getPreFilterType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getPreFilterType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setPreFilterType_int(void* instance, int preFilterType) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setPreFilterType(preFilterType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoBM_getPreFilterSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getPreFilterSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setPreFilterSize_int(void* instance, int preFilterSize) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setPreFilterSize(preFilterSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoBM_getPreFilterCap_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getPreFilterCap();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setPreFilterCap_int(void* instance, int preFilterCap) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setPreFilterCap(preFilterCap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoBM_getTextureThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getTextureThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setTextureThreshold_int(void* instance, int textureThreshold) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setTextureThreshold(textureThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoBM_getUniquenessRatio_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getUniquenessRatio();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setUniquenessRatio_int(void* instance, int uniquenessRatio) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setUniquenessRatio(uniquenessRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoBM_getSmallerBlockSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoBM*>(instance)->getSmallerBlockSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoBM_setSmallerBlockSize_int(void* instance, int blockSize) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setSmallerBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_StereoBM_getROI1_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::StereoBM*>(instance)->getROI1();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_StereoBM_setROI1_Rect(void* instance, cv::Rect roi1) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setROI1(roi1);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_StereoBM_getROI2_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::StereoBM*>(instance)->getROI2();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_StereoBM_setROI2_Rect(void* instance, cv::Rect roi2) {
		try {
			reinterpret_cast<cv::StereoBM*>(instance)->setROI2(roi2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_StereoBM_create_int_int(int numDisparities, int blockSize) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create(numDisparities, blockSize);
			return Ok<void*>(new cv::Ptr<cv::StereoBM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_StereoMatcher_compute_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, void* left, void* right, void* disparity) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(left), *reinterpret_cast<const cv::_InputArray*>(right), *reinterpret_cast<const cv::_OutputArray*>(disparity));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getMinDisparity_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getMinDisparity();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setMinDisparity_int(void* instance, int minDisparity) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setMinDisparity(minDisparity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getNumDisparities_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getNumDisparities();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setNumDisparities_int(void* instance, int numDisparities) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setNumDisparities(numDisparities);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getBlockSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getBlockSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setBlockSize_int(void* instance, int blockSize) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getSpeckleWindowSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getSpeckleWindowSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setSpeckleWindowSize_int(void* instance, int speckleWindowSize) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setSpeckleWindowSize(speckleWindowSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getSpeckleRange_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getSpeckleRange();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setSpeckleRange_int(void* instance, int speckleRange) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setSpeckleRange(speckleRange);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoMatcher_getDisp12MaxDiff_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoMatcher*>(instance)->getDisp12MaxDiff();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoMatcher_setDisp12MaxDiff_int(void* instance, int disp12MaxDiff) {
		try {
			reinterpret_cast<cv::StereoMatcher*>(instance)->setDisp12MaxDiff(disp12MaxDiff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoSGBM_getPreFilterCap_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoSGBM*>(instance)->getPreFilterCap();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoSGBM_setPreFilterCap_int(void* instance, int preFilterCap) {
		try {
			reinterpret_cast<cv::StereoSGBM*>(instance)->setPreFilterCap(preFilterCap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoSGBM_getUniquenessRatio_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoSGBM*>(instance)->getUniquenessRatio();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoSGBM_setUniquenessRatio_int(void* instance, int uniquenessRatio) {
		try {
			reinterpret_cast<cv::StereoSGBM*>(instance)->setUniquenessRatio(uniquenessRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoSGBM_getP1_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoSGBM*>(instance)->getP1();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoSGBM_setP1_int(void* instance, int P1) {
		try {
			reinterpret_cast<cv::StereoSGBM*>(instance)->setP1(P1);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoSGBM_getP2_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoSGBM*>(instance)->getP2();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoSGBM_setP2_int(void* instance, int P2) {
		try {
			reinterpret_cast<cv::StereoSGBM*>(instance)->setP2(P2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_StereoSGBM_getMode_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::StereoSGBM*>(instance)->getMode();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_StereoSGBM_setMode_int(void* instance, int mode) {
		try {
			reinterpret_cast<cv::StereoSGBM*>(instance)->setMode(mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(int minDisparity, int numDisparities, int blockSize, int P1, int P2, int disp12MaxDiff, int preFilterCap, int uniquenessRatio, int speckleWindowSize, int speckleRange, int mode) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create(minDisparity, numDisparities, blockSize, P1, P2, disp12MaxDiff, preFilterCap, uniquenessRatio, speckleWindowSize, speckleRange, mode);
			return Ok<void*>(new cv::Ptr<cv::StereoSGBM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
