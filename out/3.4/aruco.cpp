#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:635
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:624
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:624
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:635
	// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:255
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:245
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:245
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:255
	// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::detectCharucoDiamond(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:286
	// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectCharucoDiamond(InputArray, InputArrayOfArrays, InputArray, float, OutputArrayOfArrays, OutputArray, InputArray, InputArray, Ptr<Dictionary>)(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray, InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:286
	// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds", "cameraMatrix", "distCoeffs", "dictionary"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Ptr<cv::aruco::Dictionary>"]), _)]),
	void cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_PtrLDictionaryG(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Ptr<cv::aruco::Dictionary>* dictionary, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds, *cameraMatrix, *distCoeffs, *dictionary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::detectMarkers(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:220
	// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMarkers(InputArray, const Ptr<Dictionary> &, OutputArrayOfArrays, OutputArray, const Ptr<DetectorParameters> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:220
	// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids", "parameters", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR_const_PtrLDetectorParametersGR_const__OutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, const cv::_OutputArray* rejectedImgPoints, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids, *parameters, *rejectedImgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawCharucoDiamond(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:335
	// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(const cv::Ptr<cv::aruco::Dictionary>* dictionary, cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawCharucoDiamond(const Ptr<Dictionary> &, Vec4i, int, int, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:335
	// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:205
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:205
	// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, cv::Scalar* cornerColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners, *charucoIds, *cornerColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:313
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:313
	// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, const cv::_InputArray* diamondIds, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners, *diamondIds, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:537
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* corners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:537
	// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* corners, const cv::_InputArray* ids, cv::Scalar* borderColor, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners, *ids, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawMarker(CppPassByVoidPtr, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:555
	// ("cv::aruco::drawMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_drawMarker_const_PtrLDictionaryGR_int_int_const__OutputArrayR(const cv::Ptr<cv::aruco::Dictionary>* dictionary, int id, int sidePixels, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawMarker(*dictionary, id, sidePixels, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMarker(const Ptr<Dictionary> &, int, int, OutputArray, int)(CppPassByVoidPtr, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:555
	// ("cv::aruco::drawMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_drawMarker_const_PtrLDictionaryGR_int_int_const__OutputArrayR_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, int id, int sidePixels, const cv::_OutputArray* img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawMarker(*dictionary, id, sidePixels, *img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::drawPlanarBoard(CppPassByVoidPtr, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:575
	// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR(const cv::Ptr<cv::aruco::Board>* board, cv::Size* outSize, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawPlanarBoard(*board, *outSize, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawPlanarBoard(const Ptr<Board> &, Size, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:575
	// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Board>* board, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::drawPlanarBoard(*board, *outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:472
	// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseBoard(InputArrayOfArrays, InputArray, const Ptr<Board> &, InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:472
	// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseCharucoBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:186
	// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseCharucoBoard(InputArray, InputArray, const Ptr<CharucoBoard> &, InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:186
	// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, bool useExtrinsicGuess, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::estimatePoseSingleMarkers(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:310
	// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimatePoseSingleMarkers(InputArrayOfArrays, float, InputArray, InputArray, OutputArray, OutputArray, OutputArray, Ptr<EstimateParameters>)(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:310
	// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "_objPoints", "estimateParameters"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Ptr<cv::aruco::EstimateParameters>"]), _)]),
	void cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_PtrLEstimateParametersG(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* _objPoints, cv::Ptr<cv::aruco::EstimateParameters>* estimateParameters, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *_objPoints, *estimateParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::generateCustomDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:196
	// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
	void cv_aruco_generateCustomDictionary_int_int(int nMarkers, int markerSize, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::generateCustomDictionary(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:215
	// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	void cv_aruco_generateCustomDictionary_int_int_const_PtrLDictionaryGR(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, *baseDictionary);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateCustomDictionary(int, int, const Ptr<Dictionary> &, int)(Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:215
	// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
	void cv_aruco_generateCustomDictionary_int_int_const_PtrLDictionaryGR_int(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateCustomDictionary(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:196
	// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "randomSeed"], ["int", "int", "int"]), _)]),
	void cv_aruco_generateCustomDictionary_int_int_int(int nMarkers, int markerSize, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBoardObjectAndImagePoints(const Ptr<Board> &, InputArrayOfArrays, InputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:652
	// ("cv::aruco::getBoardObjectAndImagePoints", vec![(pred!(mut, ["board", "detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_getBoardObjectAndImagePoints_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* detectedCorners, const cv::_InputArray* detectedIds, const cv::_OutputArray* objPoints, const cv::_OutputArray* imgPoints, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::getBoardObjectAndImagePoints(*board, *detectedCorners, *detectedIds, *objPoints, *imgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(PREDEFINED_DICTIONARY_NAME)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:184
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PREDEFINED_DICTIONARY_NAME"]), _)]),
	void cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(cv::aruco::PREDEFINED_DICTIONARY_NAME name, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(name);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:190
	// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
	void cv_aruco_getPredefinedDictionary_int(int dict, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(dict);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::interpolateCornersCharuco(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:158
	// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// interpolateCornersCharuco(InputArrayOfArrays, InputArray, InputArray, const Ptr<CharucoBoard> &, OutputArray, OutputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:158
	// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds", "cameraMatrix", "distCoeffs", "minMarkers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, int minMarkers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds, *cameraMatrix, *distCoeffs, minMarkers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::refineDetectedMarkers(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:510
	// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// refineDetectedMarkers(InputArray, const Ptr<Board> &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, float, float, bool, OutputArray, const Ptr<DetectorParameters> &)(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:510
	// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "minRepDistance", "errorCorrectionRate", "checkAllOrders", "recoveredIdxs", "parameters"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "float", "bool", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*"]), _)]),
	void cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_PtrLDetectorParametersGR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, float minRepDistance, float errorCorrectionRate, bool checkAllOrders, const cv::_OutputArray* recoveredIdxs, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, ResultVoid* ocvrs_return) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, minRepDistance, errorCorrectionRate, checkAllOrders, *recoveredIdxs, *parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testCharucoCornersCollinear(const Ptr<CharucoBoard> &, InputArray)(CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:350
	// ("cv::aruco::testCharucoCornersCollinear", vec![(pred!(mut, ["_board", "_charucoIds"], ["const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(const cv::Ptr<cv::aruco::CharucoBoard>* _board, const cv::_InputArray* _charucoIds, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::testCharucoCornersCollinear(*_board, *_charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArrayOfArrays, const Ptr<Dictionary> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:338
	// ("cv::aruco::Board::create", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_InputArray*"]), _)]),
	void cv_aruco_Board_create_const__InputArrayR_const_PtrLDictionaryGR_const__InputArrayR(const cv::_InputArray* objPoints, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_InputArray* ids, Result<cv::Ptr<cv::aruco::Board>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Board> ret = cv::aruco::Board::create(*objPoints, *dictionary, *ids);
			Ok(new cv::Ptr<cv::aruco::Board>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIds(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:349
	// ("cv::aruco::Board::setIds", vec![(pred!(mut, ["ids"], ["const cv::_InputArray*"]), _)]),
	void cv_aruco_Board_setIds_const__InputArrayR(cv::aruco::Board* instance, const cv::_InputArray* ids, ResultVoid* ocvrs_return) {
		try {
			instance->setIds(*ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Board::defaultNew() generated
	// ("cv::aruco::Board::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::aruco::Board* cv_aruco_Board_defaultNew_const() {
			cv::aruco::Board* ret = new cv::aruco::Board();
			return ret;
	}

	// cv::aruco::Board::objPoints() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:360
	// ("cv::aruco::Board::objPoints", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point3f>>* cv_aruco_Board_propObjPoints_const(const cv::aruco::Board* instance) {
			std::vector<std::vector<cv::Point3f>> ret = instance->objPoints;
			return new std::vector<std::vector<cv::Point3f>>(ret);
	}

	// cv::aruco::Board::setObjPoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:360
	// ("cv::aruco::Board::setObjPoints", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Point3f>>"]), _)]),
	void cv_aruco_Board_propObjPoints_const_vectorLvectorLPoint3fGG(cv::aruco::Board* instance, const std::vector<std::vector<cv::Point3f>>* val) {
			instance->objPoints = *val;
	}

	// cv::aruco::Board::dictionary() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:363
	// ("cv::aruco::Board::dictionary", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::aruco::Dictionary>* cv_aruco_Board_propDictionary(cv::aruco::Board* instance) {
			cv::Ptr<cv::aruco::Dictionary> ret = instance->dictionary;
			return new cv::Ptr<cv::aruco::Dictionary>(ret);
	}

	// cv::aruco::Board::setDictionary(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:363
	// ("cv::aruco::Board::setDictionary", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::aruco::Dictionary>"]), _)]),
	void cv_aruco_Board_propDictionary_const_PtrLDictionaryG(cv::aruco::Board* instance, const cv::Ptr<cv::aruco::Dictionary>* val) {
			instance->dictionary = *val;
	}

	// cv::aruco::Board::ids() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:367
	// ("cv::aruco::Board::ids", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_aruco_Board_propIds_const(const cv::aruco::Board* instance) {
			std::vector<int> ret = instance->ids;
			return new std::vector<int>(ret);
	}

	// cv::aruco::Board::setIds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:367
	// ("cv::aruco::Board::setIds", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_aruco_Board_propIds_const_vectorLintG(cv::aruco::Board* instance, const std::vector<int>* val) {
			instance->ids = *val;
	}

	// cv::aruco::Board::rightBottomBorder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:370
	// ("cv::aruco::Board::rightBottomBorder", vec![(pred!(const, [], []), _)]),
	void cv_aruco_Board_propRightBottomBorder_const(const cv::aruco::Board* instance, cv::Point3f* ocvrs_return) {
			cv::Point3f ret = instance->rightBottomBorder;
			*ocvrs_return = ret;
	}

	// cv::aruco::Board::setRightBottomBorder(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:370
	// ("cv::aruco::Board::setRightBottomBorder", vec![(pred!(mut, ["val"], ["const cv::Point3f"]), _)]),
	void cv_aruco_Board_propRightBottomBorder_const_Point3f(cv::aruco::Board* instance, const cv::Point3f* val) {
			instance->rightBottomBorder = *val;
	}

	// cv::aruco::Board::delete() generated
	// ("cv::aruco::Board::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Board_delete(cv::aruco::Board* instance) {
			delete instance;
	}

	// draw(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:83
	// ("cv::aruco::CharucoBoard::draw", vec![(pred!(mut, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(cv::aruco::CharucoBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoBoard::draw(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:83
	// ("cv::aruco::CharucoBoard::draw", vec![(pred!(mut, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR(cv::aruco::CharucoBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->draw(*outSize, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, float, float, const Ptr<Dictionary> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:100
	// ("cv::aruco::CharucoBoard::create", vec![(pred!(mut, ["squaresX", "squaresY", "squareLength", "markerLength", "dictionary"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	void cv_aruco_CharucoBoard_create_int_int_float_float_const_PtrLDictionaryGR(int squaresX, int squaresY, float squareLength, float markerLength, const cv::Ptr<cv::aruco::Dictionary>* dictionary, Result<cv::Ptr<cv::aruco::CharucoBoard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::CharucoBoard> ret = cv::aruco::CharucoBoard::create(squaresX, squaresY, squareLength, markerLength, *dictionary);
			Ok(new cv::Ptr<cv::aruco::CharucoBoard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:106
	// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getChessboardSize_const(const cv::aruco::CharucoBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getChessboardSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSquareLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:111
	// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getSquareLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSquareLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:116
	// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_CharucoBoard_getMarkerLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::CharucoBoard::defaultNew() generated
	// ("cv::aruco::CharucoBoard::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::aruco::CharucoBoard* cv_aruco_CharucoBoard_defaultNew_const() {
			cv::aruco::CharucoBoard* ret = new cv::aruco::CharucoBoard();
			return ret;
	}

	// cv::aruco::CharucoBoard::chessboardCorners() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:66
	// ("cv::aruco::CharucoBoard::chessboardCorners", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point3f>* cv_aruco_CharucoBoard_propChessboardCorners_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<cv::Point3f> ret = instance->chessboardCorners;
			return new std::vector<cv::Point3f>(ret);
	}

	// cv::aruco::CharucoBoard::setChessboardCorners(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:66
	// ("cv::aruco::CharucoBoard::setChessboardCorners", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point3f>"]), _)]),
	void cv_aruco_CharucoBoard_propChessboardCorners_const_vectorLPoint3fG(cv::aruco::CharucoBoard* instance, const std::vector<cv::Point3f>* val) {
			instance->chessboardCorners = *val;
	}

	// cv::aruco::CharucoBoard::nearestMarkerIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:69
	// ("cv::aruco::CharucoBoard::nearestMarkerIdx", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<int>>* cv_aruco_CharucoBoard_propNearestMarkerIdx_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<std::vector<int>> ret = instance->nearestMarkerIdx;
			return new std::vector<std::vector<int>>(ret);
	}

	// cv::aruco::CharucoBoard::setNearestMarkerIdx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:69
	// ("cv::aruco::CharucoBoard::setNearestMarkerIdx", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	void cv_aruco_CharucoBoard_propNearestMarkerIdx_const_vectorLvectorLintGG(cv::aruco::CharucoBoard* instance, const std::vector<std::vector<int>>* val) {
			instance->nearestMarkerIdx = *val;
	}

	// cv::aruco::CharucoBoard::nearestMarkerCorners() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:70
	// ("cv::aruco::CharucoBoard::nearestMarkerCorners", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<int>>* cv_aruco_CharucoBoard_propNearestMarkerCorners_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<std::vector<int>> ret = instance->nearestMarkerCorners;
			return new std::vector<std::vector<int>>(ret);
	}

	// cv::aruco::CharucoBoard::setNearestMarkerCorners(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:70
	// ("cv::aruco::CharucoBoard::setNearestMarkerCorners", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	void cv_aruco_CharucoBoard_propNearestMarkerCorners_const_vectorLvectorLintGG(cv::aruco::CharucoBoard* instance, const std::vector<std::vector<int>>* val) {
			instance->nearestMarkerCorners = *val;
	}

	// cv::aruco::CharucoBoard::to_Board() generated
	// ("cv::aruco::CharucoBoard::to_Board", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Board* cv_aruco_CharucoBoard_to_Board(cv::aruco::CharucoBoard* instance) {
			return dynamic_cast<cv::aruco::Board*>(instance);
	}

	// cv::aruco::CharucoBoard::delete() generated
	// ("cv::aruco::CharucoBoard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_CharucoBoard_delete(cv::aruco::CharucoBoard* instance) {
			delete instance;
	}

	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:153
	// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_DetectorParameters(Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:154
	// ("cv::aruco::DetectorParameters::create", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_create(Result<cv::Ptr<cv::aruco::DetectorParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::DetectorParameters> ret = cv::aruco::DetectorParameters::create();
			Ok(new cv::Ptr<cv::aruco::DetectorParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:155
	// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(cv::aruco::DetectorParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDetectorParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::DetectorParameters::implicitClone() generated
	// ("cv::aruco::DetectorParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::DetectorParameters* cv_aruco_DetectorParameters_implicitClone_const(const cv::aruco::DetectorParameters* instance) {
			return new cv::aruco::DetectorParameters(*instance);
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:157
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:157
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:158
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:158
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:159
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:159
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}

	// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:160
	// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:160
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->adaptiveThreshConstant = val;
	}

	// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:161
	// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:161
	// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:162
	// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxMarkerPerimeterRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:162
	// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxMarkerPerimeterRate = val;
	}

	// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:163
	// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->polygonalApproxAccuracyRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:163
	// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->polygonalApproxAccuracyRate = val;
	}

	// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:164
	// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minCornerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:164
	// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minCornerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:165
	// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMinDistanceToBorder_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minDistanceToBorder;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:165
	// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->minDistanceToBorder = val;
	}

	// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:166
	// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerDistanceRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:166
	// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minMarkerDistanceRate = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:167
	// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMethod_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMethod;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:167
	// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMethod = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:168
	// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementWinSize;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:168
	// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementWinSize = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:169
	// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMaxIterations;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:169
	// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->cornerRefinementMaxIterations = val;
	}

	// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:170
	// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->cornerRefinementMinAccuracy;
			return ret;
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:170
	// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->cornerRefinementMinAccuracy = val;
	}

	// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:171
	// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propMarkerBorderBits_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->markerBorderBits;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:171
	// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->markerBorderBits = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:172
	// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->perspectiveRemovePixelPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:172
	// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->perspectiveRemovePixelPerCell = val;
	}

	// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:173
	// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->perspectiveRemoveIgnoredMarginPerCell;
			return ret;
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:173
	// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->perspectiveRemoveIgnoredMarginPerCell = val;
	}

	// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:174
	// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxErroneousBitsInBorderRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:174
	// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->maxErroneousBitsInBorderRate = val;
	}

	// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:175
	// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propMinOtsuStdDev_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minOtsuStdDev;
			return ret;
	}

	// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:175
	// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->minOtsuStdDev = val;
	}

	// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:176
	// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
	double cv_aruco_DetectorParameters_propErrorCorrectionRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->errorCorrectionRate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:176
	// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(cv::aruco::DetectorParameters* instance, const double val) {
			instance->errorCorrectionRate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:179
	// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadDecimate;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:179
	// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadDecimate = val;
	}

	// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:180
	// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadSigma;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:180
	// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagQuadSigma = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:183
	// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinClusterPixels;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:183
	// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinClusterPixels = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:184
	// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMaxNmaxima;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:184
	// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMaxNmaxima = val;
	}

	// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:185
	// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagCriticalRad;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:185
	// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagCriticalRad = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:186
	// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
	float cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagMaxLineFitMse;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:186
	// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(cv::aruco::DetectorParameters* instance, const float val) {
			instance->aprilTagMaxLineFitMse = val;
	}

	// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:187
	// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinWhiteBlackDiff;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:187
	// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagMinWhiteBlackDiff = val;
	}

	// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:188
	// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
	int cv_aruco_DetectorParameters_propAprilTagDeglitch_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagDeglitch;
			return ret;
	}

	// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:188
	// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(cv::aruco::DetectorParameters* instance, const int val) {
			instance->aprilTagDeglitch = val;
	}

	// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:191
	// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_DetectorParameters_propDetectInvertedMarker_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->detectInvertedMarker;
			return ret;
	}

	// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:191
	// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(cv::aruco::DetectorParameters* instance, const bool val) {
			instance->detectInvertedMarker = val;
	}

	// cv::aruco::DetectorParameters::delete() generated
	// ("cv::aruco::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
			delete instance;
	}

	// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:71
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["_bytesList", "_markerSize", "_maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_MatR_int_int(const cv::Mat* _bytesList, int _markerSize, int _maxcorr, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_bytesList, _markerSize, _maxcorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::Dictionary() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:71
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_Dictionary(Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Dictionary(const Ptr<Dictionary> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:81
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["_dictionary"], ["const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	void cv_aruco_Dictionary_Dictionary_const_PtrLDictionaryGR(const cv::Ptr<cv::aruco::Dictionary>* _dictionary, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:87
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "randomSeed"], ["int", "int", "int"]), _)]),
	void cv_aruco_Dictionary_create_int_int_int(int nMarkers, int markerSize, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::create(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:87
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
	void cv_aruco_Dictionary_create_int_int(int nMarkers, int markerSize, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, const Ptr<Dictionary> &, int)(Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:93
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
	void cv_aruco_Dictionary_create_int_int_const_PtrLDictionaryGR_int(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::create(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:93
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	void cv_aruco_Dictionary_create_int_int_const_PtrLDictionaryGR(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, *baseDictionary);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:105
	// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_aruco_Dictionary_readDictionary_const_FileNodeR(cv::aruco::Dictionary* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDictionary(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeDictionary(Ptr<FileStorage> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:110
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::Ptr<cv::FileStorage>*"]), _)]),
	void cv_aruco_Dictionary_writeDictionary_PtrLFileStorageGR(cv::aruco::Dictionary* instance, cv::Ptr<cv::FileStorage>* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeDictionary(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:114
	// ("cv::aruco::Dictionary::get", vec![(pred!(mut, ["dict"], ["int"]), _)]),
	void cv_aruco_Dictionary_get_int(int dict, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::get(dict);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:120
	// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
	void cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(const cv::aruco::Dictionary* instance, const cv::Mat* onlyBits, int* idx, int* rotation, double maxCorrectionRate, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->identify(*onlyBits, *idx, *rotation, maxCorrectionRate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:126
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, bool allRotations, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id, allRotations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:126
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:132
	// ("cv::aruco::Dictionary::drawMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
	void cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->drawMarker(id, sidePixels, *_img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::drawMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:132
	// ("cv::aruco::Dictionary::drawMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, ResultVoid* ocvrs_return) {
		try {
			instance->drawMarker(id, sidePixels, *_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:138
	// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
	void cv_aruco_Dictionary_getByteListFromBits_const_MatR(const cv::Mat* bits, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*bits);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:144
	// ("cv::aruco::Dictionary::getBitsFromByteList", vec![(pred!(mut, ["byteList", "markerSize"], ["const cv::Mat*", "int"]), _)]),
	void cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(const cv::Mat* byteList, int markerSize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getBitsFromByteList(*byteList, markerSize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:64
	// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_aruco_Dictionary_propBytesList_const(const cv::aruco::Dictionary* instance) {
			cv::Mat ret = instance->bytesList;
			return new cv::Mat(ret);
	}

	// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:64
	// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_aruco_Dictionary_propBytesList_const_Mat(cv::aruco::Dictionary* instance, const cv::Mat* val) {
			instance->bytesList = *val;
	}

	// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:65
	// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMarkerSize_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->markerSize;
			return ret;
	}

	// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:65
	// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMarkerSize_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->markerSize = val;
	}

	// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:66
	// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
	int cv_aruco_Dictionary_propMaxCorrectionBits_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->maxCorrectionBits;
			return ret;
	}

	// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:66
	// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_aruco_Dictionary_propMaxCorrectionBits_const_int(cv::aruco::Dictionary* instance, const int val) {
			instance->maxCorrectionBits = val;
	}

	// cv::aruco::Dictionary::delete() generated
	// ("cv::aruco::Dictionary::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_Dictionary_delete(cv::aruco::Dictionary* instance) {
			delete instance;
	}

	// EstimateParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:265
	// ("cv::aruco::EstimateParameters::EstimateParameters", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_EstimateParameters_EstimateParameters(Result<cv::aruco::EstimateParameters*>* ocvrs_return) {
		try {
			cv::aruco::EstimateParameters* ret = new cv::aruco::EstimateParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:268
	// ("cv::aruco::EstimateParameters::create", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_EstimateParameters_create(Result<cv::Ptr<cv::aruco::EstimateParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::EstimateParameters> ret = cv::aruco::EstimateParameters::create();
			Ok(new cv::Ptr<cv::aruco::EstimateParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::EstimateParameters::implicitClone() generated
	// ("cv::aruco::EstimateParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::aruco::EstimateParameters* cv_aruco_EstimateParameters_implicitClone_const(const cv::aruco::EstimateParameters* instance) {
			return new cv::aruco::EstimateParameters(*instance);
	}

	// cv::aruco::EstimateParameters::pattern() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:261
	// ("cv::aruco::EstimateParameters::pattern", vec![(pred!(const, [], []), _)]),
	void cv_aruco_EstimateParameters_propPattern_const(const cv::aruco::EstimateParameters* instance, cv::aruco::PatternPos* ocvrs_return) {
			cv::aruco::PatternPos ret = instance->pattern;
			*ocvrs_return = ret;
	}

	// cv::aruco::EstimateParameters::setPattern(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:261
	// ("cv::aruco::EstimateParameters::setPattern", vec![(pred!(mut, ["val"], ["const cv::aruco::PatternPos"]), _)]),
	void cv_aruco_EstimateParameters_propPattern_const_PatternPos(cv::aruco::EstimateParameters* instance, const cv::aruco::PatternPos val) {
			instance->pattern = val;
	}

	// cv::aruco::EstimateParameters::useExtrinsicGuess() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:262
	// ("cv::aruco::EstimateParameters::useExtrinsicGuess", vec![(pred!(const, [], []), _)]),
	bool cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(const cv::aruco::EstimateParameters* instance) {
			bool ret = instance->useExtrinsicGuess;
			return ret;
	}

	// cv::aruco::EstimateParameters::setUseExtrinsicGuess(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:262
	// ("cv::aruco::EstimateParameters::setUseExtrinsicGuess", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(cv::aruco::EstimateParameters* instance, const bool val) {
			instance->useExtrinsicGuess = val;
	}

	// cv::aruco::EstimateParameters::solvePnPMethod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:263
	// ("cv::aruco::EstimateParameters::solvePnPMethod", vec![(pred!(const, [], []), _)]),
	void cv_aruco_EstimateParameters_propSolvePnPMethod_const(const cv::aruco::EstimateParameters* instance, cv::SolvePnPMethod* ocvrs_return) {
			cv::SolvePnPMethod ret = instance->solvePnPMethod;
			*ocvrs_return = ret;
	}

	// cv::aruco::EstimateParameters::setSolvePnPMethod(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:263
	// ("cv::aruco::EstimateParameters::setSolvePnPMethod", vec![(pred!(mut, ["val"], ["const cv::SolvePnPMethod"]), _)]),
	void cv_aruco_EstimateParameters_propSolvePnPMethod_const_SolvePnPMethod(cv::aruco::EstimateParameters* instance, const cv::SolvePnPMethod val) {
			instance->solvePnPMethod = val;
	}

	// cv::aruco::EstimateParameters::delete() generated
	// ("cv::aruco::EstimateParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_EstimateParameters_delete(cv::aruco::EstimateParameters* instance) {
			delete instance;
	}

	// draw(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:394
	// ("cv::aruco::GridBoard::draw", vec![(pred!(mut, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(cv::aruco::GridBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, ResultVoid* ocvrs_return) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::draw(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:394
	// ("cv::aruco::GridBoard::draw", vec![(pred!(mut, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_aruco_GridBoard_draw_Size_const__OutputArrayR(cv::aruco::GridBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->draw(*outSize, *img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, float, float, const Ptr<Dictionary> &, int)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:411
	// ("cv::aruco::GridBoard::create", vec![(pred!(mut, ["markersX", "markersY", "markerLength", "markerSeparation", "dictionary", "firstMarker"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
	void cv_aruco_GridBoard_create_int_int_float_float_const_PtrLDictionaryGR_int(int markersX, int markersY, float markerLength, float markerSeparation, const cv::Ptr<cv::aruco::Dictionary>* dictionary, int firstMarker, Result<cv::Ptr<cv::aruco::GridBoard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::GridBoard> ret = cv::aruco::GridBoard::create(markersX, markersY, markerLength, markerSeparation, *dictionary, firstMarker);
			Ok(new cv::Ptr<cv::aruco::GridBoard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::create(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:411
	// ("cv::aruco::GridBoard::create", vec![(pred!(mut, ["markersX", "markersY", "markerLength", "markerSeparation", "dictionary"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	void cv_aruco_GridBoard_create_int_int_float_float_const_PtrLDictionaryGR(int markersX, int markersY, float markerLength, float markerSeparation, const cv::Ptr<cv::aruco::Dictionary>* dictionary, Result<cv::Ptr<cv::aruco::GridBoard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::GridBoard> ret = cv::aruco::GridBoard::create(markersX, markersY, markerLength, markerSeparation, *dictionary);
			Ok(new cv::Ptr<cv::aruco::GridBoard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:417
	// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getGridSize_const(const cv::aruco::GridBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:422
	// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerLength_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:427
	// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
	void cv_aruco_GridBoard_getMarkerSeparation_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerSeparation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::aruco::GridBoard::defaultNew() generated
	// ("cv::aruco::GridBoard::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::aruco::GridBoard* cv_aruco_GridBoard_defaultNew_const() {
			cv::aruco::GridBoard* ret = new cv::aruco::GridBoard();
			return ret;
	}

	// cv::aruco::GridBoard::to_Board() generated
	// ("cv::aruco::GridBoard::to_Board", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Board* cv_aruco_GridBoard_to_Board(cv::aruco::GridBoard* instance) {
			return dynamic_cast<cv::aruco::Board*>(instance);
	}

	// cv::aruco::GridBoard::delete() generated
	// ("cv::aruco::GridBoard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_aruco_GridBoard_delete(cv::aruco::GridBoard* instance) {
			delete instance;
	}

}
