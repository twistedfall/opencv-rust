// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, ocvrs_return: *mut Result<f64>);
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, ocvrs_return: *mut Result<f64>);
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
// cv::aruco::detectCharucoDiamond(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR(image: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, square_marker_length_rate: f32, diamond_corners: *const c_void, diamond_ids: *const c_void, ocvrs_return: *mut Result<()>);
// detectCharucoDiamond(InputArray, InputArrayOfArrays, InputArray, float, OutputArrayOfArrays, OutputArray, InputArray, InputArray, Ptr<Dictionary>)(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray, InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds", "cameraMatrix", "distCoeffs", "dictionary"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Ptr<cv::aruco::Dictionary>"]), _)]),
pub fn cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_PtrLDictionaryG(image: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, square_marker_length_rate: f32, diamond_corners: *const c_void, diamond_ids: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, dictionary: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::detectMarkers(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, dictionary: *const c_void, corners: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<()>);
// detectMarkers(InputArray, const Ptr<Dictionary> &, OutputArrayOfArrays, OutputArray, const Ptr<DetectorParameters> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids", "parameters", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR_const_PtrLDetectorParametersGR_const__OutputArrayR(image: *const c_void, dictionary: *const c_void, corners: *const c_void, ids: *const c_void, parameters: *const c_void, rejected_img_points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::drawCharucoDiamond(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(dictionary: *const c_void, ids: *const core::Vec4i, square_length: i32, marker_length: i32, img: *const c_void, ocvrs_return: *mut Result<()>);
// drawCharucoDiamond(const Ptr<Dictionary> &, Vec4i, int, int, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(dictionary: *const c_void, ids: *const core::Vec4i, square_length: i32, marker_length: i32, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result<()>);
// drawPlanarBoard(const Ptr<Board> &, Size, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:45
// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(board: *const c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result<()>);
// cv::aruco::estimatePoseBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(corners: *const c_void, ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<i32>);
// estimatePoseBoard(InputArrayOfArrays, InputArray, const Ptr<Board> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
pub fn cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(corners: *const c_void, ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, ocvrs_return: *mut Result<i32>);
// cv::aruco::estimatePoseCharucoBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<bool>);
// estimatePoseCharucoBoard(InputArray, InputArray, const Ptr<CharucoBoard> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
pub fn cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, ocvrs_return: *mut Result<bool>);
// cv::aruco::estimatePoseSingleMarkers(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(corners: *const c_void, marker_length: f32, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, ocvrs_return: *mut Result<()>);
// estimatePoseSingleMarkers(InputArrayOfArrays, float, InputArray, InputArray, OutputArray, OutputArray, OutputArray, const Ptr<EstimateParameters> &)(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "objPoints", "estimateParameters"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::EstimateParameters>*"]), _)]),
pub fn cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_PtrLEstimateParametersGR(corners: *const c_void, marker_length: f32, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, obj_points: *const c_void, estimate_parameters: *const c_void, ocvrs_return: *mut Result<()>);
// getBoardObjectAndImagePoints(const Ptr<Board> &, InputArrayOfArrays, InputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:51
// ("cv::aruco::getBoardObjectAndImagePoints", vec![(pred!(mut, ["board", "detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_getBoardObjectAndImagePoints_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, obj_points: *const c_void, img_points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::aruco::interpolateCornersCharuco(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR(marker_corners: *const c_void, marker_ids: *const c_void, image: *const c_void, board: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<i32>);
// interpolateCornersCharuco(InputArrayOfArrays, InputArray, InputArray, const Ptr<CharucoBoard> &, OutputArray, OutputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds", "cameraMatrix", "distCoeffs", "minMarkers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(marker_corners: *const c_void, marker_ids: *const c_void, image: *const c_void, board: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, min_markers: i32, ocvrs_return: *mut Result<i32>);
// cv::aruco::refineDetectedMarkers(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, ocvrs_return: *mut Result<()>);
// refineDetectedMarkers(InputArray, const Ptr<Board> &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, float, float, bool, OutputArray, const Ptr<DetectorParameters> &)(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "minRepDistance", "errorCorrectionRate", "checkAllOrders", "recoveredIdxs", "parameters"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "float", "bool", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*"]), _)]),
pub fn cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_PtrLDetectorParametersGR(image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: *const c_void, parameters: *const c_void, ocvrs_return: *mut Result<()>);
// testCharucoCornersCollinear(const Ptr<CharucoBoard> &, InputArray)(CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:96
// ("cv::aruco::testCharucoCornersCollinear", vec![(pred!(mut, ["board", "charucoIds"], ["const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*"]), _)]),
pub fn cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(board: *const c_void, charuco_ids: *const c_void, ocvrs_return: *mut Result<bool>);
// EstimateParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:60
// ("cv::aruco::EstimateParameters::EstimateParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return: *mut Result<*mut c_void>);
// cv::aruco::EstimateParameters::implicitClone() generated
// ("cv::aruco::EstimateParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_EstimateParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::aruco::EstimateParameters::pattern() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
// ("cv::aruco::EstimateParameters::pattern", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_EstimateParameters_propPattern_const(instance: *const c_void, ocvrs_return: *mut crate::aruco::PatternPositionType);
// cv::aruco::EstimateParameters::setPattern(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
// ("cv::aruco::EstimateParameters::setPattern", vec![(pred!(mut, ["val"], ["const cv::aruco::PatternPositionType"]), _)]),
pub fn cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(instance: *mut c_void, val: crate::aruco::PatternPositionType);
// cv::aruco::EstimateParameters::useExtrinsicGuess() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
// ("cv::aruco::EstimateParameters::useExtrinsicGuess", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(instance: *const c_void) -> bool;
// cv::aruco::EstimateParameters::setUseExtrinsicGuess(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
// ("cv::aruco::EstimateParameters::setUseExtrinsicGuess", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(instance: *mut c_void, val: bool);
// cv::aruco::EstimateParameters::solvePnPMethod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
// ("cv::aruco::EstimateParameters::solvePnPMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_aruco_EstimateParameters_propSolvePnPMethod_const(instance: *const c_void) -> i32;
// cv::aruco::EstimateParameters::setSolvePnPMethod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
// ("cv::aruco::EstimateParameters::setSolvePnPMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(instance: *mut c_void, val: i32);
// cv::aruco::EstimateParameters::delete() generated
// ("cv::aruco::EstimateParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_aruco_EstimateParameters_delete(instance: *mut c_void);
