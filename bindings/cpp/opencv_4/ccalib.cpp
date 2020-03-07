#include "ccalib.hpp"
#include "ccalib_types.hpp"

extern "C" {
	Result<double> cv_omnidir_calibrate_const__InputArrayX_const__InputArrayX_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria_const__OutputArrayX(void* objectPoints, void* imagePoints, const cv::Size* size, void* K, void* xi, void* D, void* rvecs, void* tvecs, int flags, void* criteria, void* idx) {
		try {
			double ret = cv::omnidir::calibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *size, *reinterpret_cast<const cv::_InputOutputArray*>(K), *reinterpret_cast<const cv::_InputOutputArray*>(xi), *reinterpret_cast<const cv::_InputOutputArray*>(D), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria), *reinterpret_cast<const cv::_OutputArray*>(idx));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_omnidir_initUndistortRectifyMap_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_int_const__OutputArrayX_const__OutputArrayX_int(void* K, void* D, void* xi, void* R, void* P, const cv::Size* size, int mltype, void* map1, void* map2, int flags) {
		try {
			cv::omnidir::initUndistortRectifyMap(*reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(xi), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(P), *size, mltype, *reinterpret_cast<const cv::_OutputArray*>(map1), *reinterpret_cast<const cv::_OutputArray*>(map2), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_omnidir_projectPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_double_const__InputArrayX_const__OutputArrayX(void* objectPoints, void* imagePoints, void* rvec, void* tvec, void* K, double xi, void* D, void* jacobian) {
		try {
			cv::omnidir::projectPoints(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_OutputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_InputArray*>(K), xi, *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_OutputArray*>(jacobian));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_omnidir_stereoCalibrate_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const_SizeX_const_SizeX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria_const__OutputArrayX(void* objectPoints, void* imagePoints1, void* imagePoints2, const cv::Size* imageSize1, const cv::Size* imageSize2, void* K1, void* xi1, void* D1, void* K2, void* xi2, void* D2, void* rvec, void* tvec, void* rvecsL, void* tvecsL, int flags, void* criteria, void* idx) {
		try {
			double ret = cv::omnidir::stereoCalibrate(*reinterpret_cast<const cv::_InputOutputArray*>(objectPoints), *reinterpret_cast<const cv::_InputOutputArray*>(imagePoints1), *reinterpret_cast<const cv::_InputOutputArray*>(imagePoints2), *imageSize1, *imageSize2, *reinterpret_cast<const cv::_InputOutputArray*>(K1), *reinterpret_cast<const cv::_InputOutputArray*>(xi1), *reinterpret_cast<const cv::_InputOutputArray*>(D1), *reinterpret_cast<const cv::_InputOutputArray*>(K2), *reinterpret_cast<const cv::_InputOutputArray*>(xi2), *reinterpret_cast<const cv::_InputOutputArray*>(D2), *reinterpret_cast<const cv::_OutputArray*>(rvec), *reinterpret_cast<const cv::_OutputArray*>(tvec), *reinterpret_cast<const cv::_OutputArray*>(rvecsL), *reinterpret_cast<const cv::_OutputArray*>(tvecsL), flags, *reinterpret_cast<cv::TermCriteria*>(criteria), *reinterpret_cast<const cv::_OutputArray*>(idx));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_omnidir_stereoReconstruct_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_int_int_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const_SizeX_const__InputArrayX_const__OutputArrayX_int(void* image1, void* image2, void* K1, void* D1, void* xi1, void* K2, void* D2, void* xi2, void* R, void* T, int flag, int numDisparities, int SADWindowSize, void* disparity, void* image1Rec, void* image2Rec, const cv::Size* newSize, void* Knew, void* pointCloud, int pointType) {
		try {
			cv::omnidir::stereoReconstruct(*reinterpret_cast<const cv::_InputArray*>(image1), *reinterpret_cast<const cv::_InputArray*>(image2), *reinterpret_cast<const cv::_InputArray*>(K1), *reinterpret_cast<const cv::_InputArray*>(D1), *reinterpret_cast<const cv::_InputArray*>(xi1), *reinterpret_cast<const cv::_InputArray*>(K2), *reinterpret_cast<const cv::_InputArray*>(D2), *reinterpret_cast<const cv::_InputArray*>(xi2), *reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), flag, numDisparities, SADWindowSize, *reinterpret_cast<const cv::_OutputArray*>(disparity), *reinterpret_cast<const cv::_OutputArray*>(image1Rec), *reinterpret_cast<const cv::_OutputArray*>(image2Rec), *newSize, *reinterpret_cast<const cv::_InputArray*>(Knew), *reinterpret_cast<const cv::_OutputArray*>(pointCloud), pointType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_omnidir_stereoRectify_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* R, void* T, void* R1, void* R2) {
		try {
			cv::omnidir::stereoRectify(*reinterpret_cast<const cv::_InputArray*>(R), *reinterpret_cast<const cv::_InputArray*>(T), *reinterpret_cast<const cv::_OutputArray*>(R1), *reinterpret_cast<const cv::_OutputArray*>(R2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_omnidir_undistortImage_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_const__InputArrayX_const_SizeX_const__InputArrayX(void* distorted, void* undistorted, void* K, void* D, void* xi, int flags, void* Knew, const cv::Size* new_size, void* R) {
		try {
			cv::omnidir::undistortImage(*reinterpret_cast<const cv::_InputArray*>(distorted), *reinterpret_cast<const cv::_OutputArray*>(undistorted), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(xi), flags, *reinterpret_cast<const cv::_InputArray*>(Knew), *new_size, *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_omnidir_undistortPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* distorted, void* undistorted, void* K, void* D, void* xi, void* R) {
		try {
			cv::omnidir::undistortPoints(*reinterpret_cast<const cv::_InputArray*>(distorted), *reinterpret_cast<const cv::_OutputArray*>(undistorted), *reinterpret_cast<const cv::_InputArray*>(K), *reinterpret_cast<const cv::_InputArray*>(D), *reinterpret_cast<const cv::_InputArray*>(xi), *reinterpret_cast<const cv::_InputArray*>(R));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CustomPattern_delete(cv::ccalib::CustomPattern* instance) {
		delete instance;
	}
	Result<void*> cv_ccalib_CustomPattern_CustomPattern() {
		try {
			cv::ccalib::CustomPattern* ret = new cv::ccalib::CustomPattern();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_create_const__InputArrayX_Size2f_const__OutputArrayX(void* instance, void* pattern, const cv::Size2f* boardSize, void* output) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->create(*reinterpret_cast<const cv::_InputArray*>(pattern), *boardSize, *reinterpret_cast<const cv::_OutputArray*>(output));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_findPattern_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_double_bool_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* image, void* matched_features, void* pattern_points, double ratio, double proj_error, bool refine_position, void* out, void* H, void* pattern_corners) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->findPattern(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(matched_features), *reinterpret_cast<const cv::_OutputArray*>(pattern_points), ratio, proj_error, refine_position, *reinterpret_cast<const cv::_OutputArray*>(out), *reinterpret_cast<const cv::_OutputArray*>(H), *reinterpret_cast<const cv::_OutputArray*>(pattern_corners));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_isInitialized(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->isInitialized();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ccalib_CustomPattern_getPatternPoints_vector_KeyPoint_X(void* instance, void* original_points) {
		try {
			reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->getPatternPoints(*reinterpret_cast<std::vector<cv::KeyPoint>*>(original_points));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ccalib_CustomPattern_getPixelSize(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->getPixelSize();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_setFeatureDetector_Ptr_FeatureDetector_(void* instance, void* featureDetector) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->setFeatureDetector(*reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(featureDetector));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_setDescriptorExtractor_Ptr_DescriptorExtractor_(void* instance, void* extractor) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->setDescriptorExtractor(*reinterpret_cast<cv::Ptr<cv::DescriptorExtractor>*>(extractor));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_setDescriptorMatcher_Ptr_DescriptorMatcher_(void* instance, void* matcher) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->setDescriptorMatcher(*reinterpret_cast<cv::Ptr<cv::DescriptorMatcher>*>(matcher));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ccalib_CustomPattern_getFeatureDetector(void* instance) {
		try {
			cv::Ptr<cv::FeatureDetector> ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->getFeatureDetector();
			return Ok<void*>(new cv::Ptr<cv::FeatureDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ccalib_CustomPattern_getDescriptorExtractor(void* instance) {
		try {
			cv::Ptr<cv::DescriptorExtractor> ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->getDescriptorExtractor();
			return Ok<void*>(new cv::Ptr<cv::DescriptorExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ccalib_CustomPattern_getDescriptorMatcher(void* instance) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->getDescriptorMatcher();
			return Ok<void*>(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_ccalib_CustomPattern_calibrate_const__InputArrayX_const__InputArrayX_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* instance, void* objectPoints, void* imagePoints, const cv::Size* imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, int flags, void* criteria) {
		try {
			double ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->calibrate(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRt_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_bool_int(void* instance, void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->findRt(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), useExtrinsicGuess, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRt_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_bool_int(void* instance, void* image, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->findRt(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), useExtrinsicGuess, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_bool_int_float_int_const__OutputArrayX_int(void* instance, void* objectPoints, void* imagePoints, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, void* inliers, int flags) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->findRtRANSAC(*reinterpret_cast<const cv::_InputArray*>(objectPoints), *reinterpret_cast<const cv::_InputArray*>(imagePoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *reinterpret_cast<const cv::_OutputArray*>(inliers), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_bool_int_float_int_const__OutputArrayX_int(void* instance, void* image, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, void* inliers, int flags) {
		try {
			bool ret = reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->findRtRANSAC(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputOutputArray*>(rvec), *reinterpret_cast<const cv::_InputOutputArray*>(tvec), useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *reinterpret_cast<const cv::_OutputArray*>(inliers), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_double_int(void* instance, void* image, void* tvec, void* rvec, void* cameraMatrix, void* distCoeffs, double axis_length, int axis_width) {
		try {
			reinterpret_cast<cv::ccalib::CustomPattern*>(instance)->drawOrientation(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(tvec), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), axis_length, axis_width);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MultiCameraCalibration_delete(cv::multicalib::MultiCameraCalibration* instance) {
		delete instance;
	}
	Result<void*> cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringX_float_float_int_int_int_int_TermCriteria_Ptr_FeatureDetector__Ptr_DescriptorExtractor__Ptr_DescriptorMatcher_(int cameraType, int nCameras, const char* fileName, float patternWidth, float patternHeight, int verbose, int showExtration, int nMiniMatches, int flags, void* criteria, void* detector, void* descriptor, void* matcher) {
		try {
			cv::multicalib::MultiCameraCalibration* ret = new cv::multicalib::MultiCameraCalibration(cameraType, nCameras, std::string(fileName), patternWidth, patternHeight, verbose, showExtration, nMiniMatches, flags, *reinterpret_cast<cv::TermCriteria*>(criteria), *reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(detector), *reinterpret_cast<cv::Ptr<cv::DescriptorExtractor>*>(descriptor), *reinterpret_cast<cv::Ptr<cv::DescriptorMatcher>*>(matcher));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_loadImages(void* instance) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration*>(instance)->loadImages();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_initialize(void* instance) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration*>(instance)->initialize();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(void* instance) {
		try {
			double ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration*>(instance)->optimizeExtrinsics();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_multicalib_MultiCameraCalibration_run(void* instance) {
		try {
			double ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration*>(instance)->run();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_writeParameters_const_stringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration*>(instance)->writeParameters(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_cameraVertex_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->cameraVertex;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setCameraVertex_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->cameraVertex = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_photoVertex_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->photoVertex;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPhotoVertex_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->photoVertex = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_photoIndex_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->photoIndex;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPhotoIndex_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->photoIndex = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_multicalib_MultiCameraCalibration_edge_transform(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->transform;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setTransform_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::edge*>(instance)->transform = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MultiCameraCalibration_edge_delete(cv::multicalib::MultiCameraCalibration::edge* instance) {
		delete instance;
	}
	Result<void*> cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(int cv, int pv, int pi, void* trans) {
		try {
			cv::multicalib::MultiCameraCalibration::edge* ret = new cv::multicalib::MultiCameraCalibration::edge(cv, pv, pi, *reinterpret_cast<cv::Mat*>(trans));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_multicalib_MultiCameraCalibration_vertex_pose(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::vertex*>(instance)->pose;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_vertex_setPose_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::vertex*>(instance)->pose = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_vertex_timestamp_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::multicalib::MultiCameraCalibration::vertex*>(instance)->timestamp;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_vertex_setTimestamp_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::multicalib::MultiCameraCalibration::vertex*>(instance)->timestamp = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MultiCameraCalibration_vertex_delete(cv::multicalib::MultiCameraCalibration::vertex* instance) {
		delete instance;
	}
	Result<void*> cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(void* po, int ts) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex(*reinterpret_cast<cv::Mat*>(po), ts);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_multicalib_MultiCameraCalibration_vertex_vertex() {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_RandomPatternCornerFinder_delete(cv::randpattern::RandomPatternCornerFinder* instance) {
		delete instance;
	}
	Result<void*> cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_Ptr_FeatureDetector__Ptr_DescriptorExtractor__Ptr_DescriptorMatcher_(float patternWidth, float patternHeight, int nminiMatch, int depth, int verbose, int showExtraction, void* detector, void* descriptor, void* matcher) {
		try {
			cv::randpattern::RandomPatternCornerFinder* ret = new cv::randpattern::RandomPatternCornerFinder(patternWidth, patternHeight, nminiMatch, depth, verbose, showExtraction, *reinterpret_cast<cv::Ptr<cv::FeatureDetector>*>(detector), *reinterpret_cast<cv::Ptr<cv::DescriptorExtractor>*>(descriptor), *reinterpret_cast<cv::Ptr<cv::DescriptorMatcher>*>(matcher));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatX(void* instance, void* patternImage) {
		try {
			reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->loadPattern(*reinterpret_cast<const cv::Mat*>(patternImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatX_const_vector_KeyPoint_X_const_MatX(void* instance, void* patternImage, void* patternKeyPoints, void* patternDescriptors) {
		try {
			reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->loadPattern(*reinterpret_cast<const cv::Mat*>(patternImage), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(patternKeyPoints), *reinterpret_cast<const cv::Mat*>(patternDescriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vector_Mat_(void* instance, void* inputImages) {
		try {
			reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->computeObjectImagePoints(*reinterpret_cast<std::vector<cv::Mat>*>(inputImages));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(void* instance, void* inputImage) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->computeObjectImagePointsForSingle(*reinterpret_cast<cv::Mat*>(inputImage));
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_randpattern_RandomPatternCornerFinder_getObjectPoints(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->getObjectPoints();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_randpattern_RandomPatternCornerFinder_getImagePoints(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::randpattern::RandomPatternCornerFinder*>(instance)->getImagePoints();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_RandomPatternGenerator_delete(cv::randpattern::RandomPatternGenerator* instance) {
		delete instance;
	}
	Result<void*> cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(int imageWidth, int imageHeight) {
		try {
			cv::randpattern::RandomPatternGenerator* ret = new cv::randpattern::RandomPatternGenerator(imageWidth, imageHeight);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_randpattern_RandomPatternGenerator_generatePattern(void* instance) {
		try {
			reinterpret_cast<cv::randpattern::RandomPatternGenerator*>(instance)->generatePattern();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_randpattern_RandomPatternGenerator_getPattern(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::randpattern::RandomPatternGenerator*>(instance)->getPattern();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
