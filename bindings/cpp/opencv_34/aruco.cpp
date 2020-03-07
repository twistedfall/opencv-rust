#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	Result<double> cv_aruco_calibrateCameraAruco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* corners, void* ids, void* counter, void* board, const cv::Size* imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, void* stdDeviationsIntrinsics, void* stdDeviationsExtrinsics, void* perViewErrors, int flags, void* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*reinterpret_cast<const cv::_InputArray*>(corners), *reinterpret_cast<const cv::_InputArray*>(ids), *reinterpret_cast<const cv::_InputArray*>(counter), *reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsIntrinsics), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsExtrinsics), *reinterpret_cast<const cv::_OutputArray*>(perViewErrors), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_aruco_calibrateCameraAruco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* corners, void* ids, void* counter, void* board, const cv::Size* imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, int flags, void* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*reinterpret_cast<const cv::_InputArray*>(corners), *reinterpret_cast<const cv::_InputArray*>(ids), *reinterpret_cast<const cv::_InputArray*>(counter), *reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_aruco_calibrateCameraCharuco_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* charucoCorners, void* charucoIds, void* board, const cv::Size* imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, void* stdDeviationsIntrinsics, void* stdDeviationsExtrinsics, void* perViewErrors, int flags, void* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*reinterpret_cast<const cv::_InputArray*>(charucoCorners), *reinterpret_cast<const cv::_InputArray*>(charucoIds), *reinterpret_cast<const cv::Ptr<cv::aruco::CharucoBoard>*>(board), *imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsIntrinsics), *reinterpret_cast<const cv::_OutputArray*>(stdDeviationsExtrinsics), *reinterpret_cast<const cv::_OutputArray*>(perViewErrors), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_aruco_calibrateCameraCharuco_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_Size_const__InputOutputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_TermCriteria(void* charucoCorners, void* charucoIds, void* board, const cv::Size* imageSize, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, int flags, void* criteria) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*reinterpret_cast<const cv::_InputArray*>(charucoCorners), *reinterpret_cast<const cv::_InputArray*>(charucoIds), *reinterpret_cast<const cv::Ptr<cv::aruco::CharucoBoard>*>(board), *imageSize, *reinterpret_cast<const cv::_InputOutputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputOutputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), flags, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_detectCharucoDiamond_const__InputArrayX_const__InputArrayX_const__InputArrayX_float_const__OutputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* image, void* markerCorners, void* markerIds, float squareMarkerLengthRate, void* diamondCorners, void* diamondIds, void* cameraMatrix, void* distCoeffs) {
		try {
			cv::aruco::detectCharucoDiamond(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(markerCorners), *reinterpret_cast<const cv::_InputArray*>(markerIds), squareMarkerLengthRate, *reinterpret_cast<const cv::_OutputArray*>(diamondCorners), *reinterpret_cast<const cv::_OutputArray*>(diamondIds), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_detectMarkers_const__InputArrayX_const_Ptr_Dictionary_X_const__OutputArrayX_const__OutputArrayX_const_Ptr_DetectorParameters_X_const__OutputArrayX_const__InputArrayX_const__InputArrayX(void* image, void* dictionary, void* corners, void* ids, void* parameters, void* rejectedImgPoints, void* cameraMatrix, void* distCoeff) {
		try {
			cv::aruco::detectMarkers(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary), *reinterpret_cast<const cv::_OutputArray*>(corners), *reinterpret_cast<const cv::_OutputArray*>(ids), *reinterpret_cast<const cv::Ptr<cv::aruco::DetectorParameters>*>(parameters), *reinterpret_cast<const cv::_OutputArray*>(rejectedImgPoints), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeff));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawAxis_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_float(void* image, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, float length) {
		try {
			cv::aruco::drawAxis(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_InputArray*>(rvec), *reinterpret_cast<const cv::_InputArray*>(tvec), length);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_X_Vec4i_int_int_const__OutputArrayX_int_int(void* dictionary, const cv::Vec4i* ids, int squareLength, int markerLength, void* img, int marginSize, int borderBits) {
		try {
			cv::aruco::drawCharucoDiamond(*reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary), *ids, squareLength, markerLength, *reinterpret_cast<const cv::_OutputArray*>(img), marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(void* image, void* charucoCorners, void* charucoIds, const cv::Scalar* cornerColor) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(charucoCorners), *reinterpret_cast<const cv::_InputArray*>(charucoIds), *cornerColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(void* image, void* diamondCorners, void* diamondIds, const cv::Scalar* borderColor) {
		try {
			cv::aruco::drawDetectedDiamonds(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(diamondCorners), *reinterpret_cast<const cv::_InputArray*>(diamondIds), *borderColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawDetectedMarkers_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_Scalar(void* image, void* corners, void* ids, const cv::Scalar* borderColor) {
		try {
			cv::aruco::drawDetectedMarkers(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(corners), *reinterpret_cast<const cv::_InputArray*>(ids), *borderColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawMarker_const_Ptr_Dictionary_X_int_int_const__OutputArrayX_int(void* dictionary, int id, int sidePixels, void* img, int borderBits) {
		try {
			cv::aruco::drawMarker(*reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary), id, sidePixels, *reinterpret_cast<const cv::_OutputArray*>(img), borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_aruco_drawPlanarBoard_const_Ptr_Board_X_Size_const__OutputArrayX_int_int(void* board, const cv::Size* outSize, void* img, int marginSize, int borderBits) {
		try {
			cv::aruco::drawPlanarBoard(*reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *outSize, *reinterpret_cast<const cv::_OutputArray*>(img), marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_estimatePoseBoard_const__InputArrayX_const__InputArrayX_const_Ptr_Board_X_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool(void* corners, void* ids, void* board, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*reinterpret_cast<const cv::_InputArray*>(corners), *reinterpret_cast<const cv::_InputArray*>(ids), *reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvec), *reinterpret_cast<const cv::_OutputArray*>(tvec), useExtrinsicGuess);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_aruco_estimatePoseCharucoBoard_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool(void* charucoCorners, void* charucoIds, void* board, void* cameraMatrix, void* distCoeffs, void* rvec, void* tvec, bool useExtrinsicGuess) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*reinterpret_cast<const cv::_InputArray*>(charucoCorners), *reinterpret_cast<const cv::_InputArray*>(charucoIds), *reinterpret_cast<const cv::Ptr<cv::aruco::CharucoBoard>*>(board), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvec), *reinterpret_cast<const cv::_OutputArray*>(tvec), useExtrinsicGuess);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_aruco_estimatePoseSingleMarkers_const__InputArrayX_float_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* corners, float markerLength, void* cameraMatrix, void* distCoeffs, void* rvecs, void* tvecs, void* _objPoints) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*reinterpret_cast<const cv::_InputArray*>(corners), markerLength, *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), *reinterpret_cast<const cv::_OutputArray*>(rvecs), *reinterpret_cast<const cv::_OutputArray*>(tvecs), *reinterpret_cast<const cv::_OutputArray*>(_objPoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_X_int(int nMarkers, int markerSize, void* baseDictionary, int randomSeed) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(baseDictionary), randomSeed);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_generateCustomDictionary_int_int_int(int nMarkers, int markerSize, int randomSeed) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, randomSeed);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_getBoardObjectAndImagePoints_const_Ptr_Board_X_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* board, void* detectedCorners, void* detectedIds, void* objPoints, void* imgPoints) {
		try {
			cv::aruco::getBoardObjectAndImagePoints(*reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *reinterpret_cast<const cv::_InputArray*>(detectedCorners), *reinterpret_cast<const cv::_InputArray*>(detectedIds), *reinterpret_cast<const cv::_OutputArray*>(objPoints), *reinterpret_cast<const cv::_OutputArray*>(imgPoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(cv::aruco::PREDEFINED_DICTIONARY_NAME name) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(name);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_getPredefinedDictionary_int(int dict) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(dict);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_aruco_interpolateCornersCharuco_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_Ptr_CharucoBoard_X_const__OutputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int(void* markerCorners, void* markerIds, void* image, void* board, void* charucoCorners, void* charucoIds, void* cameraMatrix, void* distCoeffs, int minMarkers) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*reinterpret_cast<const cv::_InputArray*>(markerCorners), *reinterpret_cast<const cv::_InputArray*>(markerIds), *reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::Ptr<cv::aruco::CharucoBoard>*>(board), *reinterpret_cast<const cv::_OutputArray*>(charucoCorners), *reinterpret_cast<const cv::_OutputArray*>(charucoIds), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), minMarkers);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_refineDetectedMarkers_const__InputArrayX_const_Ptr_Board_X_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_float_float_bool_const__OutputArrayX_const_Ptr_DetectorParameters_X(void* image, void* board, void* detectedCorners, void* detectedIds, void* rejectedCorners, void* cameraMatrix, void* distCoeffs, float minRepDistance, float errorCorrectionRate, bool checkAllOrders, void* recoveredIdxs, void* parameters) {
		try {
			cv::aruco::refineDetectedMarkers(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::Ptr<cv::aruco::Board>*>(board), *reinterpret_cast<const cv::_InputOutputArray*>(detectedCorners), *reinterpret_cast<const cv::_InputOutputArray*>(detectedIds), *reinterpret_cast<const cv::_InputOutputArray*>(rejectedCorners), *reinterpret_cast<const cv::_InputArray*>(cameraMatrix), *reinterpret_cast<const cv::_InputArray*>(distCoeffs), minRepDistance, errorCorrectionRate, checkAllOrders, *reinterpret_cast<const cv::_OutputArray*>(recoveredIdxs), *reinterpret_cast<const cv::Ptr<cv::aruco::DetectorParameters>*>(parameters));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_Board_objPoints(void* instance) {
		try {
			std::vector<std::vector<cv::Point3f>> ret = reinterpret_cast<cv::aruco::Board*>(instance)->objPoints;
			return Ok<void*>(new std::vector<std::vector<cv::Point3f>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_Board_setObjPoints_vector_vector_Point3f__(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::Board*>(instance)->objPoints = *reinterpret_cast<std::vector<std::vector<cv::Point3f>>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_Board_dictionary(void* instance) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = reinterpret_cast<cv::aruco::Board*>(instance)->dictionary;
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_Board_setDictionary_Ptr_Dictionary_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::Board*>(instance)->dictionary = *reinterpret_cast<cv::Ptr<cv::aruco::Dictionary>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_Board_ids(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::aruco::Board*>(instance)->ids;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_Board_setIds_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::Board*>(instance)->ids = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Board_delete(cv::aruco::Board* instance) {
		delete instance;
	}
	Result<void*> cv_aruco_Board_create_const__InputArrayX_const_Ptr_Dictionary_X_const__InputArrayX(void* objPoints, void* dictionary, void* ids) {
		try {
			cv::Ptr<cv::aruco::Board> ret = cv::aruco::Board::create(*reinterpret_cast<const cv::_InputArray*>(objPoints), *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary), *reinterpret_cast<const cv::_InputArray*>(ids));
			return Ok<void*>(new cv::Ptr<cv::aruco::Board>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_CharucoBoard_chessboardCorners(void* instance) {
		try {
			std::vector<cv::Point3f> ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->chessboardCorners;
			return Ok<void*>(new std::vector<cv::Point3f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_CharucoBoard_setChessboardCorners_vector_Point3f_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->chessboardCorners = *reinterpret_cast<std::vector<cv::Point3f>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_CharucoBoard_nearestMarkerIdx(void* instance) {
		try {
			std::vector<std::vector<int>> ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->nearestMarkerIdx;
			return Ok<void*>(new std::vector<std::vector<int>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_CharucoBoard_setNearestMarkerIdx_vector_vector_int__(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->nearestMarkerIdx = *reinterpret_cast<std::vector<std::vector<int>>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_CharucoBoard_nearestMarkerCorners(void* instance) {
		try {
			std::vector<std::vector<int>> ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->nearestMarkerCorners;
			return Ok<void*>(new std::vector<std::vector<int>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_CharucoBoard_setNearestMarkerCorners_vector_vector_int__(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->nearestMarkerCorners = *reinterpret_cast<std::vector<std::vector<int>>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CharucoBoard_delete(cv::aruco::CharucoBoard* instance) {
		delete instance;
	}
	Result_void cv_aruco_CharucoBoard_draw_Size_const__OutputArrayX_int_int(void* instance, const cv::Size* outSize, void* img, int marginSize, int borderBits) {
		try {
			reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->draw(*outSize, *reinterpret_cast<const cv::_OutputArray*>(img), marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_X(int squaresX, int squaresY, float squareLength, float markerLength, void* dictionary) {
		try {
			cv::Ptr<cv::aruco::CharucoBoard> ret = cv::aruco::CharucoBoard::create(squaresX, squaresY, squareLength, markerLength, *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary));
			return Ok<void*>(new cv::Ptr<cv::aruco::CharucoBoard>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_aruco_CharucoBoard_getChessboardSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->getChessboardSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<float> cv_aruco_CharucoBoard_getSquareLength_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->getSquareLength();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_aruco_CharucoBoard_getMarkerLength_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::CharucoBoard*>(instance)->getMarkerLength();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<int> cv_aruco_DetectorParameters_adaptiveThreshWinSizeMin_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeMin;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAdaptiveThreshWinSizeMin_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeMin = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_adaptiveThreshWinSizeMax_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeMax;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAdaptiveThreshWinSizeMax_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeMax = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_adaptiveThreshWinSizeStep_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeStep;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAdaptiveThreshWinSizeStep_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshWinSizeStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_adaptiveThreshConstant_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshConstant;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAdaptiveThreshConstant_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->adaptiveThreshConstant = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_minMarkerPerimeterRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minMarkerPerimeterRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMinMarkerPerimeterRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minMarkerPerimeterRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_maxMarkerPerimeterRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->maxMarkerPerimeterRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMaxMarkerPerimeterRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->maxMarkerPerimeterRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_polygonalApproxAccuracyRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->polygonalApproxAccuracyRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setPolygonalApproxAccuracyRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->polygonalApproxAccuracyRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_minCornerDistanceRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minCornerDistanceRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMinCornerDistanceRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minCornerDistanceRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_minDistanceToBorder_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minDistanceToBorder;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMinDistanceToBorder_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minDistanceToBorder = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_minMarkerDistanceRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minMarkerDistanceRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMinMarkerDistanceRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minMarkerDistanceRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_cornerRefinementMethod_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMethod;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setCornerRefinementMethod_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMethod = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_cornerRefinementWinSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementWinSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setCornerRefinementWinSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementWinSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_cornerRefinementMaxIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMaxIterations;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setCornerRefinementMaxIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMaxIterations = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_cornerRefinementMinAccuracy_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMinAccuracy;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setCornerRefinementMinAccuracy_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->cornerRefinementMinAccuracy = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_markerBorderBits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->markerBorderBits;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMarkerBorderBits_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->markerBorderBits = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_perspectiveRemovePixelPerCell_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->perspectiveRemovePixelPerCell;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setPerspectiveRemovePixelPerCell_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->perspectiveRemovePixelPerCell = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_perspectiveRemoveIgnoredMarginPerCell_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->perspectiveRemoveIgnoredMarginPerCell;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setPerspectiveRemoveIgnoredMarginPerCell_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->perspectiveRemoveIgnoredMarginPerCell = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_maxErroneousBitsInBorderRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->maxErroneousBitsInBorderRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMaxErroneousBitsInBorderRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->maxErroneousBitsInBorderRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_minOtsuStdDev_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minOtsuStdDev;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setMinOtsuStdDev_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->minOtsuStdDev = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_aruco_DetectorParameters_errorCorrectionRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->errorCorrectionRate;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_aruco_DetectorParameters_setErrorCorrectionRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->errorCorrectionRate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_aruco_DetectorParameters_aprilTagQuadDecimate_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagQuadDecimate;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagQuadDecimate_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagQuadDecimate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_aruco_DetectorParameters_aprilTagQuadSigma_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagQuadSigma;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagQuadSigma_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagQuadSigma = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_aprilTagMinClusterPixels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMinClusterPixels;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagMinClusterPixels_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMinClusterPixels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_aprilTagMaxNmaxima_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMaxNmaxima;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagMaxNmaxima_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMaxNmaxima = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_aruco_DetectorParameters_aprilTagCriticalRad_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagCriticalRad;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagCriticalRad_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagCriticalRad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_aruco_DetectorParameters_aprilTagMaxLineFitMse_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMaxLineFitMse;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagMaxLineFitMse_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMaxLineFitMse = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_aprilTagMinWhiteBlackDiff_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMinWhiteBlackDiff;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagMinWhiteBlackDiff_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagMinWhiteBlackDiff = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_DetectorParameters_aprilTagDeglitch_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagDeglitch;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_DetectorParameters_setAprilTagDeglitch_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->aprilTagDeglitch = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_aruco_DetectorParameters_detectInvertedMarker_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->detectInvertedMarker;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_aruco_DetectorParameters_setDetectInvertedMarker_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::aruco::DetectorParameters*>(instance)->detectInvertedMarker = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
		delete instance;
	}
	Result<void*> cv_aruco_DetectorParameters_DetectorParameters() {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_DetectorParameters_create() {
		try {
			cv::Ptr<cv::aruco::DetectorParameters> ret = cv::aruco::DetectorParameters::create();
			return Ok<void*>(new cv::Ptr<cv::aruco::DetectorParameters>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_bytesList(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::aruco::Dictionary*>(instance)->bytesList;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_aruco_Dictionary_setBytesList_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::aruco::Dictionary*>(instance)->bytesList = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_Dictionary_markerSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::Dictionary*>(instance)->markerSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_Dictionary_setMarkerSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::Dictionary*>(instance)->markerSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_aruco_Dictionary_maxCorrectionBits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::aruco::Dictionary*>(instance)->maxCorrectionBits;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_Dictionary_setMaxCorrectionBits_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::aruco::Dictionary*>(instance)->maxCorrectionBits = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Dictionary_delete(cv::aruco::Dictionary* instance) {
		delete instance;
	}
	Result<void*> cv_aruco_Dictionary_Dictionary_const_MatX_int_int(void* _bytesList, int _markerSize, int _maxcorr) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*reinterpret_cast<const cv::Mat*>(_bytesList), _markerSize, _maxcorr);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_X(void* _dictionary) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(_dictionary));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_create_int_int_int(int nMarkers, int markerSize, int randomSeed) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, randomSeed);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_X_int(int nMarkers, int markerSize, void* baseDictionary, int randomSeed) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(baseDictionary), randomSeed);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_get_int(int dict) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::get(dict);
			return Ok<void*>(new cv::Ptr<cv::aruco::Dictionary>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_aruco_Dictionary_identify_const_const_MatX_intX_intX_double(void* instance, void* onlyBits, int* idx, int* rotation, double maxCorrectionRate) {
		try {
			bool ret = reinterpret_cast<cv::aruco::Dictionary*>(instance)->identify(*reinterpret_cast<const cv::Mat*>(onlyBits), *idx, *rotation, maxCorrectionRate);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayX_int_bool(void* instance, void* bits, int id, bool allRotations) {
		try {
			int ret = reinterpret_cast<cv::aruco::Dictionary*>(instance)->getDistanceToId(*reinterpret_cast<const cv::_InputArray*>(bits), id, allRotations);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayX_int(void* instance, int id, int sidePixels, void* _img, int borderBits) {
		try {
			reinterpret_cast<cv::aruco::Dictionary*>(instance)->drawMarker(id, sidePixels, *reinterpret_cast<const cv::_OutputArray*>(_img), borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_Dictionary_getByteListFromBits_const_MatX(void* bits) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*reinterpret_cast<const cv::Mat*>(bits));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_aruco_Dictionary_getBitsFromByteList_const_MatX_int(void* byteList, int markerSize) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getBitsFromByteList(*reinterpret_cast<const cv::Mat*>(byteList), markerSize);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_GridBoard_delete(cv::aruco::GridBoard* instance) {
		delete instance;
	}
	Result_void cv_aruco_GridBoard_draw_Size_const__OutputArrayX_int_int(void* instance, const cv::Size* outSize, void* img, int marginSize, int borderBits) {
		try {
			reinterpret_cast<cv::aruco::GridBoard*>(instance)->draw(*outSize, *reinterpret_cast<const cv::_OutputArray*>(img), marginSize, borderBits);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_X_int(int markersX, int markersY, float markerLength, float markerSeparation, void* dictionary, int firstMarker) {
		try {
			cv::Ptr<cv::aruco::GridBoard> ret = cv::aruco::GridBoard::create(markersX, markersY, markerLength, markerSeparation, *reinterpret_cast<const cv::Ptr<cv::aruco::Dictionary>*>(dictionary), firstMarker);
			return Ok<void*>(new cv::Ptr<cv::aruco::GridBoard>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_aruco_GridBoard_getGridSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::aruco::GridBoard*>(instance)->getGridSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<float> cv_aruco_GridBoard_getMarkerLength_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::GridBoard*>(instance)->getMarkerLength();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_aruco_GridBoard_getMarkerSeparation_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::aruco::GridBoard*>(instance)->getMarkerSeparation();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
}
