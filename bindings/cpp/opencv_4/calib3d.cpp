#include "calib3d.hpp"
#include "calib3d_types.hpp"

extern "C" {
	Result<cv::Vec3d> cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mtxR, const cv::_OutputArray* mtxQ, const cv::_OutputArray* Qx, const cv::_OutputArray* Qy, const cv::_OutputArray* Qz) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*src, *mtxR, *mtxQ, *Qx, *Qy, *Qz);
			return Ok<cv::Vec3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3d>))
	}
	
	Result_void cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* jacobian) {
		try {
			cv::Rodrigues(*src, *dst, *jacobian);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* stdDeviationsObjPoints, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *stdDeviationsObjPoints, *perViewErrors, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, int iFixedPoint, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* newObjPoints, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::calibrateCameraRO(*objectPoints, *imagePoints, *imageSize, iFixedPoint, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *newObjPoints, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::calibrateCamera(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_HandEyeCalibrationMethod(const cv::_InputArray* R_gripper2base, const cv::_InputArray* t_gripper2base, const cv::_InputArray* R_target2cam, const cv::_InputArray* t_target2cam, const cv::_OutputArray* R_cam2gripper, const cv::_OutputArray* t_cam2gripper, cv::HandEyeCalibrationMethod method) {
		try {
			cv::calibrateHandEye(*R_gripper2base, *t_gripper2base, *R_target2cam, *t_target2cam, *R_cam2gripper, *t_cam2gripper, method);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_calibrateRobotWorldHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_RobotWorldHandEyeCalibrationMethod(const cv::_InputArray* R_world2cam, const cv::_InputArray* t_world2cam, const cv::_InputArray* R_base2gripper, const cv::_InputArray* t_base2gripper, const cv::_OutputArray* R_base2world, const cv::_OutputArray* t_base2world, const cv::_OutputArray* R_gripper2cam, const cv::_OutputArray* t_gripper2cam, cv::RobotWorldHandEyeCalibrationMethod method) {
		try {
			cv::calibrateRobotWorldHandEye(*R_world2cam, *t_world2cam, *R_base2gripper, *t_base2gripper, *R_base2world, *t_base2world, *R_gripper2cam, *t_gripper2cam, method);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_calibrationMatrixValues_const__InputArrayR_Size_double_double_doubleR_doubleR_doubleR_Point2dR_doubleR(const cv::_InputArray* cameraMatrix, cv::Size* imageSize, double apertureWidth, double apertureHeight, double* fovx, double* fovy, double* focalLength, cv::Point2d* principalPoint, double* aspectRatio) {
		try {
			cv::calibrationMatrixValues(*cameraMatrix, *imageSize, apertureWidth, apertureHeight, *fovx, *fovy, *focalLength, *principalPoint, *aspectRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_checkChessboard_const__InputArrayR_Size(const cv::_InputArray* img, cv::Size* size) {
		try {
			bool ret = cv::checkChessboard(*img, *size);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* rvec1, const cv::_InputArray* tvec1, const cv::_InputArray* rvec2, const cv::_InputArray* tvec2, const cv::_OutputArray* rvec3, const cv::_OutputArray* tvec3, const cv::_OutputArray* dr3dr1, const cv::_OutputArray* dr3dt1, const cv::_OutputArray* dr3dr2, const cv::_OutputArray* dr3dt2, const cv::_OutputArray* dt3dr1, const cv::_OutputArray* dt3dt1, const cv::_OutputArray* dt3dr2, const cv::_OutputArray* dt3dt2) {
		try {
			cv::composeRT(*rvec1, *tvec1, *rvec2, *tvec2, *rvec3, *tvec3, *dr3dr1, *dr3dt1, *dr3dr2, *dr3dt2, *dt3dr1, *dt3dt1, *dt3dr2, *dt3dt2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, int whichImage, const cv::_InputArray* F, const cv::_OutputArray* lines) {
		try {
			cv::computeCorrespondEpilines(*points, whichImage, *F, *lines);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::convertPointsFromHomogeneous(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::convertPointsHomogeneous(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::convertPointsToHomogeneous(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* newPoints1, const cv::_OutputArray* newPoints2) {
		try {
			cv::correctMatches(*F, *points1, *points2, *newPoints1, *newPoints2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* t) {
		try {
			cv::decomposeEssentialMat(*E, *R1, *R2, *t);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* H, const cv::_InputArray* K, const cv::_OutputArray* rotations, const cv::_OutputArray* translations, const cv::_OutputArray* normals) {
		try {
			int ret = cv::decomposeHomographyMat(*H, *K, *rotations, *translations, *normals);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* projMatrix, const cv::_OutputArray* cameraMatrix, const cv::_OutputArray* rotMatrix, const cv::_OutputArray* transVect, const cv::_OutputArray* rotMatrixX, const cv::_OutputArray* rotMatrixY, const cv::_OutputArray* rotMatrixZ, const cv::_OutputArray* eulerAngles) {
		try {
			cv::decomposeProjectionMatrix(*projMatrix, *cameraMatrix, *rotMatrix, *transVect, *rotMatrixX, *rotMatrixY, *rotMatrixZ, *eulerAngles);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_drawChessboardCorners_const__InputOutputArrayR_Size_const__InputArrayR_bool(const cv::_InputOutputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, bool patternWasFound) {
		try {
			cv::drawChessboardCorners(*image, *patternSize, *corners, patternWasFound);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_int(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length, int thickness) {
		try {
			cv::drawFrameAxes(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length, thickness);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* pts1, const cv::_InputArray* pts2, const cv::_OutputArray* inliers, const cv::UsacParams* params) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*pts1, *pts2, *inliers, *params);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<int> cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, double ransacThreshold, double confidence) {
		try {
			int ret = cv::estimateAffine3D(*src, *dst, *out, *inliers, ransacThreshold, confidence);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Mat*> cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Scalar> cv_estimateChessboardSharpness_const__InputArrayR_Size_const__InputArrayR_float_bool_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_InputArray* corners, float rise_distance, bool vertical, const cv::_OutputArray* sharpness) {
		try {
			cv::Scalar ret = cv::estimateChessboardSharpness(*image, *patternSize, *corners, rise_distance, vertical, *sharpness);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<int> cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, double ransacThreshold, double confidence) {
		try {
			int ret = cv::estimateTranslation3D(*src, *dst, *out, *inliers, ransacThreshold, confidence);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* rotations, const cv::_InputArray* normals, const cv::_InputArray* beforePoints, const cv::_InputArray* afterPoints, const cv::_OutputArray* possibleSolutions, const cv::_InputArray* pointsMask) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*rotations, *normals, *beforePoints, *afterPoints, *possibleSolutions, *pointsMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(const cv::_InputOutputArray* img, double newVal, int maxSpeckleSize, double maxDiff, const cv::_InputOutputArray* buf) {
		try {
			cv::filterSpeckles(*img, newVal, maxSpeckleSize, maxDiff, *buf);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_find4QuadCornerSubpix_const__InputArrayR_const__InputOutputArrayR_Size(const cv::_InputArray* img, const cv::_InputOutputArray* corners, cv::Size* region_size) {
		try {
			bool ret = cv::find4QuadCornerSubpix(*img, *corners, *region_size);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags) {
		try {
			bool ret = cv::findChessboardCornersSB(*image, *patternSize, *corners, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags, const cv::_OutputArray* meta) {
		try {
			bool ret = cv::findChessboardCornersSB(*image, *patternSize, *corners, flags, *meta);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* corners, int flags) {
		try {
			bool ret = cv::findChessboardCorners(*image, *patternSize, *corners, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_Ptr_Feature2D_R(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_Ptr_Feature2D_R_const_CirclesGridFinderParametersR(const cv::_InputArray* image, cv::Size* patternSize, const cv::_OutputArray* centers, int flags, const cv::Ptr<cv::Feature2D>* blobDetector, const cv::CirclesGridFinderParameters* parameters) {
		try {
			bool ret = cv::findCirclesGrid(*image, *patternSize, *centers, flags, *blobDetector, *parameters);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Mat*> cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* dist_coeff1, const cv::_InputArray* dist_coeff2, const cv::_OutputArray* mask, const cv::UsacParams* params) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix1, *cameraMatrix2, *dist_coeff1, *dist_coeff2, *mask, *params);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, int method, double prob, double threshold, const cv::_OutputArray* mask) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, method, prob, threshold, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, int method, double prob, double threshold, const cv::_OutputArray* mask) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix, method, prob, threshold, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, double focal, cv::Point2d* pp, int method, double prob, double threshold, const cv::_OutputArray* mask) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, focal, *pp, method, prob, threshold, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, const cv::UsacParams* params) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask, *params);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, int method, double ransacReprojThreshold, double confidence) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask, method, ransacReprojThreshold, confidence);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, const cv::_OutputArray* mask) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, int maxIters, const cv::_OutputArray* mask) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, maxIters, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, const cv::UsacParams* params) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask, *params);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, int method, double ransacReprojThreshold) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask, method, ransacReprojThreshold);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, int method, double ransacReprojThreshold, const cv::_OutputArray* mask, const int maxIters, const double confidence) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, method, ransacReprojThreshold, *mask, maxIters, confidence);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<double> cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* image_size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::fisheye::calibrate(*objectPoints, *imagePoints, *image_size, *K, *D, *rvecs, *tvecs, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* K, const cv::_InputArray* D, double alpha) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *K, *D, alpha);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(const cv::_InputArray* K, const cv::_InputArray* D, const cv::Size* image_size, const cv::_InputArray* R, const cv::_OutputArray* P, double balance, const cv::Size* new_size, double fov_scale) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*K, *D, *image_size, *R, *P, balance, *new_size, fov_scale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2) {
		try {
			cv::fisheye::initUndistortRectifyMap(*K, *D, *R, *P, *size, m1type, *map1, *map2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *affine, *K, *D, alpha, *jacobian);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, *D, alpha, *jacobian);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* D2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::fisheye::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *K1, *D1, *K2, *D2, *imageSize, *R, *T, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* tvec, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, const cv::Size* newImageSize, double balance, double fov_scale) {
		try {
			cv::fisheye::stereoRectify(*K1, *D1, *K2, *D2, *imageSize, *R, *tvec, *R1, *R2, *P1, *P2, *Q, flags, *newImageSize, balance, fov_scale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* Knew, const cv::Size* new_size) {
		try {
			cv::fisheye::undistortImage(*distorted, *undistorted, *K, *D, *Knew, *new_size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P) {
		try {
			cv::fisheye::undistortPoints(*distorted, *undistorted, *K, *D, *R, *P);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_getDefaultNewCameraMatrix_const__InputArrayR_Size_bool(const cv::_InputArray* cameraMatrix, cv::Size* imgsize, bool centerPrincipalPoint) {
		try {
			cv::Mat ret = cv::getDefaultNewCameraMatrix(*cameraMatrix, *imgsize, centerPrincipalPoint);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, double alpha, cv::Size* newImgSize, cv::Rect* validPixROI, bool centerPrincipalPoint) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*cameraMatrix, *distCoeffs, *imageSize, alpha, *newImgSize, validPixROI, centerPrincipalPoint);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Rect> cv_getValidDisparityROI_Rect_Rect_int_int_int(cv::Rect* roi1, cv::Rect* roi2, int minDisparity, int numberOfDisparities, int blockSize) {
		try {
			cv::Rect ret = cv::getValidDisparityROI(*roi1, *roi2, minDisparity, numberOfDisparities, blockSize);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Mat*> cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, double aspectRatio) {
		try {
			cv::Mat ret = cv::initCameraMatrix2D(*objectPoints, *imagePoints, *imageSize, aspectRatio);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* newCameraMatrix, cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2) {
		try {
			cv::initUndistortRectifyMap(*cameraMatrix, *distCoeffs, *R, *newCameraMatrix, *size, m1type, *map1, *map2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR_UndistortTypes_double(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, int destImageWidth, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, cv::UndistortTypes projType, double alpha) {
		try {
			float ret = cv::initWideAngleProjMap(*cameraMatrix, *distCoeffs, *imageSize, destImageWidth, m1type, *map1, *map2, projType, alpha);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* dABdA, const cv::_OutputArray* dABdB) {
		try {
			cv::matMulDeriv(*A, *B, *dABdA, *dABdB);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, const cv::_OutputArray* jacobian, double aspectRatio) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints, *jacobian, aspectRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, const cv::_InputOutputArray* mask) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, *mask);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, double distanceThresh, const cv::_InputOutputArray* mask, const cv::_OutputArray* triangulatedPoints) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, distanceThresh, *mask, *triangulatedPoints);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* R, const cv::_OutputArray* t, double focal, cv::Point2d* pp, const cv::_InputOutputArray* mask) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *R, *t, focal, *pp, *mask);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, const cv::_InputArray* cameraMatrix3, const cv::_InputArray* distCoeffs3, const cv::_InputArray* imgpt1, const cv::_InputArray* imgpt3, cv::Size* imageSize, const cv::_InputArray* R12, const cv::_InputArray* T12, const cv::_InputArray* R13, const cv::_InputArray* T13, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* R3, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* P3, const cv::_OutputArray* Q, double alpha, cv::Size* newImgSize, cv::Rect* roi1, cv::Rect* roi2, int flags) {
		try {
			float ret = cv::rectify3Collinear(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *cameraMatrix3, *distCoeffs3, *imgpt1, *imgpt3, *imageSize, *R12, *T12, *R13, *T13, *R1, *R2, *R3, *P1, *P2, *P3, *Q, alpha, *newImgSize, roi1, roi2, flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(const cv::_InputArray* disparity, const cv::_OutputArray* _3dImage, const cv::_InputArray* Q, bool handleMissingValues, int ddepth) {
		try {
			cv::reprojectImageTo3D(*disparity, *_3dImage, *Q, handleMissingValues, ddepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pt1, const cv::_InputArray* pt2, const cv::_InputArray* F) {
		try {
			double ret = cv::sampsonDistance(*pt1, *pt2, *F);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags) {
		try {
			int ret = cv::solveP3P(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_SolvePnPMethod_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, bool useExtrinsicGuess, cv::SolvePnPMethod flags, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_OutputArray* reprojectionError) {
		try {
			int ret = cv::solvePnPGeneric(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, useExtrinsicGuess, flags, *rvec, *tvec, *reprojectionError);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, double confidence, const cv::_OutputArray* inliers, int flags) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, confidence, *inliers, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputOutputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* inliers, const cv::UsacParams* params) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *inliers, *params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria) {
		try {
			cv::solvePnPRefineLM(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria, double VVSlambda) {
		try {
			cv::solvePnPRefineVVS(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria, VVSlambda);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = cv::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputOutputArray* R, const cv::_InputOutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, *perViewErrors, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints1, const cv::_InputArray* imagePoints2, const cv::_InputOutputArray* cameraMatrix1, const cv::_InputOutputArray* distCoeffs1, const cv::_InputOutputArray* cameraMatrix2, const cv::_InputOutputArray* distCoeffs2, cv::Size* imageSize, const cv::_OutputArray* R, const cv::_OutputArray* T, const cv::_OutputArray* E, const cv::_OutputArray* F, int flags, cv::TermCriteria* criteria) {
		try {
			double ret = cv::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *E, *F, flags, *criteria);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<bool> cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* F, cv::Size* imgSize, const cv::_OutputArray* H1, const cv::_OutputArray* H2, double threshold) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*points1, *points2, *F, *imgSize, *H1, *H2, threshold);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, double alpha, cv::Size* newImageSize, cv::Rect* validPixROI1, cv::Rect* validPixROI2) {
		try {
			cv::stereoRectify(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *R1, *R2, *P1, *P2, *Q, flags, alpha, *newImageSize, validPixROI1, validPixROI2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* projMatr1, const cv::_InputArray* projMatr2, const cv::_InputArray* projPoints1, const cv::_InputArray* projPoints2, const cv::_OutputArray* points4D) {
		try {
			cv::triangulatePoints(*projMatr1, *projMatr2, *projPoints1, *projPoints2, *points4D);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* P) {
		try {
			cv::undistortPoints(*src, *dst, *cameraMatrix, *distCoeffs, *R, *P);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* P, cv::TermCriteria* criteria) {
		try {
			cv::undistortPoints(*src, *dst, *cameraMatrix, *distCoeffs, *R, *P, *criteria);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* newCameraMatrix) {
		try {
			cv::undistort(*src, *dst, *cameraMatrix, *distCoeffs, *newCameraMatrix);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(const cv::_InputOutputArray* disparity, const cv::_InputArray* cost, int minDisparity, int numberOfDisparities, int disp12MaxDisp) {
		try {
			cv::validateDisparity(*disparity, *cost, minDisparity, numberOfDisparities, disp12MaxDisp);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::CirclesGridFinderParameters> cv_CirclesGridFinderParameters_CirclesGridFinderParameters() {
		try {
			cv::CirclesGridFinderParameters ret;
			return Ok<cv::CirclesGridFinderParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CirclesGridFinderParameters>))
	}
	
	Result<int> cv_LMSolver_run_const_const__InputOutputArrayR(const cv::LMSolver* instance, const cv::_InputOutputArray* param) {
		try {
			int ret = instance->run(*param);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LMSolver_setMaxIters_int(cv::LMSolver* instance, int maxIters) {
		try {
			instance->setMaxIters(maxIters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LMSolver_getMaxIters_const(const cv::LMSolver* instance) {
		try {
			int ret = instance->getMaxIters();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Ptr<cv::LMSolver>*> cv_LMSolver_create_const_Ptr_Callback_R_int(const cv::Ptr<cv::LMSolver::Callback>* cb, int maxIters) {
		try {
			cv::Ptr<cv::LMSolver> ret = cv::LMSolver::create(*cb, maxIters);
			return Ok(new cv::Ptr<cv::LMSolver>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::LMSolver>*>))
	}
	
	Result<cv::Ptr<cv::LMSolver>*> cv_LMSolver_create_const_Ptr_Callback_R_int_double(const cv::Ptr<cv::LMSolver::Callback>* cb, int maxIters, double eps) {
		try {
			cv::Ptr<cv::LMSolver> ret = cv::LMSolver::create(*cb, maxIters, eps);
			return Ok(new cv::Ptr<cv::LMSolver>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::LMSolver>*>))
	}
	
	Result<bool> cv_LMSolver_Callback_compute_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::LMSolver::Callback* instance, const cv::_InputArray* param, const cv::_OutputArray* err, const cv::_OutputArray* J) {
		try {
			bool ret = instance->compute(*param, *err, *J);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_StereoBM_getPreFilterType_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getPreFilterType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setPreFilterType_int(cv::StereoBM* instance, int preFilterType) {
		try {
			instance->setPreFilterType(preFilterType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoBM_getPreFilterSize_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getPreFilterSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setPreFilterSize_int(cv::StereoBM* instance, int preFilterSize) {
		try {
			instance->setPreFilterSize(preFilterSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoBM_getPreFilterCap_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getPreFilterCap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setPreFilterCap_int(cv::StereoBM* instance, int preFilterCap) {
		try {
			instance->setPreFilterCap(preFilterCap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoBM_getTextureThreshold_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getTextureThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setTextureThreshold_int(cv::StereoBM* instance, int textureThreshold) {
		try {
			instance->setTextureThreshold(textureThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoBM_getUniquenessRatio_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getUniquenessRatio();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setUniquenessRatio_int(cv::StereoBM* instance, int uniquenessRatio) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoBM_getSmallerBlockSize_const(const cv::StereoBM* instance) {
		try {
			int ret = instance->getSmallerBlockSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoBM_setSmallerBlockSize_int(cv::StereoBM* instance, int blockSize) {
		try {
			instance->setSmallerBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_StereoBM_getROI1_const(const cv::StereoBM* instance) {
		try {
			cv::Rect ret = instance->getROI1();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_StereoBM_setROI1_Rect(cv::StereoBM* instance, cv::Rect* roi1) {
		try {
			instance->setROI1(*roi1);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_StereoBM_getROI2_const(const cv::StereoBM* instance) {
		try {
			cv::Rect ret = instance->getROI2();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_StereoBM_setROI2_Rect(cv::StereoBM* instance, cv::Rect* roi2) {
		try {
			instance->setROI2(*roi2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::StereoBM>*> cv_StereoBM_create_int_int(int numDisparities, int blockSize) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create(numDisparities, blockSize);
			return Ok(new cv::Ptr<cv::StereoBM>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::StereoBM>*>))
	}
	
	Result_void cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::StereoMatcher* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity) {
		try {
			instance->compute(*left, *right, *disparity);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getMinDisparity_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getMinDisparity();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setMinDisparity_int(cv::StereoMatcher* instance, int minDisparity) {
		try {
			instance->setMinDisparity(minDisparity);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getNumDisparities_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getNumDisparities();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setNumDisparities_int(cv::StereoMatcher* instance, int numDisparities) {
		try {
			instance->setNumDisparities(numDisparities);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getBlockSize_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getBlockSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setBlockSize_int(cv::StereoMatcher* instance, int blockSize) {
		try {
			instance->setBlockSize(blockSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getSpeckleWindowSize_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getSpeckleWindowSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setSpeckleWindowSize_int(cv::StereoMatcher* instance, int speckleWindowSize) {
		try {
			instance->setSpeckleWindowSize(speckleWindowSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getSpeckleRange_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getSpeckleRange();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setSpeckleRange_int(cv::StereoMatcher* instance, int speckleRange) {
		try {
			instance->setSpeckleRange(speckleRange);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoMatcher_getDisp12MaxDiff_const(const cv::StereoMatcher* instance) {
		try {
			int ret = instance->getDisp12MaxDiff();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoMatcher_setDisp12MaxDiff_int(cv::StereoMatcher* instance, int disp12MaxDiff) {
		try {
			instance->setDisp12MaxDiff(disp12MaxDiff);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoSGBM_getPreFilterCap_const(const cv::StereoSGBM* instance) {
		try {
			int ret = instance->getPreFilterCap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoSGBM_setPreFilterCap_int(cv::StereoSGBM* instance, int preFilterCap) {
		try {
			instance->setPreFilterCap(preFilterCap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoSGBM_getUniquenessRatio_const(const cv::StereoSGBM* instance) {
		try {
			int ret = instance->getUniquenessRatio();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoSGBM_setUniquenessRatio_int(cv::StereoSGBM* instance, int uniquenessRatio) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoSGBM_getP1_const(const cv::StereoSGBM* instance) {
		try {
			int ret = instance->getP1();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoSGBM_setP1_int(cv::StereoSGBM* instance, int P1) {
		try {
			instance->setP1(P1);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoSGBM_getP2_const(const cv::StereoSGBM* instance) {
		try {
			int ret = instance->getP2();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoSGBM_setP2_int(cv::StereoSGBM* instance, int P2) {
		try {
			instance->setP2(P2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_StereoSGBM_getMode_const(const cv::StereoSGBM* instance) {
		try {
			int ret = instance->getMode();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_StereoSGBM_setMode_int(cv::StereoSGBM* instance, int mode) {
		try {
			instance->setMode(mode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::StereoSGBM>*> cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(int minDisparity, int numDisparities, int blockSize, int P1, int P2, int disp12MaxDiff, int preFilterCap, int uniquenessRatio, int speckleWindowSize, int speckleRange, int mode) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create(minDisparity, numDisparities, blockSize, P1, P2, disp12MaxDiff, preFilterCap, uniquenessRatio, speckleWindowSize, speckleRange, mode);
			return Ok(new cv::Ptr<cv::StereoSGBM>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::StereoSGBM>*>))
	}
	
}
