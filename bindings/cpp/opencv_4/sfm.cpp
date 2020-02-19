#include "sfm.hpp"
#include "sfm_types.hpp"

extern "C" {
	Result_void cv_sfm_KRtFromProjection_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* P, void* K, void* R, void* t) {
		try {
			cv::sfm::KRtFromProjection(*reinterpret_cast<const cv::_InputArray*>(P), *reinterpret_cast<const cv::_OutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_applyTransformationToPoints_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* points, void* T, void* transformed_points) {
		try {
			cv::sfm::applyTransformationToPoints(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_InputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(transformed_points));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_computeOrientation_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double(void* x1, void* x2, void* R, void* t, double s) {
		try {
			cv::sfm::computeOrientation(*reinterpret_cast<const cv::_InputArray*>(x1), *reinterpret_cast<const cv::_InputArray*>(x2), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t), s);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_sfm_depth_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* R, void* t, void* X) {
		try {
			double ret = cv::sfm::depth(*reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(t), *reinterpret_cast<const cv::_InputArray*>(X));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_sfm_essentialFromFundamental_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* F, void* K1, void* K2, void* E) {
		try {
			cv::sfm::essentialFromFundamental(*reinterpret_cast<const cv::_InputArray*>(F), *reinterpret_cast<const cv::_InputArray*>(K1), *reinterpret_cast<const cv::_InputArray*>(K2), *reinterpret_cast<const cv::_OutputArray*>(E));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_essentialFromRt_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* R1, void* t1, void* R2, void* t2, void* E) {
		try {
			cv::sfm::essentialFromRt(*reinterpret_cast<const cv::_InputArray*>(R1), *reinterpret_cast<const cv::_InputArray*>(t1), *reinterpret_cast<const cv::_InputArray*>(R2), *reinterpret_cast<const cv::_InputArray*>(t2), *reinterpret_cast<const cv::_OutputArray*>(E));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_euclideanToHomogeneous_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::sfm::euclideanToHomogeneous(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX_const__OutputArrayX_double(void* x1, void* x2, double max_error, void* F, void* inliers, double outliers_probability) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences7PointRobust(*reinterpret_cast<const cv::_InputArray*>(x1), *reinterpret_cast<const cv::_InputArray*>(x2), max_error, *reinterpret_cast<const cv::_OutputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(inliers), outliers_probability);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX_const__OutputArrayX_double(void* x1, void* x2, double max_error, void* F, void* inliers, double outliers_probability) {
		try {
			double ret = cv::sfm::fundamentalFromCorrespondences8PointRobust(*reinterpret_cast<const cv::_InputArray*>(x1), *reinterpret_cast<const cv::_InputArray*>(x2), max_error, *reinterpret_cast<const cv::_OutputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(inliers), outliers_probability);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_sfm_fundamentalFromEssential_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* E, void* K1, void* K2, void* F) {
		try {
			cv::sfm::fundamentalFromEssential(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_InputArray*>(K1), *reinterpret_cast<const cv::_InputArray*>(K2), *reinterpret_cast<const cv::_OutputArray*>(F));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_fundamentalFromProjections_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* P1, void* P2, void* F) {
		try {
			cv::sfm::fundamentalFromProjections(*reinterpret_cast<const cv::_InputArray*>(P1), *reinterpret_cast<const cv::_InputArray*>(P2), *reinterpret_cast<const cv::_OutputArray*>(F));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_homogeneousToEuclidean_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::sfm::homogeneousToEuclidean(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_importReconstruction_const_StringX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int(const char* file, void* Rs, void* Ts, void* Ks, void* points3d, int file_format) {
		try {
			cv::sfm::importReconstruction(std::string(file), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_OutputArray*>(Ks), *reinterpret_cast<const cv::_OutputArray*>(points3d), file_format);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_isotropicPreconditionerFromPoints_const__InputArrayX_const__OutputArrayX(void* points, void* T) {
		try {
			cv::sfm::isotropicPreconditionerFromPoints(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(T));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_meanAndVarianceAlongRows_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* A, void* mean, void* variance) {
		try {
			cv::sfm::meanAndVarianceAlongRows(*reinterpret_cast<const cv::_InputArray*>(A), *reinterpret_cast<const cv::_OutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(variance));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_sfm_motionFromEssentialChooseSolution_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* Rs, void* ts, void* K1, void* x1, void* K2, void* x2) {
		try {
			int ret = cv::sfm::motionFromEssentialChooseSolution(*reinterpret_cast<const cv::_InputArray*>(Rs), *reinterpret_cast<const cv::_InputArray*>(ts), *reinterpret_cast<const cv::_InputArray*>(K1), *reinterpret_cast<const cv::_InputArray*>(x1), *reinterpret_cast<const cv::_InputArray*>(K2), *reinterpret_cast<const cv::_InputArray*>(x2));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_sfm_motionFromEssential_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* E, void* Rs, void* ts) {
		try {
			cv::sfm::motionFromEssential(*reinterpret_cast<const cv::_InputArray*>(E), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(ts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_normalizeFundamental_const__InputArrayX_const__OutputArrayX(void* F, void* F_normalized) {
		try {
			cv::sfm::normalizeFundamental(*reinterpret_cast<const cv::_InputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(F_normalized));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_normalizeIsotropicPoints_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* points, void* normalized_points, void* T) {
		try {
			cv::sfm::normalizeIsotropicPoints(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(normalized_points), *reinterpret_cast<const cv::_OutputArray*>(T));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_normalizePoints_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* points, void* normalized_points, void* T) {
		try {
			cv::sfm::normalizePoints(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(normalized_points), *reinterpret_cast<const cv::_OutputArray*>(T));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_normalizedEightPointSolver_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* x1, void* x2, void* F) {
		try {
			cv::sfm::normalizedEightPointSolver(*reinterpret_cast<const cv::_InputArray*>(x1), *reinterpret_cast<const cv::_InputArray*>(x2), *reinterpret_cast<const cv::_OutputArray*>(F));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_preconditionerFromPoints_const__InputArrayX_const__OutputArrayX(void* points, void* T) {
		try {
			cv::sfm::preconditionerFromPoints(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(T));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_projectionFromKRt_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* K, void* R, void* t, void* P) {
		try {
			cv::sfm::projectionFromKRt(*reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(t), *reinterpret_cast<const cv::_OutputArray*>(P));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_projectionsFromFundamental_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* F, void* P1, void* P2) {
		try {
			cv::sfm::projectionsFromFundamental(*reinterpret_cast<const cv::_InputArray*>(F), *reinterpret_cast<const cv::_OutputArray*>(P1), *reinterpret_cast<const cv::_OutputArray*>(P2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_reconstruct_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX_bool(void* points2d, void* Ps, void* points3d, void* K, bool is_projective) {
		try {
			cv::sfm::reconstruct(*reinterpret_cast<const cv::_InputArray*>(points2d), *reinterpret_cast<const cv::_OutputArray*>(Ps), *reinterpret_cast<const cv::_OutputArray*>(points3d), *reinterpret_cast<const cv::_InputOutputArray*>(K), is_projective);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_reconstruct_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX_const__OutputArrayX_bool(void* points2d, void* Rs, void* Ts, void* K, void* points3d, bool is_projective) {
		try {
			cv::sfm::reconstruct(*reinterpret_cast<const cv::_InputArray*>(points2d), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(points3d), is_projective);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_reconstruct_vector_String__const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX_bool(void* images, void* Ps, void* points3d, void* K, bool is_projective) {
		try {
			cv::sfm::reconstruct(*reinterpret_cast<std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_OutputArray*>(Ps), *reinterpret_cast<const cv::_OutputArray*>(points3d), *reinterpret_cast<const cv::_InputOutputArray*>(K), is_projective);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_reconstruct_vector_String__const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX_const__OutputArrayX_bool(void* images, void* Rs, void* Ts, void* K, void* points3d, bool is_projective) {
		try {
			cv::sfm::reconstruct(*reinterpret_cast<std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(points3d), is_projective);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_relativeCameraMotion_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* R1, void* t1, void* R2, void* t2, void* R, void* t) {
		try {
			cv::sfm::relativeCameraMotion(*reinterpret_cast<const cv::_InputArray*>(R1), *reinterpret_cast<const cv::_InputArray*>(t1), *reinterpret_cast<const cv::_InputArray*>(R2), *reinterpret_cast<const cv::_InputArray*>(t2), *reinterpret_cast<const cv::_OutputArray*>(R), *reinterpret_cast<const cv::_OutputArray*>(t));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_sfm_skew_const__InputArrayX(void* x) {
		try {
			cv::Mat ret = cv::sfm::skew(*reinterpret_cast<const cv::_InputArray*>(x));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_sfm_triangulatePoints_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* points2d, void* projection_matrices, void* points3d) {
		try {
			cv::sfm::triangulatePoints(*reinterpret_cast<const cv::_InputArray*>(points2d), *reinterpret_cast<const cv::_InputArray*>(projection_matrices), *reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_run_const__InputArrayX(void* instance, void* points2d) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(points2d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_run_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* points2d, void* K, void* Rs, void* Ts, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(points2d), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_run_const_vector_String_X(void* instance, void* images) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->run(*reinterpret_cast<const std::vector<cv::String>*>(images));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_run_const_vector_String_X_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* images, void* K, void* Rs, void* Ts, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->run(*reinterpret_cast<const std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_sfm_BaseSFM_getError_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::sfm::BaseSFM*>(instance)->getError();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_sfm_BaseSFM_getPoints_const__OutputArrayX(void* instance, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->getPoints(*reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_sfm_BaseSFM_getIntrinsics_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::sfm::BaseSFM*>(instance)->getIntrinsics();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_sfm_BaseSFM_getCameras_const__OutputArrayX_const__OutputArrayX(void* instance, void* Rs, void* Ts) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->getCameras(*reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsX(void* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->setReconstructionOptions(*libmv_reconstruction_options);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsX(void* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options) {
		try {
			reinterpret_cast<cv::sfm::BaseSFM*>(instance)->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayX(void* instance, void* points2d) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(points2d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* points2d, void* K, void* Rs, void* Ts, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(points2d), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vector_String_X(void* instance, void* images) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->run(*reinterpret_cast<const std::vector<cv::String>*>(images));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vector_String_X_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* images, void* K, void* Rs, void* Ts, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->run(*reinterpret_cast<const std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts), *reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->getError();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayX(void* instance, void* points3d) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->getPoints(*reinterpret_cast<const cv::_OutputArray*>(points3d));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->getIntrinsics();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayX_const__OutputArrayX(void* instance, void* Rs, void* Ts) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->getCameras(*reinterpret_cast<const cv::_OutputArray*>(Rs), *reinterpret_cast<const cv::_OutputArray*>(Ts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsX(void* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->setReconstructionOptions(*libmv_reconstruction_options);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsX(void* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options) {
		try {
			reinterpret_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance)->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsX_const_libmv_ReconstructionOptionsX(const cv::sfm::libmv_CameraIntrinsicsOptions* camera_instrinsic_options, const cv::sfm::libmv_ReconstructionOptions* reconstruction_options) {
		try {
			cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction> ret = cv::sfm::SFMLibmvEuclideanReconstruction::create(*camera_instrinsic_options, *reconstruction_options);
			return Ok<void*>(new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::sfm::libmv_CameraIntrinsicsOptions> cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_int_double_double_double_double_double_double_double_double_double(int _distortion_model, double _focal_length_x, double _focal_length_y, double _principal_point_x, double _principal_point_y, double _polynomial_k1, double _polynomial_k2, double _polynomial_k3, double _polynomial_p1, double _polynomial_p2) {
		try {
			cv::sfm::libmv_CameraIntrinsicsOptions ret(_distortion_model, _focal_length_x, _focal_length_y, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2);
			return Ok<cv::sfm::libmv_CameraIntrinsicsOptions>(ret);
		} OCVRS_CATCH(Result<cv::sfm::libmv_CameraIntrinsicsOptions>)
	}
	
	Result<cv::sfm::libmv_ReconstructionOptions> cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_int_int_int_int_int(int _keyframe1, int _keyframe2, int _refine_intrinsics, int _select_keyframes, int _verbosity_level) {
		try {
			cv::sfm::libmv_ReconstructionOptions ret(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level);
			return Ok<cv::sfm::libmv_ReconstructionOptions>(ret);
		} OCVRS_CATCH(Result<cv::sfm::libmv_ReconstructionOptions>)
	}
	
}
