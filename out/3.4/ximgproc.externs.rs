// cv::ximgproc::BrightEdges(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/brightedges.hpp:48
// ("cv::ximgproc::BrightEdges", vec![(pred!(mut, ["_original", "_edgeview"], ["cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_ximgproc_BrightEdges_MatR_MatR(_original: *mut c_void, _edgeview: *mut c_void, ocvrs_return: *mut Result<()>);
// BrightEdges(Mat &, Mat &, int, int, int)(TraitClass, TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/brightedges.hpp:48
// ("cv::ximgproc::BrightEdges", vec![(pred!(mut, ["_original", "_edgeview", "contrast", "shortrange", "longrange"], ["cv::Mat*", "cv::Mat*", "int", "int", "int"]), _)]),
pub fn cv_ximgproc_BrightEdges_MatR_MatR_int_int_int(_original: *mut c_void, _edgeview: *mut c_void, contrast: i32, shortrange: i32, longrange: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::FastHoughTransform(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_hough_transform.hpp:133
// ("cv::ximgproc::FastHoughTransform", vec![(pred!(mut, ["src", "dst", "dstMatDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, dst_mat_depth: i32, ocvrs_return: *mut Result<()>);
// FastHoughTransform(InputArray, OutputArray, int, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_hough_transform.hpp:133
// ("cv::ximgproc::FastHoughTransform", vec![(pred!(mut, ["src", "dst", "dstMatDepth", "angleRange", "op", "makeSkew"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int"]), _)]),
pub fn cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(src: *const c_void, dst: *const c_void, dst_mat_depth: i32, angle_range: i32, op: i32, make_skew: i32, ocvrs_return: *mut Result<()>);
// GradientDericheX(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/deriche_filter.hpp:72
// ("cv::ximgproc::GradientDericheX", vec![(pred!(mut, ["op", "dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result<()>);
// GradientDericheY(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/deriche_filter.hpp:60
// ("cv::ximgproc::GradientDericheY", vec![(pred!(mut, ["op", "dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result<()>);
// GradientPaillouX(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/paillou_filter.hpp:62
// ("cv::ximgproc::GradientPaillouX", vec![(pred!(mut, ["op", "_dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, _dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result<()>);
// GradientPaillouY(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/paillou_filter.hpp:61
// ("cv::ximgproc::GradientPaillouY", vec![(pred!(mut, ["op", "_dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, _dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result<()>);
// cv::ximgproc::HoughPoint2Line(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_hough_transform.hpp:155
// ("cv::ximgproc::HoughPoint2Line", vec![(pred!(mut, ["houghPoint", "srcImgInfo"], ["const cv::Point*", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR(hough_point: *const core::Point, src_img_info: *const c_void, ocvrs_return: *mut Result<core::Vec4i>);
// HoughPoint2Line(const Point &, InputArray, int, int, int)(SimpleClass, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_hough_transform.hpp:155
// ("cv::ximgproc::HoughPoint2Line", vec![(pred!(mut, ["houghPoint", "srcImgInfo", "angleRange", "makeSkew", "rules"], ["const cv::Point*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
pub fn cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(hough_point: *const core::Point, src_img_info: *const c_void, angle_range: i32, make_skew: i32, rules: i32, ocvrs_return: *mut Result<core::Vec4i>);
// PeiLinNormalization(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/peilin.hpp:26
// ("cv::ximgproc::PeiLinNormalization", vec![(pred!(mut, ["I"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_PeiLinNormalization_const__InputArrayR(i: *const c_void, ocvrs_return: *mut Result<core::Matx23d>);
// PeiLinNormalization(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/peilin.hpp:28
// ("cv::ximgproc::PeiLinNormalization", vec![(pred!(mut, ["I", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_PeiLinNormalization_const__InputArrayR_const__OutputArrayR(i: *const c_void, t: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::amFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:284
// ("cv::ximgproc::amFilter", vec![(pred!(mut, ["joint", "src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(joint: *const c_void, src: *const c_void, dst: *const c_void, sigma_s: f64, sigma_r: f64, ocvrs_return: *mut Result<()>);
// amFilter(InputArray, InputArray, OutputArray, double, double, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:284
// ("cv::ximgproc::amFilter", vec![(pred!(mut, ["joint", "src", "dst", "sigma_s", "sigma_r", "adjust_outliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "bool"]), _)]),
pub fn cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(joint: *const c_void, src: *const c_void, dst: *const c_void, sigma_s: f64, sigma_r: f64, adjust_outliers: bool, ocvrs_return: *mut Result<()>);
// anisotropicDiffusion(InputArray, OutputArray, float, float, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc.hpp:184
// ("cv::ximgproc::anisotropicDiffusion", vec![(pred!(mut, ["src", "dst", "alpha", "K", "niters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int"]), _)]),
pub fn cv_ximgproc_anisotropicDiffusion_const__InputArrayR_const__OutputArrayR_float_float_int(src: *const c_void, dst: *const c_void, alpha: f32, k: f32, niters: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::bilateralTextureFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:339
// ("cv::ximgproc::bilateralTextureFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// bilateralTextureFilter(InputArray, OutputArray, int, int, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:339
// ("cv::ximgproc::bilateralTextureFilter", vec![(pred!(mut, ["src", "dst", "fr", "numIter", "sigmaAlpha", "sigmaAvg"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double"]), _)]),
pub fn cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(src: *const c_void, dst: *const c_void, fr: i32, num_iter: i32, sigma_alpha: f64, sigma_avg: f64, ocvrs_return: *mut Result<()>);
// cv::ximgproc::computeBadPixelPercent(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:193
// ("cv::ximgproc::computeBadPixelPercent", vec![(pred!(mut, ["GT", "src", "ROI"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect"]), _)]),
pub fn cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect(gt: *const c_void, src: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<f64>);
// computeBadPixelPercent(InputArray, InputArray, Rect, int)(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:193
// ("cv::ximgproc::computeBadPixelPercent", vec![(pred!(mut, ["GT", "src", "ROI", "thresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect", "int"]), _)]),
pub fn cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(gt: *const c_void, src: *const c_void, roi: *const core::Rect, thresh: i32, ocvrs_return: *mut Result<f64>);
// computeMSE(InputArray, InputArray, Rect)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:177
// ("cv::ximgproc::computeMSE", vec![(pred!(mut, ["GT", "src", "ROI"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect"]), _)]),
pub fn cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(gt: *const c_void, src: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<f64>);
// contourSampling(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:106
// ("cv::ximgproc::contourSampling", vec![(pred!(mut, ["src", "out", "nbElt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_contourSampling_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, out: *const c_void, nb_elt: i32, ocvrs_return: *mut Result<()>);
// covarianceEstimation(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/estimated_covariance.hpp:77
// ("cv::ximgproc::covarianceEstimation", vec![(pred!(mut, ["src", "dst", "windowRows", "windowCols"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, window_rows: i32, window_cols: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::createAMFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:262
// ("cv::ximgproc::createAMFilter", vec![(pred!(mut, ["sigma_s", "sigma_r"], ["double", "double"]), _)]),
pub fn cv_ximgproc_createAMFilter_double_double(sigma_s: f64, sigma_r: f64, ocvrs_return: *mut Result<*mut c_void>);
// createAMFilter(double, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:262
// ("cv::ximgproc::createAMFilter", vec![(pred!(mut, ["sigma_s", "sigma_r", "adjust_outliers"], ["double", "double", "bool"]), _)]),
pub fn cv_ximgproc_createAMFilter_double_double_bool(sigma_s: f64, sigma_r: f64, adjust_outliers: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createContourFitting() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:114
// ("cv::ximgproc::createContourFitting", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createContourFitting(ocvrs_return: *mut Result<*mut c_void>);
// createContourFitting(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:114
// ("cv::ximgproc::createContourFitting", vec![(pred!(mut, ["ctr", "fd"], ["int", "int"]), _)]),
pub fn cv_ximgproc_createContourFitting_int_int(ctr: i32, fd: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createDTFilter(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:102
// ("cv::ximgproc::createDTFilter", vec![(pred!(mut, ["guide", "sigmaSpatial", "sigmaColor"], ["const cv::_InputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_createDTFilter_const__InputArrayR_double_double(guide: *const c_void, sigma_spatial: f64, sigma_color: f64, ocvrs_return: *mut Result<*mut c_void>);
// createDTFilter(InputArray, double, double, int, int)(InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:102
// ("cv::ximgproc::createDTFilter", vec![(pred!(mut, ["guide", "sigmaSpatial", "sigmaColor", "mode", "numIters"], ["const cv::_InputArray*", "double", "double", "int", "int"]), _)]),
pub fn cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(guide: *const c_void, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32, ocvrs_return: *mut Result<*mut c_void>);
// createDisparityWLSFilterGeneric(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:149
// ("cv::ximgproc::createDisparityWLSFilterGeneric", vec![(pred!(mut, ["use_confidence"], ["bool"]), _)]),
pub fn cv_ximgproc_createDisparityWLSFilterGeneric_bool(use_confidence: bool, ocvrs_return: *mut Result<*mut c_void>);
// createDisparityWLSFilter(Ptr<StereoMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:131
// ("cv::ximgproc::createDisparityWLSFilter", vec![(pred!(mut, ["matcher_left"], ["cv::Ptr<cv::StereoMatcher>"]), _)]),
pub fn cv_ximgproc_createDisparityWLSFilter_PtrLStereoMatcherG(matcher_left: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createEdgeAwareInterpolator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:126
// ("cv::ximgproc::createEdgeAwareInterpolator", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createEdgeAwareInterpolator(ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createEdgeBoxes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:183
// ("cv::ximgproc::createEdgeBoxes", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createEdgeBoxes(ocvrs_return: *mut Result<*mut c_void>);
// createEdgeBoxes(float, float, float, float, int, float, float, float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:183
// ("cv::ximgproc::createEdgeBoxes", vec![(pred!(mut, ["alpha", "beta", "eta", "minScore", "maxBoxes", "edgeMinMag", "edgeMergeThr", "clusterMinMag", "maxAspectRatio", "minBoxArea", "gamma", "kappa"], ["float", "float", "float", "float", "int", "float", "float", "float", "float", "float", "float", "float"]), _)]),
pub fn cv_ximgproc_createEdgeBoxes_float_float_float_float_int_float_float_float_float_float_float_float(alpha: f32, beta: f32, eta: f32, min_score: f32, max_boxes: i32, edge_min_mag: f32, edge_merge_thr: f32, cluster_min_mag: f32, max_aspect_ratio: f32, min_box_area: f32, gamma: f32, kappa: f32, ocvrs_return: *mut Result<*mut c_void>);
// createEdgeDrawing()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:122
// ("cv::ximgproc::createEdgeDrawing", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createEdgeDrawing(ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createFastBilateralSolverFilter(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:417
// ("cv::ximgproc::createFastBilateralSolverFilter", vec![(pred!(mut, ["guide", "sigma_spatial", "sigma_luma", "sigma_chroma"], ["const cv::_InputArray*", "double", "double", "double"]), _)]),
pub fn cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double(guide: *const c_void, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, ocvrs_return: *mut Result<*mut c_void>);
// createFastBilateralSolverFilter(InputArray, double, double, double, double, int, double)(InputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:417
// ("cv::ximgproc::createFastBilateralSolverFilter", vec![(pred!(mut, ["guide", "sigma_spatial", "sigma_luma", "sigma_chroma", "lambda", "num_iter", "max_tol"], ["const cv::_InputArray*", "double", "double", "double", "double", "int", "double"]), _)]),
pub fn cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double_double_int_double(guide: *const c_void, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createFastGlobalSmootherFilter(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:488
// ("cv::ximgproc::createFastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "lambda", "sigma_color"], ["const cv::_InputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double(guide: *const c_void, lambda: f64, sigma_color: f64, ocvrs_return: *mut Result<*mut c_void>);
// createFastGlobalSmootherFilter(InputArray, double, double, double, int)(InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:488
// ("cv::ximgproc::createFastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "lambda", "sigma_color", "lambda_attenuation", "num_iter"], ["const cv::_InputArray*", "double", "double", "double", "int"]), _)]),
pub fn cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(guide: *const c_void, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createFastLineDetector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_line_detector.hpp:71
// ("cv::ximgproc::createFastLineDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createFastLineDetector(ocvrs_return: *mut Result<*mut c_void>);
// createFastLineDetector(int, float, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_line_detector.hpp:71
// ("cv::ximgproc::createFastLineDetector", vec![(pred!(mut, ["length_threshold", "distance_threshold", "canny_th1", "canny_th2", "canny_aperture_size", "do_merge"], ["int", "float", "double", "double", "int", "bool"]), _)]),
pub fn cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(length_threshold: i32, distance_threshold: f32, canny_th1: f64, canny_th2: f64, canny_aperture_size: i32, do_merge: bool, ocvrs_return: *mut Result<*mut c_void>);
// createGuidedFilter(InputArray, int, double)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:158
// ("cv::ximgproc::createGuidedFilter", vec![(pred!(mut, ["guide", "radius", "eps"], ["const cv::_InputArray*", "int", "double"]), _)]),
pub fn cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(guide: *const c_void, radius: i32, eps: f64, ocvrs_return: *mut Result<*mut c_void>);
// createRFFeatureGetter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:91
// ("cv::ximgproc::createRFFeatureGetter", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_createRFFeatureGetter(ocvrs_return: *mut Result<*mut c_void>);
// createRightMatcher(Ptr<StereoMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:139
// ("cv::ximgproc::createRightMatcher", vec![(pred!(mut, ["matcher_left"], ["cv::Ptr<cv::StereoMatcher>"]), _)]),
pub fn cv_ximgproc_createRightMatcher_PtrLStereoMatcherG(matcher_left: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createStructuredEdgeDetection(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:140
// ("cv::ximgproc::createStructuredEdgeDetection", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
pub fn cv_ximgproc_createStructuredEdgeDetection_const_StringR(model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createStructuredEdgeDetection(const String &, Ptr<const RFFeatureGetter>)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:140
// ("cv::ximgproc::createStructuredEdgeDetection", vec![(pred!(mut, ["model", "howToGetFeatures"], ["const cv::String*", "cv::Ptr<const cv::ximgproc::RFFeatureGetter>"]), _)]),
pub fn cv_ximgproc_createStructuredEdgeDetection_const_StringR_PtrLconst_RFFeatureGetterG(model: *const c_char, how_to_get_features: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createSuperpixelLSC(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:150
// ("cv::ximgproc::createSuperpixelLSC", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_createSuperpixelLSC_const__InputArrayR(image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSuperpixelLSC(InputArray, int, float)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:150
// ("cv::ximgproc::createSuperpixelLSC", vec![(pred!(mut, ["image", "region_size", "ratio"], ["const cv::_InputArray*", "int", "float"]), _)]),
pub fn cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(image: *const c_void, region_size: i32, ratio: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createSuperpixelSEEDS(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:173
// ("cv::ximgproc::createSuperpixelSEEDS", vec![(pred!(mut, ["image_width", "image_height", "image_channels", "num_superpixels", "num_levels"], ["int", "int", "int", "int", "int"]), _)]),
pub fn cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int(image_width: i32, image_height: i32, image_channels: i32, num_superpixels: i32, num_levels: i32, ocvrs_return: *mut Result<*mut c_void>);
// createSuperpixelSEEDS(int, int, int, int, int, int, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:173
// ("cv::ximgproc::createSuperpixelSEEDS", vec![(pred!(mut, ["image_width", "image_height", "image_channels", "num_superpixels", "num_levels", "prior", "histogram_bins", "double_step"], ["int", "int", "int", "int", "int", "int", "int", "bool"]), _)]),
pub fn cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(image_width: i32, image_height: i32, image_channels: i32, num_superpixels: i32, num_levels: i32, prior: i32, histogram_bins: i32, double_step: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::createSuperpixelSLIC(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:161
// ("cv::ximgproc::createSuperpixelSLIC", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_createSuperpixelSLIC_const__InputArrayR(image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSuperpixelSLIC(InputArray, int, int, float)(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:161
// ("cv::ximgproc::createSuperpixelSLIC", vec![(pred!(mut, ["image", "algorithm", "region_size", "ruler"], ["const cv::_InputArray*", "int", "int", "float"]), _)]),
pub fn cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(image: *const c_void, algorithm: i32, region_size: i32, ruler: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::dtFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:121
// ("cv::ximgproc::dtFilter", vec![(pred!(mut, ["guide", "src", "dst", "sigmaSpatial", "sigmaColor"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(guide: *const c_void, src: *const c_void, dst: *const c_void, sigma_spatial: f64, sigma_color: f64, ocvrs_return: *mut Result<()>);
// dtFilter(InputArray, InputArray, OutputArray, double, double, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:121
// ("cv::ximgproc::dtFilter", vec![(pred!(mut, ["guide", "src", "dst", "sigmaSpatial", "sigmaColor", "mode", "numIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int"]), _)]),
pub fn cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(guide: *const c_void, src: *const c_void, dst: *const c_void, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::fastBilateralSolverFilter(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:447
// ("cv::ximgproc::fastBilateralSolverFilter", vec![(pred!(mut, ["guide", "src", "confidence", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(guide: *const c_void, src: *const c_void, confidence: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// fastBilateralSolverFilter(InputArray, InputArray, InputArray, OutputArray, double, double, double, double, int, double)(InputArray, InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:447
// ("cv::ximgproc::fastBilateralSolverFilter", vec![(pred!(mut, ["guide", "src", "confidence", "dst", "sigma_spatial", "sigma_luma", "sigma_chroma", "lambda", "num_iter", "max_tol"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "double", "double", "int", "double"]), _)]),
pub fn cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_double_int_double(guide: *const c_void, src: *const c_void, confidence: *const c_void, dst: *const c_void, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64, ocvrs_return: *mut Result<()>);
// cv::ximgproc::fastGlobalSmootherFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:508
// ("cv::ximgproc::fastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "src", "dst", "lambda", "sigma_color"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(guide: *const c_void, src: *const c_void, dst: *const c_void, lambda: f64, sigma_color: f64, ocvrs_return: *mut Result<()>);
// fastGlobalSmootherFilter(InputArray, InputArray, OutputArray, double, double, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:508
// ("cv::ximgproc::fastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "src", "dst", "lambda", "sigma_color", "lambda_attenuation", "num_iter"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "double", "int"]), _)]),
pub fn cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(guide: *const c_void, src: *const c_void, dst: *const c_void, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::fourierDescriptor(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:87
// ("cv::ximgproc::fourierDescriptor", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// fourierDescriptor(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:87
// ("cv::ximgproc::fourierDescriptor", vec![(pred!(mut, ["src", "dst", "nbElt", "nbFD"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
pub fn cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, nb_elt: i32, nb_fd: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::getDisparityVis(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:204
// ("cv::ximgproc::getDisparityVis", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getDisparityVis(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:204
// ("cv::ximgproc::getDisparityVis", vec![(pred!(mut, ["src", "dst", "scale"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(src: *const c_void, dst: *const c_void, scale: f64, ocvrs_return: *mut Result<()>);
// cv::ximgproc::guidedFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:180
// ("cv::ximgproc::guidedFilter", vec![(pred!(mut, ["guide", "src", "dst", "radius", "eps"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
pub fn cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(guide: *const c_void, src: *const c_void, dst: *const c_void, radius: i32, eps: f64, ocvrs_return: *mut Result<()>);
// guidedFilter(InputArray, InputArray, OutputArray, int, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:180
// ("cv::ximgproc::guidedFilter", vec![(pred!(mut, ["guide", "src", "dst", "radius", "eps", "dDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "int"]), _)]),
pub fn cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int(guide: *const c_void, src: *const c_void, dst: *const c_void, radius: i32, eps: f64, d_depth: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::jointBilateralFilter(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:317
// ("cv::ximgproc::jointBilateralFilter", vec![(pred!(mut, ["joint", "src", "dst", "d", "sigmaColor", "sigmaSpace"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
pub fn cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(joint: *const c_void, src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, ocvrs_return: *mut Result<()>);
// jointBilateralFilter(InputArray, InputArray, OutputArray, int, double, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:317
// ("cv::ximgproc::jointBilateralFilter", vec![(pred!(mut, ["joint", "src", "dst", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int"]), _)]),
pub fn cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(joint: *const c_void, src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::l0Smooth(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:522
// ("cv::ximgproc::l0Smooth", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// l0Smooth(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:522
// ("cv::ximgproc::l0Smooth", vec![(pred!(mut, ["src", "dst", "lambda", "kappa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
pub fn cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, lambda: f64, kappa: f64, ocvrs_return: *mut Result<()>);
// cv::ximgproc::niBlackThreshold(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc.hpp:150
// ("cv::ximgproc::niBlackThreshold", vec![(pred!(mut, ["_src", "_dst", "maxValue", "type", "blockSize", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "double"]), _)]),
pub fn cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double(_src: *const c_void, _dst: *const c_void, max_value: f64, typ: i32, block_size: i32, k: f64, ocvrs_return: *mut Result<()>);
// niBlackThreshold(InputArray, OutputArray, double, int, int, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc.hpp:150
// ("cv::ximgproc::niBlackThreshold", vec![(pred!(mut, ["_src", "_dst", "maxValue", "type", "blockSize", "k", "binarizationMethod"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "double", "int"]), _)]),
pub fn cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double_int(_src: *const c_void, _dst: *const c_void, max_value: f64, typ: i32, block_size: i32, k: f64, binarization_method: i32, ocvrs_return: *mut Result<()>);
// readGT(String, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:164
// ("cv::ximgproc::readGT", vec![(pred!(mut, ["src_path", "dst"], ["cv::String", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_readGT_String_const__OutputArrayR(src_path: *const c_char, dst: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::ximgproc::rollingGuidanceFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:373
// ("cv::ximgproc::rollingGuidanceFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// rollingGuidanceFilter(InputArray, OutputArray, int, double, double, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:373
// ("cv::ximgproc::rollingGuidanceFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace", "numOfIter", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int", "int"]), _)]),
pub fn cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, num_of_iter: i32, border_type: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::createGraphSegmentation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:69
// ("cv::ximgproc::segmentation::createGraphSegmentation", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createGraphSegmentation(ocvrs_return: *mut Result<*mut c_void>);
// createGraphSegmentation(double, float, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:69
// ("cv::ximgproc::segmentation::createGraphSegmentation", vec![(pred!(mut, ["sigma", "k", "min_size"], ["double", "float", "int"]), _)]),
pub fn cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(sigma: f64, k: f32, min_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:244
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentation", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentation(ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyColor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:104
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyColor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor(ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyFill()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:131
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyFill", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill(ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyMultiple()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:149
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple(ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:154
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG(s1: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:160
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(s1: *mut c_void, s2: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:168
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2", "s3"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(s1: *mut c_void, s2: *mut c_void, s3: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:176
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2", "s3", "s4"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(s1: *mut c_void, s2: *mut c_void, s3: *mut c_void, s4: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategySize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:113
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategySize", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize(ocvrs_return: *mut Result<*mut c_void>);
// createSelectiveSearchSegmentationStrategyTexture()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:122
// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyTexture", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture(ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::thinning(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc.hpp:162
// ("cv::ximgproc::thinning", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// thinning(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc.hpp:162
// ("cv::ximgproc::thinning", vec![(pred!(mut, ["src", "dst", "thinningType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, thinning_type: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::transformFD(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:97
// ("cv::ximgproc::transformFD", vec![(pred!(mut, ["src", "t", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, t: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// transformFD(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:97
// ("cv::ximgproc::transformFD", vec![(pred!(mut, ["src", "t", "dst", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(src: *const c_void, t: *const c_void, dst: *const c_void, fd_contour: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::weightedMedianFilter(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/weighted_median_filter.hpp:90
// ("cv::ximgproc::weightedMedianFilter", vec![(pred!(mut, ["joint", "src", "dst", "r"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(joint: *const c_void, src: *const c_void, dst: *const c_void, r: i32, ocvrs_return: *mut Result<()>);
// weightedMedianFilter(InputArray, InputArray, OutputArray, int, double, int, InputArray)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/weighted_median_filter.hpp:90
// ("cv::ximgproc::weightedMedianFilter", vec![(pred!(mut, ["joint", "src", "dst", "r", "sigma", "weightType", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_const__InputArrayR(joint: *const c_void, src: *const c_void, dst: *const c_void, r: i32, sigma: f64, weight_type: i32, mask: *const c_void, ocvrs_return: *mut Result<()>);
// filter(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:214
// ("cv::ximgproc::AdaptiveManifoldFilter::filter", vec![(pred!(mut, ["src", "dst", "joint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, joint: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::AdaptiveManifoldFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:214
// ("cv::ximgproc::AdaptiveManifoldFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:216
// ("cv::ximgproc::AdaptiveManifoldFilter::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:218
// ("cv::ximgproc::AdaptiveManifoldFilter::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_create(ocvrs_return: *mut Result<*mut c_void>);
// getSigmaS()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:221
// ("cv::ximgproc::AdaptiveManifoldFilter::getSigmaS", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSigmaS(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:223
// ("cv::ximgproc::AdaptiveManifoldFilter::setSigmaS", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSigmaR()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:225
// ("cv::ximgproc::AdaptiveManifoldFilter::getSigmaR", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSigmaR(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:227
// ("cv::ximgproc::AdaptiveManifoldFilter::setSigmaR", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTreeHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:229
// ("cv::ximgproc::AdaptiveManifoldFilter::getTreeHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTreeHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:231
// ("cv::ximgproc::AdaptiveManifoldFilter::setTreeHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getPCAIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:233
// ("cv::ximgproc::AdaptiveManifoldFilter::getPCAIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPCAIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:235
// ("cv::ximgproc::AdaptiveManifoldFilter::setPCAIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getAdjustOutliers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:237
// ("cv::ximgproc::AdaptiveManifoldFilter::getAdjustOutliers", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setAdjustOutliers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:239
// ("cv::ximgproc::AdaptiveManifoldFilter::setAdjustOutliers", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUseRNG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:241
// ("cv::ximgproc::AdaptiveManifoldFilter::getUseRNG", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseRNG(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:243
// ("cv::ximgproc::AdaptiveManifoldFilter::setUseRNG", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::AdaptiveManifoldFilter::to_Algorithm() generated
// ("cv::ximgproc::AdaptiveManifoldFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::AdaptiveManifoldFilter::delete() generated
// ("cv::ximgproc::AdaptiveManifoldFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_AdaptiveManifoldFilter_delete(instance: *mut c_void);
// ContourFitting(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:38
// ("cv::ximgproc::ContourFitting::ContourFitting", vec![(pred!(mut, ["ctr", "fd"], ["int", "int"]), _)]),
pub fn cv_ximgproc_ContourFitting_ContourFitting_int_int(ctr: i32, fd: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::ContourFitting::ContourFitting() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:38
// ("cv::ximgproc::ContourFitting::ContourFitting", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_ContourFitting_ContourFitting(ocvrs_return: *mut Result<*mut c_void>);
// estimateTransformation(InputArray, InputArray, OutputArray, double *, bool)(InputArray, InputArray, OutputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:47
// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*", "bool"]), _)]),
pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, dist: *mut f64, fd_contour: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::ContourFitting::estimateTransformation(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:47
// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, ocvrs_return: *mut Result<()>);
// estimateTransformation(InputArray, InputArray, OutputArray, double &, bool)(InputArray, InputArray, OutputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:56
// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*", "bool"]), _)]),
pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, dist: *mut f64, fd_contour: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::ContourFitting::estimateTransformation(InputArray, InputArray, OutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:56
// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*"]), _)]),
pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, dist: *mut f64, ocvrs_return: *mut Result<()>);
// setCtrSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:61
// ("cv::ximgproc::ContourFitting::setCtrSize", vec![(pred!(mut, ["n"], ["int"]), _)]),
pub fn cv_ximgproc_ContourFitting_setCtrSize_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result<()>);
// setFDSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:66
// ("cv::ximgproc::ContourFitting::setFDSize", vec![(pred!(mut, ["n"], ["int"]), _)]),
pub fn cv_ximgproc_ContourFitting_setFDSize_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result<()>);
// getCtrSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:70
// ("cv::ximgproc::ContourFitting::getCtrSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_ContourFitting_getCtrSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// getFDSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fourier_descriptors.hpp:74
// ("cv::ximgproc::ContourFitting::getFDSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_ContourFitting_getFDSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::ximgproc::ContourFitting::to_Algorithm() generated
// ("cv::ximgproc::ContourFitting::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_ContourFitting_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::ContourFitting::delete() generated
// ("cv::ximgproc::ContourFitting::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_ContourFitting_delete(instance: *mut c_void);
// filter(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:79
// ("cv::ximgproc::DTFilter::filter", vec![(pred!(mut, ["src", "dst", "dDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, src: *const c_void, dst: *const c_void, d_depth: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::DTFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:79
// ("cv::ximgproc::DTFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::DTFilter::to_Algorithm() generated
// ("cv::ximgproc::DTFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DTFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::DTFilter::delete() generated
// ("cv::ximgproc::DTFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DTFilter_delete(instance: *mut c_void);
// filter(InputArray, InputArray, OutputArray, InputArray, Rect, InputArray)(InputArray, InputArray, OutputArray, InputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:75
// ("cv::ximgproc::DisparityFilter::filter", vec![(pred!(mut, ["disparity_map_left", "left_view", "filtered_disparity_map", "disparity_map_right", "ROI", "right_view"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Rect", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(instance: *mut c_void, disparity_map_left: *const c_void, left_view: *const c_void, filtered_disparity_map: *const c_void, disparity_map_right: *const c_void, roi: *const core::Rect, right_view: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::DisparityFilter::filter(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:75
// ("cv::ximgproc::DisparityFilter::filter", vec![(pred!(mut, ["disparity_map_left", "left_view", "filtered_disparity_map"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, disparity_map_left: *const c_void, left_view: *const c_void, filtered_disparity_map: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::DisparityFilter::to_DisparityWLSFilter() generated
// ("cv::ximgproc::DisparityFilter::to_DisparityWLSFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityFilter_to_DisparityWLSFilter(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::DisparityFilter::to_Algorithm() generated
// ("cv::ximgproc::DisparityFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::DisparityFilter::delete() generated
// ("cv::ximgproc::DisparityFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityFilter_delete(instance: *mut c_void);
// getLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:90
// ("cv::ximgproc::DisparityWLSFilter::getLambda", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getLambda(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:92
// ("cv::ximgproc::DisparityWLSFilter::setLambda", vec![(pred!(mut, ["_lambda"], ["double"]), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_setLambda_double(instance: *mut c_void, _lambda: f64, ocvrs_return: *mut Result<()>);
// getSigmaColor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:97
// ("cv::ximgproc::DisparityWLSFilter::getSigmaColor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getSigmaColor(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// setSigmaColor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:99
// ("cv::ximgproc::DisparityWLSFilter::setSigmaColor", vec![(pred!(mut, ["_sigma_color"], ["double"]), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(instance: *mut c_void, _sigma_color: f64, ocvrs_return: *mut Result<()>);
// getLRCthresh()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:106
// ("cv::ximgproc::DisparityWLSFilter::getLRCthresh", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getLRCthresh(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setLRCthresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:108
// ("cv::ximgproc::DisparityWLSFilter::setLRCthresh", vec![(pred!(mut, ["_LRC_thresh"], ["int"]), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(instance: *mut c_void, _lrc_thresh: i32, ocvrs_return: *mut Result<()>);
// getDepthDiscontinuityRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:112
// ("cv::ximgproc::DisparityWLSFilter::getDepthDiscontinuityRadius", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setDepthDiscontinuityRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:114
// ("cv::ximgproc::DisparityWLSFilter::setDepthDiscontinuityRadius", vec![(pred!(mut, ["_disc_radius"], ["int"]), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(instance: *mut c_void, _disc_radius: i32, ocvrs_return: *mut Result<()>);
// getConfidenceMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:119
// ("cv::ximgproc::DisparityWLSFilter::getConfidenceMap", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getConfidenceMap(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/disparity_filter.hpp:122
// ("cv::ximgproc::DisparityWLSFilter::getROI", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_getROI(instance: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::ximgproc::DisparityWLSFilter::to_Algorithm() generated
// ("cv::ximgproc::DisparityWLSFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::DisparityWLSFilter::to_DisparityFilter() generated
// ("cv::ximgproc::DisparityWLSFilter::to_DisparityFilter", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_to_DisparityFilter(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::DisparityWLSFilter::delete() generated
// ("cv::ximgproc::DisparityWLSFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_DisparityWLSFilter_delete(instance: *mut c_void);
// setK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:84
// ("cv::ximgproc::EdgeAwareInterpolator::setK", vec![(pred!(mut, ["_k"], ["int"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setK_int(instance: *mut c_void, _k: i32, ocvrs_return: *mut Result<()>);
// getK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:86
// ("cv::ximgproc::EdgeAwareInterpolator::getK", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getK(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:92
// ("cv::ximgproc::EdgeAwareInterpolator::setSigma", vec![(pred!(mut, ["_sigma"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setSigma_float(instance: *mut c_void, _sigma: f32, ocvrs_return: *mut Result<()>);
// getSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:94
// ("cv::ximgproc::EdgeAwareInterpolator::getSigma", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getSigma(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:99
// ("cv::ximgproc::EdgeAwareInterpolator::setLambda", vec![(pred!(mut, ["_lambda"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setLambda_float(instance: *mut c_void, _lambda: f32, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:101
// ("cv::ximgproc::EdgeAwareInterpolator::getLambda", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getLambda(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setUsePostProcessing(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:106
// ("cv::ximgproc::EdgeAwareInterpolator::setUsePostProcessing", vec![(pred!(mut, ["_use_post_proc"], ["bool"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(instance: *mut c_void, _use_post_proc: bool, ocvrs_return: *mut Result<()>);
// getUsePostProcessing()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:108
// ("cv::ximgproc::EdgeAwareInterpolator::getUsePostProcessing", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// setFGSLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:112
// ("cv::ximgproc::EdgeAwareInterpolator::setFGSLambda", vec![(pred!(mut, ["_lambda"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(instance: *mut c_void, _lambda: f32, ocvrs_return: *mut Result<()>);
// getFGSLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:114
// ("cv::ximgproc::EdgeAwareInterpolator::getFGSLambda", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setFGSSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:117
// ("cv::ximgproc::EdgeAwareInterpolator::setFGSSigma", vec![(pred!(mut, ["_sigma"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(instance: *mut c_void, _sigma: f32, ocvrs_return: *mut Result<()>);
// getFGSSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:119
// ("cv::ximgproc::EdgeAwareInterpolator::getFGSSigma", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// cv::ximgproc::EdgeAwareInterpolator::to_Algorithm() generated
// ("cv::ximgproc::EdgeAwareInterpolator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::EdgeAwareInterpolator::to_SparseMatchInterpolator() generated
// ("cv::ximgproc::EdgeAwareInterpolator::to_SparseMatchInterpolator", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_to_SparseMatchInterpolator(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::EdgeAwareInterpolator::delete() generated
// ("cv::ximgproc::EdgeAwareInterpolator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeAwareInterpolator_delete(instance: *mut c_void);
// getBoundingBoxes(InputArray, InputArray, std::vector<Rect> &, OutputArray)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:79
// ("cv::ximgproc::EdgeBoxes::getBoundingBoxes", vec![(pred!(mut, ["edge_map", "orientation_map", "boxes", "scores"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::Rect>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR_const__OutputArrayR(instance: *mut c_void, edge_map: *const c_void, orientation_map: *const c_void, boxes: *mut c_void, scores: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::EdgeBoxes::getBoundingBoxes(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:79
// ("cv::ximgproc::EdgeBoxes::getBoundingBoxes", vec![(pred!(mut, ["edge_map", "orientation_map", "boxes"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR(instance: *mut c_void, edge_map: *const c_void, orientation_map: *const c_void, boxes: *mut c_void, ocvrs_return: *mut Result<()>);
// getAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:83
// ("cv::ximgproc::EdgeBoxes::getAlpha", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:86
// ("cv::ximgproc::EdgeBoxes::setAlpha", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setAlpha_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getBeta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:90
// ("cv::ximgproc::EdgeBoxes::getBeta", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getBeta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setBeta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:93
// ("cv::ximgproc::EdgeBoxes::setBeta", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setBeta_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getEta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:97
// ("cv::ximgproc::EdgeBoxes::getEta", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getEta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:100
// ("cv::ximgproc::EdgeBoxes::setEta", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setEta_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMinScore()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:104
// ("cv::ximgproc::EdgeBoxes::getMinScore", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getMinScore_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinScore(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:107
// ("cv::ximgproc::EdgeBoxes::setMinScore", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setMinScore_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMaxBoxes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:111
// ("cv::ximgproc::EdgeBoxes::getMaxBoxes", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getMaxBoxes_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxBoxes(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:114
// ("cv::ximgproc::EdgeBoxes::setMaxBoxes", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setMaxBoxes_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getEdgeMinMag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:118
// ("cv::ximgproc::EdgeBoxes::getEdgeMinMag", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEdgeMinMag(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:121
// ("cv::ximgproc::EdgeBoxes::setEdgeMinMag", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getEdgeMergeThr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:125
// ("cv::ximgproc::EdgeBoxes::getEdgeMergeThr", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setEdgeMergeThr(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:128
// ("cv::ximgproc::EdgeBoxes::setEdgeMergeThr", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getClusterMinMag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:132
// ("cv::ximgproc::EdgeBoxes::getClusterMinMag", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getClusterMinMag_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setClusterMinMag(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:135
// ("cv::ximgproc::EdgeBoxes::setClusterMinMag", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setClusterMinMag_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMaxAspectRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:139
// ("cv::ximgproc::EdgeBoxes::getMaxAspectRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMaxAspectRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:142
// ("cv::ximgproc::EdgeBoxes::setMaxAspectRatio", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getMinBoxArea()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:146
// ("cv::ximgproc::EdgeBoxes::getMinBoxArea", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getMinBoxArea_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMinBoxArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:149
// ("cv::ximgproc::EdgeBoxes::setMinBoxArea", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setMinBoxArea_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:153
// ("cv::ximgproc::EdgeBoxes::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:156
// ("cv::ximgproc::EdgeBoxes::setGamma", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setGamma_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// getKappa()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:160
// ("cv::ximgproc::EdgeBoxes::getKappa", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_getKappa_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setKappa(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edgeboxes.hpp:163
// ("cv::ximgproc::EdgeBoxes::setKappa", vec![(pred!(mut, ["value"], ["float"]), _)]),
pub fn cv_ximgproc_EdgeBoxes_setKappa_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::EdgeBoxes::to_Algorithm() generated
// ("cv::ximgproc::EdgeBoxes::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::EdgeBoxes::delete() generated
// ("cv::ximgproc::EdgeBoxes::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeBoxes_delete(instance: *mut c_void);
// detectEdges(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:77
// ("cv::ximgproc::EdgeDrawing::detectEdges", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<()>);
// getEdgeImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:83
// ("cv::ximgproc::EdgeDrawing::getEdgeImage", vec![(pred!(mut, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(instance: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getGradientImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:89
// ("cv::ximgproc::EdgeDrawing::getGradientImage", vec![(pred!(mut, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(instance: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// getSegments()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:93
// ("cv::ximgproc::EdgeDrawing::getSegments", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeDrawing_getSegments(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// detectLines(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:100
// ("cv::ximgproc::EdgeDrawing::detectLines", vec![(pred!(mut, ["lines"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(instance: *mut c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// detectEllipses(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:107
// ("cv::ximgproc::EdgeDrawing::detectEllipses", vec![(pred!(mut, ["ellipses"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(instance: *mut c_void, ellipses: *const c_void, ocvrs_return: *mut Result<()>);
// setParams(const EdgeDrawing::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:116
// ("cv::ximgproc::EdgeDrawing::setParams", vec![(pred!(mut, ["parameters"], ["const cv::ximgproc::EdgeDrawing::Params*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(instance: *mut c_void, parameters: *const crate::ximgproc::EdgeDrawing_Params, ocvrs_return: *mut Result<()>);
// cv::ximgproc::EdgeDrawing::params() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:109
// ("cv::ximgproc::EdgeDrawing::params", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_EdgeDrawing_propParams_const(instance: *const c_void, ocvrs_return: *mut crate::ximgproc::EdgeDrawing_Params);
// cv::ximgproc::EdgeDrawing::setParams(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:109
// ("cv::ximgproc::EdgeDrawing::setParams", vec![(pred!(mut, ["val"], ["const cv::ximgproc::EdgeDrawing::Params"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_propParams_const_Params(instance: *mut c_void, val: *const crate::ximgproc::EdgeDrawing_Params);
// cv::ximgproc::EdgeDrawing::to_Algorithm() generated
// ("cv::ximgproc::EdgeDrawing::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeDrawing_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::EdgeDrawing::delete() generated
// ("cv::ximgproc::EdgeDrawing::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeDrawing_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:35
// ("cv::ximgproc::EdgeDrawing::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_EdgeDrawing_Params_Params(ocvrs_return: *mut Result<crate::ximgproc::EdgeDrawing_Params>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:69
// ("cv::ximgproc::EdgeDrawing::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(instance: *const crate::ximgproc::EdgeDrawing_Params, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_drawing.hpp:70
// ("cv::ximgproc::EdgeDrawing::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(instance: *const crate::ximgproc::EdgeDrawing_Params, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// filter(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:395
// ("cv::ximgproc::FastBilateralSolverFilter::filter", vec![(pred!(mut, ["src", "confidence", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, confidence: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::FastBilateralSolverFilter::to_Algorithm() generated
// ("cv::ximgproc::FastBilateralSolverFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastBilateralSolverFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::FastBilateralSolverFilter::delete() generated
// ("cv::ximgproc::FastBilateralSolverFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastBilateralSolverFilter_delete(instance: *mut c_void);
// filter(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:465
// ("cv::ximgproc::FastGlobalSmootherFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::FastGlobalSmootherFilter::to_Algorithm() generated
// ("cv::ximgproc::FastGlobalSmootherFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastGlobalSmootherFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::FastGlobalSmootherFilter::delete() generated
// ("cv::ximgproc::FastGlobalSmootherFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastGlobalSmootherFilter_delete(instance: *mut c_void);
// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_line_detector.hpp:44
// ("cv::ximgproc::FastLineDetector::detect", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// drawSegments(InputOutputArray, InputArray, bool, Scalar, int)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_line_detector.hpp:54
// ("cv::ximgproc::FastLineDetector::drawSegments", vec![(pred!(mut, ["image", "lines", "draw_arrow", "linecolor", "linethickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "cv::Scalar", "int"]), _)]),
pub fn cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(instance: *mut c_void, image: *const c_void, lines: *const c_void, draw_arrow: bool, linecolor: *const core::Scalar, linethickness: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::FastLineDetector::drawSegments(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/fast_line_detector.hpp:54
// ("cv::ximgproc::FastLineDetector::drawSegments", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::FastLineDetector::to_Algorithm() generated
// ("cv::ximgproc::FastLineDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastLineDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::FastLineDetector::delete() generated
// ("cv::ximgproc::FastLineDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_FastLineDetector_delete(instance: *mut c_void);
// filter(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:143
// ("cv::ximgproc::GuidedFilter::filter", vec![(pred!(mut, ["src", "dst", "dDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, src: *const c_void, dst: *const c_void, d_depth: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::GuidedFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/edge_filter.hpp:143
// ("cv::ximgproc::GuidedFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::GuidedFilter::to_Algorithm() generated
// ("cv::ximgproc::GuidedFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_GuidedFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::GuidedFilter::delete() generated
// ("cv::ximgproc::GuidedFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_GuidedFilter_delete(instance: *mut c_void);
// getFeatures(const Mat &, Mat &, const int, const int, const int, const int, const int)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:83
// ("cv::ximgproc::RFFeatureGetter::getFeatures", vec![(pred!(const, ["src", "features", "gnrmRad", "gsmthRad", "shrink", "outNum", "gradNum"], ["const cv::Mat*", "cv::Mat*", "const int", "const int", "const int", "const int", "const int"]), _)]),
pub fn cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(instance: *const c_void, src: *const c_void, features: *mut c_void, gnrm_rad: i32, gsmth_rad: i32, shrink: i32, out_num: i32, grad_num: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::RFFeatureGetter::to_Algorithm() generated
// ("cv::ximgproc::RFFeatureGetter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_RFFeatureGetter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::RFFeatureGetter::delete() generated
// ("cv::ximgproc::RFFeatureGetter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_RFFeatureGetter_delete(instance: *mut c_void);
// create(int, int, int, int, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/ridgefilter.hpp:42
// ("cv::ximgproc::RidgeDetectionFilter::create", vec![(pred!(mut, ["ddepth", "dx", "dy", "ksize", "out_dtype", "scale", "delta", "borderType"], ["int", "int", "int", "int", "int", "double", "double", "int"]), _)]),
pub fn cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(ddepth: i32, dx: i32, dy: i32, ksize: i32, out_dtype: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ximgproc::RidgeDetectionFilter::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/ridgefilter.hpp:42
// ("cv::ximgproc::RidgeDetectionFilter::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_RidgeDetectionFilter_create(ocvrs_return: *mut Result<*mut c_void>);
// getRidgeFilteredImage(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/ridgefilter.hpp:48
// ("cv::ximgproc::RidgeDetectionFilter::getRidgeFilteredImage", vec![(pred!(mut, ["_img", "out"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, _img: *const c_void, out: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::RidgeDetectionFilter::to_Algorithm() generated
// ("cv::ximgproc::RidgeDetectionFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_RidgeDetectionFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::RidgeDetectionFilter::delete() generated
// ("cv::ximgproc::RidgeDetectionFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_RidgeDetectionFilter_delete(instance: *mut c_void);
// interpolate(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/sparse_match_interpolator.hpp:69
// ("cv::ximgproc::SparseMatchInterpolator::interpolate", vec![(pred!(mut, ["from_image", "from_points", "to_image", "to_points", "dense_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, from_image: *const c_void, from_points: *const c_void, to_image: *const c_void, to_points: *const c_void, dense_flow: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SparseMatchInterpolator::to_EdgeAwareInterpolator() generated
// ("cv::ximgproc::SparseMatchInterpolator::to_EdgeAwareInterpolator", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SparseMatchInterpolator_to_EdgeAwareInterpolator(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::SparseMatchInterpolator::to_Algorithm() generated
// ("cv::ximgproc::SparseMatchInterpolator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SparseMatchInterpolator_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::SparseMatchInterpolator::delete() generated
// ("cv::ximgproc::SparseMatchInterpolator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SparseMatchInterpolator_delete(instance: *mut c_void);
// detectEdges(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:109
// ("cv::ximgproc::StructuredEdgeDetection::detectEdges", vec![(pred!(const, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// computeOrientation(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:116
// ("cv::ximgproc::StructuredEdgeDetection::computeOrientation", vec![(pred!(const, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// edgesNms(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:129
// ("cv::ximgproc::StructuredEdgeDetection::edgesNms", vec![(pred!(const, ["edge_image", "orientation_image", "dst", "r", "s", "m", "isParallel"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool"]), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(instance: *const c_void, edge_image: *const c_void, orientation_image: *const c_void, dst: *const c_void, r: i32, s: i32, m: f32, is_parallel: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::StructuredEdgeDetection::edgesNms(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/structured_edge_detection.hpp:129
// ("cv::ximgproc::StructuredEdgeDetection::edgesNms", vec![(pred!(const, ["edge_image", "orientation_image", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, edge_image: *const c_void, orientation_image: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::StructuredEdgeDetection::to_Algorithm() generated
// ("cv::ximgproc::StructuredEdgeDetection::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::StructuredEdgeDetection::delete() generated
// ("cv::ximgproc::StructuredEdgeDetection::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_StructuredEdgeDetection_delete(instance: *mut c_void);
// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:78
// ("cv::ximgproc::SuperpixelLSC::getNumberOfSuperpixels", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// iterate(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:94
// ("cv::ximgproc::SuperpixelLSC::iterate", vec![(pred!(mut, ["num_iterations"], ["int"]), _)]),
pub fn cv_ximgproc_SuperpixelLSC_iterate_int(instance: *mut c_void, num_iterations: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelLSC::iterate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:94
// ("cv::ximgproc::SuperpixelLSC::iterate", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelLSC_iterate(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:106
// ("cv::ximgproc::SuperpixelLSC::getLabels", vec![(pred!(const, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(instance: *const c_void, labels_out: *const c_void, ocvrs_return: *mut Result<()>);
// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:118
// ("cv::ximgproc::SuperpixelLSC::getLabelContourMask", vec![(pred!(const, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(instance: *const c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelLSC::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:118
// ("cv::ximgproc::SuperpixelLSC::getLabelContourMask", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// enforceLabelConnectivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:129
// ("cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity", vec![(pred!(mut, ["min_element_size"], ["int"]), _)]),
pub fn cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(instance: *mut c_void, min_element_size: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/lsc.hpp:129
// ("cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelLSC::to_Algorithm() generated
// ("cv::ximgproc::SuperpixelLSC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelLSC_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::SuperpixelLSC::delete() generated
// ("cv::ximgproc::SuperpixelLSC::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelLSC_delete(instance: *mut c_void);
// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:75
// ("cv::ximgproc::SuperpixelSEEDS::getNumberOfSuperpixels", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// iterate(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:99
// ("cv::ximgproc::SuperpixelSEEDS::iterate", vec![(pred!(mut, ["img", "num_iterations"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(instance: *mut c_void, img: *const c_void, num_iterations: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSEEDS::iterate(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:99
// ("cv::ximgproc::SuperpixelSEEDS::iterate", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<()>);
// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:111
// ("cv::ximgproc::SuperpixelSEEDS::getLabels", vec![(pred!(mut, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(instance: *mut c_void, labels_out: *const c_void, ocvrs_return: *mut Result<()>);
// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:139
// ("cv::ximgproc::SuperpixelSEEDS::getLabelContourMask", vec![(pred!(mut, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSEEDS::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/seeds.hpp:139
// ("cv::ximgproc::SuperpixelSEEDS::getLabelContourMask", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSEEDS::to_Algorithm() generated
// ("cv::ximgproc::SuperpixelSEEDS::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::SuperpixelSEEDS::delete() generated
// ("cv::ximgproc::SuperpixelSEEDS::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSEEDS_delete(instance: *mut c_void);
// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:86
// ("cv::ximgproc::SuperpixelSLIC::getNumberOfSuperpixels", vec![(pred!(const, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// iterate(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:102
// ("cv::ximgproc::SuperpixelSLIC::iterate", vec![(pred!(mut, ["num_iterations"], ["int"]), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_iterate_int(instance: *mut c_void, num_iterations: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSLIC::iterate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:102
// ("cv::ximgproc::SuperpixelSLIC::iterate", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_iterate(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:114
// ("cv::ximgproc::SuperpixelSLIC::getLabels", vec![(pred!(const, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(instance: *const c_void, labels_out: *const c_void, ocvrs_return: *mut Result<()>);
// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:126
// ("cv::ximgproc::SuperpixelSLIC::getLabelContourMask", vec![(pred!(const, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(instance: *const c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSLIC::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:126
// ("cv::ximgproc::SuperpixelSLIC::getLabelContourMask", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// enforceLabelConnectivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:137
// ("cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity", vec![(pred!(mut, ["min_element_size"], ["int"]), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(instance: *mut c_void, min_element_size: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/slic.hpp:137
// ("cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::SuperpixelSLIC::to_Algorithm() generated
// ("cv::ximgproc::SuperpixelSLIC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::SuperpixelSLIC::delete() generated
// ("cv::ximgproc::SuperpixelSLIC::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_SuperpixelSLIC_delete(instance: *mut c_void);
// processImage(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:52
// ("cv::ximgproc::segmentation::GraphSegmentation::processImage", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:54
// ("cv::ximgproc::segmentation::GraphSegmentation::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(instance: *mut c_void, sigma: f64, ocvrs_return: *mut Result<()>);
// getSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:55
// ("cv::ximgproc::segmentation::GraphSegmentation::getSigma", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_getSigma(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
// setK(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:57
// ("cv::ximgproc::segmentation::GraphSegmentation::setK", vec![(pred!(mut, ["k"], ["float"]), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_setK_float(instance: *mut c_void, k: f32, ocvrs_return: *mut Result<()>);
// getK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:58
// ("cv::ximgproc::segmentation::GraphSegmentation::getK", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_getK(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:60
// ("cv::ximgproc::segmentation::GraphSegmentation::setMinSize", vec![(pred!(mut, ["min_size"], ["int"]), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(instance: *mut c_void, min_size: i32, ocvrs_return: *mut Result<()>);
// getMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:61
// ("cv::ximgproc::segmentation::GraphSegmentation::getMinSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_getMinSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::ximgproc::segmentation::GraphSegmentation::to_Algorithm() generated
// ("cv::ximgproc::segmentation::GraphSegmentation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::GraphSegmentation::delete() generated
// ("cv::ximgproc::segmentation::GraphSegmentation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_GraphSegmentation_delete(instance: *mut c_void);
// setBaseImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:187
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::setBaseImage", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<()>);
// switchToSingleStrategy(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:193
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy", vec![(pred!(mut, ["k", "sigma"], ["int", "float"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(instance: *mut c_void, k: i32, sigma: f32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:193
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// switchToSelectiveSearchFast(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:200
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast", vec![(pred!(mut, ["base_k", "inc_k", "sigma"], ["int", "int", "float"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(instance: *mut c_void, base_k: i32, inc_k: i32, sigma: f32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:200
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// switchToSelectiveSearchQuality(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:207
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality", vec![(pred!(mut, ["base_k", "inc_k", "sigma"], ["int", "int", "float"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(instance: *mut c_void, base_k: i32, inc_k: i32, sigma: f32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:207
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:212
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addImage", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<()>);
// clearImages()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:216
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearImages", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addGraphSegmentation(Ptr<GraphSegmentation>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:221
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addGraphSegmentation", vec![(pred!(mut, ["g"], ["cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_PtrLGraphSegmentationG(instance: *mut c_void, g: *mut c_void, ocvrs_return: *mut Result<()>);
// clearGraphSegmentations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:225
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearGraphSegmentations", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addStrategy(Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:230
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addStrategy", vec![(pred!(mut, ["s"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_PtrLSelectiveSearchSegmentationStrategyG(instance: *mut c_void, s: *mut c_void, ocvrs_return: *mut Result<()>);
// clearStrategies()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:234
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearStrategies", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// process(std::vector<Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:239
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::process", vec![(pred!(mut, ["rects"], ["std::vector<cv::Rect>*"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vectorLRectGR(instance: *mut c_void, rects: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentation::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentation::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_delete(instance: *mut c_void);
// setImage(InputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:82
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage", vec![(pred!(mut, ["img", "regions", "sizes", "image_id"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, img: *const c_void, regions: *const c_void, sizes: *const c_void, image_id: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:82
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage", vec![(pred!(mut, ["img", "regions", "sizes"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, img: *const c_void, regions: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<()>);
// get(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:88
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::get", vec![(pred!(mut, ["r1", "r2"], ["int", "int"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(instance: *mut c_void, r1: i32, r2: i32, ocvrs_return: *mut Result<f32>);
// merge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:94
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::merge", vec![(pred!(mut, ["r1", "r2"], ["int", "int"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(instance: *mut c_void, r1: i32, r2: i32, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyColor() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyColor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyColor(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyFill() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyFill", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyFill(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyMultiple() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyMultiple(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategySize() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategySize", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategySize(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyTexture() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyTexture", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyTexture(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_delete(instance: *mut c_void);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_SelectiveSearchSegmentationStrategy() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_SelectiveSearchSegmentationStrategy(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_delete(instance: *mut c_void);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_SelectiveSearchSegmentationStrategy() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_SelectiveSearchSegmentationStrategy(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_delete(instance: *mut c_void);
// addStrategy(Ptr<SelectiveSearchSegmentationStrategy>, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:142
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::addStrategy", vec![(pred!(mut, ["g", "weight"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "float"]), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_PtrLSelectiveSearchSegmentationStrategyG_float(instance: *mut c_void, g: *mut c_void, weight: f32, ocvrs_return: *mut Result<()>);
// clearStrategies()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ximgproc/segmentation.hpp:145
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::clearStrategies", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_SelectiveSearchSegmentationStrategy() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_SelectiveSearchSegmentationStrategy(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_delete(instance: *mut c_void);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_SelectiveSearchSegmentationStrategy() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_SelectiveSearchSegmentationStrategy(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_delete(instance: *mut c_void);
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_Algorithm() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_SelectiveSearchSegmentationStrategy() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_SelectiveSearchSegmentationStrategy(instance: *mut c_void) -> *mut c_void;
// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::delete() generated
// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_delete(instance: *mut c_void);
