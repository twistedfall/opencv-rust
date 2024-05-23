// cv::Canny(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1906
// ("cv::Canny", vec![(pred!(mut, ["dx", "dy", "edges", "threshold1", "threshold2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(dx: *const c_void, dy: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, ocvrs_return: *mut Result<()>);
// Canny(InputArray, InputArray, OutputArray, double, double, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1906
// ("cv::Canny", vec![(pred!(mut, ["dx", "dy", "edges", "threshold1", "threshold2", "L2gradient"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "bool"]), _)]),
pub fn cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(dx: *const c_void, dy: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, l2gradient: bool, ocvrs_return: *mut Result<()>);
// cv::Canny(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1888
// ("cv::Canny", vec![(pred!(mut, ["image", "edges", "threshold1", "threshold2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_Canny_const__InputArrayR_const__OutputArrayR_double_double(image: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, ocvrs_return: *mut Result<()>);
// Canny(InputArray, OutputArray, double, double, int, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1888
// ("cv::Canny", vec![(pred!(mut, ["image", "edges", "threshold1", "threshold2", "apertureSize", "L2gradient"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "bool"]), _)]),
pub fn cv_Canny_const__InputArrayR_const__OutputArrayR_double_double_int_bool(image: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<()>);
// cv::EMD(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3389
// ("cv::EMD", vec![(pred!(mut, ["signature1", "signature2", "distType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_EMD_const__InputArrayR_const__InputArrayR_int(signature1: *const c_void, signature2: *const c_void, dist_type: i32, ocvrs_return: *mut Result<f32>);
// EMD(InputArray, InputArray, int, InputArray, float *, OutputArray)(InputArray, InputArray, Primitive, InputArray, Indirect, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3389
// ("cv::EMD", vec![(pred!(mut, ["signature1", "signature2", "distType", "cost", "lowerBound", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "float*", "const cv::_OutputArray*"]), _)]),
pub fn cv_EMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_floatX_const__OutputArrayR(signature1: *const c_void, signature2: *const c_void, dist_type: i32, cost: *const c_void, lower_bound: *mut f32, flow: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::GaussianBlur(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1543
// ("cv::GaussianBlur", vec![(pred!(mut, ["src", "dst", "ksize", "sigmaX"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double"]), _)]),
pub fn cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double(src: *const c_void, dst: *const c_void, ksize: *const core::Size, sigma_x: f64, ocvrs_return: *mut Result<()>);
// GaussianBlur(InputArray, OutputArray, Size, double, double, int, AlgorithmHint)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1543
// ("cv::GaussianBlur", vec![(pred!(mut, ["src", "dst", "ksize", "sigmaX", "sigmaY", "borderType", "hint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int", "cv::AlgorithmHint"]), _)]),
pub fn cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double_double_int_AlgorithmHint(src: *const c_void, dst: *const c_void, ksize: *const core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, hint: core::AlgorithmHint, ocvrs_return: *mut Result<()>);
// cv::HoughCircles(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2277
// ("cv::HoughCircles", vec![(pred!(mut, ["image", "circles", "method", "dp", "minDist"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double(image: *const c_void, circles: *const c_void, method: i32, dp: f64, min_dist: f64, ocvrs_return: *mut Result<()>);
// HoughCircles(InputArray, OutputArray, int, double, double, double, double, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2277
// ("cv::HoughCircles", vec![(pred!(mut, ["image", "circles", "method", "dp", "minDist", "param1", "param2", "minRadius", "maxRadius"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "double", "double", "int", "int"]), _)]),
pub fn cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double_double_double_int_int(image: *const c_void, circles: *const c_void, method: i32, dp: f64, min_dist: f64, param1: f64, param2: f64, min_radius: i32, max_radius: i32, ocvrs_return: *mut Result<()>);
// cv::HoughLinesP(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2204
// ("cv::HoughLinesP", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, ocvrs_return: *mut Result<()>);
// HoughLinesP(InputArray, OutputArray, double, double, int, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2204
// ("cv::HoughLinesP", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold", "minLineLength", "maxLineGap"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "double", "double"]), _)]),
pub fn cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int_double_double(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, min_line_length: f64, max_line_gap: f64, ocvrs_return: *mut Result<()>);
// HoughLinesPointSet(InputArray, OutputArray, int, int, double, double, double, double, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2226
// ("cv::HoughLinesPointSet", vec![(pred!(mut, ["point", "lines", "lines_max", "threshold", "min_rho", "max_rho", "rho_step", "min_theta", "max_theta", "theta_step"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "double", "double", "double", "double"]), _)]),
pub fn cv_HoughLinesPointSet_const__InputArrayR_const__OutputArrayR_int_int_double_double_double_double_double_double(point: *const c_void, lines: *const c_void, lines_max: i32, threshold: i32, min_rho: f64, max_rho: f64, rho_step: f64, min_theta: f64, max_theta: f64, theta_step: f64, ocvrs_return: *mut Result<()>);
// cv::HoughLines(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2170
// ("cv::HoughLines", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, ocvrs_return: *mut Result<()>);
// HoughLines(InputArray, OutputArray, double, double, int, double, double, double, double, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2170
// ("cv::HoughLines", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold", "srn", "stn", "min_theta", "max_theta", "use_edgeval"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "double", "double", "double", "double", "bool"]), _)]),
pub fn cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int_double_double_double_double_bool(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, srn: f64, stn: f64, min_theta: f64, max_theta: f64, use_edgeval: bool, ocvrs_return: *mut Result<()>);
// HuMoments(const Moments &, OutputArray)(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3835
// ("cv::HuMoments", vec![(pred!(mut, ["m", "hu"], ["const cv::Moments*", "const cv::_OutputArray*"]), _)]),
pub fn cv_HuMoments_const_MomentsR_const__OutputArrayR(m: *const core::Moments, hu: *const c_void, ocvrs_return: *mut Result<()>);
// HuMoments(const Moments &, double *)(SimpleClass, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3832
// ("cv::HuMoments", vec![(pred!(mut, ["moments", "hu"], ["const cv::Moments*", "double**"]), _)]),
pub fn cv_HuMoments_const_MomentsR_doubleXX(moments: *const core::Moments, hu: *mut [f64; 7], ocvrs_return: *mut Result<()>);
// cv::Laplacian(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1856
// ("cv::Laplacian", vec![(pred!(mut, ["src", "dst", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_Laplacian_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, ddepth: i32, ocvrs_return: *mut Result<()>);
// Laplacian(InputArray, OutputArray, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1856
// ("cv::Laplacian", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "int"]), _)]),
pub fn cv_Laplacian_const__InputArrayR_const__OutputArrayR_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::Scharr(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1825
// ("cv::Scharr", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, ocvrs_return: *mut Result<()>);
// Scharr(InputArray, OutputArray, int, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1825
// ("cv::Scharr", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int"]), _)]),
pub fn cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::Sobel(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1775
// ("cv::Sobel", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, ocvrs_return: *mut Result<()>);
// Sobel(InputArray, OutputArray, int, int, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1775
// ("cv::Sobel", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy", "ksize", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "double", "double", "int"]), _)]),
pub fn cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::accumulateProduct(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2960
// ("cv::accumulateProduct", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// accumulateProduct(InputArray, InputArray, InputOutputArray, InputArray)(InputArray, InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2960
// ("cv::accumulateProduct", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::accumulateSquare(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2941
// ("cv::accumulateSquare", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// accumulateSquare(InputArray, InputOutputArray, InputArray)(InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2941
// ("cv::accumulateSquare", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::accumulateWeighted(InputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2981
// ("cv::accumulateWeighted", vec![(pred!(mut, ["src", "dst", "alpha"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double"]), _)]),
pub fn cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double(src: *const c_void, dst: *const c_void, alpha: f64, ocvrs_return: *mut Result<()>);
// accumulateWeighted(InputArray, InputOutputArray, double, InputArray)(InputArray, InputOutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2981
// ("cv::accumulateWeighted", vec![(pred!(mut, ["src", "dst", "alpha", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "const cv::_InputArray*"]), _)]),
pub fn cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double_const__InputArrayR(src: *const c_void, dst: *const c_void, alpha: f64, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::accumulate(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2922
// ("cv::accumulate", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_accumulate_const__InputArrayR_const__InputOutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// accumulate(InputArray, InputOutputArray, InputArray)(InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2922
// ("cv::accumulate", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_accumulate_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// adaptiveThreshold(InputArray, OutputArray, double, int, int, int, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3113
// ("cv::adaptiveThreshold", vec![(pred!(mut, ["src", "dst", "maxValue", "adaptiveMethod", "thresholdType", "blockSize", "C"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "int", "double"]), _)]),
pub fn cv_adaptiveThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_int_double(src: *const c_void, dst: *const c_void, max_value: f64, adaptive_method: i32, threshold_type: i32, block_size: i32, c: f64, ocvrs_return: *mut Result<()>);
// applyColorMap(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4517
// ("cv::applyColorMap", vec![(pred!(mut, ["src", "dst", "userColor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, user_color: *const c_void, ocvrs_return: *mut Result<()>);
// applyColorMap(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4509
// ("cv::applyColorMap", vec![(pred!(mut, ["src", "dst", "colormap"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_applyColorMap_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, colormap: i32, ocvrs_return: *mut Result<()>);
// approxPolyDP(InputArray, OutputArray, double, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4063
// ("cv::approxPolyDP", vec![(pred!(mut, ["curve", "approxCurve", "epsilon", "closed"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "bool"]), _)]),
pub fn cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(curve: *const c_void, approx_curve: *const c_void, epsilon: f64, closed: bool, ocvrs_return: *mut Result<()>);
// cv::approxPolyN(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4085
// ("cv::approxPolyN", vec![(pred!(mut, ["curve", "approxCurve", "nsides"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int(curve: *const c_void, approx_curve: *const c_void, nsides: i32, ocvrs_return: *mut Result<()>);
// approxPolyN(InputArray, OutputArray, int, float, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4085
// ("cv::approxPolyN", vec![(pred!(mut, ["curve", "approxCurve", "nsides", "epsilon_percentage", "ensure_convex"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "bool"]), _)]),
pub fn cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int_float_bool(curve: *const c_void, approx_curve: *const c_void, nsides: i32, epsilon_percentage: f32, ensure_convex: bool, ocvrs_return: *mut Result<()>);
// arcLength(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4096
// ("cv::arcLength", vec![(pred!(mut, ["curve", "closed"], ["const cv::_InputArray*", "bool"]), _)]),
pub fn cv_arcLength_const__InputArrayR_bool(curve: *const c_void, closed: bool, ocvrs_return: *mut Result<f64>);
// cv::arrowedLine(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4559
// ("cv::arrowedLine", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
pub fn cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// arrowedLine(InputOutputArray, Point, Point, const Scalar &, int, int, int, double)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4559
// ("cv::arrowedLine", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "line_type", "shift", "tipLength"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int", "double"]), _)]),
pub fn cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int_double(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, tip_length: f64, ocvrs_return: *mut Result<()>);
// cv::bilateralFilter(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1576
// ("cv::bilateralFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double(src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, ocvrs_return: *mut Result<()>);
// bilateralFilter(InputArray, OutputArray, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1576
// ("cv::bilateralFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int"]), _)]),
pub fn cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int(src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3686
// ("cv::blendLinear", vec![(pred!(mut, ["src1", "src2", "weights1", "weights2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, weights1: *const c_void, weights2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::blur(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1650
// ("cv::blur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_blur_const__InputArrayR_const__OutputArrayR_Size(src: *const c_void, dst: *const c_void, ksize: *const core::Size, ocvrs_return: *mut Result<()>);
// blur(InputArray, OutputArray, Size, Point, int)(InputArray, OutputArray, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1650
// ("cv::blur", vec![(pred!(mut, ["src", "dst", "ksize", "anchor", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "cv::Point", "int"]), _)]),
pub fn cv_blur_const__InputArrayR_const__OutputArrayR_Size_Point_int(src: *const c_void, dst: *const c_void, ksize: *const core::Size, anchor: *const core::Point, border_type: i32, ocvrs_return: *mut Result<()>);
// boundingRect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4105
// ("cv::boundingRect", vec![(pred!(mut, ["array"], ["const cv::_InputArray*"]), _)]),
pub fn cv_boundingRect_const__InputArrayR(array: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::boxFilter(InputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1604
// ("cv::boxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size"]), _)]),
pub fn cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, ocvrs_return: *mut Result<()>);
// boxFilter(InputArray, OutputArray, int, Size, Point, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1604
// ("cv::boxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "anchor", "normalize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size", "cv::Point", "bool", "int"]), _)]),
pub fn cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, ocvrs_return: *mut Result<()>);
// boxPoints(RotatedRect, OutputArray)(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4158
// ("cv::boxPoints", vec![(pred!(mut, ["box", "points"], ["cv::RotatedRect", "const cv::_OutputArray*"]), _)]),
pub fn cv_boxPoints_RotatedRect_const__OutputArrayR(box_: *const core::RotatedRect, points: *const c_void, ocvrs_return: *mut Result<()>);
// cv::buildPyramid(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3179
// ("cv::buildPyramid", vec![(pred!(mut, ["src", "dst", "maxlevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, maxlevel: i32, ocvrs_return: *mut Result<()>);
// buildPyramid(InputArray, OutputArrayOfArrays, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3179
// ("cv::buildPyramid", vec![(pred!(mut, ["src", "dst", "maxlevel", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, maxlevel: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// calcBackProject(InputArrayOfArrays, const std::vector<int> &, InputArray, OutputArray, const std::vector<float> &, double)(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3305
// ("cv::calcBackProject", vec![(pred!(mut, ["images", "channels", "hist", "dst", "ranges", "scale"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*", "double"]), _)]),
pub fn cv_calcBackProject_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR_double(images: *const c_void, channels: *const c_void, hist: *const c_void, dst: *const c_void, ranges: *const c_void, scale: f64, ocvrs_return: *mut Result<()>);
// cv::calcHist(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3247
// ("cv::calcHist", vec![(pred!(mut, ["images", "channels", "mask", "hist", "histSize", "ranges"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<int>*", "const std::vector<float>*"]), _)]),
pub fn cv_calcHist_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLintGR_const_vectorLfloatGR(images: *const c_void, channels: *const c_void, mask: *const c_void, hist: *const c_void, hist_size: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<()>);
// calcHist(InputArrayOfArrays, const std::vector<int> &, InputArray, OutputArray, const std::vector<int> &, const std::vector<float> &, bool)(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3247
// ("cv::calcHist", vec![(pred!(mut, ["images", "channels", "mask", "hist", "histSize", "ranges", "accumulate"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<int>*", "const std::vector<float>*", "bool"]), _)]),
pub fn cv_calcHist_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLintGR_const_vectorLfloatGR_bool(images: *const c_void, channels: *const c_void, mask: *const c_void, hist: *const c_void, hist_size: *const c_void, ranges: *const c_void, accumulate: bool, ocvrs_return: *mut Result<()>);
// cv::circle(InputOutputArray, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4605
// ("cv::circle", vec![(pred!(mut, ["img", "center", "radius", "color"], ["const cv::_InputOutputArray*", "cv::Point", "int", "const cv::Scalar*"]), _)]),
pub fn cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR(img: *const c_void, center: *const core::Point, radius: i32, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// circle(InputOutputArray, Point, int, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4605
// ("cv::circle", vec![(pred!(mut, ["img", "center", "radius", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "int", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR_int_int_int(img: *const c_void, center: *const core::Point, radius: i32, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// clipLine(Rect, Point &, Point &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4814
// ("cv::clipLine", vec![(pred!(mut, ["imgRect", "pt1", "pt2"], ["cv::Rect", "cv::Point*", "cv::Point*"]), _)]),
pub fn cv_clipLine_Rect_PointR_PointR(img_rect: *const core::Rect, pt1: *mut core::Point, pt2: *mut core::Point, ocvrs_return: *mut Result<bool>);
// clipLine(Size2l, Point2l &, Point2l &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4807
// ("cv::clipLine", vec![(pred!(mut, ["imgSize", "pt1", "pt2"], ["cv::Size2l", "cv::Point2l*", "cv::Point2l*"]), _)]),
pub fn cv_clipLine_Size2l_Point2lR_Point2lR(img_size: *const core::Size2l, pt1: *mut core::Point2l, pt2: *mut core::Point2l, ocvrs_return: *mut Result<bool>);
// clipLine(Size, Point &, Point &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4800
// ("cv::clipLine", vec![(pred!(mut, ["imgSize", "pt1", "pt2"], ["cv::Size", "cv::Point*", "cv::Point*"]), _)]),
pub fn cv_clipLine_Size_PointR_PointR(img_size: *const core::Size, pt1: *mut core::Point, pt2: *mut core::Point, ocvrs_return: *mut Result<bool>);
// compareHist(const SparseMat &, const SparseMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3328
// ("cv::compareHist", vec![(pred!(mut, ["H1", "H2", "method"], ["const cv::SparseMat*", "const cv::SparseMat*", "int"]), _)]),
pub fn cv_compareHist_const_SparseMatR_const_SparseMatR_int(h1: *const c_void, h2: *const c_void, method: i32, ocvrs_return: *mut Result<f64>);
// compareHist(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3325
// ("cv::compareHist", vec![(pred!(mut, ["H1", "H2", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_compareHist_const__InputArrayR_const__InputArrayR_int(h1: *const c_void, h2: *const c_void, method: i32, ocvrs_return: *mut Result<f64>);
// cv::connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3998
// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, labels: *const c_void, stats: *const c_void, centroids: *const c_void, ocvrs_return: *mut Result<i32>);
// connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3998
// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(image: *const c_void, labels: *const c_void, stats: *const c_void, centroids: *const c_void, connectivity: i32, ltype: i32, ocvrs_return: *mut Result<i32>);
// connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray, int, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3983
// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(image: *const c_void, labels: *const c_void, stats: *const c_void, centroids: *const c_void, connectivity: i32, ltype: i32, ccltype: i32, ocvrs_return: *mut Result<i32>);
// cv::connectedComponents(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3956
// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_connectedComponents_const__InputArrayR_const__OutputArrayR(image: *const c_void, labels: *const c_void, ocvrs_return: *mut Result<i32>);
// connectedComponents(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3956
// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ocvrs_return: *mut Result<i32>);
// connectedComponents(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3945
// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_int(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ccltype: i32, ocvrs_return: *mut Result<i32>);
// cv::contourArea(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4137
// ("cv::contourArea", vec![(pred!(mut, ["contour"], ["const cv::_InputArray*"]), _)]),
pub fn cv_contourArea_const__InputArrayR(contour: *const c_void, ocvrs_return: *mut Result<f64>);
// contourArea(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4137
// ("cv::contourArea", vec![(pred!(mut, ["contour", "oriented"], ["const cv::_InputArray*", "bool"]), _)]),
pub fn cv_contourArea_const__InputArrayR_bool(contour: *const c_void, oriented: bool, ocvrs_return: *mut Result<f64>);
// cv::convertMaps(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2560
// ("cv::convertMaps", vec![(pred!(mut, ["map1", "map2", "dstmap1", "dstmap2", "dstmap1type"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(map1: *const c_void, map2: *const c_void, dstmap1: *const c_void, dstmap2: *const c_void, dstmap1type: i32, ocvrs_return: *mut Result<()>);
// convertMaps(InputArray, InputArray, OutputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2560
// ("cv::convertMaps", vec![(pred!(mut, ["map1", "map2", "dstmap1", "dstmap2", "dstmap1type", "nninterpolation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_bool(map1: *const c_void, map2: *const c_void, dstmap1: *const c_void, dstmap2: *const c_void, dstmap1type: i32, nninterpolation: bool, ocvrs_return: *mut Result<()>);
// cv::convexHull(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4238
// ("cv::convexHull", vec![(pred!(mut, ["points", "hull"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convexHull_const__InputArrayR_const__OutputArrayR(points: *const c_void, hull: *const c_void, ocvrs_return: *mut Result<()>);
// convexHull(InputArray, OutputArray, bool, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4238
// ("cv::convexHull", vec![(pred!(mut, ["points", "hull", "clockwise", "returnPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "bool"]), _)]),
pub fn cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(points: *const c_void, hull: *const c_void, clockwise: bool, return_points: bool, ocvrs_return: *mut Result<()>);
// convexityDefects(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4258
// ("cv::convexityDefects", vec![(pred!(mut, ["contour", "convexhull", "convexityDefects"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(contour: *const c_void, convexhull: *const c_void, convexity_defects: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cornerEigenValsAndVecs(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1977
// ("cv::cornerEigenValsAndVecs", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, ocvrs_return: *mut Result<()>);
// cornerEigenValsAndVecs(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1977
// ("cv::cornerEigenValsAndVecs", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::cornerHarris(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1947
// ("cv::cornerHarris", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double"]), _)]),
pub fn cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, k: f64, ocvrs_return: *mut Result<()>);
// cornerHarris(InputArray, OutputArray, int, int, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1947
// ("cv::cornerHarris", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "k", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "int"]), _)]),
pub fn cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, k: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::cornerMinEigenVal(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1924
// ("cv::cornerMinEigenVal", vec![(pred!(mut, ["src", "dst", "blockSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, block_size: i32, ocvrs_return: *mut Result<()>);
// cornerMinEigenVal(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1924
// ("cv::cornerMinEigenVal", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// cornerSubPix(InputArray, InputOutputArray, Size, Size, TermCriteria)(InputArray, InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2047
// ("cv::cornerSubPix", vec![(pred!(mut, ["image", "corners", "winSize", "zeroZone", "criteria"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size", "cv::TermCriteria"]), _)]),
pub fn cv_cornerSubPix_const__InputArrayR_const__InputOutputArrayR_Size_Size_TermCriteria(image: *const c_void, corners: *const c_void, win_size: *const core::Size, zero_zone: *const core::Size, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::createCLAHE() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3353
// ("cv::createCLAHE", vec![(pred!(mut, [], []), _)]),
pub fn cv_createCLAHE(ocvrs_return: *mut Result<*mut c_void>);
// createCLAHE(double, Size)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3353
// ("cv::createCLAHE", vec![(pred!(mut, ["clipLimit", "tileGridSize"], ["double", "cv::Size"]), _)]),
pub fn cv_createCLAHE_double_Size(clip_limit: f64, tile_grid_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createGeneralizedHoughBallard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4461
// ("cv::createGeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
pub fn cv_createGeneralizedHoughBallard(ocvrs_return: *mut Result<*mut c_void>);
// createGeneralizedHoughGuil()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4465
// ("cv::createGeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
pub fn cv_createGeneralizedHoughGuil(ocvrs_return: *mut Result<*mut c_void>);
// createHanningWindow(OutputArray, Size, int)(OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3037
// ("cv::createHanningWindow", vec![(pred!(mut, ["dst", "winSize", "type"], ["const cv::_OutputArray*", "cv::Size", "int"]), _)]),
pub fn cv_createHanningWindow_const__OutputArrayR_Size_int(dst: *const c_void, win_size: *const core::Size, typ: i32, ocvrs_return: *mut Result<()>);
// cv::createLineSegmentDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1414
// ("cv::createLineSegmentDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_createLineSegmentDetector(ocvrs_return: *mut Result<*mut c_void>);
// createLineSegmentDetector(int, double, double, double, double, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1414
// ("cv::createLineSegmentDetector", vec![(pred!(mut, ["refine", "scale", "sigma_scale", "quant", "ang_th", "log_eps", "density_th", "n_bins"], ["int", "double", "double", "double", "double", "double", "double", "int"]), _)]),
pub fn cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(refine: i32, scale: f64, sigma_scale: f64, quant: f64, ang_th: f64, log_eps: f64, density_th: f64, n_bins: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cvtColorTwoPlane(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3756
// ("cv::cvtColorTwoPlane", vec![(pred!(mut, ["src1", "src2", "dst", "code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result<()>);
// cvtColorTwoPlane(InputArray, InputArray, OutputArray, int, AlgorithmHint)(InputArray, InputArray, OutputArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3756
// ("cv::cvtColorTwoPlane", vec![(pred!(mut, ["src1", "src2", "dst", "code", "hint"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::AlgorithmHint"]), _)]),
pub fn cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_AlgorithmHint(src1: *const c_void, src2: *const c_void, dst: *const c_void, code: i32, hint: core::AlgorithmHint, ocvrs_return: *mut Result<()>);
// cv::cvtColor(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3735
// ("cv::cvtColor", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_cvtColor_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result<()>);
// cvtColor(InputArray, OutputArray, int, int, AlgorithmHint)(InputArray, OutputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3735
// ("cv::cvtColor", vec![(pred!(mut, ["src", "dst", "code", "dstCn", "hint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::AlgorithmHint"]), _)]),
pub fn cv_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_AlgorithmHint(src: *const c_void, dst: *const c_void, code: i32, dst_cn: i32, hint: core::AlgorithmHint, ocvrs_return: *mut Result<()>);
// cv::demosaicing(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3788
// ("cv::demosaicing", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_demosaicing_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result<()>);
// demosaicing(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3788
// ("cv::demosaicing", vec![(pred!(mut, ["src", "dst", "code", "dstCn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_demosaicing_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, code: i32, dst_cn: i32, ocvrs_return: *mut Result<()>);
// cv::dilate(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2347
// ("cv::dilate", vec![(pred!(mut, ["src", "dst", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, kernel: *const c_void, ocvrs_return: *mut Result<()>);
// dilate(InputArray, OutputArray, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2347
// ("cv::dilate", vec![(pred!(mut, ["src", "dst", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::distanceTransform(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3571
// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "labels", "distanceType", "maskSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, labels: *const c_void, distance_type: i32, mask_size: i32, ocvrs_return: *mut Result<()>);
// distanceTransform(InputArray, OutputArray, OutputArray, int, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3571
// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "labels", "distanceType", "maskSize", "labelType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, labels: *const c_void, distance_type: i32, mask_size: i32, label_type: i32, ocvrs_return: *mut Result<()>);
// cv::distanceTransform(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3586
// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "distanceType", "maskSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, distance_type: i32, mask_size: i32, ocvrs_return: *mut Result<()>);
// distanceTransform(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3586
// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "distanceType", "maskSize", "dstType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, distance_type: i32, mask_size: i32, dst_type: i32, ocvrs_return: *mut Result<()>);
// cv::divSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3052
// ("cv::divSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// divSpectrums(InputArray, InputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3052
// ("cv::divSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags", "conjB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
pub fn cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, conj_b: bool, ocvrs_return: *mut Result<()>);
// cv::drawContours(InputOutputArray, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4785
// ("cv::drawContours", vec![(pred!(mut, ["image", "contours", "contourIdx", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR(image: *const c_void, contours: *const c_void, contour_idx: i32, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// drawContours(InputOutputArray, InputArrayOfArrays, int, const Scalar &, int, int, InputArray, int, Point)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4785
// ("cv::drawContours", vec![(pred!(mut, ["image", "contours", "contourIdx", "color", "thickness", "lineType", "hierarchy", "maxLevel", "offset"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "const cv::Scalar*", "int", "int", "const cv::_InputArray*", "int", "cv::Point"]), _)]),
pub fn cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR_int_int_const__InputArrayR_int_Point(image: *const c_void, contours: *const c_void, contour_idx: i32, color: *const core::Scalar, thickness: i32, line_type: i32, hierarchy: *const c_void, max_level: i32, offset: *const core::Point, ocvrs_return: *mut Result<()>);
// cv::drawMarker(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4668
// ("cv::drawMarker", vec![(pred!(mut, ["img", "position", "color"], ["const cv::_InputOutputArray*", "cv::Point", "const cv::Scalar*"]), _)]),
pub fn cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR(img: *const c_void, position: *const core::Point, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// drawMarker(InputOutputArray, Point, const Scalar &, int, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4668
// ("cv::drawMarker", vec![(pred!(mut, ["img", "position", "color", "markerType", "markerSize", "thickness", "line_type"], ["const cv::_InputOutputArray*", "cv::Point", "const cv::Scalar*", "int", "int", "int", "int"]), _)]),
pub fn cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR_int_int_int_int(img: *const c_void, position: *const core::Point, color: *const core::Scalar, marker_type: i32, marker_size: i32, thickness: i32, line_type: i32, ocvrs_return: *mut Result<()>);
// ellipse2Poly(Point2d, Size2d, int, int, int, int, std::vector<Point2d> &)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4843
// ("cv::ellipse2Poly", vec![(pred!(mut, ["center", "axes", "angle", "arcStart", "arcEnd", "delta", "pts"], ["cv::Point2d", "cv::Size2d", "int", "int", "int", "int", "std::vector<cv::Point2d>*"]), _)]),
pub fn cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vectorLPoint2dGR(center: *const core::Point2d, axes: *const core::Size2d, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void, ocvrs_return: *mut Result<()>);
// ellipse2Poly(Point, Size, int, int, int, int, std::vector<Point> &)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4830
// ("cv::ellipse2Poly", vec![(pred!(mut, ["center", "axes", "angle", "arcStart", "arcEnd", "delta", "pts"], ["cv::Point", "cv::Size", "int", "int", "int", "int", "std::vector<cv::Point>*"]), _)]),
pub fn cv_ellipse2Poly_Point_Size_int_int_int_int_vectorLPointGR(center: *const core::Point, axes: *const core::Size, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ellipse(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4634
// ("cv::ellipse", vec![(pred!(mut, ["img", "center", "axes", "angle", "startAngle", "endAngle", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Size", "double", "double", "double", "const cv::Scalar*"]), _)]),
pub fn cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR(img: *const c_void, center: *const core::Point, axes: *const core::Size, angle: f64, start_angle: f64, end_angle: f64, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// ellipse(InputOutputArray, Point, Size, double, double, double, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4634
// ("cv::ellipse", vec![(pred!(mut, ["img", "center", "axes", "angle", "startAngle", "endAngle", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Size", "double", "double", "double", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR_int_int_int(img: *const c_void, center: *const core::Point, axes: *const core::Size, angle: f64, start_angle: f64, end_angle: f64, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// cv::ellipse(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4648
// ("cv::ellipse", vec![(pred!(mut, ["img", "box", "color"], ["const cv::_InputOutputArray*", "const cv::RotatedRect*", "const cv::Scalar*"]), _)]),
pub fn cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR(img: *const c_void, box_: *const core::RotatedRect, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// ellipse(InputOutputArray, const RotatedRect &, const Scalar &, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4648
// ("cv::ellipse", vec![(pred!(mut, ["img", "box", "color", "thickness", "lineType"], ["const cv::_InputOutputArray*", "const cv::RotatedRect*", "const cv::Scalar*", "int", "int"]), _)]),
pub fn cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int(img: *const c_void, box_: *const core::RotatedRect, color: *const core::Scalar, thickness: i32, line_type: i32, ocvrs_return: *mut Result<()>);
// equalizeHist(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3345
// ("cv::equalizeHist", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_equalizeHist_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::erode(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2315
// ("cv::erode", vec![(pred!(mut, ["src", "dst", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, kernel: *const c_void, ocvrs_return: *mut Result<()>);
// erode(InputArray, OutputArray, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2315
// ("cv::erode", vec![(pred!(mut, ["src", "dst", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::fillConvexPoly(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4689
// ("cv::fillConvexPoly", vec![(pred!(mut, ["img", "points", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img: *const c_void, points: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// fillConvexPoly(InputOutputArray, InputArray, const Scalar &, int, int)(InputOutputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4689
// ("cv::fillConvexPoly", vec![(pred!(mut, ["img", "points", "color", "lineType", "shift"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "int"]), _)]),
pub fn cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int(img: *const c_void, points: *const c_void, color: *const core::Scalar, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// cv::fillPoly(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4716
// ("cv::fillPoly", vec![(pred!(mut, ["img", "pts", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img: *const c_void, pts: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// fillPoly(InputOutputArray, InputArrayOfArrays, const Scalar &, int, int, Point)(InputOutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4716
// ("cv::fillPoly", vec![(pred!(mut, ["img", "pts", "color", "lineType", "shift", "offset"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "int", "cv::Point"]), _)]),
pub fn cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int_Point(img: *const c_void, pts: *const c_void, color: *const core::Scalar, line_type: i32, shift: i32, offset: *const core::Point, ocvrs_return: *mut Result<()>);
// cv::filter2D(InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1701
// ("cv::filter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(src: *const c_void, dst: *const c_void, ddepth: i32, kernel: *const c_void, ocvrs_return: *mut Result<()>);
// filter2D(InputArray, OutputArray, int, InputArray, Point, double, int)(InputArray, OutputArray, Primitive, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1701
// ("cv::filter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernel", "anchor", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::Point", "double", "int"]), _)]),
pub fn cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, kernel: *const c_void, anchor: *const core::Point, delta: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// findContoursLinkRuns(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4048
// ("cv::findContoursLinkRuns", vec![(pred!(mut, ["image", "contours"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findContoursLinkRuns_const__InputArrayR_const__OutputArrayR(image: *const c_void, contours: *const c_void, ocvrs_return: *mut Result<()>);
// findContoursLinkRuns(InputArray, OutputArrayOfArrays, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4045
// ("cv::findContoursLinkRuns", vec![(pred!(mut, ["image", "contours", "hierarchy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_findContoursLinkRuns_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, contours: *const c_void, hierarchy: *const c_void, ocvrs_return: *mut Result<()>);
// cv::findContours(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4029
// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "hierarchy", "mode", "method"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(image: *const c_void, contours: *const c_void, hierarchy: *const c_void, mode: i32, method: i32, ocvrs_return: *mut Result<()>);
// findContours(InputArray, OutputArrayOfArrays, OutputArray, int, int, Point)(InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4029
// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "hierarchy", "mode", "method", "offset"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "cv::Point"]), _)]),
pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point(image: *const c_void, contours: *const c_void, hierarchy: *const c_void, mode: i32, method: i32, offset: *const core::Point, ocvrs_return: *mut Result<()>);
// cv::findContours(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4034
// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "mode", "method"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_int_int(image: *const c_void, contours: *const c_void, mode: i32, method: i32, ocvrs_return: *mut Result<()>);
// findContours(InputArray, OutputArrayOfArrays, int, int, Point)(InputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4034
// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "mode", "method", "offset"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::Point"]), _)]),
pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_int_int_Point(image: *const c_void, contours: *const c_void, mode: i32, method: i32, offset: *const core::Point, ocvrs_return: *mut Result<()>);
// fitEllipseAMS(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4340
// ("cv::fitEllipseAMS", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
pub fn cv_fitEllipseAMS_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<core::RotatedRect>);
// fitEllipseDirect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4385
// ("cv::fitEllipseDirect", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
pub fn cv_fitEllipseDirect_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<core::RotatedRect>);
// fitEllipse(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4303
// ("cv::fitEllipse", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
pub fn cv_fitEllipse_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<core::RotatedRect>);
// fitLine(InputArray, OutputArray, int, double, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4421
// ("cv::fitLine", vec![(pred!(mut, ["points", "line", "distType", "param", "reps", "aeps"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "double"]), _)]),
pub fn cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(points: *const c_void, line: *const c_void, dist_type: i32, param: f64, reps: f64, aeps: f64, ocvrs_return: *mut Result<()>);
// cv::floodFill(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3674
// ("cv::floodFill", vec![(pred!(mut, ["image", "seedPoint", "newVal"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Scalar"]), _)]),
pub fn cv_floodFill_const__InputOutputArrayR_Point_Scalar(image: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, ocvrs_return: *mut Result<i32>);
// floodFill(InputOutputArray, Point, Scalar, Rect *, Scalar, Scalar, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3674
// ("cv::floodFill", vec![(pred!(mut, ["image", "seedPoint", "newVal", "rect", "loDiff", "upDiff", "flags"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Scalar", "cv::Rect*", "cv::Scalar", "cv::Scalar", "int"]), _)]),
pub fn cv_floodFill_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(image: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, rect: *mut core::Rect, lo_diff: *const core::Scalar, up_diff: *const core::Scalar, flags: i32, ocvrs_return: *mut Result<i32>);
// cv::floodFill(InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3661
// ("cv::floodFill", vec![(pred!(mut, ["image", "mask", "seedPoint", "newVal"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Point", "cv::Scalar"]), _)]),
pub fn cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar(image: *const c_void, mask: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, ocvrs_return: *mut Result<i32>);
// floodFill(InputOutputArray, InputOutputArray, Point, Scalar, Rect *, Scalar, Scalar, int)(InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3661
// ("cv::floodFill", vec![(pred!(mut, ["image", "mask", "seedPoint", "newVal", "rect", "loDiff", "upDiff", "flags"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Point", "cv::Scalar", "cv::Rect*", "cv::Scalar", "cv::Scalar", "int"]), _)]),
pub fn cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(image: *const c_void, mask: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, rect: *mut core::Rect, lo_diff: *const core::Scalar, up_diff: *const core::Scalar, flags: i32, ocvrs_return: *mut Result<i32>);
// getAffineTransform(const Point2f *, const Point2f *)(FixedArray, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2609
// ("cv::getAffineTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::Point2f*", "const cv::Point2f*"]), _)]),
pub fn cv_getAffineTransform_const_Point2fXX_const_Point2fXX(src: *const [core::Point2f; 3], dst: *const [core::Point2f; 3], ocvrs_return: *mut Result<*mut c_void>);
// getAffineTransform(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2646
// ("cv::getAffineTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_getAffineTransform_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::getDerivKernels(OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1462
// ("cv::getDerivKernels", vec![(pred!(mut, ["kx", "ky", "dx", "dy", "ksize"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
pub fn cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int(kx: *const c_void, ky: *const c_void, dx: i32, dy: i32, ksize: i32, ocvrs_return: *mut Result<()>);
// getDerivKernels(OutputArray, OutputArray, int, int, int, bool, int)(OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1462
// ("cv::getDerivKernels", vec![(pred!(mut, ["kx", "ky", "dx", "dy", "ksize", "normalize", "ktype"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int", "bool", "int"]), _)]),
pub fn cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int_bool_int(kx: *const c_void, ky: *const c_void, dx: i32, dy: i32, ksize: i32, normalize: bool, ktype: i32, ocvrs_return: *mut Result<()>);
// cv::getFontScaleFromHeight(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4928
// ("cv::getFontScaleFromHeight", vec![(pred!(mut, ["fontFace", "pixelHeight"], ["const int", "const int"]), _)]),
pub fn cv_getFontScaleFromHeight_const_int_const_int(font_face: i32, pixel_height: i32, ocvrs_return: *mut Result<f64>);
// getFontScaleFromHeight(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4928
// ("cv::getFontScaleFromHeight", vec![(pred!(mut, ["fontFace", "pixelHeight", "thickness"], ["const int", "const int", "const int"]), _)]),
pub fn cv_getFontScaleFromHeight_const_int_const_int_const_int(font_face: i32, pixel_height: i32, thickness: i32, ocvrs_return: *mut Result<f64>);
// cv::getGaborKernel(SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1479
// ("cv::getGaborKernel", vec![(pred!(mut, ["ksize", "sigma", "theta", "lambd", "gamma"], ["cv::Size", "double", "double", "double", "double"]), _)]),
pub fn cv_getGaborKernel_Size_double_double_double_double(ksize: *const core::Size, sigma: f64, theta: f64, lambd: f64, gamma: f64, ocvrs_return: *mut Result<*mut c_void>);
// getGaborKernel(Size, double, double, double, double, double, int)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1479
// ("cv::getGaborKernel", vec![(pred!(mut, ["ksize", "sigma", "theta", "lambd", "gamma", "psi", "ktype"], ["cv::Size", "double", "double", "double", "double", "double", "int"]), _)]),
pub fn cv_getGaborKernel_Size_double_double_double_double_double_int(ksize: *const core::Size, sigma: f64, theta: f64, lambd: f64, gamma: f64, psi: f64, ktype: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::getGaussianKernel(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1442
// ("cv::getGaussianKernel", vec![(pred!(mut, ["ksize", "sigma"], ["int", "double"]), _)]),
pub fn cv_getGaussianKernel_int_double(ksize: i32, sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
// getGaussianKernel(int, double, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1442
// ("cv::getGaussianKernel", vec![(pred!(mut, ["ksize", "sigma", "ktype"], ["int", "double", "int"]), _)]),
pub fn cv_getGaussianKernel_int_double_int(ksize: i32, sigma: f64, ktype: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::getPerspectiveTransform(FixedArray, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2643
// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::Point2f**", "const cv::Point2f**"]), _)]),
pub fn cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX(src: *const [core::Point2f; 4], dst: *const [core::Point2f; 4], ocvrs_return: *mut Result<*mut c_void>);
// getPerspectiveTransform(const Point2f *, const Point2f *, int)(FixedArray, FixedArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2643
// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst", "solveMethod"], ["const cv::Point2f*", "const cv::Point2f*", "int"]), _)]),
pub fn cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX_int(src: *const [core::Point2f; 4], dst: *const [core::Point2f; 4], solve_method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::getPerspectiveTransform(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2640
// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getPerspectiveTransform(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2640
// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst", "solveMethod"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(src: *const c_void, dst: *const c_void, solve_method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::getRectSubPix(InputArray, SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2668
// ("cv::getRectSubPix", vec![(pred!(mut, ["image", "patchSize", "center", "patch"], ["const cv::_InputArray*", "cv::Size", "cv::Point2f", "const cv::_OutputArray*"]), _)]),
pub fn cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR(image: *const c_void, patch_size: *const core::Size, center: *const core::Point2f, patch: *const c_void, ocvrs_return: *mut Result<()>);
// getRectSubPix(InputArray, Size, Point2f, OutputArray, int)(InputArray, SimpleClass, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2668
// ("cv::getRectSubPix", vec![(pred!(mut, ["image", "patchSize", "center", "patch", "patchType"], ["const cv::_InputArray*", "cv::Size", "cv::Point2f", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR_int(image: *const c_void, patch_size: *const core::Size, center: *const core::Point2f, patch: *const c_void, patch_type: i32, ocvrs_return: *mut Result<()>);
// getRotationMatrix2D(Point2f, double, double)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2583
// ("cv::getRotationMatrix2D", vec![(pred!(mut, ["center", "angle", "scale"], ["cv::Point2f", "double", "double"]), _)]),
pub fn cv_getRotationMatrix2D_Point2f_double_double(center: *const core::Point2f, angle: f64, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(not(target_os = "windows"))]
// getRotationMatrix2D_(Point2f, double, double)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2586
// ("cv::getRotationMatrix2D_", vec![(pred!(mut, ["center", "angle", "scale"], ["cv::Point2f", "double", "double"]), _)]),
pub fn cv_getRotationMatrix2D__Point2f_double_double(center: *const core::Point2f, angle: f64, scale: f64, ocvrs_return: *mut Result<core::Matx23d>);
// cv::getStructuringElement(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1498
// ("cv::getStructuringElement", vec![(pred!(mut, ["shape", "ksize"], ["int", "cv::Size"]), _)]),
pub fn cv_getStructuringElement_int_Size(shape: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// getStructuringElement(int, Size, Point)(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1498
// ("cv::getStructuringElement", vec![(pred!(mut, ["shape", "ksize", "anchor"], ["int", "cv::Size", "cv::Point"]), _)]),
pub fn cv_getStructuringElement_int_Size_Point(shape: i32, ksize: *const core::Size, anchor: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// getTextSize(const String &, int, double, int, int *)(InString, Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4914
// ("cv::getTextSize", vec![(pred!(mut, ["text", "fontFace", "fontScale", "thickness", "baseLine"], ["const cv::String*", "int", "double", "int", "int*"]), _)]),
pub fn cv_getTextSize_const_StringR_int_double_int_intX(text: *const c_char, font_face: i32, font_scale: f64, thickness: i32, base_line: *mut i32, ocvrs_return: *mut Result<core::Size>);
// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2095
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, ocvrs_return: *mut Result<()>);
// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2131
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "cornersQuality"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, corners_quality: *const c_void, ocvrs_return: *mut Result<()>);
// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, OutputArray, int, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2131
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "cornersQuality", "blockSize", "gradientSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "bool", "double"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, corners_quality: *const c_void, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<()>);
// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2095
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "bool", "double"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, block_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<()>);
// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2100
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "gradientSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, block_size: i32, gradient_size: i32, ocvrs_return: *mut Result<()>);
// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, int, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2100
// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "gradientSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "int", "bool", "double"]), _)]),
pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<()>);
// cv::grabCut(InputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3505
// ("cv::grabCut", vec![(pred!(mut, ["img", "mask", "rect", "bgdModel", "fgdModel", "iterCount"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Rect", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
pub fn cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int(img: *const c_void, mask: *const c_void, rect: *const core::Rect, bgd_model: *const c_void, fgd_model: *const c_void, iter_count: i32, ocvrs_return: *mut Result<()>);
// grabCut(InputArray, InputOutputArray, Rect, InputOutputArray, InputOutputArray, int, int)(InputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3505
// ("cv::grabCut", vec![(pred!(mut, ["img", "mask", "rect", "bgdModel", "fgdModel", "iterCount", "mode"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Rect", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int", "int"]), _)]),
pub fn cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int_int(img: *const c_void, mask: *const c_void, rect: *const core::Rect, bgd_model: *const c_void, fgd_model: *const c_void, iter_count: i32, mode: i32, ocvrs_return: *mut Result<()>);
// cv::integral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2894
// ("cv::integral", vec![(pred!(mut, ["src", "sum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR(src: *const c_void, sum: *const c_void, ocvrs_return: *mut Result<()>);
// cv::integral(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2897
// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, sum: *const c_void, sqsum: *const c_void, ocvrs_return: *mut Result<()>);
// cv::integral(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2889
// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "tilted"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, sum: *const c_void, sqsum: *const c_void, tilted: *const c_void, ocvrs_return: *mut Result<()>);
// integral(InputArray, OutputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2889
// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "tilted", "sdepth", "sqdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, sum: *const c_void, sqsum: *const c_void, tilted: *const c_void, sdepth: i32, sqdepth: i32, ocvrs_return: *mut Result<()>);
// integral(InputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2897
// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "sdepth", "sqdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, sum: *const c_void, sqsum: *const c_void, sdepth: i32, sqdepth: i32, ocvrs_return: *mut Result<()>);
// integral(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2894
// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_integral_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, sum: *const c_void, sdepth: i32, ocvrs_return: *mut Result<()>);
// cv::intersectConvexConvex(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4286
// ("cv::intersectConvexConvex", vec![(pred!(mut, ["p1", "p2", "p12"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR(p1: *const c_void, p2: *const c_void, p12: *const c_void, ocvrs_return: *mut Result<f32>);
// intersectConvexConvex(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4286
// ("cv::intersectConvexConvex", vec![(pred!(mut, ["p1", "p2", "p12", "handleNested"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(p1: *const c_void, p2: *const c_void, p12: *const c_void, handle_nested: bool, ocvrs_return: *mut Result<f32>);
// invertAffineTransform(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2622
// ("cv::invertAffineTransform", vec![(pred!(mut, ["M", "iM"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(m: *const c_void, i_m: *const c_void, ocvrs_return: *mut Result<()>);
// isContourConvex(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4267
// ("cv::isContourConvex", vec![(pred!(mut, ["contour"], ["const cv::_InputArray*"]), _)]),
pub fn cv_isContourConvex_const__InputArrayR(contour: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::line(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4543
// ("cv::line", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
pub fn cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// line(InputOutputArray, Point, Point, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4543
// ("cv::line", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// linearPolar(InputArray, OutputArray, Point2f, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2755
// ("cv::linearPolar", vec![(pred!(mut, ["src", "dst", "center", "maxRadius", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Point2f", "double", "int"]), _)]),
pub fn cv_linearPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(src: *const c_void, dst: *const c_void, center: *const core::Point2f, max_radius: f64, flags: i32, ocvrs_return: *mut Result<()>);
// logPolar(InputArray, OutputArray, Point2f, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2714
// ("cv::logPolar", vec![(pred!(mut, ["src", "dst", "center", "M", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Point2f", "double", "int"]), _)]),
pub fn cv_logPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(src: *const c_void, dst: *const c_void, center: *const core::Point2f, m: f64, flags: i32, ocvrs_return: *mut Result<()>);
// matchShapes(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4204
// ("cv::matchShapes", vec![(pred!(mut, ["contour1", "contour2", "method", "parameter"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
pub fn cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(contour1: *const c_void, contour2: *const c_void, method: i32, parameter: f64, ocvrs_return: *mut Result<f64>);
// cv::matchTemplate(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3915
// ("cv::matchTemplate", vec![(pred!(mut, ["image", "templ", "result", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(image: *const c_void, templ: *const c_void, result: *const c_void, method: i32, ocvrs_return: *mut Result<()>);
// matchTemplate(InputArray, InputArray, OutputArray, int, InputArray)(InputArray, InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3915
// ("cv::matchTemplate", vec![(pred!(mut, ["image", "templ", "result", "method", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(image: *const c_void, templ: *const c_void, result: *const c_void, method: i32, mask: *const c_void, ocvrs_return: *mut Result<()>);
// medianBlur(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1520
// ("cv::medianBlur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_medianBlur_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, ksize: i32, ocvrs_return: *mut Result<()>);
// minAreaRect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4147
// ("cv::minAreaRect", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
pub fn cv_minAreaRect_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<core::RotatedRect>);
// minEnclosingCircle(InputArray, Point2f &, float &)(InputArray, SimpleClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4168
// ("cv::minEnclosingCircle", vec![(pred!(mut, ["points", "center", "radius"], ["const cv::_InputArray*", "cv::Point2f*", "float*"]), _)]),
pub fn cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(points: *const c_void, center: *mut core::Point2f, radius: *mut f32, ocvrs_return: *mut Result<()>);
// minEnclosingTriangle(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4193
// ("cv::minEnclosingTriangle", vec![(pred!(mut, ["points", "triangle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(points: *const c_void, triangle: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::moments(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3811
// ("cv::moments", vec![(pred!(mut, ["array"], ["const cv::_InputArray*"]), _)]),
pub fn cv_moments_const__InputArrayR(array: *const c_void, ocvrs_return: *mut Result<core::Moments>);
// moments(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3811
// ("cv::moments", vec![(pred!(mut, ["array", "binaryImage"], ["const cv::_InputArray*", "bool"]), _)]),
pub fn cv_moments_const__InputArrayR_bool(array: *const c_void, binary_image: bool, ocvrs_return: *mut Result<core::Moments>);
// morphologyDefaultBorderValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1483
// ("cv::morphologyDefaultBorderValue", vec![(pred!(mut, [], []), _)]),
pub fn cv_morphologyDefaultBorderValue(ocvrs_return: *mut Result<core::Scalar>);
// cv::morphologyEx(InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2376
// ("cv::morphologyEx", vec![(pred!(mut, ["src", "dst", "op", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(src: *const c_void, dst: *const c_void, op: i32, kernel: *const c_void, ocvrs_return: *mut Result<()>);
// morphologyEx(InputArray, OutputArray, int, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, Primitive, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2376
// ("cv::morphologyEx", vec![(pred!(mut, ["src", "dst", "op", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, op: i32, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::phaseCorrelate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3019
// ("cv::phaseCorrelate", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_phaseCorrelate_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<core::Point2d>);
// phaseCorrelate(InputArray, InputArray, InputArray, double *)(InputArray, InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3019
// ("cv::phaseCorrelate", vec![(pred!(mut, ["src1", "src2", "window", "response"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double*"]), _)]),
pub fn cv_phaseCorrelate_const__InputArrayR_const__InputArrayR_const__InputArrayR_doubleX(src1: *const c_void, src2: *const c_void, window: *const c_void, response: *mut f64, ocvrs_return: *mut Result<core::Point2d>);
// pointPolygonTest(InputArray, Point2f, bool)(InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4440
// ("cv::pointPolygonTest", vec![(pred!(mut, ["contour", "pt", "measureDist"], ["const cv::_InputArray*", "cv::Point2f", "bool"]), _)]),
pub fn cv_pointPolygonTest_const__InputArrayR_Point2f_bool(contour: *const c_void, pt: *const core::Point2f, measure_dist: bool, ocvrs_return: *mut Result<f64>);
// cv::polylines(InputOutputArray, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4739
// ("cv::polylines", vec![(pred!(mut, ["img", "pts", "isClosed", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "const cv::Scalar*"]), _)]),
pub fn cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR(img: *const c_void, pts: *const c_void, is_closed: bool, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// polylines(InputOutputArray, InputArrayOfArrays, bool, const Scalar &, int, int, int)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4739
// ("cv::polylines", vec![(pred!(mut, ["img", "pts", "isClosed", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR_int_int_int(img: *const c_void, pts: *const c_void, is_closed: bool, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// cv::preCornerDetect(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2004
// ("cv::preCornerDetect", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, ksize: i32, ocvrs_return: *mut Result<()>);
// preCornerDetect(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2004
// ("cv::preCornerDetect", vec![(pred!(mut, ["src", "dst", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, ksize: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::putText(InputOutputArray, InString, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4864
// ("cv::putText", vec![(pred!(mut, ["img", "text", "org", "fontFace", "fontScale", "color"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "double", "cv::Scalar"]), _)]),
pub fn cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar(img: *const c_void, text: *const c_char, org: *const core::Point, font_face: i32, font_scale: f64, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// putText(InputOutputArray, const String &, Point, int, double, Scalar, int, int, bool)(InputOutputArray, InString, SimpleClass, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4864
// ("cv::putText", vec![(pred!(mut, ["img", "text", "org", "fontFace", "fontScale", "color", "thickness", "lineType", "bottomLeftOrigin"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "double", "cv::Scalar", "int", "int", "bool"]), _)]),
pub fn cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar_int_int_bool(img: *const c_void, text: *const c_char, org: *const core::Point, font_face: i32, font_scale: f64, color: *const core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool, ocvrs_return: *mut Result<()>);
// cv::pyrDown(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3145
// ("cv::pyrDown", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_pyrDown_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// pyrDown(InputArray, OutputArray, const Size &, int)(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3145
// ("cv::pyrDown", vec![(pred!(mut, ["src", "dst", "dstsize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Size*", "int"]), _)]),
pub fn cv_pyrDown_const__InputArrayR_const__OutputArrayR_const_SizeR_int(src: *const c_void, dst: *const c_void, dstsize: *const core::Size, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::pyrMeanShiftFiltering(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3473
// ("cv::pyrMeanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, sp: f64, sr: f64, ocvrs_return: *mut Result<()>);
// pyrMeanShiftFiltering(InputArray, OutputArray, double, double, int, TermCriteria)(InputArray, OutputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3473
// ("cv::pyrMeanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr", "maxLevel", "termcrit"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "cv::TermCriteria"]), _)]),
pub fn cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double_int_TermCriteria(src: *const c_void, dst: *const c_void, sp: f64, sr: f64, max_level: i32, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::pyrUp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3165
// ("cv::pyrUp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_pyrUp_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// pyrUp(InputArray, OutputArray, const Size &, int)(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3165
// ("cv::pyrUp", vec![(pred!(mut, ["src", "dst", "dstsize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Size*", "int"]), _)]),
pub fn cv_pyrUp_const__InputArrayR_const__OutputArrayR_const_SizeR_int(src: *const c_void, dst: *const c_void, dstsize: *const core::Size, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::rectangle(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4576
// ("cv::rectangle", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
pub fn cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// rectangle(InputOutputArray, Point, Point, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4576
// ("cv::rectangle", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// cv::rectangle(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4585
// ("cv::rectangle", vec![(pred!(mut, ["img", "rec", "color"], ["const cv::_InputOutputArray*", "cv::Rect", "const cv::Scalar*"]), _)]),
pub fn cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR(img: *const c_void, rec: *const core::Rect, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// rectangle(InputOutputArray, Rect, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4585
// ("cv::rectangle", vec![(pred!(mut, ["img", "rec", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Rect", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR_int_int_int(img: *const c_void, rec: *const core::Rect, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result<()>);
// cv::remap(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2525
// ("cv::remap", vec![(pred!(mut, ["src", "dst", "map1", "map2", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(src: *const c_void, dst: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, ocvrs_return: *mut Result<()>);
// remap(InputArray, OutputArray, InputArray, InputArray, int, int, const Scalar &)(InputArray, OutputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2525
// ("cv::remap", vec![(pred!(mut, ["src", "dst", "map1", "map2", "interpolation", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::resize(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2421
// ("cv::resize", vec![(pred!(mut, ["src", "dst", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_resize_const__InputArrayR_const__OutputArrayR_Size(src: *const c_void, dst: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<()>);
// resize(InputArray, OutputArray, Size, double, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2421
// ("cv::resize", vec![(pred!(mut, ["src", "dst", "dsize", "fx", "fy", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int"]), _)]),
pub fn cv_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int(src: *const c_void, dst: *const c_void, dsize: *const core::Size, fx: f64, fy: f64, interpolation: i32, ocvrs_return: *mut Result<()>);
// rotatedRectangleIntersection(const RotatedRect &, const RotatedRect &, OutputArray)(SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4457
// ("cv::rotatedRectangleIntersection", vec![(pred!(mut, ["rect1", "rect2", "intersectingRegion"], ["const cv::RotatedRect*", "const cv::RotatedRect*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(rect1: *const core::RotatedRect, rect2: *const core::RotatedRect, intersecting_region: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::sepFilter2D(InputArray, OutputArray, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1722
// ("cv::sepFilter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernelX", "kernelY"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, ocvrs_return: *mut Result<()>);
// sepFilter2D(InputArray, OutputArray, int, InputArray, InputArray, Point, double, int)(InputArray, OutputArray, Primitive, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1722
// ("cv::sepFilter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernelX", "kernelY", "anchor", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "double", "int"]), _)]),
pub fn cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR_Point_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, anchor: *const core::Point, delta: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::spatialGradient(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1799
// ("cv::spatialGradient", vec![(pred!(mut, ["src", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, dx: *const c_void, dy: *const c_void, ocvrs_return: *mut Result<()>);
// spatialGradient(InputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1799
// ("cv::spatialGradient", vec![(pred!(mut, ["src", "dx", "dy", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, dx: *const c_void, dy: *const c_void, ksize: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::sqrBoxFilter(InputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1627
// ("cv::sqrBoxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size"]), _)]),
pub fn cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, ocvrs_return: *mut Result<()>);
// sqrBoxFilter(InputArray, OutputArray, int, Size, Point, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1627
// ("cv::sqrBoxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "anchor", "normalize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size", "cv::Point", "bool", "int"]), _)]),
pub fn cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, ocvrs_return: *mut Result<()>);
// stackBlur(InputArray, OutputArray, Size)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1669
// ("cv::stackBlur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_stackBlur_const__InputArrayR_const__OutputArrayR_Size(src: *const c_void, dst: *const c_void, ksize: *const core::Size, ocvrs_return: *mut Result<()>);
// threshold(InputArray, OutputArray, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3084
// ("cv::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
pub fn cv_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(src: *const c_void, dst: *const c_void, thresh: f64, maxval: f64, typ: i32, ocvrs_return: *mut Result<f64>);
// cv::warpAffine(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2449
// ("cv::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
pub fn cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<()>);
// warpAffine(InputArray, OutputArray, InputArray, Size, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2449
// ("cv::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::warpPerspective(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2481
// ("cv::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
pub fn cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<()>);
// warpPerspective(InputArray, OutputArray, InputArray, Size, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2481
// ("cv::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// warpPolar(InputArray, OutputArray, Size, Point2f, double, int)(InputArray, OutputArray, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2846
// ("cv::warpPolar", vec![(pred!(mut, ["src", "dst", "dsize", "center", "maxRadius", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "cv::Point2f", "double", "int"]), _)]),
pub fn cv_warpPolar_const__InputArrayR_const__OutputArrayR_Size_Point2f_double_int(src: *const c_void, dst: *const c_void, dsize: *const core::Size, center: *const core::Point2f, max_radius: f64, flags: i32, ocvrs_return: *mut Result<()>);
// watershed(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3430
// ("cv::watershed", vec![(pred!(mut, ["image", "markers"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_watershed_const__InputArrayR_const__InputOutputArrayR(image: *const c_void, markers: *const c_void, ocvrs_return: *mut Result<()>);
// cv::wrapperEMD(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3393
// ("cv::wrapperEMD", vec![(pred!(mut, ["signature1", "signature2", "distType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int(signature1: *const c_void, signature2: *const c_void, dist_type: i32, ocvrs_return: *mut Result<f32>);
// wrapperEMD(InputArray, InputArray, int, InputArray, Ptr<float>, OutputArray)(InputArray, InputArray, Primitive, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3393
// ("cv::wrapperEMD", vec![(pred!(mut, ["signature1", "signature2", "distType", "cost", "lowerBound", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "cv::Ptr<float>", "const cv::_OutputArray*"]), _)]),
pub fn cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_PtrLfloatG_const__OutputArrayR(signature1: *const c_void, signature2: *const c_void, dist_type: i32, cost: *const c_void, lower_bound: *mut c_void, flow: *const c_void, ocvrs_return: *mut Result<f32>);
// apply(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1047
// ("cv::CLAHE::apply", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// setClipLimit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1053
// ("cv::CLAHE::setClipLimit", vec![(pred!(mut, ["clipLimit"], ["double"]), _)]),
pub fn cv_CLAHE_setClipLimit_double(instance: *mut c_void, clip_limit: f64, ocvrs_return: *mut Result<()>);
// getClipLimit()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1056
// ("cv::CLAHE::getClipLimit", vec![(pred!(const, [], []), _)]),
pub fn cv_CLAHE_getClipLimit_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTilesGridSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1063
// ("cv::CLAHE::setTilesGridSize", vec![(pred!(mut, ["tileGridSize"], ["cv::Size"]), _)]),
pub fn cv_CLAHE_setTilesGridSize_Size(instance: *mut c_void, tile_grid_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getTilesGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1066
// ("cv::CLAHE::getTilesGridSize", vec![(pred!(const, [], []), _)]),
pub fn cv_CLAHE_getTilesGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1068
// ("cv::CLAHE::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_CLAHE_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::CLAHE::to_Algorithm() generated
// ("cv::CLAHE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_CLAHE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::CLAHE::delete() generated
// ("cv::CLAHE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CLAHE_delete(instance: *mut c_void);
// setTemplate(InputArray, Point)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:932
// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["templ", "templCenter"], ["const cv::_InputArray*", "cv::Point"]), _)]),
pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(instance: *mut c_void, templ: *const c_void, templ_center: *const core::Point, ocvrs_return: *mut Result<()>);
// cv::GeneralizedHough::setTemplate(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:932
// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["templ"], ["const cv::_InputArray*"]), _)]),
pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR(instance: *mut c_void, templ: *const c_void, ocvrs_return: *mut Result<()>);
// setTemplate(InputArray, InputArray, InputArray, Point)(InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:933
// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["edges", "dx", "dy", "templCenter"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, templ_center: *const core::Point, ocvrs_return: *mut Result<()>);
// cv::GeneralizedHough::setTemplate(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:933
// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["edges", "dx", "dy"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:936
// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["image", "positions", "votes"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, positions: *const c_void, votes: *const c_void, ocvrs_return: *mut Result<()>);
// cv::GeneralizedHough::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:936
// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["image", "positions"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, positions: *const c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:937
// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["edges", "dx", "dy", "positions", "votes"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, positions: *const c_void, votes: *const c_void, ocvrs_return: *mut Result<()>);
// cv::GeneralizedHough::detect(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:937
// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["edges", "dx", "dy", "positions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, positions: *const c_void, ocvrs_return: *mut Result<()>);
// setCannyLowThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:940
// ("cv::GeneralizedHough::setCannyLowThresh", vec![(pred!(mut, ["cannyLowThresh"], ["int"]), _)]),
pub fn cv_GeneralizedHough_setCannyLowThresh_int(instance: *mut c_void, canny_low_thresh: i32, ocvrs_return: *mut Result<()>);
// getCannyLowThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:941
// ("cv::GeneralizedHough::getCannyLowThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHough_getCannyLowThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCannyHighThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:944
// ("cv::GeneralizedHough::setCannyHighThresh", vec![(pred!(mut, ["cannyHighThresh"], ["int"]), _)]),
pub fn cv_GeneralizedHough_setCannyHighThresh_int(instance: *mut c_void, canny_high_thresh: i32, ocvrs_return: *mut Result<()>);
// getCannyHighThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:945
// ("cv::GeneralizedHough::getCannyHighThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHough_getCannyHighThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinDist(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:948
// ("cv::GeneralizedHough::setMinDist", vec![(pred!(mut, ["minDist"], ["double"]), _)]),
pub fn cv_GeneralizedHough_setMinDist_double(instance: *mut c_void, min_dist: f64, ocvrs_return: *mut Result<()>);
// getMinDist()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:949
// ("cv::GeneralizedHough::getMinDist", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHough_getMinDist_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDp(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:952
// ("cv::GeneralizedHough::setDp", vec![(pred!(mut, ["dp"], ["double"]), _)]),
pub fn cv_GeneralizedHough_setDp_double(instance: *mut c_void, dp: f64, ocvrs_return: *mut Result<()>);
// getDp()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:953
// ("cv::GeneralizedHough::getDp", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHough_getDp_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxBufferSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:956
// ("cv::GeneralizedHough::setMaxBufferSize", vec![(pred!(mut, ["maxBufferSize"], ["int"]), _)]),
pub fn cv_GeneralizedHough_setMaxBufferSize_int(instance: *mut c_void, max_buffer_size: i32, ocvrs_return: *mut Result<()>);
// getMaxBufferSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:957
// ("cv::GeneralizedHough::getMaxBufferSize", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHough_getMaxBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::GeneralizedHough::to_GeneralizedHoughBallard() generated
// ("cv::GeneralizedHough::to_GeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHough_to_GeneralizedHoughBallard(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHough::to_GeneralizedHoughGuil() generated
// ("cv::GeneralizedHough::to_GeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHough_to_GeneralizedHoughGuil(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHough::to_Algorithm() generated
// ("cv::GeneralizedHough::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHough_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHough::delete() generated
// ("cv::GeneralizedHough::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHough_delete(instance: *mut c_void);
// setLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:968
// ("cv::GeneralizedHoughBallard::setLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
pub fn cv_GeneralizedHoughBallard_setLevels_int(instance: *mut c_void, levels: i32, ocvrs_return: *mut Result<()>);
// getLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:969
// ("cv::GeneralizedHoughBallard::getLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughBallard_getLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setVotesThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:972
// ("cv::GeneralizedHoughBallard::setVotesThreshold", vec![(pred!(mut, ["votesThreshold"], ["int"]), _)]),
pub fn cv_GeneralizedHoughBallard_setVotesThreshold_int(instance: *mut c_void, votes_threshold: i32, ocvrs_return: *mut Result<()>);
// getVotesThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:973
// ("cv::GeneralizedHoughBallard::getVotesThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughBallard_getVotesThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::GeneralizedHoughBallard::to_Algorithm() generated
// ("cv::GeneralizedHoughBallard::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughBallard_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHoughBallard::to_GeneralizedHough() generated
// ("cv::GeneralizedHoughBallard::to_GeneralizedHough", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughBallard_to_GeneralizedHough(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHoughBallard::delete() generated
// ("cv::GeneralizedHoughBallard::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughBallard_delete(instance: *mut c_void);
// setXi(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:984
// ("cv::GeneralizedHoughGuil::setXi", vec![(pred!(mut, ["xi"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setXi_double(instance: *mut c_void, xi: f64, ocvrs_return: *mut Result<()>);
// getXi()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:985
// ("cv::GeneralizedHoughGuil::getXi", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getXi_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:988
// ("cv::GeneralizedHoughGuil::setLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
pub fn cv_GeneralizedHoughGuil_setLevels_int(instance: *mut c_void, levels: i32, ocvrs_return: *mut Result<()>);
// getLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:989
// ("cv::GeneralizedHoughGuil::getLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setAngleEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:992
// ("cv::GeneralizedHoughGuil::setAngleEpsilon", vec![(pred!(mut, ["angleEpsilon"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setAngleEpsilon_double(instance: *mut c_void, angle_epsilon: f64, ocvrs_return: *mut Result<()>);
// getAngleEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:993
// ("cv::GeneralizedHoughGuil::getAngleEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getAngleEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinAngle(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:996
// ("cv::GeneralizedHoughGuil::setMinAngle", vec![(pred!(mut, ["minAngle"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setMinAngle_double(instance: *mut c_void, min_angle: f64, ocvrs_return: *mut Result<()>);
// getMinAngle()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:997
// ("cv::GeneralizedHoughGuil::getMinAngle", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getMinAngle_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxAngle(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1000
// ("cv::GeneralizedHoughGuil::setMaxAngle", vec![(pred!(mut, ["maxAngle"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setMaxAngle_double(instance: *mut c_void, max_angle: f64, ocvrs_return: *mut Result<()>);
// getMaxAngle()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1001
// ("cv::GeneralizedHoughGuil::getMaxAngle", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getMaxAngle_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAngleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1004
// ("cv::GeneralizedHoughGuil::setAngleStep", vec![(pred!(mut, ["angleStep"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setAngleStep_double(instance: *mut c_void, angle_step: f64, ocvrs_return: *mut Result<()>);
// getAngleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1005
// ("cv::GeneralizedHoughGuil::getAngleStep", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getAngleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAngleThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1008
// ("cv::GeneralizedHoughGuil::setAngleThresh", vec![(pred!(mut, ["angleThresh"], ["int"]), _)]),
pub fn cv_GeneralizedHoughGuil_setAngleThresh_int(instance: *mut c_void, angle_thresh: i32, ocvrs_return: *mut Result<()>);
// getAngleThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1009
// ("cv::GeneralizedHoughGuil::getAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getAngleThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1012
// ("cv::GeneralizedHoughGuil::setMinScale", vec![(pred!(mut, ["minScale"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setMinScale_double(instance: *mut c_void, min_scale: f64, ocvrs_return: *mut Result<()>);
// getMinScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1013
// ("cv::GeneralizedHoughGuil::getMinScale", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getMinScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1016
// ("cv::GeneralizedHoughGuil::setMaxScale", vec![(pred!(mut, ["maxScale"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setMaxScale_double(instance: *mut c_void, max_scale: f64, ocvrs_return: *mut Result<()>);
// getMaxScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1017
// ("cv::GeneralizedHoughGuil::getMaxScale", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getMaxScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1020
// ("cv::GeneralizedHoughGuil::setScaleStep", vec![(pred!(mut, ["scaleStep"], ["double"]), _)]),
pub fn cv_GeneralizedHoughGuil_setScaleStep_double(instance: *mut c_void, scale_step: f64, ocvrs_return: *mut Result<()>);
// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1021
// ("cv::GeneralizedHoughGuil::getScaleStep", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1024
// ("cv::GeneralizedHoughGuil::setScaleThresh", vec![(pred!(mut, ["scaleThresh"], ["int"]), _)]),
pub fn cv_GeneralizedHoughGuil_setScaleThresh_int(instance: *mut c_void, scale_thresh: i32, ocvrs_return: *mut Result<()>);
// getScaleThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1025
// ("cv::GeneralizedHoughGuil::getScaleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getScaleThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPosThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1028
// ("cv::GeneralizedHoughGuil::setPosThresh", vec![(pred!(mut, ["posThresh"], ["int"]), _)]),
pub fn cv_GeneralizedHoughGuil_setPosThresh_int(instance: *mut c_void, pos_thresh: i32, ocvrs_return: *mut Result<()>);
// getPosThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1029
// ("cv::GeneralizedHoughGuil::getPosThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_getPosThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::GeneralizedHoughGuil::to_Algorithm() generated
// ("cv::GeneralizedHoughGuil::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHoughGuil::to_GeneralizedHough() generated
// ("cv::GeneralizedHoughGuil::to_GeneralizedHough", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_to_GeneralizedHough(instance: *mut c_void) -> *mut c_void;
// cv::GeneralizedHoughGuil::delete() generated
// ("cv::GeneralizedHoughGuil::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GeneralizedHoughGuil_delete(instance: *mut c_void);
// LineIterator(const Mat &, Point, Point, int, bool)(TraitClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4979
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["img", "pt1", "pt2", "connectivity", "leftToRight"], ["const cv::Mat*", "cv::Point", "cv::Point", "int", "bool"]), _)]),
pub fn cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::LineIterator::LineIterator(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4979
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["img", "pt1", "pt2"], ["const cv::Mat*", "cv::Point", "cv::Point"]), _)]),
pub fn cv_LineIterator_LineIterator_const_MatR_Point_Point(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// LineIterator(Point, Point, int, bool)(SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4985
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["pt1", "pt2", "connectivity", "leftToRight"], ["cv::Point", "cv::Point", "int", "bool"]), _)]),
pub fn cv_LineIterator_LineIterator_Point_Point_int_bool(pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::LineIterator::LineIterator(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4985
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["pt1", "pt2"], ["cv::Point", "cv::Point"]), _)]),
pub fn cv_LineIterator_LineIterator_Point_Point(pt1: *const core::Point, pt2: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// LineIterator(Size, Point, Point, int, bool)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4995
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaSize", "pt1", "pt2", "connectivity", "leftToRight"], ["cv::Size", "cv::Point", "cv::Point", "int", "bool"]), _)]),
pub fn cv_LineIterator_LineIterator_Size_Point_Point_int_bool(bounding_area_size: *const core::Size, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::LineIterator::LineIterator(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4995
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaSize", "pt1", "pt2"], ["cv::Size", "cv::Point", "cv::Point"]), _)]),
pub fn cv_LineIterator_LineIterator_Size_Point_Point(bounding_area_size: *const core::Size, pt1: *const core::Point, pt2: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// LineIterator(Rect, Point, Point, int, bool)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5002
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaRect", "pt1", "pt2", "connectivity", "leftToRight"], ["cv::Rect", "cv::Point", "cv::Point", "int", "bool"]), _)]),
pub fn cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(bounding_area_rect: *const core::Rect, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::LineIterator::LineIterator(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5002
// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaRect", "pt1", "pt2"], ["cv::Rect", "cv::Point", "cv::Point"]), _)]),
pub fn cv_LineIterator_LineIterator_Rect_Point_Point(bounding_area_rect: *const core::Rect, pt1: *const core::Point, pt2: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// init(const Mat *, Rect, Point, Point, int, bool)(TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5008
// ("cv::LineIterator::init", vec![(pred!(mut, ["img", "boundingAreaRect", "pt1", "pt2", "connectivity", "leftToRight"], ["const cv::Mat*", "cv::Rect", "cv::Point", "cv::Point", "int", "bool"]), _)]),
pub fn cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(instance: *mut c_void, img: *const c_void, bounding_area_rect: *const core::Rect, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<()>);
// operator*()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5012
// ("cv::LineIterator::operator*", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineIterator_operatorX(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
// operator++()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5018
// ("cv::LineIterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// pos()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5028
// ("cv::LineIterator::pos", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_pos_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point>);
// cv::LineIterator::ptr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
// ("cv::LineIterator::ptr", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPtr_const(instance: *const c_void) -> *const u8;
// cv::LineIterator::ptrMut() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
// ("cv::LineIterator::ptrMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineIterator_propPtr(instance: *mut c_void) -> *mut u8;
// cv::LineIterator::setPtr(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
// ("cv::LineIterator::setPtr", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
pub fn cv_LineIterator_propPtr_unsigned_charX(instance: *mut c_void, val: *const u8);
// cv::LineIterator::ptr0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5031
// ("cv::LineIterator::ptr0", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPtr0_const(instance: *const c_void) -> *const u8;
// cv::LineIterator::step() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
// ("cv::LineIterator::step", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propStep_const(instance: *const c_void) -> i32;
// cv::LineIterator::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
// ("cv::LineIterator::setStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propStep_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::elemSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
// ("cv::LineIterator::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propElemSize_const(instance: *const c_void) -> i32;
// cv::LineIterator::setElemSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
// ("cv::LineIterator::setElemSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propElemSize_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::err() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
// ("cv::LineIterator::err", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propErr_const(instance: *const c_void) -> i32;
// cv::LineIterator::setErr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
// ("cv::LineIterator::setErr", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propErr_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::count() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
// ("cv::LineIterator::count", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propCount_const(instance: *const c_void) -> i32;
// cv::LineIterator::setCount(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
// ("cv::LineIterator::setCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propCount_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::minusDelta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
// ("cv::LineIterator::minusDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propMinusDelta_const(instance: *const c_void) -> i32;
// cv::LineIterator::setMinusDelta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
// ("cv::LineIterator::setMinusDelta", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propMinusDelta_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::plusDelta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
// ("cv::LineIterator::plusDelta", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPlusDelta_const(instance: *const c_void) -> i32;
// cv::LineIterator::setPlusDelta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
// ("cv::LineIterator::setPlusDelta", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propPlusDelta_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::minusStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
// ("cv::LineIterator::minusStep", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propMinusStep_const(instance: *const c_void) -> i32;
// cv::LineIterator::setMinusStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
// ("cv::LineIterator::setMinusStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propMinusStep_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::plusStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
// ("cv::LineIterator::plusStep", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPlusStep_const(instance: *const c_void) -> i32;
// cv::LineIterator::setPlusStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
// ("cv::LineIterator::setPlusStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propPlusStep_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::minusShift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
// ("cv::LineIterator::minusShift", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propMinusShift_const(instance: *const c_void) -> i32;
// cv::LineIterator::setMinusShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
// ("cv::LineIterator::setMinusShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propMinusShift_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::plusShift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
// ("cv::LineIterator::plusShift", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPlusShift_const(instance: *const c_void) -> i32;
// cv::LineIterator::setPlusShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
// ("cv::LineIterator::setPlusShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_LineIterator_propPlusShift_const_int(instance: *mut c_void, val: i32);
// cv::LineIterator::p() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5037
// ("cv::LineIterator::p", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propP_const(instance: *const c_void, ocvrs_return: *mut core::Point);
// cv::LineIterator::setP(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5037
// ("cv::LineIterator::setP", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
pub fn cv_LineIterator_propP_const_Point(instance: *mut c_void, val: *const core::Point);
// cv::LineIterator::ptmode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5038
// ("cv::LineIterator::ptmode", vec![(pred!(const, [], []), _)]),
pub fn cv_LineIterator_propPtmode_const(instance: *const c_void) -> bool;
// cv::LineIterator::setPtmode(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5038
// ("cv::LineIterator::setPtmode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_LineIterator_propPtmode_const_bool(instance: *mut c_void, val: bool);
// cv::LineIterator::delete() generated
// ("cv::LineIterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineIterator_delete(instance: *mut c_void);
// detect(InputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1376
// ("cv::LineSegmentDetector::detect", vec![(pred!(mut, ["image", "lines", "width", "prec", "nfa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, width: *const c_void, prec: *const c_void, nfa: *const c_void, ocvrs_return: *mut Result<()>);
// cv::LineSegmentDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1376
// ("cv::LineSegmentDetector::detect", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// drawSegments(InputOutputArray, InputArray)(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1385
// ("cv::LineSegmentDetector::drawSegments", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// compareSegments(const Size &, InputArray, InputArray, InputOutputArray)(SimpleClass, InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1395
// ("cv::LineSegmentDetector::compareSegments", vec![(pred!(mut, ["size", "lines1", "lines2", "image"], ["const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, size: *const core::Size, lines1: *const c_void, lines2: *const c_void, image: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::LineSegmentDetector::compareSegments(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1395
// ("cv::LineSegmentDetector::compareSegments", vec![(pred!(mut, ["size", "lines1", "lines2"], ["const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, size: *const core::Size, lines1: *const c_void, lines2: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::LineSegmentDetector::to_Algorithm() generated
// ("cv::LineSegmentDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineSegmentDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::LineSegmentDetector::delete() generated
// ("cv::LineSegmentDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_LineSegmentDetector_delete(instance: *mut c_void);
// Subdiv2D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1101
// ("cv::Subdiv2D::Subdiv2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_Subdiv2D_Subdiv2D(ocvrs_return: *mut Result<*mut c_void>);
// Subdiv2D(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1111
// ("cv::Subdiv2D::Subdiv2D", vec![(pred!(mut, ["rect"], ["cv::Rect"]), _)]),
pub fn cv_Subdiv2D_Subdiv2D_Rect(rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// initDelaunay(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1118
// ("cv::Subdiv2D::initDelaunay", vec![(pred!(mut, ["rect"], ["cv::Rect"]), _)]),
pub fn cv_Subdiv2D_initDelaunay_Rect(instance: *mut c_void, rect: *const core::Rect, ocvrs_return: *mut Result<()>);
// insert(Point2f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1130
// ("cv::Subdiv2D::insert", vec![(pred!(mut, ["pt"], ["cv::Point2f"]), _)]),
pub fn cv_Subdiv2D_insert_Point2f(instance: *mut c_void, pt: *const core::Point2f, ocvrs_return: *mut Result<i32>);
// insert(const std::vector<Point2f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1139
// ("cv::Subdiv2D::insert", vec![(pred!(mut, ["ptvec"], ["const std::vector<cv::Point2f>*"]), _)]),
pub fn cv_Subdiv2D_insert_const_vectorLPoint2fGR(instance: *mut c_void, ptvec: *const c_void, ocvrs_return: *mut Result<()>);
// locate(Point2f, int &, int &)(SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1161
// ("cv::Subdiv2D::locate", vec![(pred!(mut, ["pt", "edge", "vertex"], ["cv::Point2f", "int*", "int*"]), _)]),
pub fn cv_Subdiv2D_locate_Point2f_intR_intR(instance: *mut c_void, pt: *const core::Point2f, edge: *mut i32, vertex: *mut i32, ocvrs_return: *mut Result<i32>);
// findNearest(Point2f, Point2f *)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1175
// ("cv::Subdiv2D::findNearest", vec![(pred!(mut, ["pt", "nearestPt"], ["cv::Point2f", "cv::Point2f*"]), _)]),
pub fn cv_Subdiv2D_findNearest_Point2f_Point2fX(instance: *mut c_void, pt: *const core::Point2f, nearest_pt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
// cv::Subdiv2D::findNearest(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1175
// ("cv::Subdiv2D::findNearest", vec![(pred!(mut, ["pt"], ["cv::Point2f"]), _)]),
pub fn cv_Subdiv2D_findNearest_Point2f(instance: *mut c_void, pt: *const core::Point2f, ocvrs_return: *mut Result<i32>);
// getEdgeList(std::vector<Vec4f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1184
// ("cv::Subdiv2D::getEdgeList", vec![(pred!(const, ["edgeList"], ["std::vector<cv::Vec4f>*"]), _)]),
pub fn cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(instance: *const c_void, edge_list: *mut c_void, ocvrs_return: *mut Result<()>);
// getLeadingEdgeList(std::vector<int> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1192
// ("cv::Subdiv2D::getLeadingEdgeList", vec![(pred!(const, ["leadingEdgeList"], ["std::vector<int>*"]), _)]),
pub fn cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(instance: *const c_void, leading_edge_list: *mut c_void, ocvrs_return: *mut Result<()>);
// getTriangleList(std::vector<Vec6f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1201
// ("cv::Subdiv2D::getTriangleList", vec![(pred!(const, ["triangleList"], ["std::vector<cv::Vec6f>*"]), _)]),
pub fn cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(instance: *const c_void, triangle_list: *mut c_void, ocvrs_return: *mut Result<()>);
// getVoronoiFacetList(const std::vector<int> &, std::vector<std::vector<Point2f>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1210
// ("cv::Subdiv2D::getVoronoiFacetList", vec![(pred!(mut, ["idx", "facetList", "facetCenters"], ["const std::vector<int>*", "std::vector<std::vector<cv::Point2f>>*", "std::vector<cv::Point2f>*"]), _)]),
pub fn cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(instance: *mut c_void, idx: *const c_void, facet_list: *mut c_void, facet_centers: *mut c_void, ocvrs_return: *mut Result<()>);
// getVertex(int, int *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1220
// ("cv::Subdiv2D::getVertex", vec![(pred!(const, ["vertex", "firstEdge"], ["int", "int*"]), _)]),
pub fn cv_Subdiv2D_getVertex_const_int_intX(instance: *const c_void, vertex: i32, first_edge: *mut i32, ocvrs_return: *mut Result<core::Point2f>);
// cv::Subdiv2D::getVertex(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1220
// ("cv::Subdiv2D::getVertex", vec![(pred!(const, ["vertex"], ["int"]), _)]),
pub fn cv_Subdiv2D_getVertex_const_int(instance: *const c_void, vertex: i32, ocvrs_return: *mut Result<core::Point2f>);
// getEdge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1240
// ("cv::Subdiv2D::getEdge", vec![(pred!(const, ["edge", "nextEdgeType"], ["int", "int"]), _)]),
pub fn cv_Subdiv2D_getEdge_const_int_int(instance: *const c_void, edge: i32, next_edge_type: i32, ocvrs_return: *mut Result<i32>);
// nextEdge(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1249
// ("cv::Subdiv2D::nextEdge", vec![(pred!(const, ["edge"], ["int"]), _)]),
pub fn cv_Subdiv2D_nextEdge_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
// rotateEdge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1263
// ("cv::Subdiv2D::rotateEdge", vec![(pred!(const, ["edge", "rotate"], ["int", "int"]), _)]),
pub fn cv_Subdiv2D_rotateEdge_const_int_int(instance: *const c_void, edge: i32, rotate: i32, ocvrs_return: *mut Result<i32>);
// symEdge(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1264
// ("cv::Subdiv2D::symEdge", vec![(pred!(const, ["edge"], ["int"]), _)]),
pub fn cv_Subdiv2D_symEdge_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
// edgeOrg(int, Point2f *)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1273
// ("cv::Subdiv2D::edgeOrg", vec![(pred!(const, ["edge", "orgpt"], ["int", "cv::Point2f*"]), _)]),
pub fn cv_Subdiv2D_edgeOrg_const_int_Point2fX(instance: *const c_void, edge: i32, orgpt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
// cv::Subdiv2D::edgeOrg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1273
// ("cv::Subdiv2D::edgeOrg", vec![(pred!(const, ["edge"], ["int"]), _)]),
pub fn cv_Subdiv2D_edgeOrg_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
// edgeDst(int, Point2f *)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1282
// ("cv::Subdiv2D::edgeDst", vec![(pred!(const, ["edge", "dstpt"], ["int", "cv::Point2f*"]), _)]),
pub fn cv_Subdiv2D_edgeDst_const_int_Point2fX(instance: *const c_void, edge: i32, dstpt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
// cv::Subdiv2D::edgeDst(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1282
// ("cv::Subdiv2D::edgeDst", vec![(pred!(const, ["edge"], ["int"]), _)]),
pub fn cv_Subdiv2D_edgeDst_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
// cv::Subdiv2D::delete() generated
// ("cv::Subdiv2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Subdiv2D_delete(instance: *mut c_void);
// IntelligentScissorsMB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:34
// ("cv::segmentation::IntelligentScissorsMB::IntelligentScissorsMB", vec![(pred!(mut, [], []), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(ocvrs_return: *mut Result<*mut c_void>);
// setWeights(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:46
// ("cv::segmentation::IntelligentScissorsMB::setWeights", vec![(pred!(mut, ["weight_non_edge", "weight_gradient_direction", "weight_gradient_magnitude"], ["float", "float", "float"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(instance: *mut c_void, weight_non_edge: f32, weight_gradient_direction: f32, weight_gradient_magnitude: f32, ocvrs_return: *mut Result<*mut c_void>);
// setGradientMagnitudeMaxLimit(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:58
// ("cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit", vec![(pred!(mut, ["gradient_magnitude_threshold_max"], ["float"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(instance: *mut c_void, gradient_magnitude_threshold_max: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:58
// ("cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit", vec![(pred!(mut, [], []), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setEdgeFeatureZeroCrossingParameters(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:74
// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters", vec![(pred!(mut, ["gradient_magnitude_min_value"], ["float"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(instance: *mut c_void, gradient_magnitude_min_value: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:74
// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setEdgeFeatureCannyParameters(double, double, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:83
// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters", vec![(pred!(mut, ["threshold1", "threshold2", "apertureSize", "L2gradient"], ["double", "double", "int", "bool"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(instance: *mut c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:83
// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters", vec![(pred!(mut, ["threshold1", "threshold2"], ["double", "double"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double(instance: *mut c_void, threshold1: f64, threshold2: f64, ocvrs_return: *mut Result<*mut c_void>);
// applyImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:93
// ("cv::segmentation::IntelligentScissorsMB::applyImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// applyImageFeatures(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:105
// ("cv::segmentation::IntelligentScissorsMB::applyImageFeatures", vec![(pred!(mut, ["non_edge", "gradient_direction", "gradient_magnitude", "image"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, non_edge: *const c_void, gradient_direction: *const c_void, gradient_magnitude: *const c_void, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::segmentation::IntelligentScissorsMB::applyImageFeatures(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:105
// ("cv::segmentation::IntelligentScissorsMB::applyImageFeatures", vec![(pred!(mut, ["non_edge", "gradient_direction", "gradient_magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, non_edge: *const c_void, gradient_direction: *const c_void, gradient_magnitude: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// buildMap(const Point &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:116
// ("cv::segmentation::IntelligentScissorsMB::buildMap", vec![(pred!(mut, ["sourcePt"], ["const cv::Point*"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(instance: *mut c_void, source_pt: *const core::Point, ocvrs_return: *mut Result<()>);
// getContour(const Point &, OutputArray, bool)(SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:126
// ("cv::segmentation::IntelligentScissorsMB::getContour", vec![(pred!(const, ["targetPt", "contour", "backward"], ["const cv::Point*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(instance: *const c_void, target_pt: *const core::Point, contour: *const c_void, backward: bool, ocvrs_return: *mut Result<()>);
// cv::segmentation::IntelligentScissorsMB::getContour(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:126
// ("cv::segmentation::IntelligentScissorsMB::getContour", vec![(pred!(const, ["targetPt", "contour"], ["const cv::Point*", "const cv::_OutputArray*"]), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR(instance: *const c_void, target_pt: *const core::Point, contour: *const c_void, ocvrs_return: *mut Result<()>);
// cv::segmentation::IntelligentScissorsMB::implicitClone() generated
// ("cv::segmentation::IntelligentScissorsMB::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::segmentation::IntelligentScissorsMB::delete() generated
// ("cv::segmentation::IntelligentScissorsMB::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_segmentation_IntelligentScissorsMB_delete(instance: *mut c_void);
