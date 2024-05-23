// cv::cuda::createBoxFilter(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:102
// ("cv::cuda::createBoxFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "cv::Size"]), _)]),
pub fn cv_cuda_createBoxFilter_int_int_Size(src_type: i32, dst_type: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createBoxFilter(int, int, Size, Point, int, Scalar)(Primitive, Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:102
// ("cv::cuda::createBoxFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createBoxFilter_int_int_Size_Point_int_Scalar(src_type: i32, dst_type: i32, ksize: *const core::Size, anchor: *const core::Point, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createBoxMaxFilter(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:276
// ("cv::cuda::createBoxMaxFilter", vec![(pred!(mut, ["srcType", "ksize"], ["int", "cv::Size"]), _)]),
pub fn cv_cuda_createBoxMaxFilter_int_Size(src_type: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createBoxMaxFilter(int, Size, Point, int, Scalar)(Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:276
// ("cv::cuda::createBoxMaxFilter", vec![(pred!(mut, ["srcType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createBoxMaxFilter_int_Size_Point_int_Scalar(src_type: i32, ksize: *const core::Size, anchor: *const core::Point, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createBoxMinFilter(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:288
// ("cv::cuda::createBoxMinFilter", vec![(pred!(mut, ["srcType", "ksize"], ["int", "cv::Size"]), _)]),
pub fn cv_cuda_createBoxMinFilter_int_Size(src_type: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createBoxMinFilter(int, Size, Point, int, Scalar)(Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:288
// ("cv::cuda::createBoxMinFilter", vec![(pred!(mut, ["srcType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createBoxMinFilter_int_Size_Point_int_Scalar(src_type: i32, ksize: *const core::Size, anchor: *const core::Point, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createColumnSumFilter(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:315
// ("cv::cuda::createColumnSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_createColumnSumFilter_int_int_int(src_type: i32, dst_type: i32, ksize: i32, ocvrs_return: *mut Result<*mut c_void>);
// createColumnSumFilter(int, int, int, int, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:315
// ("cv::cuda::createColumnSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "int", "int", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createColumnSumFilter_int_int_int_int_int_Scalar(src_type: i32, dst_type: i32, ksize: i32, anchor: i32, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createDerivFilter(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:182
// ("cv::cuda::createDerivFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize"], ["int", "int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createDerivFilter_int_int_int_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32, ocvrs_return: *mut Result<*mut c_void>);
// createDerivFilter(int, int, int, int, int, bool, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:182
// ("cv::cuda::createDerivFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize", "normalize", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "int", "bool", "double", "int", "int"]), _)]),
pub fn cv_cuda_createDerivFilter_int_int_int_int_int_bool_double_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32, normalize: bool, scale: f64, row_border_mode: i32, column_border_mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createGaussianFilter(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:238
// ("cv::cuda::createGaussianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "sigma1"], ["int", "int", "cv::Size", "double"]), _)]),
pub fn cv_cuda_createGaussianFilter_int_int_Size_double(src_type: i32, dst_type: i32, ksize: *const core::Size, sigma1: f64, ocvrs_return: *mut Result<*mut c_void>);
// createGaussianFilter(int, int, Size, double, double, int, int)(Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:238
// ("cv::cuda::createGaussianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "sigma1", "sigma2", "rowBorderMode", "columnBorderMode"], ["int", "int", "cv::Size", "double", "double", "int", "int"]), _)]),
pub fn cv_cuda_createGaussianFilter_int_int_Size_double_double_int_int(src_type: i32, dst_type: i32, ksize: *const core::Size, sigma1: f64, sigma2: f64, row_border_mode: i32, column_border_mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createLaplacianFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:139
// ("cv::cuda::createLaplacianFilter", vec![(pred!(mut, ["srcType", "dstType"], ["int", "int"]), _)]),
pub fn cv_cuda_createLaplacianFilter_int_int(src_type: i32, dst_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// createLaplacianFilter(int, int, int, double, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:139
// ("cv::cuda::createLaplacianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "scale", "borderMode", "borderVal"], ["int", "int", "int", "double", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createLaplacianFilter_int_int_int_double_int_Scalar(src_type: i32, dst_type: i32, ksize: i32, scale: f64, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createLinearFilter(Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:120
// ("cv::cuda::createLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "kernel"], ["int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_createLinearFilter_int_int_const__InputArrayR(src_type: i32, dst_type: i32, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createLinearFilter(int, int, InputArray, Point, int, Scalar)(Primitive, Primitive, InputArray, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:120
// ("cv::cuda::createLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "kernel", "anchor", "borderMode", "borderVal"], ["int", "int", "const cv::_InputArray*", "cv::Point", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createLinearFilter_int_int_const__InputArrayR_Point_int_Scalar(src_type: i32, dst_type: i32, kernel: *const c_void, anchor: *const core::Point, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createMedianFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:333
// ("cv::cuda::createMedianFilter", vec![(pred!(mut, ["srcType", "windowSize"], ["int", "int"]), _)]),
pub fn cv_cuda_createMedianFilter_int_int(src_type: i32, window_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// createMedianFilter(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:333
// ("cv::cuda::createMedianFilter", vec![(pred!(mut, ["srcType", "windowSize", "partition"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_createMedianFilter_int_int_int(src_type: i32, window_size: i32, partition: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createMorphologyFilter(Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:263
// ("cv::cuda::createMorphologyFilter", vec![(pred!(mut, ["op", "srcType", "kernel"], ["int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_createMorphologyFilter_int_int_const__InputArrayR(op: i32, src_type: i32, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createMorphologyFilter(int, int, InputArray, Point, int)(Primitive, Primitive, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:263
// ("cv::cuda::createMorphologyFilter", vec![(pred!(mut, ["op", "srcType", "kernel", "anchor", "iterations"], ["int", "int", "const cv::_InputArray*", "cv::Point", "int"]), _)]),
pub fn cv_cuda_createMorphologyFilter_int_int_const__InputArrayR_Point_int(op: i32, src_type: i32, kernel: *const c_void, anchor: *const core::Point, iterations: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createRowSumFilter(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:304
// ("cv::cuda::createRowSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "int"]), _)]),
pub fn cv_cuda_createRowSumFilter_int_int_int(src_type: i32, dst_type: i32, ksize: i32, ocvrs_return: *mut Result<*mut c_void>);
// createRowSumFilter(int, int, int, int, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:304
// ("cv::cuda::createRowSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "int", "int", "int", "cv::Scalar"]), _)]),
pub fn cv_cuda_createRowSumFilter_int_int_int_int_int_Scalar(src_type: i32, dst_type: i32, ksize: i32, anchor: i32, border_mode: i32, border_val: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createScharrFilter(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:218
// ("cv::cuda::createScharrFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy"], ["int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createScharrFilter_int_int_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, ocvrs_return: *mut Result<*mut c_void>);
// createScharrFilter(int, int, int, int, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:218
// ("cv::cuda::createScharrFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "double", "int", "int"]), _)]),
pub fn cv_cuda_createScharrFilter_int_int_int_int_double_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, scale: f64, row_border_mode: i32, column_border_mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createSeparableLinearFilter(Primitive, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:161
// ("cv::cuda::createSeparableLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "rowKernel", "columnKernel"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR(src_type: i32, dst_type: i32, row_kernel: *const c_void, column_kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSeparableLinearFilter(int, int, InputArray, InputArray, Point, int, int)(Primitive, Primitive, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:161
// ("cv::cuda::createSeparableLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "rowKernel", "columnKernel", "anchor", "rowBorderMode", "columnBorderMode"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "int", "int"]), _)]),
pub fn cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR_Point_int_int(src_type: i32, dst_type: i32, row_kernel: *const c_void, column_kernel: *const c_void, anchor: *const core::Point, row_border_mode: i32, column_border_mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::createSobelFilter(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:201
// ("cv::cuda::createSobelFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy"], ["int", "int", "int", "int"]), _)]),
pub fn cv_cuda_createSobelFilter_int_int_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, ocvrs_return: *mut Result<*mut c_void>);
// createSobelFilter(int, int, int, int, int, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:201
// ("cv::cuda::createSobelFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "int", "double", "int", "int"]), _)]),
pub fn cv_cuda_createSobelFilter_int_int_int_int_int_double_int_int(src_type: i32, dst_type: i32, dx: i32, dy: i32, ksize: i32, scale: f64, row_border_mode: i32, column_border_mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// apply(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:84
// ("cv::cuda::Filter::apply", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, src: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Filter::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudafilters.hpp:84
// ("cv::cuda::Filter::apply", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::Filter::to_Algorithm() generated
// ("cv::cuda::Filter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Filter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::Filter::delete() generated
// ("cv::cuda::Filter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_Filter_delete(instance: *mut c_void);
