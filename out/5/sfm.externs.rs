// KRtFromProjection(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:88
// ("cv::sfm::KRtFromProjection", vec![(pred!(mut, ["P", "K", "R", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_KRtFromProjection_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(p: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// applyTransformationToPoints(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:82
// ("cv::sfm::applyTransformationToPoints", vec![(pred!(mut, ["points", "T", "transformed_points"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_applyTransformationToPoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points: *const c_void, t: *const c_void, transformed_points: *const c_void, ocvrs_return: *mut Result<()>);
// computeOrientation(InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:212
// ("cv::sfm::computeOrientation", vec![(pred!(mut, ["x1", "x2", "R", "t", "s"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_sfm_computeOrientation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(x1: *const c_void, x2: *const c_void, r: *const c_void, t: *const c_void, s: f64, ocvrs_return: *mut Result<()>);
// depth(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:97
// ("cv::sfm::depth", vec![(pred!(mut, ["R", "t", "X"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_sfm_depth_const__InputArrayR_const__InputArrayR_const__InputArrayR(r: *const c_void, t: *const c_void, x: *const c_void, ocvrs_return: *mut Result<f64>);
// essentialFromFundamental(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:167
// ("cv::sfm::essentialFromFundamental", vec![(pred!(mut, ["F", "K1", "K2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_essentialFromFundamental_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(f: *const c_void, k1: *const c_void, k2: *const c_void, e: *const c_void, ocvrs_return: *mut Result<()>);
// essentialFromRt(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:183
// ("cv::sfm::essentialFromRt", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_essentialFromRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(r1: *const c_void, t1: *const c_void, r2: *const c_void, t2: *const c_void, e: *const c_void, ocvrs_return: *mut Result<()>);
// euclideanToHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:63
// ("cv::sfm::euclideanToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_euclideanToHomogeneous_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::sfm::fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:90
// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(x1: *const c_void, x2: *const c_void, max_error: f64, f: *const c_void, inliers: *const c_void, ocvrs_return: *mut Result<f64>);
// fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:90
// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(x1: *const c_void, x2: *const c_void, max_error: f64, f: *const c_void, inliers: *const c_void, outliers_probability: f64, ocvrs_return: *mut Result<f64>);
// cv::sfm::fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:67
// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(x1: *const c_void, x2: *const c_void, max_error: f64, f: *const c_void, inliers: *const c_void, ocvrs_return: *mut Result<f64>);
// fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:67
// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(x1: *const c_void, x2: *const c_void, max_error: f64, f: *const c_void, inliers: *const c_void, outliers_probability: f64, ocvrs_return: *mut Result<f64>);
// fundamentalFromEssential(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:152
// ("cv::sfm::fundamentalFromEssential", vec![(pred!(mut, ["E", "K1", "K2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_fundamentalFromEssential_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(e: *const c_void, k1: *const c_void, k2: *const c_void, f: *const c_void, ocvrs_return: *mut Result<()>);
// fundamentalFromProjections(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:69
// ("cv::sfm::fundamentalFromProjections", vec![(pred!(mut, ["P1", "P2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_fundamentalFromProjections_const__InputArrayR_const__InputArrayR_const__OutputArrayR(p1: *const c_void, p2: *const c_void, f: *const c_void, ocvrs_return: *mut Result<()>);
// homogeneousToEuclidean(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:55
// ("cv::sfm::homogeneousToEuclidean", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_homogeneousToEuclidean_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::sfm::importReconstruction(InString, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:77
// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(file: *const c_char, rs: *const c_void, ts: *const c_void, ks: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// importReconstruction(const cv::String &, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays, int)(InString, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:77
// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d", "file_format"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(file: *const c_char, rs: *const c_void, ts: *const c_void, ks: *const c_void, points3d: *const c_void, file_format: i32, ocvrs_return: *mut Result<()>);
// isotropicPreconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:72
// ("cv::sfm::isotropicPreconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_isotropicPreconditionerFromPoints_const__InputArrayR_const__OutputArrayR(points: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// meanAndVarianceAlongRows(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/numeric.hpp:58
// ("cv::sfm::meanAndVarianceAlongRows", vec![(pred!(mut, ["A", "mean", "variance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_meanAndVarianceAlongRows_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(a: *const c_void, mean: *const c_void, variance: *const c_void, ocvrs_return: *mut Result<()>);
// motionFromEssentialChooseSolution(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:135
// ("cv::sfm::motionFromEssentialChooseSolution", vec![(pred!(mut, ["Rs", "ts", "K1", "x1", "K2", "x2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_sfm_motionFromEssentialChooseSolution_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(rs: *const c_void, ts: *const c_void, k1: *const c_void, x1: *const c_void, k2: *const c_void, x2: *const c_void, ocvrs_return: *mut Result<i32>);
// motionFromEssential(InputArray, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:117
// ("cv::sfm::motionFromEssential", vec![(pred!(mut, ["E", "Rs", "ts"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_motionFromEssential_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e: *const c_void, rs: *const c_void, ts: *const c_void, ocvrs_return: *mut Result<()>);
// normalizeFundamental(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:197
// ("cv::sfm::normalizeFundamental", vec![(pred!(mut, ["F", "F_normalized"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_normalizeFundamental_const__InputArrayR_const__OutputArrayR(f: *const c_void, f_normalized: *const c_void, ocvrs_return: *mut Result<()>);
// normalizeIsotropicPoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:112
// ("cv::sfm::normalizeIsotropicPoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_normalizeIsotropicPoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points: *const c_void, normalized_points: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// normalizePoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:97
// ("cv::sfm::normalizePoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_normalizePoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points: *const c_void, normalized_points: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// normalizedEightPointSolver(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:83
// ("cv::sfm::normalizedEightPointSolver", vec![(pred!(mut, ["x1", "x2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_normalizedEightPointSolver_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x1: *const c_void, x2: *const c_void, f: *const c_void, ocvrs_return: *mut Result<()>);
// preconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:59
// ("cv::sfm::preconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_preconditionerFromPoints_const__InputArrayR_const__OutputArrayR(points: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// projectionFromKRt(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:76
// ("cv::sfm::projectionFromKRt", vec![(pred!(mut, ["K", "R", "t", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_projectionFromKRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(k: *const c_void, r: *const c_void, t: *const c_void, p: *const c_void, ocvrs_return: *mut Result<()>);
// projectionsFromFundamental(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:58
// ("cv::sfm::projectionsFromFundamental", vec![(pred!(mut, ["F", "P1", "P2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_projectionsFromFundamental_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(f: *const c_void, p1: *const c_void, p2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:74
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(points2d: *const c_void, ps: *const c_void, points3d: *const c_void, k: *const c_void, ocvrs_return: *mut Result<()>);
// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:74
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
pub fn cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(points2d: *const c_void, ps: *const c_void, points3d: *const c_void, k: *const c_void, is_projective: bool, ocvrs_return: *mut Result<()>);
// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:93
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(points2d: *const c_void, rs: *const c_void, ts: *const c_void, k: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:93
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(points2d: *const c_void, rs: *const c_void, ts: *const c_void, k: *const c_void, points3d: *const c_void, is_projective: bool, ocvrs_return: *mut Result<()>);
// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:111
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(images: *const c_void, ps: *const c_void, points3d: *const c_void, k: *const c_void, ocvrs_return: *mut Result<()>);
// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:111
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
pub fn cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(images: *const c_void, ps: *const c_void, points3d: *const c_void, k: *const c_void, is_projective: bool, ocvrs_return: *mut Result<()>);
// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:131
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(images: *const c_void, rs: *const c_void, ts: *const c_void, k: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:131
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(images: *const c_void, rs: *const c_void, ts: *const c_void, k: *const c_void, points3d: *const c_void, is_projective: bool, ocvrs_return: *mut Result<()>);
// relativeCameraMotion(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:101
// ("cv::sfm::relativeCameraMotion", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_relativeCameraMotion_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r1: *const c_void, t1: *const c_void, r2: *const c_void, t2: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// skew(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/numeric.hpp:69
// ("cv::sfm::skew", vec![(pred!(mut, ["x"], ["const cv::_InputArray*"]), _)]),
pub fn cv_sfm_skew_const__InputArrayR(x: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// triangulatePoints(InputArrayOfArrays, InputArrayOfArrays, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/triangulation.hpp:59
// ("cv::sfm::triangulatePoints", vec![(pred!(mut, ["points2d", "projection_matrices", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_triangulatePoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points2d: *const c_void, projection_matrices: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:167
// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
pub fn cv_sfm_BaseSFM_run_const__InputArrayR(instance: *mut c_void, points2d: *const c_void, ocvrs_return: *mut Result<()>);
// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:170
// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_BaseSFM_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points2d: *const c_void, k: *const c_void, rs: *const c_void, ts: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:173
// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_sfm_BaseSFM_run_const_vectorLStringGR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result<()>);
// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:174
// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_BaseSFM_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, k: *const c_void, rs: *const c_void, ts: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// getError()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:177
// ("cv::sfm::BaseSFM::getError", vec![(pred!(const, [], []), _)]),
pub fn cv_sfm_BaseSFM_getError_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:178
// ("cv::sfm::BaseSFM::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_BaseSFM_getPoints_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:179
// ("cv::sfm::BaseSFM::getIntrinsics", vec![(pred!(const, [], []), _)]),
pub fn cv_sfm_BaseSFM_getIntrinsics_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:180
// ("cv::sfm::BaseSFM::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_BaseSFM_getCameras_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, rs: *const c_void, ts: *const c_void, ocvrs_return: *mut Result<()>);
// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:184
// ("cv::sfm::BaseSFM::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
pub fn cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsR(instance: *mut c_void, libmv_reconstruction_options: *const crate::sfm::libmv_ReconstructionOptions, ocvrs_return: *mut Result<()>);
// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:188
// ("cv::sfm::BaseSFM::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
pub fn cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(instance: *mut c_void, libmv_camera_intrinsics_options: *const crate::sfm::libmv_CameraIntrinsicsOptions, ocvrs_return: *mut Result<()>);
// cv::sfm::BaseSFM::to_SFMLibmvEuclideanReconstruction() generated
// ("cv::sfm::BaseSFM::to_SFMLibmvEuclideanReconstruction", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_BaseSFM_to_SFMLibmvEuclideanReconstruction(instance: *mut c_void) -> *mut c_void;
// cv::sfm::BaseSFM::delete() generated
// ("cv::sfm::BaseSFM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_BaseSFM_delete(instance: *mut c_void);
// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:203
// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR(instance: *mut c_void, points2d: *const c_void, ocvrs_return: *mut Result<()>);
// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:216
// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points2d: *const c_void, k: *const c_void, rs: *const c_void, ts: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:226
// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result<()>);
// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:239
// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, k: *const c_void, rs: *const c_void, ts: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// getError()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:245
// ("cv::sfm::SFMLibmvEuclideanReconstruction::getError", vec![(pred!(const, [], []), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:251
// ("cv::sfm::SFMLibmvEuclideanReconstruction::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:256
// ("cv::sfm::SFMLibmvEuclideanReconstruction::getIntrinsics", vec![(pred!(const, [], []), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:263
// ("cv::sfm::SFMLibmvEuclideanReconstruction::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, rs: *const c_void, ts: *const c_void, ocvrs_return: *mut Result<()>);
// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:271
// ("cv::sfm::SFMLibmvEuclideanReconstruction::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsR(instance: *mut c_void, libmv_reconstruction_options: *const crate::sfm::libmv_ReconstructionOptions, ocvrs_return: *mut Result<()>);
// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:279
// ("cv::sfm::SFMLibmvEuclideanReconstruction::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(instance: *mut c_void, libmv_camera_intrinsics_options: *const crate::sfm::libmv_CameraIntrinsicsOptions, ocvrs_return: *mut Result<()>);
// create(const libmv_CameraIntrinsicsOptions &, const libmv_ReconstructionOptions &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:283
// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, ["camera_instrinsic_options", "reconstruction_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*", "const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsR_const_libmv_ReconstructionOptionsR(camera_instrinsic_options: *const crate::sfm::libmv_CameraIntrinsicsOptions, reconstruction_options: *const crate::sfm::libmv_ReconstructionOptions, ocvrs_return: *mut Result<*mut c_void>);
// cv::sfm::SFMLibmvEuclideanReconstruction::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:283
// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::sfm::SFMLibmvEuclideanReconstruction::to_BaseSFM() generated
// ("cv::sfm::SFMLibmvEuclideanReconstruction::to_BaseSFM", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_to_BaseSFM(instance: *mut c_void) -> *mut c_void;
// cv::sfm::SFMLibmvEuclideanReconstruction::delete() generated
// ("cv::sfm::SFMLibmvEuclideanReconstruction::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_SFMLibmvEuclideanReconstruction_delete(instance: *mut c_void);
// libmv_CameraIntrinsicsOptions(const int, const double, const double, const double, const double, const double, const double, const double, const double, const double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:76
// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, ["_distortion_model", "_focal_length_x", "_focal_length_y", "_principal_point_x", "_principal_point_y", "_polynomial_k1", "_polynomial_k2", "_polynomial_k3", "_polynomial_p1", "_polynomial_p2"], ["const int", "const double", "const double", "const double", "const double", "const double", "const double", "const double", "const double", "const double"]), _)]),
pub fn cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_const_int_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double(_distortion_model: i32, _focal_length_x: f64, _focal_length_y: f64, _principal_point_x: f64, _principal_point_y: f64, _polynomial_k1: f64, _polynomial_k2: f64, _polynomial_k3: f64, _polynomial_p1: f64, _polynomial_p2: f64, ocvrs_return: *mut Result<crate::sfm::libmv_CameraIntrinsicsOptions>);
// cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:76
// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions(ocvrs_return: *mut Result<crate::sfm::libmv_CameraIntrinsicsOptions>);
// libmv_ReconstructionOptions(const int, const int, const int, const int, const int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:142
// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, ["_keyframe1", "_keyframe2", "_refine_intrinsics", "_select_keyframes", "_verbosity_level"], ["const int", "const int", "const int", "const int", "const int"]), _)]),
pub fn cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_const_int_const_int_const_int_const_int_const_int(_keyframe1: i32, _keyframe2: i32, _refine_intrinsics: i32, _select_keyframes: i32, _verbosity_level: i32, ocvrs_return: *mut Result<crate::sfm::libmv_ReconstructionOptions>);
// cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:142
// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, [], []), _)]),
pub fn cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions(ocvrs_return: *mut Result<crate::sfm::libmv_ReconstructionOptions>);
