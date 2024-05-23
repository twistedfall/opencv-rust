#include "sfm.hpp"
#include "sfm_types.hpp"

extern "C" {
	// KRtFromProjection(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/projection.hpp:88
	// ("cv::sfm::KRtFromProjection", vec![(pred!(mut, ["P", "K", "R", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_KRtFromProjection_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* P, const cv::_OutputArray* K, const cv::_OutputArray* R, const cv::_OutputArray* t, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::KRtFromProjection(*P, *K, *R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyTransformationToPoints(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/conditioning.hpp:82
	// ("cv::sfm::applyTransformationToPoints", vec![(pred!(mut, ["points", "T", "transformed_points"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_applyTransformationToPoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_InputArray* T, const cv::_OutputArray* transformed_points, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::applyTransformationToPoints(*points, *T, *transformed_points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeOrientation(InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:212
	// ("cv::sfm::computeOrientation", vec![(pred!(mut, ["x1", "x2", "R", "t", "s"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_sfm_computeOrientation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* x1, const cv::_InputArray* x2, const cv::_OutputArray* R, const cv::_OutputArray* t, double s, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::computeOrientation(*x1, *x2, *R, *t, s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/projection.hpp:97
	// ("cv::sfm::depth", vec![(pred!(mut, ["R", "t", "X"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_sfm_depth_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* R, const cv::_InputArray* t, const cv::_InputArray* X, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sfm::depth(*R, *t, *X);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// essentialFromFundamental(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:167
	// ("cv::sfm::essentialFromFundamental", vec![(pred!(mut, ["F", "K1", "K2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_essentialFromFundamental_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_InputArray* K1, const cv::_InputArray* K2, const cv::_OutputArray* E, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::essentialFromFundamental(*F, *K1, *K2, *E);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// essentialFromRt(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:183
	// ("cv::sfm::essentialFromRt", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_essentialFromRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* R1, const cv::_InputArray* t1, const cv::_InputArray* R2, const cv::_InputArray* t2, const cv::_OutputArray* E, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::essentialFromRt(*R1, *t1, *R2, *t2, *E);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// euclideanToHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/projection.hpp:63
	// ("cv::sfm::euclideanToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_euclideanToHomogeneous_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::euclideanToHomogeneous(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/robust.hpp:90
	// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* x1, const cv::_InputArray* x2, double max_error, const cv::_OutputArray* F, const cv::_OutputArray* inliers, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences7PointRobust(*x1, *x2, max_error, *F, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/robust.hpp:90
	// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* x1, const cv::_InputArray* x2, double max_error, const cv::_OutputArray* F, const cv::_OutputArray* inliers, double outliers_probability, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences7PointRobust(*x1, *x2, max_error, *F, *inliers, outliers_probability);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/robust.hpp:67
	// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* x1, const cv::_InputArray* x2, double max_error, const cv::_OutputArray* F, const cv::_OutputArray* inliers, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences8PointRobust(*x1, *x2, max_error, *F, *inliers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/robust.hpp:67
	// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* x1, const cv::_InputArray* x2, double max_error, const cv::_OutputArray* F, const cv::_OutputArray* inliers, double outliers_probability, Result<double>* ocvrs_return) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences8PointRobust(*x1, *x2, max_error, *F, *inliers, outliers_probability);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fundamentalFromEssential(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:152
	// ("cv::sfm::fundamentalFromEssential", vec![(pred!(mut, ["E", "K1", "K2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_fundamentalFromEssential_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_InputArray* K1, const cv::_InputArray* K2, const cv::_OutputArray* F, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::fundamentalFromEssential(*E, *K1, *K2, *F);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fundamentalFromProjections(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:69
	// ("cv::sfm::fundamentalFromProjections", vec![(pred!(mut, ["P1", "P2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_fundamentalFromProjections_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* P1, const cv::_InputArray* P2, const cv::_OutputArray* F, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::fundamentalFromProjections(*P1, *P2, *F);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// homogeneousToEuclidean(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/projection.hpp:55
	// ("cv::sfm::homogeneousToEuclidean", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_homogeneousToEuclidean_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::homogeneousToEuclidean(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::importReconstruction(InString, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/io.hpp:77
	// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const char* file, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* Ks, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::importReconstruction(cv::String(file), *Rs, *Ts, *Ks, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// importReconstruction(const cv::String &, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, int)(InString, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/io.hpp:77
	// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d", "file_format"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const char* file, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* Ks, const cv::_OutputArray* points3d, int file_format, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::importReconstruction(cv::String(file), *Rs, *Ts, *Ks, *points3d, file_format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isotropicPreconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/conditioning.hpp:72
	// ("cv::sfm::isotropicPreconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_isotropicPreconditionerFromPoints_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* T, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::isotropicPreconditionerFromPoints(*points, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanAndVarianceAlongRows(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/numeric.hpp:58
	// ("cv::sfm::meanAndVarianceAlongRows", vec![(pred!(mut, ["A", "mean", "variance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_meanAndVarianceAlongRows_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* A, const cv::_OutputArray* mean, const cv::_OutputArray* variance, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::meanAndVarianceAlongRows(*A, *mean, *variance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionFromEssentialChooseSolution(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:135
	// ("cv::sfm::motionFromEssentialChooseSolution", vec![(pred!(mut, ["Rs", "ts", "K1", "x1", "K2", "x2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_sfm_motionFromEssentialChooseSolution_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* Rs, const cv::_InputArray* ts, const cv::_InputArray* K1, const cv::_InputArray* x1, const cv::_InputArray* K2, const cv::_InputArray* x2, Result<int>* ocvrs_return) {
		try {
			int ret = cv::sfm::motionFromEssentialChooseSolution(*Rs, *ts, *K1, *x1, *K2, *x2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionFromEssential(InputArray, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:117
	// ("cv::sfm::motionFromEssential", vec![(pred!(mut, ["E", "Rs", "ts"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_motionFromEssential_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* E, const cv::_OutputArray* Rs, const cv::_OutputArray* ts, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::motionFromEssential(*E, *Rs, *ts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalizeFundamental(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:197
	// ("cv::sfm::normalizeFundamental", vec![(pred!(mut, ["F", "F_normalized"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_normalizeFundamental_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_OutputArray* F_normalized, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::normalizeFundamental(*F, *F_normalized);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalizeIsotropicPoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/conditioning.hpp:112
	// ("cv::sfm::normalizeIsotropicPoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_normalizeIsotropicPoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* normalized_points, const cv::_OutputArray* T, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::normalizeIsotropicPoints(*points, *normalized_points, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalizePoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/conditioning.hpp:97
	// ("cv::sfm::normalizePoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_normalizePoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* normalized_points, const cv::_OutputArray* T, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::normalizePoints(*points, *normalized_points, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalizedEightPointSolver(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:83
	// ("cv::sfm::normalizedEightPointSolver", vec![(pred!(mut, ["x1", "x2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_normalizedEightPointSolver_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x1, const cv::_InputArray* x2, const cv::_OutputArray* F, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::normalizedEightPointSolver(*x1, *x2, *F);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/conditioning.hpp:59
	// ("cv::sfm::preconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_preconditionerFromPoints_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* T, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::preconditionerFromPoints(*points, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectionFromKRt(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/projection.hpp:76
	// ("cv::sfm::projectionFromKRt", vec![(pred!(mut, ["K", "R", "t", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_projectionFromKRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* t, const cv::_OutputArray* P, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::projectionFromKRt(*K, *R, *t, *P);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectionsFromFundamental(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:58
	// ("cv::sfm::projectionsFromFundamental", vec![(pred!(mut, ["F", "P1", "P2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_projectionsFromFundamental_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* F, const cv::_OutputArray* P1, const cv::_OutputArray* P2, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::projectionsFromFundamental(*F, *P1, *P2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:74
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(const cv::_InputArray* points2d, const cv::_OutputArray* Ps, const cv::_OutputArray* points3d, const cv::_InputOutputArray* K, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*points2d, *Ps, *points3d, *K);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:74
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
	void cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(const cv::_InputArray* points2d, const cv::_OutputArray* Ps, const cv::_OutputArray* points3d, const cv::_InputOutputArray* K, bool is_projective, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*points2d, *Ps, *points3d, *K, is_projective);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:93
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* points2d, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_InputOutputArray* K, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*points2d, *Rs, *Ts, *K, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:93
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* points2d, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_InputOutputArray* K, const cv::_OutputArray* points3d, bool is_projective, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*points2d, *Rs, *Ts, *K, *points3d, is_projective);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:111
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(const std::vector<cv::String>* images, const cv::_OutputArray* Ps, const cv::_OutputArray* points3d, const cv::_InputOutputArray* K, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*images, *Ps, *points3d, *K);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:111
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
	void cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(const std::vector<cv::String>* images, const cv::_OutputArray* Ps, const cv::_OutputArray* points3d, const cv::_InputOutputArray* K, bool is_projective, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*images, *Ps, *points3d, *K, is_projective);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:131
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(const std::vector<cv::String>* images, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_InputOutputArray* K, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*images, *Rs, *Ts, *K, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/reconstruct.hpp:131
	// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(const std::vector<cv::String>* images, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_InputOutputArray* K, const cv::_OutputArray* points3d, bool is_projective, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::reconstruct(*images, *Rs, *Ts, *K, *points3d, is_projective);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// relativeCameraMotion(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/fundamental.hpp:101
	// ("cv::sfm::relativeCameraMotion", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_relativeCameraMotion_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R1, const cv::_InputArray* t1, const cv::_InputArray* R2, const cv::_InputArray* t2, const cv::_OutputArray* R, const cv::_OutputArray* t, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::relativeCameraMotion(*R1, *t1, *R2, *t2, *R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// skew(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/numeric.hpp:69
	// ("cv::sfm::skew", vec![(pred!(mut, ["x"], ["const cv::_InputArray*"]), _)]),
	void cv_sfm_skew_const__InputArrayR(const cv::_InputArray* x, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::sfm::skew(*x);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// triangulatePoints(InputArrayOfArrays, InputArrayOfArrays, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/triangulation.hpp:59
	// ("cv::sfm::triangulatePoints", vec![(pred!(mut, ["points2d", "projection_matrices", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_triangulatePoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points2d, const cv::_InputArray* projection_matrices, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::sfm::triangulatePoints(*points2d, *projection_matrices, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:163
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
	void cv_sfm_BaseSFM_run_const__InputArrayR(cv::sfm::BaseSFM* instance, const cv::_InputArray* points2d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:166
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_BaseSFM_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_InputArray* points2d, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:169
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
	void cv_sfm_BaseSFM_run_const_vectorLStringGR(cv::sfm::BaseSFM* instance, const std::vector<cv::String>* images, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:170
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_BaseSFM_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const std::vector<cv::String>* images, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getError()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:173
	// ("cv::sfm::BaseSFM::getError", vec![(pred!(const, [], []), _)]),
	void cv_sfm_BaseSFM_getError_const(const cv::sfm::BaseSFM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:174
	// ("cv::sfm::BaseSFM::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
	void cv_sfm_BaseSFM_getPoints_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:175
	// ("cv::sfm::BaseSFM::getIntrinsics", vec![(pred!(const, [], []), _)]),
	void cv_sfm_BaseSFM_getIntrinsics_const(const cv::sfm::BaseSFM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIntrinsics();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:176
	// ("cv::sfm::BaseSFM::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_BaseSFM_getCameras_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, ResultVoid* ocvrs_return) {
		try {
			instance->getCameras(*Rs, *Ts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:180
	// ("cv::sfm::BaseSFM::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	void cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsR(cv::sfm::BaseSFM* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options, ResultVoid* ocvrs_return) {
		try {
			instance->setReconstructionOptions(*libmv_reconstruction_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:184
	// ("cv::sfm::BaseSFM::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
	void cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(cv::sfm::BaseSFM* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::BaseSFM::to_SFMLibmvEuclideanReconstruction() generated
	// ("cv::sfm::BaseSFM::to_SFMLibmvEuclideanReconstruction", vec![(pred!(mut, [], []), _)]),
	cv::sfm::SFMLibmvEuclideanReconstruction* cv_sfm_BaseSFM_to_SFMLibmvEuclideanReconstruction(cv::sfm::BaseSFM* instance) {
			return dynamic_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance);
	}

	// cv::sfm::BaseSFM::delete() generated
	// ("cv::sfm::BaseSFM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_sfm_BaseSFM_delete(cv::sfm::BaseSFM* instance) {
			delete instance;
	}

	// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:199
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_InputArray* points2d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:212
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_InputArray* points2d, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:222
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const std::vector<cv::String>* images, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:235
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const std::vector<cv::String>* images, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getError()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:241
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getError", vec![(pred!(const, [], []), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(const cv::sfm::SFMLibmvEuclideanReconstruction* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:247
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:252
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getIntrinsics", vec![(pred!(const, [], []), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(const cv::sfm::SFMLibmvEuclideanReconstruction* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIntrinsics();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:259
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, ResultVoid* ocvrs_return) {
		try {
			instance->getCameras(*Rs, *Ts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:267
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options, ResultVoid* ocvrs_return) {
		try {
			instance->setReconstructionOptions(*libmv_reconstruction_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:275
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const libmv_CameraIntrinsicsOptions &, const libmv_ReconstructionOptions &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:279
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, ["camera_instrinsic_options", "reconstruction_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*", "const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsR_const_libmv_ReconstructionOptionsR(const cv::sfm::libmv_CameraIntrinsicsOptions* camera_instrinsic_options, const cv::sfm::libmv_ReconstructionOptions* reconstruction_options, Result<cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction> ret = cv::sfm::SFMLibmvEuclideanReconstruction::create(*camera_instrinsic_options, *reconstruction_options);
			Ok(new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::SFMLibmvEuclideanReconstruction::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:279
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, [], []), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_create(Result<cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction> ret = cv::sfm::SFMLibmvEuclideanReconstruction::create();
			Ok(new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::SFMLibmvEuclideanReconstruction::to_BaseSFM() generated
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::to_BaseSFM", vec![(pred!(mut, [], []), _)]),
	cv::sfm::BaseSFM* cv_sfm_SFMLibmvEuclideanReconstruction_to_BaseSFM(cv::sfm::SFMLibmvEuclideanReconstruction* instance) {
			return dynamic_cast<cv::sfm::BaseSFM*>(instance);
	}

	// cv::sfm::SFMLibmvEuclideanReconstruction::delete() generated
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::delete", vec![(pred!(mut, [], []), _)]),
	void cv_sfm_SFMLibmvEuclideanReconstruction_delete(cv::sfm::SFMLibmvEuclideanReconstruction* instance) {
			delete instance;
	}

	// libmv_CameraIntrinsicsOptions(const int, const double, const double, const double, const double, const double, const double, const double, const double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:75
	// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, ["_distortion_model", "_focal_length", "_principal_point_x", "_principal_point_y", "_polynomial_k1", "_polynomial_k2", "_polynomial_k3", "_polynomial_p1", "_polynomial_p2"], ["const int", "const double", "const double", "const double", "const double", "const double", "const double", "const double", "const double"]), _)]),
	void cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_const_int_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double(const int _distortion_model, const double _focal_length, const double _principal_point_x, const double _principal_point_y, const double _polynomial_k1, const double _polynomial_k2, const double _polynomial_k3, const double _polynomial_p1, const double _polynomial_p2, Result<cv::sfm::libmv_CameraIntrinsicsOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_CameraIntrinsicsOptions ret(_distortion_model, _focal_length, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:75
	// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, [], []), _)]),
	void cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions(Result<cv::sfm::libmv_CameraIntrinsicsOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_CameraIntrinsicsOptions ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// libmv_ReconstructionOptions(const int, const int, const int, const int, const int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:138
	// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, ["_keyframe1", "_keyframe2", "_refine_intrinsics", "_select_keyframes", "_verbosity_level"], ["const int", "const int", "const int", "const int", "const int"]), _)]),
	void cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_const_int_const_int_const_int_const_int_const_int(const int _keyframe1, const int _keyframe2, const int _refine_intrinsics, const int _select_keyframes, const int _verbosity_level, Result<cv::sfm::libmv_ReconstructionOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_ReconstructionOptions ret(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/sfm/simple_pipeline.hpp:138
	// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, [], []), _)]),
	void cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions(Result<cv::sfm::libmv_ReconstructionOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_ReconstructionOptions ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
