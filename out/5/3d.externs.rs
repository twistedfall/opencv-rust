// cv::RQDecomp3x3(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, mtx_r: *const c_void, mtx_q: *const c_void, ocvrs_return: *mut Result<core::Vec3d>);
// RQDecomp3x3(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ", "Qx", "Qy", "Qz"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, mtx_r: *const c_void, mtx_q: *const c_void, qx: *const c_void, qy: *const c_void, qz: *const c_void, ocvrs_return: *mut Result<core::Vec3d>);
// cv::Rodrigues(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Rodrigues_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// Rodrigues(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, jacobian: *const c_void, ocvrs_return: *mut Result<()>);
// cv::composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1: *const c_void, tvec1: *const c_void, rvec2: *const c_void, tvec2: *const c_void, rvec3: *const c_void, tvec3: *const c_void, ocvrs_return: *mut Result<()>);
// composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3", "dr3dr1", "dr3dt1", "dr3dr2", "dr3dt2", "dt3dr1", "dt3dt1", "dt3dr2", "dt3dt2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1: *const c_void, tvec1: *const c_void, rvec2: *const c_void, tvec2: *const c_void, rvec3: *const c_void, tvec3: *const c_void, dr3dr1: *const c_void, dr3dt1: *const c_void, dr3dr2: *const c_void, dr3dt2: *const c_void, dt3dr1: *const c_void, dt3dt1: *const c_void, dt3dr2: *const c_void, dt3dt2: *const c_void, ocvrs_return: *mut Result<()>);
// computeCorrespondEpilines(InputArray, int, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1767
// ("cv::computeCorrespondEpilines", vec![(pred!(mut, ["points", "whichImage", "F", "lines"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(points: *const c_void, which_image: i32, f: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// cv::convertPointsFromHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertPointsFromHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// convertPointsHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1334
// ("cv::convertPointsHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::convertPointsToHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertPointsToHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// correctMatches(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1813
// ("cv::correctMatches", vec![(pred!(mut, ["F", "points1", "points2", "newPoints1", "newPoints2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(f: *const c_void, points1: *const c_void, points2: *const c_void, new_points1: *const c_void, new_points2: *const c_void, ocvrs_return: *mut Result<()>);
// decomposeEssentialMat(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1554
// ("cv::decomposeEssentialMat", vec![(pred!(mut, ["E", "R1", "R2", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(e: *const c_void, r1: *const c_void, r2: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// decomposeHomographyMat(InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2108
// ("cv::decomposeHomographyMat", vec![(pred!(mut, ["H", "K", "rotations", "translations", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(h: *const c_void, k: *const c_void, rotations: *const c_void, translations: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix: *const c_void, camera_matrix: *const c_void, rot_matrix: *const c_void, trans_vect: *const c_void, ocvrs_return: *mut Result<()>);
// decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect", "rotMatrixX", "rotMatrixY", "rotMatrixZ", "eulerAngles"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix: *const c_void, camera_matrix: *const c_void, rot_matrix: *const c_void, trans_vect: *const c_void, rot_matrix_x: *const c_void, rot_matrix_y: *const c_void, rot_matrix_z: *const c_void, euler_angles: *const c_void, ocvrs_return: *mut Result<()>);
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:105
// ("cv::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, in_k: *const c_void, in_points: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// cv::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float"]), _)]),
pub fn cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float(image: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, length: f32, ocvrs_return: *mut Result<()>);
// drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, float, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length", "thickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "int"]), _)]),
pub fn cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_int(image: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, length: f32, thickness: i32, ocvrs_return: *mut Result<()>);
// cv::estimateAffine2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_estimateAffine2D_const__InputArrayR_const__InputArrayR(from: *const c_void, to: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimateAffine2D(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2029
// ("cv::estimateAffine2D", vec![(pred!(mut, ["pts1", "pts2", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
pub fn cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(pts1: *const c_void, pts2: *const c_void, inliers: *const c_void, params: *const crate::mod_3d::UsacParams, ocvrs_return: *mut Result<*mut c_void>);
// estimateAffine2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
pub fn cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from: *const c_void, to: *const c_void, inliers: *const c_void, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::estimateAffine3D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_estimateAffine3D_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, out: *const c_void, inliers: *const c_void, ocvrs_return: *mut Result<i32>);
// estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, out: *const c_void, inliers: *const c_void, ransac_threshold: f64, confidence: f64, ocvrs_return: *mut Result<i32>);
// estimateAffine3D(InputArray, InputArray, double *, bool)(InputArray, InputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "scale", "force_rotation"], ["const cv::_InputArray*", "const cv::_InputArray*", "double*", "bool"]), _)]),
pub fn cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_doubleX_bool(src: *const c_void, dst: *const c_void, scale: *mut f64, force_rotation: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::estimateAffinePartial2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR(from: *const c_void, to: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// estimateAffinePartial2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
pub fn cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from: *const c_void, to: *const c_void, inliers: *const c_void, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, out: *const c_void, inliers: *const c_void, ocvrs_return: *mut Result<i32>);
// estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, out: *const c_void, inliers: *const c_void, ransac_threshold: f64, confidence: f64, ocvrs_return: *mut Result<i32>);
// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
pub fn cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float(sampled_point_flags: *const c_void, input_pts: *const c_void, sampled_scale: f32, ocvrs_return: *mut Result<i32>);
// farthestPointSampling(OutputArray, InputArray, float, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "cv::RNG*"]), _)]),
pub fn cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float_float_RNGX(sampled_point_flags: *const c_void, input_pts: *const c_void, sampled_scale: f32, dist_lower_limit: f32, rng: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int(sampled_point_flags: *const c_void, input_pts: *const c_void, sampled_pts_size: i32, ocvrs_return: *mut Result<i32>);
// farthestPointSampling(OutputArray, InputArray, int, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "float", "cv::RNG*"]), _)]),
pub fn cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int_float_RNGX(sampled_point_flags: *const c_void, input_pts: *const c_void, sampled_pts_size: i32, dist_lower_limit: f32, rng: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::filterHomographyDecompByVisibleRefpoints(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(rotations: *const c_void, normals: *const c_void, before_points: *const c_void, after_points: *const c_void, possible_solutions: *const c_void, ocvrs_return: *mut Result<()>);
// filterHomographyDecompByVisibleRefpoints(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions", "pointsMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(rotations: *const c_void, normals: *const c_void, before_points: *const c_void, after_points: *const c_void, possible_solutions: *const c_void, points_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::findEssentialMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR(points1: *const c_void, points2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findEssentialMat(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1531
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "cameraMatrix2", "dist_coeff1", "dist_coeff2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1: *const c_void, points2: *const c_void, camera_matrix1: *const c_void, camera_matrix2: *const c_void, dist_coeff1: *const c_void, dist_coeff2: *const c_void, mask: *const c_void, params: *const crate::mod_3d::UsacParams, ocvrs_return: *mut Result<*mut c_void>);
// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, method: i32, prob: f64, threshold: f64, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int_const__OutputArrayR(points1: *const c_void, points2: *const c_void, focal: f64, pp: *const core::Point2d, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findFundamentalMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR(points1: *const c_void, points2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findFundamentalMat(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points1: *const c_void, points2: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// findFundamentalMat(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1406
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1: *const c_void, points2: *const c_void, mask: *const c_void, params: *const crate::mod_3d::UsacParams, ocvrs_return: *mut Result<*mut c_void>);
// findFundamentalMat(InputArray, InputArray, OutputArray, int, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "method", "ransacReprojThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(points1: *const c_void, points2: *const c_void, mask: *const c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64, ocvrs_return: *mut Result<*mut c_void>);
// findFundamentalMat(InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1: *const c_void, points2: *const c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findFundamentalMat(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int(points1: *const c_void, points2: *const c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32, ocvrs_return: *mut Result<*mut c_void>);
// findFundamentalMat(InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1: *const c_void, points2: *const c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findHomography(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_findHomography_const__InputArrayR_const__InputArrayR(src_points: *const c_void, dst_points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::findHomography(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src_points: *const c_void, dst_points: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// findHomography(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:812
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
pub fn cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(src_points: *const c_void, dst_points: *const c_void, mask: *const c_void, params: *const crate::mod_3d::UsacParams, ocvrs_return: *mut Result<*mut c_void>);
// findHomography(InputArray, InputArray, OutputArray, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "method", "ransacReprojThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
pub fn cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(src_points: *const c_void, dst_points: *const c_void, mask: *const c_void, method: i32, ransac_reproj_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// findHomography(InputArray, InputArray, int, double, OutputArray, const int, const double)(InputArray, InputArray, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "method", "ransacReprojThreshold", "mask", "maxIters", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::_OutputArray*", "const int", "const double"]), _)]),
pub fn cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(src_points: *const c_void, dst_points: *const c_void, method: i32, ransac_reproj_threshold: f64, mask: *const c_void, max_iters: i32, confidence: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::findPlanes(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points3d: *const c_void, normals: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, ocvrs_return: *mut Result<()>);
// findPlanes(InputArray, InputArray, OutputArray, OutputArray, int, int, double, double, double, double, RgbdPlaneMethod)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "double", "double", "cv::RgbdPlaneMethod"]), _)]),
pub fn cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_double_double_double_double_RgbdPlaneMethod(points3d: *const c_void, normals: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, method: crate::mod_3d::RgbdPlaneMethod, ocvrs_return: *mut Result<()>);
// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(undistorted: *const c_void, distorted: *const c_void, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(undistorted: *const c_void, distorted: *const c_void, kundistorted: *const c_void, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
pub fn cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted: *const c_void, distorted: *const c_void, kundistorted: *const c_void, k: *const c_void, d: *const c_void, alpha: f64, ocvrs_return: *mut Result<()>);
// distortPoints(InputArray, OutputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
pub fn cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted: *const c_void, distorted: *const c_void, k: *const c_void, d: *const c_void, alpha: f64, ocvrs_return: *mut Result<()>);
// cv::fisheye::estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, SimpleClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR(k: *const c_void, d: *const c_void, image_size: *const core::Size, r: *const c_void, p: *const c_void, ocvrs_return: *mut Result<()>);
// estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, const Size &, InputArray, OutputArray, double, const Size &, double)(InputArray, InputArray, SimpleClass, InputArray, OutputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P", "balance", "new_size", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "double"]), _)]),
pub fn cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(k: *const c_void, d: *const c_void, image_size: *const core::Size, r: *const c_void, p: *const c_void, balance: f64, new_size: *const core::Size, fov_scale: f64, ocvrs_return: *mut Result<()>);
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2580
// ("cv::fisheye::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "R", "P", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(k: *const c_void, d: *const c_void, r: *const c_void, p: *const c_void, size: *const core::Size, m1type: i32, map1: *const c_void, map2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fisheye::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR(object_points: *const c_void, image_points: *const c_void, affine: *const core::Affine3d, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, affine: *const core::Affine3d, k: *const c_void, d: *const c_void, alpha: f64, jacobian: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fisheye::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(object_points: *const c_void, image_points: *const c_void, rvec: *const c_void, tvec: *const c_void, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, rvec: *const c_void, tvec: *const c_void, k: *const c_void, d: *const c_void, alpha: f64, jacobian: *const c_void, ocvrs_return: *mut Result<()>);
// cv::fisheye::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<bool>);
// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, TermCriteria)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "cv::TermCriteria"]), _)]),
pub fn cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_TermCriteria(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<bool>);
// cv::fisheye::undistortImage(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, const Size &)(InputArray, OutputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "Knew", "new_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
pub fn cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, knew: *const c_void, new_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::fisheye::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
pub fn cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, r: *const c_void, p: *const c_void, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::getDefaultNewCameraMatrix(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix"], ["const cv::_InputArray*"]), _)]),
pub fn cv_getDefaultNewCameraMatrix_const__InputArrayR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDefaultNewCameraMatrix(InputArray, Size, bool)(InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "imgsize", "centerPrincipalPoint"], ["const cv::_InputArray*", "cv::Size", "bool"]), _)]),
pub fn cv_getDefaultNewCameraMatrix_const__InputArrayR_Size_bool(camera_matrix: *const c_void, imgsize: *const core::Size, center_principal_point: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::getOptimalNewCameraMatrix(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
pub fn cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double(camera_matrix: *const c_void, dist_coeffs: *const c_void, image_size: *const core::Size, alpha: f64, ocvrs_return: *mut Result<*mut c_void>);
// getOptimalNewCameraMatrix(InputArray, InputArray, Size, double, Size, Rect *, bool)(InputArray, InputArray, SimpleClass, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha", "newImgSize", "validPixROI", "centerPrincipalPoint"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double", "cv::Size", "cv::Rect*", "bool"]), _)]),
pub fn cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(camera_matrix: *const c_void, dist_coeffs: *const c_void, image_size: *const core::Size, alpha: f64, new_img_size: *const core::Size, valid_pix_roi: *mut core::Rect, center_principal_point: bool, ocvrs_return: *mut Result<*mut c_void>);
// getUndistortRectangles(InputArray, InputArray, InputArray, InputArray, Size, Rect_<double> &, Rect_<double> &)(InputArray, InputArray, InputArray, InputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2380
// ("cv::getUndistortRectangles", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "imgSize", "inner", "outer"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "cv::Rect_<double>*", "cv::Rect_<double>*"]), _)]),
pub fn cv_getUndistortRectangles_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_Rect_LdoubleGR_Rect_LdoubleGR(camera_matrix: *const c_void, dist_coeffs: *const c_void, r: *const c_void, new_camera_matrix: *const c_void, img_size: *const core::Size, inner: *mut core::Rect_<f64>, outer: *mut core::Rect_<f64>, ocvrs_return: *mut Result<()>);
// initInverseRectificationMap(InputArray, InputArray, InputArray, InputArray, const Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2314
// ("cv::initInverseRectificationMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_initInverseRectificationMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(camera_matrix: *const c_void, dist_coeffs: *const c_void, r: *const c_void, new_camera_matrix: *const c_void, size: *const core::Size, m1type: i32, map1: *const c_void, map2: *const c_void, ocvrs_return: *mut Result<()>);
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, Size, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2243
// ("cv::initUndistortRectifyMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_int_const__OutputArrayR_const__OutputArrayR(camera_matrix: *const c_void, dist_coeffs: *const c_void, r: *const c_void, new_camera_matrix: *const c_void, size: *const core::Size, m1type: i32, map1: *const c_void, map2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::initWideAngleProjMap(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR(camera_matrix: *const c_void, dist_coeffs: *const c_void, image_size: *const core::Size, dest_image_width: i32, m1type: i32, map1: *const c_void, map2: *const c_void, ocvrs_return: *mut Result<f32>);
// initWideAngleProjMap(InputArray, InputArray, Size, int, int, OutputArray, OutputArray, enum UndistortTypes, double)(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2", "projType", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::UndistortTypes", "double"]), _)]),
pub fn cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR_UndistortTypes_double(camera_matrix: *const c_void, dist_coeffs: *const c_void, image_size: *const core::Size, dest_image_width: i32, m1type: i32, map1: *const c_void, map2: *const c_void, proj_type: crate::mod_3d::UndistortTypes, alpha: f64, ocvrs_return: *mut Result<f32>);
// cv::loadMesh(InString, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR(filename: *const c_char, vertices: *const c_void, indices: *const c_void, ocvrs_return: *mut Result<()>);
// loadMesh(const String &, OutputArray, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename: *const c_char, vertices: *const c_void, indices: *const c_void, normals: *const c_void, colors: *const c_void, tex_coords: *const c_void, ocvrs_return: *mut Result<()>);
// cv::loadPointCloud(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_loadPointCloud_const_StringR_const__OutputArrayR(filename: *const c_char, vertices: *const c_void, ocvrs_return: *mut Result<()>);
// loadPointCloud(const String &, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_loadPointCloud_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename: *const c_char, vertices: *const c_void, normals: *const c_void, rgb: *const c_void, ocvrs_return: *mut Result<()>);
// matMulDeriv(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:881
// ("cv::matMulDeriv", vec![(pred!(mut, ["A", "B", "dABdA", "dABdB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(a: *const c_void, b: *const c_void, d_a_bd_a: *const c_void, d_a_bd_b: *const c_void, ocvrs_return: *mut Result<()>);
// cv::normalEstimate(OutputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(normals: *const c_void, curvatures: *const c_void, input_pts: *const c_void, nn_idx: *const c_void, ocvrs_return: *mut Result<()>);
// normalEstimate(OutputArray, OutputArray, InputArray, InputArrayOfArrays, int)(OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx", "max_neighbor_num"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(normals: *const c_void, curvatures: *const c_void, input_pts: *const c_void, nn_idx: *const c_void, max_neighbor_num: i32, ocvrs_return: *mut Result<()>);
// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, image_points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, image_points: *const c_void, dpdr: *const c_void, dpdt: *const c_void, ocvrs_return: *mut Result<()>);
// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt", "dpdf", "dpdc", "dpdk", "dpdo", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, image_points: *const c_void, dpdr: *const c_void, dpdt: *const c_void, dpdf: *const c_void, dpdc: *const c_void, dpdk: *const c_void, dpdo: *const c_void, aspect_ratio: f64, ocvrs_return: *mut Result<()>);
// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "jacobian", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points: *const c_void, rvec: *const c_void, tvec: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, image_points: *const c_void, jacobian: *const c_void, aspect_ratio: f64, ocvrs_return: *mut Result<()>);
// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
pub fn cv_randomSampling_const__OutputArrayR_const__InputArrayR_float(sampled_pts: *const c_void, input_pts: *const c_void, sampled_scale: f32, ocvrs_return: *mut Result<()>);
// randomSampling(OutputArray, InputArray, float, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "cv::RNG*"]), _)]),
pub fn cv_randomSampling_const__OutputArrayR_const__InputArrayR_float_RNGX(sampled_pts: *const c_void, input_pts: *const c_void, sampled_scale: f32, rng: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_randomSampling_const__OutputArrayR_const__InputArrayR_int(sampled_pts: *const c_void, input_pts: *const c_void, sampled_pts_size: i32, ocvrs_return: *mut Result<()>);
// randomSampling(OutputArray, InputArray, int, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::RNG*"]), _)]),
pub fn cv_randomSampling_const__OutputArrayR_const__InputArrayR_int_RNGX(sampled_pts: *const c_void, input_pts: *const c_void, sampled_pts_size: i32, rng: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, e: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<i32>);
// recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, int, double, double, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_double_const__InputOutputArrayR(points1: *const c_void, points2: *const c_void, camera_matrix1: *const c_void, dist_coeffs1: *const c_void, camera_matrix2: *const c_void, dist_coeffs2: *const c_void, e: *const c_void, r: *const c_void, t: *const c_void, method: i32, prob: f64, threshold: f64, mask: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e: *const c_void, points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<i32>);
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(e: *const c_void, points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, r: *const c_void, t: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(e: *const c_void, points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, r: *const c_void, t: *const c_void, distance_thresh: f64, ocvrs_return: *mut Result<i32>);
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double, InputOutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh", "mask", "triangulatedPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(e: *const c_void, points1: *const c_void, points2: *const c_void, camera_matrix: *const c_void, r: *const c_void, t: *const c_void, distance_thresh: f64, mask: *const c_void, triangulated_points: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e: *const c_void, points1: *const c_void, points2: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<i32>);
// recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray, double, Point2d, InputOutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, SimpleClass, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t", "focal", "pp", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Point2d", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(e: *const c_void, points1: *const c_void, points2: *const c_void, r: *const c_void, t: *const c_void, focal: f64, pp: *const core::Point2d, mask: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
pub fn cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, ocvrs_return: *mut Result<()>);
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, depth_dilation: bool, ocvrs_return: *mut Result<()>);
// cv::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_: *const c_void, typ: i32, out: *const c_void, ocvrs_return: *mut Result<()>);
// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_: *const c_void, typ: i32, out: *const c_void, depth_factor: f64, ocvrs_return: *mut Result<()>);
// sampsonDistance(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1833
// ("cv::sampsonDistance", vec![(pred!(mut, ["pt1", "pt2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(pt1: *const c_void, pt2: *const c_void, f: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::saveMesh(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR(filename: *const c_char, vertices: *const c_void, indices: *const c_void, ocvrs_return: *mut Result<()>);
// saveMesh(const String &, InputArray, InputArrayOfArrays, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename: *const c_char, vertices: *const c_void, indices: *const c_void, normals: *const c_void, colors: *const c_void, tex_coords: *const c_void, ocvrs_return: *mut Result<()>);
// cv::savePointCloud(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_savePointCloud_const_StringR_const__InputArrayR(filename: *const c_char, vertices: *const c_void, ocvrs_return: *mut Result<()>);
// savePointCloud(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_savePointCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename: *const c_char, vertices: *const c_void, normals: *const c_void, rgb: *const c_void, ocvrs_return: *mut Result<()>);
// solveP3P(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1132
// ("cv::solveP3P", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, ocvrs_return: *mut Result<i32>);
// cv::solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, ocvrs_return: *mut Result<i32>);
// solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, bool, int, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "useExtrinsicGuess", "flags", "rvec", "tvec", "reprojectionError"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, use_extrinsic_guess: bool, flags: i32, rvec: *const c_void, tvec: *const c_void, reprojection_error: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<bool>);
// solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, double, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "confidence", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "double", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, inliers: *const c_void, ocvrs_return: *mut Result<bool>);
// solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
pub fn cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_UsacParamsR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, inliers: *const c_void, params: *const crate::mod_3d::UsacParams, ocvrs_return: *mut Result<bool>);
// cv::solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<()>);
// solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria"]), _)]),
pub fn cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<()>);
// solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria, double)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria", "VVSlambda"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria", "double"]), _)]),
pub fn cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, criteria: *const core::TermCriteria, vv_slambda: f64, ocvrs_return: *mut Result<()>);
// cv::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, ocvrs_return: *mut Result<bool>);
// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
pub fn cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
pub fn cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices: *const c_void, indices: *const c_void, colors: *const c_void, color_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, ocvrs_return: *mut Result<()>);
// triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
pub fn cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices: *const c_void, indices: *const c_void, colors: *const c_void, color_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, settings: *const crate::mod_3d::TriangleRasterizeSettings, ocvrs_return: *mut Result<()>);
// cv::triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
pub fn cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices: *const c_void, indices: *const c_void, depth_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, ocvrs_return: *mut Result<()>);
// triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
pub fn cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices: *const c_void, indices: *const c_void, depth_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, settings: *const crate::mod_3d::TriangleRasterizeSettings, ocvrs_return: *mut Result<()>);
// cv::triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
pub fn cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices: *const c_void, indices: *const c_void, colors: *const c_void, color_buf: *const c_void, depth_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, ocvrs_return: *mut Result<()>);
// triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
pub fn cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices: *const c_void, indices: *const c_void, colors: *const c_void, color_buf: *const c_void, depth_buf: *const c_void, world2cam: *const c_void, fov_y: f64, z_near: f64, z_far: f64, settings: *const crate::mod_3d::TriangleRasterizeSettings, ocvrs_return: *mut Result<()>);
// triangulatePoints(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1794
// ("cv::triangulatePoints", vec![(pred!(mut, ["projMatr1", "projMatr2", "projPoints1", "projPoints2", "points4D"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(proj_matr1: *const c_void, proj_matr2: *const c_void, proj_points1: *const c_void, proj_points2: *const c_void, points4_d: *const c_void, ocvrs_return: *mut Result<()>);
// cv::undistortImagePoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, ocvrs_return: *mut Result<()>);
// undistortImagePoints(InputArray, OutputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "unnamed"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
pub fn cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, unnamed: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, ocvrs_return: *mut Result<()>);
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
pub fn cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, r: *const c_void, p: *const c_void, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::undistort(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, ocvrs_return: *mut Result<()>);
// undistort(InputArray, OutputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "newCameraMatrix"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, new_camera_matrix: *const c_void, ocvrs_return: *mut Result<()>);
// voxelGridSampling(OutputArray, InputArray, float, float, float)(OutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:185
// ("cv::voxelGridSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "length", "width", "height"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "float"]), _)]),
pub fn cv_voxelGridSampling_const__OutputArrayR_const__InputArrayR_float_float_float(sampled_point_flags: *const c_void, input_pts: *const c_void, length: f32, width: f32, height: f32, ocvrs_return: *mut Result<i32>);
// cv::warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth: *const c_void, image: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, ocvrs_return: *mut Result<()>);
// warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix", "warpedDepth", "warpedImage", "warpedMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(depth: *const c_void, image: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, warped_depth: *const c_void, warped_image: *const c_void, warped_mask: *const c_void, ocvrs_return: *mut Result<()>);
// optimize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:714
// ("cv::LevMarq::optimize", vec![(pred!(mut, [], []), _)]),
pub fn cv_LevMarq_optimize(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// run(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:729
// ("cv::LevMarq::run", vec![(pred!(mut, ["param"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_LevMarq_run_const__InputOutputArrayR(instance: *mut c_void, param: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::LevMarq::delete() generated
// ("cv::LevMarq::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LevMarq_delete(instance: *mut c_void);
// Report(bool, int, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:530
// ("cv::LevMarq::Report::Report", vec![(pred!(mut, ["isFound", "nIters", "finalEnergy"], ["bool", "int", "double"]), _)]),
pub fn cv_LevMarq_Report_Report_bool_int_double(is_found: bool, n_iters: i32, final_energy: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::LevMarq::Report::found() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
// ("cv::LevMarq::Report::found", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Report_propFound_const(instance: *const c_void) -> bool;
// cv::LevMarq::Report::setFound(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
// ("cv::LevMarq::Report::setFound", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Report_propFound_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Report::iters() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
// ("cv::LevMarq::Report::iters", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Report_propIters_const(instance: *const c_void) -> i32;
// cv::LevMarq::Report::setIters(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
// ("cv::LevMarq::Report::setIters", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LevMarq_Report_propIters_const_int(instance: *mut c_void, val: i32);
// cv::LevMarq::Report::energy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
// ("cv::LevMarq::Report::energy", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Report_propEnergy_const(instance: *const c_void) -> f64;
// cv::LevMarq::Report::setEnergy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
// ("cv::LevMarq::Report::setEnergy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Report_propEnergy_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Report::delete() generated
// ("cv::LevMarq::Report::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LevMarq_Report_delete(instance: *mut c_void);
// Settings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:551
// ("cv::LevMarq::Settings::Settings", vec![(pred!(mut, [], []), _)]),
pub fn cv_LevMarq_Settings_Settings(ocvrs_return: *mut Result<*mut c_void>);
// setJacobiScaling(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:553
// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setJacobiScaling_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setUpDouble(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:554
// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setUpDouble_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setUseStepQuality(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:555
// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setUseStepQuality_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setClampDiagonal(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:556
// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setClampDiagonal_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setStepNormInf(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:557
// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setStepNormInf_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setCheckRelEnergyChange(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:558
// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setCheckRelEnergyChange_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setCheckMinGradient(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:559
// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setCheckMinGradient_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setCheckStepNorm(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:560
// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setCheckStepNorm_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setGeodesic(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:561
// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_LevMarq_Settings_setGeodesic_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<*mut c_void>);
// setHGeo(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:562
// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setHGeo_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setGeoScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:563
// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setGeoScale_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setStepNormTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:564
// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setStepNormTolerance_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setRelEnergyDeltaTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:565
// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setRelEnergyDeltaTolerance_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setMinGradientTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:566
// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setMinGradientTolerance_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setSmallEnergyTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:567
// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setSmallEnergyTolerance_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:568
// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["v"], ["int"]), _)]),
pub fn cv_LevMarq_Settings_setMaxIterations_int(instance: *mut c_void, v: i32, ocvrs_return: *mut Result<*mut c_void>);
// setInitialLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:569
// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setInitialLambda_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setInitialLmUpFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:570
// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setInitialLmUpFactor_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// setInitialLmDownFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:571
// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
pub fn cv_LevMarq_Settings_setInitialLmDownFactor_double(instance: *mut c_void, v: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::LevMarq::Settings::jacobiScaling() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
// ("cv::LevMarq::Settings::jacobiScaling", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propJacobiScaling_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setJacobiScaling(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propJacobiScaling_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::upDouble() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
// ("cv::LevMarq::Settings::upDouble", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propUpDouble_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setUpDouble(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propUpDouble_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::useStepQuality() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
// ("cv::LevMarq::Settings::useStepQuality", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propUseStepQuality_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setUseStepQuality(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propUseStepQuality_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::clampDiagonal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
// ("cv::LevMarq::Settings::clampDiagonal", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propClampDiagonal_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setClampDiagonal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propClampDiagonal_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::stepNormInf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
// ("cv::LevMarq::Settings::stepNormInf", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propStepNormInf_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setStepNormInf(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propStepNormInf_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::checkRelEnergyChange() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
// ("cv::LevMarq::Settings::checkRelEnergyChange", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propCheckRelEnergyChange_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setCheckRelEnergyChange(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propCheckRelEnergyChange_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::checkMinGradient() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
// ("cv::LevMarq::Settings::checkMinGradient", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propCheckMinGradient_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setCheckMinGradient(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propCheckMinGradient_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::checkStepNorm() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
// ("cv::LevMarq::Settings::checkStepNorm", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propCheckStepNorm_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setCheckStepNorm(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propCheckStepNorm_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::geodesic() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
// ("cv::LevMarq::Settings::geodesic", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propGeodesic_const(instance: *const c_void) -> bool;
// cv::LevMarq::Settings::setGeodesic(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LevMarq_Settings_propGeodesic_const_bool(instance: *mut c_void, val: bool);
// cv::LevMarq::Settings::hGeo() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
// ("cv::LevMarq::Settings::hGeo", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propHGeo_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setHGeo(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propHGeo_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::geoScale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
// ("cv::LevMarq::Settings::geoScale", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propGeoScale_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setGeoScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propGeoScale_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::stepNormTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
// ("cv::LevMarq::Settings::stepNormTolerance", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propStepNormTolerance_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setStepNormTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propStepNormTolerance_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::relEnergyDeltaTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
// ("cv::LevMarq::Settings::relEnergyDeltaTolerance", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setRelEnergyDeltaTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::minGradientTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
// ("cv::LevMarq::Settings::minGradientTolerance", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propMinGradientTolerance_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setMinGradientTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propMinGradientTolerance_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::smallEnergyTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
// ("cv::LevMarq::Settings::smallEnergyTolerance", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propSmallEnergyTolerance_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setSmallEnergyTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propSmallEnergyTolerance_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::maxIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
// ("cv::LevMarq::Settings::maxIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propMaxIterations_const(instance: *const c_void) -> u32;
// cv::LevMarq::Settings::setMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
pub fn cv_LevMarq_Settings_propMaxIterations_const_unsigned_int(instance: *mut c_void, val: u32);
// cv::LevMarq::Settings::initialLambda() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
// ("cv::LevMarq::Settings::initialLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propInitialLambda_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setInitialLambda(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propInitialLambda_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::initialLmUpFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
// ("cv::LevMarq::Settings::initialLmUpFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propInitialLmUpFactor_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setInitialLmUpFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propInitialLmUpFactor_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::initialLmDownFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
// ("cv::LevMarq::Settings::initialLmDownFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_LevMarq_Settings_propInitialLmDownFactor_const(instance: *const c_void) -> f64;
// cv::LevMarq::Settings::setInitialLmDownFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_LevMarq_Settings_propInitialLmDownFactor_const_double(instance: *mut c_void, val: f64);
// cv::LevMarq::Settings::delete() generated
// ("cv::LevMarq::Settings::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LevMarq_Settings_delete(instance: *mut c_void);
// Octree()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2691
// ("cv::Octree::Octree", vec![(pred!(mut, [], []), _)]),
pub fn cv_Octree_Octree(ocvrs_return: *mut Result<*mut c_void>);
// createWithDepth(int, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size", "origin", "withColors"], ["int", "double", "const cv::Point3f*", "bool"]), _)]),
pub fn cv_Octree_createWithDepth_int_double_const_Point3fR_bool(max_depth: i32, size: f64, origin: *const core::Point3f, with_colors: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::Octree::createWithDepth(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size"], ["int", "double"]), _)]),
pub fn cv_Octree_createWithDepth_int_double(max_depth: i32, size: f64, ocvrs_return: *mut Result<*mut c_void>);
// createWithDepth(int, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud", "colors"], ["int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Octree_createWithDepth_int_const__InputArrayR_const__InputArrayR(max_depth: i32, point_cloud: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Octree::createWithDepth(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud"], ["int", "const cv::_InputArray*"]), _)]),
pub fn cv_Octree_createWithDepth_int_const__InputArrayR(max_depth: i32, point_cloud: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createWithResolution(double, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size", "origin", "withColors"], ["double", "double", "const cv::Point3f*", "bool"]), _)]),
pub fn cv_Octree_createWithResolution_double_double_const_Point3fR_bool(resolution: f64, size: f64, origin: *const core::Point3f, with_colors: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::Octree::createWithResolution(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size"], ["double", "double"]), _)]),
pub fn cv_Octree_createWithResolution_double_double(resolution: f64, size: f64, ocvrs_return: *mut Result<*mut c_void>);
// createWithResolution(double, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud", "colors"], ["double", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Octree_createWithResolution_double_const__InputArrayR_const__InputArrayR(resolution: f64, point_cloud: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Octree::createWithResolution(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud"], ["double", "const cv::_InputArray*"]), _)]),
pub fn cv_Octree_createWithResolution_double_const__InputArrayR(resolution: f64, point_cloud: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// insertPoint(const Point3f &, const Point3f &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point", "color"], ["const cv::Point3f*", "const cv::Point3f*"]), _)]),
pub fn cv_Octree_insertPoint_const_Point3fR_const_Point3fR(instance: *mut c_void, point: *const core::Point3f, color: *const core::Point3f, ocvrs_return: *mut Result<bool>);
// cv::Octree::insertPoint(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
pub fn cv_Octree_insertPoint_const_Point3fR(instance: *mut c_void, point: *const core::Point3f, ocvrs_return: *mut Result<bool>);
// isPointInBound(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2752
// ("cv::Octree::isPointInBound", vec![(pred!(const, ["point"], ["const cv::Point3f*"]), _)]),
pub fn cv_Octree_isPointInBound_const_const_Point3fR(instance: *const c_void, point: *const core::Point3f, ocvrs_return: *mut Result<bool>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2755
// ("cv::Octree::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_Octree_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2761
// ("cv::Octree::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_Octree_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// deletePoint(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2771
// ("cv::Octree::deletePoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
pub fn cv_Octree_deletePoint_const_Point3fR(instance: *mut c_void, point: *const core::Point3f, ocvrs_return: *mut Result<bool>);
// getPointCloudByOctree(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud", "restoredColor"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_getPointCloudByOctree_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, restored_point_cloud: *const c_void, restored_color: *const c_void, ocvrs_return: *mut Result<()>);
// cv::Octree::getPointCloudByOctree(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_getPointCloudByOctree_const__OutputArrayR(instance: *mut c_void, restored_point_cloud: *const c_void, ocvrs_return: *mut Result<()>);
// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, radius: f32, points: *const c_void, square_dists: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::Octree::radiusNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points"], ["const cv::Point3f*", "float", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, radius: f32, points: *const c_void, ocvrs_return: *mut Result<i32>);
// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2810
// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "colors", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, radius: f32, points: *const c_void, colors: *const c_void, square_dists: *const c_void, ocvrs_return: *mut Result<i32>);
// KNNSearch(const Point3f &, const int, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, k: i32, points: *const c_void, square_dists: *const c_void, ocvrs_return: *mut Result<()>);
// cv::Octree::KNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, k: i32, points: *const c_void, ocvrs_return: *mut Result<()>);
// KNNSearch(const Point3f &, const int, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2836
// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "colors", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, query: *const core::Point3f, k: i32, points: *const c_void, colors: *const c_void, square_dists: *const c_void, ocvrs_return: *mut Result<()>);
// cv::Octree::delete() generated
// ("cv::Octree::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Octree_delete(instance: *mut c_void);
// Odometry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:41
// ("cv::Odometry::Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_Odometry_Odometry(ocvrs_return: *mut Result<*mut c_void>);
// Odometry(OdometryType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:42
// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype"], ["cv::OdometryType"]), _)]),
pub fn cv_Odometry_Odometry_OdometryType(otype: crate::mod_3d::OdometryType, ocvrs_return: *mut Result<*mut c_void>);
// Odometry(OdometryType, const OdometrySettings &, OdometryAlgoType)(Enum, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:43
// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype", "settings", "algtype"], ["cv::OdometryType", "const cv::OdometrySettings*", "cv::OdometryAlgoType"]), _)]),
pub fn cv_Odometry_Odometry_OdometryType_const_OdometrySettingsR_OdometryAlgoType(otype: crate::mod_3d::OdometryType, settings: *const c_void, algtype: crate::mod_3d::OdometryAlgoType, ocvrs_return: *mut Result<*mut c_void>);
// prepareFrame(OdometryFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:49
// ("cv::Odometry::prepareFrame", vec![(pred!(const, ["frame"], ["cv::OdometryFrame*"]), _)]),
pub fn cv_Odometry_prepareFrame_const_OdometryFrameR(instance: *const c_void, frame: *mut c_void, ocvrs_return: *mut Result<()>);
// prepareFrames(OdometryFrame &, OdometryFrame &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:55
// ("cv::Odometry::prepareFrames", vec![(pred!(const, ["srcFrame", "dstFrame"], ["cv::OdometryFrame*", "cv::OdometryFrame*"]), _)]),
pub fn cv_Odometry_prepareFrames_const_OdometryFrameR_OdometryFrameR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(const OdometryFrame &, const OdometryFrame &, OutputArray)(TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:69
// ("cv::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["const cv::OdometryFrame*", "const cv::OdometryFrame*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Odometry_compute_const_const_OdometryFrameR_const_OdometryFrameR_const__OutputArrayR(instance: *const c_void, src_frame: *const c_void, dst_frame: *const c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:82
// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "dstDepth", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src_depth: *const c_void, dst_depth: *const c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// compute(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:97
// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "srcRGB", "dstDepth", "dstRGB", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src_depth: *const c_void, src_rgb: *const c_void, dst_depth: *const c_void, dst_rgb: *const c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:104
// ("cv::Odometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
pub fn cv_Odometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Odometry::delete() generated
// ("cv::Odometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Odometry_delete(instance: *mut c_void);
// OdometryFrame(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["depth", "image", "mask", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_OdometryFrame_OdometryFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth: *const c_void, image: *const c_void, mask: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::OdometryFrame::OdometryFrame() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_OdometryFrame_OdometryFrame(ocvrs_return: *mut Result<*mut c_void>);
// getImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:57
// ("cv::OdometryFrame::getImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getImage_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// getGrayImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:63
// ("cv::OdometryFrame::getGrayImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getGrayImage_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:69
// ("cv::OdometryFrame::getDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getDepth_const_const__OutputArrayR(instance: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<()>);
// getProcessedDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:75
// ("cv::OdometryFrame::getProcessedDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getProcessedDepth_const_const__OutputArrayR(instance: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<()>);
// getMask(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:81
// ("cv::OdometryFrame::getMask", vec![(pred!(const, ["mask"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getMask_const_const__OutputArrayR(instance: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:87
// ("cv::OdometryFrame::getNormals", vec![(pred!(const, ["normals"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometryFrame_getNormals_const_const__OutputArrayR(instance: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:93
// ("cv::OdometryFrame::getPyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometryFrame_getPyramidLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getPyramidAt(OutputArray, OdometryFramePyramidType, size_t)(OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:102
// ("cv::OdometryFrame::getPyramidAt", vec![(pred!(const, ["img", "pyrType", "level"], ["const cv::_OutputArray*", "cv::OdometryFramePyramidType", "size_t"]), _)]),
pub fn cv_OdometryFrame_getPyramidAt_const_const__OutputArrayR_OdometryFramePyramidType_size_t(instance: *const c_void, img: *const c_void, pyr_type: crate::mod_3d::OdometryFramePyramidType, level: size_t, ocvrs_return: *mut Result<()>);
// cv::OdometryFrame::implicitClone() generated
// ("cv::OdometryFrame::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometryFrame_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::OdometryFrame::delete() generated
// ("cv::OdometryFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_OdometryFrame_delete(instance: *mut c_void);
// OdometrySettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:16
// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, [], []), _)]),
pub fn cv_OdometrySettings_OdometrySettings(ocvrs_return: *mut Result<*mut c_void>);
// OdometrySettings(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:17
// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
pub fn cv_OdometrySettings_OdometrySettings_const_OdometrySettingsR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:18
// ("cv::OdometrySettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
pub fn cv_OdometrySettings_operatorST_const_OdometrySettingsR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// setCameraMatrix(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:20
// ("cv::OdometrySettings::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_OdometrySettings_setCameraMatrix_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getCameraMatrix(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:21
// ("cv::OdometrySettings::getCameraMatrix", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometrySettings_getCameraMatrix_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setIterCounts(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:22
// ("cv::OdometrySettings::setIterCounts", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_OdometrySettings_setIterCounts_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getIterCounts(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:23
// ("cv::OdometrySettings::getIterCounts", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometrySettings_getIterCounts_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setMinDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:25
// ("cv::OdometrySettings::setMinDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMinDepth_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:26
// ("cv::OdometrySettings::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:27
// ("cv::OdometrySettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMaxDepth_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:28
// ("cv::OdometrySettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxDepthDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:29
// ("cv::OdometrySettings::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMaxDepthDiff_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:30
// ("cv::OdometrySettings::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxPointsPart(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:31
// ("cv::OdometrySettings::setMaxPointsPart", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMaxPointsPart_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:32
// ("cv::OdometrySettings::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSobelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:34
// ("cv::OdometrySettings::setSobelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_OdometrySettings_setSobelSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getSobelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:35
// ("cv::OdometrySettings::getSobelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getSobelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSobelScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:36
// ("cv::OdometrySettings::setSobelScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_OdometrySettings_setSobelScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSobelScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:37
// ("cv::OdometrySettings::getSobelScale", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getSobelScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNormalWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:39
// ("cv::OdometrySettings::setNormalWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_OdometrySettings_setNormalWinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getNormalWinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:40
// ("cv::OdometrySettings::getNormalWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getNormalWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNormalDiffThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:41
// ("cv::OdometrySettings::setNormalDiffThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setNormalDiffThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getNormalDiffThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:42
// ("cv::OdometrySettings::getNormalDiffThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getNormalDiffThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setNormalMethod(RgbdNormals::RgbdNormalsMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:43
// ("cv::OdometrySettings::setNormalMethod", vec![(pred!(mut, ["nm"], ["cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
pub fn cv_OdometrySettings_setNormalMethod_RgbdNormalsMethod(instance: *mut c_void, nm: crate::mod_3d::RgbdNormals_RgbdNormalsMethod, ocvrs_return: *mut Result<()>);
// getNormalMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:44
// ("cv::OdometrySettings::getNormalMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getNormalMethod_const(instance: *const c_void, ocvrs_return: *mut Result<crate::mod_3d::RgbdNormals_RgbdNormalsMethod>);
// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:46
// ("cv::OdometrySettings::setAngleThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setAngleThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:47
// ("cv::OdometrySettings::getAngleThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getAngleThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxTranslation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:48
// ("cv::OdometrySettings::setMaxTranslation", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMaxTranslation_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:49
// ("cv::OdometrySettings::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxRotation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:50
// ("cv::OdometrySettings::setMaxRotation", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMaxRotation_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:51
// ("cv::OdometrySettings::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinGradientMagnitude(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:53
// ("cv::OdometrySettings::setMinGradientMagnitude", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_OdometrySettings_setMinGradientMagnitude_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitude()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:54
// ("cv::OdometrySettings::getMinGradientMagnitude", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_getMinGradientMagnitude_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinGradientMagnitudes(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:55
// ("cv::OdometrySettings::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_OdometrySettings_setMinGradientMagnitudes_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitudes(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:56
// ("cv::OdometrySettings::getMinGradientMagnitudes", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_OdometrySettings_getMinGradientMagnitudes_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// cv::OdometrySettings::implicitClone() generated
// ("cv::OdometrySettings::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_OdometrySettings_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::OdometrySettings::delete() generated
// ("cv::OdometrySettings::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_OdometrySettings_delete(instance: *mut c_void);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:303
// ("cv::RegionGrowing3D::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_RegionGrowing3D_create(ocvrs_return: *mut Result<*mut c_void>);
// segment(OutputArrayOfArrays, OutputArray, InputArray, InputArray, InputArrayOfArrays)(OutputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:332
// ("cv::RegionGrowing3D::segment", vec![(pred!(mut, ["regions_idx", "labels", "input_pts", "normals", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_RegionGrowing3D_segment_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, regions_idx: *const c_void, labels: *const c_void, input_pts: *const c_void, normals: *const c_void, nn_idx: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:339
// ("cv::RegionGrowing3D::setMinSize", vec![(pred!(mut, ["min_size"], ["int"]), _)]),
pub fn cv_RegionGrowing3D_setMinSize_int(instance: *mut c_void, min_size: i32, ocvrs_return: *mut Result<()>);
// getMinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:342
// ("cv::RegionGrowing3D::getMinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:346
// ("cv::RegionGrowing3D::setMaxSize", vec![(pred!(mut, ["max_size"], ["int"]), _)]),
pub fn cv_RegionGrowing3D_setMaxSize_int(instance: *mut c_void, max_size: i32, ocvrs_return: *mut Result<()>);
// getMaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:349
// ("cv::RegionGrowing3D::getMaxSize", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getMaxSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSmoothModeFlag(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:354
// ("cv::RegionGrowing3D::setSmoothModeFlag", vec![(pred!(mut, ["smooth_mode"], ["bool"]), _)]),
pub fn cv_RegionGrowing3D_setSmoothModeFlag_bool(instance: *mut c_void, smooth_mode: bool, ocvrs_return: *mut Result<()>);
// getSmoothModeFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:357
// ("cv::RegionGrowing3D::getSmoothModeFlag", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getSmoothModeFlag_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setSmoothnessThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:361
// ("cv::RegionGrowing3D::setSmoothnessThreshold", vec![(pred!(mut, ["smoothness_thr"], ["double"]), _)]),
pub fn cv_RegionGrowing3D_setSmoothnessThreshold_double(instance: *mut c_void, smoothness_thr: f64, ocvrs_return: *mut Result<()>);
// getSmoothnessThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:364
// ("cv::RegionGrowing3D::getSmoothnessThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getSmoothnessThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setCurvatureThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:369
// ("cv::RegionGrowing3D::setCurvatureThreshold", vec![(pred!(mut, ["curvature_thr"], ["double"]), _)]),
pub fn cv_RegionGrowing3D_setCurvatureThreshold_double(instance: *mut c_void, curvature_thr: f64, ocvrs_return: *mut Result<()>);
// getCurvatureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:372
// ("cv::RegionGrowing3D::getCurvatureThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getCurvatureThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxNumberOfNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:376
// ("cv::RegionGrowing3D::setMaxNumberOfNeighbors", vec![(pred!(mut, ["max_neighbor_num"], ["int"]), _)]),
pub fn cv_RegionGrowing3D_setMaxNumberOfNeighbors_int(instance: *mut c_void, max_neighbor_num: i32, ocvrs_return: *mut Result<()>);
// getMaxNumberOfNeighbors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:379
// ("cv::RegionGrowing3D::getMaxNumberOfNeighbors", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getMaxNumberOfNeighbors_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumberOfRegions(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:383
// ("cv::RegionGrowing3D::setNumberOfRegions", vec![(pred!(mut, ["region_num"], ["int"]), _)]),
pub fn cv_RegionGrowing3D_setNumberOfRegions_int(instance: *mut c_void, region_num: i32, ocvrs_return: *mut Result<()>);
// getNumberOfRegions()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:386
// ("cv::RegionGrowing3D::getNumberOfRegions", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getNumberOfRegions_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNeedSort(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:389
// ("cv::RegionGrowing3D::setNeedSort", vec![(pred!(mut, ["need_sort"], ["bool"]), _)]),
pub fn cv_RegionGrowing3D_setNeedSort_bool(instance: *mut c_void, need_sort: bool, ocvrs_return: *mut Result<()>);
// getNeedSort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:392
// ("cv::RegionGrowing3D::getNeedSort", vec![(pred!(const, [], []), _)]),
pub fn cv_RegionGrowing3D_getNeedSort_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setSeeds(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:398
// ("cv::RegionGrowing3D::setSeeds", vec![(pred!(mut, ["seeds"], ["const cv::_InputArray*"]), _)]),
pub fn cv_RegionGrowing3D_setSeeds_const__InputArrayR(instance: *mut c_void, seeds: *const c_void, ocvrs_return: *mut Result<()>);
// getSeeds(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:401
// ("cv::RegionGrowing3D::getSeeds", vec![(pred!(const, ["seeds"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_RegionGrowing3D_getSeeds_const_const__OutputArrayR(instance: *const c_void, seeds: *const c_void, ocvrs_return: *mut Result<()>);
// setCurvatures(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:404
// ("cv::RegionGrowing3D::setCurvatures", vec![(pred!(mut, ["curvatures"], ["const cv::_InputArray*"]), _)]),
pub fn cv_RegionGrowing3D_setCurvatures_const__InputArrayR(instance: *mut c_void, curvatures: *const c_void, ocvrs_return: *mut Result<()>);
// getCurvatures(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:407
// ("cv::RegionGrowing3D::getCurvatures", vec![(pred!(const, ["curvatures"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_RegionGrowing3D_getCurvatures_const_const__OutputArrayR(instance: *const c_void, curvatures: *const c_void, ocvrs_return: *mut Result<()>);
// cv::RegionGrowing3D::delete() generated
// ("cv::RegionGrowing3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RegionGrowing3D_delete(instance: *mut c_void);
// create(int, int, int, InputArray, int, float, RgbdNormals::RgbdNormalsMethod)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
// ("cv::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "diff_threshold", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "float", "cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
pub fn cv_RgbdNormals_create_int_int_int_const__InputArrayR_int_float_RgbdNormalsMethod(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, diff_threshold: f32, method: crate::mod_3d::RgbdNormals_RgbdNormalsMethod, ocvrs_return: *mut Result<*mut c_void>);
// cv::RgbdNormals::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
// ("cv::RgbdNormals::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_RgbdNormals_create(ocvrs_return: *mut Result<*mut c_void>);
// apply(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:57
// ("cv::RgbdNormals::apply", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_RgbdNormals_apply_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// cache()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:62
// ("cv::RgbdNormals::cache", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_cache_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// getRows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:64
// ("cv::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_getRows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:65
// ("cv::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_RgbdNormals_setRows_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:66
// ("cv::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_getCols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:67
// ("cv::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_RgbdNormals_setCols_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:68
// ("cv::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:69
// ("cv::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_RgbdNormals_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:70
// ("cv::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getK(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:71
// ("cv::RgbdNormals::getK", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_RgbdNormals_getK_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setK(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:72
// ("cv::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_RgbdNormals_setK_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:73
// ("cv::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_RgbdNormals_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<crate::mod_3d::RgbdNormals_RgbdNormalsMethod>);
// cv::RgbdNormals::delete() generated
// ("cv::RgbdNormals::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RgbdNormals_delete(instance: *mut c_void);
// create(SacModelType, SacMethod, double, int)(Enum, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
// ("cv::SACSegmentation::create", vec![(pred!(mut, ["sac_model_type", "sac_method", "threshold", "max_iterations"], ["cv::SacModelType", "cv::SacMethod", "double", "int"]), _)]),
pub fn cv_SACSegmentation_create_SacModelType_SacMethod_double_int(sac_model_type: crate::mod_3d::SacModelType, sac_method: crate::mod_3d::SacMethod, threshold: f64, max_iterations: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::SACSegmentation::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
// ("cv::SACSegmentation::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_SACSegmentation_create(ocvrs_return: *mut Result<*mut c_void>);
// segment(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:98
// ("cv::SACSegmentation::segment", vec![(pred!(mut, ["input_pts", "labels", "models_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SACSegmentation_segment_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, input_pts: *const c_void, labels: *const c_void, models_coefficients: *const c_void, ocvrs_return: *mut Result<i32>);
// setSacModelType(SacModelType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:103
// ("cv::SACSegmentation::setSacModelType", vec![(pred!(mut, ["sac_model_type"], ["cv::SacModelType"]), _)]),
pub fn cv_SACSegmentation_setSacModelType_SacModelType(instance: *mut c_void, sac_model_type: crate::mod_3d::SacModelType, ocvrs_return: *mut Result<()>);
// getSacModelType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:106
// ("cv::SACSegmentation::getSacModelType", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getSacModelType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::mod_3d::SacModelType>);
// setSacMethodType(SacMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:109
// ("cv::SACSegmentation::setSacMethodType", vec![(pred!(mut, ["sac_method"], ["cv::SacMethod"]), _)]),
pub fn cv_SACSegmentation_setSacMethodType_SacMethod(instance: *mut c_void, sac_method: crate::mod_3d::SacMethod, ocvrs_return: *mut Result<()>);
// getSacMethodType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:112
// ("cv::SACSegmentation::getSacMethodType", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getSacMethodType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::mod_3d::SacMethod>);
// setDistanceThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:116
// ("cv::SACSegmentation::setDistanceThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
pub fn cv_SACSegmentation_setDistanceThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result<()>);
// getDistanceThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:119
// ("cv::SACSegmentation::getDistanceThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getDistanceThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRadiusLimits(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:123
// ("cv::SACSegmentation::setRadiusLimits", vec![(pred!(mut, ["radius_min", "radius_max"], ["double", "double"]), _)]),
pub fn cv_SACSegmentation_setRadiusLimits_double_double(instance: *mut c_void, radius_min: f64, radius_max: f64, ocvrs_return: *mut Result<()>);
// getRadiusLimits(double &, double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:126
// ("cv::SACSegmentation::getRadiusLimits", vec![(pred!(const, ["radius_min", "radius_max"], ["double*", "double*"]), _)]),
pub fn cv_SACSegmentation_getRadiusLimits_const_doubleR_doubleR(instance: *const c_void, radius_min: *mut f64, radius_max: *mut f64, ocvrs_return: *mut Result<()>);
// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:129
// ("cv::SACSegmentation::setMaxIterations", vec![(pred!(mut, ["max_iterations"], ["int"]), _)]),
pub fn cv_SACSegmentation_setMaxIterations_int(instance: *mut c_void, max_iterations: i32, ocvrs_return: *mut Result<()>);
// getMaxIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:132
// ("cv::SACSegmentation::getMaxIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getMaxIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setConfidence(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:135
// ("cv::SACSegmentation::setConfidence", vec![(pred!(mut, ["confidence"], ["double"]), _)]),
pub fn cv_SACSegmentation_setConfidence_double(instance: *mut c_void, confidence: f64, ocvrs_return: *mut Result<()>);
// getConfidence()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:138
// ("cv::SACSegmentation::getConfidence", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getConfidence_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNumberOfModelsExpected(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:141
// ("cv::SACSegmentation::setNumberOfModelsExpected", vec![(pred!(mut, ["number_of_models_expected"], ["int"]), _)]),
pub fn cv_SACSegmentation_setNumberOfModelsExpected_int(instance: *mut c_void, number_of_models_expected: i32, ocvrs_return: *mut Result<()>);
// getNumberOfModelsExpected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:144
// ("cv::SACSegmentation::getNumberOfModelsExpected", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getNumberOfModelsExpected_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setParallel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:148
// ("cv::SACSegmentation::setParallel", vec![(pred!(mut, ["is_parallel"], ["bool"]), _)]),
pub fn cv_SACSegmentation_setParallel_bool(instance: *mut c_void, is_parallel: bool, ocvrs_return: *mut Result<()>);
// isParallel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:151
// ("cv::SACSegmentation::isParallel", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_isParallel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setRandomGeneratorState(uint64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:154
// ("cv::SACSegmentation::setRandomGeneratorState", vec![(pred!(mut, ["rng_state"], ["uint64_t"]), _)]),
pub fn cv_SACSegmentation_setRandomGeneratorState_uint64_t(instance: *mut c_void, rng_state: u64, ocvrs_return: *mut Result<()>);
// getRandomGeneratorState()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:157
// ("cv::SACSegmentation::getRandomGeneratorState", vec![(pred!(const, [], []), _)]),
pub fn cv_SACSegmentation_getRandomGeneratorState_const(instance: *const c_void, ocvrs_return: *mut Result<u64>);
// cv::SACSegmentation::delete() generated
// ("cv::SACSegmentation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SACSegmentation_delete(instance: *mut c_void);
// TriangleRasterizeSettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2945
// ("cv::TriangleRasterizeSettings::TriangleRasterizeSettings", vec![(pred!(mut, [], []), _)]),
pub fn cv_TriangleRasterizeSettings_TriangleRasterizeSettings(ocvrs_return: *mut Result<crate::mod_3d::TriangleRasterizeSettings>);
// setShadingType(TriangleShadingType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2947
// ("cv::TriangleRasterizeSettings::setShadingType", vec![(pred!(mut, ["st"], ["cv::TriangleShadingType"]), _)]),
pub fn cv_TriangleRasterizeSettings_setShadingType_TriangleShadingType(instance: *const crate::mod_3d::TriangleRasterizeSettings, st: crate::mod_3d::TriangleShadingType, ocvrs_return: *mut Result<crate::mod_3d::TriangleRasterizeSettings>);
// setCullingMode(TriangleCullingMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2948
// ("cv::TriangleRasterizeSettings::setCullingMode", vec![(pred!(mut, ["cm"], ["cv::TriangleCullingMode"]), _)]),
pub fn cv_TriangleRasterizeSettings_setCullingMode_TriangleCullingMode(instance: *const crate::mod_3d::TriangleRasterizeSettings, cm: crate::mod_3d::TriangleCullingMode, ocvrs_return: *mut Result<crate::mod_3d::TriangleRasterizeSettings>);
// setGlCompatibleMode(TriangleGlCompatibleMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2949
// ("cv::TriangleRasterizeSettings::setGlCompatibleMode", vec![(pred!(mut, ["gm"], ["cv::TriangleGlCompatibleMode"]), _)]),
pub fn cv_TriangleRasterizeSettings_setGlCompatibleMode_TriangleGlCompatibleMode(instance: *const crate::mod_3d::TriangleRasterizeSettings, gm: crate::mod_3d::TriangleGlCompatibleMode, ocvrs_return: *mut Result<crate::mod_3d::TriangleRasterizeSettings>);
// UsacParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:432
// ("cv::UsacParams::UsacParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_UsacParams_UsacParams(ocvrs_return: *mut Result<crate::mod_3d::UsacParams>);
// Volume(VolumeType, const VolumeSettings &)(Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
// ("cv::Volume::Volume", vec![(pred!(mut, ["vtype", "settings"], ["cv::VolumeType", "const cv::VolumeSettings*"]), _)]),
pub fn cv_Volume_Volume_VolumeType_const_VolumeSettingsR(vtype: crate::mod_3d::VolumeType, settings: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Volume::Volume() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
// ("cv::Volume::Volume", vec![(pred!(mut, [], []), _)]),
pub fn cv_Volume_Volume(ocvrs_return: *mut Result<*mut c_void>);
// integrate(const OdometryFrame &, InputArray)(TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:35
// ("cv::Volume::integrate", vec![(pred!(mut, ["frame", "pose"], ["const cv::OdometryFrame*", "const cv::_InputArray*"]), _)]),
pub fn cv_Volume_integrate_const_OdometryFrameR_const__InputArrayR(instance: *mut c_void, frame: *const c_void, pose: *const c_void, ocvrs_return: *mut Result<()>);
// integrate(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:44
// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Volume_integrate_const__InputArrayR_const__InputArrayR(instance: *mut c_void, depth: *const c_void, pose: *const c_void, ocvrs_return: *mut Result<()>);
// integrate(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:56
// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "image", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Volume_integrate_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, depth: *const c_void, image: *const c_void, pose: *const c_void, ocvrs_return: *mut Result<()>);
// raycast(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:68
// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// raycast(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:79
// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals", "colors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const c_void, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:92
// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const c_void, height: i32, width: i32, k: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:107
// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals", "colors"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const c_void, height: i32, width: i32, k: *const c_void, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:113
// ("cv::Volume::fetchNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:118
// ("cv::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:124
// ("cv::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:128
// ("cv::Volume::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_Volume_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getVisibleBlocks()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:133
// ("cv::Volume::getVisibleBlocks", vec![(pred!(const, [], []), _)]),
pub fn cv_Volume_getVisibleBlocks_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getTotalVolumeUnits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:137
// ("cv::Volume::getTotalVolumeUnits", vec![(pred!(const, [], []), _)]),
pub fn cv_Volume_getTotalVolumeUnits_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// getBoundingBox(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:152
// ("cv::Volume::getBoundingBox", vec![(pred!(const, ["bb", "precision"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_Volume_getBoundingBox_const_const__OutputArrayR_int(instance: *const c_void, bb: *const c_void, precision: i32, ocvrs_return: *mut Result<()>);
// setEnableGrowth(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:158
// ("cv::Volume::setEnableGrowth", vec![(pred!(mut, ["v"], ["bool"]), _)]),
pub fn cv_Volume_setEnableGrowth_bool(instance: *mut c_void, v: bool, ocvrs_return: *mut Result<()>);
// getEnableGrowth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:163
// ("cv::Volume::getEnableGrowth", vec![(pred!(const, [], []), _)]),
pub fn cv_Volume_getEnableGrowth_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::Volume::delete() generated
// ("cv::Volume::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Volume_delete(instance: *mut c_void);
// VolumeSettings(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
pub fn cv_VolumeSettings_VolumeSettings_VolumeType(volume_type: crate::mod_3d::VolumeType, ocvrs_return: *mut Result<*mut c_void>);
// cv::VolumeSettings::VolumeSettings() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, [], []), _)]),
pub fn cv_VolumeSettings_VolumeSettings(ocvrs_return: *mut Result<*mut c_void>);
// VolumeSettings(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:31
// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["vs"], ["const cv::VolumeSettings*"]), _)]),
pub fn cv_VolumeSettings_VolumeSettings_const_VolumeSettingsR(vs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:32
// ("cv::VolumeSettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::VolumeSettings*"]), _)]),
pub fn cv_VolumeSettings_operatorST_const_VolumeSettingsR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// setIntegrateWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:38
// ("cv::VolumeSettings::setIntegrateWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VolumeSettings_setIntegrateWidth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIntegrateWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:42
// ("cv::VolumeSettings::getIntegrateWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getIntegrateWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIntegrateHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:47
// ("cv::VolumeSettings::setIntegrateHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VolumeSettings_setIntegrateHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIntegrateHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:51
// ("cv::VolumeSettings::getIntegrateHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getIntegrateHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRaycastWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:56
// ("cv::VolumeSettings::setRaycastWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VolumeSettings_setRaycastWidth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRaycastWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:60
// ("cv::VolumeSettings::getRaycastWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getRaycastWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRaycastHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:65
// ("cv::VolumeSettings::setRaycastHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VolumeSettings_setRaycastHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRaycastHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:69
// ("cv::VolumeSettings::getRaycastHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getRaycastHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDepthFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:74
// ("cv::VolumeSettings::setDepthFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VolumeSettings_setDepthFactor_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getDepthFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:78
// ("cv::VolumeSettings::getDepthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getDepthFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVoxelSize(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:83
// ("cv::VolumeSettings::setVoxelSize", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VolumeSettings_setVoxelSize_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getVoxelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:87
// ("cv::VolumeSettings::getVoxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getVoxelSize_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setTsdfTruncateDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:92
// ("cv::VolumeSettings::setTsdfTruncateDistance", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VolumeSettings_setTsdfTruncateDistance_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getTsdfTruncateDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:96
// ("cv::VolumeSettings::getTsdfTruncateDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getTsdfTruncateDistance_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:101
// ("cv::VolumeSettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VolumeSettings_setMaxDepth_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:105
// ("cv::VolumeSettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxWeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:111
// ("cv::VolumeSettings::setMaxWeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_VolumeSettings_setMaxWeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxWeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:116
// ("cv::VolumeSettings::getMaxWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getMaxWeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRaycastStepFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:122
// ("cv::VolumeSettings::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_VolumeSettings_setRaycastStepFactor_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getRaycastStepFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:127
// ("cv::VolumeSettings::getRaycastStepFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_getRaycastStepFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setVolumePose(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:132
// ("cv::VolumeSettings::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_VolumeSettings_setVolumePose_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getVolumePose(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:137
// ("cv::VolumeSettings::getVolumePose", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VolumeSettings_getVolumePose_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setVolumeResolution(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:145
// ("cv::VolumeSettings::setVolumeResolution", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_VolumeSettings_setVolumeResolution_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getVolumeResolution(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:153
// ("cv::VolumeSettings::getVolumeResolution", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VolumeSettings_getVolumeResolution_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getVolumeStrides(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:159
// ("cv::VolumeSettings::getVolumeStrides", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VolumeSettings_getVolumeStrides_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setCameraIntegrateIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:169
// ("cv::VolumeSettings::setCameraIntegrateIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_VolumeSettings_setCameraIntegrateIntrinsics_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getCameraIntegrateIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:179
// ("cv::VolumeSettings::getCameraIntegrateIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VolumeSettings_getCameraIntegrateIntrinsics_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// setCameraRaycastIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:189
// ("cv::VolumeSettings::setCameraRaycastIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
pub fn cv_VolumeSettings_setCameraRaycastIntrinsics_const__InputArrayR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getCameraRaycastIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:199
// ("cv::VolumeSettings::getCameraRaycastIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VolumeSettings_getCameraRaycastIntrinsics_const_const__OutputArrayR(instance: *const c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// cv::VolumeSettings::implicitClone() generated
// ("cv::VolumeSettings::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_VolumeSettings_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::VolumeSettings::delete() generated
// ("cv::VolumeSettings::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VolumeSettings_delete(instance: *mut c_void);
