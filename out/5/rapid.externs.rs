// cv::rapid::convertCorrespondencies(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:100
// ("cv::rapid::convertCorrespondencies", vec![(pred!(mut, ["cols", "srcLocations", "pts2d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cols: *const c_void, src_locations: *const c_void, pts2d: *const c_void, ocvrs_return: *mut Result<()>);
// convertCorrespondencies(InputArray, InputArray, OutputArray, InputOutputArray, InputArray)(InputArray, InputArray, OutputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:100
// ("cv::rapid::convertCorrespondencies", vec![(pred!(mut, ["cols", "srcLocations", "pts2d", "pts3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(cols: *const c_void, src_locations: *const c_void, pts2d: *const c_void, pts3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rapid::drawCorrespondencies(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:30
// ("cv::rapid::drawCorrespondencies", vec![(pred!(mut, ["bundle", "cols"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR(bundle: *const c_void, cols: *const c_void, ocvrs_return: *mut Result<()>);
// drawCorrespondencies(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:30
// ("cv::rapid::drawCorrespondencies", vec![(pred!(mut, ["bundle", "cols", "colors"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(bundle: *const c_void, cols: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// drawSearchLines(InputOutputArray, InputArray, const Scalar &)(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:39
// ("cv::rapid::drawSearchLines", vec![(pred!(mut, ["img", "locations", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img: *const c_void, locations: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::rapid::drawWireframe(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:50
// ("cv::rapid::drawWireframe", vec![(pred!(mut, ["img", "pts2d", "tris", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR(img: *const c_void, pts2d: *const c_void, tris: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// drawWireframe(InputOutputArray, InputArray, InputArray, const Scalar &, int, bool)(InputOutputArray, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:50
// ("cv::rapid::drawWireframe", vec![(pred!(mut, ["img", "pts2d", "tris", "color", "type", "cullBackface"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "bool"]), _)]),
pub fn cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(img: *const c_void, pts2d: *const c_void, tris: *const c_void, color: *const core::Scalar, typ: i32, cull_backface: bool, ocvrs_return: *mut Result<()>);
// extractControlPoints(int, int, InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, OutputArray, OutputArray)(Primitive, Primitive, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:67
// ("cv::rapid::extractControlPoints", vec![(pred!(mut, ["num", "len", "pts3d", "rvec", "tvec", "K", "imsize", "tris", "ctl2d", "ctl3d"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(num: i32, len: i32, pts3d: *const c_void, rvec: *const c_void, tvec: *const c_void, k: *const c_void, imsize: *const core::Size, tris: *const c_void, ctl2d: *const c_void, ctl3d: *const c_void, ocvrs_return: *mut Result<()>);
// extractLineBundle(int, InputArray, InputArray, OutputArray, OutputArray)(Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:79
// ("cv::rapid::extractLineBundle", vec![(pred!(mut, ["len", "ctl2d", "img", "bundle", "srcLocations"], ["int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(len: i32, ctl2d: *const c_void, img: *const c_void, bundle: *const c_void, src_locations: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rapid::findCorrespondencies(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:89
// ("cv::rapid::findCorrespondencies", vec![(pred!(mut, ["bundle", "cols"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR(bundle: *const c_void, cols: *const c_void, ocvrs_return: *mut Result<()>);
// findCorrespondencies(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:89
// ("cv::rapid::findCorrespondencies", vec![(pred!(mut, ["bundle", "cols", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(bundle: *const c_void, cols: *const c_void, response: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rapid::rapid(InputArray, Primitive, Primitive, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:123
// ("cv::rapid::rapid", vec![(pred!(mut, ["img", "num", "len", "pts3d", "tris", "K", "rvec", "tvec"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(img: *const c_void, num: i32, len: i32, pts3d: *const c_void, tris: *const c_void, k: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<f32>);
// rapid(InputArray, int, int, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, double *)(InputArray, Primitive, Primitive, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:123
// ("cv::rapid::rapid", vec![(pred!(mut, ["img", "num", "len", "pts3d", "tris", "K", "rvec", "tvec", "rmsd"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "double*"]), _)]),
pub fn cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(img: *const c_void, num: i32, len: i32, pts3d: *const c_void, tris: *const c_void, k: *const c_void, rvec: *const c_void, tvec: *const c_void, rmsd: *mut f64, ocvrs_return: *mut Result<f32>);
// create(InputArray, InputArray, int, uchar)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:158
// ("cv::rapid::GOSTracker::create", vec![(pred!(mut, ["pts3d", "tris", "histBins", "sobelThesh"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "unsigned char"]), _)]),
pub fn cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d: *const c_void, tris: *const c_void, hist_bins: i32, sobel_thesh: u8, ocvrs_return: *mut Result<*mut c_void>);
// cv::rapid::GOSTracker::create(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:158
// ("cv::rapid::GOSTracker::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR(pts3d: *const c_void, tris: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rapid::GOSTracker::to_Algorithm() generated
// ("cv::rapid::GOSTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_GOSTracker_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rapid::GOSTracker::to_Rapid_Tracker() generated
// ("cv::rapid::GOSTracker::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_GOSTracker_to_Rapid_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::rapid::GOSTracker::delete() generated
// ("cv::rapid::GOSTracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_GOSTracker_delete(instance: *mut c_void);
// create(InputArray, InputArray, int, uchar)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:150
// ("cv::rapid::OLSTracker::create", vec![(pred!(mut, ["pts3d", "tris", "histBins", "sobelThesh"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "unsigned char"]), _)]),
pub fn cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d: *const c_void, tris: *const c_void, hist_bins: i32, sobel_thesh: u8, ocvrs_return: *mut Result<*mut c_void>);
// cv::rapid::OLSTracker::create(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:150
// ("cv::rapid::OLSTracker::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR(pts3d: *const c_void, tris: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rapid::OLSTracker::to_Algorithm() generated
// ("cv::rapid::OLSTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_OLSTracker_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rapid::OLSTracker::to_Rapid_Tracker() generated
// ("cv::rapid::OLSTracker::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_OLSTracker_to_Rapid_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::rapid::OLSTracker::delete() generated
// ("cv::rapid::OLSTracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_OLSTracker_delete(instance: *mut c_void);
// create(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:141
// ("cv::rapid::Rapid::create", vec![(pred!(mut, ["pts3d", "tris"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(pts3d: *const c_void, tris: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rapid::Rapid::to_Algorithm() generated
// ("cv::rapid::Rapid::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Rapid_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Rapid::to_Rapid_Tracker() generated
// ("cv::rapid::Rapid::to_Rapid_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Rapid_to_Rapid_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Rapid::delete() generated
// ("cv::rapid::Rapid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Rapid_delete(instance: *mut c_void);
// compute(InputArray, int, int, InputArray, InputOutputArray, InputOutputArray, const TermCriteria &)(InputArray, Primitive, Primitive, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:132
// ("cv::rapid::Tracker::compute", vec![(pred!(mut, ["img", "num", "len", "K", "rvec", "tvec", "termcrit"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::TermCriteria*"]), _)]),
pub fn cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(instance: *mut c_void, img: *const c_void, num: i32, len: i32, k: *const c_void, rvec: *const c_void, tvec: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<f32>);
// cv::rapid::Tracker::compute(InputArray, Primitive, Primitive, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:132
// ("cv::rapid::Tracker::compute", vec![(pred!(mut, ["img", "num", "len", "K", "rvec", "tvec"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, img: *const c_void, num: i32, len: i32, k: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<f32>);
// clearState()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rapid.hpp:134
// ("cv::rapid::Tracker::clearState", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_clearState(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::rapid::Tracker::to_Rapid_GOSTracker() generated
// ("cv::rapid::Tracker::to_Rapid_GOSTracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_to_Rapid_GOSTracker(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Tracker::to_Rapid_OLSTracker() generated
// ("cv::rapid::Tracker::to_Rapid_OLSTracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_to_Rapid_OLSTracker(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Tracker::to_Rapid_Rapid() generated
// ("cv::rapid::Tracker::to_Rapid_Rapid", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_to_Rapid_Rapid(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Tracker::to_Algorithm() generated
// ("cv::rapid::Tracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rapid::Tracker::delete() generated
// ("cv::rapid::Tracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rapid_Tracker_delete(instance: *mut c_void);
