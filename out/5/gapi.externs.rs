// descr_of(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:281
// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
pub fn cv_descr_of_const_MatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// descr_of(const MediaFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:107
// ("cv::descr_of", vec![(pred!(mut, ["frame"], ["const cv::MediaFrame*"]), _)]),
pub fn cv_descr_of_const_MediaFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// descr_of(const RMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:278
// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::RMat*"]), _)]),
pub fn cv_descr_of_const_RMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// descr_of(const cv::Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:134
// ("cv::descr_of", vec![(pred!(mut, ["scalar"], ["const cv::Scalar*"]), _)]),
pub fn cv_descr_of_const_ScalarR(scalar: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// descr_of(const cv::UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:269
// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
pub fn cv_descr_of_const_UMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// empty_array_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garray.hpp:45
// ("cv::empty_array_desc", vec![(pred!(mut, [], []), _)]),
pub fn cv_empty_array_desc(ocvrs_return: *mut Result<*mut c_void>);
// empty_gopaque_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gopaque.hpp:46
// ("cv::empty_gopaque_desc", vec![(pred!(mut, [], []), _)]),
pub fn cv_empty_gopaque_desc(ocvrs_return: *mut Result<*mut c_void>);
// empty_scalar_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:132
// ("cv::empty_scalar_desc", vec![(pred!(mut, [], []), _)]),
pub fn cv_empty_scalar_desc(ocvrs_return: *mut Result<*mut c_void>);
// BGR2Gray(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1415
// ("cv::gapi::BGR2Gray", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BGR2Gray_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BGR2I420(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1448
// ("cv::gapi::BGR2I420", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BGR2I420_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BGR2LUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1510
// ("cv::gapi::BGR2LUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BGR2LUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BGR2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1375
// ("cv::gapi::BGR2RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BGR2RGB_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BGR2YUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1552
// ("cv::gapi::BGR2YUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BGR2YUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BayerGR2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1639
// ("cv::gapi::BayerGR2RGB", vec![(pred!(mut, ["src_gr"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_BayerGR2RGB_const_GMatR(src_gr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::Canny(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1026
// ("cv::gapi::Canny", vec![(pred!(mut, ["image", "threshold1", "threshold2"], ["const cv::GMat*", "double", "double"]), _)]),
pub fn cv_gapi_Canny_const_GMatR_double_double(image: *const c_void, threshold1: f64, threshold2: f64, ocvrs_return: *mut Result<*mut c_void>);
// Canny(const GMat &, double, double, int, bool)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1026
// ("cv::gapi::Canny", vec![(pred!(mut, ["image", "threshold1", "threshold2", "apertureSize", "L2gradient"], ["const cv::GMat*", "double", "double", "int", "bool"]), _)]),
pub fn cv_gapi_Canny_const_GMatR_double_double_int_bool(image: *const c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
// I4202BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1480
// ("cv::gapi::I4202BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_I4202BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// I4202RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1496
// ("cv::gapi::I4202RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_I4202RGB_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// LUT(const GMat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1699
// ("cv::gapi::LUT", vec![(pred!(mut, ["src", "lut"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
pub fn cv_gapi_LUT_const_GMatR_const_MatR(src: *const c_void, lut: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// LUV2BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1524
// ("cv::gapi::LUV2BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_LUV2BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::Laplacian(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:967
// ("cv::gapi::Laplacian", vec![(pred!(mut, ["src", "ddepth"], ["const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_Laplacian_const_GMatR_int(src: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// Laplacian(const GMat &, int, int, double, double, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:967
// ("cv::gapi::Laplacian", vec![(pred!(mut, ["src", "ddepth", "ksize", "scale", "delta", "borderType"], ["const cv::GMat*", "int", "int", "double", "double", "int"]), _)]),
pub fn cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(src: *const c_void, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// NV12toBGR(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1625
// ("cv::gapi::NV12toBGR", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_NV12toBGR_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// NV12toBGRp(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1703
// ("cv::gapi::NV12toBGRp", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// NV12toGray(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1610
// ("cv::gapi::NV12toGray", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_NV12toGray_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// NV12toRGB(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1595
// ("cv::gapi::NV12toRGB", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_NV12toRGB_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// NV12toRGBp(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1685
// ("cv::gapi::NV12toRGBp", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2Gray(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1388
// ("cv::gapi::RGB2Gray", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2Gray_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2Gray(const GMat &, float, float, float)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1402
// ("cv::gapi::RGB2Gray", vec![(pred!(mut, ["src", "rY", "gY", "bY"], ["const cv::GMat*", "float", "float", "float"]), _)]),
pub fn cv_gapi_RGB2Gray_const_GMatR_float_float_float(src: *const c_void, r_y: f32, g_y: f32, b_y: f32, ocvrs_return: *mut Result<*mut c_void>);
// RGB2HSV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1653
// ("cv::gapi::RGB2HSV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2HSV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2I420(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1464
// ("cv::gapi::RGB2I420", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2I420_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2Lab(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1566
// ("cv::gapi::RGB2Lab", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2Lab_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2YUV422(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1667
// ("cv::gapi::RGB2YUV422", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2YUV422_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// RGB2YUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1432
// ("cv::gapi::RGB2YUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_RGB2YUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::SobelXY(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:937
// ("cv::gapi::SobelXY", vec![(pred!(mut, ["src", "ddepth", "order"], ["const cv::GMat*", "int", "int"]), _)]),
pub fn cv_gapi_SobelXY_const_GMatR_int_int(src: *const c_void, ddepth: i32, order: i32, ocvrs_return: *mut Result<*mut c_void>);
// SobelXY(const GMat &, int, int, int, double, double, int, const Scalar &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:937
// ("cv::gapi::SobelXY", vec![(pred!(mut, ["src", "ddepth", "order", "ksize", "scale", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "int", "double", "double", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_SobelXY_const_GMatR_int_int_int_double_double_int_const_ScalarR(src: *const c_void, ddepth: i32, order: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::Sobel(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:886
// ("cv::gapi::Sobel", vec![(pred!(mut, ["src", "ddepth", "dx", "dy"], ["const cv::GMat*", "int", "int", "int"]), _)]),
pub fn cv_gapi_Sobel_const_GMatR_int_int_int(src: *const c_void, ddepth: i32, dx: i32, dy: i32, ocvrs_return: *mut Result<*mut c_void>);
// Sobel(const GMat &, int, int, int, int, double, double, int, const Scalar &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:886
// ("cv::gapi::Sobel", vec![(pred!(mut, ["src", "ddepth", "dx", "dy", "ksize", "scale", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "int", "int", "double", "double", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(src: *const c_void, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// YUV2BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1538
// ("cv::gapi::YUV2BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_YUV2BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// YUV2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1580
// ("cv::gapi::YUV2RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_YUV2RGB_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// absDiffC(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1254
// ("cv::gapi::absDiffC", vec![(pred!(mut, ["src", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_absDiffC_const_GMatR_const_GScalarR(src: *const c_void, c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// absDiff(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1237
// ("cv::gapi::absDiff", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_absDiff_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::addC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:636
// ("cv::gapi::addC", vec![(pred!(mut, ["src1", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_addC_const_GMatR_const_GScalarR(src1: *const c_void, c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// addC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:636
// ("cv::gapi::addC", vec![(pred!(mut, ["src1", "c", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
pub fn cv_gapi_addC_const_GMatR_const_GScalarR_int(src1: *const c_void, c: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::addC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:638
// ("cv::gapi::addC", vec![(pred!(mut, ["c", "src1"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_addC_const_GScalarR_const_GMatR(c: *const c_void, src1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// addC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:638
// ("cv::gapi::addC", vec![(pred!(mut, ["c", "src1", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_addC_const_GScalarR_const_GMatR_int(c: *const c_void, src1: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::addWeighted(TraitClass, Primitive, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1302
// ("cv::gapi::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma"], ["const cv::GMat*", "double", "const cv::GMat*", "double", "double"]), _)]),
pub fn cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, ocvrs_return: *mut Result<*mut c_void>);
// addWeighted(const GMat &, double, const GMat &, double, double, int)(TraitClass, Primitive, TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1302
// ("cv::gapi::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "ddepth"], ["const cv::GMat*", "double", "const cv::GMat*", "double", "double", "int"]), _)]),
pub fn cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::add(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:615
// ("cv::gapi::add", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_add_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// add(const GMat &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:615
// ("cv::gapi::add", vec![(pred!(mut, ["src1", "src2", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_add_const_GMatR_const_GMatR_int(src1: *const c_void, src2: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::bilateralFilter(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1001
// ("cv::gapi::bilateralFilter", vec![(pred!(mut, ["src", "d", "sigmaColor", "sigmaSpace"], ["const cv::GMat*", "int", "double", "double"]), _)]),
pub fn cv_gapi_bilateralFilter_const_GMatR_int_double_double(src: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, ocvrs_return: *mut Result<*mut c_void>);
// bilateralFilter(const GMat &, int, double, double, int)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1001
// ("cv::gapi::bilateralFilter", vec![(pred!(mut, ["src", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::GMat*", "int", "double", "double", "int"]), _)]),
pub fn cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(src: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_and(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1089
// ("cv::gapi::bitwise_and", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_bitwise_and_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_and(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1095
// ("cv::gapi::bitwise_and", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_bitwise_and_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_not(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1165
// ("cv::gapi::bitwise_not", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_bitwise_not_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_or(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1113
// ("cv::gapi::bitwise_or", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_bitwise_or_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_or(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1119
// ("cv::gapi::bitwise_or", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_bitwise_or_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_xor(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1138
// ("cv::gapi::bitwise_xor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_bitwise_xor_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// bitwise_xor(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1144
// ("cv::gapi::bitwise_xor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::blur(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:657
// ("cv::gapi::blur", vec![(pred!(mut, ["src", "ksize"], ["const cv::GMat*", "const cv::Size*"]), _)]),
pub fn cv_gapi_blur_const_GMatR_const_SizeR(src: *const c_void, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// blur(const GMat &, const Size &, const Point &, int, const Scalar &)(TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:657
// ("cv::gapi::blur", vec![(pred!(mut, ["src", "ksize", "anchor", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Size*", "const cv::Point*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(src: *const c_void, ksize: *const core::Size, anchor: *const core::Point, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::boxFilter(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:630
// ("cv::gapi::boxFilter", vec![(pred!(mut, ["src", "dtype", "ksize"], ["const cv::GMat*", "int", "const cv::Size*"]), _)]),
pub fn cv_gapi_boxFilter_const_GMatR_int_const_SizeR(src: *const c_void, dtype: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// boxFilter(const GMat &, int, const Size &, const Point &, bool, int, const Scalar &)(TraitClass, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:630
// ("cv::gapi::boxFilter", vec![(pred!(mut, ["src", "dtype", "ksize", "anchor", "normalize", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Size*", "const cv::Point*", "bool", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(src: *const c_void, dtype: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::cartToPolar(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:881
// ("cv::gapi::cartToPolar", vec![(pred!(mut, ["x", "y"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cartToPolar_const_GMatR_const_GMatR(x: *const c_void, y: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cartToPolar(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:881
// ("cv::gapi::cartToPolar", vec![(pred!(mut, ["x", "y", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
pub fn cv_gapi_cartToPolar_const_GMatR_const_GMatR_bool(x: *const c_void, y: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<*mut c_void>);
// cmpEQ(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1041
// ("cv::gapi::cmpEQ", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpEQ_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpEQ(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1045
// ("cv::gapi::cmpEQ", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpEQ_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpGE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:989
// ("cv::gapi::cmpGE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpGE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpGE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:993
// ("cv::gapi::cmpGE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpGE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpGT(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:937
// ("cv::gapi::cmpGT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpGT_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpGT(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:941
// ("cv::gapi::cmpGT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpGT_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpLE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1015
// ("cv::gapi::cmpLE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpLE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpLE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1019
// ("cv::gapi::cmpLE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpLE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpLT(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:963
// ("cv::gapi::cmpLT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpLT_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpLT(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:967
// ("cv::gapi::cmpLT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpLT_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpNE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1067
// ("cv::gapi::cmpNE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_cmpNE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cmpNE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1071
// ("cv::gapi::cmpNE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_cmpNE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// combine(const cv::GKernelPackage &, const cv::GKernelPackage &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:420
// ("cv::gapi::combine", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GKernelPackage*", "const cv::GKernelPackage*"]), _)]),
pub fn cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// concatHor(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1632
// ("cv::gapi::concatHor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_concatHor_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// concatHor(const std::vector<GMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1640
// ("cv::gapi::concatHor", vec![(pred!(mut, ["v"], ["const std::vector<cv::GMat>*"]), _)]),
pub fn cv_gapi_concatHor_const_vectorLGMatGR(v: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// concatVert(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1672
// ("cv::gapi::concatVert", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_concatVert_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// concatVert(const std::vector<GMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1680
// ("cv::gapi::concatVert", vec![(pred!(mut, ["v"], ["const std::vector<cv::GMat>*"]), _)]),
pub fn cv_gapi_concatVert_const_vectorLGMatGR(v: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1716
// ("cv::gapi::convertTo", vec![(pred!(mut, ["src", "rdepth"], ["const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_convertTo_const_GMatR_int(src: *const c_void, rdepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// convertTo(const GMat &, int, double, double)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1716
// ("cv::gapi::convertTo", vec![(pred!(mut, ["src", "rdepth", "alpha", "beta"], ["const cv::GMat*", "int", "double", "double"]), _)]),
pub fn cv_gapi_convertTo_const_GMatR_int_double_double(src: *const c_void, rdepth: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<*mut c_void>);
// copy(const GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:88
// ("cv::gapi::copy", vec![(pred!(mut, ["in"], ["const cv::GFrame*"]), _)]),
pub fn cv_gapi_copy_const_GFrameR(in_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// copy(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:77
// ("cv::gapi::copy", vec![(pred!(mut, ["in"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_copy_const_GMatR(in_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// crop(const GMat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1604
// ("cv::gapi::crop", vec![(pred!(mut, ["src", "rect"], ["const cv::GMat*", "const cv::Rect*"]), _)]),
pub fn cv_gapi_crop_const_GMatR_const_RectR(src: *const c_void, rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::dilate3x3(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:804
// ("cv::gapi::dilate3x3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_dilate3x3_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dilate3x3(const GMat &, int, int, const Scalar &)(TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:804
// ("cv::gapi::dilate3x3", vec![(pred!(mut, ["src", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(src: *const c_void, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::dilate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:780
// ("cv::gapi::dilate", vec![(pred!(mut, ["src", "kernel"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
pub fn cv_gapi_dilate_const_GMatR_const_MatR(src: *const c_void, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// dilate(const GMat &, const Mat &, const Point &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:780
// ("cv::gapi::dilate", vec![(pred!(mut, ["src", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Point*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::divC(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:788
// ("cv::gapi::divC", vec![(pred!(mut, ["src", "divisor", "scale"], ["const cv::GMat*", "const cv::GScalar*", "double"]), _)]),
pub fn cv_gapi_divC_const_GMatR_const_GScalarR_double(src: *const c_void, divisor: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// divC(const GMat &, const GScalar &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:788
// ("cv::gapi::divC", vec![(pred!(mut, ["src", "divisor", "scale", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "double", "int"]), _)]),
pub fn cv_gapi_divC_const_GMatR_const_GScalarR_double_int(src: *const c_void, divisor: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::divRC(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:809
// ("cv::gapi::divRC", vec![(pred!(mut, ["divident", "src", "scale"], ["const cv::GScalar*", "const cv::GMat*", "double"]), _)]),
pub fn cv_gapi_divRC_const_GScalarR_const_GMatR_double(divident: *const c_void, src: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// divRC(const GScalar &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:809
// ("cv::gapi::divRC", vec![(pred!(mut, ["divident", "src", "scale", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "double", "int"]), _)]),
pub fn cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(divident: *const c_void, src: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::div(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:767
// ("cv::gapi::div", vec![(pred!(mut, ["src1", "src2", "scale"], ["const cv::GMat*", "const cv::GMat*", "double"]), _)]),
pub fn cv_gapi_div_const_GMatR_const_GMatR_double(src1: *const c_void, src2: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// div(const GMat &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:767
// ("cv::gapi::div", vec![(pred!(mut, ["src1", "src2", "scale", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "double", "int"]), _)]),
pub fn cv_gapi_div_const_GMatR_const_GMatR_double_int(src1: *const c_void, src2: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// equalizeHist(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1101
// ("cv::gapi::equalizeHist", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_equalizeHist_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::erode3x3(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:753
// ("cv::gapi::erode3x3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_erode3x3_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// erode3x3(const GMat &, int, int, const Scalar &)(TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:753
// ("cv::gapi::erode3x3", vec![(pred!(mut, ["src", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(src: *const c_void, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::erode(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:733
// ("cv::gapi::erode", vec![(pred!(mut, ["src", "kernel"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
pub fn cv_gapi_erode_const_GMatR_const_MatR(src: *const c_void, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// erode(const GMat &, const Mat &, const Point &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:733
// ("cv::gapi::erode", vec![(pred!(mut, ["src", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Point*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::filter2D(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:596
// ("cv::gapi::filter2D", vec![(pred!(mut, ["src", "ddepth", "kernel"], ["const cv::GMat*", "int", "const cv::Mat*"]), _)]),
pub fn cv_gapi_filter2D_const_GMatR_int_const_MatR(src: *const c_void, ddepth: i32, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// filter2D(const GMat &, int, const Mat &, const Point &, const Scalar &, int, const Scalar &)(TraitClass, Primitive, TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:596
// ("cv::gapi::filter2D", vec![(pred!(mut, ["src", "ddepth", "kernel", "anchor", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src: *const c_void, ddepth: i32, kernel: *const c_void, anchor: *const core::Point, delta: *const core::Scalar, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// flip(const GMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1590
// ("cv::gapi::flip", vec![(pred!(mut, ["src", "flipCode"], ["const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_flip_const_GMatR_int(src: *const c_void, flip_code: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::gaussianBlur(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:690
// ("cv::gapi::gaussianBlur", vec![(pred!(mut, ["src", "ksize", "sigmaX"], ["const cv::GMat*", "const cv::Size*", "double"]), _)]),
pub fn cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double(src: *const c_void, ksize: *const core::Size, sigma_x: f64, ocvrs_return: *mut Result<*mut c_void>);
// gaussianBlur(const GMat &, const Size &, double, double, int, const Scalar &)(TraitClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:690
// ("cv::gapi::gaussianBlur", vec![(pred!(mut, ["src", "ksize", "sigmaX", "sigmaY", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Size*", "double", "double", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(src: *const c_void, ksize: *const core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// inRange(const GMat &, const GScalar &, const GScalar &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1441
// ("cv::gapi::inRange", vec![(pred!(mut, ["src", "threshLow", "threshUp"], ["const cv::GMat*", "const cv::GScalar*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(src: *const c_void, thresh_low: *const c_void, thresh_up: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::integral(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1390
// ("cv::gapi::integral", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_integral_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// integral(const GMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1390
// ("cv::gapi::integral", vec![(pred!(mut, ["src", "sdepth", "sqdepth"], ["const cv::GMat*", "int", "int"]), _)]),
pub fn cv_gapi_integral_const_GMatR_int_int(src: *const c_void, sdepth: i32, sqdepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// mask(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:822
// ("cv::gapi::mask", vec![(pred!(mut, ["src", "mask"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_mask_const_GMatR_const_GMatR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// max(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1220
// ("cv::gapi::max", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_max_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mean(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:835
// ("cv::gapi::mean", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_mean_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// medianBlur(const GMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:707
// ("cv::gapi::medianBlur", vec![(pred!(mut, ["src", "ksize"], ["const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_medianBlur_const_GMatR_int(src: *const c_void, ksize: i32, ocvrs_return: *mut Result<*mut c_void>);
// merge3(const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1484
// ("cv::gapi::merge3", vec![(pred!(mut, ["src1", "src2", "src3"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, src3: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// merge4(const GMat &, const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1465
// ("cv::gapi::merge4", vec![(pred!(mut, ["src1", "src2", "src3", "src4"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// min(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1203
// ("cv::gapi::min", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_min_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::morphologyEx(TraitClass, Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:834
// ("cv::gapi::morphologyEx", vec![(pred!(mut, ["src", "op", "kernel"], ["const cv::GMat*", "const cv::MorphTypes", "const cv::Mat*"]), _)]),
pub fn cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR(src: *const c_void, op: crate::imgproc::MorphTypes, kernel: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// morphologyEx(const GMat &, const MorphTypes, const Mat &, const Point &, const int, const BorderTypes, const Scalar &)(TraitClass, Enum, TraitClass, SimpleClass, Primitive, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:834
// ("cv::gapi::morphologyEx", vec![(pred!(mut, ["src", "op", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::MorphTypes", "const cv::Mat*", "const cv::Point*", "const int", "const cv::BorderTypes", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(src: *const c_void, op: crate::imgproc::MorphTypes, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: core::BorderTypes, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::mulC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:742
// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_mulC_const_GMatR_const_GScalarR(src: *const c_void, multiplier: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mulC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:742
// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
pub fn cv_gapi_mulC_const_GMatR_const_GScalarR_int(src: *const c_void, multiplier: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::mulC(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:740
// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier"], ["const cv::GMat*", "double"]), _)]),
pub fn cv_gapi_mulC_const_GMatR_double(src: *const c_void, multiplier: f64, ocvrs_return: *mut Result<*mut c_void>);
// mulC(const GMat &, double, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:740
// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier", "ddepth"], ["const cv::GMat*", "double", "int"]), _)]),
pub fn cv_gapi_mulC_const_GMatR_double_int(src: *const c_void, multiplier: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::mulC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:744
// ("cv::gapi::mulC", vec![(pred!(mut, ["multiplier", "src"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_mulC_const_GScalarR_const_GMatR(multiplier: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mulC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:744
// ("cv::gapi::mulC", vec![(pred!(mut, ["multiplier", "src", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_mulC_const_GScalarR_const_GMatR_int(multiplier: *const c_void, src: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::mul(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:722
// ("cv::gapi::mul", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_mul_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// mul(const GMat &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:722
// ("cv::gapi::mul", vec![(pred!(mut, ["src1", "src2", "scale", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "double", "int"]), _)]),
pub fn cv_gapi_mul_const_GMatR_const_GMatR_double_int(src1: *const c_void, src2: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// normInf(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1370
// ("cv::gapi::normInf", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_normInf_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// normL1(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1325
// ("cv::gapi::normL1", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_normL1_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// normL2(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1347
// ("cv::gapi::normL2", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_normL2_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::normalize(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1738
// ("cv::gapi::normalize", vec![(pred!(mut, ["src", "alpha", "beta", "norm_type"], ["const cv::GMat*", "double", "double", "int"]), _)]),
pub fn cv_gapi_normalize_const_GMatR_double_double_int(src: *const c_void, alpha: f64, beta: f64, norm_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// normalize(const GMat &, double, double, int, int)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1738
// ("cv::gapi::normalize", vec![(pred!(mut, ["src", "alpha", "beta", "norm_type", "ddepth"], ["const cv::GMat*", "double", "double", "int", "int"]), _)]),
pub fn cv_gapi_normalize_const_GMatR_double_double_int_int(src: *const c_void, alpha: f64, beta: f64, norm_type: i32, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::phase(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:899
// ("cv::gapi::phase", vec![(pred!(mut, ["x", "y"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_phase_const_GMatR_const_GMatR(x: *const c_void, y: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// phase(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:899
// ("cv::gapi::phase", vec![(pred!(mut, ["x", "y", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
pub fn cv_gapi_phase_const_GMatR_const_GMatR_bool(x: *const c_void, y: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::polarToCart(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:857
// ("cv::gapi::polarToCart", vec![(pred!(mut, ["magnitude", "angle"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_polarToCart_const_GMatR_const_GMatR(magnitude: *const c_void, angle: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// polarToCart(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:857
// ("cv::gapi::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
pub fn cv_gapi_polarToCart_const_GMatR_const_GMatR_bool(magnitude: *const c_void, angle: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::remap(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1551
// ("cv::gapi::remap", vec![(pred!(mut, ["src", "map1", "map2", "interpolation"], ["const cv::GMat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int(src: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, ocvrs_return: *mut Result<*mut c_void>);
// remap(const GMat &, const Mat &, const Mat &, int, int, const Scalar &)(TraitClass, TraitClass, TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1551
// ("cv::gapi::remap", vec![(pred!(mut, ["src", "map1", "map2", "interpolation", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Mat*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(src: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::resizeP(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1763
// ("cv::gapi::resizeP", vec![(pred!(mut, ["src", "dsize"], ["const cv::GMatP*", "const cv::Size*"]), _)]),
pub fn cv_gapi_resizeP_const_GMatPR_const_SizeR(src: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// resizeP(const GMatP &, const Size &, int)(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1763
// ("cv::gapi::resizeP", vec![(pred!(mut, ["src", "dsize", "interpolation"], ["const cv::GMatP*", "const cv::Size*", "int"]), _)]),
pub fn cv_gapi_resizeP_const_GMatPR_const_SizeR_int(src: *const c_void, dsize: *const core::Size, interpolation: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::resize(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1745
// ("cv::gapi::resize", vec![(pred!(mut, ["src", "dsize"], ["const cv::GMat*", "const cv::Size*"]), _)]),
pub fn cv_gapi_resize_const_GMatR_const_SizeR(src: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// resize(const GMat &, const Size &, double, double, int)(TraitClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1745
// ("cv::gapi::resize", vec![(pred!(mut, ["src", "dsize", "fx", "fy", "interpolation"], ["const cv::GMat*", "const cv::Size*", "double", "double", "int"]), _)]),
pub fn cv_gapi_resize_const_GMatR_const_SizeR_double_double_int(src: *const c_void, dsize: *const core::Size, fx: f64, fy: f64, interpolation: i32, ocvrs_return: *mut Result<*mut c_void>);
// select(const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1181
// ("cv::gapi::select", vec![(pred!(mut, ["src1", "src2", "mask"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::sepFilter(TraitClass, Primitive, TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:559
// ("cv::gapi::sepFilter", vec![(pred!(mut, ["src", "ddepth", "kernelX", "kernelY", "anchor", "delta"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR(src: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, anchor: *const core::Point, delta: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// sepFilter(const GMat &, int, const Mat &, const Mat &, const Point &, const Scalar &, int, const Scalar &)(TraitClass, Primitive, TraitClass, TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:559
// ("cv::gapi::sepFilter", vec![(pred!(mut, ["src", "ddepth", "kernelX", "kernelY", "anchor", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, anchor: *const core::Point, delta: *const core::Scalar, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// split3(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1518
// ("cv::gapi::split3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_split3_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// split4(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1501
// ("cv::gapi::split4", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_split4_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// sqrt(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:910
// ("cv::gapi::sqrt", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_sqrt_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// BGR(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:43
// ("cv::gapi::streaming::BGR", vec![(pred!(mut, ["in"], ["const cv::GFrame*"]), _)]),
pub fn cv_gapi_streaming_BGR_const_GFrameR(in_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// UV(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:63
// ("cv::gapi::streaming::UV", vec![(pred!(mut, ["frame"], ["const cv::GFrame*"]), _)]),
pub fn cv_gapi_streaming_UV_const_GFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Y(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:53
// ("cv::gapi::streaming::Y", vec![(pred!(mut, ["frame"], ["const cv::GFrame*"]), _)]),
pub fn cv_gapi_streaming_Y_const_GFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// desync(const GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/desync.hpp:80
// ("cv::gapi::streaming::desync", vec![(pred!(mut, ["f"], ["const cv::GFrame*"]), _)]),
pub fn cv_gapi_streaming_desync_const_GFrameR(f: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// desync(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/desync.hpp:79
// ("cv::gapi::streaming::desync", vec![(pred!(mut, ["g"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_streaming_desync_const_GMatR(g: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// kernels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:16
// ("cv::gapi::streaming::kernels", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_streaming_kernels(ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::subC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:682
// ("cv::gapi::subC", vec![(pred!(mut, ["src", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_gapi_subC_const_GMatR_const_GScalarR(src: *const c_void, c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// subC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:682
// ("cv::gapi::subC", vec![(pred!(mut, ["src", "c", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
pub fn cv_gapi_subC_const_GMatR_const_GScalarR_int(src: *const c_void, c: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::subRC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:701
// ("cv::gapi::subRC", vec![(pred!(mut, ["c", "src"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_subRC_const_GScalarR_const_GMatR(c: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// subRC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:701
// ("cv::gapi::subRC", vec![(pred!(mut, ["c", "src", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_subRC_const_GScalarR_const_GMatR_int(c: *const c_void, src: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::sub(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:663
// ("cv::gapi::sub", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_gapi_sub_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// sub(const GMat &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:663
// ("cv::gapi::sub", vec![(pred!(mut, ["src1", "src2", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "int"]), _)]),
pub fn cv_gapi_sub_const_GMatR_const_GMatR_int(src1: *const c_void, src2: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// sum(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1266
// ("cv::gapi::sum", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_sum_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// threshold(const GMat &, const GScalar &, const GScalar &, int)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1419
// ("cv::gapi::threshold", vec![(pred!(mut, ["src", "thresh", "maxval", "type"], ["const cv::GMat*", "const cv::GScalar*", "const cv::GScalar*", "int"]), _)]),
pub fn cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(src: *const c_void, thresh: *const c_void, maxval: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// threshold(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1424
// ("cv::gapi::threshold", vec![(pred!(mut, ["src", "maxval", "type"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
pub fn cv_gapi_threshold_const_GMatR_const_GScalarR_int(src: *const c_void, maxval: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// transpose(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1876
// ("cv::gapi::transpose", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
pub fn cv_gapi_transpose_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::warpAffine(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1787
// ("cv::gapi::warpAffine", vec![(pred!(mut, ["src", "M", "dsize"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*"]), _)]),
pub fn cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR(src: *const c_void, m: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// warpAffine(const GMat &, const Mat &, const Size &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1787
// ("cv::gapi::warpAffine", vec![(pred!(mut, ["src", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::warpPerspective(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1762
// ("cv::gapi::warpPerspective", vec![(pred!(mut, ["src", "M", "dsize"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*"]), _)]),
pub fn cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR(src: *const c_void, m: *const c_void, dsize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// warpPerspective(const GMat &, const Mat &, const Size &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1762
// ("cv::gapi::warpPerspective", vec![(pred!(mut, ["src", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:16
// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorA_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:18
// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorA_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator+(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:19
// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorA_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:33
// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorD_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:31
// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorD_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator/(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:32
// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorD_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:52
// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorEQ_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:59
// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorEQ_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:66
// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorEQ_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:49
// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorGE_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:56
// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorGE_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:63
// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorGE_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:48
// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorG_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:55
// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorG_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator>(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:62
// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorG_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:51
// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorLE_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:58
// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorLE_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:65
// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorLE_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:50
// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorL_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:57
// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorL_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:64
// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorL_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:53
// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorNE_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:60
// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorNE_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator!=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:67
// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorNE_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator~(const cv::GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:38
// ("cv::operator~", vec![(pred!(mut, ["lhs"], ["const cv::GMat*"]), _)]),
pub fn cv_operatorNOTB_const_GMatR(lhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:36
// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorOR_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:45
// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorOR_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator|(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:41
// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorOR_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:35
// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorR_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:44
// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorR_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator&(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:40
// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorR_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:21
// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorS_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:23
// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorS_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator-(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:24
// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorS_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:37
// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
pub fn cv_operatorXOR_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:46
// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorXOR_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator^(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:42
// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorXOR_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:28
// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
pub fn cv_operatorX_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const cv::GMat &, float)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:26
// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "float"]), _)]),
pub fn cv_operatorX_const_GMatR_float(lhs: *const c_void, rhs: f32, ocvrs_return: *mut Result<*mut c_void>);
// operator*(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:29
// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
pub fn cv_operatorX_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator*(float, const cv::GMat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:27
// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["float", "const cv::GMat*"]), _)]),
pub fn cv_operatorX_float_const_GMatR(lhs: f32, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// validate_input_arg(const GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gproto.hpp:154
// ("cv::validate_input_arg", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
pub fn cv_validate_input_arg_const_GRunArgR(arg: *const c_void, ocvrs_return: *mut Result<()>);
// validate_input_args(const GRunArgs &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gproto.hpp:155
// ("cv::validate_input_args", vec![(pred!(mut, ["args"], ["const cv::GRunArgs*"]), _)]),
pub fn cv_validate_input_args_const_GRunArgsR(args: *const c_void, ocvrs_return: *mut Result<()>);
// GArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:49
// ("cv::GArg::GArg", vec![(pred!(mut, [], []), _)]),
pub fn cv_GArg_GArg(ocvrs_return: *mut Result<*mut c_void>);
// cv::GArg::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:87
// ("cv::GArg::kind", vec![(pred!(const, [], []), _)]),
pub fn cv_GArg_propKind_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::Detail_ArgKind);
// cv::GArg::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:87
// ("cv::GArg::setKind", vec![(pred!(mut, ["val"], ["const cv::detail::ArgKind"]), _)]),
pub fn cv_GArg_propKind_const_ArgKind(instance: *mut c_void, val: crate::gapi::Detail_ArgKind);
// cv::GArg::opaque_kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:88
// ("cv::GArg::opaque_kind", vec![(pred!(const, [], []), _)]),
pub fn cv_GArg_propOpaque_kind_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::Detail_OpaqueKind);
// cv::GArg::setOpaque_kind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:88
// ("cv::GArg::setOpaque_kind", vec![(pred!(mut, ["val"], ["const cv::detail::OpaqueKind"]), _)]),
pub fn cv_GArg_propOpaque_kind_const_OpaqueKind(instance: *mut c_void, val: crate::gapi::Detail_OpaqueKind);
// cv::GArg::delete() generated
// ("cv::GArg::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GArg_delete(instance: *mut c_void);
// operator==(const GArrayDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garray.hpp:42
// ("cv::GArrayDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GArrayDesc*"]), _)]),
pub fn cv_GArrayDesc_operatorEQ_const_const_GArrayDescR(instance: *const c_void, unnamed: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GArrayDesc::implicitClone() generated
// ("cv::GArrayDesc::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GArrayDesc_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GArrayDesc::defaultNew() generated
// ("cv::GArrayDesc::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GArrayDesc_defaultNew_const() -> *mut c_void;
// cv::GArrayDesc::delete() generated
// ("cv::GArrayDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GArrayDesc_delete(instance: *mut c_void);
// GCall(const GKernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:31
// ("cv::GCall::GCall", vec![(pred!(mut, ["k"], ["const cv::GKernel*"]), _)]),
pub fn cv_GCall_GCall_const_GKernelR(k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// yield(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:42
// ("cv::GCall::yield", vec![(pred!(mut, ["output"], ["int"]), _)]),
pub fn cv_GCall_yield_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::GCall::yield() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:42
// ("cv::GCall::yield", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_yield(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// yieldP(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:43
// ("cv::GCall::yieldP", vec![(pred!(mut, ["output"], ["int"]), _)]),
pub fn cv_GCall_yieldP_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::GCall::yieldP() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:43
// ("cv::GCall::yieldP", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_yieldP(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// yieldScalar(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:44
// ("cv::GCall::yieldScalar", vec![(pred!(mut, ["output"], ["int"]), _)]),
pub fn cv_GCall_yieldScalar_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::GCall::yieldScalar() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:44
// ("cv::GCall::yieldScalar", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_yieldScalar(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// yieldFrame(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:45
// ("cv::GCall::yieldFrame", vec![(pred!(mut, ["output"], ["int"]), _)]),
pub fn cv_GCall_yieldFrame_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::GCall::yieldFrame() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:45
// ("cv::GCall::yieldFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_yieldFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// kernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:63
// ("cv::GCall::kernel", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_kernel(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:64
// ("cv::GCall::params", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_params(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setArgs(std::vector<GArg> &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:66
// ("cv::GCall::setArgs", vec![(pred!(mut, ["args"], ["std::vector<cv::GArg>*"]), _)]),
pub fn cv_GCall_setArgs_vectorLGArgGRR(instance: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GCall::delete() generated
// ("cv::GCall::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCall_delete(instance: *mut c_void);
// GCompileArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:162
// ("cv::GCompileArg::GCompileArg", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCompileArg_GCompileArg() -> *mut c_void;
// cv::GCompileArg::tag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:164
// ("cv::GCompileArg::tag", vec![(pred!(const, [], []), _)]),
pub fn cv_GCompileArg_propTag_const(instance: *const c_void) -> *mut c_void;
// cv::GCompileArg::setTag(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:164
// ("cv::GCompileArg::setTag", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_GCompileArg_propTag_const_string(instance: *mut c_void, val: *const c_char);
// cv::GCompileArg::delete() generated
// ("cv::GCompileArg::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCompileArg_delete(instance: *mut c_void);
// GCompiled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:75
// ("cv::GCompiled::GCompiled", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCompiled_GCompiled(ocvrs_return: *mut Result<*mut c_void>);
// operator()(cv::Mat, cv::Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:107
// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Mat*"]), _)]),
pub fn cv_GCompiled_operator___Mat_MatR(instance: *mut c_void, in_: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(cv::Mat, cv::Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:117
// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Scalar*"]), _)]),
pub fn cv_GCompiled_operator___Mat_ScalarR(instance: *mut c_void, in_: *mut c_void, out: *mut core::Scalar, ocvrs_return: *mut Result<()>);
// operator()(cv::Mat, cv::Mat, cv::Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:128
// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Mat*"]), _)]),
pub fn cv_GCompiled_operator___Mat_Mat_MatR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<()>);
// operator()(cv::Mat, cv::Mat, cv::Scalar &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:139
// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Scalar*"]), _)]),
pub fn cv_GCompiled_operator___Mat_Mat_ScalarR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut core::Scalar, ocvrs_return: *mut Result<()>);
// operator()(const std::vector<cv::Mat> &, const std::vector<cv::Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:154
// ("cv::GCompiled::operator()", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_GCompiled_operator___const_vectorLMatGR_const_vectorLMatGR(instance: *mut c_void, ins: *const c_void, outs: *const c_void, ocvrs_return: *mut Result<()>);
// operator bool()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:165
// ("cv::GCompiled::operator bool", vec![(pred!(const, [], []), _)]),
pub fn cv_GCompiled_operator_bool_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// canReshape()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:196
// ("cv::GCompiled::canReshape", vec![(pred!(const, [], []), _)]),
pub fn cv_GCompiled_canReshape_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// prepareForNewStream()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:222
// ("cv::GCompiled::prepareForNewStream", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCompiled_prepareForNewStream(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GCompiled::delete() generated
// ("cv::GCompiled::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GCompiled_delete(instance: *mut c_void);
// GComputation(GMat, GMat)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:174
// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in", "out"], ["cv::GMat", "cv::GMat"]), _)]),
pub fn cv_GComputation_GComputation_GMat_GMat(in_: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GComputation(GMat, GScalar)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:183
// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in", "out"], ["cv::GMat", "cv::GScalar"]), _)]),
pub fn cv_GComputation_GComputation_GMat_GScalar(in_: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GComputation(GMat, GMat, GMat)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:193
// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::GMat", "cv::GMat", "cv::GMat"]), _)]),
pub fn cv_GComputation_GComputation_GMat_GMat_GMat(in1: *mut c_void, in2: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GComputation(GMat, GMat, GScalar)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:203
// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::GMat", "cv::GMat", "cv::GScalar"]), _)]),
pub fn cv_GComputation_GComputation_GMat_GMat_GScalar(in1: *mut c_void, in2: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GComputation(const std::vector<GMat> &, const std::vector<GMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:219
// ("cv::GComputation::GComputation", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::GMat>*", "const std::vector<cv::GMat>*"]), _)]),
pub fn cv_GComputation_GComputation_const_vectorLGMatGR_const_vectorLGMatGR(ins: *const c_void, outs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// apply(const cv::detail::ExtractArgsCallback &, GCompileArgs &&)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:262
// ("cv::GComputation::apply", vec![(pred!(mut, ["callback", "args"], ["const cv::detail::ExtractArgsCallback*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_const_ExtractArgsCallbackR_GCompileArgsRR(instance: *mut c_void, callback: *const c_void, args: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GComputation::apply(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:262
// ("cv::GComputation::apply", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractArgsCallback*"]), _)]),
pub fn cv_GComputation_apply_const_ExtractArgsCallbackR(instance: *mut c_void, callback: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// apply(const std::vector<cv::Mat> &, const std::vector<cv::Mat> &, GCompileArgs &&)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:266
// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs", "args"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR_GCompileArgsRR(instance: *mut c_void, ins: *const c_void, outs: *const c_void, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:266
// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR(instance: *mut c_void, ins: *const c_void, outs: *const c_void, ocvrs_return: *mut Result<()>);
// apply(cv::Mat, cv::Mat &, GCompileArgs &&)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:281
// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out", "args"], ["cv::Mat", "cv::Mat*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_Mat_MatR_GCompileArgsRR(instance: *mut c_void, in_: *mut c_void, out: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:281
// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Mat*"]), _)]),
pub fn cv_GComputation_apply_Mat_MatR(instance: *mut c_void, in_: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<()>);
// apply(cv::Mat, cv::Scalar &, GCompileArgs &&)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:292
// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out", "args"], ["cv::Mat", "cv::Scalar*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_Mat_ScalarR_GCompileArgsRR(instance: *mut c_void, in_: *mut c_void, out: *mut core::Scalar, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:292
// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Scalar*"]), _)]),
pub fn cv_GComputation_apply_Mat_ScalarR(instance: *mut c_void, in_: *mut c_void, out: *mut core::Scalar, ocvrs_return: *mut Result<()>);
// apply(cv::Mat, cv::Mat, cv::Mat &, GCompileArgs &&)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:304
// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out", "args"], ["cv::Mat", "cv::Mat", "cv::Mat*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_Mat_Mat_MatR_GCompileArgsRR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:304
// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Mat*"]), _)]),
pub fn cv_GComputation_apply_Mat_Mat_MatR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut c_void, ocvrs_return: *mut Result<()>);
// apply(cv::Mat, cv::Mat, cv::Scalar &, GCompileArgs &&)(TraitClass, TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:316
// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out", "args"], ["cv::Mat", "cv::Mat", "cv::Scalar*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_Mat_Mat_ScalarR_GCompileArgsRR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut core::Scalar, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:316
// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Scalar*"]), _)]),
pub fn cv_GComputation_apply_Mat_Mat_ScalarR(instance: *mut c_void, in1: *mut c_void, in2: *mut c_void, out: *mut core::Scalar, ocvrs_return: *mut Result<()>);
// apply(const std::vector<cv::Mat> &, std::vector<cv::Mat> &, GCompileArgs &&)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:333
// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs", "args"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR_GCompileArgsRR(instance: *mut c_void, ins: *const c_void, outs: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GComputation::apply(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:333
// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR(instance: *mut c_void, ins: *const c_void, outs: *mut c_void, ocvrs_return: *mut Result<()>);
// compileStreaming(GCompileArgs &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:462
// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["args"], ["cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_compileStreaming_GCompileArgsRR(instance: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GComputation::compileStreaming() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:462
// ("cv::GComputation::compileStreaming", vec![(pred!(mut, [], []), _)]),
pub fn cv_GComputation_compileStreaming(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// compileStreaming(const cv::detail::ExtractMetaCallback &, GCompileArgs &&)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:465
// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["callback", "args"], ["const cv::detail::ExtractMetaCallback*", "cv::GCompileArgs*"]), _)]),
pub fn cv_GComputation_compileStreaming_const_ExtractMetaCallbackR_GCompileArgsRR(instance: *mut c_void, callback: *const c_void, args: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GComputation::compileStreaming(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:465
// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractMetaCallback*"]), _)]),
pub fn cv_GComputation_compileStreaming_const_ExtractMetaCallbackR(instance: *mut c_void, callback: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GComputation::delete() generated
// ("cv::GComputation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GComputation_delete(instance: *mut c_void);
// GFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:71
// ("cv::GFrame::GFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFrame_GFrame(ocvrs_return: *mut Result<*mut c_void>);
// cv::GFrame::implicitClone() generated
// ("cv::GFrame::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GFrame_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GFrame::delete() generated
// ("cv::GFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFrame_delete(instance: *mut c_void);
// operator==(const GFrameDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:101
// ("cv::GFrameDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GFrameDesc*"]), _)]),
pub fn cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(instance: *const c_void, unnamed: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GFrameDesc::defaultNew() generated
// ("cv::GFrameDesc::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GFrameDesc_defaultNew_const() -> *mut c_void;
// cv::GFrameDesc::fmt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:98
// ("cv::GFrameDesc::fmt", vec![(pred!(const, [], []), _)]),
pub fn cv_GFrameDesc_propFmt_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::MediaFormat);
// cv::GFrameDesc::setFmt(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:98
// ("cv::GFrameDesc::setFmt", vec![(pred!(mut, ["val"], ["const cv::MediaFormat"]), _)]),
pub fn cv_GFrameDesc_propFmt_const_MediaFormat(instance: *mut c_void, val: crate::gapi::MediaFormat);
// cv::GFrameDesc::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:99
// ("cv::GFrameDesc::size", vec![(pred!(const, [], []), _)]),
pub fn cv_GFrameDesc_propSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::GFrameDesc::setSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:99
// ("cv::GFrameDesc::setSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_GFrameDesc_propSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::GFrameDesc::delete() generated
// ("cv::GFrameDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GFrameDesc_delete(instance: *mut c_void);
// cv::GKernel::defaultNew() generated
// ("cv::GKernel::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_defaultNew_const() -> *mut c_void;
// cv::GKernel::name() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:48
// ("cv::GKernel::name", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_propName_const(instance: *const c_void) -> *mut c_void;
// cv::GKernel::setName(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:48
// ("cv::GKernel::setName", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_GKernel_propName_const_string(instance: *mut c_void, val: *const c_char);
// cv::GKernel::tag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:49
// ("cv::GKernel::tag", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_propTag_const(instance: *const c_void) -> *mut c_void;
// cv::GKernel::setTag(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:49
// ("cv::GKernel::setTag", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_GKernel_propTag_const_string(instance: *mut c_void, val: *const c_char);
// cv::GKernel::outShapes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:51
// ("cv::GKernel::outShapes", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_propOutShapes_const(instance: *const c_void) -> *mut c_void;
// cv::GKernel::setOutShapes(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:51
// ("cv::GKernel::setOutShapes", vec![(pred!(mut, ["val"], ["const cv::GShapes"]), _)]),
pub fn cv_GKernel_propOutShapes_const_GShapes(instance: *mut c_void, val: *const c_void);
// cv::GKernel::inKinds() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:52
// ("cv::GKernel::inKinds", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_propInKinds_const(instance: *const c_void) -> *mut c_void;
// cv::GKernel::setInKinds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:52
// ("cv::GKernel::setInKinds", vec![(pred!(mut, ["val"], ["const cv::GKinds"]), _)]),
pub fn cv_GKernel_propInKinds_const_GKinds(instance: *mut c_void, val: *const c_void);
// cv::GKernel::outKinds() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:54
// ("cv::GKernel::outKinds", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernel_propOutKinds_const(instance: *const c_void) -> *mut c_void;
// cv::GKernel::setOutKinds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:54
// ("cv::GKernel::setOutKinds", vec![(pred!(mut, ["val"], ["const cv::GKinds"]), _)]),
pub fn cv_GKernel_propOutKinds_const_GKinds(instance: *mut c_void, val: *const c_void);
// cv::GKernel::delete() generated
// ("cv::GKernel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GKernel_delete(instance: *mut c_void);
// cv::GKernelImpl::defaultNew() generated
// ("cv::GKernelImpl::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelImpl_defaultNew_const() -> *mut c_void;
// cv::GKernelImpl::opaque() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:62
// ("cv::GKernelImpl::opaque", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelImpl_propOpaque_const(instance: *const c_void) -> *mut c_void;
// cv::GKernelImpl::setOpaque(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:62
// ("cv::GKernelImpl::setOpaque", vec![(pred!(mut, ["val"], ["const cv::util::any"]), _)]),
pub fn cv_GKernelImpl_propOpaque_const_any(instance: *mut c_void, val: *const c_void);
// cv::GKernelImpl::delete() generated
// ("cv::GKernelImpl::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GKernelImpl_delete(instance: *mut c_void);
// include(const cv::gapi::GFunctor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:511
// ("cv::GKernelPackage::include", vec![(pred!(mut, ["functor"], ["const cv::gapi::GFunctor*"]), _)]),
pub fn cv_GKernelPackage_include_const_GFunctorR(instance: *mut c_void, functor: *const c_void, ocvrs_return: *mut Result<()>);
// get_transformations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:526
// ("cv::GKernelPackage::get_transformations", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelPackage_get_transformations_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// get_kernel_ids()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:533
// ("cv::GKernelPackage::get_kernel_ids", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelPackage_get_kernel_ids_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// remove(const cv::gapi::GBackend &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:564
// ("cv::GKernelPackage::remove", vec![(pred!(mut, ["backend"], ["const cv::gapi::GBackend*"]), _)]),
pub fn cv_GKernelPackage_remove_const_GBackendR(instance: *mut c_void, backend: *const c_void, ocvrs_return: *mut Result<()>);
// includesAPI(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:591
// ("cv::GKernelPackage::includesAPI", vec![(pred!(const, ["id"], ["const std::string*"]), _)]),
pub fn cv_GKernelPackage_includesAPI_const_const_stringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<bool>);
// lookup(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:611
// ("cv::GKernelPackage::lookup", vec![(pred!(const, ["id"], ["const std::string*"]), _)]),
pub fn cv_GKernelPackage_lookup_const_const_stringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// include(const cv::gapi::GBackend &, const std::string &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:630
// ("cv::GKernelPackage::include", vec![(pred!(mut, ["backend", "kernel_id"], ["const cv::gapi::GBackend*", "const std::string*"]), _)]),
pub fn cv_GKernelPackage_include_const_GBackendR_const_stringR(instance: *mut c_void, backend: *const c_void, kernel_id: *const c_char, ocvrs_return: *mut Result<()>);
// backends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:637
// ("cv::GKernelPackage::backends", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelPackage_backends_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GKernelPackage::implicitClone() generated
// ("cv::GKernelPackage::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelPackage_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GKernelPackage::defaultNew() generated
// ("cv::GKernelPackage::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GKernelPackage_defaultNew_const() -> *mut c_void;
// cv::GKernelPackage::delete() generated
// ("cv::GKernelPackage::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GKernelPackage_delete(instance: *mut c_void);
// GMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:78
// ("cv::GMat::GMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMat_GMat(ocvrs_return: *mut Result<*mut c_void>);
// GMat(cv::Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:90
// ("cv::GMat::GMat", vec![(pred!(mut, ["m"], ["cv::Mat"]), _)]),
pub fn cv_GMat_GMat_Mat(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GMat::implicitClone() generated
// ("cv::GMat::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GMat_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GMat::delete() generated
// ("cv::GMat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMat_delete(instance: *mut c_void);
// GMatDesc(int, int, cv::Size, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:126
// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "c", "s", "p"], ["int", "int", "cv::Size", "bool"]), _)]),
pub fn cv_GMatDesc_GMatDesc_int_int_Size_bool(d: i32, c: i32, s: *const core::Size, p: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::GMatDesc::GMatDesc(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:126
// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "c", "s"], ["int", "int", "cv::Size"]), _)]),
pub fn cv_GMatDesc_GMatDesc_int_int_Size(d: i32, c: i32, s: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// GMatDesc(int, const std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:129
// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "dd"], ["int", "const std::vector<int>*"]), _)]),
pub fn cv_GMatDesc_GMatDesc_int_const_vectorLintGR(d: i32, dd: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// GMatDesc(int, std::vector<int> &&)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:132
// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "dd"], ["int", "std::vector<int>*"]), _)]),
pub fn cv_GMatDesc_GMatDesc_int_vectorLintGRR(d: i32, dd: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// GMatDesc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:135
// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMatDesc_GMatDesc(ocvrs_return: *mut Result<*mut c_void>);
// operator==(const GMatDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:137
// ("cv::GMatDesc::operator==", vec![(pred!(const, ["rhs"], ["const cv::GMatDesc*"]), _)]),
pub fn cv_GMatDesc_operatorEQ_const_const_GMatDescR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// operator!=(const GMatDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:146
// ("cv::GMatDesc::operator!=", vec![(pred!(const, ["rhs"], ["const cv::GMatDesc*"]), _)]),
pub fn cv_GMatDesc_operatorNE_const_const_GMatDescR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// isND()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:151
// ("cv::GMatDesc::isND", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_isND_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// canDescribe(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:157
// ("cv::GMatDesc::canDescribe", vec![(pred!(const, ["mat"], ["const cv::Mat*"]), _)]),
pub fn cv_GMatDesc_canDescribe_const_const_MatR(instance: *const c_void, mat: *const c_void, ocvrs_return: *mut Result<bool>);
// canDescribe(const cv::RMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:159
// ("cv::GMatDesc::canDescribe", vec![(pred!(const, ["mat"], ["const cv::RMat*"]), _)]),
pub fn cv_GMatDesc_canDescribe_const_const_RMatR(instance: *const c_void, mat: *const c_void, ocvrs_return: *mut Result<bool>);
// withSizeDelta(cv::Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:164
// ("cv::GMatDesc::withSizeDelta", vec![(pred!(const, ["delta"], ["cv::Size"]), _)]),
pub fn cv_GMatDesc_withSizeDelta_const_Size(instance: *const c_void, delta: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// withSizeDelta(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:174
// ("cv::GMatDesc::withSizeDelta", vec![(pred!(const, ["dx", "dy"], ["int", "int"]), _)]),
pub fn cv_GMatDesc_withSizeDelta_const_int_int(instance: *const c_void, dx: i32, dy: i32, ocvrs_return: *mut Result<*mut c_void>);
// withSize(cv::Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:179
// ("cv::GMatDesc::withSize", vec![(pred!(const, ["sz"], ["cv::Size"]), _)]),
pub fn cv_GMatDesc_withSize_const_Size(instance: *const c_void, sz: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// withDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:188
// ("cv::GMatDesc::withDepth", vec![(pred!(const, ["ddepth"], ["int"]), _)]),
pub fn cv_GMatDesc_withDepth_const_int(instance: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// withType(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:199
// ("cv::GMatDesc::withType", vec![(pred!(const, ["ddepth", "dchan"], ["int", "int"]), _)]),
pub fn cv_GMatDesc_withType_const_int_int(instance: *const c_void, ddepth: i32, dchan: i32, ocvrs_return: *mut Result<*mut c_void>);
// asPlanar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:210
// ("cv::GMatDesc::asPlanar", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_asPlanar_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// asPlanar(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:221
// ("cv::GMatDesc::asPlanar", vec![(pred!(const, ["planes"], ["int"]), _)]),
pub fn cv_GMatDesc_asPlanar_const_int(instance: *const c_void, planes: i32, ocvrs_return: *mut Result<*mut c_void>);
// asInterleaved()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:236
// ("cv::GMatDesc::asInterleaved", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_asInterleaved_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::GMatDesc::implicitClone() generated
// ("cv::GMatDesc::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GMatDesc::depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:120
// ("cv::GMatDesc::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_propDepth_const(instance: *const c_void) -> i32;
// cv::GMatDesc::setDepth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:120
// ("cv::GMatDesc::setDepth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_GMatDesc_propDepth_const_int(instance: *mut c_void, val: i32);
// cv::GMatDesc::chan() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:121
// ("cv::GMatDesc::chan", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_propChan_const(instance: *const c_void) -> i32;
// cv::GMatDesc::setChan(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:121
// ("cv::GMatDesc::setChan", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_GMatDesc_propChan_const_int(instance: *mut c_void, val: i32);
// cv::GMatDesc::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:122
// ("cv::GMatDesc::size", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_propSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::GMatDesc::setSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:122
// ("cv::GMatDesc::setSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_GMatDesc_propSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::GMatDesc::planar() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:123
// ("cv::GMatDesc::planar", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_propPlanar_const(instance: *const c_void) -> bool;
// cv::GMatDesc::setPlanar(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:123
// ("cv::GMatDesc::setPlanar", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_GMatDesc_propPlanar_const_bool(instance: *mut c_void, val: bool);
// cv::GMatDesc::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:124
// ("cv::GMatDesc::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatDesc_propDims_const(instance: *const c_void) -> *mut c_void;
// cv::GMatDesc::setDims(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:124
// ("cv::GMatDesc::setDims", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_GMatDesc_propDims_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::GMatDesc::delete() generated
// ("cv::GMatDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMatDesc_delete(instance: *mut c_void);
// cv::GMatP::defaultNew() generated
// ("cv::GMatP::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GMatP_defaultNew_const() -> *mut c_void;
// cv::GMatP::to_GMat() generated
// ("cv::GMatP::to_GMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMatP_to_GMat(instance: *mut c_void) -> *mut c_void;
// cv::GMatP::delete() generated
// ("cv::GMatP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GMatP_delete(instance: *mut c_void);
// operator==(const GOpaqueDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gopaque.hpp:43
// ("cv::GOpaqueDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GOpaqueDesc*"]), _)]),
pub fn cv_GOpaqueDesc_operatorEQ_const_const_GOpaqueDescR(instance: *const c_void, unnamed: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GOpaqueDesc::implicitClone() generated
// ("cv::GOpaqueDesc::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GOpaqueDesc_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GOpaqueDesc::defaultNew() generated
// ("cv::GOpaqueDesc::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GOpaqueDesc_defaultNew_const() -> *mut c_void;
// cv::GOpaqueDesc::delete() generated
// ("cv::GOpaqueDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GOpaqueDesc_delete(instance: *mut c_void);
// GRunArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:129
// ("cv::GRunArg::GRunArg", vec![(pred!(mut, [], []), _)]),
pub fn cv_GRunArg_GRunArg(ocvrs_return: *mut Result<*mut c_void>);
// GRunArg(const cv::GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:130
// ("cv::GRunArg::GRunArg", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
pub fn cv_GRunArg_GRunArg_const_GRunArgR(arg: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// GRunArg(cv::GRunArg &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:131
// ("cv::GRunArg::GRunArg", vec![(pred!(mut, ["arg"], ["cv::GRunArg*"]), _)]),
pub fn cv_GRunArg_GRunArg_GRunArgRR(arg: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:133
// ("cv::GRunArg::operator=", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
pub fn cv_GRunArg_operatorST_const_GRunArgR(instance: *mut c_void, arg: *const c_void, ocvrs_return: *mut Result<()>);
// operator=(GRunArg &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:134
// ("cv::GRunArg::operator=", vec![(pred!(mut, ["arg"], ["cv::GRunArg*"]), _)]),
pub fn cv_GRunArg_operatorST_GRunArgRR(instance: *mut c_void, arg: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::GRunArg::delete() generated
// ("cv::GRunArg::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GRunArg_delete(instance: *mut c_void);
// GScalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:52
// ("cv::GScalar::GScalar", vec![(pred!(mut, [], []), _)]),
pub fn cv_GScalar_GScalar(ocvrs_return: *mut Result<*mut c_void>);
// GScalar(const cv::Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:70
// ("cv::GScalar::GScalar", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
pub fn cv_GScalar_GScalar_const_ScalarR(s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// GScalar(cv::Scalar &&)(ByMove) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:78
// ("cv::GScalar::GScalar", vec![(pred!(mut, ["s"], ["cv::Scalar*"]), _)]),
pub fn cv_GScalar_GScalar_ScalarRR(s: *mut core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// GScalar(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:98
// ("cv::GScalar::GScalar", vec![(pred!(mut, ["v0"], ["double"]), _)]),
pub fn cv_GScalar_GScalar_double(v0: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::GScalar::implicitClone() generated
// ("cv::GScalar::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GScalar_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GScalar::delete() generated
// ("cv::GScalar::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GScalar_delete(instance: *mut c_void);
// operator==(const GScalarDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:121
// ("cv::GScalarDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GScalarDesc*"]), _)]),
pub fn cv_GScalarDesc_operatorEQ_const_const_GScalarDescR(instance: *const c_void, unnamed: *const c_void, ocvrs_return: *mut Result<bool>);
// operator!=(const GScalarDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:126
// ("cv::GScalarDesc::operator!=", vec![(pred!(const, ["rhs"], ["const cv::GScalarDesc*"]), _)]),
pub fn cv_GScalarDesc_operatorNE_const_const_GScalarDescR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GScalarDesc::implicitClone() generated
// ("cv::GScalarDesc::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GScalarDesc_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GScalarDesc::defaultNew() generated
// ("cv::GScalarDesc::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GScalarDesc_defaultNew_const() -> *mut c_void;
// cv::GScalarDesc::delete() generated
// ("cv::GScalarDesc::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GScalarDesc_delete(instance: *mut c_void);
// GStreamingCompiled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:159
// ("cv::GStreamingCompiled::GStreamingCompiled", vec![(pred!(mut, [], []), _)]),
pub fn cv_GStreamingCompiled_GStreamingCompiled(ocvrs_return: *mut Result<*mut c_void>);
// setSource(GRunArgs &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:202
// ("cv::GStreamingCompiled::setSource", vec![(pred!(mut, ["ins"], ["cv::GRunArgs*"]), _)]),
pub fn cv_GStreamingCompiled_setSource_GRunArgsRR(instance: *mut c_void, ins: *mut c_void, ocvrs_return: *mut Result<()>);
// setSource(const cv::detail::ExtractArgsCallback &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:205
// ("cv::GStreamingCompiled::setSource", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractArgsCallback*"]), _)]),
pub fn cv_GStreamingCompiled_setSource_const_ExtractArgsCallbackR(instance: *mut c_void, callback: *const c_void, ocvrs_return: *mut Result<()>);
// start()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:251
// ("cv::GStreamingCompiled::start", vec![(pred!(mut, [], []), _)]),
pub fn cv_GStreamingCompiled_start(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:339
// ("cv::GStreamingCompiled::stop", vec![(pred!(mut, [], []), _)]),
pub fn cv_GStreamingCompiled_stop(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// running()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:351
// ("cv::GStreamingCompiled::running", vec![(pred!(const, [], []), _)]),
pub fn cv_GStreamingCompiled_running_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// operator bool()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:361
// ("cv::GStreamingCompiled::operator bool", vec![(pred!(const, [], []), _)]),
pub fn cv_GStreamingCompiled_operator_bool_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::GStreamingCompiled::implicitClone() generated
// ("cv::GStreamingCompiled::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GStreamingCompiled_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GStreamingCompiled::delete() generated
// ("cv::GStreamingCompiled::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GStreamingCompiled_delete(instance: *mut c_void);
// cv::GTransform::description() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gtransform.hpp:30
// ("cv::GTransform::description", vec![(pred!(const, [], []), _)]),
pub fn cv_GTransform_propDescription_const(instance: *const c_void) -> *mut c_void;
// cv::GTransform::setDescription(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gtransform.hpp:30
// ("cv::GTransform::setDescription", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_GTransform_propDescription_const_string(instance: *mut c_void, val: *const c_char);
// cv::GTransform::delete() generated
// ("cv::GTransform::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GTransform_delete(instance: *mut c_void);
// cv::GTypeInfo::implicitClone() generated
// ("cv::GTypeInfo::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_GTypeInfo_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::GTypeInfo::defaultNew() generated
// ("cv::GTypeInfo::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_GTypeInfo_defaultNew_const() -> *mut c_void;
// cv::GTypeInfo::shape() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:31
// ("cv::GTypeInfo::shape", vec![(pred!(const, [], []), _)]),
pub fn cv_GTypeInfo_propShape_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::GShape);
// cv::GTypeInfo::setShape(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:31
// ("cv::GTypeInfo::setShape", vec![(pred!(mut, ["val"], ["const cv::GShape"]), _)]),
pub fn cv_GTypeInfo_propShape_const_GShape(instance: *mut c_void, val: crate::gapi::GShape);
// cv::GTypeInfo::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:32
// ("cv::GTypeInfo::kind", vec![(pred!(const, [], []), _)]),
pub fn cv_GTypeInfo_propKind_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::Detail_OpaqueKind);
// cv::GTypeInfo::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:32
// ("cv::GTypeInfo::setKind", vec![(pred!(mut, ["val"], ["const cv::detail::OpaqueKind"]), _)]),
pub fn cv_GTypeInfo_propKind_const_OpaqueKind(instance: *mut c_void, val: crate::gapi::Detail_OpaqueKind);
// cv::GTypeInfo::delete() generated
// ("cv::GTypeInfo::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_GTypeInfo_delete(instance: *mut c_void);
// MediaFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:71
// ("cv::MediaFrame::MediaFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_MediaFrame_MediaFrame(ocvrs_return: *mut Result<*mut c_void>);
// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:111
// ("cv::MediaFrame::desc", vec![(pred!(const, [], []), _)]),
pub fn cv_MediaFrame_desc_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// blobParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:116
// ("cv::MediaFrame::blobParams", vec![(pred!(const, [], []), _)]),
pub fn cv_MediaFrame_blobParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::MediaFrame::delete() generated
// ("cv::MediaFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MediaFrame_delete(instance: *mut c_void);
// meta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:240
// ("cv::MediaFrame::IAdapter::meta", vec![(pred!(const, [], []), _)]),
pub fn cv_MediaFrame_IAdapter_meta_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// blobParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:244
// ("cv::MediaFrame::IAdapter::blobParams", vec![(pred!(const, [], []), _)]),
pub fn cv_MediaFrame_IAdapter_blobParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::MediaFrame::IAdapter::delete() generated
// ("cv::MediaFrame::IAdapter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MediaFrame_IAdapter_delete(instance: *mut c_void);
// View(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:212
// ("cv::MediaFrame::View::View", vec![(pred!(mut, ["unnamed"], ["cv::MediaFrame::View*"]), _)]),
pub fn cv_MediaFrame_View_View_ViewRR(unnamed: *mut c_void) -> *mut c_void;
// cv::MediaFrame::View::delete() generated
// ("cv::MediaFrame::View::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MediaFrame_View_delete(instance: *mut c_void);
// RMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:128
// ("cv::RMat::RMat", vec![(pred!(mut, [], []), _)]),
pub fn cv_RMat_RMat() -> *mut c_void;
// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:130
// ("cv::RMat::desc", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_desc_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::RMat::delete() generated
// ("cv::RMat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RMat_delete(instance: *mut c_void);
// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:111
// ("cv::RMat::IAdapter::desc", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_IAdapter_desc_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::RMat::IAdapter::delete() generated
// ("cv::RMat::IAdapter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RMat_IAdapter_delete(instance: *mut c_void);
// View()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:62
// ("cv::RMat::View::View", vec![(pred!(mut, [], []), _)]),
pub fn cv_RMat_View_View() -> *mut c_void;
// View(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:68
// ("cv::RMat::View::View", vec![(pred!(mut, ["unnamed"], ["cv::RMat::View*"]), _)]),
pub fn cv_RMat_View_View_ViewRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:69
// ("cv::RMat::View::operator=", vec![(pred!(mut, ["v"], ["cv::RMat::View*"]), _)]),
pub fn cv_RMat_View_operatorST_ViewRR(instance: *mut c_void, v: *mut c_void, ocvrs_return: *mut Result<()>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:72
// ("cv::RMat::View::size", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// dims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:73
// ("cv::RMat::View::dims", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_dims_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:74
// ("cv::RMat::View::cols", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// rows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:75
// ("cv::RMat::View::rows", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:76
// ("cv::RMat::View::type", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:77
// ("cv::RMat::View::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// chan()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:78
// ("cv::RMat::View::chan", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_chan_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:79
// ("cv::RMat::View::elemSize", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// step(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:93
// ("cv::RMat::View::step", vec![(pred!(const, ["i"], ["size_t"]), _)]),
pub fn cv_RMat_View_step_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
// cv::RMat::View::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:93
// ("cv::RMat::View::step", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_step_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// steps()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:96
// ("cv::RMat::View::steps", vec![(pred!(const, [], []), _)]),
pub fn cv_RMat_View_steps_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::RMat::View::delete() generated
// ("cv::RMat::View::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_RMat_View_delete(instance: *mut c_void);
// operator()(const cv::GTypesInfo &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:292
// ("cv::detail::ExtractArgsCallback::operator()", vec![(pred!(const, ["info"], ["const cv::GTypesInfo*"]), _)]),
pub fn cv_detail_ExtractArgsCallback_operator___const_const_GTypesInfoR(instance: *const c_void, info: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::detail::ExtractArgsCallback::defaultNew() generated
// ("cv::detail::ExtractArgsCallback::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ExtractArgsCallback_defaultNew_const() -> *mut c_void;
// cv::detail::ExtractArgsCallback::delete() generated
// ("cv::detail::ExtractArgsCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExtractArgsCallback_delete(instance: *mut c_void);
// cv::detail::ExtractMetaCallback::defaultNew() generated
// ("cv::detail::ExtractMetaCallback::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_detail_ExtractMetaCallback_defaultNew_const() -> *mut c_void;
// cv::detail::ExtractMetaCallback::delete() generated
// ("cv::detail::ExtractMetaCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_ExtractMetaCallback_delete(instance: *mut c_void);
// cv::detail::GArrayU::delete() generated
// ("cv::detail::GArrayU::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GArrayU_delete(instance: *mut c_void);
// cv::detail::GOpaqueU::delete() generated
// ("cv::detail::GOpaqueU::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_detail_GOpaqueU_delete(instance: *mut c_void);
// GBackend()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:385
// ("cv::gapi::GBackend::GBackend", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_GBackend_GBackend(ocvrs_return: *mut Result<*mut c_void>);
// operator==(const GBackend &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:392
// ("cv::gapi::GBackend::operator==", vec![(pred!(const, ["rhs"], ["const cv::gapi::GBackend*"]), _)]),
pub fn cv_gapi_GBackend_operatorEQ_const_const_GBackendR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::gapi::GBackend::delete() generated
// ("cv::gapi::GBackend::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_GBackend_delete(instance: *mut c_void);
// impl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:427
// ("cv::gapi::GFunctor::impl", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_GFunctor_impl_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// backend()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:428
// ("cv::gapi::GFunctor::backend", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_GFunctor_backend_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// id()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:429
// ("cv::gapi::GFunctor::id", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_GFunctor_id_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::GFunctor::delete() generated
// ("cv::gapi::GFunctor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_GFunctor_delete(instance: *mut c_void);
// Scalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:23
// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_own_Scalar_Scalar() -> *mut c_void;
// Scalar(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:24
// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0"], ["double"]), _)]),
pub fn cv_gapi_own_Scalar_Scalar_double(v0: f64, ocvrs_return: *mut Result<*mut c_void>);
// Scalar(double, double, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:25
// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0", "v1", "v2", "v3"], ["double", "double", "double", "double"]), _)]),
pub fn cv_gapi_own_Scalar_Scalar_double_double_double_double(v0: f64, v1: f64, v2: f64, v3: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::own::Scalar::Scalar(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:25
// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0", "v1"], ["double", "double"]), _)]),
pub fn cv_gapi_own_Scalar_Scalar_double_double(v0: f64, v1: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:30
// ("cv::gapi::own::Scalar::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
pub fn cv_gapi_own_Scalar_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<f64>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:31
// ("cv::gapi::own::Scalar::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
pub fn cv_gapi_own_Scalar_operator___int(instance: *mut c_void, i: i32, ocvrs_return: *mut Result<f64>);
// all(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:33
// ("cv::gapi::own::Scalar::all", vec![(pred!(mut, ["v0"], ["double"]), _)]),
pub fn cv_gapi_own_Scalar_all_double(v0: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::own::Scalar::val() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:35
// ("cv::gapi::own::Scalar::val", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_own_Scalar_propVal_const(instance: *const c_void) -> *const [f64; 4];
// cv::gapi::own::Scalar::valMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:35
// ("cv::gapi::own::Scalar::valMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_own_Scalar_propVal(instance: *mut c_void) -> *mut [f64; 4];
// cv::gapi::own::Scalar::delete() generated
// ("cv::gapi::own::Scalar::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_own_Scalar_delete(instance: *mut c_void);
// queue_capacity(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:411
// ("cv::gapi::streaming::queue_capacity::queue_capacity", vec![(pred!(mut, ["cap"], ["size_t"]), _)]),
pub fn cv_gapi_streaming_queue_capacity_queue_capacity_size_t(cap: size_t, ocvrs_return: *mut Result<crate::gapi::queue_capacity>);
// cv::gapi::streaming::queue_capacity::queue_capacity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:411
// ("cv::gapi::streaming::queue_capacity::queue_capacity", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_streaming_queue_capacity_queue_capacity(ocvrs_return: *mut Result<crate::gapi::queue_capacity>);
// cv::gapi::use_only::defaultNew() generated
// ("cv::gapi::use_only::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_use_only_defaultNew_const() -> *mut c_void;
// cv::gapi::use_only::pkg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:736
// ("cv::gapi::use_only::pkg", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_use_only_propPkg_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::use_only::setPkg(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:736
// ("cv::gapi::use_only::setPkg", vec![(pred!(mut, ["val"], ["const cv::gapi::GKernelPackage"]), _)]),
pub fn cv_gapi_use_only_propPkg_const_GKernelPackage(instance: *mut c_void, val: *const c_void);
// cv::gapi::use_only::delete() generated
// ("cv::gapi::use_only::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_use_only_delete(instance: *mut c_void);
// cv::gapi::wip::Data::defaultNew() generated
// ("cv::gapi::wip::Data::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_Data_defaultNew_const() -> *mut c_void;
// cv::gapi::wip::Data::to_GRunArg() generated
// ("cv::gapi::wip::Data::to_GRunArg", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_Data_to_GRunArg(instance: *mut c_void) -> *mut c_void;
// cv::gapi::wip::Data::delete() generated
// ("cv::gapi::wip::Data::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_Data_delete(instance: *mut c_void);
// Circle(const cv::Point &, int, const cv::Scalar &, int, int, int)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:177
// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, ["center_", "radius_", "color_", "thick_", "lt_", "shift_"], ["const cv::Point*", "int", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR_int_int_int(center_: *const core::Point, radius_: i32, color_: *const core::Scalar, thick_: i32, lt_: i32, shift_: i32, ocvrs_return: *mut Result<crate::gapi::Circle>);
// cv::gapi::wip::draw::Circle::Circle(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:177
// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, ["center_", "radius_", "color_"], ["const cv::Point*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR(center_: *const core::Point, radius_: i32, color_: *const core::Scalar, ocvrs_return: *mut Result<crate::gapi::Circle>);
// Circle()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:188
// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Circle_Circle(ocvrs_return: *mut crate::gapi::Circle);
// Image(const cv::Point &, const cv::Mat &, const cv::Mat &)(SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:287
// ("cv::gapi::wip::draw::Image::Image", vec![(pred!(mut, ["org_", "img_", "alpha_"], ["const cv::Point*", "const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_gapi_wip_draw_Image_Image_const_PointR_const_MatR_const_MatR(org_: *const core::Point, img_: *const c_void, alpha_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// Image()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:295
// ("cv::gapi::wip::draw::Image::Image", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_Image() -> *mut c_void;
// cv::gapi::wip::draw::Image::implicitClone() generated
// ("cv::gapi::wip::draw::Image::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Image::org() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:298
// ("cv::gapi::wip::draw::Image::org", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_propOrg_const(instance: *const c_void, ocvrs_return: *mut core::Point);
// cv::gapi::wip::draw::Image::setOrg(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:298
// ("cv::gapi::wip::draw::Image::setOrg", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
pub fn cv_gapi_wip_draw_Image_propOrg_const_Point(instance: *mut c_void, val: *const core::Point);
// cv::gapi::wip::draw::Image::img() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:299
// ("cv::gapi::wip::draw::Image::img", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_propImg_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Image::setImg(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:299
// ("cv::gapi::wip::draw::Image::setImg", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_gapi_wip_draw_Image_propImg_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::gapi::wip::draw::Image::alpha() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:300
// ("cv::gapi::wip::draw::Image::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_propAlpha_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Image::setAlpha(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:300
// ("cv::gapi::wip::draw::Image::setAlpha", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_gapi_wip_draw_Image_propAlpha_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::gapi::wip::draw::Image::delete() generated
// ("cv::gapi::wip::draw::Image::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Image_delete(instance: *mut c_void);
// Line(const cv::Point &, const cv::Point &, const cv::Scalar &, int, int, int)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:218
// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, ["pt1_", "pt2_", "color_", "thick_", "lt_", "shift_"], ["const cv::Point*", "const cv::Point*", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR_int_int_int(pt1_: *const core::Point, pt2_: *const core::Point, color_: *const core::Scalar, thick_: i32, lt_: i32, shift_: i32, ocvrs_return: *mut Result<crate::gapi::Line>);
// cv::gapi::wip::draw::Line::Line(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:218
// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, ["pt1_", "pt2_", "color_"], ["const cv::Point*", "const cv::Point*", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR(pt1_: *const core::Point, pt2_: *const core::Point, color_: *const core::Scalar, ocvrs_return: *mut Result<crate::gapi::Line>);
// Line()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:229
// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Line_Line(ocvrs_return: *mut crate::gapi::Line);
// Mosaic(const cv::Rect &, int, int)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:255
// ("cv::gapi::wip::draw::Mosaic::Mosaic", vec![(pred!(mut, ["mos_", "cellSz_", "decim_"], ["const cv::Rect*", "int", "int"]), _)]),
pub fn cv_gapi_wip_draw_Mosaic_Mosaic_const_RectR_int_int(mos_: *const core::Rect, cell_sz_: i32, decim_: i32, ocvrs_return: *mut Result<crate::gapi::Mosaic>);
// Mosaic()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:263
// ("cv::gapi::wip::draw::Mosaic::Mosaic", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Mosaic_Mosaic(ocvrs_return: *mut Result<crate::gapi::Mosaic>);
// Poly(const std::vector<cv::Point> &, const cv::Scalar &, int, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:319
// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, ["points_", "color_", "thick_", "lt_", "shift_"], ["const std::vector<cv::Point>*", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR_int_int_int(points_: *const c_void, color_: *const core::Scalar, thick_: i32, lt_: i32, shift_: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::wip::draw::Poly::Poly(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:319
// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, ["points_", "color_"], ["const std::vector<cv::Point>*", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR(points_: *const c_void, color_: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Poly()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:329
// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_Poly() -> *mut c_void;
// cv::gapi::wip::draw::Poly::implicitClone() generated
// ("cv::gapi::wip::draw::Poly::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Poly::points() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:332
// ("cv::gapi::wip::draw::Poly::points", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_propPoints_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Poly::setPoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:332
// ("cv::gapi::wip::draw::Poly::setPoints", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
pub fn cv_gapi_wip_draw_Poly_propPoints_const_vectorLPointG(instance: *mut c_void, val: *const c_void);
// cv::gapi::wip::draw::Poly::color() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:333
// ("cv::gapi::wip::draw::Poly::color", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_propColor_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::gapi::wip::draw::Poly::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:333
// ("cv::gapi::wip::draw::Poly::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_gapi_wip_draw_Poly_propColor_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::gapi::wip::draw::Poly::thick() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:334
// ("cv::gapi::wip::draw::Poly::thick", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_propThick_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Poly::setThick(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:334
// ("cv::gapi::wip::draw::Poly::setThick", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Poly_propThick_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Poly::lt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:335
// ("cv::gapi::wip::draw::Poly::lt", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_propLt_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Poly::setLt(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:335
// ("cv::gapi::wip::draw::Poly::setLt", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Poly_propLt_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Poly::shift() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:336
// ("cv::gapi::wip::draw::Poly::shift", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_propShift_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Poly::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:336
// ("cv::gapi::wip::draw::Poly::setShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Poly_propShift_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Poly::delete() generated
// ("cv::gapi::wip::draw::Poly::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Poly_delete(instance: *mut c_void);
// Rect(const cv::Rect &, const cv::Scalar &, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:138
// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, ["rect_", "color_", "thick_", "lt_", "shift_"], ["const cv::Rect*", "const cv::Scalar*", "int", "int", "int"]), _)]),
pub fn cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR_int_int_int(rect_: *const core::Rect, color_: *const core::Scalar, thick_: i32, lt_: i32, shift_: i32, ocvrs_return: *mut Result<crate::gapi::Rect>);
// cv::gapi::wip::draw::Rect::Rect(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:138
// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, ["rect_", "color_"], ["const cv::Rect*", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR(rect_: *const core::Rect, color_: *const core::Scalar, ocvrs_return: *mut Result<crate::gapi::Rect>);
// Rect()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:148
// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Rect_Rect(ocvrs_return: *mut crate::gapi::Rect);
// Text(const std::string &, const cv::Point &, int, double, const cv::Scalar &, int, int, bool)(InString, SimpleClass, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:59
// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, ["text_", "org_", "ff_", "fs_", "color_", "thick_", "lt_", "bottom_left_origin_"], ["const std::string*", "const cv::Point*", "int", "double", "const cv::Scalar*", "int", "int", "bool"]), _)]),
pub fn cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR_int_int_bool(text_: *const c_char, org_: *const core::Point, ff_: i32, fs_: f64, color_: *const core::Scalar, thick_: i32, lt_: i32, bottom_left_origin_: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::gapi::wip::draw::Text::Text(InString, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:59
// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, ["text_", "org_", "ff_", "fs_", "color_"], ["const std::string*", "const cv::Point*", "int", "double", "const cv::Scalar*"]), _)]),
pub fn cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR(text_: *const c_char, org_: *const core::Point, ff_: i32, fs_: f64, color_: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// Text()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:73
// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_Text() -> *mut c_void;
// cv::gapi::wip::draw::Text::implicitClone() generated
// ("cv::gapi::wip::draw::Text::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Text::text() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:76
// ("cv::gapi::wip::draw::Text::text", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propText_const(instance: *const c_void) -> *mut c_void;
// cv::gapi::wip::draw::Text::setText(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:76
// ("cv::gapi::wip::draw::Text::setText", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_gapi_wip_draw_Text_propText_const_string(instance: *mut c_void, val: *const c_char);
// cv::gapi::wip::draw::Text::org() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:77
// ("cv::gapi::wip::draw::Text::org", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propOrg_const(instance: *const c_void, ocvrs_return: *mut core::Point);
// cv::gapi::wip::draw::Text::setOrg(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:77
// ("cv::gapi::wip::draw::Text::setOrg", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
pub fn cv_gapi_wip_draw_Text_propOrg_const_Point(instance: *mut c_void, val: *const core::Point);
// cv::gapi::wip::draw::Text::ff() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:78
// ("cv::gapi::wip::draw::Text::ff", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propFf_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Text::setFf(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:78
// ("cv::gapi::wip::draw::Text::setFf", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Text_propFf_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Text::fs() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:79
// ("cv::gapi::wip::draw::Text::fs", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propFs_const(instance: *const c_void) -> f64;
// cv::gapi::wip::draw::Text::setFs(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:79
// ("cv::gapi::wip::draw::Text::setFs", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_gapi_wip_draw_Text_propFs_const_double(instance: *mut c_void, val: f64);
// cv::gapi::wip::draw::Text::color() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:80
// ("cv::gapi::wip::draw::Text::color", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propColor_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::gapi::wip::draw::Text::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:80
// ("cv::gapi::wip::draw::Text::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_gapi_wip_draw_Text_propColor_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::gapi::wip::draw::Text::thick() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:81
// ("cv::gapi::wip::draw::Text::thick", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propThick_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Text::setThick(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:81
// ("cv::gapi::wip::draw::Text::setThick", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Text_propThick_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Text::lt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:82
// ("cv::gapi::wip::draw::Text::lt", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propLt_const(instance: *const c_void) -> i32;
// cv::gapi::wip::draw::Text::setLt(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:82
// ("cv::gapi::wip::draw::Text::setLt", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_gapi_wip_draw_Text_propLt_const_int(instance: *mut c_void, val: i32);
// cv::gapi::wip::draw::Text::bottom_left_origin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:83
// ("cv::gapi::wip::draw::Text::bottom_left_origin", vec![(pred!(const, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_propBottom_left_origin_const(instance: *const c_void) -> bool;
// cv::gapi::wip::draw::Text::setBottom_left_origin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:83
// ("cv::gapi::wip::draw::Text::setBottom_left_origin", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_gapi_wip_draw_Text_propBottom_left_origin_const_bool(instance: *mut c_void, val: bool);
// cv::gapi::wip::draw::Text::delete() generated
// ("cv::gapi::wip::draw::Text::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_gapi_wip_draw_Text_delete(instance: *mut c_void);
// use_threaded_executor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:275
// ("cv::use_threaded_executor::use_threaded_executor", vec![(pred!(mut, [], []), _)]),
pub fn cv_use_threaded_executor_use_threaded_executor(ocvrs_return: *mut Result<*mut c_void>);
// use_threaded_executor(const uint32_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:276
// ("cv::use_threaded_executor::use_threaded_executor", vec![(pred!(mut, ["nthreads"], ["const uint32_t"]), _)]),
pub fn cv_use_threaded_executor_use_threaded_executor_const_uint32_t(nthreads: u32, ocvrs_return: *mut Result<*mut c_void>);
// cv::use_threaded_executor::num_threads() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:278
// ("cv::use_threaded_executor::num_threads", vec![(pred!(const, [], []), _)]),
pub fn cv_use_threaded_executor_propNum_threads_const(instance: *const c_void) -> u32;
// cv::use_threaded_executor::setNum_threads(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:278
// ("cv::use_threaded_executor::setNum_threads", vec![(pred!(mut, ["val"], ["const uint32_t"]), _)]),
pub fn cv_use_threaded_executor_propNum_threads_const_uint32_t(instance: *mut c_void, val: u32);
// cv::use_threaded_executor::delete() generated
// ("cv::use_threaded_executor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_use_threaded_executor_delete(instance: *mut c_void);
// any(const any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:82
// ("cv::util::any::any", vec![(pred!(mut, ["src"], ["const cv::util::any*"]), _)]),
pub fn cv_util_any_any_const_anyR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// any(any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:84
// ("cv::util::any::any", vec![(pred!(mut, ["src"], ["cv::util::any*"]), _)]),
pub fn cv_util_any_any_anyR(src: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// any()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:86
// ("cv::util::any::any", vec![(pred!(mut, [], []), _)]),
pub fn cv_util_any_any() -> *mut c_void;
// any(any &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:87
// ("cv::util::any::any", vec![(pred!(mut, ["unnamed"], ["cv::util::any*"]), _)]),
pub fn cv_util_any_any_anyRR(unnamed: *mut c_void) -> *mut c_void;
// operator=(any &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:89
// ("cv::util::any::operator=", vec![(pred!(mut, ["unnamed"], ["cv::util::any*"]), _)]),
pub fn cv_util_any_operatorST_anyRR(instance: *mut c_void, unnamed: *mut c_void);
// operator=(const any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:91
// ("cv::util::any::operator=", vec![(pred!(mut, ["src"], ["const cv::util::any*"]), _)]),
pub fn cv_util_any_operatorST_const_anyR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<()>);
// cv::util::any::delete() generated
// ("cv::util::any::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_util_any_delete(instance: *mut c_void);
