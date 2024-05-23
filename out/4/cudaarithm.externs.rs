// cv::cuda::absSum(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:852
// ("cv::cuda::absSum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_absSum_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// absSum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:852
// ("cv::cuda::absSum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_absSum_const__InputArrayR_const__InputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::cuda::abs(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:243
// ("cv::cuda::abs", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_abs_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// abs(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:243
// ("cv::cuda::abs", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_abs_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::absdiffWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:231
// ("cv::cuda::absdiffWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_absdiffWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// absdiffWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:231
// ("cv::cuda::absdiffWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_absdiffWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::absdiff(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:220
// ("cv::cuda::absdiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// absdiff(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:220
// ("cv::cuda::absdiff", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::addWeighted(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:538
// ("cv::cuda::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// addWeighted(InputArray, double, InputArray, double, double, OutputArray, int, Stream &)(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:538
// ("cv::cuda::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst", "dtype", "stream"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int_StreamR(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, dst: *const c_void, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::addWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:103
// ("cv::cuda::addWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_addWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// addWithScalar(InputArray, Scalar, OutputArray, InputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:103
// ("cv::cuda::addWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_addWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_int_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, mask: *const c_void, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::add(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:88
// ("cv::cuda::add", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// add(InputArray, InputArray, OutputArray, InputArray, int, Stream &)(InputArray, InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:88
// ("cv::cuda::add", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_and(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:391
// ("cv::cuda::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_and(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:391
// ("cv::cuda::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_and_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:404
// ("cv::cuda::bitwise_and_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_and_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_and_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:404
// ("cv::cuda::bitwise_and_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_and_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_not(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:346
// ("cv::cuda::bitwise_not", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_not(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:346
// ("cv::cuda::bitwise_not", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_or(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:361
// ("cv::cuda::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_or(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:361
// ("cv::cuda::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_or_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:374
// ("cv::cuda::bitwise_or_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_or_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_or_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:374
// ("cv::cuda::bitwise_or_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_or_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_xor(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:421
// ("cv::cuda::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_xor(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:421
// ("cv::cuda::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::bitwise_xor_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:434
// ("cv::cuda::bitwise_xor_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_bitwise_xor_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bitwise_xor_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:434
// ("cv::cuda::bitwise_xor_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_bitwise_xor_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcAbsSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:854
// ("cv::cuda::calcAbsSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// calcAbsSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:854
// ("cv::cuda::calcAbsSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcNormDiff(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:834
// ("cv::cuda::calcNormDiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// calcNormDiff(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:834
// ("cv::cuda::calcNormDiff", vec![(pred!(mut, ["src1", "src2", "dst", "normType", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, norm_type: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcNorm(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:822
// ("cv::cuda::calcNorm", vec![(pred!(mut, ["src", "dst", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, norm_type: i32, ocvrs_return: *mut Result<()>);
// calcNorm(InputArray, OutputArray, int, InputArray, Stream &)(InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:822
// ("cv::cuda::calcNorm", vec![(pred!(mut, ["src", "dst", "normType", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, norm_type: i32, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcSqrSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:863
// ("cv::cuda::calcSqrSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// calcSqrSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:863
// ("cv::cuda::calcSqrSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::calcSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:845
// ("cv::cuda::calcSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// calcSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:845
// ("cv::cuda::calcSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::cartToPolar(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:658
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// cartToPolar(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:658
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(x: *const c_void, y: *const c_void, magnitude: *const c_void, angle: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::cartToPolar(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:681
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitudeAngle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR(xy: *const c_void, magnitude_angle: *const c_void, ocvrs_return: *mut Result<()>);
// cartToPolar(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:681
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitudeAngle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_bool_StreamR(xy: *const c_void, magnitude_angle: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::cartToPolar(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:670
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(xy: *const c_void, magnitude: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// cartToPolar(InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:670
// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitude", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(xy: *const c_void, magnitude: *const c_void, angle: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::compareWithScalar(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:334
// ("cv::cuda::compareWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_compareWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_int(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, cmpop: i32, ocvrs_return: *mut Result<()>);
// compareWithScalar(InputArray, Scalar, OutputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:334
// ("cv::cuda::compareWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_compareWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_int_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, cmpop: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::compare(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:316
// ("cv::cuda::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, cmpop: i32, ocvrs_return: *mut Result<()>);
// compare(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:316
// ("cv::cuda::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, cmpop: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::copyMakeBorder(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:804
// ("cv::cuda::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int(src: *const c_void, dst: *const c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// copyMakeBorder(InputArray, OutputArray, int, int, int, int, int, Scalar, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:804
// ("cv::cuda::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType", "value", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_Scalar_StreamR(src: *const c_void, dst: *const c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: *const core::Scalar, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// countNonZero(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:907
// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_countNonZero_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::countNonZero(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:909
// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// countNonZero(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:909
// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::createConvolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1174
// ("cv::cuda::createConvolution", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_createConvolution(ocvrs_return: *mut Result<*mut c_void>);
// createConvolution(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1174
// ("cv::cuda::createConvolution", vec![(pred!(mut, ["user_block_size"], ["cv::Size"]), _)]),
pub fn cv_cuda_createConvolution_Size(user_block_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createDFT(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1148
// ("cv::cuda::createDFT", vec![(pred!(mut, ["dft_size", "flags"], ["cv::Size", "int"]), _)]),
pub fn cv_cuda_createDFT_Size_int(dft_size: *const core::Size, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// createLookUpTable(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:787
// ("cv::cuda::createLookUpTable", vec![(pred!(mut, ["lut"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_createLookUpTable_const__InputArrayR(lut: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::dft(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1119
// ("cv::cuda::dft", vec![(pred!(mut, ["src", "dst", "dft_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size(src: *const c_void, dst: *const c_void, dft_size: *const core::Size, ocvrs_return: *mut Result<()>);
// dft(InputArray, OutputArray, Size, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1119
// ("cv::cuda::dft", vec![(pred!(mut, ["src", "dst", "dft_size", "flags", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size_int_StreamR(src: *const c_void, dst: *const c_void, dft_size: *const core::Size, flags: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::divideWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:205
// ("cv::cuda::divideWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_divideWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// divideWithScalar(InputArray, Scalar, OutputArray, double, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:205
// ("cv::cuda::divideWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_divideWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_double_int_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, scale: f64, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::divide(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:189
// ("cv::cuda::divide", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// divide(InputArray, InputArray, OutputArray, double, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:189
// ("cv::cuda::divide", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::exp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:271
// ("cv::cuda::exp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_exp_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// exp(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:271
// ("cv::cuda::exp", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_exp_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::findMinMaxLoc(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:896
// ("cv::cuda::findMinMaxLoc", vec![(pred!(mut, ["src", "minMaxVals", "loc"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, min_max_vals: *const c_void, loc: *const c_void, ocvrs_return: *mut Result<()>);
// findMinMaxLoc(InputArray, OutputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:896
// ("cv::cuda::findMinMaxLoc", vec![(pred!(mut, ["src", "minMaxVals", "loc", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, min_max_vals: *const c_void, loc: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::findMinMax(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:878
// ("cv::cuda::findMinMax", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// findMinMax(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:878
// ("cv::cuda::findMinMax", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::flip(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:766
// ("cv::cuda::flip", vec![(pred!(mut, ["src", "dst", "flipCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flip_code: i32, ocvrs_return: *mut Result<()>);
// flip(InputArray, OutputArray, int, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:766
// ("cv::cuda::flip", vec![(pred!(mut, ["src", "dst", "flipCode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int_StreamR(src: *const c_void, dst: *const c_void, flip_code: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::gemm(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1048
// ("cv::cuda::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR(src1: *const c_void, src2: *const c_void, alpha: f64, src3: *const c_void, beta: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// gemm(InputArray, InputArray, double, InputArray, double, OutputArray, int, Stream &)(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1048
// ("cv::cuda::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst", "flags", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int_StreamR(src1: *const c_void, src2: *const c_void, alpha: f64, src3: *const c_void, beta: f64, dst: *const c_void, flags: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::inRange(InputArray, SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:584
// ("cv::cuda::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst"], ["const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR(src: *const c_void, lowerb: *const core::Scalar, upperb: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// inRange(InputArray, const Scalar &, const Scalar &, OutputArray, Stream &)(InputArray, SimpleClass, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:584
// ("cv::cuda::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst", "stream"], ["const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR_StreamR(src: *const c_void, lowerb: *const core::Scalar, upperb: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::integral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1007
// ("cv::cuda::integral", vec![(pred!(mut, ["src", "sum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_integral_const__InputArrayR_const__OutputArrayR(src: *const c_void, sum: *const c_void, ocvrs_return: *mut Result<()>);
// integral(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1007
// ("cv::cuda::integral", vec![(pred!(mut, ["src", "sum", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_integral_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, sum: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::log(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:281
// ("cv::cuda::log", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_log_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// log(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:281
// ("cv::cuda::log", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_log_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::lshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:459
// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(src: *const c_void, val: *const core::Scalar_<i32>, dst: *const c_void, ocvrs_return: *mut Result<()>);
// lshift(InputArray, Scalar_<int>, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:459
// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(src: *const c_void, val: *const core::Scalar_<i32>, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::lshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:461
// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR(src: *const c_void, val: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// lshift(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:461
// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src: *const c_void, val: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::magnitudeSqr(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:622
// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result<()>);
// magnitudeSqr(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:622
// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["x", "y", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(x: *const c_void, y: *const c_void, magnitude: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::magnitudeSqr(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:602
// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["xy", "magnitude"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR(xy: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result<()>);
// magnitudeSqr(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:602
// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["xy", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR_StreamR(xy: *const c_void, magnitude: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::magnitude(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:612
// ("cv::cuda::magnitude", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result<()>);
// magnitude(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:612
// ("cv::cuda::magnitude", vec![(pred!(mut, ["x", "y", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(x: *const c_void, y: *const c_void, magnitude: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::magnitude(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:594
// ("cv::cuda::magnitude", vec![(pred!(mut, ["xy", "magnitude"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR(xy: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result<()>);
// magnitude(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:594
// ("cv::cuda::magnitude", vec![(pred!(mut, ["xy", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR_StreamR(xy: *const c_void, magnitude: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::maxWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:513
// ("cv::cuda::maxWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_maxWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// maxWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:513
// ("cv::cuda::maxWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_maxWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::max(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:502
// ("cv::cuda::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// max(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:502
// ("cv::cuda::max", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// meanStdDev(InputArray, Scalar &, Scalar &)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:967
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "mean", "stddev"], ["const cv::_InputArray*", "cv::Scalar*", "cv::Scalar*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR(mtx: *const c_void, mean: *mut core::Scalar, stddev: *mut core::Scalar, ocvrs_return: *mut Result<()>);
// meanStdDev(InputArray, Scalar &, Scalar &, InputArray)(InputArray, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:961
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev", "mask"], ["const cv::_InputArray*", "cv::Scalar*", "cv::Scalar*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR_const__InputArrayR(src: *const c_void, mean: *mut core::Scalar, stddev: *mut core::Scalar, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::meanStdDev(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:954
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR(mtx: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// meanStdDev(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:954
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_StreamR(mtx: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::meanStdDev(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:948
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// meanStdDev(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:948
// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::merge(TraitClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:727
// ("cv::cuda::merge", vec![(pred!(mut, ["src", "n", "dst"], ["const cv::cuda::GpuMat*", "size_t", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR(src: *const c_void, n: size_t, dst: *const c_void, ocvrs_return: *mut Result<()>);
// merge(const GpuMat *, size_t, OutputArray, Stream &)(TraitClass, Primitive, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:727
// ("cv::cuda::merge", vec![(pred!(mut, ["src", "n", "dst", "stream"], ["const cv::cuda::GpuMat*", "size_t", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR_StreamR(src: *const c_void, n: size_t, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::merge(CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:729
// ("cv::cuda::merge", vec![(pred!(mut, ["src", "dst"], ["const std::vector<cv::cuda::GpuMat>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// merge(const std::vector<GpuMat> &, OutputArray, Stream &)(CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:729
// ("cv::cuda::merge", vec![(pred!(mut, ["src", "dst", "stream"], ["const std::vector<cv::cuda::GpuMat>*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::minMaxLoc(InputArray, Indirect, Indirect, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:893
// ("cv::cuda::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*"]), _)]),
pub fn cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_loc: *mut core::Point, max_loc: *mut core::Point, ocvrs_return: *mut Result<()>);
// minMaxLoc(InputArray, double *, double *, Point *, Point *, InputArray)(InputArray, Indirect, Indirect, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:893
// ("cv::cuda::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_loc: *mut core::Point, max_loc: *mut core::Point, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::minMax(InputArray, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:876
// ("cv::cuda::minMax", vec![(pred!(mut, ["src", "minVal", "maxVal"], ["const cv::_InputArray*", "double*", "double*"]), _)]),
pub fn cv_cuda_minMax_const__InputArrayR_doubleX_doubleX(src: *const c_void, min_val: *mut f64, max_val: *mut f64, ocvrs_return: *mut Result<()>);
// minMax(InputArray, double *, double *, InputArray)(InputArray, Indirect, Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:876
// ("cv::cuda::minMax", vec![(pred!(mut, ["src", "minVal", "maxVal", "mask"], ["const cv::_InputArray*", "double*", "double*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_minMax_const__InputArrayR_doubleX_doubleX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::minWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:487
// ("cv::cuda::minWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_minWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// minWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:487
// ("cv::cuda::minWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_minWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::min(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:476
// ("cv::cuda::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// min(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:476
// ("cv::cuda::min", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::mulAndScaleSpectrums(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1082
// ("cv::cuda::mulAndScaleSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "float"]), _)]),
pub fn cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, scale: f32, ocvrs_return: *mut Result<()>);
// mulAndScaleSpectrums(InputArray, InputArray, OutputArray, int, float, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1082
// ("cv::cuda::mulAndScaleSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "scale", "conjB", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float_bool_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, scale: f32, conj_b: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::mulSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1065
// ("cv::cuda::mulSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// mulSpectrums(InputArray, InputArray, OutputArray, int, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1065
// ("cv::cuda::mulSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "conjB", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, conj_b: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::multiplyWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:169
// ("cv::cuda::multiplyWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_multiplyWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// multiplyWithScalar(InputArray, Scalar, OutputArray, double, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:169
// ("cv::cuda::multiplyWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_multiplyWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_double_int_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, scale: f64, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::multiply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:155
// ("cv::cuda::multiply", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// multiply(InputArray, InputArray, OutputArray, double, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:155
// ("cv::cuda::multiply", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::norm(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:832
// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_norm_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<f64>);
// norm(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:832
// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "src2", "normType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_cuda_norm_const__InputArrayR_const__InputArrayR_int(src1: *const c_void, src2: *const c_void, norm_type: i32, ocvrs_return: *mut Result<f64>);
// cv::cuda::norm(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:820
// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "normType"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_cuda_norm_const__InputArrayR_int(src1: *const c_void, norm_type: i32, ocvrs_return: *mut Result<f64>);
// norm(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:820
// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "normType", "mask"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_norm_const__InputArrayR_int_const__InputArrayR(src1: *const c_void, norm_type: i32, mask: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::cuda::normalize(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:995
// ("cv::cuda::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int"]), _)]),
pub fn cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, norm_type: i32, dtype: i32, ocvrs_return: *mut Result<()>);
// normalize(InputArray, OutputArray, double, double, int, int, InputArray, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:995
// ("cv::cuda::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int_const__InputArrayR_StreamR(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::phase(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:634
// ("cv::cuda::phase", vec![(pred!(mut, ["x", "y", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// phase(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:634
// ("cv::cuda::phase", vec![(pred!(mut, ["x", "y", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(x: *const c_void, y: *const c_void, angle: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::phase(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:645
// ("cv::cuda::phase", vec![(pred!(mut, ["xy", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_phase_const__InputArrayR_const__OutputArrayR(xy: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<()>);
// phase(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:645
// ("cv::cuda::phase", vec![(pred!(mut, ["xy", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_phase_const__InputArrayR_const__OutputArrayR_bool_StreamR(xy: *const c_void, angle: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::polarToCart(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:702
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "xy"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR(magnitude: *const c_void, angle: *const c_void, xy: *const c_void, ocvrs_return: *mut Result<()>);
// polarToCart(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:702
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "xy", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(magnitude: *const c_void, angle: *const c_void, xy: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::polarToCart(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:692
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(magnitude: *const c_void, angle: *const c_void, x: *const c_void, y: *const c_void, ocvrs_return: *mut Result<()>);
// polarToCart(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:692
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(magnitude: *const c_void, angle: *const c_void, x: *const c_void, y: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::polarToCart(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:711
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitudeAngle", "xy"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__OutputArrayR(magnitude_angle: *const c_void, xy: *const c_void, ocvrs_return: *mut Result<()>);
// polarToCart(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:711
// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitudeAngle", "xy", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_polarToCart_const__InputArrayR_const__OutputArrayR_bool_StreamR(magnitude_angle: *const c_void, xy: *const c_void, angle_in_degrees: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::pow(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:296
// ("cv::cuda::pow", vec![(pred!(mut, ["src", "power", "dst"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR(src: *const c_void, power: f64, dst: *const c_void, ocvrs_return: *mut Result<()>);
// pow(InputArray, double, OutputArray, Stream &)(InputArray, Primitive, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:296
// ("cv::cuda::pow", vec![(pred!(mut, ["src", "power", "dst", "stream"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR_StreamR(src: *const c_void, power: f64, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::rectStdDev(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:977
// ("cv::cuda::rectStdDev", vec![(pred!(mut, ["src", "sqr", "dst", "rect"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::Rect"]), _)]),
pub fn cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect(src: *const c_void, sqr: *const c_void, dst: *const c_void, rect: *const core::Rect, ocvrs_return: *mut Result<()>);
// rectStdDev(InputArray, InputArray, OutputArray, Rect, Stream &)(InputArray, InputArray, OutputArray, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:977
// ("cv::cuda::rectStdDev", vec![(pred!(mut, ["src", "sqr", "dst", "rect", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::Rect", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect_StreamR(src: *const c_void, sqr: *const c_void, dst: *const c_void, rect: *const core::Rect, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::reduce(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:937
// ("cv::cuda::reduce", vec![(pred!(mut, ["mtx", "vec", "dim", "reduceOp"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int(mtx: *const c_void, vec: *const c_void, dim: i32, reduce_op: i32, ocvrs_return: *mut Result<()>);
// reduce(InputArray, OutputArray, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:937
// ("cv::cuda::reduce", vec![(pred!(mut, ["mtx", "vec", "dim", "reduceOp", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(mtx: *const c_void, vec: *const c_void, dim: i32, reduce_op: i32, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::rshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:445
// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(src: *const c_void, val: *const core::Scalar_<i32>, dst: *const c_void, ocvrs_return: *mut Result<()>);
// rshift(InputArray, Scalar_<int>, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:445
// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(src: *const c_void, val: *const core::Scalar_<i32>, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::rshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:447
// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR(src: *const c_void, val: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// rshift(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:447
// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src: *const c_void, val: *const core::Scalar, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::split(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:739
// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cuda_split_const__InputArrayR_GpuMatX(src: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// split(InputArray, GpuMat *, Stream &)(InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:739
// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_split_const__InputArrayR_GpuMatX_StreamR(src: *const c_void, dst: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::split(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:741
// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "std::vector<cv::cuda::GpuMat>*"]), _)]),
pub fn cv_cuda_split_const__InputArrayR_vectorLGpuMatGR(src: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// split(InputArray, std::vector<GpuMat> &, Stream &)(InputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:741
// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_split_const__InputArrayR_vectorLGpuMatGR_StreamR(src: *const c_void, dst: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::sqrIntegral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1016
// ("cv::cuda::sqrIntegral", vec![(pred!(mut, ["src", "sqsum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR(src: *const c_void, sqsum: *const c_void, ocvrs_return: *mut Result<()>);
// sqrIntegral(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1016
// ("cv::cuda::sqrIntegral", vec![(pred!(mut, ["src", "sqsum", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, sqsum: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::sqrSum(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:861
// ("cv::cuda::sqrSum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_sqrSum_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// sqrSum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:861
// ("cv::cuda::sqrSum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_sqrSum_const__InputArrayR_const__InputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::cuda::sqr(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:251
// ("cv::cuda::sqr", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_sqr_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// sqr(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:251
// ("cv::cuda::sqr", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_sqr_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::sqrt(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:261
// ("cv::cuda::sqrt", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// sqrt(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:261
// ("cv::cuda::sqrt", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR_StreamR(src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::subtractWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:137
// ("cv::cuda::subtractWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_subtractWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, ocvrs_return: *mut Result<()>);
// subtractWithScalar(InputArray, Scalar, OutputArray, InputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:137
// ("cv::cuda::subtractWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_subtractWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_int_StreamR(src1: *const c_void, src2: *const core::Scalar, dst: *const c_void, mask: *const c_void, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::subtract(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:122
// ("cv::cuda::subtract", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// subtract(InputArray, InputArray, OutputArray, InputArray, int, Stream &)(InputArray, InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:122
// ("cv::cuda::subtract", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::sum(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:843
// ("cv::cuda::sum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_sum_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// sum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:843
// ("cv::cuda::sum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_sum_const__InputArrayR_const__InputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::cuda::threshold(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:559
// ("cv::cuda::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(src: *const c_void, dst: *const c_void, thresh: f64, maxval: f64, typ: i32, ocvrs_return: *mut Result<f64>);
// threshold(InputArray, OutputArray, double, double, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:559
// ("cv::cuda::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int_StreamR(src: *const c_void, dst: *const c_void, thresh: f64, maxval: f64, typ: i32, stream: *mut c_void, ocvrs_return: *mut Result<f64>);
// cv::cuda::transpose(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:751
// ("cv::cuda::transpose", vec![(pred!(mut, ["src1", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_transpose_const__InputArrayR_const__OutputArrayR(src1: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// transpose(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:751
// ("cv::cuda::transpose", vec![(pred!(mut, ["src1", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_transpose_const__InputArrayR_const__OutputArrayR_StreamR(src1: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// convolve(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1165
// ("cv::cuda::Convolution::convolve", vec![(pred!(mut, ["image", "templ", "result", "ccorr", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(instance: *mut c_void, image: *const c_void, templ: *const c_void, result: *const c_void, ccorr: bool, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Convolution::convolve(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1165
// ("cv::cuda::Convolution::convolve", vec![(pred!(mut, ["image", "templ", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, templ: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Convolution::to_Algorithm() generated
// ("cv::cuda::Convolution::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Convolution_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Convolution::delete() generated
// ("cv::cuda::Convolution::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Convolution_delete(instance: *mut c_void);
// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1132
// ("cv::cuda::DFT::compute", vec![(pred!(mut, ["image", "result", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, image: *const c_void, result: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DFT::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:1132
// ("cv::cuda::DFT::compute", vec![(pred!(mut, ["image", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DFT::to_Algorithm() generated
// ("cv::cuda::DFT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DFT_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DFT::delete() generated
// ("cv::cuda::DFT::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DFT_delete(instance: *mut c_void);
// transform(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:780
// ("cv::cuda::LookUpTable::transform", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::LookUpTable::transform(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaarithm.hpp:780
// ("cv::cuda::LookUpTable::transform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::LookUpTable::to_Algorithm() generated
// ("cv::cuda::LookUpTable::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_LookUpTable_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::LookUpTable::delete() generated
// ("cv::cuda::LookUpTable::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_LookUpTable_delete(instance: *mut c_void);
