#include "ccalib.hpp"
#include "ccalib_types.hpp"

extern "C" {
	Result<double> cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* xi, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria, const cv::_OutputArray* idx) {
		try {
			double ret = cv::omnidir::calibrate(*objectPoints, *imagePoints, *size, *K, *xi, *D, *rvecs, *tvecs, flags, *criteria, *idx);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int mltype, const cv::_OutputArray* map1, const cv::_OutputArray* map2, int flags) {
		try {
			cv::omnidir::initUndistortRectifyMap(*K, *D, *xi, *R, *P, *size, mltype, *map1, *map2, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, double xi, const cv::_InputArray* D, const cv::_OutputArray* jacobian) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, xi, *D, *jacobian);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputOutputArray* objectPoints, const cv::_InputOutputArray* imagePoints1, const cv::_InputOutputArray* imagePoints2, const cv::Size* imageSize1, const cv::Size* imageSize2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* xi1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* xi2, const cv::_InputOutputArray* D2, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* rvecsL, const cv::_OutputArray* tvecsL, int flags, const cv::TermCriteria* criteria, const cv::_OutputArray* idx) {
		try {
			double ret = cv::omnidir::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *imageSize1, *imageSize2, *K1, *xi1, *D1, *K2, *xi2, *D2, *rvec, *tvec, *rvecsL, *tvecsL, flags, *criteria, *idx);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* image1, const cv::_InputArray* image2, const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* xi1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::_InputArray* xi2, const cv::_InputArray* R, const cv::_InputArray* T, int flag, int numDisparities, int SADWindowSize, const cv::_OutputArray* disparity, const cv::_OutputArray* image1Rec, const cv::_OutputArray* image2Rec, const cv::Size* newSize, const cv::_InputArray* Knew, const cv::_OutputArray* pointCloud, int pointType) {
		try {
			cv::omnidir::stereoReconstruct(*image1, *image2, *K1, *D1, *xi1, *K2, *D2, *xi2, *R, *T, flag, numDisparities, SADWindowSize, *disparity, *image1Rec, *image2Rec, *newSize, *Knew, *pointCloud, pointType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2) {
		try {
			cv::omnidir::stereoRectify(*R, *T, *R1, *R2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, int flags, const cv::_InputArray* Knew, const cv::Size* new_size, const cv::_InputArray* R) {
		try {
			cv::omnidir::undistortImage(*distorted, *undistorted, *K, *D, *xi, flags, *Knew, *new_size, *R);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R) {
		try {
			cv::omnidir::undistortPoints(*distorted, *undistorted, *K, *D, *xi, *R);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CustomPattern_delete(cv::ccalib::CustomPattern* instance) {
		delete instance;
	}
	Result<cv::ccalib::CustomPattern*> cv_ccalib_CustomPattern_CustomPattern() {
		try {
			cv::ccalib::CustomPattern* ret = new cv::ccalib::CustomPattern();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ccalib::CustomPattern*>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_create_const__InputArrayR_Size2f_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* pattern, const cv::Size2f* boardSize, const cv::_OutputArray* output) {
		try {
			bool ret = instance->create(*pattern, *boardSize, *output);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_OutputArray* matched_features, const cv::_OutputArray* pattern_points, double ratio, double proj_error, bool refine_position, const cv::_OutputArray* out, const cv::_OutputArray* H, const cv::_OutputArray* pattern_corners) {
		try {
			bool ret = instance->findPattern(*image, *matched_features, *pattern_points, ratio, proj_error, refine_position, *out, *H, *pattern_corners);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_isInitialized(cv::ccalib::CustomPattern* instance) {
		try {
			bool ret = instance->isInitialized();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_ccalib_CustomPattern_getPatternPoints_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_OutputArray* original_points) {
		try {
			instance->getPatternPoints(*original_points);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_ccalib_CustomPattern_getPixelSize(cv::ccalib::CustomPattern* instance) {
		try {
			double ret = instance->getPixelSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_setFeatureDetector_Ptr_Feature2D_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* featureDetector) {
		try {
			bool ret = instance->setFeatureDetector(*featureDetector);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_setDescriptorExtractor_Ptr_Feature2D_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* extractor) {
		try {
			bool ret = instance->setDescriptorExtractor(*extractor);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_setDescriptorMatcher_Ptr_DescriptorMatcher_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::DescriptorMatcher>* matcher) {
		try {
			bool ret = instance->setDescriptorMatcher(*matcher);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::Feature2D>*> cv_ccalib_CustomPattern_getFeatureDetector(cv::ccalib::CustomPattern* instance) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getFeatureDetector();
			return Ok(new cv::Ptr<cv::Feature2D>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	Result<cv::Ptr<cv::Feature2D>*> cv_ccalib_CustomPattern_getDescriptorExtractor(cv::ccalib::CustomPattern* instance) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getDescriptorExtractor();
			return Ok(new cv::Ptr<cv::Feature2D>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	Result<cv::Ptr<cv::DescriptorMatcher>*> cv_ccalib_CustomPattern_getDescriptorMatcher(cv::ccalib::CustomPattern* instance) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->getDescriptorMatcher();
			return Ok(new cv::Ptr<cv::DescriptorMatcher>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	Result<double> cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria) {
		try {
			double ret = instance->calibrate(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = instance->findRt(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int flags) {
		try {
			bool ret = instance->findRt(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags) {
		try {
			bool ret = instance->findRtRANSAC(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags) {
		try {
			bool ret = instance->findRtRANSAC(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(cv::ccalib::CustomPattern* instance, const cv::_InputOutputArray* image, const cv::_InputArray* tvec, const cv::_InputArray* rvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, double axis_length, int axis_width) {
		try {
			instance->drawOrientation(*image, *tvec, *rvec, *cameraMatrix, *distCoeffs, axis_length, axis_width);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiCameraCalibration_delete(cv::multicalib::MultiCameraCalibration* instance) {
		delete instance;
	}
	Result<cv::multicalib::MultiCameraCalibration*> cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(int cameraType, int nCameras, const char* fileName, float patternWidth, float patternHeight, int verbose, int showExtration, int nMiniMatches, int flags, const cv::TermCriteria* criteria, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher) {
		try {
			cv::multicalib::MultiCameraCalibration* ret = new cv::multicalib::MultiCameraCalibration(cameraType, nCameras, std::string(fileName), patternWidth, patternHeight, verbose, showExtration, nMiniMatches, flags, *criteria, *detector, *descriptor, *matcher);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration*>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_loadImages(cv::multicalib::MultiCameraCalibration* instance) {
		try {
			instance->loadImages();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_initialize(cv::multicalib::MultiCameraCalibration* instance) {
		try {
			instance->initialize();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(cv::multicalib::MultiCameraCalibration* instance) {
		try {
			double ret = instance->optimizeExtrinsics();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_multicalib_MultiCameraCalibration_run(cv::multicalib::MultiCameraCalibration* instance) {
		try {
			double ret = instance->run();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(cv::multicalib::MultiCameraCalibration* instance, const char* filename) {
		try {
			instance->writeParameters(std::string(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_getPropCameraVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
		try {
			int ret = instance->cameraVertex;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPropCameraVertex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
		try {
			instance->cameraVertex = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_getPropPhotoVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
		try {
			int ret = instance->photoVertex;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPropPhotoVertex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
		try {
			instance->photoVertex = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_edge_getPropPhotoIndex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
		try {
			int ret = instance->photoIndex;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPropPhotoIndex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
		try {
			instance->photoIndex = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_multicalib_MultiCameraCalibration_edge_getPropTransform(cv::multicalib::MultiCameraCalibration::edge* instance) {
		try {
			cv::Mat ret = instance->transform;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_edge_setPropTransform_Mat(cv::multicalib::MultiCameraCalibration::edge* instance, cv::Mat* val) {
		try {
			instance->transform = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiCameraCalibration_edge_delete(cv::multicalib::MultiCameraCalibration::edge* instance) {
		delete instance;
	}
	Result<cv::multicalib::MultiCameraCalibration::edge*> cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(int cv, int pv, int pi, cv::Mat* trans) {
		try {
			cv::multicalib::MultiCameraCalibration::edge* ret = new cv::multicalib::MultiCameraCalibration::edge(cv, pv, pi, *trans);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::edge*>))
	}
	
	Result<cv::Mat*> cv_multicalib_MultiCameraCalibration_vertex_getPropPose(cv::multicalib::MultiCameraCalibration::vertex* instance) {
		try {
			cv::Mat ret = instance->pose;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_vertex_setPropPose_Mat(cv::multicalib::MultiCameraCalibration::vertex* instance, cv::Mat* val) {
		try {
			instance->pose = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_multicalib_MultiCameraCalibration_vertex_getPropTimestamp_const(const cv::multicalib::MultiCameraCalibration::vertex* instance) {
		try {
			int ret = instance->timestamp;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_multicalib_MultiCameraCalibration_vertex_setPropTimestamp_int(cv::multicalib::MultiCameraCalibration::vertex* instance, int val) {
		try {
			instance->timestamp = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiCameraCalibration_vertex_delete(cv::multicalib::MultiCameraCalibration::vertex* instance) {
		delete instance;
	}
	Result<cv::multicalib::MultiCameraCalibration::vertex*> cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(cv::Mat* po, int ts) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex(*po, ts);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::vertex*>))
	}
	
	Result<cv::multicalib::MultiCameraCalibration::vertex*> cv_multicalib_MultiCameraCalibration_vertex_vertex() {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::vertex*>))
	}
	
	void cv_RandomPatternCornerFinder_delete(cv::randpattern::RandomPatternCornerFinder* instance) {
		delete instance;
	}
	Result<cv::randpattern::RandomPatternCornerFinder*> cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(float patternWidth, float patternHeight, int nminiMatch, int depth, int verbose, int showExtraction, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher) {
		try {
			cv::randpattern::RandomPatternCornerFinder* ret = new cv::randpattern::RandomPatternCornerFinder(patternWidth, patternHeight, nminiMatch, depth, verbose, showExtraction, *detector, *descriptor, *matcher);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::randpattern::RandomPatternCornerFinder*>))
	}
	
	Result_void cv_randpattern_RandomPatternCornerFinder_loadPattern_Mat(cv::randpattern::RandomPatternCornerFinder* instance, cv::Mat* patternImage) {
		try {
			instance->loadPattern(*patternImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vector_Mat_(cv::randpattern::RandomPatternCornerFinder* instance, std::vector<cv::Mat>* inputImages) {
		try {
			instance->computeObjectImagePoints(*inputImages);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(cv::randpattern::RandomPatternCornerFinder* instance, cv::Mat* inputImage) {
		try {
			std::vector<cv::Mat> ret = instance->computeObjectImagePointsForSingle(*inputImage);
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result<std::vector<cv::Mat>*> cv_randpattern_RandomPatternCornerFinder_getObjectPoints(cv::randpattern::RandomPatternCornerFinder* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getObjectPoints();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result<std::vector<cv::Mat>*> cv_randpattern_RandomPatternCornerFinder_getImagePoints(cv::randpattern::RandomPatternCornerFinder* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getImagePoints();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	void cv_RandomPatternGenerator_delete(cv::randpattern::RandomPatternGenerator* instance) {
		delete instance;
	}
	Result<cv::randpattern::RandomPatternGenerator*> cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(int imageWidth, int imageHeight) {
		try {
			cv::randpattern::RandomPatternGenerator* ret = new cv::randpattern::RandomPatternGenerator(imageWidth, imageHeight);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::randpattern::RandomPatternGenerator*>))
	}
	
	Result_void cv_randpattern_RandomPatternGenerator_generatePattern(cv::randpattern::RandomPatternGenerator* instance) {
		try {
			instance->generatePattern();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_randpattern_RandomPatternGenerator_getPattern(cv::randpattern::RandomPatternGenerator* instance) {
		try {
			cv::Mat ret = instance->getPattern();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
}
