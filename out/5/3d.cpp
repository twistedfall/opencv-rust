#include "ocvrs_common.hpp"
#include <opencv2/3d.hpp>
#include "3d_types.hpp"

extern "C" {
	// cv::RQDecomp3x3(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
	// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mtxR, const cv::_OutputArray* mtxQ, Result<cv::Vec3d>* ocvrs_return) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*src, *mtxR, *mtxQ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RQDecomp3x3(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
	// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ", "Qx", "Qy", "Qz"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mtxR, const cv::_OutputArray* mtxQ, const cv::_OutputArray* Qx, const cv::_OutputArray* Qy, const cv::_OutputArray* Qz, Result<cv::Vec3d>* ocvrs_return) {
		try {
			cv::Vec3d ret = cv::RQDecomp3x3(*src, *mtxR, *mtxQ, *Qx, *Qy, *Qz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Rodrigues(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
	// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Rodrigues_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::Rodrigues(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Rodrigues(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
	// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::Rodrigues(*src, *dst, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
	// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* rvec1, const cv::_InputArray* tvec1, const cv::_InputArray* rvec2, const cv::_InputArray* tvec2, const cv::_OutputArray* rvec3, const cv::_OutputArray* tvec3, ResultVoid* ocvrs_return) {
		try {
			cv::composeRT(*rvec1, *tvec1, *rvec2, *tvec2, *rvec3, *tvec3);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
	// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3", "dr3dr1", "dr3dt1", "dr3dr2", "dr3dt2", "dt3dr1", "dt3dt1", "dt3dr2", "dt3dt2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* rvec1, const cv::_InputArray* tvec1, const cv::_InputArray* rvec2, const cv::_InputArray* tvec2, const cv::_OutputArray* rvec3, const cv::_OutputArray* tvec3, const cv::_OutputArray* dr3dr1, const cv::_OutputArray* dr3dt1, const cv::_OutputArray* dr3dr2, const cv::_OutputArray* dr3dt2, const cv::_OutputArray* dt3dr1, const cv::_OutputArray* dt3dt1, const cv::_OutputArray* dt3dr2, const cv::_OutputArray* dt3dt2, ResultVoid* ocvrs_return) {
		try {
			cv::composeRT(*rvec1, *tvec1, *rvec2, *tvec2, *rvec3, *tvec3, *dr3dr1, *dr3dt1, *dr3dr2, *dr3dt2, *dt3dr1, *dt3dt1, *dt3dr2, *dt3dt2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeCorrespondEpilines(InputArray, int, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1767
	// ("cv::computeCorrespondEpilines", vec![(pred!(mut, ["points", "whichImage", "F", "lines"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, int whichImage, const cv::_InputArray* F, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			cv::computeCorrespondEpilines(*points, whichImage, *F, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::convertPointsFromHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
	// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsFromHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsFromHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
	// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsFromHomogeneous(*src, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1334
	// ("cv::convertPointsHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::convertPointsToHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
	// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsToHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertPointsToHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
	// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::convertPointsToHomogeneous(*src, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// correctMatches(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1813
	// ("cv::correctMatches", vec![(pred!(mut, ["F", "points1", "points2", "newPoints1", "newPoints2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* newPoints1, const cv::_OutputArray* newPoints2, ResultVoid* ocvrs_return) {
		try {
			cv::correctMatches(*F, *points1, *points2, *newPoints1, *newPoints2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeEssentialMat(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1554
	// ("cv::decomposeEssentialMat", vec![(pred!(mut, ["E", "R1", "R2", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* t, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeEssentialMat(*E, *R1, *R2, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeHomographyMat(InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2108
	// ("cv::decomposeHomographyMat", vec![(pred!(mut, ["H", "K", "rotations", "translations", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* H, const cv::_InputArray* K, const cv::_OutputArray* rotations, const cv::_OutputArray* translations, const cv::_OutputArray* normals, Result<int>* ocvrs_return) {
		try {
			int ret = cv::decomposeHomographyMat(*H, *K, *rotations, *translations, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
	// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* projMatrix, const cv::_OutputArray* cameraMatrix, const cv::_OutputArray* rotMatrix, const cv::_OutputArray* transVect, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeProjectionMatrix(*projMatrix, *cameraMatrix, *rotMatrix, *transVect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
	// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect", "rotMatrixX", "rotMatrixY", "rotMatrixZ", "eulerAngles"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* projMatrix, const cv::_OutputArray* cameraMatrix, const cv::_OutputArray* rotMatrix, const cv::_OutputArray* transVect, const cv::_OutputArray* rotMatrixX, const cv::_OutputArray* rotMatrixY, const cv::_OutputArray* rotMatrixZ, const cv::_OutputArray* eulerAngles, ResultVoid* ocvrs_return) {
		try {
			cv::decomposeProjectionMatrix(*projMatrix, *cameraMatrix, *rotMatrix, *transVect, *rotMatrixX, *rotMatrixY, *rotMatrixZ, *eulerAngles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:105
	// ("cv::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* in_K, const cv::_InputArray* in_points, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::depthTo3dSparse(*depth, *in_K, *in_points, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
	// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::depthTo3d(*depth, *K, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
	// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::depthTo3d(*depth, *K, *points3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
	// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float"]), _)]),
	void cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length, ResultVoid* ocvrs_return) {
		try {
			cv::drawFrameAxes(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, float, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
	// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length", "thickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "int"]), _)]),
	void cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_int(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length, int thickness, ResultVoid* ocvrs_return) {
		try {
			cv::drawFrameAxes(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length, thickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffine2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
	// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_estimateAffine2D_const__InputArrayR_const__InputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*from, *to);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine2D(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2029
	// ("cv::estimateAffine2D", vec![(pred!(mut, ["pts1", "pts2", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
	void cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* pts1, const cv::_InputArray* pts2, const cv::_OutputArray* inliers, const cv::UsacParams* params, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*pts1, *pts2, *inliers, *params);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
	// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
	void cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffine3D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine3D(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateAffine3D(*src, *dst, *out, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, double ransacThreshold, double confidence, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateAffine3D(*src, *dst, *out, *inliers, ransacThreshold, confidence);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffine3D(InputArray, InputArray, double *, bool)(InputArray, InputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
	// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "scale", "force_rotation"], ["const cv::_InputArray*", "const cv::_InputArray*", "double*", "bool"]), _)]),
	void cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_doubleX_bool(const cv::_InputArray* src, const cv::_InputArray* dst, double* scale, bool force_rotation, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffine3D(*src, *dst, scale, force_rotation);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateAffinePartial2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
	// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*from, *to);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateAffinePartial2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
	// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
	void cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* inliers, int method, double ransacReprojThreshold, size_t maxIters, double confidence, size_t refineIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateAffinePartial2D(*from, *to, *inliers, method, ransacReprojThreshold, maxIters, confidence, refineIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
	// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateTranslation3D(*src, *dst, *out, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
	// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* out, const cv::_OutputArray* inliers, double ransacThreshold, double confidence, Result<int>* ocvrs_return) {
		try {
			int ret = cv::estimateTranslation3D(*src, *dst, *out, *inliers, ransacThreshold, confidence);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
	// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
	void cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float(const cv::_OutputArray* sampled_point_flags, const cv::_InputArray* input_pts, float sampled_scale, Result<int>* ocvrs_return) {
		try {
			int ret = cv::farthestPointSampling(*sampled_point_flags, *input_pts, sampled_scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// farthestPointSampling(OutputArray, InputArray, float, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
	// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "cv::RNG*"]), _)]),
	void cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float_float_RNGX(const cv::_OutputArray* sampled_point_flags, const cv::_InputArray* input_pts, float sampled_scale, float dist_lower_limit, cv::RNG* rng, Result<int>* ocvrs_return) {
		try {
			int ret = cv::farthestPointSampling(*sampled_point_flags, *input_pts, sampled_scale, dist_lower_limit, rng);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
	// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int(const cv::_OutputArray* sampled_point_flags, const cv::_InputArray* input_pts, int sampled_pts_size, Result<int>* ocvrs_return) {
		try {
			int ret = cv::farthestPointSampling(*sampled_point_flags, *input_pts, sampled_pts_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// farthestPointSampling(OutputArray, InputArray, int, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
	// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "float", "cv::RNG*"]), _)]),
	void cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int_float_RNGX(const cv::_OutputArray* sampled_point_flags, const cv::_InputArray* input_pts, int sampled_pts_size, float dist_lower_limit, cv::RNG* rng, Result<int>* ocvrs_return) {
		try {
			int ret = cv::farthestPointSampling(*sampled_point_flags, *input_pts, sampled_pts_size, dist_lower_limit, rng);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::filterHomographyDecompByVisibleRefpoints(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
	// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* rotations, const cv::_InputArray* normals, const cv::_InputArray* beforePoints, const cv::_InputArray* afterPoints, const cv::_OutputArray* possibleSolutions, ResultVoid* ocvrs_return) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*rotations, *normals, *beforePoints, *afterPoints, *possibleSolutions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filterHomographyDecompByVisibleRefpoints(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
	// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions", "pointsMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* rotations, const cv::_InputArray* normals, const cv::_InputArray* beforePoints, const cv::_InputArray* afterPoints, const cv::_OutputArray* possibleSolutions, const cv::_InputArray* pointsMask, ResultVoid* ocvrs_return) {
		try {
			cv::filterHomographyDecompByVisibleRefpoints(*rotations, *normals, *beforePoints, *afterPoints, *possibleSolutions, *pointsMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1531
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "cameraMatrix2", "dist_coeff1", "dist_coeff2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* dist_coeff1, const cv::_InputArray* dist_coeff2, const cv::_OutputArray* mask, const cv::UsacParams* params, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix1, *cameraMatrix2, *dist_coeff1, *dist_coeff2, *mask, *params);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, int method, double prob, double threshold, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, method, prob, threshold, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, int method, double prob, double threshold, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, *cameraMatrix, method, prob, threshold, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
	// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, double focal, cv::Point2d* pp, int method, double prob, double threshold, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findEssentialMat(*points1, *points2, focal, *pp, method, prob, threshold, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1406
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, const cv::UsacParams* params, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask, *params);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, OutputArray, int, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "method", "ransacReprojThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* mask, int method, double ransacReprojThreshold, double confidence, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, *mask, method, ransacReprojThreshold, confidence);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findFundamentalMat(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, int maxIters, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, maxIters);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFundamentalMat(InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
	// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
	void cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, int method, double ransacReprojThreshold, double confidence, int maxIters, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findFundamentalMat(*points1, *points2, method, ransacReprojThreshold, confidence, maxIters, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findHomography(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findHomography(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findHomography(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:812
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, const cv::UsacParams* params, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask, *params);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findHomography(InputArray, InputArray, OutputArray, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "method", "ransacReprojThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, const cv::_OutputArray* mask, int method, double ransacReprojThreshold, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, *mask, method, ransacReprojThreshold);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findHomography(InputArray, InputArray, int, double, OutputArray, const int, const double)(InputArray, InputArray, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
	// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "method", "ransacReprojThreshold", "mask", "maxIters", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::_OutputArray*", "const int", "const double"]), _)]),
	void cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(const cv::_InputArray* srcPoints, const cv::_InputArray* dstPoints, int method, double ransacReprojThreshold, const cv::_OutputArray* mask, const int maxIters, const double confidence, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::findHomography(*srcPoints, *dstPoints, method, ransacReprojThreshold, *mask, maxIters, confidence);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findPlanes(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
	// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points3d, const cv::_InputArray* normals, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, ResultVoid* ocvrs_return) {
		try {
			cv::findPlanes(*points3d, *normals, *mask, *plane_coefficients);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findPlanes(InputArray, InputArray, OutputArray, OutputArray, int, int, double, double, double, double, RgbdPlaneMethod)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
	// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "double", "double", "cv::RgbdPlaneMethod"]), _)]),
	void cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_double_double_double_double_RgbdPlaneMethod(const cv::_InputArray* points3d, const cv::_InputArray* normals, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, cv::RgbdPlaneMethod method, ResultVoid* ocvrs_return) {
		try {
			cv::findPlanes(*points3d, *normals, *mask, *plane_coefficients, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* Kundistorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *Kundistorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* Kundistorted, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *Kundistorted, *K, *D, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distortPoints(InputArray, OutputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
	// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
	void cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(const cv::_InputArray* undistorted, const cv::_OutputArray* distorted, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::distortPoints(*undistorted, *distorted, *K, *D, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, SimpleClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
	// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* D, const cv::Size* image_size, const cv::_InputArray* R, const cv::_OutputArray* P, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*K, *D, *image_size, *R, *P);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, const Size &, InputArray, OutputArray, double, const Size &, double)(InputArray, InputArray, SimpleClass, InputArray, OutputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
	// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P", "balance", "new_size", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "double"]), _)]),
	void cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(const cv::_InputArray* K, const cv::_InputArray* D, const cv::Size* image_size, const cv::_InputArray* R, const cv::_OutputArray* P, double balance, const cv::Size* new_size, double fov_scale, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::estimateNewCameraMatrixForUndistortRectify(*K, *D, *image_size, *R, *P, balance, *new_size, fov_scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2580
	// ("cv::fisheye::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "R", "P", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::initUndistortRectifyMap(*K, *D, *R, *P, *size, m1type, *map1, *map2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *affine, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *affine, *K, *D, alpha, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
	// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::_InputArray* D, double alpha, const cv::_OutputArray* jacobian, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, *D, alpha, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
	// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::fisheye::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, TermCriteria)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
	// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "cv::TermCriteria"]), _)]),
	void cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags, cv::TermCriteria* criteria, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::fisheye::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::undistortImage(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
	// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortImage(*distorted, *undistorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, const Size &)(InputArray, OutputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
	// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "Knew", "new_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
	void cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* Knew, const cv::Size* new_size, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortImage(*distorted, *undistorted, *K, *D, *Knew, *new_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
	// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortPoints(*distorted, *undistorted, *K, *D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
	// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
	void cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* R, const cv::_InputArray* P, cv::TermCriteria* criteria, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::undistortPoints(*distorted, *undistorted, *K, *D, *R, *P, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getDefaultNewCameraMatrix(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
	// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix"], ["const cv::_InputArray*"]), _)]),
	void cv_getDefaultNewCameraMatrix_const__InputArrayR(const cv::_InputArray* cameraMatrix, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getDefaultNewCameraMatrix(*cameraMatrix);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultNewCameraMatrix(InputArray, Size, bool)(InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
	// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "imgsize", "centerPrincipalPoint"], ["const cv::_InputArray*", "cv::Size", "bool"]), _)]),
	void cv_getDefaultNewCameraMatrix_const__InputArrayR_Size_bool(const cv::_InputArray* cameraMatrix, cv::Size* imgsize, bool centerPrincipalPoint, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getDefaultNewCameraMatrix(*cameraMatrix, *imgsize, centerPrincipalPoint);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getOptimalNewCameraMatrix(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
	// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
	void cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, double alpha, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*cameraMatrix, *distCoeffs, *imageSize, alpha);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOptimalNewCameraMatrix(InputArray, InputArray, Size, double, Size, Rect *, bool)(InputArray, InputArray, SimpleClass, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
	// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha", "newImgSize", "validPixROI", "centerPrincipalPoint"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double", "cv::Size", "cv::Rect*", "bool"]), _)]),
	void cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, double alpha, cv::Size* newImgSize, cv::Rect* validPixROI, bool centerPrincipalPoint, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getOptimalNewCameraMatrix(*cameraMatrix, *distCoeffs, *imageSize, alpha, *newImgSize, validPixROI, centerPrincipalPoint);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUndistortRectangles(InputArray, InputArray, InputArray, InputArray, Size, Rect_<double> &, Rect_<double> &)(InputArray, InputArray, InputArray, InputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2380
	// ("cv::getUndistortRectangles", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "imgSize", "inner", "outer"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "cv::Rect_<double>*", "cv::Rect_<double>*"]), _)]),
	void cv_getUndistortRectangles_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_Rect_LdoubleGR_Rect_LdoubleGR(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* newCameraMatrix, cv::Size* imgSize, cv::Rect_<double>* inner, cv::Rect_<double>* outer, ResultVoid* ocvrs_return) {
		try {
			cv::getUndistortRectangles(*cameraMatrix, *distCoeffs, *R, *newCameraMatrix, *imgSize, *inner, *outer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initInverseRectificationMap(InputArray, InputArray, InputArray, InputArray, const Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2314
	// ("cv::initInverseRectificationMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_initInverseRectificationMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* newCameraMatrix, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, ResultVoid* ocvrs_return) {
		try {
			cv::initInverseRectificationMap(*cameraMatrix, *distCoeffs, *R, *newCameraMatrix, *size, m1type, *map1, *map2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, Size, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2243
	// ("cv::initUndistortRectifyMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* newCameraMatrix, cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, ResultVoid* ocvrs_return) {
		try {
			cv::initUndistortRectifyMap(*cameraMatrix, *distCoeffs, *R, *newCameraMatrix, *size, m1type, *map1, *map2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::initWideAngleProjMap(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
	// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, int destImageWidth, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, Result<float>* ocvrs_return) {
		try {
			float ret = cv::initWideAngleProjMap(*cameraMatrix, *distCoeffs, *imageSize, destImageWidth, m1type, *map1, *map2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initWideAngleProjMap(InputArray, InputArray, Size, int, int, OutputArray, OutputArray, enum UndistortTypes, double)(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
	// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2", "projType", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::UndistortTypes", "double"]), _)]),
	void cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR_UndistortTypes_double(const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Size* imageSize, int destImageWidth, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, cv::UndistortTypes projType, double alpha, Result<float>* ocvrs_return) {
		try {
			float ret = cv::initWideAngleProjMap(*cameraMatrix, *distCoeffs, *imageSize, destImageWidth, m1type, *map1, *map2, projType, alpha);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::loadMesh(InString, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
	// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR(const char* filename, const cv::_OutputArray* vertices, const cv::_OutputArray* indices, ResultVoid* ocvrs_return) {
		try {
			cv::loadMesh(std::string(filename), *vertices, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadMesh(const String &, OutputArray, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
	// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const char* filename, const cv::_OutputArray* vertices, const cv::_OutputArray* indices, const cv::_OutputArray* normals, const cv::_OutputArray* colors, const cv::_OutputArray* texCoords, ResultVoid* ocvrs_return) {
		try {
			cv::loadMesh(std::string(filename), *vertices, *indices, *normals, *colors, *texCoords);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::loadPointCloud(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
	// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_loadPointCloud_const_StringR_const__OutputArrayR(const char* filename, const cv::_OutputArray* vertices, ResultVoid* ocvrs_return) {
		try {
			cv::loadPointCloud(std::string(filename), *vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadPointCloud(const String &, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
	// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_loadPointCloud_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const char* filename, const cv::_OutputArray* vertices, const cv::_OutputArray* normals, const cv::_OutputArray* rgb, ResultVoid* ocvrs_return) {
		try {
			cv::loadPointCloud(std::string(filename), *vertices, *normals, *rgb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matMulDeriv(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:881
	// ("cv::matMulDeriv", vec![(pred!(mut, ["A", "B", "dABdA", "dABdB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* dABdA, const cv::_OutputArray* dABdB, ResultVoid* ocvrs_return) {
		try {
			cv::matMulDeriv(*A, *B, *dABdA, *dABdB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::normalEstimate(OutputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
	// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_OutputArray* normals, const cv::_OutputArray* curvatures, const cv::_InputArray* input_pts, const cv::_InputArray* nn_idx, ResultVoid* ocvrs_return) {
		try {
			cv::normalEstimate(*normals, *curvatures, *input_pts, *nn_idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalEstimate(OutputArray, OutputArray, InputArray, InputArrayOfArrays, int)(OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
	// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx", "max_neighbor_num"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_OutputArray* normals, const cv::_OutputArray* curvatures, const cv::_InputArray* input_pts, const cv::_InputArray* nn_idx, int max_neighbor_num, ResultVoid* ocvrs_return) {
		try {
			cv::normalEstimate(*normals, *curvatures, *input_pts, *nn_idx, max_neighbor_num);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, const cv::_OutputArray* dpdr, const cv::_OutputArray* dpdt, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints, *dpdr, *dpdt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt", "dpdf", "dpdc", "dpdk", "dpdo", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, const cv::_OutputArray* dpdr, const cv::_OutputArray* dpdt, const cv::_OutputArray* dpdf, const cv::_OutputArray* dpdc, const cv::_OutputArray* dpdk, const cv::_OutputArray* dpdo, double aspectRatio, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints, *dpdr, *dpdt, *dpdf, *dpdc, *dpdk, *dpdo, aspectRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
	// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "jacobian", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* objectPoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* imagePoints, const cv::_OutputArray* jacobian, double aspectRatio, ResultVoid* ocvrs_return) {
		try {
			cv::projectPoints(*objectPoints, *rvec, *tvec, *cameraMatrix, *distCoeffs, *imagePoints, *jacobian, aspectRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
	// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
	void cv_randomSampling_const__OutputArrayR_const__InputArrayR_float(const cv::_OutputArray* sampled_pts, const cv::_InputArray* input_pts, float sampled_scale, ResultVoid* ocvrs_return) {
		try {
			cv::randomSampling(*sampled_pts, *input_pts, sampled_scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randomSampling(OutputArray, InputArray, float, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
	// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "cv::RNG*"]), _)]),
	void cv_randomSampling_const__OutputArrayR_const__InputArrayR_float_RNGX(const cv::_OutputArray* sampled_pts, const cv::_InputArray* input_pts, float sampled_scale, cv::RNG* rng, ResultVoid* ocvrs_return) {
		try {
			cv::randomSampling(*sampled_pts, *input_pts, sampled_scale, rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
	// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_randomSampling_const__OutputArrayR_const__InputArrayR_int(const cv::_OutputArray* sampled_pts, const cv::_InputArray* input_pts, int sampled_pts_size, ResultVoid* ocvrs_return) {
		try {
			cv::randomSampling(*sampled_pts, *input_pts, sampled_pts_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randomSampling(OutputArray, InputArray, int, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
	// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::RNG*"]), _)]),
	void cv_randomSampling_const__OutputArrayR_const__InputArrayR_int_RNGX(const cv::_OutputArray* sampled_pts, const cv::_InputArray* input_pts, int sampled_pts_size, cv::RNG* rng, ResultVoid* ocvrs_return) {
		try {
			cv::randomSampling(*sampled_pts, *input_pts, sampled_pts_size, rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
	// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, const cv::_OutputArray* E, const cv::_OutputArray* R, const cv::_OutputArray* t, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*points1, *points2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *E, *R, *t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, int, double, double, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
	// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputOutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_double_const__InputOutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, const cv::_OutputArray* E, const cv::_OutputArray* R, const cv::_OutputArray* t, int method, double prob, double threshold, const cv::_InputOutputArray* mask, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*points1, *points2, *cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *E, *R, *t, method, prob, threshold, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, const cv::_InputOutputArray* mask, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, double distanceThresh, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, distanceThresh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double, InputOutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh", "mask", "triangulatedPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* R, const cv::_OutputArray* t, double distanceThresh, const cv::_InputOutputArray* mask, const cv::_OutputArray* triangulatedPoints, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *cameraMatrix, *R, *t, distanceThresh, *mask, *triangulatedPoints);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* R, const cv::_OutputArray* t, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *R, *t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray, double, Point2d, InputOutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, SimpleClass, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
	// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t", "focal", "pp", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Point2d", "const cv::_InputOutputArray*"]), _)]),
	void cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(const cv::_InputArray* E, const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_OutputArray* R, const cv::_OutputArray* t, double focal, cv::Point2d* pp, const cv::_InputOutputArray* mask, Result<int>* ocvrs_return) {
		try {
			int ret = cv::recoverPose(*E, *points1, *points2, *R, *t, focal, *pp, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
	// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
	void cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, ResultVoid* ocvrs_return) {
		try {
			cv::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
	// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, bool depthDilation, ResultVoid* ocvrs_return) {
		try {
			cv::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth, depthDilation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
	// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* in, int type, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			cv::rescaleDepth(*in, type, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
	// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
	void cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(const cv::_InputArray* in, int type, const cv::_OutputArray* out, double depth_factor, ResultVoid* ocvrs_return) {
		try {
			cv::rescaleDepth(*in, type, *out, depth_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sampsonDistance(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1833
	// ("cv::sampsonDistance", vec![(pred!(mut, ["pt1", "pt2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pt1, const cv::_InputArray* pt2, const cv::_InputArray* F, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sampsonDistance(*pt1, *pt2, *F);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saveMesh(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
	// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR(const char* filename, const cv::_InputArray* vertices, const cv::_InputArray* indices, ResultVoid* ocvrs_return) {
		try {
			cv::saveMesh(std::string(filename), *vertices, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// saveMesh(const String &, InputArray, InputArrayOfArrays, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
	// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const char* filename, const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputArray* normals, const cv::_InputArray* colors, const cv::_InputArray* texCoords, ResultVoid* ocvrs_return) {
		try {
			cv::saveMesh(std::string(filename), *vertices, *indices, *normals, *colors, *texCoords);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::savePointCloud(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
	// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_savePointCloud_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* vertices, ResultVoid* ocvrs_return) {
		try {
			cv::savePointCloud(std::string(filename), *vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// savePointCloud(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
	// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_savePointCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const char* filename, const cv::_InputArray* vertices, const cv::_InputArray* normals, const cv::_InputArray* rgb, ResultVoid* ocvrs_return) {
		try {
			cv::savePointCloud(std::string(filename), *vertices, *normals, *rgb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveP3P(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1132
	// ("cv::solveP3P", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveP3P(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
	// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solvePnPGeneric(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, bool, int, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
	// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "useExtrinsicGuess", "flags", "rvec", "tvec", "reprojectionError"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, bool useExtrinsicGuess, int flags, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_OutputArray* reprojectionError, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solvePnPGeneric(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, useExtrinsicGuess, flags, *rvec, *tvec, *reprojectionError);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, double, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "confidence", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "double", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, double confidence, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, confidence, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputOutputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* inliers, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
	// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
	void cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_UsacParamsR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputOutputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* inliers, const cv::UsacParams* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnPRansac(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *inliers, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
	// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineLM(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
	// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria"]), _)]),
	void cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineLM(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
	// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineVVS(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria, double)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
	// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria", "VVSlambda"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria", "double"]), _)]),
	void cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, cv::TermCriteria* criteria, double VVSlambda, ResultVoid* ocvrs_return) {
		try {
			cv::solvePnPRefineVVS(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, *criteria, VVSlambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
	// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
	// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
	void cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solvePnP(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
	// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
	void cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputArray* colors, const cv::_InputOutputArray* colorBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterizeColor(*vertices, *indices, *colors, *colorBuf, *world2cam, fovY, zNear, zFar);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
	// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
	void cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputArray* colors, const cv::_InputOutputArray* colorBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, const cv::TriangleRasterizeSettings* settings, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterizeColor(*vertices, *indices, *colors, *colorBuf, *world2cam, fovY, zNear, zFar, *settings);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
	// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
	void cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputOutputArray* depthBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterizeDepth(*vertices, *indices, *depthBuf, *world2cam, fovY, zNear, zFar);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
	// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
	void cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputOutputArray* depthBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, const cv::TriangleRasterizeSettings* settings, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterizeDepth(*vertices, *indices, *depthBuf, *world2cam, fovY, zNear, zFar, *settings);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
	// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
	void cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputArray* colors, const cv::_InputOutputArray* colorBuf, const cv::_InputOutputArray* depthBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterize(*vertices, *indices, *colors, *colorBuf, *depthBuf, *world2cam, fovY, zNear, zFar);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
	// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
	void cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(const cv::_InputArray* vertices, const cv::_InputArray* indices, const cv::_InputArray* colors, const cv::_InputOutputArray* colorBuf, const cv::_InputOutputArray* depthBuf, const cv::_InputArray* world2cam, double fovY, double zNear, double zFar, const cv::TriangleRasterizeSettings* settings, ResultVoid* ocvrs_return) {
		try {
			cv::triangleRasterize(*vertices, *indices, *colors, *colorBuf, *depthBuf, *world2cam, fovY, zNear, zFar, *settings);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangulatePoints(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1794
	// ("cv::triangulatePoints", vec![(pred!(mut, ["projMatr1", "projMatr2", "projPoints1", "projPoints2", "points4D"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* projMatr1, const cv::_InputArray* projMatr2, const cv::_InputArray* projPoints1, const cv::_InputArray* projPoints2, const cv::_OutputArray* points4D, ResultVoid* ocvrs_return) {
		try {
			cv::triangulatePoints(*projMatr1, *projMatr2, *projPoints1, *projPoints2, *points4D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::undistortImagePoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
	// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, ResultVoid* ocvrs_return) {
		try {
			cv::undistortImagePoints(*src, *dst, *cameraMatrix, *distCoeffs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortImagePoints(InputArray, OutputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
	// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "unnamed"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
	void cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::TermCriteria* unnamed, ResultVoid* ocvrs_return) {
		try {
			cv::undistortImagePoints(*src, *dst, *cameraMatrix, *distCoeffs, *unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
	// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, ResultVoid* ocvrs_return) {
		try {
			cv::undistortPoints(*src, *dst, *cameraMatrix, *distCoeffs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
	// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
	void cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* R, const cv::_InputArray* P, cv::TermCriteria* criteria, ResultVoid* ocvrs_return) {
		try {
			cv::undistortPoints(*src, *dst, *cameraMatrix, *distCoeffs, *R, *P, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::undistort(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
	// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, ResultVoid* ocvrs_return) {
		try {
			cv::undistort(*src, *dst, *cameraMatrix, *distCoeffs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// undistort(InputArray, OutputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
	// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "newCameraMatrix"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* newCameraMatrix, ResultVoid* ocvrs_return) {
		try {
			cv::undistort(*src, *dst, *cameraMatrix, *distCoeffs, *newCameraMatrix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// voxelGridSampling(OutputArray, InputArray, float, float, float)(OutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:185
	// ("cv::voxelGridSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "length", "width", "height"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "float"]), _)]),
	void cv_voxelGridSampling_const__OutputArrayR_const__InputArrayR_float_float_float(const cv::_OutputArray* sampled_point_flags, const cv::_InputArray* input_pts, float length, float width, float height, Result<int>* ocvrs_return) {
		try {
			int ret = cv::voxelGridSampling(*sampled_point_flags, *input_pts, length, width, height);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
	// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_InputArray* Rt, const cv::_InputArray* cameraMatrix, ResultVoid* ocvrs_return) {
		try {
			cv::warpFrame(*depth, *image, *mask, *Rt, *cameraMatrix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
	// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix", "warpedDepth", "warpedImage", "warpedMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_InputArray* Rt, const cv::_InputArray* cameraMatrix, const cv::_OutputArray* warpedDepth, const cv::_OutputArray* warpedImage, const cv::_OutputArray* warpedMask, ResultVoid* ocvrs_return) {
		try {
			cv::warpFrame(*depth, *image, *mask, *Rt, *cameraMatrix, *warpedDepth, *warpedImage, *warpedMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// optimize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:714
	// ("cv::LevMarq::optimize", vec![(pred!(mut, [], []), _)]),
	void cv_LevMarq_optimize(cv::LevMarq* instance, Result<cv::LevMarq::Report*>* ocvrs_return) {
		try {
			cv::LevMarq::Report ret = instance->optimize();
			Ok(new cv::LevMarq::Report(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:729
	// ("cv::LevMarq::run", vec![(pred!(mut, ["param"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_LevMarq_run_const__InputOutputArrayR(cv::LevMarq* instance, const cv::_InputOutputArray* param, Result<cv::LevMarq::Report*>* ocvrs_return) {
		try {
			cv::LevMarq::Report ret = instance->run(*param);
			Ok(new cv::LevMarq::Report(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LevMarq::delete() generated
	// ("cv::LevMarq::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LevMarq_delete(cv::LevMarq* instance) {
			delete instance;
	}

	// Report(bool, int, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:530
	// ("cv::LevMarq::Report::Report", vec![(pred!(mut, ["isFound", "nIters", "finalEnergy"], ["bool", "int", "double"]), _)]),
	void cv_LevMarq_Report_Report_bool_int_double(bool isFound, int nIters, double finalEnergy, Result<cv::LevMarq::Report*>* ocvrs_return) {
		try {
			cv::LevMarq::Report* ret = new cv::LevMarq::Report(isFound, nIters, finalEnergy);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LevMarq::Report::found() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
	// ("cv::LevMarq::Report::found", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Report_propFound_const(const cv::LevMarq::Report* instance) {
			bool ret = instance->found;
			return ret;
	}

	// cv::LevMarq::Report::setFound(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
	// ("cv::LevMarq::Report::setFound", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Report_propFound_const_bool(cv::LevMarq::Report* instance, const bool val) {
			instance->found = val;
	}

	// cv::LevMarq::Report::iters() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
	// ("cv::LevMarq::Report::iters", vec![(pred!(const, [], []), _)]),
	int cv_LevMarq_Report_propIters_const(const cv::LevMarq::Report* instance) {
			int ret = instance->iters;
			return ret;
	}

	// cv::LevMarq::Report::setIters(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
	// ("cv::LevMarq::Report::setIters", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LevMarq_Report_propIters_const_int(cv::LevMarq::Report* instance, const int val) {
			instance->iters = val;
	}

	// cv::LevMarq::Report::energy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
	// ("cv::LevMarq::Report::energy", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Report_propEnergy_const(const cv::LevMarq::Report* instance) {
			double ret = instance->energy;
			return ret;
	}

	// cv::LevMarq::Report::setEnergy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
	// ("cv::LevMarq::Report::setEnergy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Report_propEnergy_const_double(cv::LevMarq::Report* instance, const double val) {
			instance->energy = val;
	}

	// cv::LevMarq::Report::delete() generated
	// ("cv::LevMarq::Report::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LevMarq_Report_delete(cv::LevMarq::Report* instance) {
			delete instance;
	}

	// Settings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:551
	// ("cv::LevMarq::Settings::Settings", vec![(pred!(mut, [], []), _)]),
	void cv_LevMarq_Settings_Settings(Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings* ret = new cv::LevMarq::Settings();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setJacobiScaling(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:553
	// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setJacobiScaling_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setJacobiScaling(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpDouble(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:554
	// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setUpDouble_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setUpDouble(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseStepQuality(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:555
	// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setUseStepQuality_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setUseStepQuality(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClampDiagonal(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:556
	// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setClampDiagonal_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setClampDiagonal(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStepNormInf(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:557
	// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setStepNormInf_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setStepNormInf(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCheckRelEnergyChange(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:558
	// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setCheckRelEnergyChange_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setCheckRelEnergyChange(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCheckMinGradient(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:559
	// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setCheckMinGradient_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setCheckMinGradient(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCheckStepNorm(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:560
	// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setCheckStepNorm_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setCheckStepNorm(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGeodesic(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:561
	// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_LevMarq_Settings_setGeodesic_bool(cv::LevMarq::Settings* instance, bool v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setGeodesic(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHGeo(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:562
	// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setHGeo_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setHGeo(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGeoScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:563
	// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setGeoScale_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setGeoScale(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStepNormTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:564
	// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setStepNormTolerance_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setStepNormTolerance(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRelEnergyDeltaTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:565
	// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setRelEnergyDeltaTolerance_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setRelEnergyDeltaTolerance(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:566
	// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setMinGradientTolerance_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setMinGradientTolerance(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmallEnergyTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:567
	// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setSmallEnergyTolerance_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setSmallEnergyTolerance(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:568
	// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["v"], ["int"]), _)]),
	void cv_LevMarq_Settings_setMaxIterations_int(cv::LevMarq::Settings* instance, int v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setMaxIterations(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:569
	// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setInitialLambda_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setInitialLambda(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialLmUpFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:570
	// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setInitialLmUpFactor_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setInitialLmUpFactor(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialLmDownFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:571
	// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
	void cv_LevMarq_Settings_setInitialLmDownFactor_double(cv::LevMarq::Settings* instance, double v, Result<cv::LevMarq::Settings*>* ocvrs_return) {
		try {
			cv::LevMarq::Settings ret = instance->setInitialLmDownFactor(v);
			Ok(new cv::LevMarq::Settings(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LevMarq::Settings::jacobiScaling() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
	// ("cv::LevMarq::Settings::jacobiScaling", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propJacobiScaling_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->jacobiScaling;
			return ret;
	}

	// cv::LevMarq::Settings::setJacobiScaling(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
	// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propJacobiScaling_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->jacobiScaling = val;
	}

	// cv::LevMarq::Settings::upDouble() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
	// ("cv::LevMarq::Settings::upDouble", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propUpDouble_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->upDouble;
			return ret;
	}

	// cv::LevMarq::Settings::setUpDouble(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
	// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propUpDouble_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->upDouble = val;
	}

	// cv::LevMarq::Settings::useStepQuality() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
	// ("cv::LevMarq::Settings::useStepQuality", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propUseStepQuality_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->useStepQuality;
			return ret;
	}

	// cv::LevMarq::Settings::setUseStepQuality(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
	// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propUseStepQuality_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->useStepQuality = val;
	}

	// cv::LevMarq::Settings::clampDiagonal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
	// ("cv::LevMarq::Settings::clampDiagonal", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propClampDiagonal_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->clampDiagonal;
			return ret;
	}

	// cv::LevMarq::Settings::setClampDiagonal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
	// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propClampDiagonal_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->clampDiagonal = val;
	}

	// cv::LevMarq::Settings::stepNormInf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
	// ("cv::LevMarq::Settings::stepNormInf", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propStepNormInf_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->stepNormInf;
			return ret;
	}

	// cv::LevMarq::Settings::setStepNormInf(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
	// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propStepNormInf_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->stepNormInf = val;
	}

	// cv::LevMarq::Settings::checkRelEnergyChange() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
	// ("cv::LevMarq::Settings::checkRelEnergyChange", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propCheckRelEnergyChange_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->checkRelEnergyChange;
			return ret;
	}

	// cv::LevMarq::Settings::setCheckRelEnergyChange(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
	// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propCheckRelEnergyChange_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->checkRelEnergyChange = val;
	}

	// cv::LevMarq::Settings::checkMinGradient() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
	// ("cv::LevMarq::Settings::checkMinGradient", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propCheckMinGradient_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->checkMinGradient;
			return ret;
	}

	// cv::LevMarq::Settings::setCheckMinGradient(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
	// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propCheckMinGradient_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->checkMinGradient = val;
	}

	// cv::LevMarq::Settings::checkStepNorm() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
	// ("cv::LevMarq::Settings::checkStepNorm", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propCheckStepNorm_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->checkStepNorm;
			return ret;
	}

	// cv::LevMarq::Settings::setCheckStepNorm(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
	// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propCheckStepNorm_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->checkStepNorm = val;
	}

	// cv::LevMarq::Settings::geodesic() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
	// ("cv::LevMarq::Settings::geodesic", vec![(pred!(const, [], []), _)]),
	bool cv_LevMarq_Settings_propGeodesic_const(const cv::LevMarq::Settings* instance) {
			bool ret = instance->geodesic;
			return ret;
	}

	// cv::LevMarq::Settings::setGeodesic(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
	// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LevMarq_Settings_propGeodesic_const_bool(cv::LevMarq::Settings* instance, const bool val) {
			instance->geodesic = val;
	}

	// cv::LevMarq::Settings::hGeo() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
	// ("cv::LevMarq::Settings::hGeo", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propHGeo_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->hGeo;
			return ret;
	}

	// cv::LevMarq::Settings::setHGeo(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
	// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propHGeo_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->hGeo = val;
	}

	// cv::LevMarq::Settings::geoScale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
	// ("cv::LevMarq::Settings::geoScale", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propGeoScale_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->geoScale;
			return ret;
	}

	// cv::LevMarq::Settings::setGeoScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
	// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propGeoScale_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->geoScale = val;
	}

	// cv::LevMarq::Settings::stepNormTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
	// ("cv::LevMarq::Settings::stepNormTolerance", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propStepNormTolerance_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->stepNormTolerance;
			return ret;
	}

	// cv::LevMarq::Settings::setStepNormTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
	// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propStepNormTolerance_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->stepNormTolerance = val;
	}

	// cv::LevMarq::Settings::relEnergyDeltaTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
	// ("cv::LevMarq::Settings::relEnergyDeltaTolerance", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->relEnergyDeltaTolerance;
			return ret;
	}

	// cv::LevMarq::Settings::setRelEnergyDeltaTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
	// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->relEnergyDeltaTolerance = val;
	}

	// cv::LevMarq::Settings::minGradientTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
	// ("cv::LevMarq::Settings::minGradientTolerance", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propMinGradientTolerance_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->minGradientTolerance;
			return ret;
	}

	// cv::LevMarq::Settings::setMinGradientTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
	// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propMinGradientTolerance_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->minGradientTolerance = val;
	}

	// cv::LevMarq::Settings::smallEnergyTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
	// ("cv::LevMarq::Settings::smallEnergyTolerance", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propSmallEnergyTolerance_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->smallEnergyTolerance;
			return ret;
	}

	// cv::LevMarq::Settings::setSmallEnergyTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
	// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propSmallEnergyTolerance_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->smallEnergyTolerance = val;
	}

	// cv::LevMarq::Settings::maxIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
	// ("cv::LevMarq::Settings::maxIterations", vec![(pred!(const, [], []), _)]),
	unsigned int cv_LevMarq_Settings_propMaxIterations_const(const cv::LevMarq::Settings* instance) {
			unsigned int ret = instance->maxIterations;
			return ret;
	}

	// cv::LevMarq::Settings::setMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
	// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
	void cv_LevMarq_Settings_propMaxIterations_const_unsigned_int(cv::LevMarq::Settings* instance, const unsigned int val) {
			instance->maxIterations = val;
	}

	// cv::LevMarq::Settings::initialLambda() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
	// ("cv::LevMarq::Settings::initialLambda", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propInitialLambda_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->initialLambda;
			return ret;
	}

	// cv::LevMarq::Settings::setInitialLambda(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
	// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propInitialLambda_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->initialLambda = val;
	}

	// cv::LevMarq::Settings::initialLmUpFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
	// ("cv::LevMarq::Settings::initialLmUpFactor", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propInitialLmUpFactor_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->initialLmUpFactor;
			return ret;
	}

	// cv::LevMarq::Settings::setInitialLmUpFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
	// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propInitialLmUpFactor_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->initialLmUpFactor = val;
	}

	// cv::LevMarq::Settings::initialLmDownFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
	// ("cv::LevMarq::Settings::initialLmDownFactor", vec![(pred!(const, [], []), _)]),
	double cv_LevMarq_Settings_propInitialLmDownFactor_const(const cv::LevMarq::Settings* instance) {
			double ret = instance->initialLmDownFactor;
			return ret;
	}

	// cv::LevMarq::Settings::setInitialLmDownFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
	// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_LevMarq_Settings_propInitialLmDownFactor_const_double(cv::LevMarq::Settings* instance, const double val) {
			instance->initialLmDownFactor = val;
	}

	// cv::LevMarq::Settings::delete() generated
	// ("cv::LevMarq::Settings::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LevMarq_Settings_delete(cv::LevMarq::Settings* instance) {
			delete instance;
	}

	// Octree()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2691
	// ("cv::Octree::Octree", vec![(pred!(mut, [], []), _)]),
	void cv_Octree_Octree(Result<cv::Octree*>* ocvrs_return) {
		try {
			cv::Octree* ret = new cv::Octree();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWithDepth(int, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size", "origin", "withColors"], ["int", "double", "const cv::Point3f*", "bool"]), _)]),
	void cv_Octree_createWithDepth_int_double_const_Point3fR_bool(int maxDepth, double size, const cv::Point3f* origin, bool withColors, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithDepth(maxDepth, size, *origin, withColors);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::createWithDepth(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size"], ["int", "double"]), _)]),
	void cv_Octree_createWithDepth_int_double(int maxDepth, double size, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithDepth(maxDepth, size);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWithDepth(int, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud", "colors"], ["int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Octree_createWithDepth_int_const__InputArrayR_const__InputArrayR(int maxDepth, const cv::_InputArray* pointCloud, const cv::_InputArray* colors, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithDepth(maxDepth, *pointCloud, *colors);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::createWithDepth(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud"], ["int", "const cv::_InputArray*"]), _)]),
	void cv_Octree_createWithDepth_int_const__InputArrayR(int maxDepth, const cv::_InputArray* pointCloud, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithDepth(maxDepth, *pointCloud);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWithResolution(double, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size", "origin", "withColors"], ["double", "double", "const cv::Point3f*", "bool"]), _)]),
	void cv_Octree_createWithResolution_double_double_const_Point3fR_bool(double resolution, double size, const cv::Point3f* origin, bool withColors, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithResolution(resolution, size, *origin, withColors);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::createWithResolution(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size"], ["double", "double"]), _)]),
	void cv_Octree_createWithResolution_double_double(double resolution, double size, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithResolution(resolution, size);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWithResolution(double, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud", "colors"], ["double", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Octree_createWithResolution_double_const__InputArrayR_const__InputArrayR(double resolution, const cv::_InputArray* pointCloud, const cv::_InputArray* colors, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithResolution(resolution, *pointCloud, *colors);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::createWithResolution(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud"], ["double", "const cv::_InputArray*"]), _)]),
	void cv_Octree_createWithResolution_double_const__InputArrayR(double resolution, const cv::_InputArray* pointCloud, Result<cv::Ptr<cv::Octree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Octree> ret = cv::Octree::createWithResolution(resolution, *pointCloud);
			Ok(new cv::Ptr<cv::Octree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// insertPoint(const Point3f &, const Point3f &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
	// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point", "color"], ["const cv::Point3f*", "const cv::Point3f*"]), _)]),
	void cv_Octree_insertPoint_const_Point3fR_const_Point3fR(cv::Octree* instance, const cv::Point3f* point, const cv::Point3f* color, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->insertPoint(*point, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::insertPoint(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
	// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
	void cv_Octree_insertPoint_const_Point3fR(cv::Octree* instance, const cv::Point3f* point, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->insertPoint(*point);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isPointInBound(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2752
	// ("cv::Octree::isPointInBound", vec![(pred!(const, ["point"], ["const cv::Point3f*"]), _)]),
	void cv_Octree_isPointInBound_const_const_Point3fR(const cv::Octree* instance, const cv::Point3f* point, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isPointInBound(*point);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2755
	// ("cv::Octree::empty", vec![(pred!(const, [], []), _)]),
	void cv_Octree_empty_const(const cv::Octree* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2761
	// ("cv::Octree::clear", vec![(pred!(mut, [], []), _)]),
	void cv_Octree_clear(cv::Octree* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deletePoint(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2771
	// ("cv::Octree::deletePoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
	void cv_Octree_deletePoint_const_Point3fR(cv::Octree* instance, const cv::Point3f* point, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->deletePoint(*point);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPointCloudByOctree(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
	// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud", "restoredColor"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_getPointCloudByOctree_const__OutputArrayR_const__OutputArrayR(cv::Octree* instance, const cv::_OutputArray* restoredPointCloud, const cv::_OutputArray* restoredColor, ResultVoid* ocvrs_return) {
		try {
			instance->getPointCloudByOctree(*restoredPointCloud, *restoredColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::getPointCloudByOctree(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
	// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud"], ["const cv::_OutputArray*"]), _)]),
	void cv_Octree_getPointCloudByOctree_const__OutputArrayR(cv::Octree* instance, const cv::_OutputArray* restoredPointCloud, ResultVoid* ocvrs_return) {
		try {
			instance->getPointCloudByOctree(*restoredPointCloud);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, float radius, const cv::_OutputArray* points, const cv::_OutputArray* squareDists, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radiusNNSearch(*query, radius, *points, *squareDists);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::radiusNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points"], ["const cv::Point3f*", "float", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, float radius, const cv::_OutputArray* points, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radiusNNSearch(*query, radius, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2810
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "colors", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, float radius, const cv::_OutputArray* points, const cv::_OutputArray* colors, const cv::_OutputArray* squareDists, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radiusNNSearch(*query, radius, *points, *colors, *squareDists);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KNNSearch(const Point3f &, const int, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, const int K, const cv::_OutputArray* points, const cv::_OutputArray* squareDists, ResultVoid* ocvrs_return) {
		try {
			instance->KNNSearch(*query, K, *points, *squareDists);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::KNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, const int K, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->KNNSearch(*query, K, *points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KNNSearch(const Point3f &, const int, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2836
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "colors", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Octree* instance, const cv::Point3f* query, const int K, const cv::_OutputArray* points, const cv::_OutputArray* colors, const cv::_OutputArray* squareDists, ResultVoid* ocvrs_return) {
		try {
			instance->KNNSearch(*query, K, *points, *colors, *squareDists);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Octree::delete() generated
	// ("cv::Octree::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Octree_delete(cv::Octree* instance) {
			delete instance;
	}

	// Odometry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:41
	// ("cv::Odometry::Odometry", vec![(pred!(mut, [], []), _)]),
	void cv_Odometry_Odometry(Result<cv::Odometry*>* ocvrs_return) {
		try {
			cv::Odometry* ret = new cv::Odometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Odometry(OdometryType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:42
	// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype"], ["cv::OdometryType"]), _)]),
	void cv_Odometry_Odometry_OdometryType(cv::OdometryType otype, Result<cv::Odometry*>* ocvrs_return) {
		try {
			cv::Odometry* ret = new cv::Odometry(otype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Odometry(OdometryType, const OdometrySettings &, OdometryAlgoType)(Enum, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:43
	// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype", "settings", "algtype"], ["cv::OdometryType", "const cv::OdometrySettings*", "cv::OdometryAlgoType"]), _)]),
	void cv_Odometry_Odometry_OdometryType_const_OdometrySettingsR_OdometryAlgoType(cv::OdometryType otype, const cv::OdometrySettings* settings, cv::OdometryAlgoType algtype, Result<cv::Odometry*>* ocvrs_return) {
		try {
			cv::Odometry* ret = new cv::Odometry(otype, *settings, algtype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrame(OdometryFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:49
	// ("cv::Odometry::prepareFrame", vec![(pred!(const, ["frame"], ["cv::OdometryFrame*"]), _)]),
	void cv_Odometry_prepareFrame_const_OdometryFrameR(const cv::Odometry* instance, cv::OdometryFrame* frame, ResultVoid* ocvrs_return) {
		try {
			instance->prepareFrame(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrames(OdometryFrame &, OdometryFrame &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:55
	// ("cv::Odometry::prepareFrames", vec![(pred!(const, ["srcFrame", "dstFrame"], ["cv::OdometryFrame*", "cv::OdometryFrame*"]), _)]),
	void cv_Odometry_prepareFrames_const_OdometryFrameR_OdometryFrameR(const cv::Odometry* instance, cv::OdometryFrame* srcFrame, cv::OdometryFrame* dstFrame, ResultVoid* ocvrs_return) {
		try {
			instance->prepareFrames(*srcFrame, *dstFrame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(const OdometryFrame &, const OdometryFrame &, OutputArray)(TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:69
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["const cv::OdometryFrame*", "const cv::OdometryFrame*", "const cv::_OutputArray*"]), _)]),
	void cv_Odometry_compute_const_const_OdometryFrameR_const_OdometryFrameR_const__OutputArrayR(const cv::Odometry* instance, const cv::OdometryFrame* srcFrame, const cv::OdometryFrame* dstFrame, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:82
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "dstDepth", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::Odometry* instance, const cv::_InputArray* srcDepth, const cv::_InputArray* dstDepth, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcDepth, *dstDepth, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:97
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "srcRGB", "dstDepth", "dstRGB", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::Odometry* instance, const cv::_InputArray* srcDepth, const cv::_InputArray* srcRGB, const cv::_InputArray* dstDepth, const cv::_InputArray* dstRGB, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcDepth, *srcRGB, *dstDepth, *dstRGB, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:104
	// ("cv::Odometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	void cv_Odometry_getNormalsComputer_const(const cv::Odometry* instance, Result<cv::Ptr<cv::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::RgbdNormals> ret = instance->getNormalsComputer();
			Ok(new cv::Ptr<cv::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Odometry::delete() generated
	// ("cv::Odometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Odometry_delete(cv::Odometry* instance) {
			delete instance;
	}

	// OdometryFrame(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
	// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["depth", "image", "mask", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_OdometryFrame_OdometryFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_InputArray* normals, Result<cv::OdometryFrame*>* ocvrs_return) {
		try {
			cv::OdometryFrame* ret = new cv::OdometryFrame(*depth, *image, *mask, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::OdometryFrame::OdometryFrame() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
	// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
	void cv_OdometryFrame_OdometryFrame(Result<cv::OdometryFrame*>* ocvrs_return) {
		try {
			cv::OdometryFrame* ret = new cv::OdometryFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:57
	// ("cv::OdometryFrame::getImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getImage_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGrayImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:63
	// ("cv::OdometryFrame::getGrayImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getGrayImage_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getGrayImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:69
	// ("cv::OdometryFrame::getDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getDepth_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* depth, ResultVoid* ocvrs_return) {
		try {
			instance->getDepth(*depth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProcessedDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:75
	// ("cv::OdometryFrame::getProcessedDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getProcessedDepth_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* depth, ResultVoid* ocvrs_return) {
		try {
			instance->getProcessedDepth(*depth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMask(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:81
	// ("cv::OdometryFrame::getMask", vec![(pred!(const, ["mask"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getMask_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->getMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:87
	// ("cv::OdometryFrame::getNormals", vec![(pred!(const, ["normals"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometryFrame_getNormals_const_const__OutputArrayR(const cv::OdometryFrame* instance, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:93
	// ("cv::OdometryFrame::getPyramidLevels", vec![(pred!(const, [], []), _)]),
	void cv_OdometryFrame_getPyramidLevels_const(const cv::OdometryFrame* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPyramidLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyramidAt(OutputArray, OdometryFramePyramidType, size_t)(OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:102
	// ("cv::OdometryFrame::getPyramidAt", vec![(pred!(const, ["img", "pyrType", "level"], ["const cv::_OutputArray*", "cv::OdometryFramePyramidType", "size_t"]), _)]),
	void cv_OdometryFrame_getPyramidAt_const_const__OutputArrayR_OdometryFramePyramidType_size_t(const cv::OdometryFrame* instance, const cv::_OutputArray* img, cv::OdometryFramePyramidType pyrType, size_t level, ResultVoid* ocvrs_return) {
		try {
			instance->getPyramidAt(*img, pyrType, level);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::OdometryFrame::implicitClone() generated
	// ("cv::OdometryFrame::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::OdometryFrame* cv_OdometryFrame_implicitClone_const(const cv::OdometryFrame* instance) {
			return new cv::OdometryFrame(*instance);
	}

	// cv::OdometryFrame::delete() generated
	// ("cv::OdometryFrame::delete", vec![(pred!(mut, [], []), _)]),
	void cv_OdometryFrame_delete(cv::OdometryFrame* instance) {
			delete instance;
	}

	// OdometrySettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:16
	// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, [], []), _)]),
	void cv_OdometrySettings_OdometrySettings(Result<cv::OdometrySettings*>* ocvrs_return) {
		try {
			cv::OdometrySettings* ret = new cv::OdometrySettings();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OdometrySettings(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:17
	// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
	void cv_OdometrySettings_OdometrySettings_const_OdometrySettingsR(const cv::OdometrySettings* unnamed, Result<cv::OdometrySettings*>* ocvrs_return) {
		try {
			cv::OdometrySettings* ret = new cv::OdometrySettings(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:18
	// ("cv::OdometrySettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
	void cv_OdometrySettings_operatorST_const_OdometrySettingsR(cv::OdometrySettings* instance, const cv::OdometrySettings* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:20
	// ("cv::OdometrySettings::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_OdometrySettings_setCameraMatrix_const__InputArrayR(cv::OdometrySettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:21
	// ("cv::OdometrySettings::getCameraMatrix", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometrySettings_getCameraMatrix_const_const__OutputArrayR(const cv::OdometrySettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterCounts(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:22
	// ("cv::OdometrySettings::setIterCounts", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_OdometrySettings_setIterCounts_const__InputArrayR(cv::OdometrySettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterCounts(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:23
	// ("cv::OdometrySettings::getIterCounts", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometrySettings_getIterCounts_const_const__OutputArrayR(const cv::OdometrySettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getIterCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:25
	// ("cv::OdometrySettings::setMinDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMinDepth_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:26
	// ("cv::OdometrySettings::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMinDepth_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:27
	// ("cv::OdometrySettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMaxDepth_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:28
	// ("cv::OdometrySettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMaxDepth_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:29
	// ("cv::OdometrySettings::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMaxDepthDiff_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:30
	// ("cv::OdometrySettings::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMaxDepthDiff_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:31
	// ("cv::OdometrySettings::setMaxPointsPart", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMaxPointsPart_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:32
	// ("cv::OdometrySettings::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMaxPointsPart_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSobelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:34
	// ("cv::OdometrySettings::setSobelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_OdometrySettings_setSobelSize_int(cv::OdometrySettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setSobelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSobelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:35
	// ("cv::OdometrySettings::getSobelSize", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getSobelSize_const(const cv::OdometrySettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSobelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSobelScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:36
	// ("cv::OdometrySettings::setSobelScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_OdometrySettings_setSobelScale_double(cv::OdometrySettings* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSobelScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSobelScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:37
	// ("cv::OdometrySettings::getSobelScale", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getSobelScale_const(const cv::OdometrySettings* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSobelScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormalWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:39
	// ("cv::OdometrySettings::setNormalWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_OdometrySettings_setNormalWinSize_int(cv::OdometrySettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setNormalWinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalWinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:40
	// ("cv::OdometrySettings::getNormalWinSize", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getNormalWinSize_const(const cv::OdometrySettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNormalWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormalDiffThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:41
	// ("cv::OdometrySettings::setNormalDiffThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setNormalDiffThreshold_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setNormalDiffThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalDiffThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:42
	// ("cv::OdometrySettings::getNormalDiffThreshold", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getNormalDiffThreshold_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNormalDiffThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormalMethod(RgbdNormals::RgbdNormalsMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:43
	// ("cv::OdometrySettings::setNormalMethod", vec![(pred!(mut, ["nm"], ["cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
	void cv_OdometrySettings_setNormalMethod_RgbdNormalsMethod(cv::OdometrySettings* instance, cv::RgbdNormals::RgbdNormalsMethod nm, ResultVoid* ocvrs_return) {
		try {
			instance->setNormalMethod(nm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:44
	// ("cv::OdometrySettings::getNormalMethod", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getNormalMethod_const(const cv::OdometrySettings* instance, Result<cv::RgbdNormals::RgbdNormalsMethod>* ocvrs_return) {
		try {
			cv::RgbdNormals::RgbdNormalsMethod ret = instance->getNormalMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:46
	// ("cv::OdometrySettings::setAngleThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setAngleThreshold_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:47
	// ("cv::OdometrySettings::getAngleThreshold", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getAngleThreshold_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAngleThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:48
	// ("cv::OdometrySettings::setMaxTranslation", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMaxTranslation_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:49
	// ("cv::OdometrySettings::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMaxTranslation_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:50
	// ("cv::OdometrySettings::setMaxRotation", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMaxRotation_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:51
	// ("cv::OdometrySettings::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMaxRotation_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitude(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:53
	// ("cv::OdometrySettings::setMinGradientMagnitude", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_OdometrySettings_setMinGradientMagnitude_float(cv::OdometrySettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitude(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitude()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:54
	// ("cv::OdometrySettings::getMinGradientMagnitude", vec![(pred!(const, [], []), _)]),
	void cv_OdometrySettings_getMinGradientMagnitude_const(const cv::OdometrySettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinGradientMagnitude();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitudes(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:55
	// ("cv::OdometrySettings::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_OdometrySettings_setMinGradientMagnitudes_const__InputArrayR(cv::OdometrySettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitudes(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:56
	// ("cv::OdometrySettings::getMinGradientMagnitudes", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_OdometrySettings_getMinGradientMagnitudes_const_const__OutputArrayR(const cv::OdometrySettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::OdometrySettings::implicitClone() generated
	// ("cv::OdometrySettings::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::OdometrySettings* cv_OdometrySettings_implicitClone_const(const cv::OdometrySettings* instance) {
			return new cv::OdometrySettings(*instance);
	}

	// cv::OdometrySettings::delete() generated
	// ("cv::OdometrySettings::delete", vec![(pred!(mut, [], []), _)]),
	void cv_OdometrySettings_delete(cv::OdometrySettings* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:303
	// ("cv::RegionGrowing3D::create", vec![(pred!(mut, [], []), _)]),
	void cv_RegionGrowing3D_create(Result<cv::Ptr<cv::RegionGrowing3D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::RegionGrowing3D> ret = cv::RegionGrowing3D::create();
			Ok(new cv::Ptr<cv::RegionGrowing3D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// segment(OutputArrayOfArrays, OutputArray, InputArray, InputArray, InputArrayOfArrays)(OutputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:332
	// ("cv::RegionGrowing3D::segment", vec![(pred!(mut, ["regions_idx", "labels", "input_pts", "normals", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_RegionGrowing3D_segment_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::RegionGrowing3D* instance, const cv::_OutputArray* regions_idx, const cv::_OutputArray* labels, const cv::_InputArray* input_pts, const cv::_InputArray* normals, const cv::_InputArray* nn_idx, Result<int>* ocvrs_return) {
		try {
			int ret = instance->segment(*regions_idx, *labels, *input_pts, *normals, *nn_idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:339
	// ("cv::RegionGrowing3D::setMinSize", vec![(pred!(mut, ["min_size"], ["int"]), _)]),
	void cv_RegionGrowing3D_setMinSize_int(cv::RegionGrowing3D* instance, int min_size, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSize(min_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:342
	// ("cv::RegionGrowing3D::getMinSize", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getMinSize_const(const cv::RegionGrowing3D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:346
	// ("cv::RegionGrowing3D::setMaxSize", vec![(pred!(mut, ["max_size"], ["int"]), _)]),
	void cv_RegionGrowing3D_setMaxSize_int(cv::RegionGrowing3D* instance, int max_size, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxSize(max_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:349
	// ("cv::RegionGrowing3D::getMaxSize", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getMaxSize_const(const cv::RegionGrowing3D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmoothModeFlag(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:354
	// ("cv::RegionGrowing3D::setSmoothModeFlag", vec![(pred!(mut, ["smooth_mode"], ["bool"]), _)]),
	void cv_RegionGrowing3D_setSmoothModeFlag_bool(cv::RegionGrowing3D* instance, bool smooth_mode, ResultVoid* ocvrs_return) {
		try {
			instance->setSmoothModeFlag(smooth_mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmoothModeFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:357
	// ("cv::RegionGrowing3D::getSmoothModeFlag", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getSmoothModeFlag_const(const cv::RegionGrowing3D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getSmoothModeFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmoothnessThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:361
	// ("cv::RegionGrowing3D::setSmoothnessThreshold", vec![(pred!(mut, ["smoothness_thr"], ["double"]), _)]),
	void cv_RegionGrowing3D_setSmoothnessThreshold_double(cv::RegionGrowing3D* instance, double smoothness_thr, ResultVoid* ocvrs_return) {
		try {
			instance->setSmoothnessThreshold(smoothness_thr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmoothnessThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:364
	// ("cv::RegionGrowing3D::getSmoothnessThreshold", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getSmoothnessThreshold_const(const cv::RegionGrowing3D* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSmoothnessThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCurvatureThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:369
	// ("cv::RegionGrowing3D::setCurvatureThreshold", vec![(pred!(mut, ["curvature_thr"], ["double"]), _)]),
	void cv_RegionGrowing3D_setCurvatureThreshold_double(cv::RegionGrowing3D* instance, double curvature_thr, ResultVoid* ocvrs_return) {
		try {
			instance->setCurvatureThreshold(curvature_thr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCurvatureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:372
	// ("cv::RegionGrowing3D::getCurvatureThreshold", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getCurvatureThreshold_const(const cv::RegionGrowing3D* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getCurvatureThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxNumberOfNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:376
	// ("cv::RegionGrowing3D::setMaxNumberOfNeighbors", vec![(pred!(mut, ["max_neighbor_num"], ["int"]), _)]),
	void cv_RegionGrowing3D_setMaxNumberOfNeighbors_int(cv::RegionGrowing3D* instance, int max_neighbor_num, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxNumberOfNeighbors(max_neighbor_num);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxNumberOfNeighbors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:379
	// ("cv::RegionGrowing3D::getMaxNumberOfNeighbors", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getMaxNumberOfNeighbors_const(const cv::RegionGrowing3D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxNumberOfNeighbors();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumberOfRegions(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:383
	// ("cv::RegionGrowing3D::setNumberOfRegions", vec![(pred!(mut, ["region_num"], ["int"]), _)]),
	void cv_RegionGrowing3D_setNumberOfRegions_int(cv::RegionGrowing3D* instance, int region_num, ResultVoid* ocvrs_return) {
		try {
			instance->setNumberOfRegions(region_num);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumberOfRegions()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:386
	// ("cv::RegionGrowing3D::getNumberOfRegions", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getNumberOfRegions_const(const cv::RegionGrowing3D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfRegions();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNeedSort(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:389
	// ("cv::RegionGrowing3D::setNeedSort", vec![(pred!(mut, ["need_sort"], ["bool"]), _)]),
	void cv_RegionGrowing3D_setNeedSort_bool(cv::RegionGrowing3D* instance, bool need_sort, ResultVoid* ocvrs_return) {
		try {
			instance->setNeedSort(need_sort);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNeedSort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:392
	// ("cv::RegionGrowing3D::getNeedSort", vec![(pred!(const, [], []), _)]),
	void cv_RegionGrowing3D_getNeedSort_const(const cv::RegionGrowing3D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNeedSort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSeeds(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:398
	// ("cv::RegionGrowing3D::setSeeds", vec![(pred!(mut, ["seeds"], ["const cv::_InputArray*"]), _)]),
	void cv_RegionGrowing3D_setSeeds_const__InputArrayR(cv::RegionGrowing3D* instance, const cv::_InputArray* seeds, ResultVoid* ocvrs_return) {
		try {
			instance->setSeeds(*seeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSeeds(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:401
	// ("cv::RegionGrowing3D::getSeeds", vec![(pred!(const, ["seeds"], ["const cv::_OutputArray*"]), _)]),
	void cv_RegionGrowing3D_getSeeds_const_const__OutputArrayR(const cv::RegionGrowing3D* instance, const cv::_OutputArray* seeds, ResultVoid* ocvrs_return) {
		try {
			instance->getSeeds(*seeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCurvatures(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:404
	// ("cv::RegionGrowing3D::setCurvatures", vec![(pred!(mut, ["curvatures"], ["const cv::_InputArray*"]), _)]),
	void cv_RegionGrowing3D_setCurvatures_const__InputArrayR(cv::RegionGrowing3D* instance, const cv::_InputArray* curvatures, ResultVoid* ocvrs_return) {
		try {
			instance->setCurvatures(*curvatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCurvatures(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:407
	// ("cv::RegionGrowing3D::getCurvatures", vec![(pred!(const, ["curvatures"], ["const cv::_OutputArray*"]), _)]),
	void cv_RegionGrowing3D_getCurvatures_const_const__OutputArrayR(const cv::RegionGrowing3D* instance, const cv::_OutputArray* curvatures, ResultVoid* ocvrs_return) {
		try {
			instance->getCurvatures(*curvatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RegionGrowing3D::delete() generated
	// ("cv::RegionGrowing3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RegionGrowing3D_delete(cv::RegionGrowing3D* instance) {
			delete instance;
	}

	// create(int, int, int, InputArray, int, float, RgbdNormals::RgbdNormalsMethod)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
	// ("cv::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "diff_threshold", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "float", "cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
	void cv_RgbdNormals_create_int_int_int_const__InputArrayR_int_float_RgbdNormalsMethod(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, float diff_threshold, cv::RgbdNormals::RgbdNormalsMethod method, Result<cv::Ptr<cv::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::RgbdNormals> ret = cv::RgbdNormals::create(rows, cols, depth, *K, window_size, diff_threshold, method);
			Ok(new cv::Ptr<cv::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RgbdNormals::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
	// ("cv::RgbdNormals::create", vec![(pred!(mut, [], []), _)]),
	void cv_RgbdNormals_create(Result<cv::Ptr<cv::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::RgbdNormals> ret = cv::RgbdNormals::create();
			Ok(new cv::Ptr<cv::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:57
	// ("cv::RgbdNormals::apply", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_RgbdNormals_apply_const_const__InputArrayR_const__OutputArrayR(const cv::RgbdNormals* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cache()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:62
	// ("cv::RgbdNormals::cache", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_cache_const(const cv::RgbdNormals* instance, ResultVoid* ocvrs_return) {
		try {
			instance->cache();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:64
	// ("cv::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_getRows_const(const cv::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:65
	// ("cv::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_RgbdNormals_setRows_int(cv::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRows(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:66
	// ("cv::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_getCols_const(const cv::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:67
	// ("cv::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_RgbdNormals_setCols_int(cv::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCols(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:68
	// ("cv::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_getWindowSize_const(const cv::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:69
	// ("cv::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_RgbdNormals_setWindowSize_int(cv::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:70
	// ("cv::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_getDepth_const(const cv::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:71
	// ("cv::RgbdNormals::getK", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_RgbdNormals_getK_const_const__OutputArrayR(const cv::RgbdNormals* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getK(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:72
	// ("cv::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_RgbdNormals_setK_const__InputArrayR(cv::RgbdNormals* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setK(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:73
	// ("cv::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_RgbdNormals_getMethod_const(const cv::RgbdNormals* instance, Result<cv::RgbdNormals::RgbdNormalsMethod>* ocvrs_return) {
		try {
			cv::RgbdNormals::RgbdNormalsMethod ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RgbdNormals::delete() generated
	// ("cv::RgbdNormals::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RgbdNormals_delete(cv::RgbdNormals* instance) {
			delete instance;
	}

	// create(SacModelType, SacMethod, double, int)(Enum, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
	// ("cv::SACSegmentation::create", vec![(pred!(mut, ["sac_model_type", "sac_method", "threshold", "max_iterations"], ["cv::SacModelType", "cv::SacMethod", "double", "int"]), _)]),
	void cv_SACSegmentation_create_SacModelType_SacMethod_double_int(cv::SacModelType sac_model_type, cv::SacMethod sac_method, double threshold, int max_iterations, Result<cv::Ptr<cv::SACSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SACSegmentation> ret = cv::SACSegmentation::create(sac_model_type, sac_method, threshold, max_iterations);
			Ok(new cv::Ptr<cv::SACSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SACSegmentation::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
	// ("cv::SACSegmentation::create", vec![(pred!(mut, [], []), _)]),
	void cv_SACSegmentation_create(Result<cv::Ptr<cv::SACSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SACSegmentation> ret = cv::SACSegmentation::create();
			Ok(new cv::Ptr<cv::SACSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// segment(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:98
	// ("cv::SACSegmentation::segment", vec![(pred!(mut, ["input_pts", "labels", "models_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SACSegmentation_segment_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::SACSegmentation* instance, const cv::_InputArray* input_pts, const cv::_OutputArray* labels, const cv::_OutputArray* models_coefficients, Result<int>* ocvrs_return) {
		try {
			int ret = instance->segment(*input_pts, *labels, *models_coefficients);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSacModelType(SacModelType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:103
	// ("cv::SACSegmentation::setSacModelType", vec![(pred!(mut, ["sac_model_type"], ["cv::SacModelType"]), _)]),
	void cv_SACSegmentation_setSacModelType_SacModelType(cv::SACSegmentation* instance, cv::SacModelType sac_model_type, ResultVoid* ocvrs_return) {
		try {
			instance->setSacModelType(sac_model_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSacModelType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:106
	// ("cv::SACSegmentation::getSacModelType", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getSacModelType_const(const cv::SACSegmentation* instance, Result<cv::SacModelType>* ocvrs_return) {
		try {
			cv::SacModelType ret = instance->getSacModelType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSacMethodType(SacMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:109
	// ("cv::SACSegmentation::setSacMethodType", vec![(pred!(mut, ["sac_method"], ["cv::SacMethod"]), _)]),
	void cv_SACSegmentation_setSacMethodType_SacMethod(cv::SACSegmentation* instance, cv::SacMethod sac_method, ResultVoid* ocvrs_return) {
		try {
			instance->setSacMethodType(sac_method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSacMethodType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:112
	// ("cv::SACSegmentation::getSacMethodType", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getSacMethodType_const(const cv::SACSegmentation* instance, Result<cv::SacMethod>* ocvrs_return) {
		try {
			cv::SacMethod ret = instance->getSacMethodType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDistanceThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:116
	// ("cv::SACSegmentation::setDistanceThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_SACSegmentation_setDistanceThreshold_double(cv::SACSegmentation* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setDistanceThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:119
	// ("cv::SACSegmentation::getDistanceThreshold", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getDistanceThreshold_const(const cv::SACSegmentation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDistanceThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadiusLimits(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:123
	// ("cv::SACSegmentation::setRadiusLimits", vec![(pred!(mut, ["radius_min", "radius_max"], ["double", "double"]), _)]),
	void cv_SACSegmentation_setRadiusLimits_double_double(cv::SACSegmentation* instance, double radius_min, double radius_max, ResultVoid* ocvrs_return) {
		try {
			instance->setRadiusLimits(radius_min, radius_max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadiusLimits(double &, double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:126
	// ("cv::SACSegmentation::getRadiusLimits", vec![(pred!(const, ["radius_min", "radius_max"], ["double*", "double*"]), _)]),
	void cv_SACSegmentation_getRadiusLimits_const_doubleR_doubleR(const cv::SACSegmentation* instance, double* radius_min, double* radius_max, ResultVoid* ocvrs_return) {
		try {
			instance->getRadiusLimits(*radius_min, *radius_max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:129
	// ("cv::SACSegmentation::setMaxIterations", vec![(pred!(mut, ["max_iterations"], ["int"]), _)]),
	void cv_SACSegmentation_setMaxIterations_int(cv::SACSegmentation* instance, int max_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxIterations(max_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:132
	// ("cv::SACSegmentation::getMaxIterations", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getMaxIterations_const(const cv::SACSegmentation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setConfidence(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:135
	// ("cv::SACSegmentation::setConfidence", vec![(pred!(mut, ["confidence"], ["double"]), _)]),
	void cv_SACSegmentation_setConfidence_double(cv::SACSegmentation* instance, double confidence, ResultVoid* ocvrs_return) {
		try {
			instance->setConfidence(confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getConfidence()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:138
	// ("cv::SACSegmentation::getConfidence", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getConfidence_const(const cv::SACSegmentation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getConfidence();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumberOfModelsExpected(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:141
	// ("cv::SACSegmentation::setNumberOfModelsExpected", vec![(pred!(mut, ["number_of_models_expected"], ["int"]), _)]),
	void cv_SACSegmentation_setNumberOfModelsExpected_int(cv::SACSegmentation* instance, int number_of_models_expected, ResultVoid* ocvrs_return) {
		try {
			instance->setNumberOfModelsExpected(number_of_models_expected);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumberOfModelsExpected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:144
	// ("cv::SACSegmentation::getNumberOfModelsExpected", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getNumberOfModelsExpected_const(const cv::SACSegmentation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfModelsExpected();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParallel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:148
	// ("cv::SACSegmentation::setParallel", vec![(pred!(mut, ["is_parallel"], ["bool"]), _)]),
	void cv_SACSegmentation_setParallel_bool(cv::SACSegmentation* instance, bool is_parallel, ResultVoid* ocvrs_return) {
		try {
			instance->setParallel(is_parallel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isParallel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:151
	// ("cv::SACSegmentation::isParallel", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_isParallel_const(const cv::SACSegmentation* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isParallel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRandomGeneratorState(uint64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:154
	// ("cv::SACSegmentation::setRandomGeneratorState", vec![(pred!(mut, ["rng_state"], ["uint64_t"]), _)]),
	void cv_SACSegmentation_setRandomGeneratorState_uint64_t(cv::SACSegmentation* instance, uint64_t rng_state, ResultVoid* ocvrs_return) {
		try {
			instance->setRandomGeneratorState(rng_state);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRandomGeneratorState()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:157
	// ("cv::SACSegmentation::getRandomGeneratorState", vec![(pred!(const, [], []), _)]),
	void cv_SACSegmentation_getRandomGeneratorState_const(const cv::SACSegmentation* instance, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->getRandomGeneratorState();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SACSegmentation::delete() generated
	// ("cv::SACSegmentation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SACSegmentation_delete(cv::SACSegmentation* instance) {
			delete instance;
	}

	// TriangleRasterizeSettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2945
	// ("cv::TriangleRasterizeSettings::TriangleRasterizeSettings", vec![(pred!(mut, [], []), _)]),
	void cv_TriangleRasterizeSettings_TriangleRasterizeSettings(Result<cv::TriangleRasterizeSettings>* ocvrs_return) {
		try {
			cv::TriangleRasterizeSettings ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadingType(TriangleShadingType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2947
	// ("cv::TriangleRasterizeSettings::setShadingType", vec![(pred!(mut, ["st"], ["cv::TriangleShadingType"]), _)]),
	void cv_TriangleRasterizeSettings_setShadingType_TriangleShadingType(cv::TriangleRasterizeSettings* instance, cv::TriangleShadingType st, Result<cv::TriangleRasterizeSettings>* ocvrs_return) {
		try {
			cv::TriangleRasterizeSettings ret = instance->setShadingType(st);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCullingMode(TriangleCullingMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2948
	// ("cv::TriangleRasterizeSettings::setCullingMode", vec![(pred!(mut, ["cm"], ["cv::TriangleCullingMode"]), _)]),
	void cv_TriangleRasterizeSettings_setCullingMode_TriangleCullingMode(cv::TriangleRasterizeSettings* instance, cv::TriangleCullingMode cm, Result<cv::TriangleRasterizeSettings>* ocvrs_return) {
		try {
			cv::TriangleRasterizeSettings ret = instance->setCullingMode(cm);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGlCompatibleMode(TriangleGlCompatibleMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2949
	// ("cv::TriangleRasterizeSettings::setGlCompatibleMode", vec![(pred!(mut, ["gm"], ["cv::TriangleGlCompatibleMode"]), _)]),
	void cv_TriangleRasterizeSettings_setGlCompatibleMode_TriangleGlCompatibleMode(cv::TriangleRasterizeSettings* instance, cv::TriangleGlCompatibleMode gm, Result<cv::TriangleRasterizeSettings>* ocvrs_return) {
		try {
			cv::TriangleRasterizeSettings ret = instance->setGlCompatibleMode(gm);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UsacParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:432
	// ("cv::UsacParams::UsacParams", vec![(pred!(mut, [], []), _)]),
	void cv_UsacParams_UsacParams(Result<cv::UsacParams>* ocvrs_return) {
		try {
			cv::UsacParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Volume(VolumeType, const VolumeSettings &)(Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
	// ("cv::Volume::Volume", vec![(pred!(mut, ["vtype", "settings"], ["cv::VolumeType", "const cv::VolumeSettings*"]), _)]),
	void cv_Volume_Volume_VolumeType_const_VolumeSettingsR(cv::VolumeType vtype, const cv::VolumeSettings* settings, Result<cv::Volume*>* ocvrs_return) {
		try {
			cv::Volume* ret = new cv::Volume(vtype, *settings);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Volume::Volume() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
	// ("cv::Volume::Volume", vec![(pred!(mut, [], []), _)]),
	void cv_Volume_Volume(Result<cv::Volume*>* ocvrs_return) {
		try {
			cv::Volume* ret = new cv::Volume();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integrate(const OdometryFrame &, InputArray)(TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:35
	// ("cv::Volume::integrate", vec![(pred!(mut, ["frame", "pose"], ["const cv::OdometryFrame*", "const cv::_InputArray*"]), _)]),
	void cv_Volume_integrate_const_OdometryFrameR_const__InputArrayR(cv::Volume* instance, const cv::OdometryFrame* frame, const cv::_InputArray* pose, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*frame, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integrate(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:44
	// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Volume_integrate_const__InputArrayR_const__InputArrayR(cv::Volume* instance, const cv::_InputArray* depth, const cv::_InputArray* pose, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*depth, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integrate(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:56
	// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "image", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Volume_integrate_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::Volume* instance, const cv::_InputArray* depth, const cv::_InputArray* image, const cv::_InputArray* pose, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*depth, *image, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:68
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_InputArray* cameraPose, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:79
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals", "colors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_InputArray* cameraPose, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:92
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_InputArray* cameraPose, int height, int width, const cv::_InputArray* K, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, height, width, *K, *points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:107
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals", "colors"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_InputArray* cameraPose, int height, int width, const cv::_InputArray* K, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, height, width, *K, *points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:113
	// ("cv::Volume::fetchNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->fetchNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:118
	// ("cv::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->fetchPointsNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:124
	// ("cv::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Volume* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->fetchPointsNormalsColors(*points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:128
	// ("cv::Volume::reset", vec![(pred!(mut, [], []), _)]),
	void cv_Volume_reset(cv::Volume* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVisibleBlocks()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:133
	// ("cv::Volume::getVisibleBlocks", vec![(pred!(const, [], []), _)]),
	void cv_Volume_getVisibleBlocks_const(const cv::Volume* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVisibleBlocks();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTotalVolumeUnits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:137
	// ("cv::Volume::getTotalVolumeUnits", vec![(pred!(const, [], []), _)]),
	void cv_Volume_getTotalVolumeUnits_const(const cv::Volume* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getTotalVolumeUnits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBoundingBox(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:152
	// ("cv::Volume::getBoundingBox", vec![(pred!(const, ["bb", "precision"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_Volume_getBoundingBox_const_const__OutputArrayR_int(const cv::Volume* instance, const cv::_OutputArray* bb, int precision, ResultVoid* ocvrs_return) {
		try {
			instance->getBoundingBox(*bb, precision);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEnableGrowth(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:158
	// ("cv::Volume::setEnableGrowth", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	void cv_Volume_setEnableGrowth_bool(cv::Volume* instance, bool v, ResultVoid* ocvrs_return) {
		try {
			instance->setEnableGrowth(v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEnableGrowth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:163
	// ("cv::Volume::getEnableGrowth", vec![(pred!(const, [], []), _)]),
	void cv_Volume_getEnableGrowth_const(const cv::Volume* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getEnableGrowth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Volume::delete() generated
	// ("cv::Volume::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Volume_delete(cv::Volume* instance) {
			delete instance;
	}

	// VolumeSettings(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	void cv_VolumeSettings_VolumeSettings_VolumeType(cv::VolumeType volumeType, Result<cv::VolumeSettings*>* ocvrs_return) {
		try {
			cv::VolumeSettings* ret = new cv::VolumeSettings(volumeType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VolumeSettings::VolumeSettings() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, [], []), _)]),
	void cv_VolumeSettings_VolumeSettings(Result<cv::VolumeSettings*>* ocvrs_return) {
		try {
			cv::VolumeSettings* ret = new cv::VolumeSettings();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VolumeSettings(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:31
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["vs"], ["const cv::VolumeSettings*"]), _)]),
	void cv_VolumeSettings_VolumeSettings_const_VolumeSettingsR(const cv::VolumeSettings* vs, Result<cv::VolumeSettings*>* ocvrs_return) {
		try {
			cv::VolumeSettings* ret = new cv::VolumeSettings(*vs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:32
	// ("cv::VolumeSettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::VolumeSettings*"]), _)]),
	void cv_VolumeSettings_operatorST_const_VolumeSettingsR(cv::VolumeSettings* instance, const cv::VolumeSettings* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIntegrateWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:38
	// ("cv::VolumeSettings::setIntegrateWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VolumeSettings_setIntegrateWidth_int(cv::VolumeSettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIntegrateWidth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntegrateWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:42
	// ("cv::VolumeSettings::getIntegrateWidth", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getIntegrateWidth_const(const cv::VolumeSettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntegrateWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIntegrateHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:47
	// ("cv::VolumeSettings::setIntegrateHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VolumeSettings_setIntegrateHeight_int(cv::VolumeSettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIntegrateHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntegrateHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:51
	// ("cv::VolumeSettings::getIntegrateHeight", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getIntegrateHeight_const(const cv::VolumeSettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntegrateHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRaycastWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:56
	// ("cv::VolumeSettings::setRaycastWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VolumeSettings_setRaycastWidth_int(cv::VolumeSettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRaycastWidth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRaycastWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:60
	// ("cv::VolumeSettings::getRaycastWidth", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getRaycastWidth_const(const cv::VolumeSettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRaycastWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRaycastHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:65
	// ("cv::VolumeSettings::setRaycastHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VolumeSettings_setRaycastHeight_int(cv::VolumeSettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRaycastHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRaycastHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:69
	// ("cv::VolumeSettings::getRaycastHeight", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getRaycastHeight_const(const cv::VolumeSettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRaycastHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepthFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:74
	// ("cv::VolumeSettings::setDepthFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VolumeSettings_setDepthFactor_float(cv::VolumeSettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setDepthFactor(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepthFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:78
	// ("cv::VolumeSettings::getDepthFactor", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getDepthFactor_const(const cv::VolumeSettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDepthFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVoxelSize(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:83
	// ("cv::VolumeSettings::setVoxelSize", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VolumeSettings_setVoxelSize_float(cv::VolumeSettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVoxelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVoxelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:87
	// ("cv::VolumeSettings::getVoxelSize", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getVoxelSize_const(const cv::VolumeSettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVoxelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTsdfTruncateDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:92
	// ("cv::VolumeSettings::setTsdfTruncateDistance", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VolumeSettings_setTsdfTruncateDistance_float(cv::VolumeSettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setTsdfTruncateDistance(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTsdfTruncateDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:96
	// ("cv::VolumeSettings::getTsdfTruncateDistance", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getTsdfTruncateDistance_const(const cv::VolumeSettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTsdfTruncateDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:101
	// ("cv::VolumeSettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VolumeSettings_setMaxDepth_float(cv::VolumeSettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:105
	// ("cv::VolumeSettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getMaxDepth_const(const cv::VolumeSettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxWeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:111
	// ("cv::VolumeSettings::setMaxWeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VolumeSettings_setMaxWeight_int(cv::VolumeSettings* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxWeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxWeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:116
	// ("cv::VolumeSettings::getMaxWeight", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getMaxWeight_const(const cv::VolumeSettings* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRaycastStepFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:122
	// ("cv::VolumeSettings::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VolumeSettings_setRaycastStepFactor_float(cv::VolumeSettings* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setRaycastStepFactor(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRaycastStepFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:127
	// ("cv::VolumeSettings::getRaycastStepFactor", vec![(pred!(const, [], []), _)]),
	void cv_VolumeSettings_getRaycastStepFactor_const(const cv::VolumeSettings* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRaycastStepFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVolumePose(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:132
	// ("cv::VolumeSettings::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_VolumeSettings_setVolumePose_const__InputArrayR(cv::VolumeSettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setVolumePose(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVolumePose(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:137
	// ("cv::VolumeSettings::getVolumePose", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_VolumeSettings_getVolumePose_const_const__OutputArrayR(const cv::VolumeSettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getVolumePose(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVolumeResolution(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:145
	// ("cv::VolumeSettings::setVolumeResolution", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_VolumeSettings_setVolumeResolution_const__InputArrayR(cv::VolumeSettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setVolumeResolution(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVolumeResolution(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:153
	// ("cv::VolumeSettings::getVolumeResolution", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_VolumeSettings_getVolumeResolution_const_const__OutputArrayR(const cv::VolumeSettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getVolumeResolution(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVolumeStrides(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:159
	// ("cv::VolumeSettings::getVolumeStrides", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_VolumeSettings_getVolumeStrides_const_const__OutputArrayR(const cv::VolumeSettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getVolumeStrides(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraIntegrateIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:169
	// ("cv::VolumeSettings::setCameraIntegrateIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_VolumeSettings_setCameraIntegrateIntrinsics_const__InputArrayR(cv::VolumeSettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntegrateIntrinsics(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraIntegrateIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:179
	// ("cv::VolumeSettings::getCameraIntegrateIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_VolumeSettings_getCameraIntegrateIntrinsics_const_const__OutputArrayR(const cv::VolumeSettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getCameraIntegrateIntrinsics(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraRaycastIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:189
	// ("cv::VolumeSettings::setCameraRaycastIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	void cv_VolumeSettings_setCameraRaycastIntrinsics_const__InputArrayR(cv::VolumeSettings* instance, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraRaycastIntrinsics(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraRaycastIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:199
	// ("cv::VolumeSettings::getCameraRaycastIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	void cv_VolumeSettings_getCameraRaycastIntrinsics_const_const__OutputArrayR(const cv::VolumeSettings* instance, const cv::_OutputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->getCameraRaycastIntrinsics(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VolumeSettings::implicitClone() generated
	// ("cv::VolumeSettings::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::VolumeSettings* cv_VolumeSettings_implicitClone_const(const cv::VolumeSettings* instance) {
			return new cv::VolumeSettings(*instance);
	}

	// cv::VolumeSettings::delete() generated
	// ("cv::VolumeSettings::delete", vec![(pred!(mut, [], []), _)]),
	void cv_VolumeSettings_delete(cv::VolumeSettings* instance) {
			delete instance;
	}

}
