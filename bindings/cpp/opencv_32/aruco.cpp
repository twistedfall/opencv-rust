#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	Result<double> cv_aruco_calibrateCameraAruco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, const cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, const cv::TermCriteria* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_aruco_calibrateCameraAruco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, const cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_aruco_calibrateCameraCharuco_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, const cv::TermCriteria* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_aruco_calibrateCameraCharuco_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_detectCharucoDiamond_const__InputArrayX_const__InputArrayX_const__InputArrayX_float_const__OutputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds, *cameraMatrix, *distCoeffs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_detectMarkers_const__InputArrayX_const_Ptr_Dictionary_X_const__OutputArrayX_const__OutputArrayX_const_Ptr_DetectorParameters_X_const__OutputArrayX(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, const cv::_OutputArray* rejectedImgPoints) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids, *parameters, *rejectedImgPoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawAxis_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_float(const cv::_InputOutputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputArray* rvec, const cv::_InputArray* tvec, float length) {
		try {
			cv::aruco::drawAxis(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, length);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_X_Vec4i_int_int_const__OutputArrayX_int_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, int marginSize, int borderBits) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img, marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Scalar* cornerColor) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners, *charucoIds, *cornerColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, const cv::_InputArray* diamondIds, const cv::Scalar* borderColor) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners, *diamondIds, *borderColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawDetectedMarkers_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Scalar* borderColor) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners, *ids, *borderColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawMarker_const_Ptr_Dictionary_X_int_int_const__OutputArrayX_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, int id, int sidePixels, const cv::_OutputArray* img, int borderBits) {
		try {
			cv::aruco::drawMarker(*dictionary, id, sidePixels, *img, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_aruco_drawPlanarBoard_const_Ptr_Board_X_Size_const__OutputArrayX_int_int(const cv::Ptr<cv::aruco::Board>* board, const cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits) {
		try {
			cv::aruco::drawPlanarBoard(*board, *outSize, *img, marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_estimatePoseBoard_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_aruco_estimatePoseCharucoBoard_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_aruco_estimatePoseSingleMarkers_const__InputArrayX_float_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_generateCustomDictionary_int_int(int nMarkers, int markerSize) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_X(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, *baseDictionary);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(cv::aruco::PREDEFINED_DICTIONARY_NAME name) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(name);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_getPredefinedDictionary_int(int dict) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(dict);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<int> cv_aruco_interpolateCornersCharuco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_const__OutputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, int minMarkers) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds, *cameraMatrix, *distCoeffs, minMarkers);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_refineDetectedMarkers_const__InputArrayX_const_Ptr_Board_X_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_float_float_bool_const__OutputArrayX_const_Ptr_DetectorParameters_X(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, float minRepDistance, float errorCorrectionRate, bool checkAllOrders, const cv::_OutputArray* recoveredIdxs, const cv::Ptr<cv::aruco::DetectorParameters>* parameters) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, minRepDistance, errorCorrectionRate, checkAllOrders, *recoveredIdxs, *parameters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<std::vector<cv::Point3f>>*> cv_aruco_Board_getPropObjPoints(cv::aruco::Board* instance) {
		try {
			std::vector<std::vector<cv::Point3f>> ret = instance->objPoints;
			return Ok(new std::vector<std::vector<cv::Point3f>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<cv::Point3f>>*>))
	}
	
	Result_void cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(cv::aruco::Board* instance, std::vector<std::vector<cv::Point3f>>* val) {
		try {
			instance->objPoints = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_Board_getPropDictionary(cv::aruco::Board* instance) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = instance->dictionary;
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result_void cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(cv::aruco::Board* instance, cv::Ptr<cv::aruco::Dictionary>* val) {
		try {
			instance->dictionary = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_aruco_Board_getPropIds(cv::aruco::Board* instance) {
		try {
			std::vector<int> ret = instance->ids;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_aruco_Board_setPropIds_vector_int_(cv::aruco::Board* instance, std::vector<int>* val) {
		try {
			instance->ids = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Board_delete(cv::aruco::Board* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::aruco::Board>*> cv_aruco_Board_create_const__InputArrayX_const_Ptr_Dictionary_X_const__InputArrayX(const cv::_InputArray* objPoints, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_InputArray* ids) {
		try {
			cv::Ptr<cv::aruco::Board> ret = cv::aruco::Board::create(*objPoints, *dictionary, *ids);
			return Ok(new cv::Ptr<cv::aruco::Board>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Board>*>))
	}
	
	Result<std::vector<cv::Point3f>*> cv_aruco_CharucoBoard_getPropChessboardCorners(cv::aruco::CharucoBoard* instance) {
		try {
			std::vector<cv::Point3f> ret = instance->chessboardCorners;
			return Ok(new std::vector<cv::Point3f>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point3f>*>))
	}
	
	Result_void cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(cv::aruco::CharucoBoard* instance, std::vector<cv::Point3f>* val) {
		try {
			instance->chessboardCorners = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<std::vector<int>>*> cv_aruco_CharucoBoard_getPropNearestMarkerIdx(cv::aruco::CharucoBoard* instance) {
		try {
			std::vector<std::vector<int>> ret = instance->nearestMarkerIdx;
			return Ok(new std::vector<std::vector<int>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<int>>*>))
	}
	
	Result_void cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(cv::aruco::CharucoBoard* instance, std::vector<std::vector<int>>* val) {
		try {
			instance->nearestMarkerIdx = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<std::vector<int>>*> cv_aruco_CharucoBoard_getPropNearestMarkerCorners(cv::aruco::CharucoBoard* instance) {
		try {
			std::vector<std::vector<int>> ret = instance->nearestMarkerCorners;
			return Ok(new std::vector<std::vector<int>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<int>>*>))
	}
	
	Result_void cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(cv::aruco::CharucoBoard* instance, std::vector<std::vector<int>>* val) {
		try {
			instance->nearestMarkerCorners = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CharucoBoard_delete(cv::aruco::CharucoBoard* instance) {
		delete instance;
	}
	Result_void cv_aruco_CharucoBoard_draw_Size_const__OutputArrayX_int_int(cv::aruco::CharucoBoard* instance, const cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::aruco::CharucoBoard>*> cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_X(int squaresX, int squaresY, float squareLength, float markerLength, const cv::Ptr<cv::aruco::Dictionary>* dictionary) {
		try {
			cv::Ptr<cv::aruco::CharucoBoard> ret = cv::aruco::CharucoBoard::create(squaresX, squaresY, squareLength, markerLength, *dictionary);
			return Ok(new cv::Ptr<cv::aruco::CharucoBoard>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::CharucoBoard>*>))
	}
	
	Result<cv::Size> cv_aruco_CharucoBoard_getChessboardSize_const(const cv::aruco::CharucoBoard* instance) {
		try {
			cv::Size ret = instance->getChessboardSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<float> cv_aruco_CharucoBoard_getSquareLength_const(const cv::aruco::CharucoBoard* instance) {
		try {
			float ret = instance->getSquareLength();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_aruco_CharucoBoard_getMarkerLength_const(const cv::aruco::CharucoBoard* instance) {
		try {
			float ret = instance->getMarkerLength();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->adaptiveThreshWinSizeMin;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->adaptiveThreshWinSizeMin = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->adaptiveThreshWinSizeMax;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->adaptiveThreshWinSizeMax = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->adaptiveThreshWinSizeStep;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->adaptiveThreshWinSizeStep = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->adaptiveThreshConstant;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->adaptiveThreshConstant = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->minMarkerPerimeterRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->minMarkerPerimeterRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->maxMarkerPerimeterRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->maxMarkerPerimeterRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->polygonalApproxAccuracyRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->polygonalApproxAccuracyRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->minCornerDistanceRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->minCornerDistanceRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->minDistanceToBorder;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->minDistanceToBorder = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->minMarkerDistanceRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->minMarkerDistanceRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_aruco_DetectorParameters_getPropDoCornerRefinement_const(const cv::aruco::DetectorParameters* instance) {
		try {
			bool ret = instance->doCornerRefinement;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropDoCornerRefinement_bool(cv::aruco::DetectorParameters* instance, bool val) {
		try {
			instance->doCornerRefinement = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->cornerRefinementWinSize;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->cornerRefinementWinSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->cornerRefinementMaxIterations;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->cornerRefinementMaxIterations = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->cornerRefinementMinAccuracy;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->cornerRefinementMinAccuracy = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->markerBorderBits;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->markerBorderBits = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(const cv::aruco::DetectorParameters* instance) {
		try {
			int ret = instance->perspectiveRemovePixelPerCell;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(cv::aruco::DetectorParameters* instance, int val) {
		try {
			instance->perspectiveRemovePixelPerCell = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->perspectiveRemoveIgnoredMarginPerCell;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->perspectiveRemoveIgnoredMarginPerCell = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->maxErroneousBitsInBorderRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->maxErroneousBitsInBorderRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->minOtsuStdDev;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->minOtsuStdDev = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(const cv::aruco::DetectorParameters* instance) {
		try {
			double ret = instance->errorCorrectionRate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(cv::aruco::DetectorParameters* instance, double val) {
		try {
			instance->errorCorrectionRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
		delete instance;
	}
	Result<cv::aruco::DetectorParameters*> cv_aruco_DetectorParameters_DetectorParameters() {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::DetectorParameters*>))
	}
	
	Result<cv::Ptr<cv::aruco::DetectorParameters>*> cv_aruco_DetectorParameters_create() {
		try {
			cv::Ptr<cv::aruco::DetectorParameters> ret = cv::aruco::DetectorParameters::create();
			return Ok(new cv::Ptr<cv::aruco::DetectorParameters>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::DetectorParameters>*>))
	}
	
	Result<cv::Mat*> cv_aruco_Dictionary_getPropBytesList(cv::aruco::Dictionary* instance) {
		try {
			cv::Mat ret = instance->bytesList;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_aruco_Dictionary_setPropBytesList_Mat(cv::aruco::Dictionary* instance, cv::Mat* val) {
		try {
			instance->bytesList = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_Dictionary_getPropMarkerSize_const(const cv::aruco::Dictionary* instance) {
		try {
			int ret = instance->markerSize;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_Dictionary_setPropMarkerSize_int(cv::aruco::Dictionary* instance, int val) {
		try {
			instance->markerSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_aruco_Dictionary_getPropMaxCorrectionBits_const(const cv::aruco::Dictionary* instance) {
		try {
			int ret = instance->maxCorrectionBits;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_Dictionary_setPropMaxCorrectionBits_int(cv::aruco::Dictionary* instance, int val) {
		try {
			instance->maxCorrectionBits = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Dictionary_delete(cv::aruco::Dictionary* instance) {
		delete instance;
	}
	Result<cv::aruco::Dictionary*> cv_aruco_Dictionary_Dictionary_const_MatX_int_int(const cv::Mat* _bytesList, int _markerSize, int _maxcorr) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_bytesList, _markerSize, _maxcorr);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::Dictionary*>))
	}
	
	Result<cv::aruco::Dictionary*> cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_X(const cv::Ptr<cv::aruco::Dictionary>* _dictionary) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_dictionary);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::Dictionary*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_Dictionary_create_int_int(int nMarkers, int markerSize) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_X(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, *baseDictionary);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<cv::Ptr<cv::aruco::Dictionary>*> cv_aruco_Dictionary_get_int(int dict) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::get(dict);
			return Ok(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	Result<bool> cv_aruco_Dictionary_identify_const_const_MatX_intR_intR_double(const cv::aruco::Dictionary* instance, const cv::Mat* onlyBits, int* idx, int* rotation, double maxCorrectionRate) {
		try {
			bool ret = instance->identify(*onlyBits, *idx, *rotation, maxCorrectionRate);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayX_int_bool(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, bool allRotations) {
		try {
			int ret = instance->getDistanceToId(*bits, id, allRotations);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayX_int(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, int borderBits) {
		try {
			instance->drawMarker(id, sidePixels, *_img, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_aruco_Dictionary_getByteListFromBits_const_MatX(const cv::Mat* bits) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*bits);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_aruco_Dictionary_getBitsFromByteList_const_MatX_int(const cv::Mat* byteList, int markerSize) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getBitsFromByteList(*byteList, markerSize);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_GridBoard_delete(cv::aruco::GridBoard* instance) {
		delete instance;
	}
	Result_void cv_aruco_GridBoard_draw_Size_const__OutputArrayX_int_int(cv::aruco::GridBoard* instance, const cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::aruco::GridBoard>*> cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_X_int(int markersX, int markersY, float markerLength, float markerSeparation, const cv::Ptr<cv::aruco::Dictionary>* dictionary, int firstMarker) {
		try {
			cv::Ptr<cv::aruco::GridBoard> ret = cv::aruco::GridBoard::create(markersX, markersY, markerLength, markerSeparation, *dictionary, firstMarker);
			return Ok(new cv::Ptr<cv::aruco::GridBoard>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::GridBoard>*>))
	}
	
	Result<cv::Size> cv_aruco_GridBoard_getGridSize_const(const cv::aruco::GridBoard* instance) {
		try {
			cv::Size ret = instance->getGridSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<float> cv_aruco_GridBoard_getMarkerLength_const(const cv::aruco::GridBoard* instance) {
		try {
			float ret = instance->getMarkerLength();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_aruco_GridBoard_getMarkerSeparation_const(const cv::aruco::GridBoard* instance) {
		try {
			float ret = instance->getMarkerSeparation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
}
