#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, const cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, const cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, const cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::detectCharucoDiamond(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
	// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectCharucoDiamond(InputArray, InputArrayOfArrays, InputArray, float, OutputArrayOfArrays, OutputArray, InputArray, InputArray, Ptr<Dictionary>)(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray, InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
	// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds", "cameraMatrix", "distCoeffs", "dictionary"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Ptr<cv::aruco::Dictionary>"]), _)]),
	void cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_PtrLDictionaryG(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Ptr<cv::aruco::Dictionary>* dictionary, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds, *cameraMatrix, *distCoeffs, *dictionary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::detectMarkers(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
	// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMarkers(InputArray, const Ptr<Dictionary> &, OutputArrayOfArrays, OutputArray, const Ptr<DetectorParameters> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
	// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids", "parameters", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR_const_PtrLDetectorParametersGR_const__OutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, const cv::_OutputArray* rejectedImgPoints, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids, *parameters, *rejectedImgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawCharucoDiamond(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
	// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(const cv::Ptr<cv::aruco::Dictionary>* dictionary, cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawCharucoDiamond(const Ptr<Dictionary> &, Vec4i, int, int, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
	// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawPlanarBoard(const Ptr<Board> &, Size, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:45
	// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Board>* board, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawPlanarBoard(*board, *outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
	// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseBoard(InputArrayOfArrays, InputArray, const Ptr<Board> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
	// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
	void cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseCharucoBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
	// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseCharucoBoard(InputArray, InputArray, const Ptr<CharucoBoard> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
	// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
	void cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseSingleMarkers(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
	// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseSingleMarkers(InputArrayOfArrays, float, InputArray, InputArray, OutputArray, OutputArray, OutputArray, const Ptr<EstimateParameters> &)(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
	// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "objPoints", "estimateParameters"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::EstimateParameters>*"]), _)]),
	void cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_PtrLEstimateParametersGR(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* objPoints, const cv::Ptr<cv::aruco::EstimateParameters>* estimateParameters, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *objPoints, *estimateParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBoardObjectAndImagePoints(const Ptr<Board> &, InputArrayOfArrays, InputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:51
	// ("cv::aruco::getBoardObjectAndImagePoints", vec![(pred!(mut, ["board", "detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_getBoardObjectAndImagePoints_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* detectedCorners, const cv::_InputArray* detectedIds, const cv::_OutputArray* objPoints, const cv::_OutputArray* imgPoints, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::getBoardObjectAndImagePoints(*board, *detectedCorners, *detectedIds, *objPoints, *imgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::interpolateCornersCharuco(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
	// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// interpolateCornersCharuco(InputArrayOfArrays, InputArray, InputArray, const Ptr<CharucoBoard> &, OutputArray, OutputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
	// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds", "cameraMatrix", "distCoeffs", "minMarkers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, int minMarkers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds, *cameraMatrix, *distCoeffs, minMarkers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::refineDetectedMarkers(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
	// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// refineDetectedMarkers(InputArray, const Ptr<Board> &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, float, float, bool, OutputArray, const Ptr<DetectorParameters> &)(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
	// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "minRepDistance", "errorCorrectionRate", "checkAllOrders", "recoveredIdxs", "parameters"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "float", "bool", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*"]), _)]),
	void cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_PtrLDetectorParametersGR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, float minRepDistance, float errorCorrectionRate, bool checkAllOrders, const cv::_OutputArray* recoveredIdxs, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, minRepDistance, errorCorrectionRate, checkAllOrders, *recoveredIdxs, *parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testCharucoCornersCollinear(const Ptr<CharucoBoard> &, InputArray)(CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:96
	// ("cv::aruco::testCharucoCornersCollinear", vec![(pred!(mut, ["board", "charucoIds"], ["const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* charucoIds, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::testCharucoCornersCollinear(*board, *charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// EstimateParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:60
	// ("cv::aruco::EstimateParameters::EstimateParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_EstimateParameters_EstimateParameters(Result<cv::aruco::EstimateParameters*>* ocvrs_return) {
		try {
			cv::aruco::EstimateParameters* ret = new cv::aruco::EstimateParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::EstimateParameters::implicitClone() generated
	// ("cv::aruco::EstimateParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::EstimateParameters* cv_aruco_EstimateParameters_implicitClone_const(const cv::aruco::EstimateParameters* instance) {
			return new cv::aruco::EstimateParameters(*instance);
	}

	// cv::aruco::EstimateParameters::pattern() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
	// ("cv::aruco::EstimateParameters::pattern", vec![(pred!(const, [], []), _)]),
	void cv_aruco_EstimateParameters_propPattern_const(const cv::aruco::EstimateParameters* instance, cv::aruco::PatternPositionType* ocvrs_return) {
			cv::aruco::PatternPositionType ret = instance->pattern;
			*ocvrs_return = ret;
	}

	// cv::aruco::EstimateParameters::setPattern(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
	// ("cv::aruco::EstimateParameters::setPattern", vec![(pred!(mut, ["val"], ["const cv::aruco::PatternPositionType"]), _)]),
	void cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(cv::aruco::EstimateParameters* instance, const cv::aruco::PatternPositionType val) {
			instance->pattern = val;
	}

	// cv::aruco::EstimateParameters::useExtrinsicGuess() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
	// ("cv::aruco::EstimateParameters::useExtrinsicGuess", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(const cv::aruco::EstimateParameters* instance) {
			bool ret = instance->useExtrinsicGuess;
			return ret;
	}

	// cv::aruco::EstimateParameters::setUseExtrinsicGuess(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
	// ("cv::aruco::EstimateParameters::setUseExtrinsicGuess", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(cv::aruco::EstimateParameters* instance, const bool val) {
			instance->useExtrinsicGuess = val;
	}

	// cv::aruco::EstimateParameters::solvePnPMethod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
	// ("cv::aruco::EstimateParameters::solvePnPMethod", vec![(pred!(const, [], []), _)]),
	int cv_aruco_EstimateParameters_propSolvePnPMethod_const(const cv::aruco::EstimateParameters* instance) {
			int ret = instance->solvePnPMethod;
			return ret;
	}

	// cv::aruco::EstimateParameters::setSolvePnPMethod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
	// ("cv::aruco::EstimateParameters::setSolvePnPMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(cv::aruco::EstimateParameters* instance, const int val) {
			instance->solvePnPMethod = val;
	}

	// cv::aruco::EstimateParameters::delete() generated
	// ("cv::aruco::EstimateParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_EstimateParameters_delete(cv::aruco::EstimateParameters* instance) {
			delete instance;
	}

}
