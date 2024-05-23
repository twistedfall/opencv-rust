// Cholesky(double *, size_t, int, double *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:357
// ("cv::Cholesky", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["double*", "size_t", "int", "double*", "size_t", "int"]), _)]),
pub fn cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(a: *mut f64, astep: size_t, m: i32, b: *mut f64, bstep: size_t, n: i32, ocvrs_return: *mut Result<bool>);
// Cholesky(float *, size_t, int, float *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:355
// ("cv::Cholesky", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["float*", "size_t", "int", "float*", "size_t", "int"]), _)]),
pub fn cv_Cholesky_floatX_size_t_int_floatX_size_t_int(a: *mut f32, astep: size_t, m: i32, b: *mut f32, bstep: size_t, n: i32, ocvrs_return: *mut Result<bool>);
// LUT(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:496
// ("cv::LUT", vec![(pred!(mut, ["src", "lut", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, lut: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// LU(double *, size_t, int, double *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:353
// ("cv::LU", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["double*", "size_t", "int", "double*", "size_t", "int"]), _)]),
pub fn cv_LU_doubleX_size_t_int_doubleX_size_t_int(a: *mut f64, astep: size_t, m: i32, b: *mut f64, bstep: size_t, n: i32, ocvrs_return: *mut Result<i32>);
// LU(float *, size_t, int, float *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:351
// ("cv::LU", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["float*", "size_t", "int", "float*", "size_t", "int"]), _)]),
pub fn cv_LU_floatX_size_t_int_floatX_size_t_int(a: *mut f32, astep: size_t, m: i32, b: *mut f32, bstep: size_t, n: i32, ocvrs_return: *mut Result<i32>);
// Mahalanobis(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2096
// ("cv::Mahalanobis", vec![(pred!(mut, ["v1", "v2", "icovar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(v1: *const c_void, v2: *const c_void, icovar: *const c_void, ocvrs_return: *mut Result<f64>);
// PCABackProject(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2076
// ("cv::PCABackProject", vec![(pred!(mut, ["data", "mean", "eigenvectors", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// cv::PCACompute(InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2054
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2058
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, eigenvalues: *const c_void, ocvrs_return: *mut Result<()>);
// PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray, double)(InputArray, InputOutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2067
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, eigenvalues: *const c_void, retained_variance: f64, ocvrs_return: *mut Result<()>);
// PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray, int)(InputArray, InputOutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2058
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, eigenvalues: *const c_void, max_components: i32, ocvrs_return: *mut Result<()>);
// PCACompute(InputArray, InputOutputArray, OutputArray, double)(InputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2063
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, retained_variance: f64, ocvrs_return: *mut Result<()>);
// PCACompute(InputArray, InputOutputArray, OutputArray, int)(InputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2054
// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, max_components: i32, ocvrs_return: *mut Result<()>);
// PCAProject(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2072
// ("cv::PCAProject", vec![(pred!(mut, ["data", "mean", "eigenvectors", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// cv::PSNR(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:697
// ("cv::PSNR", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_PSNR_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<f64>);
// PSNR(InputArray, InputArray, double)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:697
// ("cv::PSNR", vec![(pred!(mut, ["src1", "src2", "R"], ["const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
pub fn cv_PSNR_const__InputArrayR_const__InputArrayR_double(src1: *const c_void, src2: *const c_void, r: f64, ocvrs_return: *mut Result<f64>);
// SVBackSubst(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2083
// ("cv::SVBackSubst", vec![(pred!(mut, ["w", "u", "vt", "rhs", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w: *const c_void, u: *const c_void, vt: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::SVDecomp(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2080
// ("cv::SVDecomp", vec![(pred!(mut, ["src", "w", "u", "vt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, ocvrs_return: *mut Result<()>);
// SVDecomp(InputArray, OutputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2080
// ("cv::SVDecomp", vec![(pred!(mut, ["src", "w", "u", "vt", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// abs(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4085
// ("cv::abs", vec![(pred!(mut, ["e"], ["const cv::MatExpr*"]), _)]),
pub fn cv_abs_const_MatExprR(e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// abs(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4081
// ("cv::abs", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_abs_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// absdiff(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1401
// ("cv::absdiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::addWeighted(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:446
// ("cv::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// addWeighted(InputArray, double, InputArray, double, double, OutputArray, int)(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:446
// ("cv::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst", "dtype"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// cv::add(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:291
// ("cv::add", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// add(InputArray, InputArray, OutputArray, InputArray, int)(InputArray, InputArray, OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:291
// ("cv::add", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// cv::batchDistance(InputArray, InputArray, OutputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:704
// ("cv::batchDistance", vec![(pred!(mut, ["src1", "src2", "dist", "dtype", "nidx"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dist: *const c_void, dtype: i32, nidx: *const c_void, ocvrs_return: *mut Result<()>);
// batchDistance(InputArray, InputArray, OutputArray, int, OutputArray, int, int, InputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, OutputArray, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:704
// ("cv::batchDistance", vec![(pred!(mut, ["src1", "src2", "dist", "dtype", "nidx", "normType", "K", "mask", "update", "crosscheck"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_OutputArray*", "int", "int", "const cv::_InputArray*", "int", "bool"]), _)]),
pub fn cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(src1: *const c_void, src2: *const c_void, dist: *const c_void, dtype: i32, nidx: *const c_void, norm_type: i32, k: i32, mask: *const c_void, update: i32, crosscheck: bool, ocvrs_return: *mut Result<()>);
// cv::bitwise_and(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1300
// ("cv::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_and(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1300
// ("cv::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bitwise_not(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1372
// ("cv::bitwise_not", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bitwise_not_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_not(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1372
// ("cv::bitwise_not", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bitwise_or(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1327
// ("cv::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_or(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1327
// ("cv::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::bitwise_xor(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1355
// ("cv::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_xor(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1355
// ("cv::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// borderInterpolate(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:198
// ("cv::borderInterpolate", vec![(pred!(mut, ["p", "len", "borderType"], ["int", "int", "int"]), _)]),
pub fn cv_borderInterpolate_int_int_int(p: i32, len: i32, border_type: i32, ocvrs_return: *mut Result<i32>);
// broadcast(InputArray, const MatShape &, OutputArray)(InputArray, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1104
// ("cv::broadcast", vec![(pred!(mut, ["src", "shape", "dst"], ["const cv::_InputArray*", "const cv::MatShape*", "const cv::_OutputArray*"]), _)]),
pub fn cv_broadcast_const__InputArrayR_const_MatShapeR_const__OutputArrayR(src: *const c_void, shape: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// broadcast(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1097
// ("cv::broadcast", vec![(pred!(mut, ["src", "shape", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_broadcast_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, shape: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::calcCovarMatrix(InputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2050
// ("cv::calcCovarMatrix", vec![(pred!(mut, ["samples", "covar", "mean", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
pub fn cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int(samples: *const c_void, covar: *const c_void, mean: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// calcCovarMatrix(InputArray, OutputArray, InputOutputArray, int, int)(InputArray, OutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2050
// ("cv::calcCovarMatrix", vec![(pred!(mut, ["samples", "covar", "mean", "flags", "ctype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "int", "int"]), _)]),
pub fn cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(samples: *const c_void, covar: *const c_void, mean: *const c_void, flags: i32, ctype: i32, ocvrs_return: *mut Result<()>);
// cv::cartToPolar(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1612
// ("cv::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// cartToPolar(InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1612
// ("cv::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(x: *const c_void, y: *const c_void, magnitude: *const c_void, angle: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<()>);
// checkHardwareSupport(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:483
// ("cv::checkHardwareSupport", vec![(pred!(mut, ["feature"], ["int"]), _)]),
pub fn cv_checkHardwareSupport_int(feature: i32, ocvrs_return: *mut Result<bool>);
// cv::checkRange(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1663
// ("cv::checkRange", vec![(pred!(mut, ["a"], ["const cv::_InputArray*"]), _)]),
pub fn cv_checkRange_const__InputArrayR(a: *const c_void, ocvrs_return: *mut Result<bool>);
// checkRange(InputArray, bool, Point *, double, double)(InputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1663
// ("cv::checkRange", vec![(pred!(mut, ["a", "quiet", "pos", "minVal", "maxVal"], ["const cv::_InputArray*", "bool", "cv::Point*", "double", "double"]), _)]),
pub fn cv_checkRange_const__InputArrayR_bool_PointX_double_double(a: *const c_void, quiet: bool, pos: *mut core::Point, min_val: f64, max_val: f64, ocvrs_return: *mut Result<bool>);
// compare(InputArray, InputArray, OutputArray, int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1462
// ("cv::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, cmpop: i32, ocvrs_return: *mut Result<()>);
// cv::completeSymm(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1832
// ("cv::completeSymm", vec![(pred!(mut, ["m"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_completeSymm_const__InputOutputArrayR(m: *const c_void, ocvrs_return: *mut Result<()>);
// completeSymm(InputOutputArray, bool)(InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1832
// ("cv::completeSymm", vec![(pred!(mut, ["m", "lowerToUpper"], ["const cv::_InputOutputArray*", "bool"]), _)]),
pub fn cv_completeSymm_const__InputOutputArrayR_bool(m: *const c_void, lower_to_upper: bool, ocvrs_return: *mut Result<()>);
// cv::convertScaleAbs(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:474
// ("cv::convertScaleAbs", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertScaleAbs(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:474
// ("cv::convertScaleAbs", vec![(pred!(mut, ["src", "dst", "alpha", "beta"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::copyMakeBorder(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:248
// ("cv::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int"]), _)]),
pub fn cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int(src: *const c_void, dst: *const c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// copyMakeBorder(InputArray, OutputArray, int, int, int, int, int, const Scalar &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:248
// ("cv::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType", "value"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// copyTo(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1413
// ("cv::copyTo", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_copyTo_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// countNonZero(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:544
// ("cv::countNonZero", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_countNonZero_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<i32>);
// cubeRoot(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:329
// ("cv::cubeRoot", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_cubeRoot_float(val: f32, ocvrs_return: *mut Result<f32>);
// createContinuous(int, int, int, OutputArray)(Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:586
// ("cv::cuda::createContinuous", vec![(pred!(mut, ["rows", "cols", "type", "arr"], ["int", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_createContinuous_int_int_int_const__OutputArrayR(rows: i32, cols: i32, typ: i32, arr: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::createGpuMatFromCudaMemory(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:618
// ("cv::cuda::createGpuMatFromCudaMemory", vec![(pred!(mut, ["size", "type", "cudaMemoryAddress"], ["cv::Size", "int", "size_t"]), _)]),
pub fn cv_cuda_createGpuMatFromCudaMemory_Size_int_size_t(size: *const core::Size, typ: i32, cuda_memory_address: size_t, ocvrs_return: *mut Result<*mut c_void>);
// createGpuMatFromCudaMemory(Size, int, size_t, size_t)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:618
// ("cv::cuda::createGpuMatFromCudaMemory", vec![(pred!(mut, ["size", "type", "cudaMemoryAddress", "step"], ["cv::Size", "int", "size_t", "size_t"]), _)]),
pub fn cv_cuda_createGpuMatFromCudaMemory_Size_int_size_t_size_t(size: *const core::Size, typ: i32, cuda_memory_address: size_t, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createGpuMatFromCudaMemory(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:607
// ("cv::cuda::createGpuMatFromCudaMemory", vec![(pred!(mut, ["rows", "cols", "type", "cudaMemoryAddress"], ["int", "int", "int", "size_t"]), _)]),
pub fn cv_cuda_createGpuMatFromCudaMemory_int_int_int_size_t(rows: i32, cols: i32, typ: i32, cuda_memory_address: size_t, ocvrs_return: *mut Result<*mut c_void>);
// createGpuMatFromCudaMemory(int, int, int, size_t, size_t)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:607
// ("cv::cuda::createGpuMatFromCudaMemory", vec![(pred!(mut, ["rows", "cols", "type", "cudaMemoryAddress", "step"], ["int", "int", "int", "size_t", "size_t"]), _)]),
pub fn cv_cuda_createGpuMatFromCudaMemory_int_int_int_size_t_size_t(rows: i32, cols: i32, typ: i32, cuda_memory_address: size_t, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// deviceSupports(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1077
// ("cv::cuda::deviceSupports", vec![(pred!(mut, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
pub fn cv_cuda_deviceSupports_FeatureSet(feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
// ensureSizeIsEnough(int, int, int, OutputArray)(Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:597
// ("cv::cuda::ensureSizeIsEnough", vec![(pred!(mut, ["rows", "cols", "type", "arr"], ["int", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(rows: i32, cols: i32, typ: i32, arr: *const c_void, ocvrs_return: *mut Result<()>);
// getCudaEnabledDeviceCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1033
// ("cv::cuda::getCudaEnabledDeviceCount", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_getCudaEnabledDeviceCount(ocvrs_return: *mut Result<i32>);
// getDevice()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1045
// ("cv::cuda::getDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_getDevice(ocvrs_return: *mut Result<i32>);
// printCudaDeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1319
// ("cv::cuda::printCudaDeviceInfo", vec![(pred!(mut, ["device"], ["int"]), _)]),
pub fn cv_cuda_printCudaDeviceInfo_int(device: i32, ocvrs_return: *mut Result<()>);
// printShortCudaDeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1320
// ("cv::cuda::printShortCudaDeviceInfo", vec![(pred!(mut, ["device"], ["int"]), _)]),
pub fn cv_cuda_printShortCudaDeviceInfo_int(device: i32, ocvrs_return: *mut Result<()>);
// registerPageLocked(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:869
// ("cv::cuda::registerPageLocked", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_cuda_registerPageLocked_MatR(m: *mut c_void, ocvrs_return: *mut Result<()>);
// resetDevice()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1052
// ("cv::cuda::resetDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_resetDevice(ocvrs_return: *mut Result<()>);
// setBufferPoolConfig(int, size_t, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:769
// ("cv::cuda::setBufferPoolConfig", vec![(pred!(mut, ["deviceId", "stackSize", "stackCount"], ["int", "size_t", "int"]), _)]),
pub fn cv_cuda_setBufferPoolConfig_int_size_t_int(device_id: i32, stack_size: size_t, stack_count: i32, ocvrs_return: *mut Result<()>);
// setBufferPoolUsage(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:768
// ("cv::cuda::setBufferPoolUsage", vec![(pred!(mut, ["on"], ["bool"]), _)]),
pub fn cv_cuda_setBufferPoolUsage_bool(on: bool, ocvrs_return: *mut Result<()>);
// setDevice(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1041
// ("cv::cuda::setDevice", vec![(pred!(mut, ["device"], ["int"]), _)]),
pub fn cv_cuda_setDevice_int(device: i32, ocvrs_return: *mut Result<()>);
// cv::cuda::setGlDevice() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:572
// ("cv::cuda::setGlDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_setGlDevice(ocvrs_return: *mut Result<()>);
// setGlDevice(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:572
// ("cv::cuda::setGlDevice", vec![(pred!(mut, ["device"], ["int"]), _)]),
pub fn cv_cuda_setGlDevice_int(device: i32, ocvrs_return: *mut Result<()>);
// unregisterPageLocked(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:875
// ("cv::cuda::unregisterPageLocked", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_cuda_unregisterPageLocked_MatR(m: *mut c_void, ocvrs_return: *mut Result<()>);
// wrapStream(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:981
// ("cv::cuda::wrapStream", vec![(pred!(mut, ["cudaStreamMemoryAddress"], ["size_t"]), _)]),
pub fn cv_cuda_wrapStream_size_t(cuda_stream_memory_address: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::dct(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2251
// ("cv::dct", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dct_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// dct(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2251
// ("cv::dct", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_dct_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// depthToString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:15
// ("cv::depthToString", vec![(pred!(mut, ["depth"], ["int"]), _)]),
pub fn cv_depthToString_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
// check_failed_MatChannels(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:91
// ("cv::detail::check_failed_MatChannels", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatChannels_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_MatChannels(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:78
// ("cv::detail::check_failed_MatChannels", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatChannels_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_MatDepth(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:89
// ("cv::detail::check_failed_MatDepth", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatDepth_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_MatDepth(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:76
// ("cv::detail::check_failed_MatDepth", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatDepth_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_MatType(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:90
// ("cv::detail::check_failed_MatType", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatType_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_MatType(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:77
// ("cv::detail::check_failed_MatType", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_MatType_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const Size_<int>, const CheckContext &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:87
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const cv::Size_<int>", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_Size_LintG_const_CheckContextR(v: *const core::Size_<i32>, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const Size_<int>, const Size_<int>, const CheckContext &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:75
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const cv::Size_<int>", "const cv::Size_<int>", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_Size_LintG_const_Size_LintG_const_CheckContextR(v1: *const core::Size_<i32>, v2: *const core::Size_<i32>, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const bool, const bool, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:70
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const bool", "const bool", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_bool_const_bool_const_CheckContextR(v1: bool, v2: bool, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const double, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:86
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const double", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_double_const_CheckContextR(v: f64, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const double, const double, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:74
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const double", "const double", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_double_const_double_const_CheckContextR(v1: f64, v2: f64, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const float, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:85
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const float", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_float_const_CheckContextR(v: f32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const float, const float, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:73
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const float", "const float", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_float_const_float_const_CheckContextR(v1: f32, v2: f32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:83
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:71
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const size_t, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:84
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const size_t", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_size_t_const_CheckContextR(v: size_t, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const size_t, const size_t, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:72
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const size_t", "const size_t", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_size_t_const_size_t_const_CheckContextR(v1: size_t, v2: size_t, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_auto(const std::string &, const CheckContext &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:88
// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "ctx"], ["const std::string*", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_auto_const_stringR_const_CheckContextR(v1: *const c_char, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_false(const bool, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:81
// ("cv::detail::check_failed_false", vec![(pred!(mut, ["v", "ctx"], ["const bool", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_false_const_bool_const_CheckContextR(v: bool, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// check_failed_true(const bool, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:80
// ("cv::detail::check_failed_true", vec![(pred!(mut, ["v", "ctx"], ["const bool", "const cv::detail::CheckContext*"]), _)]),
pub fn cv_detail_check_failed_true_const_bool_const_CheckContextR(v: bool, ctx: *const c_void, ocvrs_return: *mut Result<()>);
// determinant(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1864
// ("cv::determinant", vec![(pred!(mut, ["mtx"], ["const cv::_InputArray*"]), _)]),
pub fn cv_determinant_const__InputArrayR(mtx: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::dft(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2196
// ("cv::dft", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dft_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// dft(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2196
// ("cv::dft", vec![(pred!(mut, ["src", "dst", "flags", "nonzeroRows"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_dft_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, flags: i32, nonzero_rows: i32, ocvrs_return: *mut Result<()>);
// convertFromD3D10Texture2D(ID3D10Texture2D *, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:148
// ("cv::directx::convertFromD3D10Texture2D", vec![(pred!(mut, ["pD3D10Texture2D", "dst"], ["ID3D10Texture2D*", "const cv::_OutputArray*"]), _)]),
pub fn cv_directx_convertFromD3D10Texture2D_ID3D10Texture2DX_const__OutputArrayR(p_d3d10_texture_2d: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertFromD3D11Texture2D(ID3D11Texture2D *, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:130
// ("cv::directx::convertFromD3D11Texture2D", vec![(pred!(mut, ["pD3D11Texture2D", "dst"], ["ID3D11Texture2D*", "const cv::_OutputArray*"]), _)]),
pub fn cv_directx_convertFromD3D11Texture2D_ID3D11Texture2DX_const__OutputArrayR(p_d3d11_texture_2d: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::directx::convertFromDirect3DSurface9(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:168
// ("cv::directx::convertFromDirect3DSurface9", vec![(pred!(mut, ["pDirect3DSurface9", "dst"], ["IDirect3DSurface9*", "const cv::_OutputArray*"]), _)]),
pub fn cv_directx_convertFromDirect3DSurface9_IDirect3DSurface9X_const__OutputArrayR(p_direct_3d_surface9: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertFromDirect3DSurface9(IDirect3DSurface9 *, OutputArray, void *)(TraitClass, OutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:168
// ("cv::directx::convertFromDirect3DSurface9", vec![(pred!(mut, ["pDirect3DSurface9", "dst", "surfaceSharedHandle"], ["IDirect3DSurface9*", "const cv::_OutputArray*", "void*"]), _)]),
pub fn cv_directx_convertFromDirect3DSurface9_IDirect3DSurface9X_const__OutputArrayR_voidX(p_direct_3d_surface9: *mut c_void, dst: *const c_void, surface_shared_handle: *mut c_void, ocvrs_return: *mut Result<()>);
// convertToD3D10Texture2D(InputArray, ID3D10Texture2D *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:139
// ("cv::directx::convertToD3D10Texture2D", vec![(pred!(mut, ["src", "pD3D10Texture2D"], ["const cv::_InputArray*", "ID3D10Texture2D*"]), _)]),
pub fn cv_directx_convertToD3D10Texture2D_const__InputArrayR_ID3D10Texture2DX(src: *const c_void, p_d3d10_texture_2d: *mut c_void, ocvrs_return: *mut Result<()>);
// convertToD3D11Texture2D(InputArray, ID3D11Texture2D *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:120
// ("cv::directx::convertToD3D11Texture2D", vec![(pred!(mut, ["src", "pD3D11Texture2D"], ["const cv::_InputArray*", "ID3D11Texture2D*"]), _)]),
pub fn cv_directx_convertToD3D11Texture2D_const__InputArrayR_ID3D11Texture2DX(src: *const c_void, p_d3d11_texture_2d: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::directx::convertToDirect3DSurface9(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:158
// ("cv::directx::convertToDirect3DSurface9", vec![(pred!(mut, ["src", "pDirect3DSurface9"], ["const cv::_InputArray*", "IDirect3DSurface9*"]), _)]),
pub fn cv_directx_convertToDirect3DSurface9_const__InputArrayR_IDirect3DSurface9X(src: *const c_void, p_direct_3d_surface9: *mut c_void, ocvrs_return: *mut Result<()>);
// convertToDirect3DSurface9(InputArray, IDirect3DSurface9 *, void *)(InputArray, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:158
// ("cv::directx::convertToDirect3DSurface9", vec![(pred!(mut, ["src", "pDirect3DSurface9", "surfaceSharedHandle"], ["const cv::_InputArray*", "IDirect3DSurface9*", "void*"]), _)]),
pub fn cv_directx_convertToDirect3DSurface9_const__InputArrayR_IDirect3DSurface9X_voidX(src: *const c_void, p_direct_3d_surface9: *mut c_void, surface_shared_handle: *mut c_void, ocvrs_return: *mut Result<()>);
// getTypeFromD3DFORMAT(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:178
// ("cv::directx::getTypeFromD3DFORMAT", vec![(pred!(mut, ["iD3DFORMAT"], ["const int"]), _)]),
pub fn cv_directx_getTypeFromD3DFORMAT_const_int(id_3d_format: i32, ocvrs_return: *mut Result<i32>);
// getTypeFromDXGI_FORMAT(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:173
// ("cv::directx::getTypeFromDXGI_FORMAT", vec![(pred!(mut, ["iDXGI_FORMAT"], ["const int"]), _)]),
pub fn cv_directx_getTypeFromDXGI_FORMAT_const_int(i_dxgi_format: i32, ocvrs_return: *mut Result<i32>);
// initializeContextFromD3D10Device(ID3D10Device *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:91
// ("cv::directx::ocl::initializeContextFromD3D10Device", vec![(pred!(mut, ["pD3D10Device"], ["ID3D10Device*"]), _)]),
pub fn cv_directx_ocl_initializeContextFromD3D10Device_ID3D10DeviceX(p_d3d10_device: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// initializeContextFromD3D11Device(ID3D11Device *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:85
// ("cv::directx::ocl::initializeContextFromD3D11Device", vec![(pred!(mut, ["pD3D11Device"], ["ID3D11Device*"]), _)]),
pub fn cv_directx_ocl_initializeContextFromD3D11Device_ID3D11DeviceX(p_d3d11_device: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// initializeContextFromDirect3DDevice9Ex(IDirect3DDevice9Ex *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:97
// ("cv::directx::ocl::initializeContextFromDirect3DDevice9Ex", vec![(pred!(mut, ["pDirect3DDevice9Ex"], ["IDirect3DDevice9Ex*"]), _)]),
pub fn cv_directx_ocl_initializeContextFromDirect3DDevice9Ex_IDirect3DDevice9ExX(p_direct_3d_device9_ex: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// initializeContextFromDirect3DDevice9(IDirect3DDevice9 *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/directx.hpp:103
// ("cv::directx::ocl::initializeContextFromDirect3DDevice9", vec![(pred!(mut, ["pDirect3DDevice9"], ["IDirect3DDevice9*"]), _)]),
pub fn cv_directx_ocl_initializeContextFromDirect3DDevice9_IDirect3DDevice9X(p_direct_3d_device9: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::divide(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:393
// ("cv::divide", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// divide(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:393
// ("cv::divide", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
pub fn cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result<()>);
// cv::divide(Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:397
// ("cv::divide", vec![(pred!(mut, ["scale", "src2", "dst"], ["double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_divide_double_const__InputArrayR_const__OutputArrayR(scale: f64, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// divide(double, InputArray, OutputArray, int)(Primitive, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:397
// ("cv::divide", vec![(pred!(mut, ["scale", "src2", "dst", "dtype"], ["double", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_divide_double_const__InputArrayR_const__OutputArrayR_int(scale: f64, src2: *const c_void, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// eigenNonSymmetric(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2023
// ("cv::eigenNonSymmetric", vec![(pred!(mut, ["src", "eigenvalues", "eigenvectors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, eigenvalues: *const c_void, eigenvectors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::eigen(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2006
// ("cv::eigen", vec![(pred!(mut, ["src", "eigenvalues"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_eigen_const__InputArrayR_const__OutputArrayR(src: *const c_void, eigenvalues: *const c_void, ocvrs_return: *mut Result<bool>);
// eigen(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2006
// ("cv::eigen", vec![(pred!(mut, ["src", "eigenvalues", "eigenvectors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, eigenvalues: *const c_void, eigenvectors: *const c_void, ocvrs_return: *mut Result<bool>);
// error(Error::Code, const String &, const char *, const char *, int)(Enum, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:137
// ("cv::error", vec![(pred!(mut, ["code", "err", "func", "file", "line"], ["cv::Error::Code", "const cv::String*", "const char*", "const char*", "int"]), _)]),
pub fn cv_error_Code_const_StringR_const_charX_const_charX_int(code: core::Code, err: *const c_char, func: *const c_char, file: *const c_char, line: i32, ocvrs_return: *mut Result<()>);
// error(const Exception &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:123
// ("cv::error", vec![(pred!(mut, ["exc"], ["const cv::Exception*"]), _)]),
pub fn cv_error_const_ExceptionR(exc: *const c_void, ocvrs_return: *mut Result<()>);
// exp(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1556
// ("cv::exp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_exp_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// extractChannel(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1041
// ("cv::extractChannel", vec![(pred!(mut, ["src", "dst", "coi"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, coi: i32, ocvrs_return: *mut Result<()>);
// fastAtan2(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:348
// ("cv::fastAtan2", vec![(pred!(mut, ["y", "x"], ["float", "float"]), _)]),
pub fn cv_fastAtan2_float_float(y: f32, x: f32, ocvrs_return: *mut Result<f32>);
// findNonZero(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:583
// ("cv::findNonZero", vec![(pred!(mut, ["src", "idx"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findNonZero_const__InputArrayR_const__OutputArrayR(src: *const c_void, idx: *const c_void, ocvrs_return: *mut Result<()>);
// finiteMask(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1678
// ("cv::finiteMask", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_finiteMask_const__InputArrayR_const__OutputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// flipND(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1090
// ("cv::flipND", vec![(pred!(mut, ["src", "dst", "axis"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_flipND_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, axis: i32, ocvrs_return: *mut Result<()>);
// flip(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1083
// ("cv::flip", vec![(pred!(mut, ["src", "dst", "flipCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_flip_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flip_code: i32, ocvrs_return: *mut Result<()>);
// cv::gemm(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1709
// ("cv::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR(src1: *const c_void, src2: *const c_void, alpha: f64, src3: *const c_void, beta: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// gemm(InputArray, InputArray, double, InputArray, double, OutputArray, int)(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1709
// ("cv::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, alpha: f64, src3: *const c_void, beta: f64, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// getBuildInformation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:273
// ("cv::getBuildInformation", vec![(pred!(mut, [], []), _)]),
pub fn cv_getBuildInformation(ocvrs_return: *mut Result<*mut c_void>);
// getCPUFeaturesLine()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:501
// ("cv::getCPUFeaturesLine", vec![(pred!(mut, [], []), _)]),
pub fn cv_getCPUFeaturesLine(ocvrs_return: *mut Result<*mut c_void>);
// getCPUTickCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:473
// ("cv::getCPUTickCount", vec![(pred!(mut, [], []), _)]),
pub fn cv_getCPUTickCount(ocvrs_return: *mut Result<i64>);
// getDefaultAlgorithmHint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:615
// ("cv::getDefaultAlgorithmHint", vec![(pred!(mut, [], []), _)]),
pub fn cv_getDefaultAlgorithmHint(ocvrs_return: *mut Result<core::AlgorithmHint>);
// getElemSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:638
// ("cv::getElemSize", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_getElemSize_int(typ: i32, ocvrs_return: *mut Result<size_t>);
// getHardwareFeatureName(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:489
// ("cv::getHardwareFeatureName", vec![(pred!(mut, ["feature"], ["int"]), _)]),
pub fn cv_getHardwareFeatureName_int(feature: i32, ocvrs_return: *mut Result<*mut c_void>);
// getLogLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:348
// ("cv::getLogLevel", vec![(pred!(mut, [], []), _)]),
pub fn cv_getLogLevel(ocvrs_return: *mut Result<i32>);
// getNumThreads()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:249
// ("cv::getNumThreads", vec![(pred!(mut, [], []), _)]),
pub fn cv_getNumThreads(ocvrs_return: *mut Result<i32>);
// getNumberOfCPUs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:505
// ("cv::getNumberOfCPUs", vec![(pred!(mut, [], []), _)]),
pub fn cv_getNumberOfCPUs(ocvrs_return: *mut Result<i32>);
// getOptimalDFTSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2304
// ("cv::getOptimalDFTSize", vec![(pred!(mut, ["vecsize"], ["int"]), _)]),
pub fn cv_getOptimalDFTSize_int(vecsize: i32, ocvrs_return: *mut Result<i32>);
// getThreadNum()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:265
// ("cv::getThreadNum", vec![(pred!(mut, [], []), _)]),
pub fn cv_getThreadNum(ocvrs_return: *mut Result<i32>);
// getTickCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:299
// ("cv::getTickCount", vec![(pred!(mut, [], []), _)]),
pub fn cv_getTickCount(ocvrs_return: *mut Result<i64>);
// getTickFrequency()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:312
// ("cv::getTickFrequency", vec![(pred!(mut, [], []), _)]),
pub fn cv_getTickFrequency(ocvrs_return: *mut Result<f64>);
// getVersionMajor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:284
// ("cv::getVersionMajor", vec![(pred!(mut, [], []), _)]),
pub fn cv_getVersionMajor() -> i32;
// getVersionMinor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:287
// ("cv::getVersionMinor", vec![(pred!(mut, [], []), _)]),
pub fn cv_getVersionMinor() -> i32;
// getVersionRevision()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:290
// ("cv::getVersionRevision", vec![(pred!(mut, [], []), _)]),
pub fn cv_getVersionRevision() -> i32;
// getVersionString()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:281
// ("cv::getVersionString", vec![(pred!(mut, [], []), _)]),
pub fn cv_getVersionString(ocvrs_return: *mut Result<*mut c_void>);
// cv::glob(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:211
// ("cv::glob", vec![(pred!(mut, ["pattern", "result"], ["cv::String", "std::vector<cv::String>*"]), _)]),
pub fn cv_glob_String_vectorLStringGR(pattern: *const c_char, result: *mut c_void, ocvrs_return: *mut Result<()>);
// glob(String, std::vector<String> &, bool)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:211
// ("cv::glob", vec![(pred!(mut, ["pattern", "result", "recursive"], ["cv::String", "std::vector<cv::String>*", "bool"]), _)]),
pub fn cv_glob_String_vectorLStringGR_bool(pattern: *const c_char, result: *mut c_void, recursive: bool, ocvrs_return: *mut Result<()>);
// hasNonZero(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:524
// ("cv::hasNonZero", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_hasNonZero_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<bool>);
// hconcat(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1190
// ("cv::hconcat", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// hconcat(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1209
// ("cv::hconcat", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_hconcat_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::idct(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2261
// ("cv::idct", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_idct_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// idct(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2261
// ("cv::idct", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_idct_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// cv::idft(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2210
// ("cv::idft", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_idft_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// idft(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2210
// ("cv::idft", vec![(pred!(mut, ["src", "dst", "flags", "nonzeroRows"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_idft_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, flags: i32, nonzero_rows: i32, ocvrs_return: *mut Result<()>);
// inRange(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1433
// ("cv::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, lowerb: *const c_void, upperb: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// insertChannel(InputArray, InputOutputArray, int)(InputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1049
// ("cv::insertChannel", vec![(pred!(mut, ["src", "dst", "coi"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
pub fn cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(src: *const c_void, dst: *const c_void, coi: i32, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:117
// ("cv::instr::getFlags", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_getFlags(ocvrs_return: *mut Result<core::FLAGS>);
// resetTrace()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:106
// ("cv::instr::resetTrace", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_resetTrace(ocvrs_return: *mut Result<()>);
// setFlags(FLAGS)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:115
// ("cv::instr::setFlags", vec![(pred!(mut, ["modeFlags"], ["cv::instr::FLAGS"]), _)]),
pub fn cv_instr_setFlags_FLAGS(mode_flags: core::FLAGS, ocvrs_return: *mut Result<()>);
// setUseInstrumentation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:105
// ("cv::instr::setUseInstrumentation", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_instr_setUseInstrumentation_bool(flag: bool, ocvrs_return: *mut Result<()>);
// useInstrumentation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:104
// ("cv::instr::useInstrumentation", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_useInstrumentation(ocvrs_return: *mut Result<bool>);
// cv::invert(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1900
// ("cv::invert", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_invert_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<f64>);
// invert(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1900
// ("cv::invert", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_invert_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<f64>);
// getIppErrorLocation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:370
// ("cv::ipp::getIppErrorLocation", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_getIppErrorLocation(ocvrs_return: *mut Result<*mut c_void>);
// getIppFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:366
// ("cv::ipp::getIppFeatures", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_getIppFeatures(ocvrs_return: *mut Result<u64>);
// getIppStatus()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:369
// ("cv::ipp::getIppStatus", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_getIppStatus(ocvrs_return: *mut Result<i32>);
// getIppVersion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:373
// ("cv::ipp::getIppVersion", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_getIppVersion(ocvrs_return: *mut Result<*mut c_void>);
// cv::ipp::setIppStatus(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:367
// ("cv::ipp::setIppStatus", vec![(pred!(mut, ["status"], ["int"]), _)]),
pub fn cv_ipp_setIppStatus_int(status: i32, ocvrs_return: *mut Result<()>);
// setIppStatus(int, const char *const, const char *const, int)(Primitive, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:367
// ("cv::ipp::setIppStatus", vec![(pred!(mut, ["status", "funcname", "filename", "line"], ["int", "const char*", "const char*", "int"]), _)]),
pub fn cv_ipp_setIppStatus_int_const_charX_const_charX_int(status: i32, funcname: *const c_char, filename: *const c_char, line: i32, ocvrs_return: *mut Result<()>);
// setUseIPP_NotExact(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:378
// ("cv::ipp::setUseIPP_NotExact", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ipp_setUseIPP_NotExact_bool(flag: bool, ocvrs_return: *mut Result<()>);
// setUseIPP(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:372
// ("cv::ipp::setUseIPP", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ipp_setUseIPP_bool(flag: bool, ocvrs_return: *mut Result<()>);
// useIPP()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:371
// ("cv::ipp::useIPP", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_useIPP(ocvrs_return: *mut Result<bool>);
// useIPP_NotExact()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:377
// ("cv::ipp::useIPP_NotExact", vec![(pred!(mut, [], []), _)]),
pub fn cv_ipp_useIPP_NotExact(ocvrs_return: *mut Result<bool>);
// cv::kmeans(InputArray, Primitive, InputOutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3049
// ("cv::kmeans", vec![(pred!(mut, ["data", "K", "bestLabels", "criteria", "attempts", "flags"], ["const cv::_InputArray*", "int", "const cv::_InputOutputArray*", "cv::TermCriteria", "int", "int"]), _)]),
pub fn cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int(data: *const c_void, k: i32, best_labels: *const c_void, criteria: *const core::TermCriteria, attempts: i32, flags: i32, ocvrs_return: *mut Result<f64>);
// kmeans(InputArray, int, InputOutputArray, TermCriteria, int, int, OutputArray)(InputArray, Primitive, InputOutputArray, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3049
// ("cv::kmeans", vec![(pred!(mut, ["data", "K", "bestLabels", "criteria", "attempts", "flags", "centers"], ["const cv::_InputArray*", "int", "const cv::_InputOutputArray*", "cv::TermCriteria", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(data: *const c_void, k: i32, best_labels: *const c_void, criteria: *const core::TermCriteria, attempts: i32, flags: i32, centers: *const c_void, ocvrs_return: *mut Result<f64>);
// layoutToString(DataLayout)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:96
// ("cv::layoutToString", vec![(pred!(mut, ["layout"], ["cv::DataLayout"]), _)]),
pub fn cv_layoutToString_DataLayout(layout: core::DataLayout, ocvrs_return: *mut Result<*mut c_void>);
// log(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1569
// ("cv::log", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_log_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// magnitude(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1646
// ("cv::magnitude", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result<()>);
// max(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4060
// ("cv::max", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_max_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// max(const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1500
// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_max_const_MatR_const_MatR_MatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// max(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4061
// ("cv::max", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_max_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// max(const UMat &, const UMat &, UMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1504
// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::UMat*", "const cv::UMat*", "cv::UMat*"]), _)]),
pub fn cv_max_const_UMatR_const_UMatR_UMatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// max(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1496
// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// max(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4062
// ("cv::max", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_max_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::meanStdDev(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:618
// ("cv::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, mean: *const c_void, stddev: *const c_void, ocvrs_return: *mut Result<()>);
// meanStdDev(InputArray, OutputArray, OutputArray, InputArray)(InputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:618
// ("cv::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, mean: *const c_void, stddev: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::mean(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:596
// ("cv::mean", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_mean_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// mean(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:596
// ("cv::mean", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_mean_const__InputArrayR_const__InputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// merge(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:933
// ("cv::merge", vec![(pred!(mut, ["mv", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_merge_const__InputArrayR_const__OutputArrayR(mv: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::minMaxIdx(InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:865
// ("cv::minMaxIdx", vec![(pred!(mut, ["src", "minVal"], ["const cv::_InputArray*", "double*"]), _)]),
pub fn cv_minMaxIdx_const__InputArrayR_doubleX(src: *const c_void, min_val: *mut f64, ocvrs_return: *mut Result<()>);
// minMaxIdx(InputArray, double *, double *, int *, int *, InputArray)(InputArray, Indirect, Indirect, Indirect, Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:865
// ("cv::minMaxIdx", vec![(pred!(mut, ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"], ["const cv::_InputArray*", "double*", "double*", "int*", "int*", "const cv::_InputArray*"]), _)]),
pub fn cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_idx: *mut i32, max_idx: *mut i32, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::minMaxLoc(TraitClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:877
// ("cv::minMaxLoc", vec![(pred!(mut, ["a", "minVal", "maxVal"], ["const cv::SparseMat*", "double*", "double*"]), _)]),
pub fn cv_minMaxLoc_const_SparseMatR_doubleX_doubleX(a: *const c_void, min_val: *mut f64, max_val: *mut f64, ocvrs_return: *mut Result<()>);
// minMaxLoc(const SparseMat &, double *, double *, int *, int *)(TraitClass, Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:877
// ("cv::minMaxLoc", vec![(pred!(mut, ["a", "minVal", "maxVal", "minIdx", "maxIdx"], ["const cv::SparseMat*", "double*", "double*", "int*", "int*"]), _)]),
pub fn cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(a: *const c_void, min_val: *mut f64, max_val: *mut f64, min_idx: *mut i32, max_idx: *mut i32, ocvrs_return: *mut Result<()>);
// cv::minMaxLoc(InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:806
// ("cv::minMaxLoc", vec![(pred!(mut, ["src", "minVal"], ["const cv::_InputArray*", "double*"]), _)]),
pub fn cv_minMaxLoc_const__InputArrayR_doubleX(src: *const c_void, min_val: *mut f64, ocvrs_return: *mut Result<()>);
// minMaxLoc(InputArray, double *, double *, Point *, Point *, InputArray)(InputArray, Indirect, Indirect, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:806
// ("cv::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*", "const cv::_InputArray*"]), _)]),
pub fn cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_loc: *mut core::Point, max_loc: *mut core::Point, mask: *const c_void, ocvrs_return: *mut Result<()>);
// min(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4052
// ("cv::min", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_min_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// min(const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1479
// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_min_const_MatR_const_MatR_MatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// min(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4053
// ("cv::min", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_min_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// min(const UMat &, const UMat &, UMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1483
// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::UMat*", "const cv::UMat*", "cv::UMat*"]), _)]),
pub fn cv_min_const_UMatR_const_UMatR_UMatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// min(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1475
// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// min(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4054
// ("cv::min", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_min_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mixChannels(InputArrayOfArrays, InputOutputArrayOfArrays, const int *, size_t)(InputArray, InputOutputArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1016
// ("cv::mixChannels", vec![(pred!(mut, ["src", "dst", "fromTo", "npairs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const int*", "size_t"]), _)]),
pub fn cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(src: *const c_void, dst: *const c_void, from_to: *const i32, npairs: size_t, ocvrs_return: *mut Result<()>);
// mixChannels(InputArrayOfArrays, InputOutputArrayOfArrays, const std::vector<int> &)(InputArray, InputOutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1032
// ("cv::mixChannels", vec![(pred!(mut, ["src", "dst", "fromTo"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vectorLintGR(src: *const c_void, dst: *const c_void, from_to: *const c_void, ocvrs_return: *mut Result<()>);
// cv::mulSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2280
// ("cv::mulSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// mulSpectrums(InputArray, InputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2280
// ("cv::mulSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags", "conjB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, conj_b: bool, ocvrs_return: *mut Result<()>);
// cv::mulTransposed(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1740
// ("cv::mulTransposed", vec![(pred!(mut, ["src", "dst", "aTa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool(src: *const c_void, dst: *const c_void, a_ta: bool, ocvrs_return: *mut Result<()>);
// mulTransposed(InputArray, OutputArray, bool, InputArray, double, int)(InputArray, OutputArray, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1740
// ("cv::mulTransposed", vec![(pred!(mut, ["src", "dst", "aTa", "delta", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "const cv::_InputArray*", "double", "int"]), _)]),
pub fn cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(src: *const c_void, dst: *const c_void, a_ta: bool, delta: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result<()>);
// cv::multiply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:362
// ("cv::multiply", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// multiply(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:362
// ("cv::multiply", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
pub fn cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result<()>);
// noArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:588
// ("cv::noArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_noArray() -> *mut c_void;
// norm(const SparseMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:676
// ("cv::norm", vec![(pred!(mut, ["src", "normType"], ["const cv::SparseMat*", "int"]), _)]),
pub fn cv_norm_const_SparseMatR_int(src: *const c_void, norm_type: i32, ocvrs_return: *mut Result<f64>);
// cv::norm(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:657
// ("cv::norm", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
pub fn cv_norm_const__InputArrayR(src1: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::norm(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:670
// ("cv::norm", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_norm_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<f64>);
// norm(InputArray, InputArray, int, InputArray)(InputArray, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:670
// ("cv::norm", vec![(pred!(mut, ["src1", "src2", "normType", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(src1: *const c_void, src2: *const c_void, norm_type: i32, mask: *const c_void, ocvrs_return: *mut Result<f64>);
// norm(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:657
// ("cv::norm", vec![(pred!(mut, ["src1", "normType", "mask"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_norm_const__InputArrayR_int_const__InputArrayR(src1: *const c_void, norm_type: i32, mask: *const c_void, ocvrs_return: *mut Result<f64>);
// normalize(const SparseMat &, SparseMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:778
// ("cv::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "normType"], ["const cv::SparseMat*", "cv::SparseMat*", "double", "int"]), _)]),
pub fn cv_normalize_const_SparseMatR_SparseMatR_double_int(src: *const c_void, dst: *mut c_void, alpha: f64, norm_type: i32, ocvrs_return: *mut Result<()>);
// cv::normalize(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:768
// ("cv::normalize", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_normalize_const__InputArrayR_const__InputOutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// normalize(InputArray, InputOutputArray, double, double, int, int, InputArray)(InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:768
// ("cv::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double", "int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: *const c_void, ocvrs_return: *mut Result<()>);
// attachContext(const String &, void *, void *, void *)(InString, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:361
// ("cv::ocl::attachContext", vec![(pred!(mut, ["platformName", "platformID", "context", "deviceID"], ["const cv::String*", "void*", "void*", "void*"]), _)]),
pub fn cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(platform_name: *const c_char, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void, ocvrs_return: *mut Result<()>);
// buildOptionsAddMatrixDescription(String &, const String &, InputArray)(OutString, InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:738
// ("cv::ocl::buildOptionsAddMatrixDescription", vec![(pred!(mut, ["buildOptions", "name", "_m"], ["cv::String*", "const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(build_options: *mut *mut c_void, name: *const c_char, _m: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ocl::checkOptimalVectorWidth(Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:727
// ("cv::ocl::checkOptimalVectorWidth", vec![(pred!(mut, ["vectorWidths", "src1"], ["const int*", "const cv::_InputArray*"]), _)]),
pub fn cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR(vector_widths: *const i32, src1: *const c_void, ocvrs_return: *mut Result<i32>);
// checkOptimalVectorWidth(const int *, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OclVectorStrategy)(Indirect, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:727
// ("cv::ocl::checkOptimalVectorWidth", vec![(pred!(mut, ["vectorWidths", "src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9", "strat"], ["const int*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::ocl::OclVectorStrategy"]), _)]),
pub fn cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(vector_widths: *const i32, src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, strat: core::OclVectorStrategy, ocvrs_return: *mut Result<i32>);
// convertFromBuffer(void *, size_t, int, int, int, UMat &)(Indirect, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:375
// ("cv::ocl::convertFromBuffer", vec![(pred!(mut, ["cl_mem_buffer", "step", "rows", "cols", "type", "dst"], ["void*", "size_t", "int", "int", "int", "cv::UMat*"]), _)]),
pub fn cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(cl_mem_buffer: *mut c_void, step: size_t, rows: i32, cols: i32, typ: i32, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// convertFromImage(void *, UMat &)(Indirect, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:384
// ("cv::ocl::convertFromImage", vec![(pred!(mut, ["cl_mem_image", "dst"], ["void*", "cv::UMat*"]), _)]),
pub fn cv_ocl_convertFromImage_voidX_UMatR(cl_mem_image: *mut c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTypeStr(int, int, int, char *)(Primitive, Primitive, Primitive, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:700
// ("cv::ocl::convertTypeStr", vec![(pred!(mut, ["sdepth", "ddepth", "cn", "buf"], ["int", "int", "int", "char*"]), _)]),
pub fn cv_ocl_convertTypeStr_int_int_int_charX(sdepth: i32, ddepth: i32, cn: i32, buf: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// convertTypeStr(int, int, int, char *, size_t)(Primitive, Primitive, Primitive, OutString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:701
// ("cv::ocl::convertTypeStr", vec![(pred!(mut, ["sdepth", "ddepth", "cn", "buf", "buf_size"], ["int", "int", "int", "char*", "size_t"]), _)]),
pub fn cv_ocl_convertTypeStr_int_int_int_charX_size_t(sdepth: i32, ddepth: i32, cn: i32, buf: *mut *mut c_void, buf_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// finish()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:59
// ("cv::ocl::finish", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_finish(ocvrs_return: *mut Result<()>);
// getOpenCLErrorString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:705
// ("cv::ocl::getOpenCLErrorString", vec![(pred!(mut, ["errorCode"], ["int"]), _)]),
pub fn cv_ocl_getOpenCLErrorString_int(error_code: i32, ocvrs_return: *mut Result<*mut c_void>);
// getPlatfomsInfo(std::vector<PlatformInfo> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:707
// ("cv::ocl::getPlatfomsInfo", vec![(pred!(mut, ["platform_info"], ["std::vector<cv::ocl::PlatformInfo>*"]), _)]),
pub fn cv_ocl_getPlatfomsInfo_vectorLPlatformInfoGR(platform_info: *mut c_void, ocvrs_return: *mut Result<()>);
// haveAmdBlas()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:56
// ("cv::ocl::haveAmdBlas", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_haveAmdBlas(ocvrs_return: *mut Result<bool>);
// haveAmdFft()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:57
// ("cv::ocl::haveAmdFft", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_haveAmdFft(ocvrs_return: *mut Result<bool>);
// haveOpenCL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:54
// ("cv::ocl::haveOpenCL", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_haveOpenCL(ocvrs_return: *mut Result<bool>);
// haveSVM()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:61
// ("cv::ocl::haveSVM", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_haveSVM(ocvrs_return: *mut Result<bool>);
// cv::ocl::kernelToStr(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:706
// ("cv::ocl::kernelToStr", vec![(pred!(mut, ["_kernel"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ocl_kernelToStr_const__InputArrayR(_kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// kernelToStr(InputArray, int, const char *)(InputArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:706
// ("cv::ocl::kernelToStr", vec![(pred!(mut, ["_kernel", "ddepth", "name"], ["const cv::_InputArray*", "int", "const char*"]), _)]),
pub fn cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(_kernel: *const c_void, ddepth: i32, name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// memopTypeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:703
// ("cv::ocl::memopTypeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
pub fn cv_ocl_memopTypeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::predictOptimalVectorWidthMax(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:734
// ("cv::ocl::predictOptimalVectorWidthMax", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR(src1: *const c_void, ocvrs_return: *mut Result<i32>);
// predictOptimalVectorWidthMax(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:734
// ("cv::ocl::predictOptimalVectorWidthMax", vec![(pred!(mut, ["src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::ocl::predictOptimalVectorWidth(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:722
// ("cv::ocl::predictOptimalVectorWidth", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ocl_predictOptimalVectorWidth_const__InputArrayR(src1: *const c_void, ocvrs_return: *mut Result<i32>);
// predictOptimalVectorWidth(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OclVectorStrategy)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:722
// ("cv::ocl::predictOptimalVectorWidth", vec![(pred!(mut, ["src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9", "strat"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::ocl::OclVectorStrategy"]), _)]),
pub fn cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, strat: core::OclVectorStrategy, ocvrs_return: *mut Result<i32>);
// setUseOpenCL(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:58
// ("cv::ocl::setUseOpenCL", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ocl_setUseOpenCL_bool(flag: bool, ocvrs_return: *mut Result<()>);
// typeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:702
// ("cv::ocl::typeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
pub fn cv_ocl_typeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
// useOpenCL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:55
// ("cv::ocl::useOpenCL", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_useOpenCL(ocvrs_return: *mut Result<bool>);
// vecopTypeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:704
// ("cv::ocl::vecopTypeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
pub fn cv_ocl_vecopTypeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
// convertFromGLTexture2D(const Texture2D &, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:538
// ("cv::ogl::convertFromGLTexture2D", vec![(pred!(mut, ["texture", "dst"], ["const cv::ogl::Texture2D*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ogl_convertFromGLTexture2D_const_Texture2DR_const__OutputArrayR(texture: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertToGLTexture2D(InputArray, Texture2D &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:532
// ("cv::ogl::convertToGLTexture2D", vec![(pred!(mut, ["src", "texture"], ["const cv::_InputArray*", "cv::ogl::Texture2D*"]), _)]),
pub fn cv_ogl_convertToGLTexture2D_const__InputArrayR_Texture2DR(src: *const c_void, texture: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ogl::mapGLBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:551
// ("cv::ogl::mapGLBuffer", vec![(pred!(mut, ["buffer"], ["const cv::ogl::Buffer*"]), _)]),
pub fn cv_ogl_mapGLBuffer_const_BufferR(buffer: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mapGLBuffer(const Buffer &, AccessFlag)(TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:551
// ("cv::ogl::mapGLBuffer", vec![(pred!(mut, ["buffer", "accessFlags"], ["const cv::ogl::Buffer*", "cv::AccessFlag"]), _)]),
pub fn cv_ogl_mapGLBuffer_const_BufferR_AccessFlag(buffer: *const c_void, access_flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
// initializeContextFromGL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:524
// ("cv::ogl::ocl::initializeContextFromGL", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_ocl_initializeContextFromGL(ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::render(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:505
// ("cv::ogl::render", vec![(pred!(mut, ["arr"], ["const cv::ogl::Arrays*"]), _)]),
pub fn cv_ogl_render_const_ArraysR(arr: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ogl::render(TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:513
// ("cv::ogl::render", vec![(pred!(mut, ["arr", "indices"], ["const cv::ogl::Arrays*", "const cv::_InputArray*"]), _)]),
pub fn cv_ogl_render_const_ArraysR_const__InputArrayR(arr: *const c_void, indices: *const c_void, ocvrs_return: *mut Result<()>);
// render(const Arrays &, InputArray, int, Scalar)(TraitClass, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:513
// ("cv::ogl::render", vec![(pred!(mut, ["arr", "indices", "mode", "color"], ["const cv::ogl::Arrays*", "const cv::_InputArray*", "int", "cv::Scalar"]), _)]),
pub fn cv_ogl_render_const_ArraysR_const__InputArrayR_int_Scalar(arr: *const c_void, indices: *const c_void, mode: i32, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// render(const Arrays &, int, Scalar)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:505
// ("cv::ogl::render", vec![(pred!(mut, ["arr", "mode", "color"], ["const cv::ogl::Arrays*", "int", "cv::Scalar"]), _)]),
pub fn cv_ogl_render_const_ArraysR_int_Scalar(arr: *const c_void, mode: i32, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::ogl::render(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:496
// ("cv::ogl::render", vec![(pred!(mut, ["tex"], ["const cv::ogl::Texture2D*"]), _)]),
pub fn cv_ogl_render_const_Texture2DR(tex: *const c_void, ocvrs_return: *mut Result<()>);
// render(const Texture2D &, Rect_<double>, Rect_<double>)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:496
// ("cv::ogl::render", vec![(pred!(mut, ["tex", "wndRect", "texRect"], ["const cv::ogl::Texture2D*", "cv::Rect_<double>", "cv::Rect_<double>"]), _)]),
pub fn cv_ogl_render_const_Texture2DR_Rect_LdoubleG_Rect_LdoubleG(tex: *const c_void, wnd_rect: *const core::Rect_<f64>, tex_rect: *const core::Rect_<f64>, ocvrs_return: *mut Result<()>);
// unmapGLBuffer(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:559
// ("cv::ogl::unmapGLBuffer", vec![(pred!(mut, ["u"], ["cv::UMat*"]), _)]),
pub fn cv_ogl_unmapGLBuffer_UMatR(u: *mut c_void, ocvrs_return: *mut Result<()>);
// operator+(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3930
// ("cv::operator+", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorA_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3926
// ("cv::operator+", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
pub fn cv_operatorA_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const MatExpr &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3928
// ("cv::operator+", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorA_const_MatExprR_const_ScalarR(e: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3927
// ("cv::operator+", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorA_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3923
// ("cv::operator+", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorA_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3924
// ("cv::operator+", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorA_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const Scalar &, const MatExpr &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3929
// ("cv::operator+", vec![(pred!(mut, ["s", "e"], ["const cv::Scalar*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorA_const_ScalarR_const_MatExprR(s: *const core::Scalar, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3925
// ("cv::operator+", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
pub fn cv_operatorA_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3972
// ("cv::operator/", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorD_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3968
// ("cv::operator/", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
pub fn cv_operatorD_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3970
// ("cv::operator/", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "double"]), _)]),
pub fn cv_operatorD_const_MatExprR_double(e: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3969
// ("cv::operator/", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorD_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3965
// ("cv::operator/", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorD_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3966
// ("cv::operator/", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorD_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator/(double, const MatExpr &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3971
// ("cv::operator/", vec![(pred!(mut, ["s", "e"], ["double", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorD_double_const_MatExprR(s: f64, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3967
// ("cv::operator/", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorD_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const FileNodeIterator &, const FileNodeIterator &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:1257
// ("cv::operator==", vec![(pred!(mut, ["it1", "it2"], ["const cv::FileNodeIterator*", "const cv::FileNodeIterator*"]), _)]),
pub fn cv_operatorEQ_const_FileNodeIteratorR_const_FileNodeIteratorR(it1: *const c_void, it2: *const c_void, ocvrs_return: *mut Result<bool>);
// operator==(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3994
// ("cv::operator==", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorEQ_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3995
// ("cv::operator==", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorEQ_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const MatShape &, const MatShape &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:174
// ("cv::operator==", vec![(pred!(mut, ["shape1", "shape2"], ["const cv::MatShape*", "const cv::MatShape*"]), _)]),
pub fn cv_operatorEQ_const_MatShapeR_const_MatShapeR(shape1: *const c_void, shape2: *const c_void, ocvrs_return: *mut Result<bool>);
// operator==(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3996
// ("cv::operator==", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorEQ_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4010
// ("cv::operator>=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorGE_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4011
// ("cv::operator>=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorGE_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4012
// ("cv::operator>=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorGE_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4018
// ("cv::operator>", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorG_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4019
// ("cv::operator>", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorG_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator>(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4020
// ("cv::operator>", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorG_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3986
// ("cv::operator<=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorLE_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3987
// ("cv::operator<=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorLE_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3988
// ("cv::operator<=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorLE_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3978
// ("cv::operator<", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorL_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3979
// ("cv::operator<", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorL_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator<(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3980
// ("cv::operator<", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorL_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const FileNodeIterator &, const FileNodeIterator &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:1258
// ("cv::operator!=", vec![(pred!(mut, ["it1", "it2"], ["const cv::FileNodeIterator*", "const cv::FileNodeIterator*"]), _)]),
pub fn cv_operatorNE_const_FileNodeIteratorR_const_FileNodeIteratorR(it1: *const c_void, it2: *const c_void, ocvrs_return: *mut Result<bool>);
// operator!=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4002
// ("cv::operator!=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorNE_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4003
// ("cv::operator!=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorNE_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const MatShape &, const MatShape &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:175
// ("cv::operator!=", vec![(pred!(mut, ["shape1", "shape2"], ["const cv::MatShape*", "const cv::MatShape*"]), _)]),
pub fn cv_operatorNE_const_MatShapeR_const_MatShapeR(shape1: *const c_void, shape2: *const c_void, ocvrs_return: *mut Result<bool>);
// operator!=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4004
// ("cv::operator!=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorNE_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator~(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4050
// ("cv::operator~", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_operatorNOTB_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4034
// ("cv::operator|", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorOR_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4035
// ("cv::operator|", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorOR_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4036
// ("cv::operator|", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
pub fn cv_operatorOR_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4026
// ("cv::operator&", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorR_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4027
// ("cv::operator&", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorR_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4028
// ("cv::operator&", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
pub fn cv_operatorR_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3950
// ("cv::operator-", vec![(pred!(mut, ["e"], ["const cv::MatExpr*"]), _)]),
pub fn cv_operatorS_const_MatExprR(e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3943
// ("cv::operator-", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorS_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3939
// ("cv::operator-", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
pub fn cv_operatorS_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const MatExpr &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3941
// ("cv::operator-", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorS_const_MatExprR_const_ScalarR(e: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3949
// ("cv::operator-", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_operatorS_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3940
// ("cv::operator-", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorS_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3936
// ("cv::operator-", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorS_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3937
// ("cv::operator-", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorS_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Scalar &, const MatExpr &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3942
// ("cv::operator-", vec![(pred!(mut, ["s", "e"], ["const cv::Scalar*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorS_const_ScalarR_const_MatExprR(s: *const core::Scalar, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3938
// ("cv::operator-", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
pub fn cv_operatorS_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4042
// ("cv::operator^", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorXOR_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4043
// ("cv::operator^", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
pub fn cv_operatorXOR_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:4044
// ("cv::operator^", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
pub fn cv_operatorXOR_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3959
// ("cv::operator*", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorX_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3955
// ("cv::operator*", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
pub fn cv_operatorX_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3957
// ("cv::operator*", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "double"]), _)]),
pub fn cv_operatorX_const_MatExprR_double(e: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3956
// ("cv::operator*", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorX_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3952
// ("cv::operator*", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_operatorX_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3953
// ("cv::operator*", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_operatorX_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator*(double, const MatExpr &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3958
// ("cv::operator*", vec![(pred!(mut, ["s", "e"], ["double", "const cv::MatExpr*"]), _)]),
pub fn cv_operatorX_double_const_MatExprR(s: f64, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3954
// ("cv::operator*", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
pub fn cv_operatorX_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::parallel_for_(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:657
// ("cv::parallel_for_", vec![(pred!(mut, ["range", "body"], ["const cv::Range*", "const cv::ParallelLoopBody*"]), _)]),
pub fn cv_parallel_for__const_RangeR_const_ParallelLoopBodyR(range: *const c_void, body: *const c_void, ocvrs_return: *mut Result<()>);
// parallel_for_(const Range &, const ParallelLoopBody &, double)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:657
// ("cv::parallel_for_", vec![(pred!(mut, ["range", "body", "nstripes"], ["const cv::Range*", "const cv::ParallelLoopBody*", "double"]), _)]),
pub fn cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(range: *const c_void, body: *const c_void, nstripes: f64, ocvrs_return: *mut Result<()>);
// cv::patchNaNs(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1670
// ("cv::patchNaNs", vec![(pred!(mut, ["a"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_patchNaNs_const__InputOutputArrayR(a: *const c_void, ocvrs_return: *mut Result<()>);
// patchNaNs(InputOutputArray, double)(InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1670
// ("cv::patchNaNs", vec![(pred!(mut, ["a", "val"], ["const cv::_InputOutputArray*", "double"]), _)]),
pub fn cv_patchNaNs_const__InputOutputArrayR_double(a: *const c_void, val: f64, ocvrs_return: *mut Result<()>);
// perspectiveTransform(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1816
// ("cv::perspectiveTransform", vec![(pred!(mut, ["src", "dst", "m"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// cv::phase(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1632
// ("cv::phase", vec![(pred!(mut, ["x", "y", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// phase(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1632
// ("cv::phase", vec![(pred!(mut, ["x", "y", "angle", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(x: *const c_void, y: *const c_void, angle: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<()>);
// cv::polarToCart(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1591
// ("cv::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(magnitude: *const c_void, angle: *const c_void, x: *const c_void, y: *const c_void, ocvrs_return: *mut Result<()>);
// polarToCart(InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1591
// ("cv::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(magnitude: *const c_void, angle: *const c_void, x: *const c_void, y: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<()>);
// pow(InputArray, double, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1540
// ("cv::pow", vec![(pred!(mut, ["src", "power", "dst"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_pow_const__InputArrayR_double_const__OutputArrayR(src: *const c_void, power: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::randShuffle(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2361
// ("cv::randShuffle", vec![(pred!(mut, ["dst"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_randShuffle_const__InputOutputArrayR(dst: *const c_void, ocvrs_return: *mut Result<()>);
// randShuffle(InputOutputArray, double, RNG *)(InputOutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2361
// ("cv::randShuffle", vec![(pred!(mut, ["dst", "iterFactor", "rng"], ["const cv::_InputOutputArray*", "double", "cv::RNG*"]), _)]),
pub fn cv_randShuffle_const__InputOutputArrayR_double_RNGX(dst: *const c_void, iter_factor: f64, rng: *mut c_void, ocvrs_return: *mut Result<()>);
// randn(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2348
// ("cv::randn", vec![(pred!(mut, ["dst", "mean", "stddev"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst: *const c_void, mean: *const c_void, stddev: *const c_void, ocvrs_return: *mut Result<()>);
// randu(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2335
// ("cv::randu", vec![(pred!(mut, ["dst", "low", "high"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst: *const c_void, low: *const c_void, high: *const c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &, DMatch &, const DMatch &)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:690
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "cv::DMatch*", "const cv::DMatch*"]), _)]),
pub fn cv_read_const_FileNodeR_DMatchR_const_DMatchR(node: *const c_void, value: *mut core::DMatch, default_value: *const core::DMatch, ocvrs_return: *mut Result<()>);
// read(const FileNode &, KeyPoint &, const KeyPoint &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:689
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "cv::KeyPoint*", "const cv::KeyPoint*"]), _)]),
pub fn cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(node: *const c_void, value: *mut c_void, default_value: *const c_void, ocvrs_return: *mut Result<()>);
// cv::read(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:683
// ("cv::read", vec![(pred!(mut, ["node", "mat"], ["const cv::FileNode*", "cv::Mat*"]), _)]),
pub fn cv_read_const_FileNodeR_MatR(node: *const c_void, mat: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &, Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:683
// ("cv::read", vec![(pred!(mut, ["node", "mat", "default_mat"], ["const cv::FileNode*", "cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_read_const_FileNodeR_MatR_const_MatR(node: *const c_void, mat: *mut c_void, default_mat: *const c_void, ocvrs_return: *mut Result<()>);
// cv::read(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:684
// ("cv::read", vec![(pred!(mut, ["node", "mat"], ["const cv::FileNode*", "cv::SparseMat*"]), _)]),
pub fn cv_read_const_FileNodeR_SparseMatR(node: *const c_void, mat: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &, SparseMat &, const SparseMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:684
// ("cv::read", vec![(pred!(mut, ["node", "mat", "default_mat"], ["const cv::FileNode*", "cv::SparseMat*", "const cv::SparseMat*"]), _)]),
pub fn cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(node: *const c_void, mat: *mut c_void, default_mat: *const c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &, double &, double)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:681
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "double*", "double"]), _)]),
pub fn cv_read_const_FileNodeR_doubleR_double(node: *const c_void, value: *mut f64, default_value: f64, ocvrs_return: *mut Result<()>);
// read(const FileNode &, float &, float)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:680
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "float*", "float"]), _)]),
pub fn cv_read_const_FileNodeR_floatR_float(node: *const c_void, value: *mut f32, default_value: f32, ocvrs_return: *mut Result<()>);
// read(const FileNode &, int64_t &, int64_t)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:679
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "int64_t*", "int64_t"]), _)]),
pub fn cv_read_const_FileNodeR_int64_tR_int64_t(node: *const c_void, value: *mut i64, default_value: i64, ocvrs_return: *mut Result<()>);
// read(const FileNode &, int &, int)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:678
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "int*", "int"]), _)]),
pub fn cv_read_const_FileNodeR_intR_int(node: *const c_void, value: *mut i32, default_value: i32, ocvrs_return: *mut Result<()>);
// read(const FileNode &, std::string &, const std::string &)(TraitClass, OutString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:682
// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "std::string*", "const std::string*"]), _)]),
pub fn cv_read_const_FileNodeR_stringR_const_stringR(node: *const c_void, value: *mut *mut c_void, default_value: *const c_char, ocvrs_return: *mut Result<()>);
// read(const FileNode &, std::vector<DMatch> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:687
// ("cv::read", vec![(pred!(mut, ["node", "matches"], ["const cv::FileNode*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_read_const_FileNodeR_vectorLDMatchGR(node: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:686
// ("cv::read", vec![(pred!(mut, ["node", "keypoints"], ["const cv::FileNode*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_read_const_FileNodeR_vectorLKeyPointGR(node: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// rectangleIntersectionArea(const Rect2d &, const Rect2d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:2062
// ("cv::rectangleIntersectionArea", vec![(pred!(mut, ["a", "b"], ["const cv::Rect2d*", "const cv::Rect2d*"]), _)]),
pub fn cv_rectangleIntersectionArea_const_Rect2dR_const_Rect2dR(a: *const core::Rect2d, b: *const core::Rect2d, ocvrs_return: *mut Result<f64>);
// cv::reduceArgMax(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:842
// ("cv::reduceArgMax", vec![(pred!(mut, ["src", "dst", "axis"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_reduceArgMax_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, axis: i32, ocvrs_return: *mut Result<()>);
// reduceArgMax(InputArray, OutputArray, int, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:842
// ("cv::reduceArgMax", vec![(pred!(mut, ["src", "dst", "axis", "lastIndex"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_reduceArgMax_const__InputArrayR_const__OutputArrayR_int_bool(src: *const c_void, dst: *const c_void, axis: i32, last_index: bool, ocvrs_return: *mut Result<()>);
// cv::reduceArgMin(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:825
// ("cv::reduceArgMin", vec![(pred!(mut, ["src", "dst", "axis"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_reduceArgMin_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, axis: i32, ocvrs_return: *mut Result<()>);
// reduceArgMin(InputArray, OutputArray, int, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:825
// ("cv::reduceArgMin", vec![(pred!(mut, ["src", "dst", "axis", "lastIndex"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_reduceArgMin_const__InputArrayR_const__OutputArrayR_int_bool(src: *const c_void, dst: *const c_void, axis: i32, last_index: bool, ocvrs_return: *mut Result<()>);
// cv::reduce(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:904
// ("cv::reduce", vec![(pred!(mut, ["src", "dst", "dim", "rtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_reduce_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, dim: i32, rtype: i32, ocvrs_return: *mut Result<()>);
// reduce(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:904
// ("cv::reduce", vec![(pred!(mut, ["src", "dst", "dim", "rtype", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, dim: i32, rtype: i32, dtype: i32, ocvrs_return: *mut Result<()>);
// repeat(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1146
// ("cv::repeat", vec![(pred!(mut, ["src", "ny", "nx"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_repeat_const_MatR_int_int(src: *const c_void, ny: i32, nx: i32, ocvrs_return: *mut Result<*mut c_void>);
// repeat(InputArray, int, int, OutputArray)(InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1137
// ("cv::repeat", vec![(pred!(mut, ["src", "ny", "nx", "dst"], ["const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(src: *const c_void, ny: i32, nx: i32, dst: *const c_void, ocvrs_return: *mut Result<()>);
// rotate(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1122
// ("cv::rotate", vec![(pred!(mut, ["src", "dst", "rotateCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_rotate_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, rotate_code: i32, ocvrs_return: *mut Result<()>);
// addSamplesDataSearchPath(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1272
// ("cv::samples::addSamplesDataSearchPath", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_samples_addSamplesDataSearchPath_const_StringR(path: *const c_char, ocvrs_return: *mut Result<()>);
// addSamplesDataSearchSubDirectory(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1281
// ("cv::samples::addSamplesDataSearchSubDirectory", vec![(pred!(mut, ["subdir"], ["const cv::String*"]), _)]),
pub fn cv_samples_addSamplesDataSearchSubDirectory_const_StringR(subdir: *const c_char, ocvrs_return: *mut Result<()>);
// cv::samples::findFileOrKeep(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1255
// ("cv::samples::findFileOrKeep", vec![(pred!(mut, ["relative_path"], ["const cv::String*"]), _)]),
pub fn cv_samples_findFileOrKeep_const_StringR(relative_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// findFileOrKeep(const cv::String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1255
// ("cv::samples::findFileOrKeep", vec![(pred!(mut, ["relative_path", "silentMode"], ["const cv::String*", "bool"]), _)]),
pub fn cv_samples_findFileOrKeep_const_StringR_bool(relative_path: *const c_char, silent_mode: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::samples::findFile(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1253
// ("cv::samples::findFile", vec![(pred!(mut, ["relative_path"], ["const cv::String*"]), _)]),
pub fn cv_samples_findFile_const_StringR(relative_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// findFile(const cv::String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1253
// ("cv::samples::findFile", vec![(pred!(mut, ["relative_path", "required", "silentMode"], ["const cv::String*", "bool", "bool"]), _)]),
pub fn cv_samples_findFile_const_StringR_bool_bool(relative_path: *const c_char, required: bool, silent_mode: bool, ocvrs_return: *mut Result<*mut c_void>);
// scaleAdd(InputArray, double, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:418
// ("cv::scaleAdd", vec![(pred!(mut, ["src1", "alpha", "src2", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(src1: *const c_void, alpha: f64, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// setBreakOnError(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:160
// ("cv::setBreakOnError", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_setBreakOnError_bool(flag: bool, ocvrs_return: *mut Result<bool>);
// cv::setIdentity(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1849
// ("cv::setIdentity", vec![(pred!(mut, ["mtx"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_setIdentity_const__InputOutputArrayR(mtx: *const c_void, ocvrs_return: *mut Result<()>);
// setIdentity(InputOutputArray, const Scalar &)(InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1849
// ("cv::setIdentity", vec![(pred!(mut, ["mtx", "s"], ["const cv::_InputOutputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_setIdentity_const__InputOutputArrayR_const_ScalarR(mtx: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setLogLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:341
// ("cv::setLogLevel", vec![(pred!(mut, ["level"], ["int"]), _)]),
pub fn cv_setLogLevel_int(level: i32, ocvrs_return: *mut Result<i32>);
// setNumThreads(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:231
// ("cv::setNumThreads", vec![(pred!(mut, ["nthreads"], ["int"]), _)]),
pub fn cv_setNumThreads_int(nthreads: i32, ocvrs_return: *mut Result<()>);
// setRNGSeed(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2323
// ("cv::setRNGSeed", vec![(pred!(mut, ["seed"], ["int"]), _)]),
pub fn cv_setRNGSeed_int(seed: i32, ocvrs_return: *mut Result<()>);
// setUseOptimized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:630
// ("cv::setUseOptimized", vec![(pred!(mut, ["onoff"], ["bool"]), _)]),
pub fn cv_setUseOptimized_bool(onoff: bool, ocvrs_return: *mut Result<()>);
// solveCubic(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1975
// ("cv::solveCubic", vec![(pred!(mut, ["coeffs", "roots"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solveCubic_const__InputArrayR_const__OutputArrayR(coeffs: *const c_void, roots: *const c_void, ocvrs_return: *mut Result<i32>);
// solveLP(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:301
// ("cv::solveLP", vec![(pred!(mut, ["Func", "Constr", "z"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR(func: *const c_void, constr: *const c_void, z: *const c_void, ocvrs_return: *mut Result<i32>);
// solveLP(InputArray, InputArray, OutputArray, double)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:298
// ("cv::solveLP", vec![(pred!(mut, ["Func", "Constr", "z", "constr_eps"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(func: *const c_void, constr: *const c_void, z: *const c_void, constr_eps: f64, ocvrs_return: *mut Result<i32>);
// cv::solvePoly(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1985
// ("cv::solvePoly", vec![(pred!(mut, ["coeffs", "roots"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solvePoly_const__InputArrayR_const__OutputArrayR(coeffs: *const c_void, roots: *const c_void, ocvrs_return: *mut Result<f64>);
// solvePoly(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1985
// ("cv::solvePoly", vec![(pred!(mut, ["coeffs", "roots", "maxIters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(coeffs: *const c_void, roots: *const c_void, max_iters: i32, ocvrs_return: *mut Result<f64>);
// cv::solve(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1924
// ("cv::solve", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<bool>);
// solve(InputArray, InputArray, OutputArray, int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1924
// ("cv::solve", vec![(pred!(mut, ["src1", "src2", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// sortIdx(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1960
// ("cv::sortIdx", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// sort(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1940
// ("cv::sort", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_sort_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// split(const Mat &, Mat *)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:950
// ("cv::split", vec![(pred!(mut, ["src", "mvbegin"], ["const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_split_const_MatR_MatX(src: *const c_void, mvbegin: *mut c_void, ocvrs_return: *mut Result<()>);
// split(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:956
// ("cv::split", vec![(pred!(mut, ["m", "mv"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_split_const__InputArrayR_const__OutputArrayR(m: *const c_void, mv: *const c_void, ocvrs_return: *mut Result<()>);
// sqrt(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1515
// ("cv::sqrt", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_sqrt_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::subtract(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:334
// ("cv::subtract", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// subtract(InputArray, InputArray, OutputArray, InputArray, int)(InputArray, InputArray, OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:334
// ("cv::subtract", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, ocvrs_return: *mut Result<()>);
// sum(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:505
// ("cv::sum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_sum_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// swap(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:173
// ("cv::swap", vec![(pred!(mut, ["a", "b"], ["cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_swap_MatR_MatR(a: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result<()>);
// swap(UMat &, UMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:175
// ("cv::swap", vec![(pred!(mut, ["a", "b"], ["cv::UMat*", "cv::UMat*"]), _)]),
pub fn cv_swap_UMatR_UMatR(a: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::tempfile() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:194
// ("cv::tempfile", vec![(pred!(mut, [], []), _)]),
pub fn cv_tempfile(ocvrs_return: *mut Result<*mut c_void>);
// tempfile(const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:194
// ("cv::tempfile", vec![(pred!(mut, ["suffix"], ["const char*"]), _)]),
pub fn cv_tempfile_const_charX(suffix: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// terminate(Error::Code, const String &, const char *, const char *, int)(Enum, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:151
// ("cv::terminate", vec![(pred!(mut, ["code", "err", "func", "file", "line"], ["cv::Error::Code", "const cv::String*", "const char*", "const char*", "int"]), _)]),
pub fn cv_terminate_Code_const_StringR_const_charX_const_charX_int(code: core::Code, err: *const c_char, func: *const c_char, file: *const c_char, line: i32);
// theRNG()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2315
// ("cv::theRNG", vec![(pred!(mut, [], []), _)]),
pub fn cv_theRNG(ocvrs_return: *mut Result<*mut c_void>);
// trace(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1873
// ("cv::trace", vec![(pred!(mut, ["mtx"], ["const cv::_InputArray*"]), _)]),
pub fn cv_trace_const__InputArrayR(mtx: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// transform(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1789
// ("cv::transform", vec![(pred!(mut, ["src", "dst", "m"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// transposeND(InputArray, const std::vector<int> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1763
// ("cv::transposeND", vec![(pred!(mut, ["src", "order", "dst"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_transposeND_const__InputArrayR_const_vectorLintGR_const__OutputArrayR(src: *const c_void, order: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// transpose(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1753
// ("cv::transpose", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_transpose_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// typeToString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:18
// ("cv::typeToString", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_typeToString_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// useOptimized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:636
// ("cv::useOptimized", vec![(pred!(mut, [], []), _)]),
pub fn cv_useOptimized(ocvrs_return: *mut Result<bool>);
// cv::utils::copyMatAndDumpNamedArguments(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:268
// ("cv::utils::copyMatAndDumpNamedArguments", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_utils_copyMatAndDumpNamedArguments_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copyMatAndDumpNamedArguments(InputArray, OutputArray, const FunctionParams &)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:268
// ("cv::utils::copyMatAndDumpNamedArguments", vec![(pred!(mut, ["src", "dst", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::utils::FunctionParams*"]), _)]),
pub fn cv_utils_copyMatAndDumpNamedArguments_const__InputArrayR_const__OutputArrayR_const_FunctionParamsR(src: *const c_void, dst: *const c_void, params: *const core::FunctionParams, ocvrs_return: *mut Result<*mut c_void>);
// dumpBool(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:27
// ("cv::utils::dumpBool", vec![(pred!(mut, ["argument"], ["bool"]), _)]),
pub fn cv_utils_dumpBool_bool(argument: bool, ocvrs_return: *mut Result<*mut c_void>);
// dumpCString(const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:67
// ("cv::utils::dumpCString", vec![(pred!(mut, ["argument"], ["const char*"]), _)]),
pub fn cv_utils_dumpCString_const_charX(argument: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// dumpDouble(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:61
// ("cv::utils::dumpDouble", vec![(pred!(mut, ["argument"], ["double"]), _)]),
pub fn cv_utils_dumpDouble_double(argument: f64, ocvrs_return: *mut Result<*mut c_void>);
// dumpFloat(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:55
// ("cv::utils::dumpFloat", vec![(pred!(mut, ["argument"], ["float"]), _)]),
pub fn cv_utils_dumpFloat_float(argument: f32, ocvrs_return: *mut Result<*mut c_void>);
// dumpInputArrayOfArrays(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:20
// ("cv::utils::dumpInputArrayOfArrays", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
pub fn cv_utils_dumpInputArrayOfArrays_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpInputArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:18
// ("cv::utils::dumpInputArray", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
pub fn cv_utils_dumpInputArray_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpInputOutputArrayOfArrays(InputOutputArrayOfArrays)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:24
// ("cv::utils::dumpInputOutputArrayOfArrays", vec![(pred!(mut, ["argument"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpInputOutputArray(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:22
// ("cv::utils::dumpInputOutputArray", vec![(pred!(mut, ["argument"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_utils_dumpInputOutputArray_const__InputOutputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpInt64(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:39
// ("cv::utils::dumpInt64", vec![(pred!(mut, ["argument"], ["int64_t"]), _)]),
pub fn cv_utils_dumpInt64_int64_t(argument: i64, ocvrs_return: *mut Result<*mut c_void>);
// dumpInt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:33
// ("cv::utils::dumpInt", vec![(pred!(mut, ["argument"], ["int"]), _)]),
pub fn cv_utils_dumpInt_int(argument: i32, ocvrs_return: *mut Result<*mut c_void>);
// dumpRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:101
// ("cv::utils::dumpRange", vec![(pred!(mut, ["argument"], ["const cv::Range*"]), _)]),
pub fn cv_utils_dumpRange_const_RangeR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpRect(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:79
// ("cv::utils::dumpRect", vec![(pred!(mut, ["argument"], ["const cv::Rect*"]), _)]),
pub fn cv_utils_dumpRect_const_RectR(argument: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// dumpRotatedRect(const RotatedRect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:93
// ("cv::utils::dumpRotatedRect", vec![(pred!(mut, ["argument"], ["const cv::RotatedRect*"]), _)]),
pub fn cv_utils_dumpRotatedRect_const_RotatedRectR(argument: *const core::RotatedRect, ocvrs_return: *mut Result<*mut c_void>);
// dumpSizeT(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:47
// ("cv::utils::dumpSizeT", vec![(pred!(mut, ["argument"], ["size_t"]), _)]),
pub fn cv_utils_dumpSizeT_size_t(argument: size_t, ocvrs_return: *mut Result<*mut c_void>);
// dumpString(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:73
// ("cv::utils::dumpString", vec![(pred!(mut, ["argument"], ["const cv::String*"]), _)]),
pub fn cv_utils_dumpString_const_StringR(argument: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// dumpTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:86
// ("cv::utils::dumpTermCriteria", vec![(pred!(mut, ["argument"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_utils_dumpTermCriteria_const_TermCriteriaR(argument: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::dumpVec2i() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:234
// ("cv::utils::dumpVec2i", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_dumpVec2i(ocvrs_return: *mut Result<*mut c_void>);
// dumpVec2i(const cv::Vec2i)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:234
// ("cv::utils::dumpVec2i", vec![(pred!(mut, ["value"], ["const cv::Vec2i"]), _)]),
pub fn cv_utils_dumpVec2i_const_Vec2i(value: *const core::Vec2i, ocvrs_return: *mut Result<*mut c_void>);
// dumpVectorOfDouble(const std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:115
// ("cv::utils::dumpVectorOfDouble", vec![(pred!(mut, ["vec"], ["const std::vector<double>*"]), _)]),
pub fn cv_utils_dumpVectorOfDouble_const_vectorLdoubleGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpVectorOfInt(const std::vector<int> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:113
// ("cv::utils::dumpVectorOfInt", vec![(pred!(mut, ["vec"], ["const std::vector<int>*"]), _)]),
pub fn cv_utils_dumpVectorOfInt_const_vectorLintGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpVectorOfRect(const std::vector<Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:117
// ("cv::utils::dumpVectorOfRect", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Rect>*"]), _)]),
pub fn cv_utils_dumpVectorOfRect_const_vectorLRectGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCacheDirectoryForDownloads()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:332
// ("cv::utils::fs::getCacheDirectoryForDownloads", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_fs_getCacheDirectoryForDownloads(ocvrs_return: *mut Result<*mut c_void>);
// generateVectorOfInt(size_t, std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:177
// ("cv::utils::generateVectorOfInt", vec![(pred!(mut, ["len", "vec"], ["size_t", "std::vector<int>*"]), _)]),
pub fn cv_utils_generateVectorOfInt_size_t_vectorLintGR(len: size_t, vec: *mut c_void, ocvrs_return: *mut Result<()>);
// generateVectorOfMat(size_t, int, int, int, std::vector<Mat> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:190
// ("cv::utils::generateVectorOfMat", vec![(pred!(mut, ["len", "rows", "cols", "dtype", "vec"], ["size_t", "int", "int", "int", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_utils_generateVectorOfMat_size_t_int_int_int_vectorLMatGR(len: size_t, rows: i32, cols: i32, dtype: i32, vec: *mut c_void, ocvrs_return: *mut Result<()>);
// generateVectorOfRect(size_t, std::vector<Rect> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:164
// ("cv::utils::generateVectorOfRect", vec![(pred!(mut, ["len", "vec"], ["size_t", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_utils_generateVectorOfRect_size_t_vectorLRectGR(len: size_t, vec: *mut c_void, ocvrs_return: *mut Result<()>);
// getThreadID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1288
// ("cv::utils::getThreadID", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_getThreadID(ocvrs_return: *mut Result<i32>);
// getLogLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:27
// ("cv::utils::logging::getLogLevel", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_logging_getLogLevel(ocvrs_return: *mut Result<core::LogLevel>);
// getLogTagLevel(const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:33
// ("cv::utils::logging::getLogTagLevel", vec![(pred!(mut, ["tag"], ["const char*"]), _)]),
pub fn cv_utils_logging_getLogTagLevel_const_charX(tag: *const c_char, ocvrs_return: *mut Result<core::LogLevel>);
// getGlobalLogTag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:38
// ("cv::utils::logging::internal::getGlobalLogTag", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_logging_internal_getGlobalLogTag(ocvrs_return: *mut Result<*mut c_void>);
// writeLogMessageEx(LogLevel, const char *, const char *, int, const char *, const char *)(Enum, InString, InString, Primitive, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:44
// ("cv::utils::logging::internal::writeLogMessageEx", vec![(pred!(mut, ["logLevel", "tag", "file", "line", "func", "message"], ["cv::utils::logging::LogLevel", "const char*", "const char*", "int", "const char*", "const char*"]), _)]),
pub fn cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(log_level: core::LogLevel, tag: *const c_char, file: *const c_char, line: i32, func: *const c_char, message: *const c_char, ocvrs_return: *mut Result<()>);
// writeLogMessage(LogLevel, const char *)(Enum, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:41
// ("cv::utils::logging::internal::writeLogMessage", vec![(pred!(mut, ["logLevel", "message"], ["cv::utils::logging::LogLevel", "const char*"]), _)]),
pub fn cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(log_level: core::LogLevel, message: *const c_char, ocvrs_return: *mut Result<()>);
// registerLogTag(cv::utils::logging::LogTag *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:29
// ("cv::utils::logging::registerLogTag", vec![(pred!(mut, ["plogtag"], ["cv::utils::logging::LogTag*"]), _)]),
pub fn cv_utils_logging_registerLogTag_LogTagX(plogtag: *mut c_void, ocvrs_return: *mut Result<()>);
// setLogLevel(LogLevel)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:25
// ("cv::utils::logging::setLogLevel", vec![(pred!(mut, ["logLevel"], ["cv::utils::logging::LogLevel"]), _)]),
pub fn cv_utils_logging_setLogLevel_LogLevel(log_level: core::LogLevel, ocvrs_return: *mut Result<core::LogLevel>);
// setLogTagLevel(const char *, cv::utils::logging::LogLevel)(InString, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logger.hpp:31
// ("cv::utils::logging::setLogTagLevel", vec![(pred!(mut, ["tag", "level"], ["const char*", "cv::utils::logging::LogLevel"]), _)]),
pub fn cv_utils_logging_setLogTagLevel_const_charX_LogLevel(tag: *const c_char, level: core::LogLevel, ocvrs_return: *mut Result<()>);
// testEchoBooleanFunction(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:277
// ("cv::utils::nested::testEchoBooleanFunction", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_utils_nested_testEchoBooleanFunction_bool(flag: bool, ocvrs_return: *mut Result<bool>);
// testAsyncArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:211
// ("cv::utils::testAsyncArray", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
pub fn cv_utils_testAsyncArray_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// testAsyncException()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:219
// ("cv::utils::testAsyncException", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_testAsyncException(ocvrs_return: *mut Result<*mut c_void>);
// testOverloadResolution(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:130
// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["rect"], ["const cv::Rect*"]), _)]),
pub fn cv_utils_testOverloadResolution_const_RectR(rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::testOverloadResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:123
// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_utils_testOverloadResolution_int(value: i32, ocvrs_return: *mut Result<*mut c_void>);
// testOverloadResolution(int, const Point &)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:123
// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["value", "point"], ["int", "const cv::Point*"]), _)]),
pub fn cv_utils_testOverloadResolution_int_const_PointR(value: i32, point: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// testOverwriteNativeMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:152
// ("cv::utils::testOverwriteNativeMethod", vec![(pred!(mut, ["argument"], ["int"]), _)]),
pub fn cv_utils_testOverwriteNativeMethod_int(argument: i32, ocvrs_return: *mut Result<i32>);
// testRaiseGeneralException()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:205
// ("cv::utils::testRaiseGeneralException", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_testRaiseGeneralException(ocvrs_return: *mut Result<()>);
// cv::utils::testReservedKeywordConversion(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:158
// ("cv::utils::testReservedKeywordConversion", vec![(pred!(mut, ["positional_argument"], ["int"]), _)]),
pub fn cv_utils_testReservedKeywordConversion_int(positional_argument: i32, ocvrs_return: *mut Result<*mut c_void>);
// testReservedKeywordConversion(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:158
// ("cv::utils::testReservedKeywordConversion", vec![(pred!(mut, ["positional_argument", "lambda", "from"], ["int", "int", "int"]), _)]),
pub fn cv_utils_testReservedKeywordConversion_int_int_int(positional_argument: i32, lambda: i32, from: i32, ocvrs_return: *mut Result<*mut c_void>);
// testRotatedRectVector(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:143
// ("cv::utils::testRotatedRectVector", vec![(pred!(mut, ["x", "y", "w", "h", "angle"], ["float", "float", "float", "float", "float"]), _)]),
pub fn cv_utils_testRotatedRectVector_float_float_float_float_float(x: f32, y: f32, w: f32, h: f32, angle: f32, ocvrs_return: *mut Result<*mut c_void>);
// testRotatedRect(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:137
// ("cv::utils::testRotatedRect", vec![(pred!(mut, ["x", "y", "w", "h", "angle"], ["float", "float", "float", "float", "float"]), _)]),
pub fn cv_utils_testRotatedRect_float_float_float_float_float(x: f32, y: f32, w: f32, h: f32, angle: f32, ocvrs_return: *mut Result<core::RotatedRect>);
// convertFromVASurface(VADisplay, VASurfaceID, Size, OutputArray)(Indirect, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/va_intel.hpp:69
// ("cv::va_intel::convertFromVASurface", vec![(pred!(mut, ["display", "surface", "size", "dst"], ["VADisplay", "VASurfaceID", "cv::Size", "const cv::_OutputArray*"]), _)]),
pub fn cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(display: core::VADisplay, surface: core::VASurfaceID, size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result<()>);
// convertToVASurface(VADisplay, InputArray, VASurfaceID, Size)(Indirect, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/va_intel.hpp:61
// ("cv::va_intel::convertToVASurface", vec![(pred!(mut, ["display", "src", "surface", "size"], ["VADisplay", "const cv::_InputArray*", "VASurfaceID", "cv::Size"]), _)]),
pub fn cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(display: core::VADisplay, src: *const c_void, surface: core::VASurfaceID, size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::va_intel::ocl::initializeContextFromVA(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/va_intel.hpp:51
// ("cv::va_intel::ocl::initializeContextFromVA", vec![(pred!(mut, ["display"], ["VADisplay"]), _)]),
pub fn cv_va_intel_ocl_initializeContextFromVA_VADisplay(display: core::VADisplay, ocvrs_return: *mut Result<*mut c_void>);
// initializeContextFromVA(VADisplay, bool)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/va_intel.hpp:51
// ("cv::va_intel::ocl::initializeContextFromVA", vec![(pred!(mut, ["display", "tryInterop"], ["VADisplay", "bool"]), _)]),
pub fn cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(display: core::VADisplay, try_interop: bool, ocvrs_return: *mut Result<*mut c_void>);
// vconcat(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1255
// ("cv::vconcat", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// vconcat(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:1273
// ("cv::vconcat", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_vconcat_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// writeScalar(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:676
// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_writeScalar_FileStorageR_const_StringR(fs: *mut c_void, value: *const c_char, ocvrs_return: *mut Result<()>);
// writeScalar(FileStorage &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:675
// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "double"]), _)]),
pub fn cv_writeScalar_FileStorageR_double(fs: *mut c_void, value: f64, ocvrs_return: *mut Result<()>);
// writeScalar(FileStorage &, float)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:674
// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "float"]), _)]),
pub fn cv_writeScalar_FileStorageR_float(fs: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// writeScalar(FileStorage &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:672
// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "int"]), _)]),
pub fn cv_writeScalar_FileStorageR_int(fs: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// writeScalar(FileStorage &, int64_t)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:673
// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "int64_t"]), _)]),
pub fn cv_writeScalar_FileStorageR_int64_t(fs: *mut c_void, value: i64, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const Mat &)(TraitClass, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:665
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::Mat*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_MatR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const SparseMat &)(TraitClass, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:666
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::SparseMat*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_SparseMatR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const String &)(TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:664
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::String*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_StringR(fs: *mut c_void, name: *const c_char, value: *const c_char, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const std::vector<DMatch> &)(TraitClass, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:669
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const std::vector<cv::DMatch>*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_vectorLDMatchGR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, const std::vector<KeyPoint> &)(TraitClass, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:668
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_const_vectorLKeyPointGR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, double)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:663
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "double"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_double(fs: *mut c_void, name: *const c_char, value: f64, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, float)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:662
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "float"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_float(fs: *mut c_void, name: *const c_char, value: f32, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, int)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:660
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "int"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_int(fs: *mut c_void, name: *const c_char, value: i32, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &, int64_t)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:661
// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "int64_t"]), _)]),
pub fn cv_write_FileStorageR_const_StringR_int64_t(fs: *mut c_void, name: *const c_char, value: i64, ocvrs_return: *mut Result<()>);
// Algorithm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3136
// ("cv::Algorithm::Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_Algorithm(ocvrs_return: *mut Result<*mut c_void>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3141
// ("cv::Algorithm::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3145
// ("cv::Algorithm::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_Algorithm_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3150
// ("cv::Algorithm::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
pub fn cv_Algorithm_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3158
// ("cv::Algorithm::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_Algorithm_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3162
// ("cv::Algorithm::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_Algorithm_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3226
// ("cv::Algorithm::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_Algorithm_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3230
// ("cv::Algorithm::getDefaultName", vec![(pred!(const, [], []), _)]),
pub fn cv_Algorithm_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Algorithm::to_ConjGradSolver() generated
// ("cv::Algorithm::to_ConjGradSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_to_ConjGradSolver(instance: *mut c_void) -> *mut c_void;
// cv::Algorithm::to_DownhillSolver() generated
// ("cv::Algorithm::to_DownhillSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_to_DownhillSolver(instance: *mut c_void) -> *mut c_void;
// cv::Algorithm::to_MinProblemSolver() generated
// ("cv::Algorithm::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_to_MinProblemSolver(instance: *mut c_void) -> *mut c_void;
// cv::Algorithm::delete() generated
// ("cv::Algorithm::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Algorithm_delete(instance: *mut c_void);
// AsyncArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:33
// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncArray_AsyncArray() -> *mut c_void;
// AsyncArray(const AsyncArray &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:34
// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, ["o"], ["const cv::AsyncArray*"]), _)]),
pub fn cv_AsyncArray_AsyncArray_const_AsyncArrayR(o: *const c_void) -> *mut c_void;
// operator=(const AsyncArray &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:35
// ("cv::AsyncArray::operator=", vec![(pred!(mut, ["o"], ["const cv::AsyncArray*"]), _)]),
pub fn cv_AsyncArray_operatorST_const_AsyncArrayR(instance: *mut c_void, o: *const c_void);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:36
// ("cv::AsyncArray::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncArray_release(instance: *mut c_void);
// get(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:48
// ("cv::AsyncArray::get", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_AsyncArray_get_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// get(OutputArray, int64)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:58
// ("cv::AsyncArray::get", vec![(pred!(const, ["dst", "timeoutNs"], ["const cv::_OutputArray*", "int64_t"]), _)]),
pub fn cv_AsyncArray_get_const_const__OutputArrayR_int64_t(instance: *const c_void, dst: *const c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
// get(OutputArray, double)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:61
// ("cv::AsyncArray::get", vec![(pred!(const, ["dst", "timeoutNs"], ["const cv::_OutputArray*", "double"]), _)]),
pub fn cv_AsyncArray_get_const_const__OutputArrayR_double(instance: *const c_void, dst: *const c_void, timeout_ns: f64, ocvrs_return: *mut Result<bool>);
// wait_for(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:63
// ("cv::AsyncArray::wait_for", vec![(pred!(const, ["timeoutNs"], ["int64_t"]), _)]),
pub fn cv_AsyncArray_wait_for_const_int64_t(instance: *const c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
// wait_for(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:66
// ("cv::AsyncArray::wait_for", vec![(pred!(const, ["timeoutNs"], ["double"]), _)]),
pub fn cv_AsyncArray_wait_for_const_double(instance: *const c_void, timeout_ns: f64, ocvrs_return: *mut Result<bool>);
// valid()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:68
// ("cv::AsyncArray::valid", vec![(pred!(const, [], []), _)]),
pub fn cv_AsyncArray_valid_const(instance: *const c_void) -> bool;
// AsyncArray(AsyncArray &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:70
// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, ["o"], ["cv::AsyncArray*"]), _)]),
pub fn cv_AsyncArray_AsyncArray_AsyncArrayRR(o: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(AsyncArray &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/async.hpp:71
// ("cv::AsyncArray::operator=", vec![(pred!(mut, ["o"], ["cv::AsyncArray*"]), _)]),
pub fn cv_AsyncArray_operatorST_AsyncArrayRR(instance: *mut c_void, o: *mut c_void);
// cv::AsyncArray::delete() generated
// ("cv::AsyncArray::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncArray_delete(instance: *mut c_void);
// AsyncPromise()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:26
// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncPromise_AsyncPromise() -> *mut c_void;
// AsyncPromise(const AsyncPromise &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:27
// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, ["o"], ["const cv::AsyncPromise*"]), _)]),
pub fn cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(o: *const c_void) -> *mut c_void;
// operator=(const AsyncPromise &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:28
// ("cv::AsyncPromise::operator=", vec![(pred!(mut, ["o"], ["const cv::AsyncPromise*"]), _)]),
pub fn cv_AsyncPromise_operatorST_const_AsyncPromiseR(instance: *mut c_void, o: *const c_void);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:29
// ("cv::AsyncPromise::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncPromise_release(instance: *mut c_void);
// getArrayResult()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:34
// ("cv::AsyncPromise::getArrayResult", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncPromise_getArrayResult(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setValue(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:39
// ("cv::AsyncPromise::setValue", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
pub fn cv_AsyncPromise_setValue_const__InputArrayR(instance: *mut c_void, value: *const c_void, ocvrs_return: *mut Result<()>);
// setException(const cv::Exception &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:53
// ("cv::AsyncPromise::setException", vec![(pred!(mut, ["exception"], ["const cv::Exception*"]), _)]),
pub fn cv_AsyncPromise_setException_const_ExceptionR(instance: *mut c_void, exception: *const c_void, ocvrs_return: *mut Result<()>);
// AsyncPromise(AsyncPromise &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:55
// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, ["o"], ["cv::AsyncPromise*"]), _)]),
pub fn cv_AsyncPromise_AsyncPromise_AsyncPromiseRR(o: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(AsyncPromise &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:56
// ("cv::AsyncPromise::operator=", vec![(pred!(mut, ["o"], ["cv::AsyncPromise*"]), _)]),
pub fn cv_AsyncPromise_operatorST_AsyncPromiseRR(instance: *mut c_void, o: *mut c_void);
// _getImpl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/detail/async_promise.hpp:61
// ("cv::AsyncPromise::_getImpl", vec![(pred!(const, [], []), _)]),
pub fn cv_AsyncPromise__getImpl_const(instance: *const c_void) -> *mut c_void;
// cv::AsyncPromise::delete() generated
// ("cv::AsyncPromise::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AsyncPromise_delete(instance: *mut c_void);
// CommandLineParser(int, const char *const *, const String &)(Primitive, VariableArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:901
// ("cv::CommandLineParser::CommandLineParser", vec![(pred!(mut, ["argc", "argv", "keys"], ["int", "const char**", "const cv::String*"]), _)]),
pub fn cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(argc: i32, argv: *const *const c_char, keys: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// CommandLineParser(const CommandLineParser &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:904
// ("cv::CommandLineParser::CommandLineParser", vec![(pred!(mut, ["parser"], ["const cv::CommandLineParser*"]), _)]),
pub fn cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(parser: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const CommandLineParser &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:907
// ("cv::CommandLineParser::operator=", vec![(pred!(mut, ["parser"], ["const cv::CommandLineParser*"]), _)]),
pub fn cv_CommandLineParser_operatorST_const_CommandLineParserR(instance: *mut c_void, parser: *const c_void, ocvrs_return: *mut Result<()>);
// getPathToApplication()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:922
// ("cv::CommandLineParser::getPathToApplication", vec![(pred!(const, [], []), _)]),
pub fn cv_CommandLineParser_getPathToApplication_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
pub fn cv_CommandLineParser_get_bool_const_const_StringR_bool(instance: *const c_void, name: *const c_char, space_delete: bool, ocvrs_return: *mut Result<bool>);
// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_get_bool_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
pub fn cv_CommandLineParser_get_int_const_const_StringR_bool(instance: *const c_void, name: *const c_char, space_delete: bool, ocvrs_return: *mut Result<i32>);
// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_get_int_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<i32>);
// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
pub fn cv_CommandLineParser_get_double_const_const_StringR_bool(instance: *const c_void, name: *const c_char, space_delete: bool, ocvrs_return: *mut Result<f64>);
// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_get_double_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<f64>);
// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
pub fn cv_CommandLineParser_get_cv_String_const_const_StringR_bool(instance: *const c_void, name: *const c_char, space_delete: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_get_cv_String_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
pub fn cv_CommandLineParser_get_uint64_t_const_const_StringR_bool(instance: *const c_void, name: *const c_char, space_delete: bool, ocvrs_return: *mut Result<u64>);
// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:956
// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_get_uint64_t_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<u64>);
// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
pub fn cv_CommandLineParser_get_bool_const_int_bool(instance: *const c_void, index: i32, space_delete: bool, ocvrs_return: *mut Result<bool>);
// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
pub fn cv_CommandLineParser_get_bool_const_int(instance: *const c_void, index: i32, ocvrs_return: *mut Result<bool>);
// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
pub fn cv_CommandLineParser_get_int_const_int_bool(instance: *const c_void, index: i32, space_delete: bool, ocvrs_return: *mut Result<i32>);
// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
pub fn cv_CommandLineParser_get_int_const_int(instance: *const c_void, index: i32, ocvrs_return: *mut Result<i32>);
// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
pub fn cv_CommandLineParser_get_double_const_int_bool(instance: *const c_void, index: i32, space_delete: bool, ocvrs_return: *mut Result<f64>);
// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
pub fn cv_CommandLineParser_get_double_const_int(instance: *const c_void, index: i32, ocvrs_return: *mut Result<f64>);
// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
pub fn cv_CommandLineParser_get_cv_String_const_int_bool(instance: *const c_void, index: i32, space_delete: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
pub fn cv_CommandLineParser_get_cv_String_const_int(instance: *const c_void, index: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
pub fn cv_CommandLineParser_get_uint64_t_const_int_bool(instance: *const c_void, index: i32, space_delete: bool, ocvrs_return: *mut Result<u64>);
// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:988
// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
pub fn cv_CommandLineParser_get_uint64_t_const_int(instance: *const c_void, index: i32, ocvrs_return: *mut Result<u64>);
// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:999
// ("cv::CommandLineParser::has", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_has_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
// check()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1006
// ("cv::CommandLineParser::check", vec![(pred!(const, [], []), _)]),
pub fn cv_CommandLineParser_check_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// about(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1012
// ("cv::CommandLineParser::about", vec![(pred!(mut, ["message"], ["const cv::String*"]), _)]),
pub fn cv_CommandLineParser_about_const_StringR(instance: *mut c_void, message: *const c_char, ocvrs_return: *mut Result<()>);
// printMessage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1020
// ("cv::CommandLineParser::printMessage", vec![(pred!(const, [], []), _)]),
pub fn cv_CommandLineParser_printMessage_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// printErrors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:1026
// ("cv::CommandLineParser::printErrors", vec![(pred!(const, [], []), _)]),
pub fn cv_CommandLineParser_printErrors_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// cv::CommandLineParser::delete() generated
// ("cv::CommandLineParser::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CommandLineParser_delete(instance: *mut c_void);
// create(const Ptr<MinProblemSolver::Function> &, TermCriteria)(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:252
// ("cv::ConjGradSolver::create", vec![(pred!(mut, ["f", "termcrit"], ["const cv::Ptr<cv::MinProblemSolver::Function>*", "cv::TermCriteria"]), _)]),
pub fn cv_ConjGradSolver_create_const_PtrLFunctionGR_TermCriteria(f: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
// cv::ConjGradSolver::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:252
// ("cv::ConjGradSolver::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ConjGradSolver_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::ConjGradSolver::to_Algorithm() generated
// ("cv::ConjGradSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ConjGradSolver_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ConjGradSolver::to_MinProblemSolver() generated
// ("cv::ConjGradSolver::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_ConjGradSolver_to_MinProblemSolver(instance: *mut c_void) -> *mut c_void;
// cv::ConjGradSolver::delete() generated
// ("cv::ConjGradSolver::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ConjGradSolver_delete(instance: *mut c_void);
// DMatch()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:854
// ("cv::DMatch::DMatch", vec![(pred!(mut, [], []), _)]),
pub fn cv_DMatch_DMatch(ocvrs_return: *mut Result<core::DMatch>);
// DMatch(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:855
// ("cv::DMatch::DMatch", vec![(pred!(mut, ["_queryIdx", "_trainIdx", "_distance"], ["int", "int", "float"]), _)]),
pub fn cv_DMatch_DMatch_int_int_float(_query_idx: i32, _train_idx: i32, _distance: f32, ocvrs_return: *mut Result<core::DMatch>);
// DMatch(int, int, int, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:856
// ("cv::DMatch::DMatch", vec![(pred!(mut, ["_queryIdx", "_trainIdx", "_imgIdx", "_distance"], ["int", "int", "int", "float"]), _)]),
pub fn cv_DMatch_DMatch_int_int_int_float(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32, ocvrs_return: *mut Result<core::DMatch>);
// operator<(const DMatch &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:865
// ("cv::DMatch::operator<", vec![(pred!(const, ["m"], ["const cv::DMatch*"]), _)]),
pub fn cv_DMatch_operatorL_const_const_DMatchR(instance: *const core::DMatch, m: *const core::DMatch, ocvrs_return: *mut Result<bool>);
// getInitStep(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:164
// ("cv::DownhillSolver::getInitStep", vec![(pred!(const, ["step"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_DownhillSolver_getInitStep_const_const__OutputArrayR(instance: *const c_void, step: *const c_void, ocvrs_return: *mut Result<()>);
// setInitStep(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:180
// ("cv::DownhillSolver::setInitStep", vec![(pred!(mut, ["step"], ["const cv::_InputArray*"]), _)]),
pub fn cv_DownhillSolver_setInitStep_const__InputArrayR(instance: *mut c_void, step: *const c_void, ocvrs_return: *mut Result<()>);
// create(const Ptr<MinProblemSolver::Function> &, InputArray, TermCriteria)(CppPassByVoidPtr, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:198
// ("cv::DownhillSolver::create", vec![(pred!(mut, ["f", "initStep", "termcrit"], ["const cv::Ptr<cv::MinProblemSolver::Function>*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
pub fn cv_DownhillSolver_create_const_PtrLFunctionGR_const__InputArrayR_TermCriteria(f: *const c_void, init_step: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
// cv::DownhillSolver::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:198
// ("cv::DownhillSolver::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_DownhillSolver_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::DownhillSolver::to_Algorithm() generated
// ("cv::DownhillSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_DownhillSolver_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::DownhillSolver::to_MinProblemSolver() generated
// ("cv::DownhillSolver::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_DownhillSolver_to_MinProblemSolver(instance: *mut c_void) -> *mut c_void;
// cv::DownhillSolver::delete() generated
// ("cv::DownhillSolver::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_DownhillSolver_delete(instance: *mut c_void);
// Exception()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:91
// ("cv::Exception::Exception", vec![(pred!(mut, [], []), _)]),
pub fn cv_Exception_Exception(ocvrs_return: *mut Result<*mut c_void>);
// Exception(Error::Code, const std::string &, const std::string &, const std::string &, int)(Enum, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:96
// ("cv::Exception::Exception", vec![(pred!(mut, ["_code", "_err", "_func", "_file", "_line"], ["cv::Error::Code", "const std::string*", "const std::string*", "const std::string*", "int"]), _)]),
pub fn cv_Exception_Exception_Code_const_stringR_const_stringR_const_stringR_int(_code: core::Code, _err: *const c_char, _func: *const c_char, _file: *const c_char, _line: i32, ocvrs_return: *mut Result<*mut c_void>);
// what()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:102
// ("cv::Exception::what", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_what_const(instance: *const c_void) -> *mut c_void;
// formatMessage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:103
// ("cv::Exception::formatMessage", vec![(pred!(mut, [], []), _)]),
pub fn cv_Exception_formatMessage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// codeMessage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:104
// ("cv::Exception::codeMessage", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_codeMessage_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Exception::msg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:106
// ("cv::Exception::msg", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propMsg_const(instance: *const c_void) -> *mut c_void;
// cv::Exception::setMsg(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:106
// ("cv::Exception::setMsg", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_Exception_propMsg_const_string(instance: *mut c_void, val: *const c_char);
// cv::Exception::code() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:108
// ("cv::Exception::code", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propCode_const(instance: *const c_void, ocvrs_return: *mut core::Code);
// cv::Exception::setCode(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:108
// ("cv::Exception::setCode", vec![(pred!(mut, ["val"], ["const cv::Error::Code"]), _)]),
pub fn cv_Exception_propCode_const_Code(instance: *mut c_void, val: core::Code);
// cv::Exception::err() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:109
// ("cv::Exception::err", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propErr_const(instance: *const c_void) -> *mut c_void;
// cv::Exception::setErr(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:109
// ("cv::Exception::setErr", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_Exception_propErr_const_string(instance: *mut c_void, val: *const c_char);
// cv::Exception::func() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:110
// ("cv::Exception::func", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propFunc_const(instance: *const c_void) -> *mut c_void;
// cv::Exception::setFunc(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:110
// ("cv::Exception::setFunc", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_Exception_propFunc_const_string(instance: *mut c_void, val: *const c_char);
// cv::Exception::file() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:111
// ("cv::Exception::file", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propFile_const(instance: *const c_void) -> *mut c_void;
// cv::Exception::setFile(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:111
// ("cv::Exception::setFile", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_Exception_propFile_const_string(instance: *mut c_void, val: *const c_char);
// cv::Exception::line() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:112
// ("cv::Exception::line", vec![(pred!(const, [], []), _)]),
pub fn cv_Exception_propLine_const(instance: *const c_void) -> i32;
// cv::Exception::setLine(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/exception.hpp:112
// ("cv::Exception::setLine", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Exception_propLine_const_int(instance: *mut c_void, val: i32);
// cv::Exception::delete() generated
// ("cv::Exception::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Exception_delete(instance: *mut c_void);
// FileNode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:467
// ("cv::FileNode::FileNode", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNode_FileNode(ocvrs_return: *mut Result<*mut c_void>);
// FileNode(const FileStorage *, size_t, size_t)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:476
// ("cv::FileNode::FileNode", vec![(pred!(mut, ["fs", "blockIdx", "ofs"], ["const cv::FileStorage*", "size_t", "size_t"]), _)]),
pub fn cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(fs: *const c_void, block_idx: size_t, ofs: size_t, ocvrs_return: *mut Result<*mut c_void>);
// FileNode(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:481
// ("cv::FileNode::FileNode", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_FileNode_FileNode_const_FileNodeR(node: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:483
// ("cv::FileNode::operator=", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_FileNode_operatorST_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<()>);
// operator[](const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:489
// ("cv::FileNode::operator[]", vec![(pred!(const, ["nodename"], ["const cv::String*"]), _)]),
pub fn cv_FileNode_operator___const_const_StringR(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// operator[](const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:494
// ("cv::FileNode::operator[]", vec![(pred!(const, ["nodename"], ["const char*"]), _)]),
pub fn cv_FileNode_operator___const_const_charX(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:499
// ("cv::FileNode::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_FileNode_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
// keys()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:504
// ("cv::FileNode::keys", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_keys_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:509
// ("cv::FileNode::type", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:512
// ("cv::FileNode::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isNone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:514
// ("cv::FileNode::isNone", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isNone_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isSeq()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:516
// ("cv::FileNode::isSeq", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isSeq_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isMap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:518
// ("cv::FileNode::isMap", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isMap_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isInt()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:520
// ("cv::FileNode::isInt", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isInt_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isReal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:522
// ("cv::FileNode::isReal", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isReal_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isString()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:524
// ("cv::FileNode::isString", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isString_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isNamed()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:526
// ("cv::FileNode::isNamed", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_isNamed_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:528
// ("cv::FileNode::name", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:530
// ("cv::FileNode::size", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// rawSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:532
// ("cv::FileNode::rawSize", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_rawSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// operator int()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:534
// ("cv::FileNode::operator int", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_operator_int_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// operator long()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:536
// ("cv::FileNode::operator int64_t", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_operator_int64_t_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
// operator float()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:538
// ("cv::FileNode::operator float", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_operator_float_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// operator double()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:540
// ("cv::FileNode::operator double", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_operator_double_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// operator basic_string()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:542
// ("cv::FileNode::operator std::string", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_operator_std_string_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// isMap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:544
// ("cv::FileNode::isMap", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FileNode_isMap_int(flags: i32, ocvrs_return: *mut Result<bool>);
// isSeq(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:545
// ("cv::FileNode::isSeq", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FileNode_isSeq_int(flags: i32, ocvrs_return: *mut Result<bool>);
// isCollection(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:546
// ("cv::FileNode::isCollection", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FileNode_isCollection_int(flags: i32, ocvrs_return: *mut Result<bool>);
// isEmptyCollection(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:547
// ("cv::FileNode::isEmptyCollection", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FileNode_isEmptyCollection_int(flags: i32, ocvrs_return: *mut Result<bool>);
// isFlow(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:548
// ("cv::FileNode::isFlow", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_FileNode_isFlow_int(flags: i32, ocvrs_return: *mut Result<bool>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:550
// ("cv::FileNode::ptr", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNode_ptr(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:551
// ("cv::FileNode::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:554
// ("cv::FileNode::begin", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_begin_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:556
// ("cv::FileNode::end", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_end_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readRaw(const String &, void *, size_t)(InString, VoidSlice, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:566
// ("cv::FileNode::readRaw", vec![(pred!(const, ["fmt", "vec", "len"], ["const cv::String*", "void*", "size_t"]), _)]),
pub fn cv_FileNode_readRaw_const_const_StringR_voidX_size_t(instance: *const c_void, fmt: *const c_char, vec: *mut c_void, len: size_t, ocvrs_return: *mut Result<()>);
// setValue(int, const void *, int)(Primitive, VoidSlice, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:571
// ("cv::FileNode::setValue", vec![(pred!(mut, ["type", "value", "len"], ["int", "const void*", "int"]), _)]),
pub fn cv_FileNode_setValue_int_const_voidX_int(instance: *mut c_void, typ: i32, value: *const c_void, len: i32, ocvrs_return: *mut Result<()>);
// real()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:574
// ("cv::FileNode::real", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_real_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// string()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:576
// ("cv::FileNode::string", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_string_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:578
// ("cv::FileNode::mat", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_mat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::FileNode::implicitClone() generated
// ("cv::FileNode::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::FileNode::blockIdx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:584
// ("cv::FileNode::blockIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_propBlockIdx_const(instance: *const c_void) -> size_t;
// cv::FileNode::setBlockIdx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:584
// ("cv::FileNode::setBlockIdx", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_FileNode_propBlockIdx_const_size_t(instance: *mut c_void, val: size_t);
// cv::FileNode::ofs() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:585
// ("cv::FileNode::ofs", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNode_propOfs_const(instance: *const c_void) -> size_t;
// cv::FileNode::setOfs(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:585
// ("cv::FileNode::setOfs", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_FileNode_propOfs_const_size_t(instance: *mut c_void, val: size_t);
// cv::FileNode::delete() generated
// ("cv::FileNode::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNode_delete(instance: *mut c_void);
// FileNodeIterator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:602
// ("cv::FileNodeIterator::FileNodeIterator", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNodeIterator_FileNodeIterator(ocvrs_return: *mut Result<*mut c_void>);
// FileNodeIterator(const FileNode &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:612
// ("cv::FileNodeIterator::FileNodeIterator", vec![(pred!(mut, ["node", "seekEnd"], ["const cv::FileNode*", "bool"]), _)]),
pub fn cv_FileNodeIterator_FileNodeIterator_const_FileNodeR_bool(node: *const c_void, seek_end: bool, ocvrs_return: *mut Result<*mut c_void>);
// FileNodeIterator(const FileNodeIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:617
// ("cv::FileNodeIterator::FileNodeIterator", vec![(pred!(mut, ["it"], ["const cv::FileNodeIterator*"]), _)]),
pub fn cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const FileNodeIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:619
// ("cv::FileNodeIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::FileNodeIterator*"]), _)]),
pub fn cv_FileNodeIterator_operatorST_const_FileNodeIteratorR(instance: *mut c_void, it: *const c_void, ocvrs_return: *mut Result<()>);
// operator*()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:622
// ("cv::FileNodeIterator::operator*", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNodeIterator_operatorX_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:625
// ("cv::FileNodeIterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNodeIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// readRaw(const String &, void *, size_t)(InString, VoidSlice, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:639
// ("cv::FileNodeIterator::readRaw", vec![(pred!(mut, ["fmt", "vec", "len"], ["const cv::String*", "void*", "size_t"]), _)]),
pub fn cv_FileNodeIterator_readRaw_const_StringR_voidX_size_t(instance: *mut c_void, fmt: *const c_char, vec: *mut c_void, len: size_t, ocvrs_return: *mut Result<*mut c_void>);
// remaining()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:643
// ("cv::FileNodeIterator::remaining", vec![(pred!(const, [], []), _)]),
pub fn cv_FileNodeIterator_remaining_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// equalTo(const FileNodeIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:645
// ("cv::FileNodeIterator::equalTo", vec![(pred!(const, ["it"], ["const cv::FileNodeIterator*"]), _)]),
pub fn cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorR(instance: *const c_void, it: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::FileNodeIterator::delete() generated
// ("cv::FileNodeIterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileNodeIterator_delete(instance: *mut c_void);
// FileStorage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:293
// ("cv::FileStorage::FileStorage", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileStorage_FileStorage(ocvrs_return: *mut Result<*mut c_void>);
// FileStorage(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:298
// ("cv::FileStorage::FileStorage", vec![(pred!(mut, ["filename", "flags", "encoding"], ["const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_FileStorage_FileStorage_const_StringR_int_const_StringR(filename: *const c_char, flags: i32, encoding: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::FileStorage::FileStorage(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:298
// ("cv::FileStorage::FileStorage", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_FileStorage_FileStorage_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// open(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:318
// ("cv::FileStorage::open", vec![(pred!(mut, ["filename", "flags", "encoding"], ["const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_FileStorage_open_const_StringR_int_const_StringR(instance: *mut c_void, filename: *const c_char, flags: i32, encoding: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::FileStorage::open(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:318
// ("cv::FileStorage::open", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_FileStorage_open_const_StringR_int(instance: *mut c_void, filename: *const c_char, flags: i32, ocvrs_return: *mut Result<bool>);
// isOpened()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:325
// ("cv::FileStorage::isOpened", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:331
// ("cv::FileStorage::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileStorage_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// releaseAndGetString()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:338
// ("cv::FileStorage::releaseAndGetString", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileStorage_releaseAndGetString(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getFirstTopLevelNode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:343
// ("cv::FileStorage::getFirstTopLevelNode", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_getFirstTopLevelNode_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// root(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:350
// ("cv::FileStorage::root", vec![(pred!(const, ["streamidx"], ["int"]), _)]),
pub fn cv_FileStorage_root_const_int(instance: *const c_void, streamidx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::FileStorage::root() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:350
// ("cv::FileStorage::root", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_root_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator[](const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:356
// ("cv::FileStorage::operator[]", vec![(pred!(const, ["nodename"], ["const cv::String*"]), _)]),
pub fn cv_FileStorage_operator___const_const_StringR(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// operator[](const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:359
// ("cv::FileStorage::operator[]", vec![(pred!(const, ["nodename"], ["const char*"]), _)]),
pub fn cv_FileStorage_operator___const_const_charX(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// write(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:366
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "int"]), _)]),
pub fn cv_FileStorage_write_const_StringR_int(instance: *mut c_void, name: *const c_char, val: i32, ocvrs_return: *mut Result<()>);
// write(const String &, int64_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:368
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "int64_t"]), _)]),
pub fn cv_FileStorage_write_const_StringR_int64_t(instance: *mut c_void, name: *const c_char, val: i64, ocvrs_return: *mut Result<()>);
// write(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:370
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "double"]), _)]),
pub fn cv_FileStorage_write_const_StringR_double(instance: *mut c_void, name: *const c_char, val: f64, ocvrs_return: *mut Result<()>);
// write(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:372
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_FileStorage_write_const_StringR_const_StringR(instance: *mut c_void, name: *const c_char, val: *const c_char, ocvrs_return: *mut Result<()>);
// write(const String &, const Mat &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:374
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "const cv::Mat*"]), _)]),
pub fn cv_FileStorage_write_const_StringR_const_MatR(instance: *mut c_void, name: *const c_char, val: *const c_void, ocvrs_return: *mut Result<()>);
// write(const String &, const std::vector<String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:376
// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "const std::vector<cv::String>*"]), _)]),
pub fn cv_FileStorage_write_const_StringR_const_vectorLStringGR(instance: *mut c_void, name: *const c_char, val: *const c_void, ocvrs_return: *mut Result<()>);
// writeRaw(const String &, const void *, size_t)(InString, VoidSlice, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:386
// ("cv::FileStorage::writeRaw", vec![(pred!(mut, ["fmt", "vec", "len"], ["const cv::String*", "const void*", "size_t"]), _)]),
pub fn cv_FileStorage_writeRaw_const_StringR_const_voidX_size_t(instance: *mut c_void, fmt: *const c_char, vec: *const c_void, len: size_t, ocvrs_return: *mut Result<()>);
// writeComment(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:396
// ("cv::FileStorage::writeComment", vec![(pred!(mut, ["comment", "append"], ["const cv::String*", "bool"]), _)]),
pub fn cv_FileStorage_writeComment_const_StringR_bool(instance: *mut c_void, comment: *const c_char, append: bool, ocvrs_return: *mut Result<()>);
// cv::FileStorage::writeComment(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:396
// ("cv::FileStorage::writeComment", vec![(pred!(mut, ["comment"], ["const cv::String*"]), _)]),
pub fn cv_FileStorage_writeComment_const_StringR(instance: *mut c_void, comment: *const c_char, ocvrs_return: *mut Result<()>);
// startWriteStruct(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:404
// ("cv::FileStorage::startWriteStruct", vec![(pred!(mut, ["name", "flags", "typeName"], ["const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(instance: *mut c_void, name: *const c_char, flags: i32, type_name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::FileStorage::startWriteStruct(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:404
// ("cv::FileStorage::startWriteStruct", vec![(pred!(mut, ["name", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_FileStorage_startWriteStruct_const_StringR_int(instance: *mut c_void, name: *const c_char, flags: i32, ocvrs_return: *mut Result<()>);
// endWriteStruct()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:408
// ("cv::FileStorage::endWriteStruct", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileStorage_endWriteStruct(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getDefaultObjectName(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:414
// ("cv::FileStorage::getDefaultObjectName", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_FileStorage_getDefaultObjectName_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getFormat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:419
// ("cv::FileStorage::getFormat", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_getFormat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::FileStorage::state() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:421
// ("cv::FileStorage::state", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_propState_const(instance: *const c_void) -> i32;
// cv::FileStorage::setState(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:421
// ("cv::FileStorage::setState", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_FileStorage_propState_const_int(instance: *mut c_void, val: i32);
// cv::FileStorage::elname() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:422
// ("cv::FileStorage::elname", vec![(pred!(const, [], []), _)]),
pub fn cv_FileStorage_propElname_const(instance: *const c_void) -> *mut c_void;
// cv::FileStorage::setElname(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:422
// ("cv::FileStorage::setElname", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_FileStorage_propElname_const_string(instance: *mut c_void, val: *const c_char);
// cv::FileStorage::delete() generated
// ("cv::FileStorage::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_FileStorage_delete(instance: *mut c_void);
// next()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3064
// ("cv::Formatted::next", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatted_next(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3065
// ("cv::Formatted::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatted_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::Formatted::delete() generated
// ("cv::Formatted::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatted_delete(instance: *mut c_void);
// format(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3084
// ("cv::Formatter::format", vec![(pred!(const, ["mtx"], ["const cv::Mat*"]), _)]),
pub fn cv_Formatter_format_const_const_MatR(instance: *const c_void, mtx: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// set16fPrecision(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3086
// ("cv::Formatter::set16fPrecision", vec![(pred!(mut, ["p"], ["int"]), _)]),
pub fn cv_Formatter_set16fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result<()>);
// cv::Formatter::set16fPrecision() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3086
// ("cv::Formatter::set16fPrecision", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_set16fPrecision(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// set32fPrecision(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3087
// ("cv::Formatter::set32fPrecision", vec![(pred!(mut, ["p"], ["int"]), _)]),
pub fn cv_Formatter_set32fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result<()>);
// cv::Formatter::set32fPrecision() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3087
// ("cv::Formatter::set32fPrecision", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_set32fPrecision(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// set64fPrecision(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3088
// ("cv::Formatter::set64fPrecision", vec![(pred!(mut, ["p"], ["int"]), _)]),
pub fn cv_Formatter_set64fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result<()>);
// cv::Formatter::set64fPrecision() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3088
// ("cv::Formatter::set64fPrecision", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_set64fPrecision(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setMultiline(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3089
// ("cv::Formatter::setMultiline", vec![(pred!(mut, ["ml"], ["bool"]), _)]),
pub fn cv_Formatter_setMultiline_bool(instance: *mut c_void, ml: bool, ocvrs_return: *mut Result<()>);
// cv::Formatter::setMultiline() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3089
// ("cv::Formatter::setMultiline", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_setMultiline(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// get(Formatter::FormatType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3091
// ("cv::Formatter::get", vec![(pred!(mut, ["fmt"], ["cv::Formatter::FormatType"]), _)]),
pub fn cv_Formatter_get_FormatType(fmt: core::Formatter_FormatType, ocvrs_return: *mut Result<*mut c_void>);
// cv::Formatter::get() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:3091
// ("cv::Formatter::get", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_get(ocvrs_return: *mut Result<*mut c_void>);
// cv::Formatter::delete() generated
// ("cv::Formatter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Formatter_delete(instance: *mut c_void);
// operator()(const unsigned char *, const unsigned char *, int)(VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/base.hpp:233
// ("cv::Hamming::operator()", vec![(pred!(const, ["a", "b", "size"], ["const unsigned char*", "const unsigned char*", "int"]), _)]),
pub fn cv_Hamming_operator___const_const_unsigned_charX_const_unsigned_charX_int(instance: *const c_void, a: *const u8, b: *const u8, size: i32, ocvrs_return: *mut Result<core::Hamming_ResultType>);
// cv::Hamming::defaultNew() generated
// ("cv::Hamming::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_Hamming_defaultNew_const() -> *mut c_void;
// cv::Hamming::delete() generated
// ("cv::Hamming::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Hamming_delete(instance: *mut c_void);
// KeyPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:758
// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, [], []), _)]),
pub fn cv_KeyPoint_KeyPoint(ocvrs_return: *mut Result<*mut c_void>);
// KeyPoint(Point2f, float, float, float, int, int)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:767
// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["pt", "size", "angle", "response", "octave", "class_id"], ["cv::Point2f", "float", "float", "float", "int", "int"]), _)]),
pub fn cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(pt: *const core::Point2f, size: f32, angle: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::KeyPoint::KeyPoint(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:767
// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["pt", "size"], ["cv::Point2f", "float"]), _)]),
pub fn cv_KeyPoint_KeyPoint_Point2f_float(pt: *const core::Point2f, size: f32, ocvrs_return: *mut Result<*mut c_void>);
// KeyPoint(float, float, float, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:777
// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["x", "y", "size", "angle", "response", "octave", "class_id"], ["float", "float", "float", "float", "float", "int", "int"]), _)]),
pub fn cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(x: f32, y: f32, size: f32, angle: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::KeyPoint::KeyPoint(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:777
// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["x", "y", "size"], ["float", "float", "float"]), _)]),
pub fn cv_KeyPoint_KeyPoint_float_float_float(x: f32, y: f32, size: f32, ocvrs_return: *mut Result<*mut c_void>);
// hash()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:779
// ("cv::KeyPoint::hash", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_hash_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// convert(const std::vector<KeyPoint> &, std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:790
// ("cv::KeyPoint::convert", vec![(pred!(mut, ["keypoints", "points2f", "keypointIndexes"], ["const std::vector<cv::KeyPoint>*", "std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
pub fn cv_KeyPoint_convert_const_vectorLKeyPointGR_vectorLPoint2fGR_const_vectorLintGR(keypoints: *const c_void, points2f: *mut c_void, keypoint_indexes: *const c_void, ocvrs_return: *mut Result<()>);
// cv::KeyPoint::convert(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:790
// ("cv::KeyPoint::convert", vec![(pred!(mut, ["keypoints", "points2f"], ["const std::vector<cv::KeyPoint>*", "std::vector<cv::Point2f>*"]), _)]),
pub fn cv_KeyPoint_convert_const_vectorLKeyPointGR_vectorLPoint2fGR(keypoints: *const c_void, points2f: *mut c_void, ocvrs_return: *mut Result<()>);
// convert(const std::vector<Point2f> &, std::vector<KeyPoint> &, float, float, int, int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:801
// ("cv::KeyPoint::convert", vec![(pred!(mut, ["points2f", "keypoints", "size", "response", "octave", "class_id"], ["const std::vector<cv::Point2f>*", "std::vector<cv::KeyPoint>*", "float", "float", "int", "int"]), _)]),
pub fn cv_KeyPoint_convert_const_vectorLPoint2fGR_vectorLKeyPointGR_float_float_int_int(points2f: *const c_void, keypoints: *mut c_void, size: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result<()>);
// cv::KeyPoint::convert(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:801
// ("cv::KeyPoint::convert", vec![(pred!(mut, ["points2f", "keypoints"], ["const std::vector<cv::Point2f>*", "std::vector<cv::KeyPoint>*"]), _)]),
pub fn cv_KeyPoint_convert_const_vectorLPoint2fGR_vectorLKeyPointGR(points2f: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// overlap(const KeyPoint &, const KeyPoint &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:812
// ("cv::KeyPoint::overlap", vec![(pred!(mut, ["kp1", "kp2"], ["const cv::KeyPoint*", "const cv::KeyPoint*"]), _)]),
pub fn cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(kp1: *const c_void, kp2: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::KeyPoint::implicitClone() generated
// ("cv::KeyPoint::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::KeyPoint::pt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:814
// ("cv::KeyPoint::pt", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propPt_const(instance: *const c_void, ocvrs_return: *mut core::Point2f);
// cv::KeyPoint::setPt(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:814
// ("cv::KeyPoint::setPt", vec![(pred!(mut, ["val"], ["const cv::Point2f"]), _)]),
pub fn cv_KeyPoint_propPt_const_Point2f(instance: *mut c_void, val: *const core::Point2f);
// cv::KeyPoint::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:815
// ("cv::KeyPoint::size", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propSize_const(instance: *const c_void) -> f32;
// cv::KeyPoint::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:815
// ("cv::KeyPoint::setSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_KeyPoint_propSize_const_float(instance: *mut c_void, val: f32);
// cv::KeyPoint::angle() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:816
// ("cv::KeyPoint::angle", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propAngle_const(instance: *const c_void) -> f32;
// cv::KeyPoint::setAngle(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:816
// ("cv::KeyPoint::setAngle", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_KeyPoint_propAngle_const_float(instance: *mut c_void, val: f32);
// cv::KeyPoint::response() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:819
// ("cv::KeyPoint::response", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propResponse_const(instance: *const c_void) -> f32;
// cv::KeyPoint::setResponse(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:819
// ("cv::KeyPoint::setResponse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_KeyPoint_propResponse_const_float(instance: *mut c_void, val: f32);
// cv::KeyPoint::octave() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:820
// ("cv::KeyPoint::octave", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propOctave_const(instance: *const c_void) -> i32;
// cv::KeyPoint::setOctave(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:820
// ("cv::KeyPoint::setOctave", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_KeyPoint_propOctave_const_int(instance: *mut c_void, val: i32);
// cv::KeyPoint::class_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:821
// ("cv::KeyPoint::class_id", vec![(pred!(const, [], []), _)]),
pub fn cv_KeyPoint_propClass_id_const(instance: *const c_void) -> i32;
// cv::KeyPoint::setClass_id(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:821
// ("cv::KeyPoint::setClass_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_KeyPoint_propClass_id_const_int(instance: *mut c_void, val: i32);
// cv::KeyPoint::delete() generated
// ("cv::KeyPoint::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_KeyPoint_delete(instance: *mut c_void);
// LDA(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2590
// ("cv::LDA::LDA", vec![(pred!(mut, ["num_components"], ["int"]), _)]),
pub fn cv_LDA_LDA_int(num_components: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::LDA::LDA() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2590
// ("cv::LDA::LDA", vec![(pred!(mut, [], []), _)]),
pub fn cv_LDA_LDA(ocvrs_return: *mut Result<*mut c_void>);
// LDA(InputArrayOfArrays, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2597
// ("cv::LDA::LDA", vec![(pred!(mut, ["src", "labels", "num_components"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(src: *const c_void, labels: *const c_void, num_components: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::LDA::LDA(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2597
// ("cv::LDA::LDA", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LDA_LDA_const__InputArrayR_const__InputArrayR(src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2601
// ("cv::LDA::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_LDA_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2605
// ("cv::LDA::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_LDA_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// save(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2609
// ("cv::LDA::save", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_LDA_save_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// load(const FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2613
// ("cv::LDA::load", vec![(pred!(mut, ["node"], ["const cv::FileStorage*"]), _)]),
pub fn cv_LDA_load_const_FileStorageR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2621
// ("cv::LDA::compute", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LDA_compute_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<()>);
// project(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2626
// ("cv::LDA::project", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_LDA_project_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// reconstruct(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2631
// ("cv::LDA::reconstruct", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_LDA_reconstruct_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// eigenvectors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2635
// ("cv::LDA::eigenvectors", vec![(pred!(const, [], []), _)]),
pub fn cv_LDA_eigenvectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// eigenvalues()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2639
// ("cv::LDA::eigenvalues", vec![(pred!(const, [], []), _)]),
pub fn cv_LDA_eigenvalues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// subspaceProject(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2641
// ("cv::LDA::subspaceProject", vec![(pred!(mut, ["W", "mean", "src"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(w: *const c_void, mean: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// subspaceReconstruct(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2642
// ("cv::LDA::subspaceReconstruct", vec![(pred!(mut, ["W", "mean", "src"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(w: *const c_void, mean: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::LDA::delete() generated
// ("cv::LDA::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LDA_delete(instance: *mut c_void);
// Mat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:959
// ("cv::Mat::Mat", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_Mat() -> *mut c_void;
// Mat(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:967
// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_Mat_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// Mat(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:975
// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_Mat_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// Mat(int, int, int, const Scalar &)(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:986
// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_Mat_int_int_int_const_ScalarR(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Mat(Size, int, const Scalar &)(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:997
// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_Mat_Size_int_const_ScalarR(size: *const core::Size, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Mat(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1005
// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_Mat_Mat_int_const_intX_int(ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const std::vector<int> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1012
// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
pub fn cv_Mat_Mat_const_vectorLintGR_int(sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const MatShape &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1019
// ("cv::Mat::Mat", vec![(pred!(mut, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv_Mat_Mat_const_MatShapeR_int(shape: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// Mat(int, const int *, int, const Scalar &)(Primitive, VariableArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1037
// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "s"], ["int", "const int*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_Mat_int_const_intX_int_const_ScalarR(ndims: i32, sizes: *const i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const std::vector<int> &, int, const Scalar &)(CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1047
// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "s"], ["const std::vector<int>*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_Mat_const_vectorLintGR_int_const_ScalarR(sizes: *const c_void, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const MatShape &, int, const Scalar &)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1057
// ("cv::Mat::Mat", vec![(pred!(mut, ["shape", "type", "s"], ["const cv::MatShape*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_Mat_const_MatShapeR_int_const_ScalarR(shape: *const c_void, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1076
// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_Mat_Mat_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1076
// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_Mat_Mat_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(int, int, int, void *, size_t)(Primitive, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1092
// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "data", "step"], ["int", "int", "int", "void*", "size_t"]), _)]),
pub fn cv_Mat_Mat_int_int_int_voidX_size_t(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1092
// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "data"], ["int", "int", "int", "void*"]), _)]),
pub fn cv_Mat_Mat_int_int_int_voidX(rows: i32, cols: i32, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(Size, int, void *, size_t)(SimpleClass, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1108
// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "data", "step"], ["cv::Size", "int", "void*", "size_t"]), _)]),
pub fn cv_Mat_Mat_Size_int_voidX_size_t(size: *const core::Size, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(SimpleClass, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1108
// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "data"], ["cv::Size", "int", "void*"]), _)]),
pub fn cv_Mat_Mat_Size_int_voidX(size: *const core::Size, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(int, const int *, int, void *, const size_t *)(Primitive, VariableArray, Primitive, Indirect, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1123
// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "data", "steps"], ["int", "const int*", "int", "void*", "const size_t*"]), _)]),
pub fn cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(ndims: i32, sizes: *const i32, typ: i32, data: *mut c_void, steps: *const size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(Primitive, VariableArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1123
// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "data"], ["int", "const int*", "int", "void*"]), _)]),
pub fn cv_Mat_Mat_int_const_intX_int_voidX(ndims: i32, sizes: *const i32, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const std::vector<int> &, int, void *, const size_t *)(CppPassByVoidPtr, Primitive, Indirect, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1137
// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "data", "steps"], ["const std::vector<int>*", "int", "void*", "const size_t*"]), _)]),
pub fn cv_Mat_Mat_const_vectorLintGR_int_voidX_const_size_tX(sizes: *const c_void, typ: i32, data: *mut c_void, steps: *const size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(CppPassByVoidPtr, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1137
// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "data"], ["const std::vector<int>*", "int", "void*"]), _)]),
pub fn cv_Mat_Mat_const_vectorLintGR_int_voidX(sizes: *const c_void, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const MatShape &, int, void *, const size_t *)(TraitClass, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1151
// ("cv::Mat::Mat", vec![(pred!(mut, ["shape", "type", "data", "steps"], ["const cv::MatShape*", "int", "void*", "const size_t*"]), _)]),
pub fn cv_Mat_Mat_const_MatShapeR_int_voidX_const_size_tX(shape: *const c_void, typ: i32, data: *mut c_void, steps: *const size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1151
// ("cv::Mat::Mat", vec![(pred!(mut, ["shape", "type", "data"], ["const cv::MatShape*", "int", "void*"]), _)]),
pub fn cv_Mat_Mat_const_MatShapeR_int_voidX(shape: *const c_void, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const Mat &, const Range &, const Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1177
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::Mat*", "const cv::Range*", "const cv::Range*"]), _)]),
pub fn cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(m: *const c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1177
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange"], ["cv::Mat*", "const cv::Range*"]), _)]),
pub fn cv_Mat_Mat_MatR_const_RangeR(m: *mut c_void, row_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1177
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange"], ["const cv::Mat*", "const cv::Range*"]), _)]),
pub fn cv_Mat_Mat_const_MatR_const_RangeR(m: *const c_void, row_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1177
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::Mat*", "const cv::Range*", "const cv::Range*"]), _)]),
pub fn cv_Mat_Mat_MatR_const_RangeR_const_RangeR(m: *mut c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const Mat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1187
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "roi"], ["const cv::Mat*", "const cv::Rect*"]), _)]),
pub fn cv_Mat_Mat_const_MatR_const_RectR(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1187
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "roi"], ["cv::Mat*", "const cv::Rect*"]), _)]),
pub fn cv_Mat_Mat_MatR_const_RectR(m: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const Mat &, const std::vector<Range> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1207
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "ranges"], ["const cv::Mat*", "const std::vector<cv::Range>*"]), _)]),
pub fn cv_Mat_Mat_const_MatR_const_vectorLRangeGR(m: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::Mat(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1207
// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "ranges"], ["cv::Mat*", "const std::vector<cv::Range>*"]), _)]),
pub fn cv_Mat_Mat_MatR_const_vectorLRangeGR(m: *mut c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Mat(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1261
// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv_Mat_Mat_const_GpuMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1283
// ("cv::Mat::operator=", vec![(pred!(mut, ["expr"], ["const cv::MatExpr*"]), _)]),
pub fn cv_Mat_operatorST_const_MatExprR(instance: *mut c_void, expr: *const c_void, ocvrs_return: *mut Result<()>);
// getUMat(AccessFlag, UMatUsageFlags)(Enum, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1286
// ("cv::Mat::getUMat", vec![(pred!(const, ["accessFlags", "usageFlags"], ["cv::AccessFlag", "cv::UMatUsageFlags"]), _)]),
pub fn cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(instance: *const c_void, access_flags: core::AccessFlag, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::getUMat(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1286
// ("cv::Mat::getUMat", vec![(pred!(const, ["accessFlags"], ["cv::AccessFlag"]), _)]),
pub fn cv_Mat_getUMat_const_AccessFlag(instance: *const c_void, access_flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1321
// ("cv::Mat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
pub fn cv_Mat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::row(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1321
// ("cv::Mat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
pub fn cv_Mat_row_int(instance: *mut c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1330
// ("cv::Mat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
pub fn cv_Mat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::col(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1330
// ("cv::Mat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
pub fn cv_Mat_col_int(instance: *mut c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1339
// ("cv::Mat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_Mat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1339
// ("cv::Mat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_Mat_rowRange_int_int(instance: *mut c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1344
// ("cv::Mat::rowRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_Mat_rowRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1344
// ("cv::Mat::rowRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_Mat_rowRange_const_RangeR(instance: *mut c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1353
// ("cv::Mat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_Mat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1353
// ("cv::Mat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_Mat_colRange_int_int(instance: *mut c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// colRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1358
// ("cv::Mat::colRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_Mat_colRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1358
// ("cv::Mat::colRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_Mat_colRange_const_RangeR(instance: *mut c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1394
// ("cv::Mat::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
pub fn cv_Mat_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::diag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1394
// ("cv::Mat::diag", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_diag(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::diag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1394
// ("cv::Mat::diag", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_diag_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::diag(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1394
// ("cv::Mat::diag", vec![(pred!(mut, ["d"], ["int"]), _)]),
pub fn cv_Mat_diag_int(instance: *mut c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
// diag(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1401
// ("cv::Mat::diag", vec![(pred!(mut, ["d"], ["const cv::Mat*"]), _)]),
pub fn cv_Mat_diag_const_MatR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1408
// ("cv::Mat::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1425
// ("cv::Mat::copyTo", vec![(pred!(const, ["m"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_Mat_copyTo_const_const__OutputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1434
// ("cv::Mat::copyTo", vec![(pred!(const, ["m", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, m: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1449
// ("cv::Mat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, m: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::Mat::convertTo(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1449
// ("cv::Mat::convertTo", vec![(pred!(const, ["m", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_Mat_convertTo_const_const__OutputArrayR_int(instance: *const c_void, m: *const c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// assignTo(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1457
// ("cv::Mat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::Mat*", "int"]), _)]),
pub fn cv_Mat_assignTo_const_MatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::Mat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1457
// ("cv::Mat::assignTo", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_Mat_assignTo_const_MatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// operator=(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1462
// ("cv::Mat::operator=", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
pub fn cv_Mat_operatorST_const_ScalarR(instance: *mut c_void, s: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setTo(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1472
// ("cv::Mat::setTo", vec![(pred!(mut, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_Mat_setTo_const__InputArrayR_const__InputArrayR(instance: *mut c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1472
// ("cv::Mat::setTo", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
pub fn cv_Mat_setTo_const__InputArrayR(instance: *mut c_void, value: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setZero()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1476
// ("cv::Mat::setZero", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_setZero(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1514
// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_Mat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1514
// ("cv::Mat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
pub fn cv_Mat_reshape_int(instance: *mut c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1514
// ("cv::Mat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
pub fn cv_Mat_reshape_const_int(instance: *const c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1514
// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_Mat_reshape_int_int(instance: *mut c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, int, const int *)(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1522
// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
pub fn cv_Mat_reshape_const_int_int_const_intX(instance: *const c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1522
// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
pub fn cv_Mat_reshape_int_int_const_intX(instance: *mut c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, const std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1529
// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "newshape"], ["int", "const std::vector<int>*"]), _)]),
pub fn cv_Mat_reshape_const_int_const_vectorLintGR(instance: *const c_void, cn: i32, newshape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1529
// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "newshape"], ["int", "const std::vector<int>*"]), _)]),
pub fn cv_Mat_reshape_int_const_vectorLintGR(instance: *mut c_void, cn: i32, newshape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, const MatShape &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1535
// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "newshape"], ["int", "const cv::MatShape*"]), _)]),
pub fn cv_Mat_reshape_const_int_const_MatShapeR(instance: *const c_void, cn: i32, newshape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::reshape(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1535
// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "newshape"], ["int", "const cv::MatShape*"]), _)]),
pub fn cv_Mat_reshape_int_const_MatShapeR(instance: *mut c_void, cn: i32, newshape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// t()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1553
// ("cv::Mat::t", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1562
// ("cv::Mat::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
pub fn cv_Mat_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::inv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1562
// ("cv::Mat::inv", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_inv_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mul(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1576
// ("cv::Mat::mul", vec![(pred!(const, ["m", "scale"], ["const cv::_InputArray*", "double"]), _)]),
pub fn cv_Mat_mul_const_const__InputArrayR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::mul(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1576
// ("cv::Mat::mul", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
pub fn cv_Mat_mul_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cross(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1585
// ("cv::Mat::cross", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
pub fn cv_Mat_cross_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dot(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1595
// ("cv::Mat::dot", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
pub fn cv_Mat_dot_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
// zeros(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1611
// ("cv::Mat::zeros", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_zeros_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// zeros(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1617
// ("cv::Mat::zeros", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_zeros_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// zeros(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1624
// ("cv::Mat::zeros", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_Mat_zeros_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1642
// ("cv::Mat::ones", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_ones_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1648
// ("cv::Mat::ones", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_ones_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1655
// ("cv::Mat::ones", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_Mat_ones_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// eye(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1671
// ("cv::Mat::eye", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_eye_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// eye(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1677
// ("cv::Mat::eye", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_eye_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1712
// ("cv::Mat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1718
// ("cv::Mat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// create(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1725
// ("cv::Mat::create", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_Mat_create_int_const_intX_int(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(const std::vector<int> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1731
// ("cv::Mat::create", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
pub fn cv_Mat_create_const_vectorLintGR_int(instance: *mut c_void, sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// create(const MatShape &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1737
// ("cv::Mat::create", vec![(pred!(mut, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv_Mat_create_const_MatShapeR_int(instance: *mut c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// createSameSize(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1752
// ("cv::Mat::createSameSize", vec![(pred!(mut, ["arr", "type"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_Mat_createSameSize_const__InputArrayR_int(instance: *mut c_void, arr: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1759
// ("cv::Mat::fit", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_fit_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1765
// ("cv::Mat::fit", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_Mat_fit_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1772
// ("cv::Mat::fit", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_Mat_fit_int_const_intX_int(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(const std::vector<int> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1778
// ("cv::Mat::fit", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
pub fn cv_Mat_fit_const_vectorLintGR_int(instance: *mut c_void, sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fit(const MatShape &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1784
// ("cv::Mat::fit", vec![(pred!(mut, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv_Mat_fit_const_MatShapeR_int(instance: *mut c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fitSameSize(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1796
// ("cv::Mat::fitSameSize", vec![(pred!(mut, ["arr", "type"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_Mat_fitSameSize_const__InputArrayR_int(instance: *mut c_void, arr: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// addref()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1807
// ("cv::Mat::addref", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_addref(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1822
// ("cv::Mat::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// deallocate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1825
// ("cv::Mat::deallocate", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_deallocate(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// reserve(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1836
// ("cv::Mat::reserve", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
pub fn cv_Mat_reserve_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result<()>);
// reserveBuffer(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1844
// ("cv::Mat::reserveBuffer", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
pub fn cv_Mat_reserveBuffer_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result<()>);
// resize(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1853
// ("cv::Mat::resize", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
pub fn cv_Mat_resize_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result<()>);
// resize(size_t, const Scalar &)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1859
// ("cv::Mat::resize", vec![(pred!(mut, ["sz", "s"], ["size_t", "const cv::Scalar*"]), _)]),
pub fn cv_Mat_resize_size_t_const_ScalarR(instance: *mut c_void, sz: size_t, s: *const core::Scalar, ocvrs_return: *mut Result<()>);
// push_back(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1886
// ("cv::Mat::push_back", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_Mat_push_back_const_MatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// pop_back(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1894
// ("cv::Mat::pop_back", vec![(pred!(mut, ["nelems"], ["size_t"]), _)]),
pub fn cv_Mat_pop_back_size_t(instance: *mut c_void, nelems: size_t, ocvrs_return: *mut Result<()>);
// cv::Mat::pop_back() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1894
// ("cv::Mat::pop_back", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_pop_back(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1907
// ("cv::Mat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
pub fn cv_Mat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result<()>);
// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1936
// ("cv::Mat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
pub fn cv_Mat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1949
// ("cv::Mat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_Mat_operator___const_Range_Range(instance: *const c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1949
// ("cv::Mat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_Mat_operator___Range_Range(instance: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1954
// ("cv::Mat::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
pub fn cv_Mat_operator___const_const_RectR(instance: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1954
// ("cv::Mat::operator()", vec![(pred!(mut, ["roi"], ["const cv::Rect*"]), _)]),
pub fn cv_Mat_operator___const_RectR(instance: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const std::vector<Range> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1964
// ("cv::Mat::operator()", vec![(pred!(const, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_Mat_operator___const_const_vectorLRangeGR(instance: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::operator()(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:1964
// ("cv::Mat::operator()", vec![(pred!(mut, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_Mat_operator___const_vectorLRangeGR(instance: *mut c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// isContinuous()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2046
// ("cv::Mat::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_isContinuous_const(instance: *const c_void) -> bool;
// isSubmatrix()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2049
// ("cv::Mat::isSubmatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_isSubmatrix_const(instance: *const c_void) -> bool;
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2056
// ("cv::Mat::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2063
// ("cv::Mat::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_elemSize1_const(instance: *const c_void) -> size_t;
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2070
// ("cv::Mat::type", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_type_const(instance: *const c_void) -> i32;
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2085
// ("cv::Mat::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_depth_const(instance: *const c_void) -> i32;
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2091
// ("cv::Mat::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_channels_const(instance: *const c_void) -> i32;
// step1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2098
// ("cv::Mat::step1", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_Mat_step1_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
// cv::Mat::step1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2098
// ("cv::Mat::step1", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// shape()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2102
// ("cv::Mat::shape", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_shape_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2109
// ("cv::Mat::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_empty_const(instance: *const c_void) -> bool;
// total()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2116
// ("cv::Mat::total", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_total_const(instance: *const c_void) -> size_t;
// total(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2122
// ("cv::Mat::total", vec![(pred!(const, ["startDim", "endDim"], ["int", "int"]), _)]),
pub fn cv_Mat_total_const_int_int(instance: *const c_void, start_dim: i32, end_dim: i32, ocvrs_return: *mut Result<size_t>);
// cv::Mat::total(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2122
// ("cv::Mat::total", vec![(pred!(const, ["startDim"], ["int"]), _)]),
pub fn cv_Mat_total_const_int(instance: *const c_void, start_dim: i32, ocvrs_return: *mut Result<size_t>);
// checkVector(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2145
// ("cv::Mat::checkVector", vec![(pred!(const, ["elemChannels", "depth", "requireContinuous"], ["int", "int", "bool"]), _)]),
pub fn cv_Mat_checkVector_const_int_int_bool(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool, ocvrs_return: *mut Result<i32>);
// cv::Mat::checkVector(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2145
// ("cv::Mat::checkVector", vec![(pred!(const, ["elemChannels"], ["int"]), _)]),
pub fn cv_Mat_checkVector_const_int(instance: *const c_void, elem_channels: i32, ocvrs_return: *mut Result<i32>);
// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2153
// ("cv::Mat::ptr", vec![(pred!(mut, ["i0"], ["int"]), _)]),
pub fn cv_Mat_ptr_int(instance: *mut c_void, i0: i32, ocvrs_return: *mut Result<*mut u8>);
// cv::Mat::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2153
// ("cv::Mat::ptr", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_ptr(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2155
// ("cv::Mat::ptr", vec![(pred!(const, ["i0"], ["int"]), _)]),
pub fn cv_Mat_ptr_const_int(instance: *const c_void, i0: i32, ocvrs_return: *mut Result<*const u8>);
// cv::Mat::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2155
// ("cv::Mat::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
// ptr(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2161
// ("cv::Mat::ptr", vec![(pred!(mut, ["row", "col"], ["int", "int"]), _)]),
pub fn cv_Mat_ptr_int_int(instance: *mut c_void, row: i32, col: i32, ocvrs_return: *mut Result<*mut u8>);
// ptr(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2166
// ("cv::Mat::ptr", vec![(pred!(const, ["row", "col"], ["int", "int"]), _)]),
pub fn cv_Mat_ptr_const_int_int(instance: *const c_void, row: i32, col: i32, ocvrs_return: *mut Result<*const u8>);
// ptr(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2169
// ("cv::Mat::ptr", vec![(pred!(mut, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_ptr_int_int_int(instance: *mut c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<*mut u8>);
// ptr(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2171
// ("cv::Mat::ptr", vec![(pred!(const, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
pub fn cv_Mat_ptr_const_int_int_int(instance: *const c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<*const u8>);
// ptr(const int *)(VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2174
// ("cv::Mat::ptr", vec![(pred!(mut, ["idx"], ["const int*"]), _)]),
pub fn cv_Mat_ptr_const_intX(instance: *mut c_void, idx: *const i32, ocvrs_return: *mut Result<*mut u8>);
// ptr(const int *)(VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2176
// ("cv::Mat::ptr", vec![(pred!(const, ["idx"], ["const int*"]), _)]),
pub fn cv_Mat_ptr_const_const_intX(instance: *const c_void, idx: *const i32, ocvrs_return: *mut Result<*const u8>);
// operator=(Mat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2410
// ("cv::Mat::operator=", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_Mat_operatorST_MatRR(instance: *mut c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2443
// ("cv::Mat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::Mat::size() generated
// ("cv::Mat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// cv::Mat::getDataDump() generated
// ("cv::Mat::getDataDump", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_getDataDump_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Mat::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2421
// ("cv::Mat::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propFlags_const(instance: *const c_void) -> i32;
// cv::Mat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2421
// ("cv::Mat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Mat_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::Mat::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2423
// ("cv::Mat::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propDims_const(instance: *const c_void) -> i32;
// cv::Mat::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2423
// ("cv::Mat::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Mat_propDims_const_int(instance: *mut c_void, val: i32);
// cv::Mat::rows() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2425
// ("cv::Mat::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propRows_const(instance: *const c_void) -> i32;
// cv::Mat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2425
// ("cv::Mat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Mat_propRows_const_int(instance: *mut c_void, val: i32);
// cv::Mat::cols() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2425
// ("cv::Mat::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propCols_const(instance: *const c_void) -> i32;
// cv::Mat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2425
// ("cv::Mat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Mat_propCols_const_int(instance: *mut c_void, val: i32);
// cv::Mat::dummy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2426
// ("cv::Mat::dummy", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propDummy_const(instance: *const c_void) -> i32;
// cv::Mat::setDummy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2426
// ("cv::Mat::setDummy", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Mat_propDummy_const_int(instance: *mut c_void, val: i32);
// cv::Mat::data() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2428
// ("cv::Mat::data", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propData_const(instance: *const c_void) -> *const u8;
// cv::Mat::dataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2428
// ("cv::Mat::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_propData(instance: *mut c_void) -> *mut u8;
// cv::Mat::setData(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2428
// ("cv::Mat::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_Mat_propData_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::Mat::datastart() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2431
// ("cv::Mat::datastart", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propDatastart_const(instance: *const c_void) -> *const u8;
// cv::Mat::dataend() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2432
// ("cv::Mat::dataend", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propDataend_const(instance: *const c_void) -> *const u8;
// cv::Mat::datalimit() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2433
// ("cv::Mat::datalimit", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propDatalimit_const(instance: *const c_void) -> *const u8;
// cv::Mat::u() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2446
// ("cv::Mat::u", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_propU(instance: *mut c_void) -> *mut c_void;
// cv::Mat::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2446
// ("cv::Mat::setU", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
pub fn cv_Mat_propU_UMatDataX(instance: *mut c_void, val: *const c_void);
// cv::Mat::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2448
// ("cv::Mat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propSize_const(instance: *const c_void) -> *mut c_void;
// cv::Mat::setSize(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2448
// ("cv::Mat::setSize", vec![(pred!(mut, ["val"], ["const cv::MatSize"]), _)]),
pub fn cv_Mat_propSize_const_MatSize(instance: *mut c_void, val: *const c_void);
// cv::Mat::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2449
// ("cv::Mat::step", vec![(pred!(const, [], []), _)]),
pub fn cv_Mat_propStep_const(instance: *const c_void) -> *mut c_void;
// cv::Mat::delete() generated
// ("cv::Mat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Mat_delete(instance: *mut c_void);
// MatConstIterator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3389
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatConstIterator_MatConstIterator(ocvrs_return: *mut Result<*mut c_void>);
// MatConstIterator(const Mat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3391
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m"], ["const cv::Mat*"]), _)]),
pub fn cv_MatConstIterator_MatConstIterator_const_MatX(_m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// MatConstIterator(const Mat *, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3393
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_row", "_col"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_MatConstIterator_MatConstIterator_const_MatX_int_int(_m: *const c_void, _row: i32, _col: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatConstIterator::MatConstIterator(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3393
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_row"], ["const cv::Mat*", "int"]), _)]),
pub fn cv_MatConstIterator_MatConstIterator_const_MatX_int(_m: *const c_void, _row: i32, ocvrs_return: *mut Result<*mut c_void>);
// MatConstIterator(const Mat *, Point)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3395
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_pt"], ["const cv::Mat*", "cv::Point"]), _)]),
pub fn cv_MatConstIterator_MatConstIterator_const_MatX_Point(_m: *const c_void, _pt: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// MatConstIterator(const MatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3399
// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["it"], ["const cv::MatConstIterator*"]), _)]),
pub fn cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const MatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3402
// ("cv::MatConstIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::MatConstIterator*"]), _)]),
pub fn cv_MatConstIterator_operatorST_const_MatConstIteratorR(instance: *mut c_void, it: *const c_void, ocvrs_return: *mut Result<()>);
// operator*()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3404
// ("cv::MatConstIterator::operator*", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_operatorX_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
// operator[](ptrdiff_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3406
// ("cv::MatConstIterator::operator[]", vec![(pred!(const, ["i"], ["ptrdiff_t"]), _)]),
pub fn cv_MatConstIterator_operator___const_ptrdiff_t(instance: *const c_void, i: ptrdiff_t, ocvrs_return: *mut Result<*const u8>);
// operator--()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3413
// ("cv::MatConstIterator::operator--", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatConstIterator_operatorSS(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3417
// ("cv::MatConstIterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatConstIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// pos()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3421
// ("cv::MatConstIterator::pos", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_pos_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point>);
// pos(int *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3423
// ("cv::MatConstIterator::pos", vec![(pred!(const, ["_idx"], ["int*"]), _)]),
pub fn cv_MatConstIterator_pos_const_intX(instance: *const c_void, _idx: *mut i32, ocvrs_return: *mut Result<()>);
// lpos()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3425
// ("cv::MatConstIterator::lpos", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_lpos_const(instance: *const c_void, ocvrs_return: *mut Result<ptrdiff_t>);
// seek(ptrdiff_t, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3426
// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["ofs", "relative"], ["ptrdiff_t", "bool"]), _)]),
pub fn cv_MatConstIterator_seek_ptrdiff_t_bool(instance: *mut c_void, ofs: ptrdiff_t, relative: bool, ocvrs_return: *mut Result<()>);
// cv::MatConstIterator::seek(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3426
// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["ofs"], ["ptrdiff_t"]), _)]),
pub fn cv_MatConstIterator_seek_ptrdiff_t(instance: *mut c_void, ofs: ptrdiff_t, ocvrs_return: *mut Result<()>);
// seek(const int *, bool)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3427
// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["_idx", "relative"], ["const int*", "bool"]), _)]),
pub fn cv_MatConstIterator_seek_const_intX_bool(instance: *mut c_void, _idx: *const i32, relative: bool, ocvrs_return: *mut Result<()>);
// cv::MatConstIterator::seek(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3427
// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["_idx"], ["const int*"]), _)]),
pub fn cv_MatConstIterator_seek_const_intX(instance: *mut c_void, _idx: *const i32, ocvrs_return: *mut Result<()>);
// cv::MatConstIterator::type() generated
// ("cv::MatConstIterator::type", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_type_const(instance: *const c_void) -> i32;
// cv::MatConstIterator::m() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3429
// ("cv::MatConstIterator::m", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_propM_const(instance: *const c_void) -> *mut c_void;
// cv::MatConstIterator::elemSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3430
// ("cv::MatConstIterator::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_propElemSize_const(instance: *const c_void) -> size_t;
// cv::MatConstIterator::setElemSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3430
// ("cv::MatConstIterator::setElemSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_MatConstIterator_propElemSize_const_size_t(instance: *mut c_void, val: size_t);
// cv::MatConstIterator::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3431
// ("cv::MatConstIterator::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_propPtr_const(instance: *const c_void) -> *const u8;
// cv::MatConstIterator::sliceStart() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3432
// ("cv::MatConstIterator::sliceStart", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_propSliceStart_const(instance: *const c_void) -> *const u8;
// cv::MatConstIterator::sliceEnd() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3433
// ("cv::MatConstIterator::sliceEnd", vec![(pred!(const, [], []), _)]),
pub fn cv_MatConstIterator_propSliceEnd_const(instance: *const c_void) -> *const u8;
// cv::MatConstIterator::delete() generated
// ("cv::MatConstIterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatConstIterator_delete(instance: *mut c_void);
// MatExpr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3883
// ("cv::MatExpr::MatExpr", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatExpr_MatExpr(ocvrs_return: *mut Result<*mut c_void>);
// MatExpr(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3884
// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_MatExpr_MatExpr_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// MatExpr(const MatOp *, int, const Mat &, const Mat &, const Mat &, double, double, const Scalar &)(TraitClass, Primitive, TraitClass, TraitClass, TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3886
// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["_op", "_flags", "_a", "_b", "_c", "_alpha", "_beta", "_s"], ["const cv::MatOp*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "double", "double", "const cv::Scalar*"]), _)]),
pub fn cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(_op: *const c_void, _flags: i32, _a: *const c_void, _b: *const c_void, _c: *const c_void, _alpha: f64, _beta: f64, _s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatExpr::MatExpr(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3886
// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["_op", "_flags"], ["const cv::MatOp*", "int"]), _)]),
pub fn cv_MatExpr_MatExpr_const_MatOpX_int(_op: *const c_void, _flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator Mat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3889
// ("cv::MatExpr::operator cv::Mat", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_operator_cv_Mat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3892
// ("cv::MatExpr::size", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3893
// ("cv::MatExpr::type", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3895
// ("cv::MatExpr::row", vec![(pred!(const, ["y"], ["int"]), _)]),
pub fn cv_MatExpr_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3896
// ("cv::MatExpr::col", vec![(pred!(const, ["x"], ["int"]), _)]),
pub fn cv_MatExpr_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3897
// ("cv::MatExpr::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
pub fn cv_MatExpr_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatExpr::diag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3897
// ("cv::MatExpr::diag", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_diag_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Range &, const Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3898
// ("cv::MatExpr::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["const cv::Range*", "const cv::Range*"]), _)]),
pub fn cv_MatExpr_operator___const_const_RangeR_const_RangeR(instance: *const c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3899
// ("cv::MatExpr::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
pub fn cv_MatExpr_operator___const_const_RectR(instance: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// t()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3901
// ("cv::MatExpr::t", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3902
// ("cv::MatExpr::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
pub fn cv_MatExpr_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatExpr::inv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3902
// ("cv::MatExpr::inv", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_inv_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mul(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3903
// ("cv::MatExpr::mul", vec![(pred!(const, ["e", "scale"], ["const cv::MatExpr*", "double"]), _)]),
pub fn cv_MatExpr_mul_const_const_MatExprR_double(instance: *const c_void, e: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatExpr::mul(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3903
// ("cv::MatExpr::mul", vec![(pred!(const, ["e"], ["const cv::MatExpr*"]), _)]),
pub fn cv_MatExpr_mul_const_const_MatExprR(instance: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mul(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3904
// ("cv::MatExpr::mul", vec![(pred!(const, ["m", "scale"], ["const cv::Mat*", "double"]), _)]),
pub fn cv_MatExpr_mul_const_const_MatR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatExpr::mul(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3904
// ("cv::MatExpr::mul", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_MatExpr_mul_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cross(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3906
// ("cv::MatExpr::cross", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_MatExpr_cross_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dot(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3907
// ("cv::MatExpr::dot", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_MatExpr_dot_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
// swap(MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3909
// ("cv::MatExpr::swap", vec![(pred!(mut, ["b"], ["cv::MatExpr*"]), _)]),
pub fn cv_MatExpr_swap_MatExprR(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::MatExpr::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3912
// ("cv::MatExpr::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propFlags_const(instance: *const c_void) -> i32;
// cv::MatExpr::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3912
// ("cv::MatExpr::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_MatExpr_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::MatExpr::a() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::a", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propA_const(instance: *const c_void) -> *mut c_void;
// cv::MatExpr::setA(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::setA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_MatExpr_propA_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::MatExpr::b() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::b", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propB_const(instance: *const c_void) -> *mut c_void;
// cv::MatExpr::setB(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::setB", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_MatExpr_propB_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::MatExpr::c() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::c", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propC_const(instance: *const c_void) -> *mut c_void;
// cv::MatExpr::setC(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3914
// ("cv::MatExpr::setC", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_MatExpr_propC_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::MatExpr::alpha() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3915
// ("cv::MatExpr::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propAlpha_const(instance: *const c_void) -> f64;
// cv::MatExpr::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3915
// ("cv::MatExpr::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_MatExpr_propAlpha_const_double(instance: *mut c_void, val: f64);
// cv::MatExpr::beta() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3915
// ("cv::MatExpr::beta", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propBeta_const(instance: *const c_void) -> f64;
// cv::MatExpr::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3915
// ("cv::MatExpr::setBeta", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_MatExpr_propBeta_const_double(instance: *mut c_void, val: f64);
// cv::MatExpr::s() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3916
// ("cv::MatExpr::s", vec![(pred!(const, [], []), _)]),
pub fn cv_MatExpr_propS_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::MatExpr::setS(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3916
// ("cv::MatExpr::setS", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_MatExpr_propS_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::MatExpr::delete() generated
// ("cv::MatExpr::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatExpr_delete(instance: *mut c_void);
// elementWise(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3799
// ("cv::MatOp::elementWise", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
pub fn cv_MatOp_elementWise_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<bool>);
// assign(const MatExpr &, Mat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3800
// ("cv::MatOp::assign", vec![(pred!(const, ["expr", "m", "type"], ["const cv::MatExpr*", "cv::Mat*", "int"]), _)]),
pub fn cv_MatOp_assign_const_const_MatExprR_MatR_int(instance: *const c_void, expr: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::MatOp::assign(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3800
// ("cv::MatOp::assign", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_assign_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// roi(const MatExpr &, const Range &, const Range &, MatExpr &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3801
// ("cv::MatOp::roi", vec![(pred!(const, ["expr", "rowRange", "colRange", "res"], ["const cv::MatExpr*", "const cv::Range*", "const cv::Range*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(instance: *const c_void, expr: *const c_void, row_range: *const c_void, col_range: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// diag(const MatExpr &, int, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3803
// ("cv::MatOp::diag", vec![(pred!(const, ["expr", "d", "res"], ["const cv::MatExpr*", "int", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_diag_const_const_MatExprR_int_MatExprR(instance: *const c_void, expr: *const c_void, d: i32, res: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignAdd(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3804
// ("cv::MatOp::augAssignAdd", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignSubtract(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3805
// ("cv::MatOp::augAssignSubtract", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignMultiply(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3806
// ("cv::MatOp::augAssignMultiply", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignDivide(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3807
// ("cv::MatOp::augAssignDivide", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignAnd(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3808
// ("cv::MatOp::augAssignAnd", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignOr(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3809
// ("cv::MatOp::augAssignOr", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignOr_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// augAssignXor(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3810
// ("cv::MatOp::augAssignXor", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
pub fn cv_MatOp_augAssignXor_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// add(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3812
// ("cv::MatOp::add", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// add(const MatExpr &, const Scalar &, MatExpr &)(TraitClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3813
// ("cv::MatOp::add", vec![(pred!(const, ["expr1", "s", "res"], ["const cv::MatExpr*", "const cv::Scalar*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(instance: *const c_void, expr1: *const c_void, s: *const core::Scalar, res: *mut c_void, ocvrs_return: *mut Result<()>);
// subtract(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3815
// ("cv::MatOp::subtract", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// subtract(const Scalar &, const MatExpr &, MatExpr &)(SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3816
// ("cv::MatOp::subtract", vec![(pred!(const, ["s", "expr", "res"], ["const cv::Scalar*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(instance: *const c_void, s: *const core::Scalar, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// multiply(const MatExpr &, const MatExpr &, MatExpr &, double)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3818
// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "expr2", "res", "scale"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*", "double"]), _)]),
pub fn cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, scale: f64, ocvrs_return: *mut Result<()>);
// cv::MatOp::multiply(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3818
// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// multiply(const MatExpr &, double, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3819
// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "s", "res"], ["const cv::MatExpr*", "double", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(instance: *const c_void, expr1: *const c_void, s: f64, res: *mut c_void, ocvrs_return: *mut Result<()>);
// divide(const MatExpr &, const MatExpr &, MatExpr &, double)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3821
// ("cv::MatOp::divide", vec![(pred!(const, ["expr1", "expr2", "res", "scale"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*", "double"]), _)]),
pub fn cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, scale: f64, ocvrs_return: *mut Result<()>);
// cv::MatOp::divide(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3821
// ("cv::MatOp::divide", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// divide(double, const MatExpr &, MatExpr &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3822
// ("cv::MatOp::divide", vec![(pred!(const, ["s", "expr", "res"], ["double", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_divide_const_double_const_MatExprR_MatExprR(instance: *const c_void, s: f64, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// abs(const MatExpr &, MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3824
// ("cv::MatOp::abs", vec![(pred!(const, ["expr", "res"], ["const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_abs_const_const_MatExprR_MatExprR(instance: *const c_void, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// transpose(const MatExpr &, MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3826
// ("cv::MatOp::transpose", vec![(pred!(const, ["expr", "res"], ["const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_transpose_const_const_MatExprR_MatExprR(instance: *const c_void, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// matmul(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3827
// ("cv::MatOp::matmul", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result<()>);
// invert(const MatExpr &, int, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3828
// ("cv::MatOp::invert", vec![(pred!(const, ["expr", "method", "res"], ["const cv::MatExpr*", "int", "cv::MatExpr*"]), _)]),
pub fn cv_MatOp_invert_const_const_MatExprR_int_MatExprR(instance: *const c_void, expr: *const c_void, method: i32, res: *mut c_void, ocvrs_return: *mut Result<()>);
// size(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3830
// ("cv::MatOp::size", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
pub fn cv_MatOp_size_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<core::Size>);
// type(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3831
// ("cv::MatOp::type", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
pub fn cv_MatOp_type_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::MatOp::delete() generated
// ("cv::MatOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatOp_delete(instance: *mut c_void);
// MatShape()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:110
// ("cv::MatShape::MatShape", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_MatShape(ocvrs_return: *mut Result<*mut c_void>);
// MatShape(size_t, const int *, DataLayout, int)(Primitive, VariableArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:111
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "sizes", "layout", "C"], ["size_t", "const int*", "cv::DataLayout", "int"]), _)]),
pub fn cv_MatShape_MatShape_size_t_const_intX_DataLayout_int(dims: size_t, sizes: *const i32, layout: core::DataLayout, c: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::MatShape(Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:111
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "sizes"], ["size_t", "const int*"]), _)]),
pub fn cv_MatShape_MatShape_size_t_const_intX(dims: size_t, sizes: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// MatShape(size_t, int, DataLayout)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:112
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "value", "layout"], ["size_t", "int", "cv::DataLayout"]), _)]),
pub fn cv_MatShape_MatShape_size_t_int_DataLayout(dims: size_t, value: i32, layout: core::DataLayout, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::MatShape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:112
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "value"], ["size_t", "int"]), _)]),
pub fn cv_MatShape_MatShape_size_t_int(dims: size_t, value: i32, ocvrs_return: *mut Result<*mut c_void>);
// MatShape(int, int, DataLayout)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:113
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "value", "layout"], ["int", "int", "cv::DataLayout"]), _)]),
pub fn cv_MatShape_MatShape_int_int_DataLayout(dims: i32, value: i32, layout: core::DataLayout, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::MatShape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:113
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["dims", "value"], ["int", "int"]), _)]),
pub fn cv_MatShape_MatShape_int_int(dims: i32, value: i32, ocvrs_return: *mut Result<*mut c_void>);
// MatShape(const std::vector<int> &, DataLayout, int)(CppPassByVoidPtr, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:114
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["shape", "layout", "C"], ["const std::vector<int>*", "cv::DataLayout", "int"]), _)]),
pub fn cv_MatShape_MatShape_const_vectorLintGR_DataLayout_int(shape: *const c_void, layout: core::DataLayout, c: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::MatShape(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:114
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["shape"], ["const std::vector<int>*"]), _)]),
pub fn cv_MatShape_MatShape_const_vectorLintGR(shape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// MatShape(const int *, const int *, DataLayout, int)(Indirect, Indirect, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:115
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["begin", "end", "layout", "C"], ["const int*", "const int*", "cv::DataLayout", "int"]), _)]),
pub fn cv_MatShape_MatShape_const_intX_const_intX_DataLayout_int(begin: *const i32, end: *const i32, layout: core::DataLayout, c: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::MatShape(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:115
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["begin", "end"], ["const int*", "const int*"]), _)]),
pub fn cv_MatShape_MatShape_const_intX_const_intX(begin: *const i32, end: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// MatShape(const MatShape &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:117
// ("cv::MatShape::MatShape", vec![(pred!(mut, ["shape"], ["const cv::MatShape*"]), _)]),
pub fn cv_MatShape_MatShape_const_MatShapeR(shape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const MatShape &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:118
// ("cv::MatShape::operator=", vec![(pred!(mut, ["shape"], ["const cv::MatShape*"]), _)]),
pub fn cv_MatShape_operatorST_const_MatShapeR(instance: *mut c_void, shape: *const c_void, ocvrs_return: *mut Result<()>);
// scalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:119
// ("cv::MatShape::scalar", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_scalar(ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:123
// ("cv::MatShape::size", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:124
// ("cv::MatShape::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isScalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:125
// ("cv::MatShape::isScalar", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_isScalar_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:126
// ("cv::MatShape::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// resize(size_t, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:127
// ("cv::MatShape::resize", vec![(pred!(mut, ["newSize", "value"], ["size_t", "int"]), _)]),
pub fn cv_MatShape_resize_size_t_int(instance: *mut c_void, new_size: size_t, value: i32, ocvrs_return: *mut Result<()>);
// cv::MatShape::resize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:127
// ("cv::MatShape::resize", vec![(pred!(mut, ["newSize"], ["size_t"]), _)]),
pub fn cv_MatShape_resize_size_t(instance: *mut c_void, new_size: size_t, ocvrs_return: *mut Result<()>);
// reserve(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:128
// ("cv::MatShape::reserve", vec![(pred!(mut, ["maxSize"], ["size_t"]), _)]),
pub fn cv_MatShape_reserve_size_t(instance: *mut c_void, max_size: size_t, ocvrs_return: *mut Result<()>);
// assign(size_t, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:129
// ("cv::MatShape::assign", vec![(pred!(mut, ["newSize", "value"], ["size_t", "int"]), _)]),
pub fn cv_MatShape_assign_size_t_int(instance: *mut c_void, new_size: size_t, value: i32, ocvrs_return: *mut Result<()>);
// assign(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:130
// ("cv::MatShape::assign", vec![(pred!(mut, ["newSize", "value"], ["int", "int"]), _)]),
pub fn cv_MatShape_assign_int_int(instance: *mut c_void, new_size: i32, value: i32, ocvrs_return: *mut Result<()>);
// assign(const int *, const int *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:131
// ("cv::MatShape::assign", vec![(pred!(mut, ["begin", "end"], ["const int*", "const int*"]), _)]),
pub fn cv_MatShape_assign_const_intX_const_intX(instance: *mut c_void, begin: *const i32, end: *const i32, ocvrs_return: *mut Result<()>);
// assign_(const int *, const int *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:132
// ("cv::MatShape::assign_", vec![(pred!(mut, ["begin", "end"], ["const int*", "const int*"]), _)]),
pub fn cv_MatShape_assign__const_intX_const_intX(instance: *mut c_void, begin: *const i32, end: *const i32, ocvrs_return: *mut Result<()>);
// insert(int *, int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:134
// ("cv::MatShape::insert", vec![(pred!(mut, ["where", "value"], ["int*", "int"]), _)]),
pub fn cv_MatShape_insert_intX_int(instance: *mut c_void, where_: *mut i32, value: i32, ocvrs_return: *mut Result<()>);
// insert(int *, const int *, const int *)(Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:135
// ("cv::MatShape::insert", vec![(pred!(mut, ["where", "begin", "end"], ["int*", "const int*", "const int*"]), _)]),
pub fn cv_MatShape_insert_intX_const_intX_const_intX(instance: *mut c_void, where_: *mut i32, begin: *const i32, end: *const i32, ocvrs_return: *mut Result<()>);
// insert_(int *, const int *, const int *)(Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:136
// ("cv::MatShape::insert_", vec![(pred!(mut, ["where", "begin", "end"], ["int*", "const int*", "const int*"]), _)]),
pub fn cv_MatShape_insert__intX_const_intX_const_intX(instance: *mut c_void, where_: *mut i32, begin: *const i32, end: *const i32, ocvrs_return: *mut Result<()>);
// insert(int *, size_t, int)(Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:137
// ("cv::MatShape::insert", vec![(pred!(mut, ["where", "count", "value"], ["int*", "size_t", "int"]), _)]),
pub fn cv_MatShape_insert_intX_size_t_int(instance: *mut c_void, where_: *mut i32, count: size_t, value: i32, ocvrs_return: *mut Result<()>);
// insert(int *, int, int)(Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:138
// ("cv::MatShape::insert", vec![(pred!(mut, ["where", "count", "value"], ["int*", "int", "int"]), _)]),
pub fn cv_MatShape_insert_intX_int_int(instance: *mut c_void, where_: *mut i32, count: i32, value: i32, ocvrs_return: *mut Result<()>);
// erase(int *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:140
// ("cv::MatShape::erase", vec![(pred!(mut, ["where"], ["int*"]), _)]),
pub fn cv_MatShape_erase_intX(instance: *mut c_void, where_: *mut i32, ocvrs_return: *mut Result<()>);
// data()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:141
// ("cv::MatShape::data", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_data(instance: *mut c_void, ocvrs_return: *mut Result<*mut i32>);
// data()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:142
// ("cv::MatShape::data", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_data_const(instance: *const c_void, ocvrs_return: *mut Result<*const i32>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:143
// ("cv::MatShape::begin", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_begin(instance: *mut c_void, ocvrs_return: *mut Result<*mut i32>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:144
// ("cv::MatShape::begin", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_begin_const(instance: *const c_void, ocvrs_return: *mut Result<*const i32>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:145
// ("cv::MatShape::end", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_end(instance: *mut c_void, ocvrs_return: *mut Result<*mut i32>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:146
// ("cv::MatShape::end", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_end_const(instance: *const c_void, ocvrs_return: *mut Result<*const i32>);
// back()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:147
// ("cv::MatShape::back", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_back(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// back()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:148
// ("cv::MatShape::back", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_back_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// push_back(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:149
// ("cv::MatShape::push_back", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_MatShape_push_back_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// emplace_back(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:150
// ("cv::MatShape::emplace_back", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_MatShape_emplace_back_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// operator[](size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:151
// ("cv::MatShape::operator[]", vec![(pred!(mut, ["idx"], ["size_t"]), _)]),
pub fn cv_MatShape_operator___size_t(instance: *mut c_void, idx: size_t, ocvrs_return: *mut Result<i32>);
// operator[](size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:152
// ("cv::MatShape::operator[]", vec![(pred!(const, ["idx"], ["size_t"]), _)]),
pub fn cv_MatShape_operator___const_size_t(instance: *const c_void, idx: size_t, ocvrs_return: *mut Result<i32>);
// hasSymbols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:154
// ("cv::MatShape::hasSymbols", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_hasSymbols_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// expand(const MatShape &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:157
// ("cv::MatShape::expand", vec![(pred!(const, ["another"], ["const cv::MatShape*"]), _)]),
pub fn cv_MatShape_expand_const_const_MatShapeR(instance: *const c_void, another: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// toBlock(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:160
// ("cv::MatShape::toBlock", vec![(pred!(const, ["C0"], ["int"]), _)]),
pub fn cv_MatShape_toBlock_const_int(instance: *const c_void, c0: i32, ocvrs_return: *mut Result<*mut c_void>);
// fromBlock(DataLayout)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:161
// ("cv::MatShape::fromBlock", vec![(pred!(const, ["newLayout"], ["cv::DataLayout"]), _)]),
pub fn cv_MatShape_fromBlock_const_DataLayout(instance: *const c_void, new_layout: core::DataLayout, ocvrs_return: *mut Result<*mut c_void>);
// total()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:163
// ("cv::MatShape::total", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_total_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// operator vector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:165
// ("cv::MatShape::operator std::vector<int>", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_operator_std_vectorLintG_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// str()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:166
// ("cv::MatShape::str", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_str_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::MatShape::implicitClone() generated
// ("cv::MatShape::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::MatShape::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:168
// ("cv::MatShape::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_propDims_const(instance: *const c_void) -> i32;
// cv::MatShape::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:168
// ("cv::MatShape::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_MatShape_propDims_const_int(instance: *mut c_void, val: i32);
// cv::MatShape::layout() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:169
// ("cv::MatShape::layout", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_propLayout_const(instance: *const c_void, ocvrs_return: *mut core::DataLayout);
// cv::MatShape::setLayout(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:169
// ("cv::MatShape::setLayout", vec![(pred!(mut, ["val"], ["const cv::DataLayout"]), _)]),
pub fn cv_MatShape_propLayout_const_DataLayout(instance: *mut c_void, val: core::DataLayout);
// cv::MatShape::C() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:170
// ("cv::MatShape::C", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_propC_const(instance: *const c_void) -> i32;
// cv::MatShape::setC(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:170
// ("cv::MatShape::setC", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_MatShape_propC_const_int(instance: *mut c_void, val: i32);
// cv::MatShape::p() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:171
// ("cv::MatShape::p", vec![(pred!(const, [], []), _)]),
pub fn cv_MatShape_propP_const(instance: *const c_void) -> *const [i32; 10];
// cv::MatShape::pMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:171
// ("cv::MatShape::pMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_propP(instance: *mut c_void) -> *mut [i32; 10];
// cv::MatShape::delete() generated
// ("cv::MatShape::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatShape_delete(instance: *mut c_void);
// MatSize(int *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:728
// ("cv::MatSize::MatSize", vec![(pred!(mut, ["_p"], ["int*"]), _)]),
pub fn cv_MatSize_MatSize_intX(_p: *mut i32) -> *mut c_void;
// dims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:729
// ("cv::MatSize::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_MatSize_dims_const(instance: *const c_void) -> i32;
// operator()()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:730
// ("cv::MatSize::operator()", vec![(pred!(const, [], []), _)]),
pub fn cv_MatSize_operator___const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:731
// ("cv::MatSize::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_MatSize_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:732
// ("cv::MatSize::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
pub fn cv_MatSize_operator___int(instance: *mut c_void, i: i32, ocvrs_return: *mut Result<i32>);
// operator const int *()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:733
// ("cv::MatSize::operator const int*", vec![(pred!(const, [], []), _)]),
pub fn cv_MatSize_operator_const_intX_const(instance: *const c_void) -> *const i32;
// operator==(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:734
// ("cv::MatSize::operator==", vec![(pred!(const, ["sz"], ["const cv::MatSize*"]), _)]),
pub fn cv_MatSize_operatorEQ_const_const_MatSizeR(instance: *const c_void, sz: *const c_void) -> bool;
// operator!=(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:735
// ("cv::MatSize::operator!=", vec![(pred!(const, ["sz"], ["const cv::MatSize*"]), _)]),
pub fn cv_MatSize_operatorNE_const_const_MatSizeR(instance: *const c_void, sz: *const c_void) -> bool;
// cv::MatSize::p() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:737
// ("cv::MatSize::p", vec![(pred!(const, [], []), _)]),
pub fn cv_MatSize_propP_const(instance: *const c_void) -> *const i32;
// cv::MatSize::pMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:737
// ("cv::MatSize::pMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatSize_propP(instance: *mut c_void) -> *mut i32;
// cv::MatSize::setP(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:737
// ("cv::MatSize::setP", vec![(pred!(mut, ["val"], ["int*"]), _)]),
pub fn cv_MatSize_propP_intX(instance: *mut c_void, val: *const i32);
// cv::MatSize::delete() generated
// ("cv::MatSize::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatSize_delete(instance: *mut c_void);
// MatStep()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:742
// ("cv::MatStep::MatStep", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatStep_MatStep() -> *mut c_void;
// MatStep(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:743
// ("cv::MatStep::MatStep", vec![(pred!(mut, ["s"], ["size_t"]), _)]),
pub fn cv_MatStep_MatStep_size_t(s: size_t) -> *mut c_void;
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:744
// ("cv::MatStep::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_MatStep_operator___const_int(instance: *const c_void, i: i32) -> size_t;
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:745
// ("cv::MatStep::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
pub fn cv_MatStep_operator___int(instance: *mut c_void, i: i32) -> size_t;
// operator unsigned long()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:746
// ("cv::MatStep::operator size_t", vec![(pred!(const, [], []), _)]),
pub fn cv_MatStep_operator_size_t_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// operator=(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:747
// ("cv::MatStep::operator=", vec![(pred!(mut, ["s"], ["size_t"]), _)]),
pub fn cv_MatStep_operatorST_size_t(instance: *mut c_void, s: size_t, ocvrs_return: *mut Result<()>);
// cv::MatStep::p() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:749
// ("cv::MatStep::p", vec![(pred!(const, [], []), _)]),
pub fn cv_MatStep_propP_const(instance: *const c_void) -> *const size_t;
// cv::MatStep::pMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:749
// ("cv::MatStep::pMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatStep_propP(instance: *mut c_void) -> *mut size_t;
// cv::MatStep::setP(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:749
// ("cv::MatStep::setP", vec![(pred!(mut, ["val"], ["size_t*"]), _)]),
pub fn cv_MatStep_propP_size_tX(instance: *mut c_void, val: *const size_t);
// cv::MatStep::buf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:750
// ("cv::MatStep::buf", vec![(pred!(const, [], []), _)]),
pub fn cv_MatStep_propBuf_const(instance: *const c_void) -> *const [size_t; 3];
// cv::MatStep::bufMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:750
// ("cv::MatStep::bufMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatStep_propBuf(instance: *mut c_void) -> *mut [size_t; 3];
// cv::MatStep::delete() generated
// ("cv::MatStep::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MatStep_delete(instance: *mut c_void);
// Matx_AddOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:72
// ("cv::Matx_AddOp::Matx_AddOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_AddOp_Matx_AddOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_AddOp(const Matx_AddOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:72
// ("cv::Matx_AddOp::Matx_AddOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_AddOp*"]), _)]),
pub fn cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_AddOp::delete() generated
// ("cv::Matx_AddOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_AddOp_delete(instance: *mut c_void);
// Matx_DivOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:76
// ("cv::Matx_DivOp::Matx_DivOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_DivOp_Matx_DivOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_DivOp(const Matx_DivOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:76
// ("cv::Matx_DivOp::Matx_DivOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_DivOp*"]), _)]),
pub fn cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_DivOp::delete() generated
// ("cv::Matx_DivOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_DivOp_delete(instance: *mut c_void);
// Matx_MatMulOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:77
// ("cv::Matx_MatMulOp::Matx_MatMulOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_MatMulOp_Matx_MatMulOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_MatMulOp(const Matx_MatMulOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:77
// ("cv::Matx_MatMulOp::Matx_MatMulOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_MatMulOp*"]), _)]),
pub fn cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_MatMulOp::delete() generated
// ("cv::Matx_MatMulOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_MatMulOp_delete(instance: *mut c_void);
// Matx_MulOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:75
// ("cv::Matx_MulOp::Matx_MulOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_MulOp_Matx_MulOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_MulOp(const Matx_MulOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:75
// ("cv::Matx_MulOp::Matx_MulOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_MulOp*"]), _)]),
pub fn cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_MulOp::delete() generated
// ("cv::Matx_MulOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_MulOp_delete(instance: *mut c_void);
// Matx_ScaleOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:74
// ("cv::Matx_ScaleOp::Matx_ScaleOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_ScaleOp_Matx_ScaleOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_ScaleOp(const Matx_ScaleOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:74
// ("cv::Matx_ScaleOp::Matx_ScaleOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_ScaleOp*"]), _)]),
pub fn cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_ScaleOp::delete() generated
// ("cv::Matx_ScaleOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_ScaleOp_delete(instance: *mut c_void);
// Matx_SubOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:73
// ("cv::Matx_SubOp::Matx_SubOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_SubOp_Matx_SubOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_SubOp(const Matx_SubOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:73
// ("cv::Matx_SubOp::Matx_SubOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_SubOp*"]), _)]),
pub fn cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_SubOp::delete() generated
// ("cv::Matx_SubOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_SubOp_delete(instance: *mut c_void);
// Matx_TOp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:78
// ("cv::Matx_TOp::Matx_TOp", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_TOp_Matx_TOp(ocvrs_return: *mut Result<*mut c_void>);
// Matx_TOp(const Matx_TOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/matx.hpp:78
// ("cv::Matx_TOp::Matx_TOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_TOp*"]), _)]),
pub fn cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::Matx_TOp::delete() generated
// ("cv::Matx_TOp::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Matx_TOp_delete(instance: *mut c_void);
// getFunction()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:81
// ("cv::MinProblemSolver::getFunction", vec![(pred!(const, [], []), _)]),
pub fn cv_MinProblemSolver_getFunction_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setFunction(const Ptr<Function> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:89
// ("cv::MinProblemSolver::setFunction", vec![(pred!(mut, ["f"], ["const cv::Ptr<cv::MinProblemSolver::Function>*"]), _)]),
pub fn cv_MinProblemSolver_setFunction_const_PtrLFunctionGR(instance: *mut c_void, f: *const c_void, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:95
// ("cv::MinProblemSolver::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_MinProblemSolver_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:108
// ("cv::MinProblemSolver::setTermCriteria", vec![(pred!(mut, ["termcrit"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// minimize(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:122
// ("cv::MinProblemSolver::minimize", vec![(pred!(mut, ["x"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_MinProblemSolver_minimize_const__InputOutputArrayR(instance: *mut c_void, x: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::MinProblemSolver::to_ConjGradSolver() generated
// ("cv::MinProblemSolver::to_ConjGradSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_MinProblemSolver_to_ConjGradSolver(instance: *mut c_void) -> *mut c_void;
// cv::MinProblemSolver::to_DownhillSolver() generated
// ("cv::MinProblemSolver::to_DownhillSolver", vec![(pred!(mut, [], []), _)]),
pub fn cv_MinProblemSolver_to_DownhillSolver(instance: *mut c_void) -> *mut c_void;
// cv::MinProblemSolver::to_Algorithm() generated
// ("cv::MinProblemSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MinProblemSolver_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MinProblemSolver::delete() generated
// ("cv::MinProblemSolver::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MinProblemSolver_delete(instance: *mut c_void);
// getDims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:67
// ("cv::MinProblemSolver::Function::getDims", vec![(pred!(const, [], []), _)]),
pub fn cv_MinProblemSolver_Function_getDims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getGradientEps()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:68
// ("cv::MinProblemSolver::Function::getGradientEps", vec![(pred!(const, [], []), _)]),
pub fn cv_MinProblemSolver_Function_getGradientEps_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// calc(const double *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:69
// ("cv::MinProblemSolver::Function::calc", vec![(pred!(const, ["x"], ["const double*"]), _)]),
pub fn cv_MinProblemSolver_Function_calc_const_const_doubleX(instance: *const c_void, x: *const f64, ocvrs_return: *mut Result<f64>);
// getGradient(const double *, double *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/optim.hpp:70
// ("cv::MinProblemSolver::Function::getGradient", vec![(pred!(mut, ["x", "grad"], ["const double*", "double*"]), _)]),
pub fn cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(instance: *mut c_void, x: *const f64, grad: *mut f64, ocvrs_return: *mut Result<()>);
// cv::MinProblemSolver::Function::delete() generated
// ("cv::MinProblemSolver::Function::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MinProblemSolver_Function_delete(instance: *mut c_void);
// Moments()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:974
// ("cv::Moments::Moments", vec![(pred!(mut, [], []), _)]),
pub fn cv_Moments_Moments(ocvrs_return: *mut Result<core::Moments>);
// Moments(double, double, double, double, double, double, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:976
// ("cv::Moments::Moments", vec![(pred!(mut, ["m00", "m10", "m01", "m20", "m11", "m02", "m30", "m21", "m12", "m03"], ["double", "double", "double", "double", "double", "double", "double", "double", "double", "double"]), _)]),
pub fn cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64, ocvrs_return: *mut Result<core::Moments>);
// PCA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2441
// ("cv::PCA::PCA", vec![(pred!(mut, [], []), _)]),
pub fn cv_PCA_PCA(ocvrs_return: *mut Result<*mut c_void>);
// PCA(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2452
// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(data: *const c_void, mean: *const c_void, flags: i32, max_components: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PCA::PCA(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2452
// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int(data: *const c_void, mean: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// PCA(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2464
// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
pub fn cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(data: *const c_void, mean: *const c_void, flags: i32, retained_variance: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2486
// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_PCA_operator___const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, data: *const c_void, mean: *const c_void, flags: i32, max_components: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::PCA::operator()(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2486
// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_PCA_operator___const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, data: *const c_void, mean: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2499
// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
pub fn cv_PCA_operator___const__InputArrayR_const__InputArrayR_int_double(instance: *mut c_void, data: *const c_void, mean: *const c_void, flags: i32, retained_variance: f64, ocvrs_return: *mut Result<*mut c_void>);
// project(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2515
// ("cv::PCA::project", vec![(pred!(const, ["vec"], ["const cv::_InputArray*"]), _)]),
pub fn cv_PCA_project_const_const__InputArrayR(instance: *const c_void, vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// project(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2529
// ("cv::PCA::project", vec![(pred!(const, ["vec", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, vec: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// backProject(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2544
// ("cv::PCA::backProject", vec![(pred!(const, ["vec"], ["const cv::_InputArray*"]), _)]),
pub fn cv_PCA_backProject_const_const__InputArrayR(instance: *const c_void, vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// backProject(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2553
// ("cv::PCA::backProject", vec![(pred!(const, ["vec", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, vec: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2559
// ("cv::PCA::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_PCA_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2565
// ("cv::PCA::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_PCA_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// cv::PCA::eigenvectors() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2567
// ("cv::PCA::eigenvectors", vec![(pred!(const, [], []), _)]),
pub fn cv_PCA_propEigenvectors_const(instance: *const c_void) -> *mut c_void;
// cv::PCA::setEigenvectors(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2567
// ("cv::PCA::setEigenvectors", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_PCA_propEigenvectors_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::PCA::eigenvalues() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2568
// ("cv::PCA::eigenvalues", vec![(pred!(const, [], []), _)]),
pub fn cv_PCA_propEigenvalues_const(instance: *const c_void) -> *mut c_void;
// cv::PCA::setEigenvalues(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2568
// ("cv::PCA::setEigenvalues", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_PCA_propEigenvalues_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::PCA::mean() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2569
// ("cv::PCA::mean", vec![(pred!(const, [], []), _)]),
pub fn cv_PCA_propMean_const(instance: *const c_void) -> *mut c_void;
// cv::PCA::setMean(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2569
// ("cv::PCA::setMean", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_PCA_propMean_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::PCA::delete() generated
// ("cv::PCA::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_PCA_delete(instance: *mut c_void);
// operator()(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:650
// ("cv::ParallelLoopBody::operator()", vec![(pred!(const, ["range"], ["const cv::Range*"]), _)]),
pub fn cv_ParallelLoopBody_operator___const_const_RangeR(instance: *const c_void, range: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ParallelLoopBody::delete() generated
// ("cv::ParallelLoopBody::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ParallelLoopBody_delete(instance: *mut c_void);
// RNG()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2819
// ("cv::RNG::RNG", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_RNG(ocvrs_return: *mut Result<*mut c_void>);
// RNG(uint64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2823
// ("cv::RNG::RNG", vec![(pred!(mut, ["state"], ["uint64_t"]), _)]),
pub fn cv_RNG_RNG_uint64_t(state: u64, ocvrs_return: *mut Result<*mut c_void>);
// next()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2826
// ("cv::RNG::next", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_next(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
// operator unsigned char()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2834
// ("cv::RNG::operator unsigned char", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_unsigned_char(instance: *mut c_void, ocvrs_return: *mut Result<u8>);
// operator signed char()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2836
// ("cv::RNG::operator signed char", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_signed_char(instance: *mut c_void, ocvrs_return: *mut Result<i8>);
// operator unsigned short()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2838
// ("cv::RNG::operator unsigned short", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_unsigned_short(instance: *mut c_void, ocvrs_return: *mut Result<u16>);
// operator short()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2840
// ("cv::RNG::operator short", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_short(instance: *mut c_void, ocvrs_return: *mut Result<i16>);
// operator unsigned int()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2842
// ("cv::RNG::operator unsigned int", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_unsigned_int(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
// operator int()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2844
// ("cv::RNG::operator int", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_int(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// operator float()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2846
// ("cv::RNG::operator float", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_float(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// operator double()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2848
// ("cv::RNG::operator double", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator_double(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// operator()()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2857
// ("cv::RNG::operator()", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_operator__(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
// operator()(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2861
// ("cv::RNG::operator()", vec![(pred!(mut, ["N"], ["unsigned int"]), _)]),
pub fn cv_RNG_operator___unsigned_int(instance: *mut c_void, n: u32, ocvrs_return: *mut Result<u32>);
// uniform(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2899
// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["int", "int"]), _)]),
pub fn cv_RNG_uniform_int_int(instance: *mut c_void, a: i32, b: i32, ocvrs_return: *mut Result<i32>);
// uniform(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2901
// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["float", "float"]), _)]),
pub fn cv_RNG_uniform_float_float(instance: *mut c_void, a: f32, b: f32, ocvrs_return: *mut Result<f32>);
// uniform(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2903
// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["double", "double"]), _)]),
pub fn cv_RNG_uniform_double_double(instance: *mut c_void, a: f64, b: f64, ocvrs_return: *mut Result<f64>);
// fill(InputOutputArray, int, InputArray, InputArray, bool)(InputOutputArray, Primitive, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2939
// ("cv::RNG::fill", vec![(pred!(mut, ["mat", "distType", "a", "b", "saturateRange"], ["const cv::_InputOutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(instance: *mut c_void, mat: *const c_void, dist_type: i32, a: *const c_void, b: *const c_void, saturate_range: bool, ocvrs_return: *mut Result<()>);
// cv::RNG::fill(InputOutputArray, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2939
// ("cv::RNG::fill", vec![(pred!(mut, ["mat", "distType", "a", "b"], ["const cv::_InputOutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR(instance: *mut c_void, mat: *const c_void, dist_type: i32, a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<()>);
// gaussian(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2949
// ("cv::RNG::gaussian", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
pub fn cv_RNG_gaussian_double(instance: *mut c_void, sigma: f64, ocvrs_return: *mut Result<f64>);
// operator==(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2953
// ("cv::RNG::operator==", vec![(pred!(const, ["other"], ["const cv::RNG*"]), _)]),
pub fn cv_RNG_operatorEQ_const_const_RNGR(instance: *const c_void, other: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::RNG::state() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2951
// ("cv::RNG::state", vec![(pred!(const, [], []), _)]),
pub fn cv_RNG_propState_const(instance: *const c_void) -> u64;
// cv::RNG::setState(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2951
// ("cv::RNG::setState", vec![(pred!(mut, ["val"], ["const uint64_t"]), _)]),
pub fn cv_RNG_propState_const_uint64_t(instance: *mut c_void, val: u64);
// cv::RNG::delete() generated
// ("cv::RNG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_delete(instance: *mut c_void);
// RNG_MT19937()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2964
// ("cv::RNG_MT19937::RNG_MT19937", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_RNG_MT19937(ocvrs_return: *mut Result<*mut c_void>);
// RNG_MT19937(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2965
// ("cv::RNG_MT19937::RNG_MT19937", vec![(pred!(mut, ["s"], ["unsigned int"]), _)]),
pub fn cv_RNG_MT19937_RNG_MT19937_unsigned_int(s: u32, ocvrs_return: *mut Result<*mut c_void>);
// seed(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2966
// ("cv::RNG_MT19937::seed", vec![(pred!(mut, ["s"], ["unsigned int"]), _)]),
pub fn cv_RNG_MT19937_seed_unsigned_int(instance: *mut c_void, s: u32, ocvrs_return: *mut Result<()>);
// next()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2968
// ("cv::RNG_MT19937::next", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_next(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
// operator int()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2970
// ("cv::RNG_MT19937::operator int", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_operator_int(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// operator unsigned int()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2971
// ("cv::RNG_MT19937::operator unsigned int", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_operator_unsigned_int(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
// operator float()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2972
// ("cv::RNG_MT19937::operator float", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_operator_float(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// operator double()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2973
// ("cv::RNG_MT19937::operator double", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_operator_double(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// operator()(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2975
// ("cv::RNG_MT19937::operator()", vec![(pred!(mut, ["N"], ["unsigned int"]), _)]),
pub fn cv_RNG_MT19937_operator___unsigned_int(instance: *mut c_void, n: u32, ocvrs_return: *mut Result<u32>);
// uniform(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2979
// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["int", "int"]), _)]),
pub fn cv_RNG_MT19937_uniform_int_int(instance: *mut c_void, a: i32, b: i32, ocvrs_return: *mut Result<i32>);
// uniform(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2981
// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["float", "float"]), _)]),
pub fn cv_RNG_MT19937_uniform_float_float(instance: *mut c_void, a: f32, b: f32, ocvrs_return: *mut Result<f32>);
// uniform(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2983
// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["double", "double"]), _)]),
pub fn cv_RNG_MT19937_uniform_double_double(instance: *mut c_void, a: f64, b: f64, ocvrs_return: *mut Result<f64>);
// cv::RNG_MT19937::delete() generated
// ("cv::RNG_MT19937::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RNG_MT19937_delete(instance: *mut c_void);
// Range()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:635
// ("cv::Range::Range", vec![(pred!(mut, [], []), _)]),
pub fn cv_Range_Range(ocvrs_return: *mut Result<*mut c_void>);
// Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:636
// ("cv::Range::Range", vec![(pred!(mut, ["_start", "_end"], ["int", "int"]), _)]),
pub fn cv_Range_Range_int_int(_start: i32, _end: i32, ocvrs_return: *mut Result<*mut c_void>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:637
// ("cv::Range::size", vec![(pred!(const, [], []), _)]),
pub fn cv_Range_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:638
// ("cv::Range::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_Range_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// all()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:639
// ("cv::Range::all", vec![(pred!(mut, [], []), _)]),
pub fn cv_Range_all(ocvrs_return: *mut Result<*mut c_void>);
// cv::Range::start() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:641
// ("cv::Range::start", vec![(pred!(const, [], []), _)]),
pub fn cv_Range_propStart_const(instance: *const c_void) -> i32;
// cv::Range::setStart(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:641
// ("cv::Range::setStart", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Range_propStart_const_int(instance: *mut c_void, val: i32);
// cv::Range::end() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:641
// ("cv::Range::end", vec![(pred!(const, [], []), _)]),
pub fn cv_Range_propEnd_const(instance: *const c_void) -> i32;
// cv::Range::setEnd(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:641
// ("cv::Range::setEnd", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Range_propEnd_const_int(instance: *mut c_void, val: i32);
// cv::Range::delete() generated
// ("cv::Range::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Range_delete(instance: *mut c_void);
// RotatedRect()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:544
// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, [], []), _)]),
pub fn cv_RotatedRect_RotatedRect(ocvrs_return: *mut Result<core::RotatedRect>);
// RotatedRect(const Point2f &, const Size2f &, float)(SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:551
// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, ["center", "size", "angle"], ["const cv::Point2f*", "const cv::Size2f*", "float"]), _)]),
pub fn cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(center: *const core::Point2f, size: *const core::Size2f, angle: f32, ocvrs_return: *mut Result<core::RotatedRect>);
// RotatedRect(const Point2f &, const Point2f &, const Point2f &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:556
// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, ["point1", "point2", "point3"], ["const cv::Point2f*", "const cv::Point2f*", "const cv::Point2f*"]), _)]),
pub fn cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(point1: *const core::Point2f, point2: *const core::Point2f, point3: *const core::Point2f, ocvrs_return: *mut Result<core::RotatedRect>);
// points(Point2f *)(FixedArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:564
// ("cv::RotatedRect::points", vec![(pred!(const, ["pts"], ["cv::Point2f*"]), _)]),
pub fn cv_RotatedRect_points_const_Point2fXX(instance: *const core::RotatedRect, pts: *mut [core::Point2f; 4], ocvrs_return: *mut Result<()>);
// points(std::vector<Point2f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:566
// ("cv::RotatedRect::points", vec![(pred!(const, ["pts"], ["std::vector<cv::Point2f>*"]), _)]),
pub fn cv_RotatedRect_points_const_vectorLPoint2fGR(instance: *const core::RotatedRect, pts: *mut c_void, ocvrs_return: *mut Result<()>);
// boundingRect()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:569
// ("cv::RotatedRect::boundingRect", vec![(pred!(const, [], []), _)]),
pub fn cv_RotatedRect_boundingRect_const(instance: *const core::RotatedRect, ocvrs_return: *mut Result<core::Rect>);
// boundingRect2f()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:571
// ("cv::RotatedRect::boundingRect2f", vec![(pred!(const, [], []), _)]),
pub fn cv_RotatedRect_boundingRect2f_const(instance: *const core::RotatedRect, ocvrs_return: *mut Result<core::Rect2f>);
// SVD()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2685
// ("cv::SVD::SVD", vec![(pred!(mut, [], []), _)]),
pub fn cv_SVD_SVD(ocvrs_return: *mut Result<*mut c_void>);
// SVD(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2692
// ("cv::SVD::SVD", vec![(pred!(mut, ["src", "flags"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_SVD_SVD_const__InputArrayR_int(src: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::SVD::SVD(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2692
// ("cv::SVD::SVD", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_SVD_SVD_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2705
// ("cv::SVD::operator()", vec![(pred!(mut, ["src", "flags"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_SVD_operator___const__InputArrayR_int(instance: *mut c_void, src: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::SVD::operator()(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2705
// ("cv::SVD::operator()", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_SVD_operator___const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, OutputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2724
// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "u", "vt", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// cv::SVD::compute(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2724
// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "u", "vt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2733
// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// cv::SVD::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2733
// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR(src: *const c_void, w: *const c_void, ocvrs_return: *mut Result<()>);
// backSubst(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2737
// ("cv::SVD::backSubst", vec![(pred!(mut, ["w", "u", "vt", "rhs", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w: *const c_void, u: *const c_void, vt: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// solveZ(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2751
// ("cv::SVD::solveZ", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// backSubst(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2775
// ("cv::SVD::backSubst", vec![(pred!(const, ["rhs", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::SVD::u() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::u", vec![(pred!(const, [], []), _)]),
pub fn cv_SVD_propU_const(instance: *const c_void) -> *mut c_void;
// cv::SVD::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::setU", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_SVD_propU_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::SVD::w() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::w", vec![(pred!(const, [], []), _)]),
pub fn cv_SVD_propW_const(instance: *const c_void) -> *mut c_void;
// cv::SVD::setW(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::setW", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_SVD_propW_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::SVD::vt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::vt", vec![(pred!(const, [], []), _)]),
pub fn cv_SVD_propVt_const(instance: *const c_void) -> *mut c_void;
// cv::SVD::setVt(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core.hpp:2789
// ("cv::SVD::setVt", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_SVD_propVt_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::SVD::delete() generated
// ("cv::SVD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SVD_delete(instance: *mut c_void);
// SparseMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3067
// ("cv::SparseMat::SparseMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_SparseMat(ocvrs_return: *mut Result<*mut c_void>);
// SparseMat(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3074
// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
pub fn cv_SparseMat_SparseMat_int_const_intX_int(dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result<*mut c_void>);
// SparseMat(const SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3080
// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["m"], ["const cv::SparseMat*"]), _)]),
pub fn cv_SparseMat_SparseMat_const_SparseMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// SparseMat(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3086
// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_SparseMat_SparseMat_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3092
// ("cv::SparseMat::operator=", vec![(pred!(mut, ["m"], ["const cv::SparseMat*"]), _)]),
pub fn cv_SparseMat_operatorST_const_SparseMatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// operator=(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3094
// ("cv::SparseMat::operator=", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_SparseMat_operatorST_const_MatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3097
// ("cv::SparseMat::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copyTo(SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3100
// ("cv::SparseMat::copyTo", vec![(pred!(const, ["m"], ["cv::SparseMat*"]), _)]),
pub fn cv_SparseMat_copyTo_const_SparseMatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3102
// ("cv::SparseMat::copyTo", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
pub fn cv_SparseMat_copyTo_const_MatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTo(SparseMat &, int, double)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3104
// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha"], ["cv::SparseMat*", "int", "double"]), _)]),
pub fn cv_SparseMat_convertTo_const_SparseMatR_int_double(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, ocvrs_return: *mut Result<()>);
// cv::SparseMat::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3104
// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["cv::SparseMat*", "int"]), _)]),
pub fn cv_SparseMat_convertTo_const_SparseMatR_int(instance: *const c_void, m: *mut c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// convertTo(Mat &, int, double, double)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3115
// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["cv::Mat*", "int", "double", "double"]), _)]),
pub fn cv_SparseMat_convertTo_const_MatR_int_double_double(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::SparseMat::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3115
// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["cv::Mat*", "int"]), _)]),
pub fn cv_SparseMat_convertTo_const_MatR_int(instance: *const c_void, m: *mut c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// assignTo(SparseMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3118
// ("cv::SparseMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::SparseMat*", "int"]), _)]),
pub fn cv_SparseMat_assignTo_const_SparseMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::SparseMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3118
// ("cv::SparseMat::assignTo", vec![(pred!(const, ["m"], ["cv::SparseMat*"]), _)]),
pub fn cv_SparseMat_assignTo_const_SparseMatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// create(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3126
// ("cv::SparseMat::create", vec![(pred!(mut, ["dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
pub fn cv_SparseMat_create_int_const_intX_int(instance: *mut c_void, dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result<()>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3128
// ("cv::SparseMat::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addref()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3130
// ("cv::SparseMat::addref", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_addref(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3132
// ("cv::SparseMat::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3137
// ("cv::SparseMat::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_elemSize_const(instance: *const c_void) -> size_t;
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3139
// ("cv::SparseMat::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_elemSize1_const(instance: *const c_void) -> size_t;
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3142
// ("cv::SparseMat::type", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_type_const(instance: *const c_void) -> i32;
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3144
// ("cv::SparseMat::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_depth_const(instance: *const c_void) -> i32;
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3146
// ("cv::SparseMat::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_channels_const(instance: *const c_void) -> i32;
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3149
// ("cv::SparseMat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_size_const(instance: *const c_void, ocvrs_return: *mut Result<*const i32>);
// size(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3151
// ("cv::SparseMat::size", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_SparseMat_size_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// dims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3153
// ("cv::SparseMat::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_dims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nzcount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3155
// ("cv::SparseMat::nzcount", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_nzcount_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// hash(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3158
// ("cv::SparseMat::hash", vec![(pred!(const, ["i0"], ["int"]), _)]),
pub fn cv_SparseMat_hash_const_int(instance: *const c_void, i0: i32, ocvrs_return: *mut Result<size_t>);
// hash(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3160
// ("cv::SparseMat::hash", vec![(pred!(const, ["i0", "i1"], ["int", "int"]), _)]),
pub fn cv_SparseMat_hash_const_int_int(instance: *const c_void, i0: i32, i1: i32, ocvrs_return: *mut Result<size_t>);
// hash(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3162
// ("cv::SparseMat::hash", vec![(pred!(const, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
pub fn cv_SparseMat_hash_const_int_int_int(instance: *const c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<size_t>);
// hash(const int *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3164
// ("cv::SparseMat::hash", vec![(pred!(const, ["idx"], ["const int*"]), _)]),
pub fn cv_SparseMat_hash_const_const_intX(instance: *const c_void, idx: *const i32, ocvrs_return: *mut Result<size_t>);
// ptr(int, bool, size_t *)(Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3178
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "createMissing", "hashval"], ["int", "bool", "size_t*"]), _)]),
pub fn cv_SparseMat_ptr_int_bool_size_tX(instance: *mut c_void, i0: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
// cv::SparseMat::ptr(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3178
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "createMissing"], ["int", "bool"]), _)]),
pub fn cv_SparseMat_ptr_int_bool(instance: *mut c_void, i0: i32, create_missing: bool, ocvrs_return: *mut Result<*mut u8>);
// ptr(int, int, bool, size_t *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3180
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "createMissing", "hashval"], ["int", "int", "bool", "size_t*"]), _)]),
pub fn cv_SparseMat_ptr_int_int_bool_size_tX(instance: *mut c_void, i0: i32, i1: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
// cv::SparseMat::ptr(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3180
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "createMissing"], ["int", "int", "bool"]), _)]),
pub fn cv_SparseMat_ptr_int_int_bool(instance: *mut c_void, i0: i32, i1: i32, create_missing: bool, ocvrs_return: *mut Result<*mut u8>);
// ptr(int, int, int, bool, size_t *)(Primitive, Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3182
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "i2", "createMissing", "hashval"], ["int", "int", "int", "bool", "size_t*"]), _)]),
pub fn cv_SparseMat_ptr_int_int_int_bool_size_tX(instance: *mut c_void, i0: i32, i1: i32, i2: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
// cv::SparseMat::ptr(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3182
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "i2", "createMissing"], ["int", "int", "int", "bool"]), _)]),
pub fn cv_SparseMat_ptr_int_int_int_bool(instance: *mut c_void, i0: i32, i1: i32, i2: i32, create_missing: bool, ocvrs_return: *mut Result<*mut u8>);
// ptr(const int *, bool, size_t *)(Indirect, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3184
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["idx", "createMissing", "hashval"], ["const int*", "bool", "size_t*"]), _)]),
pub fn cv_SparseMat_ptr_const_intX_bool_size_tX(instance: *mut c_void, idx: *const i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
// cv::SparseMat::ptr(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3184
// ("cv::SparseMat::ptr", vec![(pred!(mut, ["idx", "createMissing"], ["const int*", "bool"]), _)]),
pub fn cv_SparseMat_ptr_const_intX_bool(instance: *mut c_void, idx: *const i32, create_missing: bool, ocvrs_return: *mut Result<*mut u8>);
// erase(int, int, size_t *)(Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3245
// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "hashval"], ["int", "int", "size_t*"]), _)]),
pub fn cv_SparseMat_erase_int_int_size_tX(instance: *mut c_void, i0: i32, i1: i32, hashval: *mut size_t, ocvrs_return: *mut Result<()>);
// cv::SparseMat::erase(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3245
// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1"], ["int", "int"]), _)]),
pub fn cv_SparseMat_erase_int_int(instance: *mut c_void, i0: i32, i1: i32, ocvrs_return: *mut Result<()>);
// erase(int, int, int, size_t *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3247
// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "i2", "hashval"], ["int", "int", "int", "size_t*"]), _)]),
pub fn cv_SparseMat_erase_int_int_int_size_tX(instance: *mut c_void, i0: i32, i1: i32, i2: i32, hashval: *mut size_t, ocvrs_return: *mut Result<()>);
// cv::SparseMat::erase(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3247
// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
pub fn cv_SparseMat_erase_int_int_int(instance: *mut c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<()>);
// erase(const int *, size_t *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3249
// ("cv::SparseMat::erase", vec![(pred!(mut, ["idx", "hashval"], ["const int*", "size_t*"]), _)]),
pub fn cv_SparseMat_erase_const_intX_size_tX(instance: *mut c_void, idx: *const i32, hashval: *mut size_t, ocvrs_return: *mut Result<()>);
// cv::SparseMat::erase(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3249
// ("cv::SparseMat::erase", vec![(pred!(mut, ["idx"], ["const int*"]), _)]),
pub fn cv_SparseMat_erase_const_intX(instance: *mut c_void, idx: *const i32, ocvrs_return: *mut Result<()>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3256
// ("cv::SparseMat::begin", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_begin(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3260
// ("cv::SparseMat::begin", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_begin_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3268
// ("cv::SparseMat::end", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_end(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3270
// ("cv::SparseMat::end", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_end_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// node(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3282
// ("cv::SparseMat::node", vec![(pred!(mut, ["nidx"], ["size_t"]), _)]),
pub fn cv_SparseMat_node_size_t(instance: *mut c_void, nidx: size_t, ocvrs_return: *mut Result<*mut c_void>);
// node(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3283
// ("cv::SparseMat::node", vec![(pred!(const, ["nidx"], ["size_t"]), _)]),
pub fn cv_SparseMat_node_const_size_t(instance: *const c_void, nidx: size_t, ocvrs_return: *mut Result<*mut c_void>);
// newNode(const int *, size_t)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3285
// ("cv::SparseMat::newNode", vec![(pred!(mut, ["idx", "hashval"], ["const int*", "size_t"]), _)]),
pub fn cv_SparseMat_newNode_const_intX_size_t(instance: *mut c_void, idx: *const i32, hashval: size_t, ocvrs_return: *mut Result<*mut u8>);
// removeNode(size_t, size_t, size_t)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3286
// ("cv::SparseMat::removeNode", vec![(pred!(mut, ["hidx", "nidx", "previdx"], ["size_t", "size_t", "size_t"]), _)]),
pub fn cv_SparseMat_removeNode_size_t_size_t_size_t(instance: *mut c_void, hidx: size_t, nidx: size_t, previdx: size_t, ocvrs_return: *mut Result<()>);
// resizeHashTab(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3287
// ("cv::SparseMat::resizeHashTab", vec![(pred!(mut, ["newsize"], ["size_t"]), _)]),
pub fn cv_SparseMat_resizeHashTab_size_t(instance: *mut c_void, newsize: size_t, ocvrs_return: *mut Result<()>);
// cv::SparseMat::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3289
// ("cv::SparseMat::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_propFlags_const(instance: *const c_void) -> i32;
// cv::SparseMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3289
// ("cv::SparseMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_SparseMat_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::SparseMat::hdr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3290
// ("cv::SparseMat::hdr", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_propHdr(instance: *mut c_void) -> *mut c_void;
// cv::SparseMat::setHdr(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3290
// ("cv::SparseMat::setHdr", vec![(pred!(mut, ["val"], ["cv::SparseMat::Hdr*"]), _)]),
pub fn cv_SparseMat_propHdr_HdrX(instance: *mut c_void, val: *const c_void);
// cv::SparseMat::delete() generated
// ("cv::SparseMat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_delete(instance: *mut c_void);
// Hdr(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3041
// ("cv::SparseMat::Hdr::Hdr", vec![(pred!(mut, ["_dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
pub fn cv_SparseMat_Hdr_Hdr_int_const_intX_int(_dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result<*mut c_void>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3042
// ("cv::SparseMat::Hdr::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_Hdr_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::SparseMat::Hdr::refcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3043
// ("cv::SparseMat::Hdr::refcount", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propRefcount_const(instance: *const c_void) -> i32;
// cv::SparseMat::Hdr::setRefcount(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3043
// ("cv::SparseMat::Hdr::setRefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_SparseMat_Hdr_propRefcount_const_int(instance: *mut c_void, val: i32);
// cv::SparseMat::Hdr::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3044
// ("cv::SparseMat::Hdr::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propDims_const(instance: *const c_void) -> i32;
// cv::SparseMat::Hdr::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3044
// ("cv::SparseMat::Hdr::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_SparseMat_Hdr_propDims_const_int(instance: *mut c_void, val: i32);
// cv::SparseMat::Hdr::valueOffset() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3045
// ("cv::SparseMat::Hdr::valueOffset", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propValueOffset_const(instance: *const c_void) -> i32;
// cv::SparseMat::Hdr::setValueOffset(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3045
// ("cv::SparseMat::Hdr::setValueOffset", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_SparseMat_Hdr_propValueOffset_const_int(instance: *mut c_void, val: i32);
// cv::SparseMat::Hdr::nodeSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3046
// ("cv::SparseMat::Hdr::nodeSize", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propNodeSize_const(instance: *const c_void) -> size_t;
// cv::SparseMat::Hdr::setNodeSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3046
// ("cv::SparseMat::Hdr::setNodeSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMat_Hdr_propNodeSize_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMat::Hdr::nodeCount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3047
// ("cv::SparseMat::Hdr::nodeCount", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propNodeCount_const(instance: *const c_void) -> size_t;
// cv::SparseMat::Hdr::setNodeCount(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3047
// ("cv::SparseMat::Hdr::setNodeCount", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMat_Hdr_propNodeCount_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMat::Hdr::freeList() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3048
// ("cv::SparseMat::Hdr::freeList", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propFreeList_const(instance: *const c_void) -> size_t;
// cv::SparseMat::Hdr::setFreeList(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3048
// ("cv::SparseMat::Hdr::setFreeList", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMat_Hdr_propFreeList_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMat::Hdr::pool() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3049
// ("cv::SparseMat::Hdr::pool", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propPool_const(instance: *const c_void) -> *mut c_void;
// cv::SparseMat::Hdr::setPool(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3049
// ("cv::SparseMat::Hdr::setPool", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
pub fn cv_SparseMat_Hdr_propPool_const_vectorLunsigned_charG(instance: *mut c_void, val: *const c_void);
// cv::SparseMat::Hdr::hashtab() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3050
// ("cv::SparseMat::Hdr::hashtab", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propHashtab_const(instance: *const c_void) -> *mut c_void;
// cv::SparseMat::Hdr::setHashtab(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3050
// ("cv::SparseMat::Hdr::setHashtab", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_SparseMat_Hdr_propHashtab_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::SparseMat::Hdr::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3051
// ("cv::SparseMat::Hdr::size", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Hdr_propSize_const(instance: *const c_void) -> *const [i32; 32];
// cv::SparseMat::Hdr::sizeMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3051
// ("cv::SparseMat::Hdr::sizeMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_Hdr_propSize(instance: *mut c_void) -> *mut [i32; 32];
// cv::SparseMat::Hdr::delete() generated
// ("cv::SparseMat::Hdr::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_Hdr_delete(instance: *mut c_void);
// cv::SparseMat::Node::defaultNew() generated
// ("cv::SparseMat::Node::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Node_defaultNew_const() -> *mut c_void;
// cv::SparseMat::Node::hashval() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3058
// ("cv::SparseMat::Node::hashval", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Node_propHashval_const(instance: *const c_void) -> size_t;
// cv::SparseMat::Node::setHashval(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3058
// ("cv::SparseMat::Node::setHashval", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMat_Node_propHashval_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMat::Node::next() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3060
// ("cv::SparseMat::Node::next", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Node_propNext_const(instance: *const c_void) -> size_t;
// cv::SparseMat::Node::setNext(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3060
// ("cv::SparseMat::Node::setNext", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMat_Node_propNext_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMat::Node::idx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3062
// ("cv::SparseMat::Node::idx", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMat_Node_propIdx_const(instance: *const c_void) -> *const [i32; 32];
// cv::SparseMat::Node::idxMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3062
// ("cv::SparseMat::Node::idxMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_Node_propIdx(instance: *mut c_void) -> *mut [i32; 32];
// cv::SparseMat::Node::delete() generated
// ("cv::SparseMat::Node::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMat_Node_delete(instance: *mut c_void);
// SparseMatConstIterator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3558
// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatConstIterator_SparseMatConstIterator(ocvrs_return: *mut Result<*mut c_void>);
// SparseMatConstIterator(const SparseMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3560
// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, ["_m"], ["const cv::SparseMat*"]), _)]),
pub fn cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(_m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// SparseMatConstIterator(const SparseMatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3562
// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, ["it"], ["const cv::SparseMatConstIterator*"]), _)]),
pub fn cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const SparseMatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3565
// ("cv::SparseMatConstIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::SparseMatConstIterator*"]), _)]),
pub fn cv_SparseMatConstIterator_operatorST_const_SparseMatConstIteratorR(instance: *mut c_void, it: *const c_void, ocvrs_return: *mut Result<()>);
// node()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3570
// ("cv::SparseMatConstIterator::node", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMatConstIterator_node_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3577
// ("cv::SparseMatConstIterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatConstIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// seekEnd()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3582
// ("cv::SparseMatConstIterator::seekEnd", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatConstIterator_seekEnd(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::SparseMatConstIterator::m() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3584
// ("cv::SparseMatConstIterator::m", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMatConstIterator_propM_const(instance: *const c_void) -> *mut c_void;
// cv::SparseMatConstIterator::hashidx() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3585
// ("cv::SparseMatConstIterator::hashidx", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMatConstIterator_propHashidx_const(instance: *const c_void) -> size_t;
// cv::SparseMatConstIterator::setHashidx(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3585
// ("cv::SparseMatConstIterator::setHashidx", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_SparseMatConstIterator_propHashidx_const_size_t(instance: *mut c_void, val: size_t);
// cv::SparseMatConstIterator::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3586
// ("cv::SparseMatConstIterator::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMatConstIterator_propPtr_const(instance: *const c_void) -> *const u8;
// cv::SparseMatConstIterator::ptrMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3586
// ("cv::SparseMatConstIterator::ptrMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatConstIterator_propPtr(instance: *mut c_void) -> *mut u8;
// cv::SparseMatConstIterator::setPtr(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3586
// ("cv::SparseMatConstIterator::setPtr", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_SparseMatConstIterator_propPtr_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::SparseMatConstIterator::delete() generated
// ("cv::SparseMatConstIterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatConstIterator_delete(instance: *mut c_void);
// SparseMatIterator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3602
// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatIterator_SparseMatIterator(ocvrs_return: *mut Result<*mut c_void>);
// SparseMatIterator(SparseMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3604
// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, ["_m"], ["cv::SparseMat*"]), _)]),
pub fn cv_SparseMatIterator_SparseMatIterator_SparseMatX(_m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// SparseMatIterator(const SparseMatIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3608
// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, ["it"], ["const cv::SparseMatIterator*"]), _)]),
pub fn cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const SparseMatIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3611
// ("cv::SparseMatIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::SparseMatIterator*"]), _)]),
pub fn cv_SparseMatIterator_operatorST_const_SparseMatIteratorR(instance: *mut c_void, it: *const c_void, ocvrs_return: *mut Result<()>);
// node()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3615
// ("cv::SparseMatIterator::node", vec![(pred!(const, [], []), _)]),
pub fn cv_SparseMatIterator_node_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:3618
// ("cv::SparseMatIterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::SparseMatIterator::to_SparseMatConstIterator() generated
// ("cv::SparseMatIterator::to_SparseMatConstIterator", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatIterator_to_SparseMatConstIterator(instance: *mut c_void) -> *mut c_void;
// cv::SparseMatIterator::delete() generated
// ("cv::SparseMatIterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_SparseMatIterator_delete(instance: *mut c_void);
// TermCriteria()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:909
// ("cv::TermCriteria::TermCriteria", vec![(pred!(mut, [], []), _)]),
pub fn cv_TermCriteria_TermCriteria(ocvrs_return: *mut Result<core::TermCriteria>);
// TermCriteria(int, int, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:915
// ("cv::TermCriteria::TermCriteria", vec![(pred!(mut, ["type", "maxCount", "epsilon"], ["int", "int", "double"]), _)]),
pub fn cv_TermCriteria_TermCriteria_int_int_double(typ: i32, max_count: i32, epsilon: f64, ocvrs_return: *mut Result<core::TermCriteria>);
// TermCriteria(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:916
// ("cv::TermCriteria::TermCriteria", vec![(pred!(mut, ["maxCount", "epsilon"], ["int", "double"]), _)]),
pub fn cv_TermCriteria_TermCriteria_int_double(max_count: i32, epsilon: f64, ocvrs_return: *mut Result<core::TermCriteria>);
// isValid()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/types.hpp:918
// ("cv::TermCriteria::isValid", vec![(pred!(const, [], []), _)]),
pub fn cv_TermCriteria_isValid_const(instance: *const core::TermCriteria, ocvrs_return: *mut Result<bool>);
// TickMeter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:329
// ("cv::TickMeter::TickMeter", vec![(pred!(mut, [], []), _)]),
pub fn cv_TickMeter_TickMeter(ocvrs_return: *mut Result<*mut c_void>);
// start()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:335
// ("cv::TickMeter::start", vec![(pred!(mut, [], []), _)]),
pub fn cv_TickMeter_start(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:341
// ("cv::TickMeter::stop", vec![(pred!(mut, [], []), _)]),
pub fn cv_TickMeter_stop(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getTimeTicks()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:353
// ("cv::TickMeter::getTimeTicks", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getTimeTicks_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
// getTimeMicro()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:359
// ("cv::TickMeter::getTimeMicro", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getTimeMicro_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getTimeMilli()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:365
// ("cv::TickMeter::getTimeMilli", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getTimeMilli_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getTimeSec()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:371
// ("cv::TickMeter::getTimeSec", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getTimeSec_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getLastTimeTicks()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:377
// ("cv::TickMeter::getLastTimeTicks", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getLastTimeTicks_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
// getLastTimeMicro()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:383
// ("cv::TickMeter::getLastTimeMicro", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getLastTimeMicro_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getLastTimeMilli()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:389
// ("cv::TickMeter::getLastTimeMilli", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getLastTimeMilli_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getLastTimeSec()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:395
// ("cv::TickMeter::getLastTimeSec", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getLastTimeSec_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getCounter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:401
// ("cv::TickMeter::getCounter", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getCounter_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
// getFPS()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:407
// ("cv::TickMeter::getFPS", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getFPS_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getAvgTimeSec()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:416
// ("cv::TickMeter::getAvgTimeSec", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getAvgTimeSec_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getAvgTimeMilli()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:424
// ("cv::TickMeter::getAvgTimeMilli", vec![(pred!(const, [], []), _)]),
pub fn cv_TickMeter_getAvgTimeMilli_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utility.hpp:430
// ("cv::TickMeter::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_TickMeter_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TickMeter::delete() generated
// ("cv::TickMeter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TickMeter_delete(instance: *mut c_void);
// UMat(UMatUsageFlags)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2726
// ("cv::UMat::UMat", vec![(pred!(mut, ["usageFlags"], ["cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_UMatUsageFlags(usage_flags: core::UMatUsageFlags) -> *mut c_void;
// cv::UMat::UMat() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2726
// ("cv::UMat::UMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_UMat() -> *mut c_void;
// UMat(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2729
// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2729
// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_UMat_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// UMat(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2730
// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2730
// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_UMat_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// UMat(int, int, int, const Scalar &, UMatUsageFlags)(Primitive, Primitive, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2732
// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "s", "usageFlags"], ["int", "int", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2732
// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_UMat_UMat_int_int_int_const_ScalarR(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// UMat(Size, int, const Scalar &, UMatUsageFlags)(SimpleClass, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2733
// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "s", "usageFlags"], ["cv::Size", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(size: *const core::Size, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2733
// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "const cv::Scalar*"]), _)]),
pub fn cv_UMat_UMat_Size_int_const_ScalarR(size: *const core::Size, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// UMat(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2736
// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(ndims: i32, sizes: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2736
// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_UMat_UMat_int_const_intX_int(ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// UMat(int, const int *, int, const Scalar &, UMatUsageFlags)(Primitive, VariableArray, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2737
// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "s", "usageFlags"], ["int", "const int*", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(ndims: i32, sizes: *const i32, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(Primitive, VariableArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2737
// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "s"], ["int", "const int*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_UMat_UMat_int_const_intX_int_const_ScalarR(ndims: i32, sizes: *const i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// UMat(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2740
// ("cv::UMat::UMat", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_UMat_UMat_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// UMat(const UMat &, const Range &, const Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2743
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::UMat*", "const cv::Range*", "const cv::Range*"]), _)]),
pub fn cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(m: *const c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2743
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange"], ["cv::UMat*", "const cv::Range*"]), _)]),
pub fn cv_UMat_UMat_UMatR_const_RangeR(m: *mut c_void, row_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2743
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange"], ["const cv::UMat*", "const cv::Range*"]), _)]),
pub fn cv_UMat_UMat_const_UMatR_const_RangeR(m: *const c_void, row_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2743
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::UMat*", "const cv::Range*", "const cv::Range*"]), _)]),
pub fn cv_UMat_UMat_UMatR_const_RangeR_const_RangeR(m: *mut c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// UMat(const UMat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2744
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "roi"], ["const cv::UMat*", "const cv::Rect*"]), _)]),
pub fn cv_UMat_UMat_const_UMatR_const_RectR(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2744
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "roi"], ["cv::UMat*", "const cv::Rect*"]), _)]),
pub fn cv_UMat_UMat_UMatR_const_RectR(m: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// UMat(const UMat &, const std::vector<Range> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2746
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "ranges"], ["const cv::UMat*", "const std::vector<cv::Range>*"]), _)]),
pub fn cv_UMat_UMat_const_UMatR_const_vectorLRangeGR(m: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::UMat(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2746
// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "ranges"], ["cv::UMat*", "const std::vector<cv::Range>*"]), _)]),
pub fn cv_UMat_UMat_UMatR_const_vectorLRangeGR(m: *mut c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2751
// ("cv::UMat::operator=", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_UMat_operatorST_const_UMatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// getMat(AccessFlag)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2753
// ("cv::UMat::getMat", vec![(pred!(const, ["flags"], ["cv::AccessFlag"]), _)]),
pub fn cv_UMat_getMat_const_AccessFlag(instance: *const c_void, flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2756
// ("cv::UMat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
pub fn cv_UMat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::row(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2756
// ("cv::UMat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
pub fn cv_UMat_row_int(instance: *mut c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2758
// ("cv::UMat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
pub fn cv_UMat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::col(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2758
// ("cv::UMat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
pub fn cv_UMat_col_int(instance: *mut c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2760
// ("cv::UMat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_UMat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2760
// ("cv::UMat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_UMat_rowRange_int_int(instance: *mut c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2761
// ("cv::UMat::rowRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_UMat_rowRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2761
// ("cv::UMat::rowRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_UMat_rowRange_const_RangeR(instance: *mut c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2763
// ("cv::UMat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_UMat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2763
// ("cv::UMat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_UMat_colRange_int_int(instance: *mut c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// colRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2764
// ("cv::UMat::colRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_UMat_colRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2764
// ("cv::UMat::colRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_UMat_colRange_const_RangeR(instance: *mut c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2769
// ("cv::UMat::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
pub fn cv_UMat_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::diag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2769
// ("cv::UMat::diag", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_diag(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::diag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2769
// ("cv::UMat::diag", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_diag_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::diag(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2769
// ("cv::UMat::diag", vec![(pred!(mut, ["d"], ["int"]), _)]),
pub fn cv_UMat_diag_int(instance: *mut c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
// diag(const UMat &, UMatUsageFlags)(TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2771
// ("cv::UMat::diag", vec![(pred!(mut, ["d", "usageFlags"], ["const cv::UMat*", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_diag_const_UMatR_UMatUsageFlags(d: *const c_void, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::diag(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2771
// ("cv::UMat::diag", vec![(pred!(mut, ["d"], ["const cv::UMat*"]), _)]),
pub fn cv_UMat_diag_const_UMatR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2774
// ("cv::UMat::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2777
// ("cv::UMat::copyTo", vec![(pred!(const, ["m"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_UMat_copyTo_const_const__OutputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2779
// ("cv::UMat::copyTo", vec![(pred!(const, ["m", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, m: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2781
// ("cv::UMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, m: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::UMat::convertTo(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2781
// ("cv::UMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_UMat_convertTo_const_const__OutputArrayR_int(instance: *const c_void, m: *const c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// assignTo(UMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2783
// ("cv::UMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::UMat*", "int"]), _)]),
pub fn cv_UMat_assignTo_const_UMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::UMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2783
// ("cv::UMat::assignTo", vec![(pred!(const, ["m"], ["cv::UMat*"]), _)]),
pub fn cv_UMat_assignTo_const_UMatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// operator=(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2786
// ("cv::UMat::operator=", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
pub fn cv_UMat_operatorST_const_ScalarR(instance: *mut c_void, s: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setTo(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2788
// ("cv::UMat::setTo", vec![(pred!(mut, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_UMat_setTo_const__InputArrayR_const__InputArrayR(instance: *mut c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2788
// ("cv::UMat::setTo", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
pub fn cv_UMat_setTo_const__InputArrayR(instance: *mut c_void, value: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2791
// ("cv::UMat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_UMat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2791
// ("cv::UMat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
pub fn cv_UMat_reshape_int(instance: *mut c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2791
// ("cv::UMat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
pub fn cv_UMat_reshape_const_int(instance: *const c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2791
// ("cv::UMat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_UMat_reshape_int_int(instance: *mut c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, int, const int *)(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2792
// ("cv::UMat::reshape", vec![(pred!(const, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
pub fn cv_UMat_reshape_const_int_int_const_intX(instance: *const c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::reshape(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2792
// ("cv::UMat::reshape", vec![(pred!(mut, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
pub fn cv_UMat_reshape_int_int_const_intX(instance: *mut c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, const MatShape &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2793
// ("cv::UMat::reshape", vec![(pred!(const, ["cn", "shape"], ["int", "const cv::MatShape*"]), _)]),
pub fn cv_UMat_reshape_const_int_const_MatShapeR(instance: *const c_void, cn: i32, shape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// t()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2796
// ("cv::UMat::t", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2798
// ("cv::UMat::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
pub fn cv_UMat_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::inv() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2798
// ("cv::UMat::inv", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_inv_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mul(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2800
// ("cv::UMat::mul", vec![(pred!(const, ["m", "scale"], ["const cv::_InputArray*", "double"]), _)]),
pub fn cv_UMat_mul_const_const__InputArrayR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::mul(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2800
// ("cv::UMat::mul", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
pub fn cv_UMat_mul_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dot(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2803
// ("cv::UMat::dot", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
pub fn cv_UMat_dot_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
// zeros(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2806
// ("cv::UMat::zeros", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_zeros_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::zeros(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2806
// ("cv::UMat::zeros", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_zeros_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// zeros(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2807
// ("cv::UMat::zeros", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_zeros_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::zeros(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2807
// ("cv::UMat::zeros", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_zeros_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// zeros(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2808
// ("cv::UMat::zeros", vec![(pred!(mut, ["ndims", "sz", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_zeros_int_const_intX_int_UMatUsageFlags(ndims: i32, sz: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::zeros(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2808
// ("cv::UMat::zeros", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_UMat_zeros_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2809
// ("cv::UMat::ones", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_ones_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::ones(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2809
// ("cv::UMat::ones", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_ones_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2810
// ("cv::UMat::ones", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_ones_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::ones(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2810
// ("cv::UMat::ones", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_ones_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// ones(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2811
// ("cv::UMat::ones", vec![(pred!(mut, ["ndims", "sz", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_ones_int_const_intX_int_UMatUsageFlags(ndims: i32, sz: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::ones(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2811
// ("cv::UMat::ones", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_UMat_ones_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// eye(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2812
// ("cv::UMat::eye", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_eye_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::eye(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2812
// ("cv::UMat::eye", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_eye_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// eye(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2813
// ("cv::UMat::eye", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_eye_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::eye(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2813
// ("cv::UMat::eye", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_eye_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2817
// ("cv::UMat::create", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_create_int_int_int_UMatUsageFlags(instance: *mut c_void, rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2817
// ("cv::UMat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2818
// ("cv::UMat::create", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_create_Size_int_UMatUsageFlags(instance: *mut c_void, size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2818
// ("cv::UMat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// create(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2819
// ("cv::UMat::create", vec![(pred!(mut, ["ndims", "sizes", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_create_int_const_intX_int_UMatUsageFlags(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::create(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2819
// ("cv::UMat::create", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_UMat_create_int_const_intX_int(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(const std::vector<int> &, int, UMatUsageFlags)(CppPassByVoidPtr, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2820
// ("cv::UMat::create", vec![(pred!(mut, ["sizes", "type", "usageFlags"], ["const std::vector<int>*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_create_const_vectorLintGR_int_UMatUsageFlags(instance: *mut c_void, sizes: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::create(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2820
// ("cv::UMat::create", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
pub fn cv_UMat_create_const_vectorLintGR_int(instance: *mut c_void, sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// create(const MatShape &, int, UMatUsageFlags)(TraitClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2821
// ("cv::UMat::create", vec![(pred!(mut, ["shape", "type", "usageFlags"], ["const cv::MatShape*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_create_const_MatShapeR_int_UMatUsageFlags(instance: *mut c_void, shape: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::create(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2821
// ("cv::UMat::create", vec![(pred!(mut, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv_UMat_create_const_MatShapeR_int(instance: *mut c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2824
// ("cv::UMat::fit", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fit_int_int_int_UMatUsageFlags(instance: *mut c_void, rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fit(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2824
// ("cv::UMat::fit", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_UMat_fit_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2825
// ("cv::UMat::fit", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fit_Size_int_UMatUsageFlags(instance: *mut c_void, size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fit(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2825
// ("cv::UMat::fit", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_UMat_fit_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2826
// ("cv::UMat::fit", vec![(pred!(mut, ["ndims", "sizes", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fit_int_const_intX_int_UMatUsageFlags(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fit(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2826
// ("cv::UMat::fit", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv_UMat_fit_int_const_intX_int(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(const std::vector<int> &, int, UMatUsageFlags)(CppPassByVoidPtr, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2827
// ("cv::UMat::fit", vec![(pred!(mut, ["sizes", "type", "usageFlags"], ["const std::vector<int>*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fit_const_vectorLintGR_int_UMatUsageFlags(instance: *mut c_void, sizes: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fit(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2827
// ("cv::UMat::fit", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
pub fn cv_UMat_fit_const_vectorLintGR_int(instance: *mut c_void, sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fit(const MatShape &, int, UMatUsageFlags)(TraitClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2828
// ("cv::UMat::fit", vec![(pred!(mut, ["shape", "type", "usageFlags"], ["const cv::MatShape*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fit_const_MatShapeR_int_UMatUsageFlags(instance: *mut c_void, shape: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fit(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2828
// ("cv::UMat::fit", vec![(pred!(mut, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv_UMat_fit_const_MatShapeR_int(instance: *mut c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// createSameSize(InputArray, int, UMatUsageFlags)(InputArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2832
// ("cv::UMat::createSameSize", vec![(pred!(mut, ["arr", "type", "usageFlags"], ["const cv::_InputArray*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_createSameSize_const__InputArrayR_int_UMatUsageFlags(instance: *mut c_void, arr: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::createSameSize(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2832
// ("cv::UMat::createSameSize", vec![(pred!(mut, ["arr", "type"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_UMat_createSameSize_const__InputArrayR_int(instance: *mut c_void, arr: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fitSameSize(InputArray, int, UMatUsageFlags)(InputArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2834
// ("cv::UMat::fitSameSize", vec![(pred!(mut, ["arr", "type", "usageFlags"], ["const cv::_InputArray*", "int", "cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_fitSameSize_const__InputArrayR_int_UMatUsageFlags(instance: *mut c_void, arr: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<()>);
// cv::UMat::fitSameSize(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2834
// ("cv::UMat::fitSameSize", vec![(pred!(mut, ["arr", "type"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_UMat_fitSameSize_const__InputArrayR_int(instance: *mut c_void, arr: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// addref()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2837
// ("cv::UMat::addref", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_addref(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2840
// ("cv::UMat::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// deallocate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2843
// ("cv::UMat::deallocate", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_deallocate(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2848
// ("cv::UMat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
pub fn cv_UMat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result<()>);
// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2850
// ("cv::UMat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
pub fn cv_UMat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2853
// ("cv::UMat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_UMat_operator___const_Range_Range(instance: *const c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2853
// ("cv::UMat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_UMat_operator___Range_Range(instance: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2854
// ("cv::UMat::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
pub fn cv_UMat_operator___const_const_RectR(instance: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2854
// ("cv::UMat::operator()", vec![(pred!(mut, ["roi"], ["const cv::Rect*"]), _)]),
pub fn cv_UMat_operator___const_RectR(instance: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const std::vector<Range> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2856
// ("cv::UMat::operator()", vec![(pred!(const, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_UMat_operator___const_const_vectorLRangeGR(instance: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::UMat::operator()(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2856
// ("cv::UMat::operator()", vec![(pred!(mut, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_UMat_operator___const_vectorLRangeGR(instance: *mut c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// isContinuous()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2861
// ("cv::UMat::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_isContinuous_const(instance: *const c_void) -> bool;
// isSubmatrix()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2864
// ("cv::UMat::isSubmatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_isSubmatrix_const(instance: *const c_void) -> bool;
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2868
// ("cv::UMat::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2870
// ("cv::UMat::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_elemSize1_const(instance: *const c_void) -> size_t;
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2872
// ("cv::UMat::type", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_type_const(instance: *const c_void) -> i32;
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2874
// ("cv::UMat::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_depth_const(instance: *const c_void) -> i32;
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2876
// ("cv::UMat::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_channels_const(instance: *const c_void) -> i32;
// step1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2878
// ("cv::UMat::step1", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_UMat_step1_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
// cv::UMat::step1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2878
// ("cv::UMat::step1", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2880
// ("cv::UMat::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_empty_const(instance: *const c_void) -> bool;
// total()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2882
// ("cv::UMat::total", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_total_const(instance: *const c_void) -> size_t;
// shape()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2886
// ("cv::UMat::shape", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_shape_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// checkVector(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2889
// ("cv::UMat::checkVector", vec![(pred!(const, ["elemChannels", "depth", "requireContinuous"], ["int", "int", "bool"]), _)]),
pub fn cv_UMat_checkVector_const_int_int_bool(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool, ocvrs_return: *mut Result<i32>);
// cv::UMat::checkVector(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2889
// ("cv::UMat::checkVector", vec![(pred!(const, ["elemChannels"], ["int"]), _)]),
pub fn cv_UMat_checkVector_const_int(instance: *const c_void, elem_channels: i32, ocvrs_return: *mut Result<i32>);
// UMat(UMat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2891
// ("cv::UMat::UMat", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
pub fn cv_UMat_UMat_UMatRR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(UMat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2892
// ("cv::UMat::operator=", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
pub fn cv_UMat_operatorST_UMatRR(instance: *mut c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// handle(AccessFlag)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2898
// ("cv::UMat::handle", vec![(pred!(const, ["accessFlags"], ["cv::AccessFlag"]), _)]),
pub fn cv_UMat_handle_const_AccessFlag(instance: *const c_void, access_flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
// ndoffset(size_t *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2899
// ("cv::UMat::ndoffset", vec![(pred!(const, ["ofs"], ["size_t*"]), _)]),
pub fn cv_UMat_ndoffset_const_size_tX(instance: *const c_void, ofs: *mut size_t, ocvrs_return: *mut Result<()>);
// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2931
// ("cv::UMat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::UMat::size() generated
// ("cv::UMat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// cv::UMat::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2910
// ("cv::UMat::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propFlags_const(instance: *const c_void) -> i32;
// cv::UMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2910
// ("cv::UMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMat_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::UMat::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2913
// ("cv::UMat::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propDims_const(instance: *const c_void) -> i32;
// cv::UMat::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2913
// ("cv::UMat::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMat_propDims_const_int(instance: *mut c_void, val: i32);
// cv::UMat::rows() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2916
// ("cv::UMat::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propRows_const(instance: *const c_void) -> i32;
// cv::UMat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2916
// ("cv::UMat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMat_propRows_const_int(instance: *mut c_void, val: i32);
// cv::UMat::cols() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2919
// ("cv::UMat::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propCols_const(instance: *const c_void) -> i32;
// cv::UMat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2919
// ("cv::UMat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMat_propCols_const_int(instance: *mut c_void, val: i32);
// cv::UMat::usageFlags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2925
// ("cv::UMat::usageFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propUsageFlags_const(instance: *const c_void, ocvrs_return: *mut core::UMatUsageFlags);
// cv::UMat::setUsageFlags(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2925
// ("cv::UMat::setUsageFlags", vec![(pred!(mut, ["val"], ["const cv::UMatUsageFlags"]), _)]),
pub fn cv_UMat_propUsageFlags_const_UMatUsageFlags(instance: *mut c_void, val: core::UMatUsageFlags);
// cv::UMat::u() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2934
// ("cv::UMat::u", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_propU(instance: *mut c_void) -> *mut c_void;
// cv::UMat::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2934
// ("cv::UMat::setU", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
pub fn cv_UMat_propU_UMatDataX(instance: *mut c_void, val: *const c_void);
// cv::UMat::offset() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2937
// ("cv::UMat::offset", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propOffset_const(instance: *const c_void) -> size_t;
// cv::UMat::setOffset(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2937
// ("cv::UMat::setOffset", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_UMat_propOffset_const_size_t(instance: *mut c_void, val: size_t);
// cv::UMat::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2940
// ("cv::UMat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propSize_const(instance: *const c_void) -> *mut c_void;
// cv::UMat::setSize(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2940
// ("cv::UMat::setSize", vec![(pred!(mut, ["val"], ["const cv::MatSize"]), _)]),
pub fn cv_UMat_propSize_const_MatSize(instance: *mut c_void, val: *const c_void);
// cv::UMat::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:2943
// ("cv::UMat::step", vec![(pred!(const, [], []), _)]),
pub fn cv_UMat_propStep_const(instance: *const c_void) -> *mut c_void;
// cv::UMat::delete() generated
// ("cv::UMat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMat_delete(instance: *mut c_void);
// lock()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:694
// ("cv::UMatData::lock", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_lock(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// unlock()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:695
// ("cv::UMatData::unlock", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_unlock(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// hostCopyObsolete()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:697
// ("cv::UMatData::hostCopyObsolete", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_hostCopyObsolete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// deviceCopyObsolete()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:698
// ("cv::UMatData::deviceCopyObsolete", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_deviceCopyObsolete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// deviceMemMapped()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:699
// ("cv::UMatData::deviceMemMapped", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_deviceMemMapped_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// copyOnMap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:700
// ("cv::UMatData::copyOnMap", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_copyOnMap_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// tempUMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:701
// ("cv::UMatData::tempUMat", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_tempUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// tempCopiedUMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:702
// ("cv::UMatData::tempCopiedUMat", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_tempCopiedUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// markHostCopyObsolete(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:703
// ("cv::UMatData::markHostCopyObsolete", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_UMatData_markHostCopyObsolete_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// markDeviceCopyObsolete(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:704
// ("cv::UMatData::markDeviceCopyObsolete", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_UMatData_markDeviceCopyObsolete_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// markDeviceMemMapped(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:705
// ("cv::UMatData::markDeviceMemMapped", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_UMatData_markDeviceMemMapped_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// cv::UMatData::urefcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:709
// ("cv::UMatData::urefcount", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propUrefcount_const(instance: *const c_void) -> i32;
// cv::UMatData::setUrefcount(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:709
// ("cv::UMatData::setUrefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMatData_propUrefcount_const_int(instance: *mut c_void, val: i32);
// cv::UMatData::refcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:710
// ("cv::UMatData::refcount", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propRefcount_const(instance: *const c_void) -> i32;
// cv::UMatData::setRefcount(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:710
// ("cv::UMatData::setRefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMatData_propRefcount_const_int(instance: *mut c_void, val: i32);
// cv::UMatData::data() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:711
// ("cv::UMatData::data", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propData_const(instance: *const c_void) -> *const u8;
// cv::UMatData::dataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:711
// ("cv::UMatData::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_propData(instance: *mut c_void) -> *mut u8;
// cv::UMatData::setData(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:711
// ("cv::UMatData::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_UMatData_propData_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::UMatData::origdata() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:712
// ("cv::UMatData::origdata", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propOrigdata_const(instance: *const c_void) -> *const u8;
// cv::UMatData::origdataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:712
// ("cv::UMatData::origdataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_propOrigdata(instance: *mut c_void) -> *mut u8;
// cv::UMatData::setOrigdata(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:712
// ("cv::UMatData::setOrigdata", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_UMatData_propOrigdata_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::UMatData::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:713
// ("cv::UMatData::size", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propSize_const(instance: *const c_void) -> size_t;
// cv::UMatData::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:713
// ("cv::UMatData::setSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_UMatData_propSize_const_size_t(instance: *mut c_void, val: size_t);
// cv::UMatData::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:715
// ("cv::UMatData::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propFlags_const(instance: *const c_void, ocvrs_return: *mut core::UMatData_MemoryFlag);
// cv::UMatData::setFlags(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:715
// ("cv::UMatData::setFlags", vec![(pred!(mut, ["val"], ["const cv::UMatData::MemoryFlag"]), _)]),
pub fn cv_UMatData_propFlags_const_MemoryFlag(instance: *mut c_void, val: core::UMatData_MemoryFlag);
// cv::UMatData::handle() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:716
// ("cv::UMatData::handle", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_propHandle(instance: *mut c_void) -> *mut c_void;
// cv::UMatData::setHandle(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:716
// ("cv::UMatData::setHandle", vec![(pred!(mut, ["val"], ["void*"]), _)]),
pub fn cv_UMatData_propHandle_voidX(instance: *mut c_void, val: *const c_void);
// cv::UMatData::userdata() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:717
// ("cv::UMatData::userdata", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_propUserdata(instance: *mut c_void) -> *mut c_void;
// cv::UMatData::setUserdata(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:717
// ("cv::UMatData::setUserdata", vec![(pred!(mut, ["val"], ["void*"]), _)]),
pub fn cv_UMatData_propUserdata_voidX(instance: *mut c_void, val: *const c_void);
// cv::UMatData::allocatorFlags_() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:718
// ("cv::UMatData::allocatorFlags_", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propAllocatorFlags__const(instance: *const c_void) -> i32;
// cv::UMatData::setAllocatorFlags_(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:718
// ("cv::UMatData::setAllocatorFlags_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMatData_propAllocatorFlags__const_int(instance: *mut c_void, val: i32);
// cv::UMatData::mapcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:719
// ("cv::UMatData::mapcount", vec![(pred!(const, [], []), _)]),
pub fn cv_UMatData_propMapcount_const(instance: *const c_void) -> i32;
// cv::UMatData::setMapcount(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:719
// ("cv::UMatData::setMapcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_UMatData_propMapcount_const_int(instance: *mut c_void, val: i32);
// cv::UMatData::originalUMatData() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:720
// ("cv::UMatData::originalUMatData", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_propOriginalUMatData(instance: *mut c_void) -> *mut c_void;
// cv::UMatData::setOriginalUMatData(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:720
// ("cv::UMatData::setOriginalUMatData", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
pub fn cv_UMatData_propOriginalUMatData_UMatDataX(instance: *mut c_void, val: *const c_void);
// cv::UMatData::delete() generated
// ("cv::UMatData::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_UMatData_delete(instance: *mut c_void);
// _InputArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:292
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputArray__InputArray(ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:293
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
pub fn cv__InputArray__InputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:294
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv__InputArray__InputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:295
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["expr"], ["const cv::MatExpr*"]), _)]),
pub fn cv__InputArray__InputArray_const_MatExprR(expr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:296
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv__InputArray__InputArray_const_vectorLMatGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const std::vector<bool> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:299
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec"], ["const std::vector<bool>*"]), _)]),
pub fn cv__InputArray__InputArray_const_vectorLboolGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:305
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["val"], ["const double*"]), _)]),
pub fn cv__InputArray__InputArray_const_doubleR(val: *const f64, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:306
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv__InputArray__InputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:307
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["d_mat_array"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv__InputArray__InputArray_const_vectorLGpuMatGR(d_mat_array: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:308
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
pub fn cv__InputArray__InputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:309
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
pub fn cv__InputArray__InputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:311
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["um"], ["const cv::UMat*"]), _)]),
pub fn cv__InputArray__InputArray_const_UMatR(um: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:312
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["umv"], ["const std::vector<cv::UMat>*"]), _)]),
pub fn cv__InputArray__InputArray_const_vectorLUMatGR(umv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMat(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:320
// ("cv::_InputArray::getMat", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv__InputArray_getMat_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputArray::getMat() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:320
// ("cv::_InputArray::getMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMat_(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:321
// ("cv::_InputArray::getMat_", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv__InputArray_getMat__const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputArray::getMat_() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:321
// ("cv::_InputArray::getMat_", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getMat__const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getUMat(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:322
// ("cv::_InputArray::getUMat", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv__InputArray_getUMat_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputArray::getUMat() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:322
// ("cv::_InputArray::getUMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getUMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMatVector(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:323
// ("cv::_InputArray::getMatVector", vec![(pred!(const, ["mv"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv__InputArray_getMatVector_const_vectorLMatGR(instance: *const c_void, mv: *mut c_void, ocvrs_return: *mut Result<()>);
// getUMatVector(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:324
// ("cv::_InputArray::getUMatVector", vec![(pred!(const, ["umv"], ["std::vector<cv::UMat>*"]), _)]),
pub fn cv__InputArray_getUMatVector_const_vectorLUMatGR(instance: *const c_void, umv: *mut c_void, ocvrs_return: *mut Result<()>);
// getGpuMatVector(std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:325
// ("cv::_InputArray::getGpuMatVector", vec![(pred!(const, ["gpumv"], ["std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv__InputArray_getGpuMatVector_const_vectorLGpuMatGR(instance: *const c_void, gpumv: *mut c_void, ocvrs_return: *mut Result<()>);
// getGpuMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:326
// ("cv::_InputArray::getGpuMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getGpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getOGlBuffer()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:327
// ("cv::_InputArray::getOGlBuffer", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getOGlBuffer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:329
// ("cv::_InputArray::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getObj()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:330
// ("cv::_InputArray::getObj", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getObj_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSz()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:331
// ("cv::_InputArray::getSz", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_getSz_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// kind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:333
// ("cv::_InputArray::kind", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_kind_const(instance: *const c_void, ocvrs_return: *mut Result<core::_InputArray_KindFlag>);
// dims(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:334
// ("cv::_InputArray::dims", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_dims_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:334
// ("cv::_InputArray::dims", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_dims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:335
// ("cv::_InputArray::cols", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_cols_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::cols() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:335
// ("cv::_InputArray::cols", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// rows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:336
// ("cv::_InputArray::rows", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_rows_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::rows() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:336
// ("cv::_InputArray::rows", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// size(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:337
// ("cv::_InputArray::size", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_size_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<core::Size>);
// cv::_InputArray::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:337
// ("cv::_InputArray::size", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// shape(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:338
// ("cv::_InputArray::shape", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_shape_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputArray::shape() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:338
// ("cv::_InputArray::shape", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_shape_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// sizend(int *, int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:339
// ("cv::_InputArray::sizend", vec![(pred!(const, ["sz", "i"], ["int*", "int"]), _)]),
pub fn cv__InputArray_sizend_const_intX_int(instance: *const c_void, sz: *mut i32, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::sizend(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:339
// ("cv::_InputArray::sizend", vec![(pred!(const, ["sz"], ["int*"]), _)]),
pub fn cv__InputArray_sizend_const_intX(instance: *const c_void, sz: *mut i32, ocvrs_return: *mut Result<i32>);
// sameSize(const _InputArray &)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:340
// ("cv::_InputArray::sameSize", vec![(pred!(const, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv__InputArray_sameSize_const_const__InputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result<bool>);
// total(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:341
// ("cv::_InputArray::total", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_total_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
// cv::_InputArray::total() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:341
// ("cv::_InputArray::total", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_total_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:342
// ("cv::_InputArray::type", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_type_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::type() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:342
// ("cv::_InputArray::type", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// depth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:343
// ("cv::_InputArray::depth", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_depth_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:343
// ("cv::_InputArray::depth", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// channels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:344
// ("cv::_InputArray::channels", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_channels_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
// cv::_InputArray::channels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:344
// ("cv::_InputArray::channels", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// isContinuous(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:345
// ("cv::_InputArray::isContinuous", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_isContinuous_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<bool>);
// cv::_InputArray::isContinuous() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:345
// ("cv::_InputArray::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isSubmatrix(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:346
// ("cv::_InputArray::isSubmatrix", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_isSubmatrix_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<bool>);
// cv::_InputArray::isSubmatrix() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:346
// ("cv::_InputArray::isSubmatrix", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isSubmatrix_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:347
// ("cv::_InputArray::empty", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// empty(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:348
// ("cv::_InputArray::empty", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_empty_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<bool>);
// copyTo(const _OutputArray &)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:349
// ("cv::_InputArray::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
pub fn cv__InputArray_copyTo_const_const__OutputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(const _OutputArray &, const _InputArray &)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:350
// ("cv::_InputArray::copyTo", vec![(pred!(const, ["arr", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, arr: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// offset(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:351
// ("cv::_InputArray::offset", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_offset_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
// cv::_InputArray::offset() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:351
// ("cv::_InputArray::offset", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_offset_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// step(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:352
// ("cv::_InputArray::step", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__InputArray_step_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
// cv::_InputArray::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:352
// ("cv::_InputArray::step", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_step_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// isMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:353
// ("cv::_InputArray::isMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isUMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:354
// ("cv::_InputArray::isUMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isMatVector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:355
// ("cv::_InputArray::isMatVector", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isUMatVector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:356
// ("cv::_InputArray::isUMatVector", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isUMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isVecVector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:357
// ("cv::_InputArray::isVecVector", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isVecVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isMatx()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:358
// ("cv::_InputArray::isMatx", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isMatx_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isVector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:359
// ("cv::_InputArray::isVector", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isGpuMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:360
// ("cv::_InputArray::isGpuMat", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isGpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isGpuMatVector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:361
// ("cv::_InputArray::isGpuMatVector", vec![(pred!(const, [], []), _)]),
pub fn cv__InputArray_isGpuMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::_InputArray::_InputArray(VariableArray, Primitive) generated
// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec", "n"], ["const unsigned char*", "int"]), _)]),
pub fn cv__InputArray__InputArray_const_unsigned_charX_int(vec: *const u8, n: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputArray::delete() generated
// ("cv::_InputArray::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputArray_delete(instance: *mut c_void);
// _InputOutputArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:512
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputOutputArray__InputOutputArray(ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:513
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:514
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:515
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_vectorLMatGR(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:516
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_GpuMatR(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:517
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["buf"], ["cv::ogl::Buffer*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_BufferR(buf: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:518
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["cuda_mem"], ["cv::cuda::HostMem*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_HostMemR(cuda_mem: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:527
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_UMatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:528
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::UMat>*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_vectorLUMatGR(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:530
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:531
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_vectorLMatGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:532
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:533
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_vectorLGpuMatGR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:534
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:535
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:543
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _InputOutputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:544
// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::UMat>*"]), _)]),
pub fn cv__InputOutputArray__InputOutputArray_const_vectorLUMatGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::_InputOutputArray::to__InputArray() generated
// ("cv::_InputOutputArray::to__InputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputOutputArray_to__InputArray(instance: *mut c_void) -> *mut c_void;
// cv::_InputOutputArray::to__OutputArray() generated
// ("cv::_InputOutputArray::to__OutputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputOutputArray_to__OutputArray(instance: *mut c_void) -> *mut c_void;
// cv::_InputOutputArray::delete() generated
// ("cv::_InputOutputArray::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv__InputOutputArray_delete(instance: *mut c_void);
// _OutputArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:424
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__OutputArray__OutputArray(ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:425
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
pub fn cv__OutputArray__OutputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:426
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
pub fn cv__OutputArray__OutputArray_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:427
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv__OutputArray__OutputArray_vectorLMatGR(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:428
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv__OutputArray__OutputArray_GpuMatR(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:429
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv__OutputArray__OutputArray_vectorLGpuMatGR(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:430
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["buf"], ["cv::ogl::Buffer*"]), _)]),
pub fn cv__OutputArray__OutputArray_BufferR(buf: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:431
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["cuda_mem"], ["cv::cuda::HostMem*"]), _)]),
pub fn cv__OutputArray__OutputArray_HostMemR(cuda_mem: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:441
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
pub fn cv__OutputArray__OutputArray_UMatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:442
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::UMat>*"]), _)]),
pub fn cv__OutputArray__OutputArray_vectorLUMatGR(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:444
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:445
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_vectorLMatGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:446
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:448
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:449
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:457
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _OutputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:458
// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::UMat>*"]), _)]),
pub fn cv__OutputArray__OutputArray_const_vectorLUMatGR(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// fixedSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:468
// ("cv::_OutputArray::fixedSize", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_fixedSize_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// fixedType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:469
// ("cv::_OutputArray::fixedType", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_fixedType_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// needed()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:470
// ("cv::_OutputArray::needed", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_needed_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getMatRef(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:471
// ("cv::_OutputArray::getMatRef", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__OutputArray_getMatRef_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_OutputArray::getMatRef() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:471
// ("cv::_OutputArray::getMatRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getMatRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getUMatRef(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:472
// ("cv::_OutputArray::getUMatRef", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv__OutputArray_getUMatRef_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::_OutputArray::getUMatRef() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:472
// ("cv::_OutputArray::getUMatRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getUMatRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGpuMatRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:473
// ("cv::_OutputArray::getGpuMatRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getGpuMatRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getGpuMatVecRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:474
// ("cv::_OutputArray::getGpuMatVecRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getGpuMatVecRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMatVecRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:475
// ("cv::_OutputArray::getMatVecRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getMatVecRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getUMatVecRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:476
// ("cv::_OutputArray::getUMatVecRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getUMatVecRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getOGlBufferRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:478
// ("cv::_OutputArray::getOGlBufferRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getOGlBufferRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getHostMemRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:479
// ("cv::_OutputArray::getHostMemRef", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_getHostMemRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(Size, int, int, bool, _OutputArray::DepthMask)(SimpleClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:481
// ("cv::_OutputArray::create", vec![(pred!(const, ["sz", "type", "i", "allowTransposed", "fixedDepthMask"], ["cv::Size", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_create_const_Size_int_int_bool_DepthMask(instance: *const c_void, sz: *const core::Size, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:481
// ("cv::_OutputArray::create", vec![(pred!(const, ["sz", "type"], ["cv::Size", "int"]), _)]),
pub fn cv__OutputArray_create_const_Size_int(instance: *const c_void, sz: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// create(int, int, int, int, bool, _OutputArray::DepthMask)(Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:482
// ("cv::_OutputArray::create", vec![(pred!(const, ["rows", "cols", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "int", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(instance: *const c_void, rows: i32, cols: i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:482
// ("cv::_OutputArray::create", vec![(pred!(const, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv__OutputArray_create_const_int_int_int(instance: *const c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(int, const int *, int, int, bool, _OutputArray::DepthMask)(Primitive, VariableArray, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:483
// ("cv::_OutputArray::create", vec![(pred!(const, ["dims", "size", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "const int*", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(instance: *const c_void, dims: i32, size: *const i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::create(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:483
// ("cv::_OutputArray::create", vec![(pred!(const, ["dims", "size", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv__OutputArray_create_const_int_const_intX_int(instance: *const c_void, dims: i32, size: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(const MatShape &, int, int, bool, _OutputArray::DepthMask)(TraitClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:484
// ("cv::_OutputArray::create", vec![(pred!(const, ["shape", "type", "i", "allowTransposed", "fixedDepthMask"], ["const cv::MatShape*", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_create_const_const_MatShapeR_int_int_bool_DepthMask(instance: *const c_void, shape: *const c_void, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::create(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:484
// ("cv::_OutputArray::create", vec![(pred!(const, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv__OutputArray_create_const_const_MatShapeR_int(instance: *const c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// createSameSize(const _InputArray &, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:485
// ("cv::_OutputArray::createSameSize", vec![(pred!(const, ["arr", "mtype"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv__OutputArray_createSameSize_const_const__InputArrayR_int(instance: *const c_void, arr: *const c_void, mtype: i32, ocvrs_return: *mut Result<()>);
// fit(Size, int, int, bool, _OutputArray::DepthMask)(SimpleClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:487
// ("cv::_OutputArray::fit", vec![(pred!(const, ["sz", "type", "i", "allowTransposed", "fixedDepthMask"], ["cv::Size", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_fit_const_Size_int_int_bool_DepthMask(instance: *const c_void, sz: *const core::Size, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::fit(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:487
// ("cv::_OutputArray::fit", vec![(pred!(const, ["sz", "type"], ["cv::Size", "int"]), _)]),
pub fn cv__OutputArray_fit_const_Size_int(instance: *const c_void, sz: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, int, int, int, bool, _OutputArray::DepthMask)(Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:488
// ("cv::_OutputArray::fit", vec![(pred!(const, ["rows", "cols", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "int", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_fit_const_int_int_int_int_bool_DepthMask(instance: *const c_void, rows: i32, cols: i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::fit(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:488
// ("cv::_OutputArray::fit", vec![(pred!(const, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv__OutputArray_fit_const_int_int_int(instance: *const c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(int, const int *, int, int, bool, _OutputArray::DepthMask)(Primitive, VariableArray, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:489
// ("cv::_OutputArray::fit", vec![(pred!(const, ["dims", "size", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "const int*", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_fit_const_int_const_intX_int_int_bool_DepthMask(instance: *const c_void, dims: i32, size: *const i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::fit(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:489
// ("cv::_OutputArray::fit", vec![(pred!(const, ["dims", "size", "type"], ["int", "const int*", "int"]), _)]),
pub fn cv__OutputArray_fit_const_int_const_intX_int(instance: *const c_void, dims: i32, size: *const i32, typ: i32, ocvrs_return: *mut Result<()>);
// fit(const MatShape &, int, int, bool, _OutputArray::DepthMask)(TraitClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:490
// ("cv::_OutputArray::fit", vec![(pred!(const, ["shape", "type", "i", "allowTransposed", "fixedDepthMask"], ["const cv::MatShape*", "int", "int", "bool", "cv::_OutputArray::DepthMask"]), _)]),
pub fn cv__OutputArray_fit_const_const_MatShapeR_int_int_bool_DepthMask(instance: *const c_void, shape: *const c_void, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::fit(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:490
// ("cv::_OutputArray::fit", vec![(pred!(const, ["shape", "type"], ["const cv::MatShape*", "int"]), _)]),
pub fn cv__OutputArray_fit_const_const_MatShapeR_int(instance: *const c_void, shape: *const c_void, typ: i32, ocvrs_return: *mut Result<()>);
// fitSameSize(const _InputArray &, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:491
// ("cv::_OutputArray::fitSameSize", vec![(pred!(const, ["arr", "mtype"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv__OutputArray_fitSameSize_const_const__InputArrayR_int(instance: *const c_void, arr: *const c_void, mtype: i32, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:493
// ("cv::_OutputArray::release", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_release_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:494
// ("cv::_OutputArray::clear", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_clear_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// setTo(const _InputArray &, const _InputArray &)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:495
// ("cv::_OutputArray::setTo", vec![(pred!(const, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:495
// ("cv::_OutputArray::setTo", vec![(pred!(const, ["value"], ["const cv::_InputArray*"]), _)]),
pub fn cv__OutputArray_setTo_const_const__InputArrayR(instance: *const c_void, value: *const c_void, ocvrs_return: *mut Result<()>);
// setZero()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:496
// ("cv::_OutputArray::setZero", vec![(pred!(const, [], []), _)]),
pub fn cv__OutputArray_setZero_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// assign(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:498
// ("cv::_OutputArray::assign", vec![(pred!(const, ["u"], ["const cv::UMat*"]), _)]),
pub fn cv__OutputArray_assign_const_const_UMatR(instance: *const c_void, u: *const c_void, ocvrs_return: *mut Result<()>);
// assign(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:499
// ("cv::_OutputArray::assign", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv__OutputArray_assign_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// assign(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:501
// ("cv::_OutputArray::assign", vec![(pred!(const, ["v"], ["const std::vector<cv::UMat>*"]), _)]),
pub fn cv__OutputArray_assign_const_const_vectorLUMatGR(instance: *const c_void, v: *const c_void, ocvrs_return: *mut Result<()>);
// assign(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:502
// ("cv::_OutputArray::assign", vec![(pred!(const, ["v"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv__OutputArray_assign_const_const_vectorLMatGR(instance: *const c_void, v: *const c_void, ocvrs_return: *mut Result<()>);
// move(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:504
// ("cv::_OutputArray::move", vec![(pred!(const, ["u"], ["cv::UMat*"]), _)]),
pub fn cv__OutputArray_move_const_UMatR(instance: *const c_void, u: *mut c_void, ocvrs_return: *mut Result<()>);
// move(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/mat.hpp:505
// ("cv::_OutputArray::move", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
pub fn cv__OutputArray_move_const_MatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::_OutputArray::to__InputArray() generated
// ("cv::_OutputArray::to__InputArray", vec![(pred!(mut, [], []), _)]),
pub fn cv__OutputArray_to__InputArray(instance: *mut c_void) -> *mut c_void;
// cv::_OutputArray::delete() generated
// ("cv::_OutputArray::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv__OutputArray_delete(instance: *mut c_void);
// bfloat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:946
// ("cv::bfloat::bfloat", vec![(pred!(mut, [], []), _)]),
pub fn cv_bfloat_bfloat(ocvrs_return: *mut Result<core::bfloat>);
// bfloat(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:948
// ("cv::bfloat::bfloat", vec![(pred!(mut, ["x"], ["float"]), _)]),
pub fn cv_bfloat_bfloat_float(x: f32, ocvrs_return: *mut Result<core::bfloat>);
// operator float()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:955
// ("cv::bfloat::operator float", vec![(pred!(const, [], []), _)]),
pub fn cv_bfloat_operator_float_const(instance: *const core::bfloat, ocvrs_return: *mut Result<f32>);
// BufferPool(Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:744
// ("cv::cuda::BufferPool::BufferPool", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_BufferPool_BufferPool_StreamR(stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBuffer(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:747
// ("cv::cuda::BufferPool::getBuffer", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_BufferPool_getBuffer_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// getBuffer(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:755
// ("cv::cuda::BufferPool::getBuffer", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_BufferPool_getBuffer_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// getAllocator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:761
// ("cv::cuda::BufferPool::getAllocator", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BufferPool_getAllocator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::BufferPool::delete() generated
// ("cv::cuda::BufferPool::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BufferPool_delete(instance: *mut c_void);
// DeviceInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1116
// ("cv::cuda::DeviceInfo::DeviceInfo", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DeviceInfo_DeviceInfo(ocvrs_return: *mut Result<*mut c_void>);
// DeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1125
// ("cv::cuda::DeviceInfo::DeviceInfo", vec![(pred!(mut, ["device_id"], ["int"]), _)]),
pub fn cv_cuda_DeviceInfo_DeviceInfo_int(device_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// deviceID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1129
// ("cv::cuda::DeviceInfo::deviceID", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_deviceID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1132
// ("cv::cuda::DeviceInfo::name", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// totalGlobalMem()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1135
// ("cv::cuda::DeviceInfo::totalGlobalMem", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_totalGlobalMem_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// sharedMemPerBlock()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1138
// ("cv::cuda::DeviceInfo::sharedMemPerBlock", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_sharedMemPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// regsPerBlock()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1141
// ("cv::cuda::DeviceInfo::regsPerBlock", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_regsPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// warpSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1144
// ("cv::cuda::DeviceInfo::warpSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_warpSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// memPitch()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1147
// ("cv::cuda::DeviceInfo::memPitch", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_memPitch_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// maxThreadsPerBlock()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1150
// ("cv::cuda::DeviceInfo::maxThreadsPerBlock", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxThreadsPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxThreadsDim()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1153
// ("cv::cuda::DeviceInfo::maxThreadsDim", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxThreadsDim_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxGridSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1156
// ("cv::cuda::DeviceInfo::maxGridSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// clockRate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1159
// ("cv::cuda::DeviceInfo::clockRate", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_clockRate_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// totalConstMem()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1162
// ("cv::cuda::DeviceInfo::totalConstMem", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_totalConstMem_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// majorVersion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1165
// ("cv::cuda::DeviceInfo::majorVersion", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_majorVersion_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// minorVersion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1168
// ("cv::cuda::DeviceInfo::minorVersion", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_minorVersion_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// textureAlignment()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1171
// ("cv::cuda::DeviceInfo::textureAlignment", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_textureAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// texturePitchAlignment()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1174
// ("cv::cuda::DeviceInfo::texturePitchAlignment", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_texturePitchAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// multiProcessorCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1177
// ("cv::cuda::DeviceInfo::multiProcessorCount", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_multiProcessorCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// kernelExecTimeoutEnabled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1180
// ("cv::cuda::DeviceInfo::kernelExecTimeoutEnabled", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// integrated()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1183
// ("cv::cuda::DeviceInfo::integrated", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_integrated_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// canMapHostMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1186
// ("cv::cuda::DeviceInfo::canMapHostMemory", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_canMapHostMemory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// computeMode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1197
// ("cv::cuda::DeviceInfo::computeMode", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_computeMode_const(instance: *const c_void, ocvrs_return: *mut Result<core::DeviceInfo_ComputeMode>);
// maxTexture1D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1200
// ("cv::cuda::DeviceInfo::maxTexture1D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture1D_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxTexture1DMipmap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1203
// ("cv::cuda::DeviceInfo::maxTexture1DMipmap", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture1DMipmap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxTexture1DLinear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1206
// ("cv::cuda::DeviceInfo::maxTexture1DLinear", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture1DLinear_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxTexture2D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1209
// ("cv::cuda::DeviceInfo::maxTexture2D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture2D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxTexture2DMipmap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1212
// ("cv::cuda::DeviceInfo::maxTexture2DMipmap", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture2DMipmap_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxTexture2DLinear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1215
// ("cv::cuda::DeviceInfo::maxTexture2DLinear", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture2DLinear_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxTexture2DGather()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1218
// ("cv::cuda::DeviceInfo::maxTexture2DGather", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture2DGather_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxTexture3D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1221
// ("cv::cuda::DeviceInfo::maxTexture3D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture3D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxTextureCubemap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1224
// ("cv::cuda::DeviceInfo::maxTextureCubemap", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTextureCubemap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxTexture1DLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1227
// ("cv::cuda::DeviceInfo::maxTexture1DLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture1DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxTexture2DLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1230
// ("cv::cuda::DeviceInfo::maxTexture2DLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTexture2DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxTextureCubemapLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1233
// ("cv::cuda::DeviceInfo::maxTextureCubemapLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxSurface1D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1236
// ("cv::cuda::DeviceInfo::maxSurface1D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurface1D_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxSurface2D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1239
// ("cv::cuda::DeviceInfo::maxSurface2D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurface2D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxSurface3D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1242
// ("cv::cuda::DeviceInfo::maxSurface3D", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurface3D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxSurface1DLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1245
// ("cv::cuda::DeviceInfo::maxSurface1DLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurface1DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// maxSurface2DLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1248
// ("cv::cuda::DeviceInfo::maxSurface2DLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurface2DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
// maxSurfaceCubemap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1251
// ("cv::cuda::DeviceInfo::maxSurfaceCubemap", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurfaceCubemap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxSurfaceCubemapLayered()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1254
// ("cv::cuda::DeviceInfo::maxSurfaceCubemapLayered", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
// surfaceAlignment()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1257
// ("cv::cuda::DeviceInfo::surfaceAlignment", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_surfaceAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// concurrentKernels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1260
// ("cv::cuda::DeviceInfo::concurrentKernels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_concurrentKernels_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// ECCEnabled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1263
// ("cv::cuda::DeviceInfo::ECCEnabled", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_ECCEnabled_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// pciBusID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1266
// ("cv::cuda::DeviceInfo::pciBusID", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_pciBusID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// pciDeviceID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1269
// ("cv::cuda::DeviceInfo::pciDeviceID", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_pciDeviceID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// pciDomainID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1272
// ("cv::cuda::DeviceInfo::pciDomainID", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_pciDomainID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// tccDriver()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1275
// ("cv::cuda::DeviceInfo::tccDriver", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_tccDriver_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// asyncEngineCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1278
// ("cv::cuda::DeviceInfo::asyncEngineCount", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_asyncEngineCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// unifiedAddressing()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1281
// ("cv::cuda::DeviceInfo::unifiedAddressing", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_unifiedAddressing_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// memoryClockRate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1284
// ("cv::cuda::DeviceInfo::memoryClockRate", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_memoryClockRate_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// memoryBusWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1287
// ("cv::cuda::DeviceInfo::memoryBusWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_memoryBusWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// l2CacheSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1290
// ("cv::cuda::DeviceInfo::l2CacheSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_l2CacheSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxThreadsPerMultiProcessor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1293
// ("cv::cuda::DeviceInfo::maxThreadsPerMultiProcessor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// queryMemory(size_t &, size_t &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1296
// ("cv::cuda::DeviceInfo::queryMemory", vec![(pred!(const, ["totalMemory", "freeMemory"], ["size_t*", "size_t*"]), _)]),
pub fn cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(instance: *const c_void, total_memory: *mut size_t, free_memory: *mut size_t, ocvrs_return: *mut Result<()>);
// freeMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1297
// ("cv::cuda::DeviceInfo::freeMemory", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_freeMemory_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// totalMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1298
// ("cv::cuda::DeviceInfo::totalMemory", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_totalMemory_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// supports(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1306
// ("cv::cuda::DeviceInfo::supports", vec![(pred!(const, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
pub fn cv_cuda_DeviceInfo_supports_const_FeatureSet(instance: *const c_void, feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
// isCompatible()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1313
// ("cv::cuda::DeviceInfo::isCompatible", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DeviceInfo_isCompatible_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::cuda::DeviceInfo::delete() generated
// ("cv::cuda::DeviceInfo::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DeviceInfo_delete(instance: *mut c_void);
// Event(const Event::CreateFlags)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:994
// ("cv::cuda::Event::Event", vec![(pred!(mut, ["flags"], ["const cv::cuda::Event::CreateFlags"]), _)]),
pub fn cv_cuda_Event_Event_const_CreateFlags(flags: core::Event_CreateFlags, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::Event::Event() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:994
// ("cv::cuda::Event::Event", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Event_Event(ocvrs_return: *mut Result<*mut c_void>);
// record(Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:997
// ("cv::cuda::Event::record", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Event_record_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Event::record() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:997
// ("cv::cuda::Event::record", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Event_record(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// queryIfComplete()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1000
// ("cv::cuda::Event::queryIfComplete", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_Event_queryIfComplete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// waitForCompletion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1003
// ("cv::cuda::Event::waitForCompletion", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Event_waitForCompletion(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// elapsedTime(const Event &, const Event &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1006
// ("cv::cuda::Event::elapsedTime", vec![(pred!(mut, ["start", "end"], ["const cv::cuda::Event*", "const cv::cuda::Event*"]), _)]),
pub fn cv_cuda_Event_elapsedTime_const_EventR_const_EventR(start: *const c_void, end: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::cuda::Event::delete() generated
// ("cv::cuda::Event::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Event_delete(instance: *mut c_void);
// GpuData(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:374
// ("cv::cuda::GpuData::GpuData", vec![(pred!(mut, ["_size"], ["size_t"]), _)]),
pub fn cv_cuda_GpuData_GpuData_size_t(_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuData::data() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:383
// ("cv::cuda::GpuData::data", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuData_propData_const(instance: *const c_void) -> *const u8;
// cv::cuda::GpuData::dataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:383
// ("cv::cuda::GpuData::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuData_propData(instance: *mut c_void) -> *mut u8;
// cv::cuda::GpuData::setData(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:383
// ("cv::cuda::GpuData::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_cuda_GpuData_propData_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::cuda::GpuData::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:384
// ("cv::cuda::GpuData::size", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuData_propSize_const(instance: *const c_void) -> size_t;
// cv::cuda::GpuData::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:384
// ("cv::cuda::GpuData::setSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_cuda_GpuData_propSize_const_size_t(instance: *mut c_void, val: size_t);
// cv::cuda::GpuData::delete() generated
// ("cv::cuda::GpuData::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuData_delete(instance: *mut c_void);
// defaultAllocator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:119
// ("cv::cuda::GpuMat::defaultAllocator", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_defaultAllocator(ocvrs_return: *mut Result<*mut c_void>);
// setDefaultAllocator(GpuMat::Allocator *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:120
// ("cv::cuda::GpuMat::setDefaultAllocator", vec![(pred!(mut, ["allocator"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(allocator: *mut c_void, ocvrs_return: *mut Result<()>);
// getStdAllocator()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:121
// ("cv::cuda::GpuMat::getStdAllocator", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_getStdAllocator(ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(GpuMat::Allocator *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:124
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["allocator"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_AllocatorX(allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:124
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_GpuMat(ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(int, int, int, GpuMat::Allocator *)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:127
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "allocator"], ["int", "int", "int", "cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(rows: i32, cols: i32, typ: i32, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:127
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(Size, int, GpuMat::Allocator *)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:128
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "allocator"], ["cv::Size", "int", "cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(size: *const core::Size, typ: i32, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:128
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(int, int, int, Scalar, GpuMat::Allocator *)(Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:131
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "s", "allocator"], ["int", "int", "int", "cv::Scalar", "cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:131
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int_Scalar(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(Size, int, Scalar, GpuMat::Allocator *)(SimpleClass, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:132
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "s", "allocator"], ["cv::Size", "int", "cv::Scalar", "cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(size: *const core::Size, typ: i32, s: *const core::Scalar, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:132
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int_Scalar(size: *const core::Size, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(const GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:135
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(int, int, int, void *, size_t)(Primitive, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:138
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "data", "step"], ["int", "int", "int", "void*", "size_t"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:138
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "data"], ["int", "int", "int", "void*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_int_int_int_voidX(rows: i32, cols: i32, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(Size, int, void *, size_t)(SimpleClass, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:139
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "data", "step"], ["cv::Size", "int", "void*", "size_t"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(size: *const core::Size, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:139
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "data"], ["cv::Size", "int", "void*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_Size_int_voidX(size: *const core::Size, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(const GpuMat &, Range, Range)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:142
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::cuda::GpuMat*", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(m: *const c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:142
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::cuda::GpuMat*", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_GpuMatR_Range_Range(m: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(const GpuMat &, Rect)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:143
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "roi"], ["const cv::cuda::GpuMat*", "cv::Rect"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:143
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "roi"], ["cv::cuda::GpuMat*", "cv::Rect"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_GpuMatR_Rect(m: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// GpuMat(InputArray, GpuMat::Allocator *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:146
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["arr", "allocator"], ["const cv::_InputArray*", "cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(arr: *const c_void, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::GpuMat(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:146
// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_GpuMat_GpuMat_const__InputArrayR(arr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:152
// ("cv::cuda::GpuMat::operator=", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_operatorST_const_GpuMatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:155
// ("cv::cuda::GpuMat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_GpuMat_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:156
// ("cv::cuda::GpuMat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_GpuMat_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:159
// ("cv::cuda::GpuMat::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// swap(GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:162
// ("cv::cuda::GpuMat::swap", vec![(pred!(mut, ["mat"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_swap_GpuMatR(instance: *mut c_void, mat: *mut c_void, ocvrs_return: *mut Result<()>);
// upload(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:169
// ("cv::cuda::GpuMat::upload", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_GpuMat_upload_const__InputArrayR(instance: *mut c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// upload(InputArray, Stream &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:179
// ("cv::cuda::GpuMat::upload", vec![(pred!(mut, ["arr", "stream"], ["const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(instance: *mut c_void, arr: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// download(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:186
// ("cv::cuda::GpuMat::download", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_GpuMat_download_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// download(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:196
// ("cv::cuda::GpuMat::download", vec![(pred!(const, ["dst", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:199
// ("cv::cuda::GpuMat::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:202
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:204
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_GpuMatR(instance: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:209
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:211
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_GpuMatR_StreamR(instance: *const c_void, dst: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:216
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(GpuMat &, GpuMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:218
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_GpuMatR_GpuMatR(instance: *const c_void, dst: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, InputArray, Stream &)(OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:223
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask", "stream"], ["const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(instance: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:225
// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_copyTo_const_GpuMatR_GpuMatR_StreamR(instance: *const c_void, dst: *mut c_void, mask: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// setTo(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:230
// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s"], ["cv::Scalar"]), _)]),
pub fn cv_cuda_GpuMat_setTo_Scalar(instance: *mut c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// setTo(Scalar, Stream &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:233
// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "stream"], ["cv::Scalar", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_setTo_Scalar_StreamR(instance: *mut c_void, s: *const core::Scalar, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setTo(Scalar, InputArray)(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:236
// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "mask"], ["cv::Scalar", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(instance: *mut c_void, s: *const core::Scalar, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setTo(Scalar, InputArray, Stream &)(SimpleClass, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:239
// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "mask", "stream"], ["cv::Scalar", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(instance: *mut c_void, s: *const core::Scalar, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// convertTo(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:242
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(instance: *const c_void, dst: *const c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, Stream &)(OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:245
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "stream"], ["const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTo(GpuMat &, int, Stream &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:247
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "stream"], ["cv::cuda::GpuMat*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_GpuMatR_int_StreamR(instance: *const c_void, dst: *mut c_void, rtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:252
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::GpuMat::convertTo(OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:252
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha"], ["const cv::_OutputArray*", "int", "double"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, ocvrs_return: *mut Result<()>);
// convertTo(GpuMat &, int, double, double)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:254
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta"], ["cv::cuda::GpuMat*", "int", "double", "double"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_GpuMatR_int_double_double(instance: *const c_void, dst: *mut c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<()>);
// cv::cuda::GpuMat::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:254
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype"], ["cv::cuda::GpuMat*", "int"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_GpuMatR_int(instance: *const c_void, dst: *mut c_void, rtype: i32, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, double, Stream &)(OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:259
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "stream"], ["const cv::_OutputArray*", "int", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTo(OutputArray, int, double, double, Stream &)(OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:262
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta", "stream"], ["const cv::_OutputArray*", "int", "double", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, beta: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// convertTo(GpuMat &, int, double, double, Stream &)(TraitClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:264
// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta", "stream"], ["cv::cuda::GpuMat*", "int", "double", "double", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMat_convertTo_const_GpuMatR_int_double_double_StreamR(instance: *const c_void, dst: *mut c_void, rtype: i32, alpha: f64, beta: f64, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// assignTo(GpuMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:268
// ("cv::cuda::GpuMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::cuda::GpuMat*", "int"]), _)]),
pub fn cv_cuda_GpuMat_assignTo_const_GpuMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::cuda::GpuMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:268
// ("cv::cuda::GpuMat::assignTo", vec![(pred!(const, ["m"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_assignTo_const_GpuMatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result<()>);
// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:271
// ("cv::cuda::GpuMat::ptr", vec![(pred!(mut, ["y"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_ptr_int(instance: *mut c_void, y: i32, ocvrs_return: *mut Result<*mut u8>);
// cv::cuda::GpuMat::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:271
// ("cv::cuda::GpuMat::ptr", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_ptr(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:272
// ("cv::cuda::GpuMat::ptr", vec![(pred!(const, ["y"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_ptr_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*const u8>);
// cv::cuda::GpuMat::ptr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:272
// ("cv::cuda::GpuMat::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:282
// ("cv::cuda::GpuMat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::row(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:282
// ("cv::cuda::GpuMat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_row_int(instance: *mut c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:285
// ("cv::cuda::GpuMat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::col(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:285
// ("cv::cuda::GpuMat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_col_int(instance: *mut c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:288
// ("cv::cuda::GpuMat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:288
// ("cv::cuda::GpuMat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_rowRange_int_int(instance: *mut c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
// rowRange(Range)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:289
// ("cv::cuda::GpuMat::rowRange", vec![(pred!(const, ["r"], ["cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_rowRange_const_Range(instance: *const c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:289
// ("cv::cuda::GpuMat::rowRange", vec![(pred!(mut, ["r"], ["cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_rowRange_Range(instance: *mut c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:292
// ("cv::cuda::GpuMat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:292
// ("cv::cuda::GpuMat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_colRange_int_int(instance: *mut c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
// colRange(Range)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:293
// ("cv::cuda::GpuMat::colRange", vec![(pred!(const, ["r"], ["cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_colRange_const_Range(instance: *const c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:293
// ("cv::cuda::GpuMat::colRange", vec![(pred!(mut, ["r"], ["cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_colRange_Range(instance: *mut c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:296
// ("cv::cuda::GpuMat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_operator___const_Range_Range(instance: *const c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:296
// ("cv::cuda::GpuMat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMat_operator___Range_Range(instance: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:297
// ("cv::cuda::GpuMat::operator()", vec![(pred!(const, ["roi"], ["cv::Rect"]), _)]),
pub fn cv_cuda_GpuMat_operator___const_Rect(instance: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:297
// ("cv::cuda::GpuMat::operator()", vec![(pred!(mut, ["roi"], ["cv::Rect"]), _)]),
pub fn cv_cuda_GpuMat_operator___Rect(instance: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:301
// ("cv::cuda::GpuMat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:301
// ("cv::cuda::GpuMat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_reshape_int(instance: *mut c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:301
// ("cv::cuda::GpuMat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
pub fn cv_cuda_GpuMat_reshape_const_int(instance: *const c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:301
// ("cv::cuda::GpuMat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_cuda_GpuMat_reshape_int_int(instance: *mut c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:304
// ("cv::cuda::GpuMat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
pub fn cv_cuda_GpuMat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result<()>);
// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:307
// ("cv::cuda::GpuMat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
pub fn cv_cuda_GpuMat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
// isContinuous()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:311
// ("cv::cuda::GpuMat::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:314
// ("cv::cuda::GpuMat::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:317
// ("cv::cuda::GpuMat::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:320
// ("cv::cuda::GpuMat::type", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:323
// ("cv::cuda::GpuMat::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:326
// ("cv::cuda::GpuMat::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// step1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:329
// ("cv::cuda::GpuMat::step1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:332
// ("cv::cuda::GpuMat::size", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:335
// ("cv::cuda::GpuMat::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cudaPtr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:338
// ("cv::cuda::GpuMat::cudaPtr", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_cudaPtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:341
// ("cv::cuda::GpuMat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::GpuMat::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:349
// ("cv::cuda::GpuMat::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propFlags_const(instance: *const c_void) -> i32;
// cv::cuda::GpuMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:349
// ("cv::cuda::GpuMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_GpuMat_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::cuda::GpuMat::rows() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:352
// ("cv::cuda::GpuMat::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propRows_const(instance: *const c_void) -> i32;
// cv::cuda::GpuMat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:352
// ("cv::cuda::GpuMat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_GpuMat_propRows_const_int(instance: *mut c_void, val: i32);
// cv::cuda::GpuMat::cols() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:352
// ("cv::cuda::GpuMat::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propCols_const(instance: *const c_void) -> i32;
// cv::cuda::GpuMat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:352
// ("cv::cuda::GpuMat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_GpuMat_propCols_const_int(instance: *mut c_void, val: i32);
// cv::cuda::GpuMat::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:355
// ("cv::cuda::GpuMat::step", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propStep_const(instance: *const c_void) -> size_t;
// cv::cuda::GpuMat::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:355
// ("cv::cuda::GpuMat::setStep", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_cuda_GpuMat_propStep_const_size_t(instance: *mut c_void, val: size_t);
// cv::cuda::GpuMat::data() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:358
// ("cv::cuda::GpuMat::data", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propData_const(instance: *const c_void) -> *const u8;
// cv::cuda::GpuMat::dataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:358
// ("cv::cuda::GpuMat::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_propData(instance: *mut c_void) -> *mut u8;
// cv::cuda::GpuMat::setData(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:358
// ("cv::cuda::GpuMat::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_cuda_GpuMat_propData_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::cuda::GpuMat::refcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:362
// ("cv::cuda::GpuMat::refcount", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propRefcount_const(instance: *const c_void) -> *const i32;
// cv::cuda::GpuMat::refcountMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:362
// ("cv::cuda::GpuMat::refcountMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_propRefcount(instance: *mut c_void) -> *mut i32;
// cv::cuda::GpuMat::setRefcount(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:362
// ("cv::cuda::GpuMat::setRefcount", vec![(pred!(mut, ["val"], ["int*"]), _)]),
pub fn cv_cuda_GpuMat_propRefcount_intX(instance: *mut c_void, val: *const i32);
// cv::cuda::GpuMat::datastart() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:365
// ("cv::cuda::GpuMat::datastart", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propDatastart_const(instance: *const c_void) -> *const u8;
// cv::cuda::GpuMat::datastartMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:365
// ("cv::cuda::GpuMat::datastartMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_propDatastart(instance: *mut c_void) -> *mut u8;
// cv::cuda::GpuMat::setDatastart(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:365
// ("cv::cuda::GpuMat::setDatastart", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_cuda_GpuMat_propDatastart_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::cuda::GpuMat::dataend() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:366
// ("cv::cuda::GpuMat::dataend", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMat_propDataend_const(instance: *const c_void) -> *const u8;
// cv::cuda::GpuMat::allocator() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:369
// ("cv::cuda::GpuMat::allocator", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_propAllocator(instance: *mut c_void) -> *mut c_void;
// cv::cuda::GpuMat::setAllocator(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:369
// ("cv::cuda::GpuMat::setAllocator", vec![(pred!(mut, ["val"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
pub fn cv_cuda_GpuMat_propAllocator_AllocatorX(instance: *mut c_void, val: *const c_void);
// cv::cuda::GpuMat::delete() generated
// ("cv::cuda::GpuMat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_delete(instance: *mut c_void);
// allocate(GpuMat *, int, int, size_t)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:114
// ("cv::cuda::GpuMat::Allocator::allocate", vec![(pred!(mut, ["mat", "rows", "cols", "elemSize"], ["cv::cuda::GpuMat*", "int", "int", "size_t"]), _)]),
pub fn cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(instance: *mut c_void, mat: *mut c_void, rows: i32, cols: i32, elem_size: size_t, ocvrs_return: *mut Result<bool>);
// free(GpuMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:115
// ("cv::cuda::GpuMat::Allocator::free", vec![(pred!(mut, ["mat"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_GpuMat_Allocator_free_GpuMatX(instance: *mut c_void, mat: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::GpuMat::Allocator::delete() generated
// ("cv::cuda::GpuMat::Allocator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMat_Allocator_delete(instance: *mut c_void);
// GpuMatND()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:398
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND(ocvrs_return: *mut Result<*mut c_void>);
// GpuMatND(SizeArray, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:405
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, ["size", "type"], ["cv::cuda::GpuMatND::SizeArray", "int"]), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND_SizeArray_int(size: *mut c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// GpuMatND(SizeArray, int, void *, StepArray)(CppPassByVoidPtr, Primitive, Indirect, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:419
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, ["size", "type", "data", "step"], ["cv::cuda::GpuMatND::SizeArray", "int", "void*", "cv::cuda::GpuMatND::StepArray"]), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND_SizeArray_int_voidX_StepArray(size: *mut c_void, typ: i32, data: *mut c_void, step: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMatND::GpuMatND(CppPassByVoidPtr, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:419
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, ["size", "type", "data"], ["cv::cuda::GpuMatND::SizeArray", "int", "void*"]), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND_SizeArray_int_voidX(size: *mut c_void, typ: i32, data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(SizeArray, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:428
// ("cv::cuda::GpuMatND::create", vec![(pred!(mut, ["size", "type"], ["cv::cuda::GpuMatND::SizeArray", "int"]), _)]),
pub fn cv_cuda_GpuMatND_create_SizeArray_int(instance: *mut c_void, size: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:430
// ("cv::cuda::GpuMatND::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMatND_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// swap(GpuMatND &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:432
// ("cv::cuda::GpuMatND::swap", vec![(pred!(mut, ["m"], ["cv::cuda::GpuMatND*"]), _)]),
pub fn cv_cuda_GpuMatND_swap_GpuMatNDR(instance: *mut c_void, m: *mut c_void);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:439
// ("cv::cuda::GpuMatND::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// clone(Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:444
// ("cv::cuda::GpuMatND::clone", vec![(pred!(const, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMatND_clone_const_StreamR(instance: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(const std::vector<Range> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:451
// ("cv::cuda::GpuMatND::operator()", vec![(pred!(const, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_cuda_GpuMatND_operator___const_const_vectorLRangeGR(instance: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMatND::operator()(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:451
// ("cv::cuda::GpuMatND::operator()", vec![(pred!(mut, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
pub fn cv_cuda_GpuMatND_operator___const_vectorLRangeGR(instance: *mut c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createGpuMatHeader(IndexArray, Range, Range)(CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:458
// ("cv::cuda::GpuMatND::createGpuMatHeader", vec![(pred!(const, ["idx", "rowRange", "colRange"], ["cv::cuda::GpuMatND::IndexArray", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMatND_createGpuMatHeader_const_IndexArray_Range_Range(instance: *const c_void, idx: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMatND::createGpuMatHeader(CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:458
// ("cv::cuda::GpuMatND::createGpuMatHeader", vec![(pred!(mut, ["idx", "rowRange", "colRange"], ["cv::cuda::GpuMatND::IndexArray", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMatND_createGpuMatHeader_IndexArray_Range_Range(instance: *mut c_void, idx: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createGpuMatHeader()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:466
// ("cv::cuda::GpuMatND::createGpuMatHeader", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_createGpuMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMatND::createGpuMatHeader() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:466
// ("cv::cuda::GpuMatND::createGpuMatHeader", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMatND_createGpuMatHeader(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(IndexArray, Range, Range)(CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:473
// ("cv::cuda::GpuMatND::operator()", vec![(pred!(const, ["idx", "rowRange", "colRange"], ["cv::cuda::GpuMatND::IndexArray", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMatND_operator___const_IndexArray_Range_Range(instance: *const c_void, idx: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::GpuMatND::operator()(CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:473
// ("cv::cuda::GpuMatND::operator()", vec![(pred!(mut, ["idx", "rowRange", "colRange"], ["cv::cuda::GpuMatND::IndexArray", "cv::Range", "cv::Range"]), _)]),
pub fn cv_cuda_GpuMatND_operator___IndexArray_Range_Range(instance: *mut c_void, idx: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator GpuMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:479
// ("cv::cuda::GpuMatND::operator cv::cuda::GpuMat", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_operator_cv_cuda_GpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// GpuMatND(const GpuMatND &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:481
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, ["unnamed"], ["const cv::cuda::GpuMatND*"]), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND_const_GpuMatNDR(unnamed: *const c_void) -> *mut c_void;
// operator=(const GpuMatND &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:482
// ("cv::cuda::GpuMatND::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::cuda::GpuMatND*"]), _)]),
pub fn cv_cuda_GpuMatND_operatorST_const_GpuMatNDR(instance: *mut c_void, unnamed: *const c_void);
// GpuMatND(GpuMatND &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:488
// ("cv::cuda::GpuMatND::GpuMatND", vec![(pred!(mut, ["unnamed"], ["cv::cuda::GpuMatND*"]), _)]),
pub fn cv_cuda_GpuMatND_GpuMatND_GpuMatNDRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(GpuMatND &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:489
// ("cv::cuda::GpuMatND::operator=", vec![(pred!(mut, ["unnamed"], ["cv::cuda::GpuMatND*"]), _)]),
pub fn cv_cuda_GpuMatND_operatorST_GpuMatNDRR(instance: *mut c_void, unnamed: *mut c_void);
// upload(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:495
// ("cv::cuda::GpuMatND::upload", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_GpuMatND_upload_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<()>);
// upload(InputArray, Stream &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:496
// ("cv::cuda::GpuMatND::upload", vec![(pred!(mut, ["src", "stream"], ["const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMatND_upload_const__InputArrayR_StreamR(instance: *mut c_void, src: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// download(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:497
// ("cv::cuda::GpuMatND::download", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_GpuMatND_download_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// download(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:498
// ("cv::cuda::GpuMatND::download", vec![(pred!(const, ["dst", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_GpuMatND_download_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// isContinuous()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:502
// ("cv::cuda::GpuMatND::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isSubmatrix()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:505
// ("cv::cuda::GpuMatND::isSubmatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_isSubmatrix_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:508
// ("cv::cuda::GpuMatND::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:511
// ("cv::cuda::GpuMatND::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:514
// ("cv::cuda::GpuMatND::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// external()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:517
// ("cv::cuda::GpuMatND::external", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_external_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getDevicePtr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:520
// ("cv::cuda::GpuMatND::getDevicePtr", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_getDevicePtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut u8>);
// total()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:523
// ("cv::cuda::GpuMatND::total", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_total_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// totalMemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:526
// ("cv::cuda::GpuMatND::totalMemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_totalMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:529
// ("cv::cuda::GpuMatND::type", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::GpuMatND::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:542
// ("cv::cuda::GpuMatND::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_propFlags_const(instance: *const c_void) -> i32;
// cv::cuda::GpuMatND::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:542
// ("cv::cuda::GpuMatND::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_GpuMatND_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::cuda::GpuMatND::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:545
// ("cv::cuda::GpuMatND::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_propDims_const(instance: *const c_void) -> i32;
// cv::cuda::GpuMatND::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:545
// ("cv::cuda::GpuMatND::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_GpuMatND_propDims_const_int(instance: *mut c_void, val: i32);
// cv::cuda::GpuMatND::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:548
// ("cv::cuda::GpuMatND::size", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_propSize_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::GpuMatND::setSize(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:548
// ("cv::cuda::GpuMatND::setSize", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMatND::SizeArray"]), _)]),
pub fn cv_cuda_GpuMatND_propSize_const_SizeArray(instance: *mut c_void, val: *const c_void);
// cv::cuda::GpuMatND::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:553
// ("cv::cuda::GpuMatND::step", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_GpuMatND_propStep_const(instance: *const c_void) -> *mut c_void;
// cv::cuda::GpuMatND::setStep(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:553
// ("cv::cuda::GpuMatND::setStep", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMatND::StepArray"]), _)]),
pub fn cv_cuda_GpuMatND_propStep_const_StepArray(instance: *mut c_void, val: *const c_void);
// cv::cuda::GpuMatND::delete() generated
// ("cv::cuda::GpuMatND::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_GpuMatND_delete(instance: *mut c_void);
// HostMem(HostMem::AllocType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:797
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["alloc_type"], ["cv::cuda::HostMem::AllocType"]), _)]),
pub fn cv_cuda_HostMem_HostMem_AllocType(alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HostMem::HostMem() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:797
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_HostMem(ocvrs_return: *mut Result<*mut c_void>);
// HostMem(const HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:799
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["m"], ["const cv::cuda::HostMem*"]), _)]),
pub fn cv_cuda_HostMem_HostMem_const_HostMemR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// HostMem(int, int, int, HostMem::AllocType)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:801
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["rows", "cols", "type", "alloc_type"], ["int", "int", "int", "cv::cuda::HostMem::AllocType"]), _)]),
pub fn cv_cuda_HostMem_HostMem_int_int_int_AllocType(rows: i32, cols: i32, typ: i32, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HostMem::HostMem(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:801
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_HostMem_HostMem_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// HostMem(Size, int, HostMem::AllocType)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:802
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["size", "type", "alloc_type"], ["cv::Size", "int", "cv::cuda::HostMem::AllocType"]), _)]),
pub fn cv_cuda_HostMem_HostMem_Size_int_AllocType(size: *const core::Size, typ: i32, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HostMem::HostMem(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:802
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_HostMem_HostMem_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// HostMem(InputArray, HostMem::AllocType)(InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:805
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["arr", "alloc_type"], ["const cv::_InputArray*", "cv::cuda::HostMem::AllocType"]), _)]),
pub fn cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(arr: *const c_void, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HostMem::HostMem(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:805
// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_HostMem_HostMem_const__InputArrayR(arr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:809
// ("cv::cuda::HostMem::operator=", vec![(pred!(mut, ["m"], ["const cv::cuda::HostMem*"]), _)]),
pub fn cv_cuda_HostMem_operatorST_const_HostMemR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result<()>);
// swap(HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:812
// ("cv::cuda::HostMem::swap", vec![(pred!(mut, ["b"], ["cv::cuda::HostMem*"]), _)]),
pub fn cv_cuda_HostMem_swap_HostMemR(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result<()>);
// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:815
// ("cv::cuda::HostMem::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:818
// ("cv::cuda::HostMem::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_HostMem_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<()>);
// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:819
// ("cv::cuda::HostMem::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_HostMem_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:823
// ("cv::cuda::HostMem::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
pub fn cv_cuda_HostMem_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HostMem::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:823
// ("cv::cuda::HostMem::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
pub fn cv_cuda_HostMem_reshape_const_int(instance: *const c_void, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:826
// ("cv::cuda::HostMem::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// createMatHeader()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:829
// ("cv::cuda::HostMem::createMatHeader", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_createMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createGpuMatHeader()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:838
// ("cv::cuda::HostMem::createGpuMatHeader", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_createGpuMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// isContinuous()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:841
// ("cv::cuda::HostMem::isContinuous", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:842
// ("cv::cuda::HostMem::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:843
// ("cv::cuda::HostMem::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:844
// ("cv::cuda::HostMem::type", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:845
// ("cv::cuda::HostMem::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:846
// ("cv::cuda::HostMem::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// step1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:847
// ("cv::cuda::HostMem::step1", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:848
// ("cv::cuda::HostMem::size", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:849
// ("cv::cuda::HostMem::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::cuda::HostMem::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:852
// ("cv::cuda::HostMem::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propFlags_const(instance: *const c_void) -> i32;
// cv::cuda::HostMem::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:852
// ("cv::cuda::HostMem::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_HostMem_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::cuda::HostMem::rows() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:853
// ("cv::cuda::HostMem::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propRows_const(instance: *const c_void) -> i32;
// cv::cuda::HostMem::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:853
// ("cv::cuda::HostMem::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_HostMem_propRows_const_int(instance: *mut c_void, val: i32);
// cv::cuda::HostMem::cols() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:853
// ("cv::cuda::HostMem::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propCols_const(instance: *const c_void) -> i32;
// cv::cuda::HostMem::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:853
// ("cv::cuda::HostMem::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_cuda_HostMem_propCols_const_int(instance: *mut c_void, val: i32);
// cv::cuda::HostMem::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:854
// ("cv::cuda::HostMem::step", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propStep_const(instance: *const c_void) -> size_t;
// cv::cuda::HostMem::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:854
// ("cv::cuda::HostMem::setStep", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_cuda_HostMem_propStep_const_size_t(instance: *mut c_void, val: size_t);
// cv::cuda::HostMem::data() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:856
// ("cv::cuda::HostMem::data", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propData_const(instance: *const c_void) -> *const u8;
// cv::cuda::HostMem::dataMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:856
// ("cv::cuda::HostMem::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_propData(instance: *mut c_void) -> *mut u8;
// cv::cuda::HostMem::setData(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:856
// ("cv::cuda::HostMem::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_cuda_HostMem_propData_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::cuda::HostMem::refcount() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:857
// ("cv::cuda::HostMem::refcount", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propRefcount_const(instance: *const c_void) -> *const i32;
// cv::cuda::HostMem::refcountMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:857
// ("cv::cuda::HostMem::refcountMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_propRefcount(instance: *mut c_void) -> *mut i32;
// cv::cuda::HostMem::setRefcount(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:857
// ("cv::cuda::HostMem::setRefcount", vec![(pred!(mut, ["val"], ["int*"]), _)]),
pub fn cv_cuda_HostMem_propRefcount_intX(instance: *mut c_void, val: *const i32);
// cv::cuda::HostMem::datastart() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:859
// ("cv::cuda::HostMem::datastart", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propDatastart_const(instance: *const c_void) -> *const u8;
// cv::cuda::HostMem::datastartMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:859
// ("cv::cuda::HostMem::datastartMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_propDatastart(instance: *mut c_void) -> *mut u8;
// cv::cuda::HostMem::setDatastart(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:859
// ("cv::cuda::HostMem::setDatastart", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_cuda_HostMem_propDatastart_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::cuda::HostMem::dataend() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:860
// ("cv::cuda::HostMem::dataend", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propDataend_const(instance: *const c_void) -> *const u8;
// cv::cuda::HostMem::alloc_type() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:862
// ("cv::cuda::HostMem::alloc_type", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HostMem_propAlloc_type_const(instance: *const c_void, ocvrs_return: *mut core::HostMem_AllocType);
// cv::cuda::HostMem::setAlloc_type(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:862
// ("cv::cuda::HostMem::setAlloc_type", vec![(pred!(mut, ["val"], ["const cv::cuda::HostMem::AllocType"]), _)]),
pub fn cv_cuda_HostMem_propAlloc_type_const_AllocType(instance: *mut c_void, val: core::HostMem_AllocType);
// cv::cuda::HostMem::delete() generated
// ("cv::cuda::HostMem::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HostMem_delete(instance: *mut c_void);
// Stream()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:917
// ("cv::cuda::Stream::Stream", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Stream_Stream(ocvrs_return: *mut Result<*mut c_void>);
// Stream(const Ptr<GpuMat::Allocator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:920
// ("cv::cuda::Stream::Stream", vec![(pred!(mut, ["allocator"], ["const cv::Ptr<cv::cuda::GpuMat::Allocator>*"]), _)]),
pub fn cv_cuda_Stream_Stream_const_PtrLAllocatorGR(allocator: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Stream(const size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:932
// ("cv::cuda::Stream::Stream", vec![(pred!(mut, ["cudaFlags"], ["const size_t"]), _)]),
pub fn cv_cuda_Stream_Stream_const_size_t(cuda_flags: size_t, ocvrs_return: *mut Result<*mut c_void>);
// queryIfComplete()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:936
// ("cv::cuda::Stream::queryIfComplete", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_Stream_queryIfComplete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// waitForCompletion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:940
// ("cv::cuda::Stream::waitForCompletion", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Stream_waitForCompletion(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// waitEvent(const Event &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:944
// ("cv::cuda::Stream::waitEvent", vec![(pred!(mut, ["event"], ["const cv::cuda::Event*"]), _)]),
pub fn cv_cuda_Stream_waitEvent_const_EventR(instance: *mut c_void, event: *const c_void, ocvrs_return: *mut Result<()>);
// enqueueHostCallback(StreamCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:954
// ("cv::cuda::Stream::enqueueHostCallback", vec![(pred!(mut, ["callback", "userData"], ["cv::cuda::Stream::StreamCallback", "void*"]), _)]),
pub fn cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, user_data: *mut c_void, ocvrs_return: *mut Result<()>);
// Null()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:957
// ("cv::cuda::Stream::Null", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Stream_Null(ocvrs_return: *mut Result<*mut c_void>);
// cudaPtr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:963
// ("cv::cuda::Stream::cudaPtr", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_Stream_cudaPtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::Stream::delete() generated
// ("cv::cuda::Stream::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Stream_delete(instance: *mut c_void);
// builtWith(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1092
// ("cv::cuda::TargetArchs::builtWith", vec![(pred!(mut, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
pub fn cv_cuda_TargetArchs_builtWith_FeatureSet(feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
// has(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1100
// ("cv::cuda::TargetArchs::has", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_has_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1101
// ("cv::cuda::TargetArchs::hasPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasBin(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1102
// ("cv::cuda::TargetArchs::hasBin", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasBin_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasEqualOrLessPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1104
// ("cv::cuda::TargetArchs::hasEqualOrLessPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasEqualOrGreater(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1105
// ("cv::cuda::TargetArchs::hasEqualOrGreater", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasEqualOrGreater_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasEqualOrGreaterPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1106
// ("cv::cuda::TargetArchs::hasEqualOrGreaterPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// hasEqualOrGreaterBin(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cuda.hpp:1107
// ("cv::cuda::TargetArchs::hasEqualOrGreaterBin", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
pub fn cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
// cv::cuda::TargetArchs::defaultNew() generated
// ("cv::cuda::TargetArchs::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_TargetArchs_defaultNew_const() -> *mut c_void;
// cv::cuda::TargetArchs::delete() generated
// ("cv::cuda::TargetArchs::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_TargetArchs_delete(instance: *mut c_void);
// cv::detail::CheckContext::implicitClone() generated
// ("cv::detail::CheckContext::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::defaultNew() generated
// ("cv::detail::CheckContext::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_defaultNew_const() -> *mut c_void;
// cv::detail::CheckContext::func() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:42
// ("cv::detail::CheckContext::func", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propFunc_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::file() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:43
// ("cv::detail::CheckContext::file", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propFile_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::line() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:44
// ("cv::detail::CheckContext::line", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propLine_const(instance: *const c_void) -> i32;
// cv::detail::CheckContext::setLine(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:44
// ("cv::detail::CheckContext::setLine", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_detail_CheckContext_propLine_const_int(instance: *mut c_void, val: i32);
// cv::detail::CheckContext::testOp() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:45
// ("cv::detail::CheckContext::testOp", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propTestOp_const(instance: *const c_void, ocvrs_return: *mut core::Detail_TestOp);
// cv::detail::CheckContext::setTestOp(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:45
// ("cv::detail::CheckContext::setTestOp", vec![(pred!(mut, ["val"], ["const cv::detail::TestOp"]), _)]),
pub fn cv_detail_CheckContext_propTestOp_const_TestOp(instance: *mut c_void, val: core::Detail_TestOp);
// cv::detail::CheckContext::message() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:46
// ("cv::detail::CheckContext::message", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propMessage_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::p1_str() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:47
// ("cv::detail::CheckContext::p1_str", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propP1_str_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::p2_str() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/check.hpp:48
// ("cv::detail::CheckContext::p2_str", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_CheckContext_propP2_str_const(instance: *const c_void) -> *mut c_void;
// cv::detail::CheckContext::delete() generated
// ("cv::detail::CheckContext::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_CheckContext_delete(instance: *mut c_void);
// hfloat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:850
// ("cv::hfloat::hfloat", vec![(pred!(mut, [], []), _)]),
pub fn cv_hfloat_hfloat(ocvrs_return: *mut core::hfloat);
// operator float()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:851
// ("cv::hfloat::operator float", vec![(pred!(const, [], []), _)]),
pub fn cv_hfloat_operator_float_const(instance: *const core::hfloat, ocvrs_return: *mut Result<f32>);
// hfloat(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/cvdef.h:853
// ("cv::hfloat::hfloat", vec![(pred!(mut, ["x"], ["float"]), _)]),
pub fn cv_hfloat_hfloat_float(x: f32, ocvrs_return: *mut Result<core::hfloat>);
// NodeData(const char *, const char *, int, void *, bool, cv::instr::TYPE, cv::instr::IMPL)(InString, InString, Primitive, Indirect, Primitive, Enum, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:72
// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, ["funName", "fileName", "lineNum", "retAddress", "alwaysExpand", "instrType", "implType"], ["const char*", "const char*", "int", "void*", "bool", "cv::instr::TYPE", "cv::instr::IMPL"]), _)]),
pub fn cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(fun_name: *const c_char, file_name: *const c_char, line_num: i32, ret_address: *mut c_void, always_expand: bool, instr_type: core::TYPE, impl_type: core::IMPL, ocvrs_return: *mut Result<*mut c_void>);
// cv::instr::NodeData::NodeData() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:72
// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_NodeData_NodeData(ocvrs_return: *mut Result<*mut c_void>);
// NodeData(NodeData &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:73
// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, ["ref"], ["cv::instr::NodeData*"]), _)]),
pub fn cv_instr_NodeData_NodeData_NodeDataR(ref_: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const NodeData &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:75
// ("cv::instr::NodeData::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::instr::NodeData*"]), _)]),
pub fn cv_instr_NodeData_operatorST_const_NodeDataR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// getTotalMs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:92
// ("cv::instr::NodeData::getTotalMs", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_getTotalMs_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getMeanMs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:93
// ("cv::instr::NodeData::getMeanMs", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_getMeanMs_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::instr::NodeData::m_funName() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:77
// ("cv::instr::NodeData::m_funName", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_funName_const(instance: *const c_void) -> *mut c_void;
// cv::instr::NodeData::setM_funName(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:77
// ("cv::instr::NodeData::setM_funName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_instr_NodeData_propM_funName_const_String(instance: *mut c_void, val: *const c_char);
// cv::instr::NodeData::m_instrType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:78
// ("cv::instr::NodeData::m_instrType", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_instrType_const(instance: *const c_void, ocvrs_return: *mut core::TYPE);
// cv::instr::NodeData::setM_instrType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:78
// ("cv::instr::NodeData::setM_instrType", vec![(pred!(mut, ["val"], ["const cv::instr::TYPE"]), _)]),
pub fn cv_instr_NodeData_propM_instrType_const_TYPE(instance: *mut c_void, val: core::TYPE);
// cv::instr::NodeData::m_implType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:79
// ("cv::instr::NodeData::m_implType", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_implType_const(instance: *const c_void, ocvrs_return: *mut core::IMPL);
// cv::instr::NodeData::setM_implType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:79
// ("cv::instr::NodeData::setM_implType", vec![(pred!(mut, ["val"], ["const cv::instr::IMPL"]), _)]),
pub fn cv_instr_NodeData_propM_implType_const_IMPL(instance: *mut c_void, val: core::IMPL);
// cv::instr::NodeData::m_fileName() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:80
// ("cv::instr::NodeData::m_fileName", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_fileName_const(instance: *const c_void) -> *mut c_void;
// cv::instr::NodeData::m_lineNum() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:81
// ("cv::instr::NodeData::m_lineNum", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_lineNum_const(instance: *const c_void) -> i32;
// cv::instr::NodeData::setM_lineNum(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:81
// ("cv::instr::NodeData::setM_lineNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_instr_NodeData_propM_lineNum_const_int(instance: *mut c_void, val: i32);
// cv::instr::NodeData::m_retAddress() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:82
// ("cv::instr::NodeData::m_retAddress", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_NodeData_propM_retAddress(instance: *mut c_void) -> *mut c_void;
// cv::instr::NodeData::setM_retAddress(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:82
// ("cv::instr::NodeData::setM_retAddress", vec![(pred!(mut, ["val"], ["void*"]), _)]),
pub fn cv_instr_NodeData_propM_retAddress_voidX(instance: *mut c_void, val: *const c_void);
// cv::instr::NodeData::m_alwaysExpand() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:83
// ("cv::instr::NodeData::m_alwaysExpand", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_alwaysExpand_const(instance: *const c_void) -> bool;
// cv::instr::NodeData::setM_alwaysExpand(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:83
// ("cv::instr::NodeData::setM_alwaysExpand", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_instr_NodeData_propM_alwaysExpand_const_bool(instance: *mut c_void, val: bool);
// cv::instr::NodeData::m_funError() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:84
// ("cv::instr::NodeData::m_funError", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_funError_const(instance: *const c_void) -> bool;
// cv::instr::NodeData::setM_funError(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:84
// ("cv::instr::NodeData::setM_funError", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_instr_NodeData_propM_funError_const_bool(instance: *mut c_void, val: bool);
// cv::instr::NodeData::m_counter() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:86
// ("cv::instr::NodeData::m_counter", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_counter_const(instance: *const c_void) -> i32;
// cv::instr::NodeData::setM_counter(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:86
// ("cv::instr::NodeData::setM_counter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_instr_NodeData_propM_counter_const_int(instance: *mut c_void, val: i32);
// cv::instr::NodeData::m_ticksTotal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:87
// ("cv::instr::NodeData::m_ticksTotal", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_ticksTotal_const(instance: *const c_void) -> u64;
// cv::instr::NodeData::setM_ticksTotal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:87
// ("cv::instr::NodeData::setM_ticksTotal", vec![(pred!(mut, ["val"], ["const uint64_t"]), _)]),
pub fn cv_instr_NodeData_propM_ticksTotal_const_uint64_t(instance: *mut c_void, val: u64);
// cv::instr::NodeData::m_threads() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:89
// ("cv::instr::NodeData::m_threads", vec![(pred!(const, [], []), _)]),
pub fn cv_instr_NodeData_propM_threads_const(instance: *const c_void) -> i32;
// cv::instr::NodeData::setM_threads(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/instrumentation.hpp:89
// ("cv::instr::NodeData::setM_threads", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_instr_NodeData_propM_threads_const_int(instance: *mut c_void, val: i32);
// cv::instr::NodeData::delete() generated
// ("cv::instr::NodeData::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_instr_NodeData_delete(instance: *mut c_void);
// WriteStructContext(FileStorage &, const String &, int, const String &)(TraitClass, InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:766
// ("cv::internal::WriteStructContext::WriteStructContext", vec![(pred!(mut, ["_fs", "name", "flags", "typeName"], ["cv::FileStorage*", "const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(_fs: *mut c_void, name: *const c_char, flags: i32, type_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::internal::WriteStructContext::WriteStructContext(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/persistence.hpp:766
// ("cv::internal::WriteStructContext::WriteStructContext", vec![(pred!(mut, ["_fs", "name", "flags"], ["cv::FileStorage*", "const cv::String*", "int"]), _)]),
pub fn cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int(_fs: *mut c_void, name: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::internal::WriteStructContext::delete() generated
// ("cv::internal::WriteStructContext::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_internal_WriteStructContext_delete(instance: *mut c_void);
// Context()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:261
// ("cv::ocl::Context::Context", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_Context() -> *mut c_void;
// Context(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:262
// ("cv::ocl::Context::Context", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
pub fn cv_ocl_Context_Context_int(dtype: i32, ocvrs_return: *mut Result<*mut c_void>);
// Context(const Context &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:264
// ("cv::ocl::Context::Context", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
pub fn cv_ocl_Context_Context_const_ContextR(c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Context &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:265
// ("cv::ocl::Context::operator=", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
pub fn cv_ocl_Context_operatorST_const_ContextR(instance: *mut c_void, c: *const c_void, ocvrs_return: *mut Result<()>);
// Context(Context &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:266
// ("cv::ocl::Context::Context", vec![(pred!(mut, ["c"], ["cv::ocl::Context*"]), _)]),
pub fn cv_ocl_Context_Context_ContextRR(c: *mut c_void) -> *mut c_void;
// operator=(Context &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:267
// ("cv::ocl::Context::operator=", vec![(pred!(mut, ["c"], ["cv::ocl::Context*"]), _)]),
pub fn cv_ocl_Context_operatorST_ContextRR(instance: *mut c_void, c: *mut c_void);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:270
// ("cv::ocl::Context::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_create(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:272
// ("cv::ocl::Context::create", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
pub fn cv_ocl_Context_create_int(instance: *mut c_void, dtype: i32, ocvrs_return: *mut Result<bool>);
// ndevices()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:274
// ("cv::ocl::Context::ndevices", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Context_ndevices_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// device(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:275
// ("cv::ocl::Context::device", vec![(pred!(const, ["idx"], ["size_t"]), _)]),
pub fn cv_ocl_Context_device_const_size_t(instance: *const c_void, idx: size_t, ocvrs_return: *mut Result<*mut c_void>);
// getProg(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:276
// ("cv::ocl::Context::getProg", vec![(pred!(mut, ["prog", "buildopt", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
pub fn cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(instance: *mut c_void, prog: *const c_void, buildopt: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// unloadProg(Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:278
// ("cv::ocl::Context::unloadProg", vec![(pred!(mut, ["prog"], ["cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Context_unloadProg_ProgramR(instance: *mut c_void, prog: *mut c_void, ocvrs_return: *mut Result<()>);
// getDefault()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:282
// ("cv::ocl::Context::getDefault", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_getDefault(ocvrs_return: *mut Result<*mut c_void>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:285
// ("cv::ocl::Context::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Context_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getOpenCLContextProperty(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:292
// ("cv::ocl::Context::getOpenCLContextProperty", vec![(pred!(const, ["propertyId"], ["int"]), _)]),
pub fn cv_ocl_Context_getOpenCLContextProperty_const_int(instance: *const c_void, property_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// useSVM()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:294
// ("cv::ocl::Context::useSVM", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Context_useSVM_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseSVM(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:295
// ("cv::ocl::Context::setUseSVM", vec![(pred!(mut, ["enabled"], ["bool"]), _)]),
pub fn cv_ocl_Context_setUseSVM_bool(instance: *mut c_void, enabled: bool, ocvrs_return: *mut Result<()>);
// fromHandle(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:300
// ("cv::ocl::Context::fromHandle", vec![(pred!(mut, ["context"], ["void*"]), _)]),
pub fn cv_ocl_Context_fromHandle_voidX(context: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// fromDevice(const ocl::Device &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:301
// ("cv::ocl::Context::fromDevice", vec![(pred!(mut, ["device"], ["const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Context_fromDevice_const_DeviceR(device: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:302
// ("cv::ocl::Context::create", vec![(pred!(mut, ["configuration"], ["const std::string*"]), _)]),
pub fn cv_ocl_Context_create_const_stringR(configuration: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:304
// ("cv::ocl::Context::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:323
// ("cv::ocl::Context::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Context_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Context::delete() generated
// ("cv::ocl::Context::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_delete(instance: *mut c_void);
// cv::ocl::Context::UserContext::defaultNew() generated
// ("cv::ocl::Context::UserContext::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Context_UserContext_defaultNew_const() -> *mut c_void;
// cv::ocl::Context::UserContext::delete() generated
// ("cv::ocl::Context::UserContext::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Context_UserContext_delete(instance: *mut c_void);
// Device()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:75
// ("cv::ocl::Device::Device", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Device_Device() -> *mut c_void;
// Device(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:76
// ("cv::ocl::Device::Device", vec![(pred!(mut, ["d"], ["void*"]), _)]),
pub fn cv_ocl_Device_Device_voidX(d: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Device(const Device &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:77
// ("cv::ocl::Device::Device", vec![(pred!(mut, ["d"], ["const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Device_Device_const_DeviceR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Device &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:78
// ("cv::ocl::Device::operator=", vec![(pred!(mut, ["d"], ["const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Device_operatorST_const_DeviceR(instance: *mut c_void, d: *const c_void, ocvrs_return: *mut Result<()>);
// Device(Device &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:79
// ("cv::ocl::Device::Device", vec![(pred!(mut, ["d"], ["cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Device_Device_DeviceRR(d: *mut c_void) -> *mut c_void;
// operator=(Device &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:80
// ("cv::ocl::Device::operator=", vec![(pred!(mut, ["d"], ["cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Device_operatorST_DeviceRR(instance: *mut c_void, d: *mut c_void);
// set(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:83
// ("cv::ocl::Device::set", vec![(pred!(mut, ["d"], ["void*"]), _)]),
pub fn cv_ocl_Device_set_voidX(instance: *mut c_void, d: *mut c_void, ocvrs_return: *mut Result<()>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:96
// ("cv::ocl::Device::name", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// extensions()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:97
// ("cv::ocl::Device::extensions", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_extensions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// isExtensionSupported(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:98
// ("cv::ocl::Device::isExtensionSupported", vec![(pred!(const, ["extensionName"], ["const cv::String*"]), _)]),
pub fn cv_ocl_Device_isExtensionSupported_const_const_StringR(instance: *const c_void, extension_name: *const c_char, ocvrs_return: *mut Result<bool>);
// version()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:99
// ("cv::ocl::Device::version", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// vendorName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:100
// ("cv::ocl::Device::vendorName", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_vendorName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// OpenCL_C_Version()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:101
// ("cv::ocl::Device::OpenCL_C_Version", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_OpenCL_C_Version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// OpenCLVersion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:102
// ("cv::ocl::Device::OpenCLVersion", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_OpenCLVersion_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// deviceVersionMajor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:103
// ("cv::ocl::Device::deviceVersionMajor", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_deviceVersionMajor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// deviceVersionMinor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:104
// ("cv::ocl::Device::deviceVersionMinor", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_deviceVersionMinor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// driverVersion()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:105
// ("cv::ocl::Device::driverVersion", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_driverVersion_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:106
// ("cv::ocl::Device::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:108
// ("cv::ocl::Device::type", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// addressBits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:110
// ("cv::ocl::Device::addressBits", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_addressBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// available()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:111
// ("cv::ocl::Device::available", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_available_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// compilerAvailable()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:112
// ("cv::ocl::Device::compilerAvailable", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_compilerAvailable_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// linkerAvailable()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:113
// ("cv::ocl::Device::linkerAvailable", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_linkerAvailable_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// doubleFPConfig()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:126
// ("cv::ocl::Device::doubleFPConfig", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_doubleFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// singleFPConfig()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:127
// ("cv::ocl::Device::singleFPConfig", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_singleFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// halfFPConfig()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:128
// ("cv::ocl::Device::halfFPConfig", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_halfFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// hasFP64()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:131
// ("cv::ocl::Device::hasFP64", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_hasFP64_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// hasFP16()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:133
// ("cv::ocl::Device::hasFP16", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_hasFP16_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// endianLittle()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:135
// ("cv::ocl::Device::endianLittle", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_endianLittle_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// errorCorrectionSupport()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:136
// ("cv::ocl::Device::errorCorrectionSupport", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_errorCorrectionSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// executionCapabilities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:143
// ("cv::ocl::Device::executionCapabilities", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_executionCapabilities_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// globalMemCacheSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:145
// ("cv::ocl::Device::globalMemCacheSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_globalMemCacheSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// globalMemCacheType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:153
// ("cv::ocl::Device::globalMemCacheType", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_globalMemCacheType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// globalMemCacheLineSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:154
// ("cv::ocl::Device::globalMemCacheLineSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_globalMemCacheLineSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// globalMemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:155
// ("cv::ocl::Device::globalMemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_globalMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// localMemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:157
// ("cv::ocl::Device::localMemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_localMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// localMemType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:164
// ("cv::ocl::Device::localMemType", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_localMemType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// hostUnifiedMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:165
// ("cv::ocl::Device::hostUnifiedMemory", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_hostUnifiedMemory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// imageSupport()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:167
// ("cv::ocl::Device::imageSupport", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imageSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// imageFromBufferSupport()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:169
// ("cv::ocl::Device::imageFromBufferSupport", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imageFromBufferSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// imagePitchAlignment()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:170
// ("cv::ocl::Device::imagePitchAlignment", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imagePitchAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
// imageBaseAddressAlignment()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:171
// ("cv::ocl::Device::imageBaseAddressAlignment", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imageBaseAddressAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
// intelSubgroupsSupport()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:174
// ("cv::ocl::Device::intelSubgroupsSupport", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_intelSubgroupsSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// image2DMaxWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:176
// ("cv::ocl::Device::image2DMaxWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_image2DMaxWidth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// image2DMaxHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:177
// ("cv::ocl::Device::image2DMaxHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_image2DMaxHeight_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// image3DMaxWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:179
// ("cv::ocl::Device::image3DMaxWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_image3DMaxWidth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// image3DMaxHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:180
// ("cv::ocl::Device::image3DMaxHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_image3DMaxHeight_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// image3DMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:181
// ("cv::ocl::Device::image3DMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_image3DMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// imageMaxBufferSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:183
// ("cv::ocl::Device::imageMaxBufferSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imageMaxBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// imageMaxArraySize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:184
// ("cv::ocl::Device::imageMaxArraySize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_imageMaxArraySize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// vendorID()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:193
// ("cv::ocl::Device::vendorID", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_vendorID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// isAMD()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:198
// ("cv::ocl::Device::isAMD", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_isAMD_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isIntel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:199
// ("cv::ocl::Device::isIntel", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_isIntel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isNVidia()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:200
// ("cv::ocl::Device::isNVidia", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_isNVidia_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// maxClockFrequency()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:202
// ("cv::ocl::Device::maxClockFrequency", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxClockFrequency_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxComputeUnits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:203
// ("cv::ocl::Device::maxComputeUnits", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxComputeUnits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxConstantArgs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:204
// ("cv::ocl::Device::maxConstantArgs", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxConstantArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxConstantBufferSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:205
// ("cv::ocl::Device::maxConstantBufferSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxConstantBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// maxMemAllocSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:207
// ("cv::ocl::Device::maxMemAllocSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxMemAllocSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// maxParameterSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:208
// ("cv::ocl::Device::maxParameterSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxParameterSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// maxReadImageArgs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:210
// ("cv::ocl::Device::maxReadImageArgs", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxReadImageArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxWriteImageArgs()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:211
// ("cv::ocl::Device::maxWriteImageArgs", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxWriteImageArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxSamplers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:212
// ("cv::ocl::Device::maxSamplers", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxSamplers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxWorkGroupSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:214
// ("cv::ocl::Device::maxWorkGroupSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxWorkGroupSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// maxWorkItemDims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:215
// ("cv::ocl::Device::maxWorkItemDims", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_maxWorkItemDims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// maxWorkItemSizes(size_t *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:216
// ("cv::ocl::Device::maxWorkItemSizes", vec![(pred!(const, ["unnamed"], ["size_t*"]), _)]),
pub fn cv_ocl_Device_maxWorkItemSizes_const_size_tX(instance: *const c_void, unnamed: *mut size_t, ocvrs_return: *mut Result<()>);
// memBaseAddrAlign()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:218
// ("cv::ocl::Device::memBaseAddrAlign", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_memBaseAddrAlign_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthChar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:220
// ("cv::ocl::Device::nativeVectorWidthChar", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthChar_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthShort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:221
// ("cv::ocl::Device::nativeVectorWidthShort", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthShort_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthInt()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:222
// ("cv::ocl::Device::nativeVectorWidthInt", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthInt_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthLong()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:223
// ("cv::ocl::Device::nativeVectorWidthLong", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthLong_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthFloat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:224
// ("cv::ocl::Device::nativeVectorWidthFloat", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthFloat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthDouble()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:225
// ("cv::ocl::Device::nativeVectorWidthDouble", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthDouble_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// nativeVectorWidthHalf()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:226
// ("cv::ocl::Device::nativeVectorWidthHalf", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_nativeVectorWidthHalf_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthChar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:228
// ("cv::ocl::Device::preferredVectorWidthChar", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthChar_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthShort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:229
// ("cv::ocl::Device::preferredVectorWidthShort", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthShort_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthInt()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:230
// ("cv::ocl::Device::preferredVectorWidthInt", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthInt_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthLong()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:231
// ("cv::ocl::Device::preferredVectorWidthLong", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthLong_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthFloat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:232
// ("cv::ocl::Device::preferredVectorWidthFloat", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthFloat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthDouble()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:233
// ("cv::ocl::Device::preferredVectorWidthDouble", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthDouble_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// preferredVectorWidthHalf()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:234
// ("cv::ocl::Device::preferredVectorWidthHalf", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_preferredVectorWidthHalf_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// printfBufferSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:236
// ("cv::ocl::Device::printfBufferSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_printfBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// profilingTimerResolution()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:237
// ("cv::ocl::Device::profilingTimerResolution", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_profilingTimerResolution_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// getDefault()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:239
// ("cv::ocl::Device::getDefault", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Device_getDefault(ocvrs_return: *mut Result<*mut c_void>);
// fromHandle(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:248
// ("cv::ocl::Device::fromHandle", vec![(pred!(mut, ["d"], ["void*"]), _)]),
pub fn cv_ocl_Device_fromHandle_voidX(d: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:252
// ("cv::ocl::Device::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Device::implicitClone() generated
// ("cv::ocl::Device::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Device_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::ocl::Device::delete() generated
// ("cv::ocl::Device::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Device_delete(instance: *mut c_void);
// Image2D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:743
// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Image2D_Image2D() -> *mut c_void;
// Image2D(const UMat &, bool, bool)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:751
// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["src", "norm", "alias"], ["const cv::UMat*", "bool", "bool"]), _)]),
pub fn cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(src: *const c_void, norm: bool, alias: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Image2D::Image2D(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:751
// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["src"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_Image2D_Image2D_const_UMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Image2D(const Image2D &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:752
// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["i"], ["const cv::ocl::Image2D*"]), _)]),
pub fn cv_ocl_Image2D_Image2D_const_Image2DR(i: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Image2D &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:755
// ("cv::ocl::Image2D::operator=", vec![(pred!(mut, ["i"], ["const cv::ocl::Image2D*"]), _)]),
pub fn cv_ocl_Image2D_operatorST_const_Image2DR(instance: *mut c_void, i: *const c_void, ocvrs_return: *mut Result<()>);
// Image2D(Image2D &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:756
// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["unnamed"], ["cv::ocl::Image2D*"]), _)]),
pub fn cv_ocl_Image2D_Image2D_Image2DRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(Image2D &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:757
// ("cv::ocl::Image2D::operator=", vec![(pred!(mut, ["unnamed"], ["cv::ocl::Image2D*"]), _)]),
pub fn cv_ocl_Image2D_operatorST_Image2DRR(instance: *mut c_void, unnamed: *mut c_void);
// canCreateAlias(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:762
// ("cv::ocl::Image2D::canCreateAlias", vec![(pred!(mut, ["u"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_Image2D_canCreateAlias_const_UMatR(u: *const c_void, ocvrs_return: *mut Result<bool>);
// isFormatSupported(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:766
// ("cv::ocl::Image2D::isFormatSupported", vec![(pred!(mut, ["depth", "cn", "norm"], ["int", "int", "bool"]), _)]),
pub fn cv_ocl_Image2D_isFormatSupported_int_int_bool(depth: i32, cn: i32, norm: bool, ocvrs_return: *mut Result<bool>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:768
// ("cv::ocl::Image2D::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Image2D_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Image2D::delete() generated
// ("cv::ocl::Image2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Image2D_delete(instance: *mut c_void);
// Kernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:459
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Kernel_Kernel() -> *mut c_void;
// Kernel(const char *, const Program &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:460
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(kname: *const c_char, prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Kernel(const char *, const ProgramSource &, const String &, String *)(InString, TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:461
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog", "buildopts", "errmsg"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
pub fn cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(kname: *const c_char, prog: *const c_void, buildopts: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Kernel::Kernel(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:461
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::ProgramSource*"]), _)]),
pub fn cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR(kname: *const c_char, prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Kernel(const Kernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:464
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["k"], ["const cv::ocl::Kernel*"]), _)]),
pub fn cv_ocl_Kernel_Kernel_const_KernelR(k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Kernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:465
// ("cv::ocl::Kernel::operator=", vec![(pred!(mut, ["k"], ["const cv::ocl::Kernel*"]), _)]),
pub fn cv_ocl_Kernel_operatorST_const_KernelR(instance: *mut c_void, k: *const c_void, ocvrs_return: *mut Result<()>);
// Kernel(Kernel &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:466
// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["k"], ["cv::ocl::Kernel*"]), _)]),
pub fn cv_ocl_Kernel_Kernel_KernelRR(k: *mut c_void) -> *mut c_void;
// operator=(Kernel &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:467
// ("cv::ocl::Kernel::operator=", vec![(pred!(mut, ["k"], ["cv::ocl::Kernel*"]), _)]),
pub fn cv_ocl_Kernel_operatorST_KernelRR(instance: *mut c_void, k: *mut c_void);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:469
// ("cv::ocl::Kernel::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Kernel_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// create(const char *, const Program &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:470
// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Kernel_create_const_charX_const_ProgramR(instance: *mut c_void, kname: *const c_char, prog: *const c_void, ocvrs_return: *mut Result<bool>);
// create(const char *, const ProgramSource &, const String &, String *)(InString, TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:471
// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog", "buildopts", "errmsg"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
pub fn cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(instance: *mut c_void, kname: *const c_char, prog: *const c_void, buildopts: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Kernel::create(InString, TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:471
// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog", "buildopts"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*"]), _)]),
pub fn cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR(instance: *mut c_void, kname: *const c_char, prog: *const c_void, buildopts: *const c_char, ocvrs_return: *mut Result<bool>);
// set(int, const void *, size_t)(Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:474
// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "value", "sz"], ["int", "const void*", "size_t"]), _)]),
pub fn cv_ocl_Kernel_set_int_const_voidX_size_t(instance: *mut c_void, i: i32, value: *const c_void, sz: size_t, ocvrs_return: *mut Result<i32>);
// set(int, const Image2D &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:475
// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "image2D"], ["int", "const cv::ocl::Image2D*"]), _)]),
pub fn cv_ocl_Kernel_set_int_const_Image2DR(instance: *mut c_void, i: i32, image_2d: *const c_void, ocvrs_return: *mut Result<i32>);
// set(int, const UMat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:476
// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "m"], ["int", "const cv::UMat*"]), _)]),
pub fn cv_ocl_Kernel_set_int_const_UMatR(instance: *mut c_void, i: i32, m: *const c_void, ocvrs_return: *mut Result<i32>);
// set(int, const KernelArg &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:477
// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "arg"], ["int", "const cv::ocl::KernelArg*"]), _)]),
pub fn cv_ocl_Kernel_set_int_const_KernelArgR(instance: *mut c_void, i: i32, arg: *const c_void, ocvrs_return: *mut Result<i32>);
// run(int, size_t *, size_t *, bool, const Queue &)(Primitive, VariableArray, VariableArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:515
// ("cv::ocl::Kernel::run", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync", "q"], ["int", "size_t*", "size_t*", "bool", "const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Kernel::run(Primitive, VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:515
// ("cv::ocl::Kernel::run", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync"], ["int", "size_t*", "size_t*", "bool"]), _)]),
pub fn cv_ocl_Kernel_run_int_size_tX_size_tX_bool(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, ocvrs_return: *mut Result<bool>);
// run_(int, size_t *, size_t *, bool, const Queue &)(Primitive, VariableArray, VariableArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:526
// ("cv::ocl::Kernel::run_", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync", "q"], ["int", "size_t*", "size_t*", "bool", "const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Kernel_run__int_size_tX_size_tX_bool_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Kernel::run_(Primitive, VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:526
// ("cv::ocl::Kernel::run_", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync"], ["int", "size_t*", "size_t*", "bool"]), _)]),
pub fn cv_ocl_Kernel_run__int_size_tX_size_tX_bool(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, ocvrs_return: *mut Result<bool>);
// runTask(bool, const Queue &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:528
// ("cv::ocl::Kernel::runTask", vec![(pred!(mut, ["sync", "q"], ["bool", "const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Kernel_runTask_bool_const_QueueR(instance: *mut c_void, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Kernel::runTask(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:528
// ("cv::ocl::Kernel::runTask", vec![(pred!(mut, ["sync"], ["bool"]), _)]),
pub fn cv_ocl_Kernel_runTask_bool(instance: *mut c_void, sync: bool, ocvrs_return: *mut Result<bool>);
// runProfiling(int, size_t *, size_t *, const Queue &)(Primitive, VariableArray, VariableArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:535
// ("cv::ocl::Kernel::runProfiling", vec![(pred!(mut, ["dims", "globalsize", "localsize", "q"], ["int", "size_t*", "size_t*", "const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, q: *const c_void, ocvrs_return: *mut Result<i64>);
// cv::ocl::Kernel::runProfiling(Primitive, VariableArray, VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:535
// ("cv::ocl::Kernel::runProfiling", vec![(pred!(mut, ["dims", "globalsize", "localsize"], ["int", "size_t*", "size_t*"]), _)]),
pub fn cv_ocl_Kernel_runProfiling_int_size_tX_size_tX(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, ocvrs_return: *mut Result<i64>);
// workGroupSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:537
// ("cv::ocl::Kernel::workGroupSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Kernel_workGroupSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// preferedWorkGroupSizeMultiple()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:538
// ("cv::ocl::Kernel::preferedWorkGroupSizeMultiple", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// compileWorkGroupSize(size_t *)(VariableArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:539
// ("cv::ocl::Kernel::compileWorkGroupSize", vec![(pred!(const, ["wsz"], ["size_t*"]), _)]),
pub fn cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(instance: *const c_void, wsz: *mut size_t, ocvrs_return: *mut Result<bool>);
// localMemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:540
// ("cv::ocl::Kernel::localMemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Kernel_localMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:542
// ("cv::ocl::Kernel::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Kernel_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Kernel::delete() generated
// ("cv::ocl::Kernel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Kernel_delete(instance: *mut c_void);
// KernelArg(int, UMat *, int, int, const void *, size_t)(Primitive, TraitClass, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:421
// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, ["_flags", "_m", "wscale", "iwscale", "_obj", "_sz"], ["int", "cv::UMat*", "int", "int", "const void*", "size_t"]), _)]),
pub fn cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(_flags: i32, _m: *mut c_void, wscale: i32, iwscale: i32, _obj: *const c_void, _sz: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::KernelArg(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:421
// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, ["_flags", "_m"], ["int", "cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_KernelArg_int_UMatX(_flags: i32, _m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// KernelArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:422
// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_KernelArg_KernelArg() -> *mut c_void;
// Local(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:424
// ("cv::ocl::KernelArg::Local", vec![(pred!(mut, ["localMemSize"], ["size_t"]), _)]),
pub fn cv_ocl_KernelArg_Local_size_t(local_mem_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// PtrWriteOnly(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:426
// ("cv::ocl::KernelArg::PtrWriteOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// PtrReadOnly(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:428
// ("cv::ocl::KernelArg::PtrReadOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_PtrReadOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// PtrReadWrite(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:430
// ("cv::ocl::KernelArg::PtrReadWrite", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_PtrReadWrite_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// ReadWrite(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:432
// ("cv::ocl::KernelArg::ReadWrite", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::ReadWrite(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:432
// ("cv::ocl::KernelArg::ReadWrite", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_ReadWrite_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// ReadWriteNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:434
// ("cv::ocl::KernelArg::ReadWriteNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::ReadWriteNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:434
// ("cv::ocl::KernelArg::ReadWriteNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// ReadOnly(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:436
// ("cv::ocl::KernelArg::ReadOnly", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::ReadOnly(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:436
// ("cv::ocl::KernelArg::ReadOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_ReadOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WriteOnly(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:438
// ("cv::ocl::KernelArg::WriteOnly", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::WriteOnly(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:438
// ("cv::ocl::KernelArg::WriteOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_WriteOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// ReadOnlyNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:440
// ("cv::ocl::KernelArg::ReadOnlyNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::ReadOnlyNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:440
// ("cv::ocl::KernelArg::ReadOnlyNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WriteOnlyNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:442
// ("cv::ocl::KernelArg::WriteOnlyNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
pub fn cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::WriteOnlyNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:442
// ("cv::ocl::KernelArg::WriteOnlyNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Constant(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:444
// ("cv::ocl::KernelArg::Constant", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
pub fn cv_ocl_KernelArg_Constant_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::KernelArg::flags() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:448
// ("cv::ocl::KernelArg::flags", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_KernelArg_propFlags_const(instance: *const c_void) -> i32;
// cv::ocl::KernelArg::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:448
// ("cv::ocl::KernelArg::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ocl_KernelArg_propFlags_const_int(instance: *mut c_void, val: i32);
// cv::ocl::KernelArg::m() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:449
// ("cv::ocl::KernelArg::m", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_KernelArg_propM(instance: *mut c_void) -> *mut c_void;
// cv::ocl::KernelArg::setM(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:449
// ("cv::ocl::KernelArg::setM", vec![(pred!(mut, ["val"], ["cv::UMat*"]), _)]),
pub fn cv_ocl_KernelArg_propM_UMatX(instance: *mut c_void, val: *const c_void);
// cv::ocl::KernelArg::obj() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:450
// ("cv::ocl::KernelArg::obj", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_KernelArg_propObj_const(instance: *const c_void) -> *const c_void;
// cv::ocl::KernelArg::sz() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:451
// ("cv::ocl::KernelArg::sz", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_KernelArg_propSz_const(instance: *const c_void) -> size_t;
// cv::ocl::KernelArg::setSz(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:451
// ("cv::ocl::KernelArg::setSz", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_ocl_KernelArg_propSz_const_size_t(instance: *mut c_void, val: size_t);
// cv::ocl::KernelArg::wscale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:452
// ("cv::ocl::KernelArg::wscale", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_KernelArg_propWscale_const(instance: *const c_void) -> i32;
// cv::ocl::KernelArg::setWscale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:452
// ("cv::ocl::KernelArg::setWscale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ocl_KernelArg_propWscale_const_int(instance: *mut c_void, val: i32);
// cv::ocl::KernelArg::iwscale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:452
// ("cv::ocl::KernelArg::iwscale", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_KernelArg_propIwscale_const(instance: *const c_void) -> i32;
// cv::ocl::KernelArg::setIwscale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:452
// ("cv::ocl::KernelArg::setIwscale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ocl_KernelArg_propIwscale_const_int(instance: *mut c_void, val: i32);
// cv::ocl::KernelArg::delete() generated
// ("cv::ocl::KernelArg::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_KernelArg_delete(instance: *mut c_void);
// OpenCLExecutionContext()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:799
// ("cv::ocl::OpenCLExecutionContext::OpenCLExecutionContext", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext() -> *mut c_void;
// OpenCLExecutionContext(const OpenCLExecutionContext &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:802
// ("cv::ocl::OpenCLExecutionContext::OpenCLExecutionContext", vec![(pred!(mut, ["unnamed"], ["const cv::ocl::OpenCLExecutionContext*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_const_OpenCLExecutionContextR(unnamed: *const c_void) -> *mut c_void;
// OpenCLExecutionContext(OpenCLExecutionContext &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:803
// ("cv::ocl::OpenCLExecutionContext::OpenCLExecutionContext", vec![(pred!(mut, ["unnamed"], ["cv::ocl::OpenCLExecutionContext*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_OpenCLExecutionContextRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(const OpenCLExecutionContext &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:805
// ("cv::ocl::OpenCLExecutionContext::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::ocl::OpenCLExecutionContext*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_operatorST_const_OpenCLExecutionContextR(instance: *mut c_void, unnamed: *const c_void);
// operator=(OpenCLExecutionContext &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:806
// ("cv::ocl::OpenCLExecutionContext::operator=", vec![(pred!(mut, ["unnamed"], ["cv::ocl::OpenCLExecutionContext*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_operatorST_OpenCLExecutionContextRR(instance: *mut c_void, unnamed: *mut c_void);
// getContext()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:809
// ("cv::ocl::OpenCLExecutionContext::getContext", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_getContext_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDevice()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:811
// ("cv::ocl::OpenCLExecutionContext::getDevice", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_getDevice_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getQueue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:815
// ("cv::ocl::OpenCLExecutionContext::getQueue", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_getQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// useOpenCL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:817
// ("cv::ocl::OpenCLExecutionContext::useOpenCL", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_useOpenCL_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseOpenCL(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:818
// ("cv::ocl::OpenCLExecutionContext::setUseOpenCL", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_setUseOpenCL_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// getCurrent()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:826
// ("cv::ocl::OpenCLExecutionContext::getCurrent", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_getCurrent(ocvrs_return: *mut Result<*mut c_void>);
// getCurrentRef()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:829
// ("cv::ocl::OpenCLExecutionContext::getCurrentRef", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_getCurrentRef(ocvrs_return: *mut Result<*mut c_void>);
// bind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:837
// ("cv::ocl::OpenCLExecutionContext::bind", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_bind_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// cloneWithNewQueue(const ocl::Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:843
// ("cv::ocl::OpenCLExecutionContext::cloneWithNewQueue", vec![(pred!(const, ["q"], ["const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const_const_QueueR(instance: *const c_void, q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cloneWithNewQueue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:845
// ("cv::ocl::OpenCLExecutionContext::cloneWithNewQueue", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const std::string &, void *, void *, void *)(InString, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:861
// ("cv::ocl::OpenCLExecutionContext::create", vec![(pred!(mut, ["platformName", "platformID", "context", "deviceID"], ["const std::string*", "void*", "void*", "void*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_create_const_stringR_voidX_voidX_voidX(platform_name: *const c_char, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Context &, const Device &, const ocl::Queue &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:869
// ("cv::ocl::OpenCLExecutionContext::create", vec![(pred!(mut, ["context", "device", "queue"], ["const cv::ocl::Context*", "const cv::ocl::Device*", "const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR_const_QueueR(context: *const c_void, device: *const c_void, queue: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Context &, const Device &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:871
// ("cv::ocl::OpenCLExecutionContext::create", vec![(pred!(mut, ["context", "device"], ["const cv::ocl::Context*", "const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR(context: *const c_void, device: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:874
// ("cv::ocl::OpenCLExecutionContext::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:875
// ("cv::ocl::OpenCLExecutionContext::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ocl::OpenCLExecutionContext::delete() generated
// ("cv::ocl::OpenCLExecutionContext::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_OpenCLExecutionContext_delete(instance: *mut c_void);
// Platform()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:332
// ("cv::ocl::Platform::Platform", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Platform_Platform() -> *mut c_void;
// Platform(const Platform &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:334
// ("cv::ocl::Platform::Platform", vec![(pred!(mut, ["p"], ["const cv::ocl::Platform*"]), _)]),
pub fn cv_ocl_Platform_Platform_const_PlatformR(p: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Platform &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:335
// ("cv::ocl::Platform::operator=", vec![(pred!(mut, ["p"], ["const cv::ocl::Platform*"]), _)]),
pub fn cv_ocl_Platform_operatorST_const_PlatformR(instance: *mut c_void, p: *const c_void, ocvrs_return: *mut Result<()>);
// Platform(Platform &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:336
// ("cv::ocl::Platform::Platform", vec![(pred!(mut, ["p"], ["cv::ocl::Platform*"]), _)]),
pub fn cv_ocl_Platform_Platform_PlatformRR(p: *mut c_void) -> *mut c_void;
// operator=(Platform &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:337
// ("cv::ocl::Platform::operator=", vec![(pred!(mut, ["p"], ["cv::ocl::Platform*"]), _)]),
pub fn cv_ocl_Platform_operatorST_PlatformRR(instance: *mut c_void, p: *mut c_void);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:339
// ("cv::ocl::Platform::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Platform_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDefault()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:342
// ("cv::ocl::Platform::getDefault", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Platform_getDefault(ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:346
// ("cv::ocl::Platform::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Platform_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Platform::delete() generated
// ("cv::ocl::Platform::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Platform_delete(instance: *mut c_void);
// PlatformInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:671
// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_PlatformInfo_PlatformInfo() -> *mut c_void;
// PlatformInfo(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:675
// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, ["id"], ["void*"]), _)]),
pub fn cv_ocl_PlatformInfo_PlatformInfo_voidX(id: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// PlatformInfo(const PlatformInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:678
// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, ["i"], ["const cv::ocl::PlatformInfo*"]), _)]),
pub fn cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(i: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const PlatformInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:679
// ("cv::ocl::PlatformInfo::operator=", vec![(pred!(mut, ["i"], ["const cv::ocl::PlatformInfo*"]), _)]),
pub fn cv_ocl_PlatformInfo_operatorST_const_PlatformInfoR(instance: *mut c_void, i: *const c_void, ocvrs_return: *mut Result<()>);
// PlatformInfo(PlatformInfo &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:680
// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, ["i"], ["cv::ocl::PlatformInfo*"]), _)]),
pub fn cv_ocl_PlatformInfo_PlatformInfo_PlatformInfoRR(i: *mut c_void) -> *mut c_void;
// operator=(PlatformInfo &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:681
// ("cv::ocl::PlatformInfo::operator=", vec![(pred!(mut, ["i"], ["cv::ocl::PlatformInfo*"]), _)]),
pub fn cv_ocl_PlatformInfo_operatorST_PlatformInfoRR(instance: *mut c_void, i: *mut c_void);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:683
// ("cv::ocl::PlatformInfo::name", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// vendor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:684
// ("cv::ocl::PlatformInfo::vendor", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_vendor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// version()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:687
// ("cv::ocl::PlatformInfo::version", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// versionMajor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:688
// ("cv::ocl::PlatformInfo::versionMajor", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_versionMajor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// versionMinor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:689
// ("cv::ocl::PlatformInfo::versionMinor", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_versionMinor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// deviceNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:691
// ("cv::ocl::PlatformInfo::deviceNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_deviceNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getDevice(Device &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:692
// ("cv::ocl::PlatformInfo::getDevice", vec![(pred!(const, ["device", "d"], ["cv::ocl::Device*", "int"]), _)]),
pub fn cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(instance: *const c_void, device: *mut c_void, d: i32, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:695
// ("cv::ocl::PlatformInfo::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_PlatformInfo_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::PlatformInfo::delete() generated
// ("cv::ocl::PlatformInfo::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_PlatformInfo_delete(instance: *mut c_void);
// Program()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:552
// ("cv::ocl::Program::Program", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Program_Program() -> *mut c_void;
// Program(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:553
// ("cv::ocl::Program::Program", vec![(pred!(mut, ["src", "buildflags", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
pub fn cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(src: *const c_void, buildflags: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// Program(const Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:555
// ("cv::ocl::Program::Program", vec![(pred!(mut, ["prog"], ["const cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Program_Program_const_ProgramR(prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:556
// ("cv::ocl::Program::operator=", vec![(pred!(mut, ["prog"], ["const cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Program_operatorST_const_ProgramR(instance: *mut c_void, prog: *const c_void, ocvrs_return: *mut Result<()>);
// Program(Program &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:557
// ("cv::ocl::Program::Program", vec![(pred!(mut, ["prog"], ["cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Program_Program_ProgramRR(prog: *mut c_void) -> *mut c_void;
// operator=(Program &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:558
// ("cv::ocl::Program::operator=", vec![(pred!(mut, ["prog"], ["cv::ocl::Program*"]), _)]),
pub fn cv_ocl_Program_operatorST_ProgramRR(instance: *mut c_void, prog: *mut c_void);
// create(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:561
// ("cv::ocl::Program::create", vec![(pred!(mut, ["src", "buildflags", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
pub fn cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(instance: *mut c_void, src: *const c_void, buildflags: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:564
// ("cv::ocl::Program::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Program_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getBinary(std::vector<char> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:575
// ("cv::ocl::Program::getBinary", vec![(pred!(const, ["binary"], ["std::vector<char>*"]), _)]),
pub fn cv_ocl_Program_getBinary_const_vectorLcharGR(instance: *const c_void, binary: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:579
// ("cv::ocl::Program::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Program_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// read(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:585
// ("cv::ocl::Program::read", vec![(pred!(mut, ["buf", "buildflags"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ocl_Program_read_const_StringR_const_StringR(instance: *mut c_void, buf: *const c_char, buildflags: *const c_char, ocvrs_return: *mut Result<bool>);
// write(String &)(OutString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:586
// ("cv::ocl::Program::write", vec![(pred!(const, ["buf"], ["cv::String*"]), _)]),
pub fn cv_ocl_Program_write_const_StringR(instance: *const c_void, buf: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
// source()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:587
// ("cv::ocl::Program::source", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Program_source_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getPrefix()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:588
// ("cv::ocl::Program::getPrefix", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Program_getPrefix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getPrefix(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:589
// ("cv::ocl::Program::getPrefix", vec![(pred!(mut, ["buildflags"], ["const cv::String*"]), _)]),
pub fn cv_ocl_Program_getPrefix_const_StringR(buildflags: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Program::delete() generated
// ("cv::ocl::Program::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Program_delete(instance: *mut c_void);
// ProgramSource()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:599
// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_ProgramSource_ProgramSource() -> *mut c_void;
// ProgramSource(const String &, const String &, const String &, const String &)(InString, InString, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:600
// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["module", "name", "codeStr", "codeHash"], ["const cv::String*", "const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(module: *const c_char, name: *const c_char, code_str: *const c_char, code_hash: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// ProgramSource(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:601
// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["prog"], ["const cv::String*"]), _)]),
pub fn cv_ocl_ProgramSource_ProgramSource_const_StringR(prog: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// ProgramSource(const ProgramSource &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:604
// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["prog"], ["const cv::ocl::ProgramSource*"]), _)]),
pub fn cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const ProgramSource &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:605
// ("cv::ocl::ProgramSource::operator=", vec![(pred!(mut, ["prog"], ["const cv::ocl::ProgramSource*"]), _)]),
pub fn cv_ocl_ProgramSource_operatorST_const_ProgramSourceR(instance: *mut c_void, prog: *const c_void, ocvrs_return: *mut Result<()>);
// ProgramSource(ProgramSource &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:606
// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["prog"], ["cv::ocl::ProgramSource*"]), _)]),
pub fn cv_ocl_ProgramSource_ProgramSource_ProgramSourceRR(prog: *mut c_void) -> *mut c_void;
// operator=(ProgramSource &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:607
// ("cv::ocl::ProgramSource::operator=", vec![(pred!(mut, ["prog"], ["cv::ocl::ProgramSource*"]), _)]),
pub fn cv_ocl_ProgramSource_operatorST_ProgramSourceRR(instance: *mut c_void, prog: *mut c_void);
// source()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:609
// ("cv::ocl::ProgramSource::source", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_ProgramSource_source_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// hash()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:610
// ("cv::ocl::ProgramSource::hash", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_ProgramSource_hash_const(instance: *const c_void, ocvrs_return: *mut Result<core::ProgramSource_hash_t>);
// fromBinary(const String &, const String &, const unsigned char *, const size_t, const cv::String &)(InString, InString, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:627
// ("cv::ocl::ProgramSource::fromBinary", vec![(pred!(mut, ["module", "name", "binary", "size", "buildOptions"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t", "const cv::String*"]), _)]),
pub fn cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, build_options: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::ProgramSource::fromBinary(InString, InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:627
// ("cv::ocl::ProgramSource::fromBinary", vec![(pred!(mut, ["module", "name", "binary", "size"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t"]), _)]),
pub fn cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// fromSPIR(const String &, const String &, const unsigned char *, const size_t, const cv::String &)(InString, InString, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:652
// ("cv::ocl::ProgramSource::fromSPIR", vec![(pred!(mut, ["module", "name", "binary", "size", "buildOptions"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t", "const cv::String*"]), _)]),
pub fn cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, build_options: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::ProgramSource::fromSPIR(InString, InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:652
// ("cv::ocl::ProgramSource::fromSPIR", vec![(pred!(mut, ["module", "name", "binary", "size"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t"]), _)]),
pub fn cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:663
// ("cv::ocl::ProgramSource::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_ProgramSource_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::ProgramSource::delete() generated
// ("cv::ocl::ProgramSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_ProgramSource_delete(instance: *mut c_void);
// Queue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:393
// ("cv::ocl::Queue::Queue", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Queue_Queue() -> *mut c_void;
// Queue(const Context &, const Device &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:394
// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["c", "d"], ["const cv::ocl::Context*", "const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(c: *const c_void, d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ocl::Queue::Queue(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:394
// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
pub fn cv_ocl_Queue_Queue_const_ContextR(c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Queue(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:396
// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Queue_Queue_const_QueueR(q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:397
// ("cv::ocl::Queue::operator=", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Queue_operatorST_const_QueueR(instance: *mut c_void, q: *const c_void, ocvrs_return: *mut Result<()>);
// Queue(Queue &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:398
// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["q"], ["cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Queue_Queue_QueueRR(q: *mut c_void) -> *mut c_void;
// operator=(Queue &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:399
// ("cv::ocl::Queue::operator=", vec![(pred!(mut, ["q"], ["cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Queue_operatorST_QueueRR(instance: *mut c_void, q: *mut c_void);
// create(const Context &, const Device &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:401
// ("cv::ocl::Queue::create", vec![(pred!(mut, ["c", "d"], ["const cv::ocl::Context*", "const cv::ocl::Device*"]), _)]),
pub fn cv_ocl_Queue_create_const_ContextR_const_DeviceR(instance: *mut c_void, c: *const c_void, d: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Queue::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:401
// ("cv::ocl::Queue::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Queue_create(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// finish()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:402
// ("cv::ocl::Queue::finish", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Queue_finish(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// ptr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:403
// ("cv::ocl::Queue::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Queue_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDefault()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:404
// ("cv::ocl::Queue::getDefault", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Queue_getDefault(ocvrs_return: *mut Result<*mut c_void>);
// getProfilingQueue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:407
// ("cv::ocl::Queue::getProfilingQueue", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Queue_getProfilingQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:411
// ("cv::ocl::Queue::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Queue_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ocl::Queue::delete() generated
// ("cv::ocl::Queue::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Queue_delete(instance: *mut c_void);
// Timer(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:777
// ("cv::ocl::Timer::Timer", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
pub fn cv_ocl_Timer_Timer_const_QueueR(q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// start()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:779
// ("cv::ocl::Timer::start", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Timer_start(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:780
// ("cv::ocl::Timer::stop", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Timer_stop(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// durationNS()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/ocl.hpp:782
// ("cv::ocl::Timer::durationNS", vec![(pred!(const, [], []), _)]),
pub fn cv_ocl_Timer_durationNS_const(instance: *const c_void, ocvrs_return: *mut Result<u64>);
// cv::ocl::Timer::delete() generated
// ("cv::ocl::Timer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ocl_Timer_delete(instance: *mut c_void);
// Arrays()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:411
// ("cv::ogl::Arrays::Arrays", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_Arrays(ocvrs_return: *mut Result<*mut c_void>);
// setVertexArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:416
// ("cv::ogl::Arrays::setVertexArray", vec![(pred!(mut, ["vertex"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Arrays_setVertexArray_const__InputArrayR(instance: *mut c_void, vertex: *const c_void, ocvrs_return: *mut Result<()>);
// resetVertexArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:420
// ("cv::ogl::Arrays::resetVertexArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_resetVertexArray(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setColorArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:425
// ("cv::ogl::Arrays::setColorArray", vec![(pred!(mut, ["color"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Arrays_setColorArray_const__InputArrayR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result<()>);
// resetColorArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:429
// ("cv::ogl::Arrays::resetColorArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_resetColorArray(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setNormalArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:434
// ("cv::ogl::Arrays::setNormalArray", vec![(pred!(mut, ["normal"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Arrays_setNormalArray_const__InputArrayR(instance: *mut c_void, normal: *const c_void, ocvrs_return: *mut Result<()>);
// resetNormalArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:438
// ("cv::ogl::Arrays::resetNormalArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_resetNormalArray(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setTexCoordArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:443
// ("cv::ogl::Arrays::setTexCoordArray", vec![(pred!(mut, ["texCoord"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Arrays_setTexCoordArray_const__InputArrayR(instance: *mut c_void, tex_coord: *const c_void, ocvrs_return: *mut Result<()>);
// resetTexCoordArray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:447
// ("cv::ogl::Arrays::resetTexCoordArray", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_resetTexCoordArray(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:451
// ("cv::ogl::Arrays::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:456
// ("cv::ogl::Arrays::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ogl_Arrays_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// bind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:460
// ("cv::ogl::Arrays::bind", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Arrays_bind_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:464
// ("cv::ogl::Arrays::size", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Arrays_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:465
// ("cv::ogl::Arrays::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Arrays_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ogl::Arrays::delete() generated
// ("cv::ogl::Arrays::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Arrays_delete(instance: *mut c_void);
// Buffer()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:104
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_Buffer(ocvrs_return: *mut Result<*mut c_void>);
// Buffer(int, int, int, unsigned int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:113
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "abufId", "autoRelease"], ["int", "int", "int", "unsigned int", "bool"]), _)]),
pub fn cv_ogl_Buffer_Buffer_int_int_int_unsigned_int_bool(arows: i32, acols: i32, atype: i32, abuf_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::Buffer(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:113
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "abufId"], ["int", "int", "int", "unsigned int"]), _)]),
pub fn cv_ogl_Buffer_Buffer_int_int_int_unsigned_int(arows: i32, acols: i32, atype: i32, abuf_id: u32, ocvrs_return: *mut Result<*mut c_void>);
// Buffer(Size, int, unsigned int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:121
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "abufId", "autoRelease"], ["cv::Size", "int", "unsigned int", "bool"]), _)]),
pub fn cv_ogl_Buffer_Buffer_Size_int_unsigned_int_bool(asize: *const core::Size, atype: i32, abuf_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::Buffer(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:121
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "abufId"], ["cv::Size", "int", "unsigned int"]), _)]),
pub fn cv_ogl_Buffer_Buffer_Size_int_unsigned_int(asize: *const core::Size, atype: i32, abuf_id: u32, ocvrs_return: *mut Result<*mut c_void>);
// Buffer(int, int, int, Target, bool)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:130
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "target", "autoRelease"], ["int", "int", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_Buffer_int_int_int_Target_bool(arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::Buffer(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:130
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype"], ["int", "int", "int"]), _)]),
pub fn cv_ogl_Buffer_Buffer_int_int_int(arows: i32, acols: i32, atype: i32, ocvrs_return: *mut Result<*mut c_void>);
// Buffer(Size, int, Target, bool)(SimpleClass, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:138
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "target", "autoRelease"], ["cv::Size", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_Buffer_Size_int_Target_bool(asize: *const core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::Buffer(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:138
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype"], ["cv::Size", "int"]), _)]),
pub fn cv_ogl_Buffer_Buffer_Size_int(asize: *const core::Size, atype: i32, ocvrs_return: *mut Result<*mut c_void>);
// Buffer(InputArray, Target, bool)(InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:145
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arr", "target", "autoRelease"], ["const cv::_InputArray*", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_Buffer_const__InputArrayR_Target_bool(arr: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::Buffer(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:145
// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Buffer_Buffer_const__InputArrayR(arr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, Target, bool)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:155
// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["arows", "acols", "atype", "target", "autoRelease"], ["int", "int", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_create_int_int_int_Target_bool(instance: *mut c_void, arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Buffer::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:155
// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["arows", "acols", "atype"], ["int", "int", "int"]), _)]),
pub fn cv_ogl_Buffer_create_int_int_int(instance: *mut c_void, arows: i32, acols: i32, atype: i32, ocvrs_return: *mut Result<()>);
// create(Size, int, Target, bool)(SimpleClass, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:163
// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["asize", "atype", "target", "autoRelease"], ["cv::Size", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_create_Size_int_Target_bool(instance: *mut c_void, asize: *const core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Buffer::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:163
// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["asize", "atype"], ["cv::Size", "int"]), _)]),
pub fn cv_ogl_Buffer_create_Size_int(instance: *mut c_void, asize: *const core::Size, atype: i32, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:169
// ("cv::ogl::Buffer::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:180
// ("cv::ogl::Buffer::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ogl_Buffer_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// copyFrom(InputArray, Target, bool)(InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:187
// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "target", "autoRelease"], ["const cv::_InputArray*", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR_Target_bool(instance: *mut c_void, arr: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Buffer::copyFrom(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:187
// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR(instance: *mut c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// copyFrom(InputArray, cuda::Stream &, Target, bool)(InputArray, TraitClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:190
// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "stream", "target", "autoRelease"], ["const cv::_InputArray*", "cv::cuda::Stream*", "cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR_Target_bool(instance: *mut c_void, arr: *const c_void, stream: *mut c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Buffer::copyFrom(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:190
// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "stream"], ["const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR(instance: *mut c_void, arr: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:197
// ("cv::ogl::Buffer::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ogl_Buffer_copyTo_const_const__OutputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, cuda::Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:200
// ("cv::ogl::Buffer::copyTo", vec![(pred!(const, ["arr", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_ogl_Buffer_copyTo_const_const__OutputArrayR_StreamR(instance: *const c_void, arr: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// clone(Target, bool)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:207
// ("cv::ogl::Buffer::clone", vec![(pred!(const, ["target", "autoRelease"], ["cv::ogl::Buffer::Target", "bool"]), _)]),
pub fn cv_ogl_Buffer_clone_const_Target_bool(instance: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Buffer::clone() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:207
// ("cv::ogl::Buffer::clone", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bind(Target)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:213
// ("cv::ogl::Buffer::bind", vec![(pred!(const, ["target"], ["cv::ogl::Buffer::Target"]), _)]),
pub fn cv_ogl_Buffer_bind_const_Target(instance: *const c_void, target: core::Buffer_Target, ocvrs_return: *mut Result<()>);
// unbind(Target)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:219
// ("cv::ogl::Buffer::unbind", vec![(pred!(mut, ["target"], ["cv::ogl::Buffer::Target"]), _)]),
pub fn cv_ogl_Buffer_unbind_Target(target: core::Buffer_Target, ocvrs_return: *mut Result<()>);
// mapHost(Access)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:236
// ("cv::ogl::Buffer::mapHost", vec![(pred!(mut, ["access"], ["cv::ogl::Buffer::Access"]), _)]),
pub fn cv_ogl_Buffer_mapHost_Access(instance: *mut c_void, access: core::Buffer_Access, ocvrs_return: *mut Result<*mut c_void>);
// unmapHost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:240
// ("cv::ogl::Buffer::unmapHost", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_unmapHost(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// mapDevice()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:243
// ("cv::ogl::Buffer::mapDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_mapDevice(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// unmapDevice()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:244
// ("cv::ogl::Buffer::unmapDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_unmapDevice(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// mapDevice(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:252
// ("cv::ogl::Buffer::mapDevice", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_ogl_Buffer_mapDevice_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// unmapDevice(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:256
// ("cv::ogl::Buffer::unmapDevice", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_ogl_Buffer_unmapDevice_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// rows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:258
// ("cv::ogl::Buffer::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:259
// ("cv::ogl::Buffer::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:260
// ("cv::ogl::Buffer::size", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:261
// ("cv::ogl::Buffer::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:263
// ("cv::ogl::Buffer::type", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:264
// ("cv::ogl::Buffer::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// channels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:265
// ("cv::ogl::Buffer::channels", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:266
// ("cv::ogl::Buffer::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// elemSize1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:267
// ("cv::ogl::Buffer::elemSize1", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// bufId()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:270
// ("cv::ogl::Buffer::bufId", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Buffer_bufId_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
// cv::ogl::Buffer::delete() generated
// ("cv::ogl::Buffer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Buffer_delete(instance: *mut c_void);
// Texture2D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:301
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Texture2D_Texture2D(ocvrs_return: *mut Result<*mut c_void>);
// Texture2D(int, int, Format, unsigned int, bool)(Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:304
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "atexId", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "unsigned int", "bool"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int_bool(arows: i32, acols: i32, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Texture2D::Texture2D(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:304
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "atexId"], ["int", "int", "cv::ogl::Texture2D::Format", "unsigned int"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int(arows: i32, acols: i32, aformat: core::Texture2D_Format, atex_id: u32, ocvrs_return: *mut Result<*mut c_void>);
// Texture2D(Size, Format, unsigned int, bool)(SimpleClass, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:307
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "atexId", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "unsigned int", "bool"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int_bool(asize: *const core::Size, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Texture2D::Texture2D(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:307
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "atexId"], ["cv::Size", "cv::ogl::Texture2D::Format", "unsigned int"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int(asize: *const core::Size, aformat: core::Texture2D_Format, atex_id: u32, ocvrs_return: *mut Result<*mut c_void>);
// Texture2D(int, int, Format, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:315
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "bool"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_int_int_Format_bool(arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Texture2D::Texture2D(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:315
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat"], ["int", "int", "cv::ogl::Texture2D::Format"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_int_int_Format(arows: i32, acols: i32, aformat: core::Texture2D_Format, ocvrs_return: *mut Result<*mut c_void>);
// Texture2D(Size, Format, bool)(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:322
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "bool"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_Size_Format_bool(asize: *const core::Size, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Texture2D::Texture2D(SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:322
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat"], ["cv::Size", "cv::ogl::Texture2D::Format"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_Size_Format(asize: *const core::Size, aformat: core::Texture2D_Format, ocvrs_return: *mut Result<*mut c_void>);
// Texture2D(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:328
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arr", "autoRelease"], ["const cv::_InputArray*", "bool"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_const__InputArrayR_bool(arr: *const c_void, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ogl::Texture2D::Texture2D(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:328
// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Texture2D_Texture2D_const__InputArrayR(arr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, Format, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:337
// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["arows", "acols", "aformat", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "bool"]), _)]),
pub fn cv_ogl_Texture2D_create_int_int_Format_bool(instance: *mut c_void, arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Texture2D::create(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:337
// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["arows", "acols", "aformat"], ["int", "int", "cv::ogl::Texture2D::Format"]), _)]),
pub fn cv_ogl_Texture2D_create_int_int_Format(instance: *mut c_void, arows: i32, acols: i32, aformat: core::Texture2D_Format, ocvrs_return: *mut Result<()>);
// create(Size, Format, bool)(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:343
// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["asize", "aformat", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "bool"]), _)]),
pub fn cv_ogl_Texture2D_create_Size_Format_bool(instance: *mut c_void, asize: *const core::Size, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Texture2D::create(SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:343
// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["asize", "aformat"], ["cv::Size", "cv::ogl::Texture2D::Format"]), _)]),
pub fn cv_ogl_Texture2D_create_Size_Format(instance: *mut c_void, asize: *const core::Size, aformat: core::Texture2D_Format, ocvrs_return: *mut Result<()>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:349
// ("cv::ogl::Texture2D::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Texture2D_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:361
// ("cv::ogl::Texture2D::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
pub fn cv_ogl_Texture2D_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result<()>);
// copyFrom(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:368
// ("cv::ogl::Texture2D::copyFrom", vec![(pred!(mut, ["arr", "autoRelease"], ["const cv::_InputArray*", "bool"]), _)]),
pub fn cv_ogl_Texture2D_copyFrom_const__InputArrayR_bool(instance: *mut c_void, arr: *const c_void, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Texture2D::copyFrom(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:368
// ("cv::ogl::Texture2D::copyFrom", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ogl_Texture2D_copyFrom_const__InputArrayR(instance: *mut c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// copyTo(OutputArray, int, bool)(OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:377
// ("cv::ogl::Texture2D::copyTo", vec![(pred!(const, ["arr", "ddepth", "autoRelease"], ["const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_ogl_Texture2D_copyTo_const_const__OutputArrayR_int_bool(instance: *const c_void, arr: *const c_void, ddepth: i32, auto_release: bool, ocvrs_return: *mut Result<()>);
// cv::ogl::Texture2D::copyTo(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:377
// ("cv::ogl::Texture2D::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ogl_Texture2D_copyTo_const_const__OutputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result<()>);
// bind()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:381
// ("cv::ogl::Texture2D::bind", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_bind_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// rows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:383
// ("cv::ogl::Texture2D::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:384
// ("cv::ogl::Texture2D::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:385
// ("cv::ogl::Texture2D::size", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:386
// ("cv::ogl::Texture2D::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// format()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:388
// ("cv::ogl::Texture2D::format", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_format_const(instance: *const c_void, ocvrs_return: *mut Result<core::Texture2D_Format>);
// texId()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/opengl.hpp:391
// ("cv::ogl::Texture2D::texId", vec![(pred!(const, [], []), _)]),
pub fn cv_ogl_Texture2D_texId_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
// cv::ogl::Texture2D::delete() generated
// ("cv::ogl::Texture2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ogl_Texture2D_delete(instance: *mut c_void);
// ClassWithKeywordProperties(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:242
// ("cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties", vec![(pred!(mut, ["lambda_arg", "except_arg"], ["int", "int"]), _)]),
pub fn cv_utils_ClassWithKeywordProperties_ClassWithKeywordProperties_int_int(lambda_arg: i32, except_arg: i32, ocvrs_return: *mut Result<core::ClassWithKeywordProperties>);
// cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:242
// ("cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_ClassWithKeywordProperties_ClassWithKeywordProperties(ocvrs_return: *mut Result<core::ClassWithKeywordProperties>);
// setLambda(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:254
// ("cv::utils::FunctionParams::setLambda", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_utils_FunctionParams_setLambda_int(instance: *const core::FunctionParams, value: i32, ocvrs_return: *mut core::FunctionParams);
// setSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:260
// ("cv::utils::FunctionParams::setSigma", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_utils_FunctionParams_setSigma_float(instance: *const core::FunctionParams, value: f32, ocvrs_return: *mut core::FunctionParams);
// LogTag(const char *, LogLevel)(InString, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logtag.hpp:20
// ("cv::utils::logging::LogTag::LogTag", vec![(pred!(mut, ["_name", "_level"], ["const char*", "cv::utils::logging::LogLevel"]), _)]),
pub fn cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(_name: *const c_char, _level: core::LogLevel, ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::logging::LogTag::name() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logtag.hpp:17
// ("cv::utils::logging::LogTag::name", vec![(pred!(const, [], []), _)]),
pub fn cv_utils_logging_LogTag_propName_const(instance: *const c_void) -> *mut c_void;
// cv::utils::logging::LogTag::level() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logtag.hpp:18
// ("cv::utils::logging::LogTag::level", vec![(pred!(const, [], []), _)]),
pub fn cv_utils_logging_LogTag_propLevel_const(instance: *const c_void, ocvrs_return: *mut core::LogLevel);
// cv::utils::logging::LogTag::setLevel(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/utils/logtag.hpp:18
// ("cv::utils::logging::LogTag::setLevel", vec![(pred!(mut, ["val"], ["const cv::utils::logging::LogLevel"]), _)]),
pub fn cv_utils_logging_LogTag_propLevel_const_LogLevel(instance: *mut c_void, val: core::LogLevel);
// cv::utils::logging::LogTag::delete() generated
// ("cv::utils::logging::LogTag::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_logging_LogTag_delete(instance: *mut c_void);
// OriginalClassName(const OriginalClassName::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:296
// ("cv::utils::nested::OriginalClassName::OriginalClassName", vec![(pred!(mut, ["params"], ["const cv::utils::nested::OriginalClassName::Params*"]), _)]),
pub fn cv_utils_nested_OriginalClassName_OriginalClassName_const_ParamsR(params: *const core::OriginalClassName_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::nested::OriginalClassName::OriginalClassName() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:296
// ("cv::utils::nested::OriginalClassName::OriginalClassName", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_OriginalClassName(ocvrs_return: *mut Result<*mut c_void>);
// getIntParam()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:301
// ("cv::utils::nested::OriginalClassName::getIntParam", vec![(pred!(const, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_getIntParam_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getFloatParam()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:306
// ("cv::utils::nested::OriginalClassName::getFloatParam", vec![(pred!(const, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_getFloatParam_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// originalName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:311
// ("cv::utils::nested::OriginalClassName::originalName", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_originalName(ocvrs_return: *mut Result<*mut c_void>);
// create(const OriginalClassName::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:317
// ("cv::utils::nested::OriginalClassName::create", vec![(pred!(mut, ["params"], ["const cv::utils::nested::OriginalClassName::Params*"]), _)]),
pub fn cv_utils_nested_OriginalClassName_create_const_ParamsR(params: *const core::OriginalClassName_Params, ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::nested::OriginalClassName::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:317
// ("cv::utils::nested::OriginalClassName::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::utils::nested::OriginalClassName::delete() generated
// ("cv::utils::nested::OriginalClassName::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_delete(instance: *mut c_void);
// Params(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:289
// ("cv::utils::nested::OriginalClassName::Params::Params", vec![(pred!(mut, ["int_param", "float_param"], ["int", "float"]), _)]),
pub fn cv_utils_nested_OriginalClassName_Params_Params_int_float(int_param: i32, float_param: f32, ocvrs_return: *mut Result<core::OriginalClassName_Params>);
// cv::utils::nested::OriginalClassName::Params::Params() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/core/bindings_utils.hpp:289
// ("cv::utils::nested::OriginalClassName::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_utils_nested_OriginalClassName_Params_Params(ocvrs_return: *mut Result<core::OriginalClassName_Params>);
