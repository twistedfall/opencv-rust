use libc::{c_void, c_char, size_t};
use crate::{core};

// cv::CirclesGridFinderParameters2
#[repr(C)]
pub struct cv_return_value_c_CirclesGridFinderParameters2 {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: super::calib3d::CirclesGridFinderParameters2
}

// CvRNG
#[repr(C)]
pub struct cv_return_value_c_CvRNG {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::CvRNG
}

// cv::DMatch
#[repr(C)]
pub struct cv_return_value_c_DMatch {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::DMatch
}

// cv::KeyPoint
#[repr(C)]
pub struct cv_return_value_c_KeyPoint {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::KeyPoint
}

// cv::Moments
#[repr(C)]
pub struct cv_return_value_c_Moments {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Moments
}

// Point
#[repr(C)]
pub struct cv_return_value_c_Point {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Point
}

// Point2f
#[repr(C)]
pub struct cv_return_value_c_Point2f {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Point2f
}

// Rect
#[repr(C)]
pub struct cv_return_value_c_Rect {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Rect
}

// Scalar
#[repr(C)]
pub struct cv_return_value_c_Scalar {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Scalar
}

// cv::SimpleBlobDetector::Params
#[repr(C)]
pub struct cv_return_value_c_SimpleBlobDetector_Params {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: super::features2d::SimpleBlobDetector_Params
}

// Size
#[repr(C)]
pub struct cv_return_value_c_Size {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Size
}

// Size2f
#[repr(C)]
pub struct cv_return_value_c_Size2f {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Size2f
}

// Vec2d
#[repr(C)]
pub struct cv_return_value_c_Vec2d {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Vec2d
}

// Vec3d
#[repr(C)]
pub struct cv_return_value_c_Vec3d {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: core::Vec3d
}

// cv::dnn::Net
#[repr(C)]
pub struct cv_return_value_c_dnn_Net {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: super::dnn::dnn_Net
}

// bool
#[repr(C)]
pub struct cv_return_value_char {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: bool
}

// String
#[repr(C)]
pub struct cv_return_value_char_X {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: *const c_char
}

// double
#[repr(C)]
pub struct cv_return_value_double {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: f64
}

// float
#[repr(C)]
pub struct cv_return_value_float {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: f32
}

// int
#[repr(C)]
pub struct cv_return_value_int {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: i32
}

// int64
#[repr(C)]
pub struct cv_return_value_int64 {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: i64
}

// int64_t
#[repr(C)]
pub struct cv_return_value_int64_t {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: i64
}

// size_t
#[repr(C)]
pub struct cv_return_value_std_size_t {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: size_t
}

// uchar*
#[repr(C)]
pub struct cv_return_value_unsigned_charX {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: *mut u8
}

// unsigned long long
#[repr(C)]
pub struct cv_return_value_unsigned_long_long {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: u64
}

// ushort
#[repr(C)]
pub struct cv_return_value_unsigned_short {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: u16
}

// void
#[repr(C)]
pub struct cv_return_value_void {
    pub error_code: i32,
    pub error_msg: *const c_char,
}

// std::vector<Mat>
#[repr(C)]
pub struct cv_return_value_void_X {
    pub error_code: i32,
    pub error_msg: *const c_char,
    pub result: *mut c_void
}
extern "C" {
#[doc(hidden)] pub fn cv_core_cvCbrt_float_value(value: f32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_core_cvCheckHardwareSupport_int_feature(feature: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvErrorFromIppStatus_int_ipp_status(ipp_status: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvFastArctan_float_y_float_x(y: f32, x: f32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_core_cvGetErrMode() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvGetErrStatus() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvGetNumThreads() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvGetOptimalDFTSize_int_size0(size0: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvGetThreadNum() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvGetTickCount() -> cv_return_value_int64;
#[doc(hidden)] pub fn cv_core_cvGetTickFrequency() -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cvIplDepth_int_type(_type: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvRNG_int64_seed(seed: i64) -> cv_return_value_c_CvRNG;
#[doc(hidden)] pub fn cv_core_cvRound64_softdouble_a(a: *mut c_void) -> cv_return_value_int64_t;
#[doc(hidden)] pub fn cv_core_cvSetErrMode_int_mode(mode: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvSetErrStatus_int_status(status: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cvSetNumThreads_int_threads(threads: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cvTrunc_softdouble_a(a: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvTrunc_softfloat_a(a: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cvUseOptimized_int_on_off(on_off: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_LUT_Mat_src_Mat_lut_Mat_dst(src: *mut c_void, lut: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mahalanobis_Mat_v1_Mat_v2_Mat_icovar(v1: *mut c_void, v2: *mut c_void, icovar: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_PCABackProject_Mat_data_Mat_mean_Mat_eigenvectors_Mat_result(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, result: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_Mat_eigenvalues_double_retainedVariance(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, eigenvalues: *mut c_void, retained_variance: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_Mat_eigenvalues_int_maxComponents(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, eigenvalues: *mut c_void, max_components: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_double_retainedVariance(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, retained_variance: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_int_maxComponents(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, max_components: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCAProject_Mat_data_Mat_mean_Mat_eigenvectors_Mat_result(data: *mut c_void, mean: *mut c_void, eigenvectors: *mut c_void, result: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PSNR_Mat_src1_Mat_src2(src1: *mut c_void, src2: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_SVBackSubst_Mat_w_Mat_u_Mat_vt_Mat_rhs_Mat_dst(w: *mut c_void, u: *mut c_void, vt: *mut c_void, rhs: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_SVDecomp_Mat_src_Mat_w_Mat_u_Mat_vt_int_flags(src: *mut c_void, w: *mut c_void, u: *mut c_void, vt: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_abs_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_abs_softfloat_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_absdiff_Mat_src1_Mat_src2_Mat_dst(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_addWeighted_Mat_src1_double_alpha_Mat_src2_double_beta_double_gamma_Mat_dst_int_dtype(src1: *mut c_void, alpha: f64, src2: *mut c_void, beta: f64, gamma: f64, dst: *mut c_void, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_add_Mat_src1_Mat_src2_Mat_dst_Mat_mask_int_dtype(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_alignSize_size_t_sz_int_n(sz: size_t, n: i32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_batchDistance_Mat_src1_Mat_src2_Mat_dist_int_dtype_Mat_nidx_int_normType_int_K_Mat_mask_int_update_bool_crosscheck(src1: *mut c_void, src2: *mut c_void, dist: *mut c_void, dtype: i32, nidx: *mut c_void, norm_type: i32, k: i32, mask: *mut c_void, update: i32, crosscheck: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_bitwise_and_Mat_src1_Mat_src2_Mat_dst_Mat_mask(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_bitwise_not_Mat_src_Mat_dst_Mat_mask(src: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_bitwise_or_Mat_src1_Mat_src2_Mat_dst_Mat_mask(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_bitwise_xor_Mat_src1_Mat_src2_Mat_dst_Mat_mask(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_borderInterpolate_int_p_int_len_int_borderType(p: i32, len: i32, border_type: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_calcCovarMatrix_Mat_samples_Mat_covar_Mat_mean_int_flags_int_ctype(samples: *mut c_void, covar: *mut c_void, mean: *mut c_void, flags: i32, ctype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_calcCovarMatrix_Mat_samples_int_nsamples_Mat_covar_Mat_mean_int_flags_int_ctype(samples: *mut c_void, nsamples: i32, covar: *mut c_void, mean: *mut c_void, flags: i32, ctype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_cartToPolar_Mat_x_Mat_y_Mat_magnitude_Mat_angle_bool_angleInDegrees(x: *mut c_void, y: *mut c_void, magnitude: *mut c_void, angle: *mut c_void, angle_in_degrees: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_cbrt_softfloat_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_checkHardwareSupport_int_feature(feature: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_compare_Mat_src1_Mat_src2_Mat_dst_int_cmpop(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, cmpop: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_completeSymm_Mat_m_bool_lowerToUpper(m: *mut c_void, lower_to_upper: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_convertFp16_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_convertScaleAbs_Mat_src_Mat_dst_double_alpha_double_beta(src: *mut c_void, dst: *mut c_void, alpha: f64, beta: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_copyMakeBorder_Mat_src_Mat_dst_int_top_int_bottom_int_left_int_right_int_borderType_Scalar_value(src: *mut c_void, dst: *mut c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_cos_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_countNonZero_Mat_src(src: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_cubeRoot_float_val(val: f32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_core_cv_cv_abs_schar_x(x: i8) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_cv_abs_ushort_x(x: u16) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_dct_Mat_src_Mat_dst_int_flags(src: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_determinant_Mat_mtx(mtx: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_dft_Mat_src_Mat_dst_int_flags_int_nonzeroRows(src: *mut c_void, dst: *mut c_void, flags: i32, nonzero_rows: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_directx_getTypeFromD3DFORMAT_int_iD3DFORMAT(i_d3_dformat: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_directx_getTypeFromDXGI_FORMAT_int_iDXGI_FORMAT(i_dxgi_format: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_divUp_int_a_unsigned_int_b(a: i32, b: u32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_divUp_size_t_a_unsigned_int_b(a: size_t, b: u32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_divide_Mat_src1_Mat_src2_Mat_dst_double_scale_int_dtype(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, scale: f64, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_divide_double_scale_Mat_src2_Mat_dst_int_dtype(scale: f64, src2: *mut c_void, dst: *mut c_void, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_eigenNonSymmetric_Mat_src_Mat_eigenvalues_Mat_eigenvectors(src: *mut c_void, eigenvalues: *mut c_void, eigenvectors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_eigen_Mat_src_Mat_eigenvalues_Mat_eigenvectors(src: *mut c_void, eigenvalues: *mut c_void, eigenvectors: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_exp_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_exp_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_exp_softfloat_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_extractChannel_Mat_src_Mat_dst_int_coi(src: *mut c_void, dst: *mut c_void, coi: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_fastAtan2_float_y_float_x(y: f32, x: f32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_core_cv_findNonZero_Mat_src_Mat_idx(src: *mut c_void, idx: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_flip_Mat_src_Mat_dst_int_flipCode(src: *mut c_void, dst: *mut c_void, flip_code: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_gemm_Mat_src1_Mat_src2_double_alpha_Mat_src3_double_beta_Mat_dst_int_flags(src1: *mut c_void, src2: *mut c_void, alpha: f64, src3: *mut c_void, beta: f64, dst: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_getBuildInformation() -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_getCPUTickCount() -> cv_return_value_int64;
#[doc(hidden)] pub fn cv_core_cv_getElemSize_int_type(_type: i32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_getHardwareFeatureName_int_feature(feature: i32) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_getNumThreads() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getNumberOfCPUs() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getOptimalDFTSize_int_vecsize(vecsize: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getThreadNum() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getTickCount() -> cv_return_value_int64;
#[doc(hidden)] pub fn cv_core_cv_getTickFrequency() -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_getVersionMajor() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getVersionMinor() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getVersionRevision() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_getVersionString() -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_glob_String_pattern_VectorOfString_result_bool_recursive(pattern: *const c_char, result: *mut c_void, recursive: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_haveOpenVX() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_hconcat_Mat_src1_Mat_src2_Mat_dst(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_hconcat_VectorOfMat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_idct_Mat_src_Mat_dst_int_flags(src: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_idft_Mat_src_Mat_dst_int_flags_int_nonzeroRows(src: *mut c_void, dst: *mut c_void, flags: i32, nonzero_rows: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_inRange_Mat_src_Mat_lowerb_Mat_upperb_Mat_dst(src: *mut c_void, lowerb: *mut c_void, upperb: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_insertChannel_Mat_src_Mat_dst_int_coi(src: *mut c_void, dst: *mut c_void, coi: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_instr_resetTrace() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_instr_setFlags_int_modeFlags(mode_flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_instr_setUseInstrumentation_bool_flag(flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_instr_useInstrumentation() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_invert_Mat_src_Mat_dst_int_flags(src: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_ipp_getIppErrorLocation() -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_ipp_getIppFeatures() -> cv_return_value_unsigned_long_long;
#[doc(hidden)] pub fn cv_core_cv_ipp_getIppStatus() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_ipp_getIppVersion() -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_ipp_setUseIPP_NE_bool_flag(flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_ipp_setUseIPP_NotExact_bool_flag(flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_ipp_setUseIPP_bool_flag(flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_ipp_useIPP() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_ipp_useIPP_NE() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_ipp_useIPP_NotExact() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_kmeans_Mat_data_int_K_Mat_bestLabels_TermCriteria_criteria_int_attempts_int_flags_Mat_centers(data: *mut c_void, k: i32, best_labels: *mut c_void, criteria: *mut c_void, attempts: i32, flags: i32, centers: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_log_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_log_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_log_softfloat_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_magnitude_Mat_x_Mat_y_Mat_magnitude(x: *mut c_void, y: *mut c_void, magnitude: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_max_Mat_src1_Mat_src2_Mat_dst(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_max_softdouble_a_softdouble_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_max_softfloat_a_softfloat_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_meanStdDev_Mat_src_Mat_mean_Mat_stddev_Mat_mask(src: *mut c_void, mean: *mut c_void, stddev: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_mean_Mat_src_Mat_mask(src: *mut c_void, mask: *mut c_void) -> cv_return_value_c_Scalar;
#[doc(hidden)] pub fn cv_core_cv_merge_VectorOfMat_mv_Mat_dst(mv: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_min_Mat_src1_Mat_src2_Mat_dst(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_min_softdouble_a_softdouble_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_min_softfloat_a_softfloat_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_mixChannels_VectorOfMat_src_VectorOfMat_dst_VectorOfint_fromTo(src: *mut c_void, dst: *mut c_void, from_to: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_mulAdd_softdouble_a_softdouble_b_softdouble_c(a: *mut c_void, b: *mut c_void, c: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_mulAdd_softfloat_a_softfloat_b_softfloat_c(a: *mut c_void, b: *mut c_void, c: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_mulSpectrums_Mat_a_Mat_b_Mat_c_int_flags_bool_conjB(a: *mut c_void, b: *mut c_void, c: *mut c_void, flags: i32, conj_b: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_mulTransposed_Mat_src_Mat_dst_bool_aTa_Mat_delta_double_scale_int_dtype(src: *mut c_void, dst: *mut c_void, a_ta: bool, delta: *mut c_void, scale: f64, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_multiply_Mat_src1_Mat_src2_Mat_dst_double_scale_int_dtype(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, scale: f64, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_norm_Mat_src1_Mat_src2_int_normType_Mat_mask(src1: *mut c_void, src2: *mut c_void, norm_type: i32, mask: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_norm_Mat_src1_int_normType_Mat_mask(src1: *mut c_void, norm_type: i32, mask: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_normalize_Mat_src_Mat_dst_double_alpha_double_beta_int_norm_type_int_dtype_Mat_mask(src: *mut c_void, dst: *mut c_void, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_parallel_for__Range_range_ParallelLoopBody_body_double_nstripes(range: *mut c_void, body: *mut c_void, nstripes: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_patchNaNs_Mat_a_double_val(a: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_perspectiveTransform_Mat_src_Mat_dst_Mat_m(src: *mut c_void, dst: *mut c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_phase_Mat_x_Mat_y_Mat_angle_bool_angleInDegrees(x: *mut c_void, y: *mut c_void, angle: *mut c_void, angle_in_degrees: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_polarToCart_Mat_magnitude_Mat_angle_Mat_x_Mat_y_bool_angleInDegrees(magnitude: *mut c_void, angle: *mut c_void, x: *mut c_void, y: *mut c_void, angle_in_degrees: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_pow_Mat_src_double_power_Mat_dst(src: *mut c_void, power: f64, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_pow_softdouble_a_softdouble_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_pow_softfloat_a_softfloat_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_randn_Mat_dst_Mat_mean_Mat_stddev(dst: *mut c_void, mean: *mut c_void, stddev: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_randu_Mat_dst_Mat_low_Mat_high(dst: *mut c_void, low: *mut c_void, high: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_reduce_Mat_src_Mat_dst_int_dim_int_rtype_int_dtype(src: *mut c_void, dst: *mut c_void, dim: i32, rtype: i32, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_repeat_Mat_src_int_ny_int_nx(src: *mut c_void, ny: i32, nx: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_repeat_Mat_src_int_ny_int_nx_Mat_dst(src: *mut c_void, ny: i32, nx: i32, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_rotate_Mat_src_Mat_dst_int_rotateCode(src: *mut c_void, dst: *mut c_void, rotate_code: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_roundUp_int_a_unsigned_int_b(a: i32, b: u32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_roundUp_size_t_a_unsigned_int_b(a: size_t, b: u32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_scaleAdd_Mat_src1_double_alpha_Mat_src2_Mat_dst(src1: *mut c_void, alpha: f64, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_setBreakOnError_bool_flag(flag: bool) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_setIdentity_Mat_mtx_Scalar_s(mtx: *mut c_void, s: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_setNumThreads_int_nthreads(nthreads: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_setRNGSeed_int_seed(seed: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_setUseOpenVX_bool_flag(flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_setUseOptimized_bool_onoff(onoff: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_sin_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_solveCubic_Mat_coeffs_Mat_roots(coeffs: *mut c_void, roots: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_solveLP_Mat_Func_Mat_Constr_Mat_z(func: *mut c_void, constr: *mut c_void, z: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_solvePoly_Mat_coeffs_Mat_roots_int_maxIters(coeffs: *mut c_void, roots: *mut c_void, max_iters: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_solve_Mat_src1_Mat_src2_Mat_dst_int_flags(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_sortIdx_Mat_src_Mat_dst_int_flags(src: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_sort_Mat_src_Mat_dst_int_flags(src: *mut c_void, dst: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_split_Mat_m_VectorOfMat_mv(m: *mut c_void, mv: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_split_Mat_src_Mat_mvbegin(src: *mut c_void, mvbegin: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_sqrt_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_sqrt_softdouble_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_sqrt_softfloat_a(a: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_subtract_Mat_src1_Mat_src2_Mat_dst_Mat_mask_int_dtype(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_sum_Mat_src(src: *mut c_void) -> cv_return_value_c_Scalar;
#[doc(hidden)] pub fn cv_core_cv_swap_Mat_a_Mat_b(a: *mut c_void, b: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_trace_Mat_mtx(mtx: *mut c_void) -> cv_return_value_c_Scalar;
#[doc(hidden)] pub fn cv_core_cv_transform_Mat_src_Mat_dst_Mat_m(src: *mut c_void, dst: *mut c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_transpose_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_typeToString_int_type(_type: i32) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_useOpenVX() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_useOptimized() -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_utils_dumpInputArrayOfArrays_VectorOfMat_argument(argument: *mut c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_utils_dumpInputArray_Mat_argument(argument: *mut c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_utils_dumpInputOutputArrayOfArrays_VectorOfMat_argument(argument: *mut c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_utils_dumpInputOutputArray_Mat_argument(argument: *mut c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_core_cv_utils_getThreadID() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_vconcat_Mat_src1_Mat_src2_Mat_dst(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_vconcat_VectorOfMat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Algorithm_clear(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Algorithm_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_Algorithm_save_String_filename(instance: *const c_void, filename: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Algorithm_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
pub fn cv_delete_AutoLock(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_BufferPoolController_getReservedSize(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_BufferPoolController_getMaxReservedSize(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_BufferPoolController_setMaxReservedSize_size_t_size(instance: *mut c_void, size: size_t) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_BufferPoolController_freeAllReservedBuffers(instance: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_ConjGradSolver(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_ConjGradSolver_create_PtrOfFunction_f_TermCriteria_termcrit(f: *mut c_void, termcrit: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_DMatch_DMatch() -> cv_return_value_c_DMatch;
#[doc(hidden)] pub fn cv_core_cv_DMatch_DMatch_int__queryIdx_int__trainIdx_float__distance(_query_idx: i32, _train_idx: i32, _distance: f32) -> cv_return_value_c_DMatch;
#[doc(hidden)] pub fn cv_core_cv_DMatch_DMatch_int__queryIdx_int__trainIdx_int__imgIdx_float__distance(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32) -> cv_return_value_c_DMatch;
#[doc(hidden)] pub fn cv_core_cv_DownhillSolver_getInitStep_Mat_step(instance: *const c_void, step: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_DownhillSolver_setInitStep_Mat_step(instance: *mut c_void, step: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_DownhillSolver_create_PtrOfFunction_f_Mat_initStep_TermCriteria_termcrit(f: *mut c_void, init_step: *mut c_void, termcrit: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Formatted_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Formatter_format_Mat_mtx(instance: *const c_void, mtx: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Formatter_set32fPrecision_int_p(instance: *mut c_void, p: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Formatter_set64fPrecision_int_p(instance: *mut c_void, p: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Formatter_setMultiline_bool_ml(instance: *mut c_void, ml: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Formatter_get_int_fmt(fmt: i32) -> cv_return_value_void_X;
pub fn cv_delete_Hamming(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_KeyPoint() -> cv_return_value_c_KeyPoint;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_KeyPoint_Point2f__pt_float__size_float__angle_float__response_int__octave_int__class_id(_pt: core::Point2f, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> cv_return_value_c_KeyPoint;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_KeyPoint_float_x_float_y_float__size_float__angle_float__response_int__octave_int__class_id(x: f32, y: f32, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> cv_return_value_c_KeyPoint;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_hash(instance: core::KeyPoint) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_convert_VectorOfKeyPoint_keypoints_VectorOfPoint2f_points2f_VectorOfint_keypointIndexes(keypoints: *mut c_void, points2f: *mut c_void, keypoint_indexes: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_convert_VectorOfPoint2f_points2f_VectorOfKeyPoint_keypoints_float_size_float_response_int_octave_int_class_id(points2f: *mut c_void, keypoints: *mut c_void, size: f32, response: f32, octave: i32, class_id: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_KeyPoint_overlap_KeyPoint_kp1_KeyPoint_kp2(kp1: core::KeyPoint, kp2: core::KeyPoint) -> cv_return_value_float;
pub fn cv_delete_LDA(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_LDA_LDA_int_num_components(num_components: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_LDA_VectorOfMat_src_Mat_labels_int_num_components(src: *mut c_void, labels: *mut c_void, num_components: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_save_String_filename(instance: *const c_void, filename: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_LDA_load_String_filename(instance: *mut c_void, filename: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_LDA_compute_VectorOfMat_src_Mat_labels(instance: *mut c_void, src: *mut c_void, labels: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_LDA_project_Mat_src(instance: *mut c_void, src: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_reconstruct_Mat_src(instance: *mut c_void, src: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_eigenvectors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_eigenvalues(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_subspaceProject_Mat_W_Mat_mean_Mat_src(w: *mut c_void, mean: *mut c_void, src: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_LDA_subspaceReconstruct_Mat_W_Mat_mean_Mat_src(w: *mut c_void, mean: *mut c_void, src: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Mat(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_int_rows_int_cols_int_type(rows: i32, cols: i32, _type: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Size_size_int_type(size: core::Size, _type: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_int_rows_int_cols_int_type_Scalar_s(rows: i32, cols: i32, _type: i32, s: core::Scalar) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Size_size_int_type_Scalar_s(size: core::Size, _type: i32, s: core::Scalar) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_VectorOfint_sizes_int_type(sizes: *mut c_void, _type: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_VectorOfint_sizes_int_type_Scalar_s(sizes: *mut c_void, _type: i32, s: core::Scalar) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Mat_m(m: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Mat_m_Range_rowRange_Range_colRange(m: *mut c_void, row_range: *mut c_void, col_range: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Mat_m_Rect_roi(m: *mut c_void, roi: core::Rect) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Mat_m_Range_ranges(m: *mut c_void, ranges: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_Mat_Mat_m_VectorOfRange_ranges(m: *mut c_void, ranges: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_row_int_y(instance: *const c_void, y: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_col_int_x(instance: *const c_void, x: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_rowRange_int_startrow_int_endrow(instance: *const c_void, startrow: i32, endrow: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_rowRange_Range_r(instance: *const c_void, r: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_colRange_int_startcol_int_endcol(instance: *const c_void, startcol: i32, endcol: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_colRange_Range_r(instance: *const c_void, r: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_diag_int_d(instance: *const c_void, d: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_diag_Mat_d(d: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_clone(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_copyTo_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_copyTo_Mat_m_Mat_mask(instance: *const c_void, m: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_convertTo_Mat_m_int_rtype_double_alpha_double_beta(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, beta: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_assignTo_Mat_m_int_type(instance: *const c_void, m: *mut c_void, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_setTo_Mat_value_Mat_mask(instance: *mut c_void, value: *mut c_void, mask: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_reshape_int_cn_int_rows(instance: *const c_void, cn: i32, rows: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_reshape_int_cn_VectorOfint_newshape(instance: *const c_void, cn: i32, newshape: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_cross_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_dot_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_Mat_create_VectorOfint_sizes_int_type(instance: *mut c_void, sizes: *mut c_void, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_addref(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_release(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_deallocate(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_copySize_Mat_m(instance: *mut c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_reserve_size_t_sz(instance: *mut c_void, sz: size_t) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_reserveBuffer_size_t_sz(instance: *mut c_void, sz: size_t) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_resize_size_t_sz(instance: *mut c_void, sz: size_t) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_resize_size_t_sz_Scalar_s(instance: *mut c_void, sz: size_t, s: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_push_back_Mat_m(instance: *mut c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_pop_back_size_t_nelems(instance: *mut c_void, nelems: size_t) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_locateROI_Size_wholeSize_Point_ofs(instance: *const c_void, whole_size: core::Size, ofs: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_adjustROI_int_dtop_int_dbottom_int_dleft_int_dright(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Mat_isContinuous(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_Mat_isSubmatrix(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_Mat_elemSize(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_Mat_elemSize1(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_Mat_type(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_Mat_depth(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_Mat_channels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_Mat_step1_int_i(instance: *const c_void, i: i32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_Mat_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_Mat_total(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_Mat_total_int_startDim_int_endDim(instance: *const c_void, start_dim: i32, end_dim: i32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_Mat_checkVector_int_elemChannels_int_depth_bool_requireContinuous(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_Mat_ptr_int_i0(instance: *mut c_void, i0: i32) -> cv_return_value_unsigned_charX;
#[doc(hidden)] pub fn cv_core_cv_Mat_ptr_int_row_int_col(instance: *mut c_void, row: i32, col: i32) -> cv_return_value_unsigned_charX;
#[doc(hidden)] pub fn cv_core_cv_Mat_ptr_int_i0_int_i1_int_i2(instance: *mut c_void, i0: i32, i1: i32, i2: i32) -> cv_return_value_unsigned_charX;
#[doc(hidden)] pub fn cv_core_cv_Mat_updateContinuityFlag(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_Mat_size(instance: *const c_void) -> cv_return_value_c_Size;
pub fn cv_delete_MatExpr(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_MatExpr_size(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_core_cv_MatExpr_type(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_MatExpr_cross_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_MatExpr_dot_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_double;
pub fn cv_delete_MatSize(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_MatSize_dims(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_MatStep(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_MatStep_MatStep() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_MatStep_MatStep_size_t_s(s: size_t) -> cv_return_value_void_X;
pub fn cv_delete_Matx_AddOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_AddOp_Matx_AddOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_AddOp_Matx_AddOp_Matx_AddOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_DivOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_DivOp_Matx_DivOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_DivOp_Matx_DivOp_Matx_DivOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_MatMulOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_MatMulOp_Matx_MatMulOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_MatMulOp_Matx_MatMulOp_Matx_MatMulOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_MulOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_MulOp_Matx_MulOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_MulOp_Matx_MulOp_Matx_MulOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_ScaleOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_ScaleOp_Matx_ScaleOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_ScaleOp_Matx_ScaleOp_Matx_ScaleOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_SubOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_SubOp_Matx_SubOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_SubOp_Matx_SubOp_Matx_SubOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_Matx_TOp(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Matx_TOp_Matx_TOp() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Matx_TOp_Matx_TOp_Matx_TOp_unnamed_arg(unnamed_arg: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_getFunction(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_setFunction_PtrOfFunction_f(instance: *mut c_void, f: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_setTermCriteria_TermCriteria_termcrit(instance: *mut c_void, termcrit: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_minimize_Mat_x(instance: *mut c_void, x: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_Function_getDims(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_MinProblemSolver_Function_getGradientEps(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_Moments_Moments() -> cv_return_value_c_Moments;
#[doc(hidden)] pub fn cv_core_cv_Moments_Moments_double_m00_double_m10_double_m01_double_m20_double_m11_double_m02_double_m30_double_m21_double_m12_double_m03(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64) -> cv_return_value_c_Moments;
pub fn cv_delete_NAryMatIterator(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_NAryMatIterator_NAryMatIterator() -> cv_return_value_void_X;
pub fn cv_delete_PCA(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_PCA_PCA() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_PCA_PCA_Mat_data_Mat_mean_int_flags_int_maxComponents(data: *mut c_void, mean: *mut c_void, flags: i32, max_components: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_PCA_PCA_Mat_data_Mat_mean_int_flags_double_retainedVariance(data: *mut c_void, mean: *mut c_void, flags: i32, retained_variance: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_PCA_project_Mat_vec(instance: *const c_void, vec: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_PCA_project_Mat_vec_Mat_result(instance: *const c_void, vec: *mut c_void, result: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_PCA_backProject_Mat_vec(instance: *const c_void, vec: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_PCA_backProject_Mat_vec_Mat_result(instance: *const c_void, vec: *mut c_void, result: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_ParallelLoopBodyLambdaWrapper(ptr : *mut c_void);
pub fn cv_delete_Range(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_Range_Range() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Range_Range_int__start_int__end(_start: i32, _end: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_Range_size(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_Range_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_Range_all() -> cv_return_value_void_X;
pub fn cv_delete_RotatedRect(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_RotatedRect_get_angle(instance: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_core_RotatedRect_get_center(instance: *mut c_void) -> cv_return_value_c_Point2f;
#[doc(hidden)] pub fn cv_core_RotatedRect_get_size(instance: *mut c_void) -> cv_return_value_c_Size2f;
#[doc(hidden)] pub fn cv_core_cv_RotatedRect_RotatedRect() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_RotatedRect_RotatedRect_Point2f_center_Size2f_size_float_angle(center: core::Point2f, size: core::Size2f, angle: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_RotatedRect_RotatedRect_Point2f_point1_Point2f_point2_Point2f_point3(point1: core::Point2f, point2: core::Point2f, point3: core::Point2f) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_RotatedRect_boundingRect(instance: *const c_void) -> cv_return_value_c_Rect;
pub fn cv_delete_TermCriteria(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_TermCriteria_TermCriteria() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_TermCriteria_TermCriteria_int_type_int_maxCount_double_epsilon(_type: i32, max_count: i32, epsilon: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_TermCriteria_isValid(instance: *const c_void) -> cv_return_value_char;
pub fn cv_delete_TickMeter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_TickMeter_TickMeter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_start(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_stop(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_getTimeTicks(instance: *const c_void) -> cv_return_value_int64;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_getTimeMicro(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_getTimeMilli(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_getTimeSec(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_getCounter(instance: *const c_void) -> cv_return_value_int64;
#[doc(hidden)] pub fn cv_core_cv_TickMeter_reset(instance: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_UMat(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_UMat_getMat_int_flags(instance: *const c_void, flags: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_UMat_copyTo_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_copyTo_Mat_m_Mat_mask(instance: *const c_void, m: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_convertTo_Mat_m_int_rtype_double_alpha_double_beta(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, beta: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_dot_Mat_m(instance: *const c_void, m: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_UMat_addref(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_release(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_deallocate(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_locateROI_Size_wholeSize_Point_ofs(instance: *const c_void, whole_size: core::Size, ofs: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMat_isContinuous(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMat_isSubmatrix(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMat_elemSize(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_UMat_elemSize1(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_UMat_type(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_UMat_depth(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_UMat_channels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_UMat_step1_int_i(instance: *const c_void, i: i32) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_UMat_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMat_total(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_core_cv_UMat_checkVector_int_elemChannels_int_depth_bool_requireContinuous(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_UMat_updateContinuityFlag(instance: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_UMatData(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_UMatData_lock(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMatData_unlock(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMatData_hostCopyObsolete(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_deviceCopyObsolete(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_deviceMemMapped(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_copyOnMap(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_tempUMat(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_tempCopiedUMat(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_UMatData_markHostCopyObsolete_bool_flag(instance: *mut c_void, flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMatData_markDeviceCopyObsolete_bool_flag(instance: *mut c_void, flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_core_cv_UMatData_markDeviceMemMapped_bool_flag(instance: *mut c_void, flag: bool) -> cv_return_value_void;
pub fn cv_delete_CheckContext(ptr : *mut c_void);
pub fn cv_delete_float16_t(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_float16_t_float16_t() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_float16_t_float16_t_float_x(x: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_float16_t_fromBits_ushort_w(w: u16) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_float16_t_zero() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_float16_t_bits(instance: *const c_void) -> cv_return_value_unsigned_short;
pub fn cv_delete_NodeData(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_instr_NodeData_NodeData_NodeData_ref(_ref: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_instr_NodeData_getTotalMs(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_core_cv_instr_NodeData_getMeanMs(instance: *const c_void) -> cv_return_value_double;
pub fn cv_delete_NodeDataTls(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_instr_NodeDataTls_NodeDataTls() -> cv_return_value_void_X;
pub fn cv_delete_softdouble(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_softdouble_softdouble_softdouble_c(c: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_fromRaw_uint64_t_a(a: u64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_softdouble_int_a(a: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_softdouble_double_a(a: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_isNaN(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softdouble_isInf(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softdouble_isSubnormal(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softdouble_getSign(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softdouble_setSign_bool_sign(instance: *const c_void, sign: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_getExp(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_softdouble_setExp_int_e(instance: *const c_void, e: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_getFrac(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_setFrac_softdouble_s(instance: *const c_void, s: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_zero() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_inf() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_nan() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_one() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_min() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_eps() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_max() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softdouble_pi() -> cv_return_value_void_X;
pub fn cv_delete_softfloat(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_core_cv_softfloat_softfloat() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_softfloat_softfloat_c(c: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_fromRaw_uint32_t_a(a: u32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_softfloat_int_a(a: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_softfloat_float_a(a: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_isNaN(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softfloat_isInf(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softfloat_isSubnormal(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softfloat_getSign(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_core_cv_softfloat_setSign_bool_sign(instance: *const c_void, sign: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_getExp(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_core_cv_softfloat_setExp_int_e(instance: *const c_void, e: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_getFrac(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_setFrac_softfloat_s(instance: *const c_void, s: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_zero() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_inf() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_nan() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_one() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_min() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_eps() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_max() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_core_cv_softfloat_pi() -> cv_return_value_void_X;
}
extern "C" {
#[doc(hidden)] pub fn cv_calib3d_cvRANSACUpdateNumIters_double_p_double_err_prob_int_model_points_int_max_iters(p: f64, err_prob: f64, model_points: i32, max_iters: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_RQDecomp3x3_Mat_src_Mat_mtxR_Mat_mtxQ_Mat_Qx_Mat_Qy_Mat_Qz(src: *mut c_void, mtx_r: *mut c_void, mtx_q: *mut c_void, qx: *mut c_void, qy: *mut c_void, qz: *mut c_void) -> cv_return_value_c_Vec3d;
#[doc(hidden)] pub fn cv_calib3d_cv_Rodrigues_Mat_src_Mat_dst_Mat_jacobian(src: *mut c_void, dst: *mut c_void, jacobian: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_calibrateCamera_VectorOfMat_objectPoints_VectorOfMat_imagePoints_Size_imageSize_Mat_cameraMatrix_Mat_distCoeffs_VectorOfMat_rvecs_VectorOfMat_tvecs_Mat_stdDeviationsIntrinsics_Mat_stdDeviationsExtrinsics_Mat_perViewErrors_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points: *mut c_void, image_size: core::Size, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvecs: *mut c_void, tvecs: *mut c_void, std_deviations_intrinsics: *mut c_void, std_deviations_extrinsics: *mut c_void, per_view_errors: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_calibrateCamera_VectorOfMat_objectPoints_VectorOfMat_imagePoints_Size_imageSize_Mat_cameraMatrix_Mat_distCoeffs_VectorOfMat_rvecs_VectorOfMat_tvecs_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points: *mut c_void, image_size: core::Size, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvecs: *mut c_void, tvecs: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_calibrationMatrixValues_Mat_cameraMatrix_Size_imageSize_double_apertureWidth_double_apertureHeight_double_fovx_double_fovy_double_focalLength_Point2d_principalPoint_double_aspectRatio(camera_matrix: *mut c_void, image_size: core::Size, aperture_width: f64, aperture_height: f64, fovx: f64, fovy: f64, focal_length: f64, principal_point: core::Point2d, aspect_ratio: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_composeRT_Mat_rvec1_Mat_tvec1_Mat_rvec2_Mat_tvec2_Mat_rvec3_Mat_tvec3_Mat_dr3dr1_Mat_dr3dt1_Mat_dr3dr2_Mat_dr3dt2_Mat_dt3dr1_Mat_dt3dt1_Mat_dt3dr2_Mat_dt3dt2(rvec1: *mut c_void, tvec1: *mut c_void, rvec2: *mut c_void, tvec2: *mut c_void, rvec3: *mut c_void, tvec3: *mut c_void, dr3dr1: *mut c_void, dr3dt1: *mut c_void, dr3dr2: *mut c_void, dr3dt2: *mut c_void, dt3dr1: *mut c_void, dt3dt1: *mut c_void, dt3dr2: *mut c_void, dt3dt2: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_computeCorrespondEpilines_Mat_points_int_whichImage_Mat_F_Mat_lines(points: *mut c_void, which_image: i32, f: *mut c_void, lines: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_convertPointsFromHomogeneous_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_convertPointsHomogeneous_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_convertPointsToHomogeneous_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_correctMatches_Mat_F_Mat_points1_Mat_points2_Mat_newPoints1_Mat_newPoints2(f: *mut c_void, points1: *mut c_void, points2: *mut c_void, new_points1: *mut c_void, new_points2: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_decomposeEssentialMat_Mat_E_Mat_R1_Mat_R2_Mat_t(e: *mut c_void, r1: *mut c_void, r2: *mut c_void, t: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_decomposeHomographyMat_Mat_H_Mat_K_VectorOfMat_rotations_VectorOfMat_translations_VectorOfMat_normals(h: *mut c_void, k: *mut c_void, rotations: *mut c_void, translations: *mut c_void, normals: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_decomposeProjectionMatrix_Mat_projMatrix_Mat_cameraMatrix_Mat_rotMatrix_Mat_transVect_Mat_rotMatrixX_Mat_rotMatrixY_Mat_rotMatrixZ_Mat_eulerAngles(proj_matrix: *mut c_void, camera_matrix: *mut c_void, rot_matrix: *mut c_void, trans_vect: *mut c_void, rot_matrix_x: *mut c_void, rot_matrix_y: *mut c_void, rot_matrix_z: *mut c_void, euler_angles: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_drawChessboardCorners_Mat_image_Size_patternSize_Mat_corners_bool_patternWasFound(image: *mut c_void, pattern_size: core::Size, corners: *mut c_void, pattern_was_found: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_drawFrameAxes_Mat_image_Mat_cameraMatrix_Mat_distCoeffs_Mat_rvec_Mat_tvec_float_length_int_thickness(image: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvec: *mut c_void, tvec: *mut c_void, length: f32, thickness: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_estimateAffine2D_Mat_from_Mat_to_Mat_inliers_int_method_double_ransacReprojThreshold_size_t_maxIters_double_confidence_size_t_refineIters(from: *mut c_void, to: *mut c_void, inliers: *mut c_void, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_estimateAffine3D_Mat_src_Mat_dst_Mat_out_Mat_inliers_double_ransacThreshold_double_confidence(src: *mut c_void, dst: *mut c_void, out: *mut c_void, inliers: *mut c_void, ransac_threshold: f64, confidence: f64) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_estimateAffinePartial2D_Mat_from_Mat_to_Mat_inliers_int_method_double_ransacReprojThreshold_size_t_maxIters_double_confidence_size_t_refineIters(from: *mut c_void, to: *mut c_void, inliers: *mut c_void, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_filterHomographyDecompByVisibleRefpoints_VectorOfMat_rotations_VectorOfMat_normals_Mat_beforePoints_Mat_afterPoints_Mat_possibleSolutions_Mat_pointsMask(rotations: *mut c_void, normals: *mut c_void, before_points: *mut c_void, after_points: *mut c_void, possible_solutions: *mut c_void, points_mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_filterSpeckles_Mat_img_double_newVal_int_maxSpeckleSize_double_maxDiff_Mat_buf(img: *mut c_void, new_val: f64, max_speckle_size: i32, max_diff: f64, buf: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_find4QuadCornerSubpix_Mat_img_Mat_corners_Size_region_size(img: *mut c_void, corners: *mut c_void, region_size: core::Size) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_calib3d_cv_findChessboardCorners_Mat_image_Size_patternSize_Mat_corners_int_flags(image: *mut c_void, pattern_size: core::Size, corners: *mut c_void, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_calib3d_cv_findEssentialMat_Mat_points1_Mat_points2_Mat_cameraMatrix_int_method_double_prob_double_threshold_Mat_mask(points1: *mut c_void, points2: *mut c_void, camera_matrix: *mut c_void, method: i32, prob: f64, threshold: f64, mask: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_findEssentialMat_Mat_points1_Mat_points2_double_focal_Point2d_pp_int_method_double_prob_double_threshold_Mat_mask(points1: *mut c_void, points2: *mut c_void, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, mask: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_findFundamentalMat_Mat_points1_Mat_points2_Mat_mask_int_method_double_ransacReprojThreshold_double_confidence(points1: *mut c_void, points2: *mut c_void, mask: *mut c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_findFundamentalMat_Mat_points1_Mat_points2_int_method_double_ransacReprojThreshold_double_confidence_Mat_mask(points1: *mut c_void, points2: *mut c_void, method: i32, ransac_reproj_threshold: f64, confidence: f64, mask: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_findHomography_Mat_srcPoints_Mat_dstPoints_Mat_mask_int_method_double_ransacReprojThreshold(src_points: *mut c_void, dst_points: *mut c_void, mask: *mut c_void, method: i32, ransac_reproj_threshold: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_findHomography_Mat_srcPoints_Mat_dstPoints_int_method_double_ransacReprojThreshold_Mat_mask_int_maxIters_double_confidence(src_points: *mut c_void, dst_points: *mut c_void, method: i32, ransac_reproj_threshold: f64, mask: *mut c_void, max_iters: i32, confidence: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_calibrate_VectorOfMat_objectPoints_VectorOfMat_imagePoints_Size_image_size_Mat_K_Mat_D_VectorOfMat_rvecs_VectorOfMat_tvecs_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points: *mut c_void, image_size: core::Size, k: *mut c_void, d: *mut c_void, rvecs: *mut c_void, tvecs: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_distortPoints_Mat_undistorted_Mat_distorted_Mat_K_Mat_D_double_alpha(undistorted: *mut c_void, distorted: *mut c_void, k: *mut c_void, d: *mut c_void, alpha: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_estimateNewCameraMatrixForUndistortRectify_Mat_K_Mat_D_Size_image_size_Mat_R_Mat_P_double_balance_Size_new_size_double_fov_scale(k: *mut c_void, d: *mut c_void, image_size: core::Size, r: *mut c_void, p: *mut c_void, balance: f64, new_size: core::Size, fov_scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_projectPoints_Mat_objectPoints_Mat_imagePoints_Mat_rvec_Mat_tvec_Mat_K_Mat_D_double_alpha_Mat_jacobian(object_points: *mut c_void, image_points: *mut c_void, rvec: *mut c_void, tvec: *mut c_void, k: *mut c_void, d: *mut c_void, alpha: f64, jacobian: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_stereoCalibrate_VectorOfMat_objectPoints_VectorOfMat_imagePoints1_VectorOfMat_imagePoints2_Mat_K1_Mat_D1_Mat_K2_Mat_D2_Size_imageSize_Mat_R_Mat_T_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points1: *mut c_void, image_points2: *mut c_void, k1: *mut c_void, d1: *mut c_void, k2: *mut c_void, d2: *mut c_void, image_size: core::Size, r: *mut c_void, t: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_stereoRectify_Mat_K1_Mat_D1_Mat_K2_Mat_D2_Size_imageSize_Mat_R_Mat_tvec_Mat_R1_Mat_R2_Mat_P1_Mat_P2_Mat_Q_int_flags_Size_newImageSize_double_balance_double_fov_scale(k1: *mut c_void, d1: *mut c_void, k2: *mut c_void, d2: *mut c_void, image_size: core::Size, r: *mut c_void, tvec: *mut c_void, r1: *mut c_void, r2: *mut c_void, p1: *mut c_void, p2: *mut c_void, q: *mut c_void, flags: i32, new_image_size: core::Size, balance: f64, fov_scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_undistortImage_Mat_distorted_Mat_undistorted_Mat_K_Mat_D_Mat_Knew_Size_new_size(distorted: *mut c_void, undistorted: *mut c_void, k: *mut c_void, d: *mut c_void, knew: *mut c_void, new_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_fisheye_undistortPoints_Mat_distorted_Mat_undistorted_Mat_K_Mat_D_Mat_R_Mat_P(distorted: *mut c_void, undistorted: *mut c_void, k: *mut c_void, d: *mut c_void, r: *mut c_void, p: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_getValidDisparityROI_Rect_roi1_Rect_roi2_int_minDisparity_int_numberOfDisparities_int_SADWindowSize(roi1: core::Rect, roi2: core::Rect, min_disparity: i32, number_of_disparities: i32, sad_window_size: i32) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_calib3d_cv_initCameraMatrix2D_VectorOfMat_objectPoints_VectorOfMat_imagePoints_Size_imageSize_double_aspectRatio(object_points: *mut c_void, image_points: *mut c_void, image_size: core::Size, aspect_ratio: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_matMulDeriv_Mat_A_Mat_B_Mat_dABdA_Mat_dABdB(a: *mut c_void, b: *mut c_void, d_a_bd_a: *mut c_void, d_a_bd_b: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_projectPoints_Mat_objectPoints_Mat_rvec_Mat_tvec_Mat_cameraMatrix_Mat_distCoeffs_Mat_imagePoints_Mat_jacobian_double_aspectRatio(object_points: *mut c_void, rvec: *mut c_void, tvec: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, image_points: *mut c_void, jacobian: *mut c_void, aspect_ratio: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_recoverPose_Mat_E_Mat_points1_Mat_points2_Mat_R_Mat_t_double_focal_Point2d_pp_Mat_mask(e: *mut c_void, points1: *mut c_void, points2: *mut c_void, r: *mut c_void, t: *mut c_void, focal: f64, pp: core::Point2d, mask: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_recoverPose_Mat_E_Mat_points1_Mat_points2_Mat_cameraMatrix_Mat_R_Mat_t_Mat_mask(e: *mut c_void, points1: *mut c_void, points2: *mut c_void, camera_matrix: *mut c_void, r: *mut c_void, t: *mut c_void, mask: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_recoverPose_Mat_E_Mat_points1_Mat_points2_Mat_cameraMatrix_Mat_R_Mat_t_double_distanceThresh_Mat_mask_Mat_triangulatedPoints(e: *mut c_void, points1: *mut c_void, points2: *mut c_void, camera_matrix: *mut c_void, r: *mut c_void, t: *mut c_void, distance_thresh: f64, mask: *mut c_void, triangulated_points: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_reprojectImageTo3D_Mat_disparity_Mat__3dImage_Mat_Q_bool_handleMissingValues_int_ddepth(disparity: *mut c_void, _3d_image: *mut c_void, q: *mut c_void, handle_missing_values: bool, ddepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_sampsonDistance_Mat_pt1_Mat_pt2_Mat_F(pt1: *mut c_void, pt2: *mut c_void, f: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_solveP3P_Mat_objectPoints_Mat_imagePoints_Mat_cameraMatrix_Mat_distCoeffs_VectorOfMat_rvecs_VectorOfMat_tvecs_int_flags(object_points: *mut c_void, image_points: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvecs: *mut c_void, tvecs: *mut c_void, flags: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_solvePnPRansac_Mat_objectPoints_Mat_imagePoints_Mat_cameraMatrix_Mat_distCoeffs_Mat_rvec_Mat_tvec_bool_useExtrinsicGuess_int_iterationsCount_float_reprojectionError_double_confidence_Mat_inliers_int_flags(object_points: *mut c_void, image_points: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvec: *mut c_void, tvec: *mut c_void, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: *mut c_void, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_calib3d_cv_solvePnP_Mat_objectPoints_Mat_imagePoints_Mat_cameraMatrix_Mat_distCoeffs_Mat_rvec_Mat_tvec_bool_useExtrinsicGuess_int_flags(object_points: *mut c_void, image_points: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, rvec: *mut c_void, tvec: *mut c_void, use_extrinsic_guess: bool, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_calib3d_cv_stereoCalibrate_VectorOfMat_objectPoints_VectorOfMat_imagePoints1_VectorOfMat_imagePoints2_Mat_cameraMatrix1_Mat_distCoeffs1_Mat_cameraMatrix2_Mat_distCoeffs2_Size_imageSize_Mat_R_Mat_T_Mat_E_Mat_F_Mat_perViewErrors_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points1: *mut c_void, image_points2: *mut c_void, camera_matrix1: *mut c_void, dist_coeffs1: *mut c_void, camera_matrix2: *mut c_void, dist_coeffs2: *mut c_void, image_size: core::Size, r: *mut c_void, t: *mut c_void, e: *mut c_void, f: *mut c_void, per_view_errors: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_stereoCalibrate_VectorOfMat_objectPoints_VectorOfMat_imagePoints1_VectorOfMat_imagePoints2_Mat_cameraMatrix1_Mat_distCoeffs1_Mat_cameraMatrix2_Mat_distCoeffs2_Size_imageSize_Mat_R_Mat_T_Mat_E_Mat_F_int_flags_TermCriteria_criteria(object_points: *mut c_void, image_points1: *mut c_void, image_points2: *mut c_void, camera_matrix1: *mut c_void, dist_coeffs1: *mut c_void, camera_matrix2: *mut c_void, dist_coeffs2: *mut c_void, image_size: core::Size, r: *mut c_void, t: *mut c_void, e: *mut c_void, f: *mut c_void, flags: i32, criteria: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_calib3d_cv_stereoRectifyUncalibrated_Mat_points1_Mat_points2_Mat_F_Size_imgSize_Mat_H1_Mat_H2_double_threshold(points1: *mut c_void, points2: *mut c_void, f: *mut c_void, img_size: core::Size, h1: *mut c_void, h2: *mut c_void, threshold: f64) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_calib3d_cv_triangulatePoints_Mat_projMatr1_Mat_projMatr2_Mat_projPoints1_Mat_projPoints2_Mat_points4D(proj_matr1: *mut c_void, proj_matr2: *mut c_void, proj_points1: *mut c_void, proj_points2: *mut c_void, points4_d: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_validateDisparity_Mat_disparity_Mat_cost_int_minDisparity_int_numberOfDisparities_int_disp12MaxDisp(disparity: *mut c_void, cost: *mut c_void, min_disparity: i32, number_of_disparities: i32, disp12_max_disp: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_CirclesGridFinderParameters2_CirclesGridFinderParameters2() -> cv_return_value_c_CirclesGridFinderParameters2;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getPreFilterType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setPreFilterType_int_preFilterType(instance: *mut c_void, pre_filter_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getPreFilterSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setPreFilterSize_int_preFilterSize(instance: *mut c_void, pre_filter_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getPreFilterCap(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setPreFilterCap_int_preFilterCap(instance: *mut c_void, pre_filter_cap: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getTextureThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setTextureThreshold_int_textureThreshold(instance: *mut c_void, texture_threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getUniquenessRatio(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setUniquenessRatio_int_uniquenessRatio(instance: *mut c_void, uniqueness_ratio: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getSmallerBlockSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setSmallerBlockSize_int_blockSize(instance: *mut c_void, block_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getROI1(instance: *const c_void) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setROI1_Rect_roi1(instance: *mut c_void, roi1: core::Rect) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_getROI2(instance: *const c_void) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_setROI2_Rect_roi2(instance: *mut c_void, roi2: core::Rect) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoBM_create_int_numDisparities_int_blockSize(num_disparities: i32, block_size: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_compute_Mat_left_Mat_right_Mat_disparity(instance: *mut c_void, left: *mut c_void, right: *mut c_void, disparity: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getMinDisparity(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setMinDisparity_int_minDisparity(instance: *mut c_void, min_disparity: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getNumDisparities(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setNumDisparities_int_numDisparities(instance: *mut c_void, num_disparities: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getBlockSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setBlockSize_int_blockSize(instance: *mut c_void, block_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getSpeckleWindowSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setSpeckleWindowSize_int_speckleWindowSize(instance: *mut c_void, speckle_window_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getSpeckleRange(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setSpeckleRange_int_speckleRange(instance: *mut c_void, speckle_range: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_getDisp12MaxDiff(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoMatcher_setDisp12MaxDiff_int_disp12MaxDiff(instance: *mut c_void, disp12_max_diff: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_getPreFilterCap(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_setPreFilterCap_int_preFilterCap(instance: *mut c_void, pre_filter_cap: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_getUniquenessRatio(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_setUniquenessRatio_int_uniquenessRatio(instance: *mut c_void, uniqueness_ratio: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_getP1(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_setP1_int_P1(instance: *mut c_void, p1: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_getP2(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_setP2_int_P2(instance: *mut c_void, p2: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_getMode(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_setMode_int_mode(instance: *mut c_void, mode: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_calib3d_cv_StereoSGBM_create_int_minDisparity_int_numDisparities_int_blockSize_int_P1_int_P2_int_disp12MaxDiff_int_preFilterCap_int_uniquenessRatio_int_speckleWindowSize_int_speckleRange_int_mode(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32) -> cv_return_value_void_X;
}
extern "C" {
#[doc(hidden)] pub fn cv_dnn_cv_dnn_NMSBoxes_VectorOfRect2d_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes: *mut c_void, scores: *mut c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_NMSBoxes_VectorOfRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes: *mut c_void, scores: *mut c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_NMSBoxes_VectorOfRotatedRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes: *mut c_void, scores: *mut c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_blobFromImage_Mat_image_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(image: *mut c_void, blob: *mut c_void, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_blobFromImage_Mat_image_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(image: *mut c_void, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_blobFromImages_VectorOfMat_images_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(images: *mut c_void, blob: *mut c_void, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_blobFromImages_VectorOfMat_images_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(images: *mut c_void, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_clamp_Range_r_int_axisSize(r: *mut c_void, axis_size: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_clamp_int_ax_int_dims(ax: i32, dims: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_getPlane_Mat_m_int_n_int_cn(m: *mut c_void, n: i32, cn: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_imagesFromBlob_Mat_blob__VectorOfMat_images_(blob_: *mut c_void, images_: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromCaffe_String_prototxt_String_caffeModel(prototxt: *const c_char, caffe_model: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromCaffe_VectorOfuchar_bufferProto_VectorOfuchar_bufferModel(buffer_proto: *mut c_void, buffer_model: *mut c_void) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromDarknet_String_cfgFile_String_darknetModel(cfg_file: *const c_char, darknet_model: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromDarknet_VectorOfuchar_bufferCfg_VectorOfuchar_bufferModel(buffer_cfg: *mut c_void, buffer_model: *mut c_void) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromModelOptimizer_String_xml_String_bin(xml: *const c_char, bin: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromONNX_String_onnxFile(onnx_file: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromTensorflow_String_model_String_config(model: *const c_char, config: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromTensorflow_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig(buffer_model: *mut c_void, buffer_config: *mut c_void) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNetFromTorch_String_model_bool_isBinary_bool_evaluate(model: *const c_char, is_binary: bool, evaluate: bool) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNet_String_framework_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig(framework: *const c_char, buffer_model: *mut c_void, buffer_config: *mut c_void) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readNet_String_model_String_config_String_framework(model: *const c_char, config: *const c_char, framework: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readTensorFromONNX_String_path(path: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_readTorchBlob_String_filename_bool_isBinary(filename: *const c_char, is_binary: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_resetMyriadDevice() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_shrinkCaffeModel_String_src_String_dst_VectorOfString_layersTypes(src: *const c_char, dst: *const c_char, layers_types: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_slice_Mat_m_Range_r0(m: *mut c_void, r0: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1(m: *mut c_void, r0: *mut c_void, r1: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2(m: *mut c_void, r0: *mut c_void, r1: *mut c_void, r2: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2_Range_r3(m: *mut c_void, r0: *mut c_void, r1: *mut c_void, r2: *mut c_void, r3: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_writeTextGraph_String_model_String_output(model: *const c_char, output: *const c_char) -> cv_return_value_void;
pub fn cv_delete_AbsLayer(ptr : *mut c_void);
pub fn cv_delete_BNLLLayer(ptr : *mut c_void);
pub fn cv_delete_BackendNode(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_BackendNode_BackendNode_int_backendId(backend_id: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_BackendWrapper_copyToHost(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_BackendWrapper_setHostDirty(instance: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_BatchNormLayer(ptr : *mut c_void);
pub fn cv_delete_BlankLayer(ptr : *mut c_void);
pub fn cv_delete_ChannelsPReLULayer(ptr : *mut c_void);
pub fn cv_delete_ConcatLayer(ptr : *mut c_void);
pub fn cv_delete_ConstLayer(ptr : *mut c_void);
pub fn cv_delete_ConvolutionLayer(ptr : *mut c_void);
pub fn cv_delete_CropAndResizeLayer(ptr : *mut c_void);
pub fn cv_delete_CropLayer(ptr : *mut c_void);
pub fn cv_delete_DeconvolutionLayer(ptr : *mut c_void);
pub fn cv_delete_DetectionOutputLayer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Dict_has_String_key(instance: *const c_void, key: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Dict_ptr_String_key(instance: *mut c_void, key: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Dict_get_String_key(instance: *const c_void, key: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Dict_erase_String_key(instance: *mut c_void, key: *const c_char) -> cv_return_value_void;
pub fn cv_delete_DictValue(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_DictValue_DictValue_r(r: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_size(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_isInt(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_isString(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_isReal(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_getIntValue_int_idx(instance: *const c_void, idx: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_getRealValue_int_idx(instance: *const c_void, idx: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_DictValue_getStringValue_int_idx(instance: *const c_void, idx: i32) -> cv_return_value_char_X;
pub fn cv_delete_ELULayer(ptr : *mut c_void);
pub fn cv_delete_EltwiseLayer(ptr : *mut c_void);
pub fn cv_delete_FlattenLayer(ptr : *mut c_void);
pub fn cv_delete_InnerProductLayer(ptr : *mut c_void);
pub fn cv_delete_InterpLayer(ptr : *mut c_void);
pub fn cv_delete_LRNLayer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LSTMLayer_setWeights_Mat_Wh_Mat_Wx_Mat_b(instance: *mut c_void, wh: *mut c_void, wx: *mut c_void, b: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LSTMLayer_setUseTimstampsDim_bool_use(instance: *mut c_void, _use: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LSTMLayer_setProduceCellOutput_bool_produce(instance: *mut c_void, produce: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LSTMLayer_inputNameToIndex_String_inputName(instance: *mut c_void, input_name: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LSTMLayer_outputNameToIndex_String_outputName(instance: *mut c_void, output_name: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_finalize_VectorOfMat_inputs_VectorOfMat_outputs(instance: *mut c_void, inputs: *mut c_void, outputs: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_forward_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(instance: *mut c_void, inputs: *mut c_void, outputs: *mut c_void, internals: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_forward_fallback_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(instance: *mut c_void, inputs: *mut c_void, outputs: *mut c_void, internals: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_finalize_VectorOfMat_inputs(instance: *mut c_void, inputs: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_run_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(instance: *mut c_void, inputs: *mut c_void, outputs: *mut c_void, internals: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_inputNameToIndex_String_inputName(instance: *mut c_void, input_name: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_outputNameToIndex_String_outputName(instance: *mut c_void, output_name: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_supportBackend_int_backendId(instance: *mut c_void, backend_id: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_initHalide_VectorOfPtrOfBackendWrapper_inputs(instance: *mut c_void, inputs: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_initInfEngine_VectorOfPtrOfBackendWrapper_inputs(instance: *mut c_void, inputs: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_tryAttach_PtrOfBackendNode_node(instance: *mut c_void, node: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_setActivation_PtrOfActivationLayer_layer(instance: *mut c_void, layer: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_getScaleShift_Mat_scale_Mat_shift(instance: *const c_void, scale: *mut c_void, shift: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Layer_unsetAttached(instance: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_LayerFactory(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_LayerFactory_unregisterLayer_String_type(_type: *const c_char) -> cv_return_value_void;
pub fn cv_delete_LayerParams(ptr : *mut c_void);
pub fn cv_delete_MVNLayer(ptr : *mut c_void);
pub fn cv_delete_MaxUnpoolLayer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_Net() -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_readFromModelOptimizer_String_xml_String_bin(xml: *const c_char, bin: *const c_char) -> cv_return_value_c_dnn_Net;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_empty(instance: super::dnn::dnn_Net) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_addLayer_String_name_String_type_LayerParams_params(instance: super::dnn::dnn_Net, name: *const c_char, _type: *const c_char, params: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_addLayerToPrev_String_name_String_type_LayerParams_params(instance: super::dnn::dnn_Net, name: *const c_char, _type: *const c_char, params: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getLayerId_String_layer(instance: super::dnn::dnn_Net, layer: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getLayerNames(instance: super::dnn::dnn_Net) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_connect_String_outPin_String_inpPin(instance: super::dnn::dnn_Net, out_pin: *const c_char, inp_pin: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_connect_int_outLayerId_int_outNum_int_inpLayerId_int_inpNum(instance: super::dnn::dnn_Net, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_setInputsNames_VectorOfString_inputBlobNames(instance: super::dnn::dnn_Net, input_blob_names: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_forward_String_outputName(instance: super::dnn::dnn_Net, output_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_forward_VectorOfMat_outputBlobs_String_outputName(instance: super::dnn::dnn_Net, output_blobs: *mut c_void, output_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_forward_VectorOfMat_outputBlobs_VectorOfString_outBlobNames(instance: super::dnn::dnn_Net, output_blobs: *mut c_void, out_blob_names: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_forward_VectorOfVectorOfMat_outputBlobs_VectorOfString_outBlobNames(instance: super::dnn::dnn_Net, output_blobs: *mut c_void, out_blob_names: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_setHalideScheduler_String_scheduler(instance: super::dnn::dnn_Net, scheduler: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_setPreferableBackend_int_backendId(instance: super::dnn::dnn_Net, backend_id: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_setPreferableTarget_int_targetId(instance: super::dnn::dnn_Net, target_id: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_setInput_Mat_blob_String_name_double_scalefactor_Scalar_mean(instance: super::dnn::dnn_Net, blob: *mut c_void, name: *const c_char, scalefactor: f64, mean: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getUnconnectedOutLayers(instance: super::dnn::dnn_Net) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getUnconnectedOutLayersNames(instance: super::dnn::dnn_Net) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getLayerTypes_VectorOfString_layersTypes(instance: super::dnn::dnn_Net, layers_types: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getLayersCount_String_layerType(instance: super::dnn::dnn_Net, layer_type: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_enableFusion_bool_fusion(instance: super::dnn::dnn_Net, fusion: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_Net_getPerfProfile_VectorOfdouble_timings(instance: super::dnn::dnn_Net, timings: *mut c_void) -> cv_return_value_int64;
pub fn cv_delete_NormalizeBBoxLayer(ptr : *mut c_void);
pub fn cv_delete_PaddingLayer(ptr : *mut c_void);
pub fn cv_delete_PermuteLayer(ptr : *mut c_void);
pub fn cv_delete_PoolingLayer(ptr : *mut c_void);
pub fn cv_delete_PowerLayer(ptr : *mut c_void);
pub fn cv_delete_PriorBoxLayer(ptr : *mut c_void);
pub fn cv_delete_ProposalLayer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_dnn_cv_dnn_RNNLayer_setWeights_Mat_Wxh_Mat_bh_Mat_Whh_Mat_Who_Mat_bo(instance: *mut c_void, wxh: *mut c_void, bh: *mut c_void, whh: *mut c_void, who: *mut c_void, bo: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_dnn_cv_dnn_RNNLayer_setProduceHiddenOutput_bool_produce(instance: *mut c_void, produce: bool) -> cv_return_value_void;
pub fn cv_delete_ReLU6Layer(ptr : *mut c_void);
pub fn cv_delete_ReLULayer(ptr : *mut c_void);
pub fn cv_delete_RegionLayer(ptr : *mut c_void);
pub fn cv_delete_ReorgLayer(ptr : *mut c_void);
pub fn cv_delete_ReshapeLayer(ptr : *mut c_void);
pub fn cv_delete_ResizeLayer(ptr : *mut c_void);
pub fn cv_delete_ScaleLayer(ptr : *mut c_void);
pub fn cv_delete_ShiftLayer(ptr : *mut c_void);
pub fn cv_delete_ShuffleChannelLayer(ptr : *mut c_void);
pub fn cv_delete_SigmoidLayer(ptr : *mut c_void);
pub fn cv_delete_SliceLayer(ptr : *mut c_void);
pub fn cv_delete_SoftmaxLayer(ptr : *mut c_void);
pub fn cv_delete_SplitLayer(ptr : *mut c_void);
pub fn cv_delete_TanHLayer(ptr : *mut c_void);
}
extern "C" {
#[doc(hidden)] pub fn cv_features2d_cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression(image: *mut c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression_int_type(image: *mut c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression(image: *mut c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression_int_type(image: *mut c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_computeRecallPrecisionCurve_VectorOfVectorOfDMatch_matches1to2_VectorOfVectorOfuchar_correctMatches1to2Mask_VectorOfPoint2f_recallPrecisionCurve(matches1to2: *mut c_void, correct_matches1to2_mask: *mut c_void, recall_precision_curve: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_drawKeypoints_Mat_image_VectorOfKeyPoint_keypoints_Mat_outImage_Scalar_color_int_flags(image: *mut c_void, keypoints: *mut c_void, out_image: *mut c_void, color: core::Scalar, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_drawMatches_Mat_img1_VectorOfKeyPoint_keypoints1_Mat_img2_VectorOfKeyPoint_keypoints2_VectorOfDMatch_matches1to2_Mat_outImg_Scalar_matchColor_Scalar_singlePointColor_VectorOfchar_matchesMask_int_flags(img1: *mut c_void, keypoints1: *mut c_void, img2: *mut c_void, keypoints2: *mut c_void, matches1to2: *mut c_void, out_img: *mut c_void, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_drawMatches_Mat_img1_VectorOfKeyPoint_keypoints1_Mat_img2_VectorOfKeyPoint_keypoints2_VectorOfVectorOfDMatch_matches1to2_Mat_outImg_Scalar_matchColor_Scalar_singlePointColor_VectorOfVectorOfchar_matchesMask_int_flags(img1: *mut c_void, keypoints1: *mut c_void, img2: *mut c_void, keypoints2: *mut c_void, matches1to2: *mut c_void, out_img: *mut c_void, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_getNearestPoint_VectorOfPoint2f_recallPrecisionCurve_float_l_precision(recall_precision_curve: *mut c_void, l_precision: f32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_getRecall_VectorOfPoint2f_recallPrecisionCurve_float_l_precision(recall_precision_curve: *mut c_void, l_precision: f32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setDescriptorType_int_dtype(instance: *mut c_void, dtype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getDescriptorType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setDescriptorSize_int_dsize(instance: *mut c_void, dsize: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getDescriptorSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setDescriptorChannels_int_dch(instance: *mut c_void, dch: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getDescriptorChannels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setThreshold_double_threshold(instance: *mut c_void, threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setNOctaves_int_octaves(instance: *mut c_void, octaves: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getNOctaves(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setNOctaveLayers_int_octaveLayers(instance: *mut c_void, octave_layers: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getNOctaveLayers(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_setDiffusivity_int_diff(instance: *mut c_void, diff: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getDiffusivity(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_AKAZE_create_int_descriptor_type_int_descriptor_size_int_descriptor_channels_float_threshold_int_nOctaves_int_nOctaveLayers_int_diffusivity(descriptor_type: i32, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_setThreshold_int_threshold(instance: *mut c_void, threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_getThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_setNonmaxSuppression_bool_f(instance: *mut c_void, f: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_getNonmaxSuppression(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_setType_int_type(instance: *mut c_void, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_getType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_AgastFeatureDetector_create_int_threshold_bool_nonmaxSuppression_int_type(threshold: i32, nonmax_suppression: bool, _type: i32) -> cv_return_value_void_X;
pub fn cv_delete_BFMatcher(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_BFMatcher_BFMatcher_int_normType_bool_crossCheck(norm_type: i32, cross_check: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BFMatcher_isMaskSupported(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_BFMatcher_create_int_normType_bool_crossCheck(norm_type: i32, cross_check: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BFMatcher_clone_bool_emptyTrainData(instance: *const c_void, empty_train_data: bool) -> cv_return_value_void_X;
pub fn cv_delete_BOWImgDescriptorExtractor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_PtrOfDescriptorMatcher_dmatcher(dmatcher: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_setVocabulary_Mat_vocabulary(instance: *mut c_void, vocabulary: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_getVocabulary(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_imgDescriptor_VectorOfVectorOfint_pointIdxsOfClusters_Mat_descriptors(instance: *mut c_void, image: *mut c_void, keypoints: *mut c_void, img_descriptor: *mut c_void, point_idxs_of_clusters: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_compute_Mat_keypointDescriptors_Mat_imgDescriptor_VectorOfVectorOfint_pointIdxsOfClusters(instance: *mut c_void, keypoint_descriptors: *mut c_void, img_descriptor: *mut c_void, point_idxs_of_clusters: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_compute2_Mat_image_VectorOfKeyPoint_keypoints_Mat_imgDescriptor(instance: *mut c_void, image: *mut c_void, keypoints: *mut c_void, img_descriptor: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_descriptorSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_BOWImgDescriptorExtractor_descriptorType(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_BOWKMeansTrainer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_BOWKMeansTrainer_BOWKMeansTrainer_int_clusterCount_TermCriteria_termcrit_int_attempts_int_flags(cluster_count: i32, termcrit: *mut c_void, attempts: i32, flags: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWKMeansTrainer_cluster(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWKMeansTrainer_cluster_Mat_descriptors(instance: *const c_void, descriptors: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_add_Mat_descriptors(instance: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_getDescriptors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_descriptorsCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_clear(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_cluster(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BOWTrainer_cluster_Mat_descriptors(instance: *const c_void, descriptors: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_BRISK(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_BRISK_create_int_thresh_int_octaves_float_patternScale(thresh: i32, octaves: i32, pattern_scale: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BRISK_create_VectorOffloat_radiusList_VectorOfint_numberList_float_dMax_float_dMin_VectorOfint_indexChange(radius_list: *mut c_void, number_list: *mut c_void, d_max: f32, d_min: f32, index_change: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BRISK_create_int_thresh_int_octaves_VectorOffloat_radiusList_VectorOfint_numberList_float_dMax_float_dMin_VectorOfint_indexChange(thresh: i32, octaves: i32, radius_list: *mut c_void, number_list: *mut c_void, d_max: f32, d_min: f32, index_change: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_BRISK_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_add_VectorOfMat_descriptors(instance: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_getTrainDescriptors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_clear(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_isMaskSupported(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_train(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_match_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfDMatch_matches_Mat_mask(instance: *const c_void, query_descriptors: *mut c_void, train_descriptors: *mut c_void, matches: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_knnMatch_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_int_k_Mat_mask_bool_compactResult(instance: *const c_void, query_descriptors: *mut c_void, train_descriptors: *mut c_void, matches: *mut c_void, k: i32, mask: *mut c_void, compact_result: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_radiusMatch_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_float_maxDistance_Mat_mask_bool_compactResult(instance: *const c_void, query_descriptors: *mut c_void, train_descriptors: *mut c_void, matches: *mut c_void, max_distance: f32, mask: *mut c_void, compact_result: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_match_Mat_queryDescriptors_VectorOfDMatch_matches_VectorOfMat_masks(instance: *mut c_void, query_descriptors: *mut c_void, matches: *mut c_void, masks: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_knnMatch_Mat_queryDescriptors_VectorOfVectorOfDMatch_matches_int_k_VectorOfMat_masks_bool_compactResult(instance: *mut c_void, query_descriptors: *mut c_void, matches: *mut c_void, k: i32, masks: *mut c_void, compact_result: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_radiusMatch_Mat_queryDescriptors_VectorOfVectorOfDMatch_matches_float_maxDistance_VectorOfMat_masks_bool_compactResult(instance: *mut c_void, query_descriptors: *mut c_void, matches: *mut c_void, max_distance: f32, masks: *mut c_void, compact_result: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_write_String_fileName(instance: *const c_void, file_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_read_String_fileName(instance: *mut c_void, file_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_clone_bool_emptyTrainData(instance: *const c_void, empty_train_data: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_create_String_descriptorMatcherType(descriptor_matcher_type: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_DescriptorMatcher_create_int_matcherType(matcher_type: i32) -> cv_return_value_void_X;
pub fn cv_delete_DrawMatchesFlags(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_setThreshold_int_threshold(instance: *mut c_void, threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_getThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_setNonmaxSuppression_bool_f(instance: *mut c_void, f: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_getNonmaxSuppression(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_setType_int_type(instance: *mut c_void, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_getType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_FastFeatureDetector_create_int_threshold_bool_nonmaxSuppression_int_type(threshold: i32, nonmax_suppression: bool, _type: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_detect_Mat_image_VectorOfKeyPoint_keypoints_Mat_mask(instance: *mut c_void, image: *mut c_void, keypoints: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_detect_VectorOfMat_images_VectorOfVectorOfKeyPoint_keypoints_VectorOfMat_masks(instance: *mut c_void, images: *mut c_void, keypoints: *mut c_void, masks: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_descriptors(instance: *mut c_void, image: *mut c_void, keypoints: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_compute_VectorOfMat_images_VectorOfVectorOfKeyPoint_keypoints_VectorOfMat_descriptors(instance: *mut c_void, images: *mut c_void, keypoints: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_detectAndCompute_Mat_image_Mat_mask_VectorOfKeyPoint_keypoints_Mat_descriptors_bool_useProvidedKeypoints(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, keypoints: *mut c_void, descriptors: *mut c_void, use_provided_keypoints: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_descriptorSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_descriptorType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_defaultNorm(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_write_String_fileName(instance: *const c_void, file_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_read_String_fileName(instance: *mut c_void, file_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_Feature2D_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
pub fn cv_delete_FlannBasedMatcher(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_add_VectorOfMat_descriptors(instance: *mut c_void, descriptors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_clear(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_train(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_isMaskSupported(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_FlannBasedMatcher_clone_bool_emptyTrainData(instance: *const c_void, empty_train_data: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setMaxFeatures_int_maxFeatures(instance: *mut c_void, max_features: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getMaxFeatures(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setQualityLevel_double_qlevel(instance: *mut c_void, qlevel: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getQualityLevel(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setMinDistance_double_minDistance(instance: *mut c_void, min_distance: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getMinDistance(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setBlockSize_int_blockSize(instance: *mut c_void, block_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getBlockSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setHarrisDetector_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getHarrisDetector(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_setK_double_k(instance: *mut c_void, k: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getK(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_create_int_maxCorners_double_qualityLevel_double_minDistance_int_blockSize_bool_useHarrisDetector_double_k(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_GFTTDetector_create_int_maxCorners_double_qualityLevel_double_minDistance_int_blockSize_int_gradiantSize_bool_useHarrisDetector_double_k(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setExtended_bool_extended(instance: *mut c_void, extended: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getExtended(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setUpright_bool_upright(instance: *mut c_void, upright: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getUpright(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setThreshold_double_threshold(instance: *mut c_void, threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setNOctaves_int_octaves(instance: *mut c_void, octaves: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getNOctaves(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setNOctaveLayers_int_octaveLayers(instance: *mut c_void, octave_layers: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getNOctaveLayers(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_setDiffusivity_int_diff(instance: *mut c_void, diff: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getDiffusivity(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_KAZE_create_bool_extended_bool_upright_float_threshold_int_nOctaves_int_nOctaveLayers_int_diffusivity(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> cv_return_value_void_X;
pub fn cv_delete_KeyPointsFilter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_KeyPointsFilter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_runByImageBorder_VectorOfKeyPoint_keypoints_Size_imageSize_int_borderSize(keypoints: *mut c_void, image_size: core::Size, border_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_runByKeypointSize_VectorOfKeyPoint_keypoints_float_minSize_float_maxSize(keypoints: *mut c_void, min_size: f32, max_size: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_runByPixelsMask_VectorOfKeyPoint_keypoints_Mat_mask(keypoints: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_removeDuplicated_VectorOfKeyPoint_keypoints(keypoints: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_removeDuplicatedSorted_VectorOfKeyPoint_keypoints(keypoints: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_KeyPointsFilter_retainBest_VectorOfKeyPoint_keypoints_int_npoints(keypoints: *mut c_void, npoints: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_detectRegions_Mat_image_VectorOfVectorOfPoint_msers_VectorOfRect_bboxes(instance: *mut c_void, image: *mut c_void, msers: *mut c_void, bboxes: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_setDelta_int_delta(instance: *mut c_void, delta: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_getDelta(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_setMinArea_int_minArea(instance: *mut c_void, min_area: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_getMinArea(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_setMaxArea_int_maxArea(instance: *mut c_void, max_area: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_getMaxArea(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_setPass2Only_bool_f(instance: *mut c_void, f: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_getPass2Only(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_MSER_create_int__delta_int__min_area_int__max_area_double__max_variation_double__min_diversity_int__max_evolution_double__area_threshold_double__min_margin_int__edge_blur_size(_delta: i32, _min_area: i32, _max_area: i32, _max_variation: f64, _min_diversity: f64, _max_evolution: i32, _area_threshold: f64, _min_margin: f64, _edge_blur_size: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setMaxFeatures_int_maxFeatures(instance: *mut c_void, max_features: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getMaxFeatures(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setScaleFactor_double_scaleFactor(instance: *mut c_void, scale_factor: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getScaleFactor(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setNLevels_int_nlevels(instance: *mut c_void, nlevels: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getNLevels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setEdgeThreshold_int_edgeThreshold(instance: *mut c_void, edge_threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getEdgeThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setFirstLevel_int_firstLevel(instance: *mut c_void, first_level: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getFirstLevel(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setWTA_K_int_wta_k(instance: *mut c_void, wta_k: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getWTA_K(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setScoreType_int_scoreType(instance: *mut c_void, score_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getScoreType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setPatchSize_int_patchSize(instance: *mut c_void, patch_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getPatchSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_setFastThreshold_int_fastThreshold(instance: *mut c_void, fast_threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getFastThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_ORB_create_int_nfeatures_float_scaleFactor_int_nlevels_int_edgeThreshold_int_firstLevel_int_WTA_K_int_scoreType_int_patchSize_int_fastThreshold(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: i32, patch_size: i32, fast_threshold: i32) -> cv_return_value_void_X;
pub fn cv_delete_SimpleBlobDetector(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_features2d_cv_SimpleBlobDetector_create_SimpleBlobDetector_Params_parameters(parameters: super::features2d::SimpleBlobDetector_Params) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_features2d_cv_SimpleBlobDetector_getDefaultName(instance: *const c_void) -> cv_return_value_char_X;
#[doc(hidden)] pub fn cv_features2d_cv_SimpleBlobDetector_Params_Params() -> cv_return_value_c_SimpleBlobDetector_Params;
}
extern "C" {
#[doc(hidden)] pub fn cv_highgui_cvDestroyAllWindows() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cvStartWindowThread() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cvStopLoop() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cvWaitKey_int_delay(delay: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cv_addText_Mat_img_String_text_Point_org_QtFont_font(img: *mut c_void, text: *const c_char, org: core::Point, font: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_addText_Mat_img_String_text_Point_org_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing(img: *mut c_void, text: *const c_char, org: core::Point, name_font: *const c_char, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_destroyAllWindows() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_destroyWindow_String_winname(winname: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_displayOverlay_String_winname_String_text_int_delayms(winname: *const c_char, text: *const c_char, delayms: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_displayStatusBar_String_winname_String_text_int_delayms(winname: *const c_char, text: *const c_char, delayms: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_fontQt_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing(name_font: *const c_char, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_highgui_cv_getMouseWheelDelta_int_flags(flags: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cv_getTrackbarPos_String_trackbarname_String_winname(trackbarname: *const c_char, winname: *const c_char) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cv_getWindowImageRect_String_winname(winname: *const c_char) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_highgui_cv_getWindowProperty_String_winname_int_prop_id(winname: *const c_char, prop_id: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_highgui_cv_imshow_String_winname_Mat_mat(winname: *const c_char, mat: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_loadWindowParameters_String_windowName(window_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_moveWindow_String_winname_int_x_int_y(winname: *const c_char, x: i32, y: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_namedWindow_String_winname_int_flags(winname: *const c_char, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_resizeWindow_String_winname_int_width_int_height(winname: *const c_char, width: i32, height: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_saveWindowParameters_String_windowName(window_name: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_selectROI_Mat_img_bool_showCrosshair_bool_fromCenter(img: *mut c_void, show_crosshair: bool, from_center: bool) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_highgui_cv_selectROI_String_windowName_Mat_img_bool_showCrosshair_bool_fromCenter(window_name: *const c_char, img: *mut c_void, show_crosshair: bool, from_center: bool) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_highgui_cv_selectROIs_String_windowName_Mat_img_VectorOfRect_boundingBoxes_bool_showCrosshair_bool_fromCenter(window_name: *const c_char, img: *mut c_void, bounding_boxes: *mut c_void, show_crosshair: bool, from_center: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setOpenGlContext_String_winname(winname: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setTrackbarMax_String_trackbarname_String_winname_int_maxval(trackbarname: *const c_char, winname: *const c_char, maxval: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setTrackbarMin_String_trackbarname_String_winname_int_minval(trackbarname: *const c_char, winname: *const c_char, minval: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setTrackbarPos_String_trackbarname_String_winname_int_pos(trackbarname: *const c_char, winname: *const c_char, pos: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setWindowProperty_String_winname_int_prop_id_double_prop_value(winname: *const c_char, prop_id: i32, prop_value: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_setWindowTitle_String_winname_String_title(winname: *const c_char, title: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_startWindowThread() -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cv_stopLoop() -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_updateWindow_String_winname(winname: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_highgui_cv_waitKeyEx_int_delay(delay: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_highgui_cv_waitKey_int_delay(delay: i32) -> cv_return_value_int;
pub fn cv_delete_QtFont(ptr : *mut c_void);
}
extern "C" {
#[doc(hidden)] pub fn cv_imgcodecs_cv_imdecode_Mat_buf_int_flags(buf: *mut c_void, flags: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgcodecs_cv_imdecode_Mat_buf_int_flags_Mat_dst(buf: *mut c_void, flags: i32, dst: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgcodecs_cv_imencode_String_ext_Mat_img_VectorOfuchar_buf_VectorOfint_params(ext: *const c_char, img: *mut c_void, buf: *mut c_void, params: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgcodecs_cv_imread_String_filename_int_flags(filename: *const c_char, flags: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgcodecs_cv_imreadmulti_String_filename_VectorOfMat_mats_int_flags(filename: *const c_char, mats: *mut c_void, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgcodecs_cv_imwrite_String_filename_Mat_img_VectorOfint_params(filename: *const c_char, img: *mut c_void, params: *mut c_void) -> cv_return_value_char;
}
extern "C" {
#[doc(hidden)] pub fn cv_imgproc_cv_Canny_Mat_dx_Mat_dy_Mat_edges_double_threshold1_double_threshold2_bool_L2gradient(dx: *mut c_void, dy: *mut c_void, edges: *mut c_void, threshold1: f64, threshold2: f64, l2gradient: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Canny_Mat_image_Mat_edges_double_threshold1_double_threshold2_int_apertureSize_bool_L2gradient(image: *mut c_void, edges: *mut c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GaussianBlur_Mat_src_Mat_dst_Size_ksize_double_sigmaX_double_sigmaY_int_borderType(src: *mut c_void, dst: *mut c_void, ksize: core::Size, sigma_x: f64, sigma_y: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_HoughCircles_Mat_image_Mat_circles_int_method_double_dp_double_minDist_double_param1_double_param2_int_minRadius_int_maxRadius(image: *mut c_void, circles: *mut c_void, method: i32, dp: f64, min_dist: f64, param1: f64, param2: f64, min_radius: i32, max_radius: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_HoughLinesP_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_minLineLength_double_maxLineGap(image: *mut c_void, lines: *mut c_void, rho: f64, theta: f64, threshold: i32, min_line_length: f64, max_line_gap: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_HoughLinesPointSet_Mat__point_Mat__lines_int_lines_max_int_threshold_double_min_rho_double_max_rho_double_rho_step_double_min_theta_double_max_theta_double_theta_step(_point: *mut c_void, _lines: *mut c_void, lines_max: i32, threshold: i32, min_rho: f64, max_rho: f64, rho_step: f64, min_theta: f64, max_theta: f64, theta_step: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_HoughLines_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_srn_double_stn_double_min_theta_double_max_theta(image: *mut c_void, lines: *mut c_void, rho: f64, theta: f64, threshold: i32, srn: f64, stn: f64, min_theta: f64, max_theta: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Laplacian_Mat_src_Mat_dst_int_ddepth_int_ksize_double_scale_double_delta_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Scharr_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_double_scale_double_delta_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Sobel_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_int_ksize_double_scale_double_delta_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_accumulateProduct_Mat_src1_Mat_src2_Mat_dst_Mat_mask(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_accumulateSquare_Mat_src_Mat_dst_Mat_mask(src: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_accumulateWeighted_Mat_src_Mat_dst_double_alpha_Mat_mask(src: *mut c_void, dst: *mut c_void, alpha: f64, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_accumulate_Mat_src_Mat_dst_Mat_mask(src: *mut c_void, dst: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_adaptiveThreshold_Mat_src_Mat_dst_double_maxValue_int_adaptiveMethod_int_thresholdType_int_blockSize_double_C(src: *mut c_void, dst: *mut c_void, max_value: f64, adaptive_method: i32, threshold_type: i32, block_size: i32, c: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_applyColorMap_Mat_src_Mat_dst_Mat_userColor(src: *mut c_void, dst: *mut c_void, user_color: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_applyColorMap_Mat_src_Mat_dst_int_colormap(src: *mut c_void, dst: *mut c_void, colormap: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_approxPolyDP_Mat_curve_Mat_approxCurve_double_epsilon_bool_closed(curve: *mut c_void, approx_curve: *mut c_void, epsilon: f64, closed: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_arcLength_Mat_curve_bool_closed(curve: *mut c_void, closed: bool) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_arrowedLine_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_line_type_int_shift_double_tipLength(img: *mut c_void, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32, tip_length: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_bilateralFilter_Mat_src_Mat_dst_int_d_double_sigmaColor_double_sigmaSpace_int_borderType(src: *mut c_void, dst: *mut c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_blendLinear_Mat_src1_Mat_src2_Mat_weights1_Mat_weights2_Mat_dst(src1: *mut c_void, src2: *mut c_void, weights1: *mut c_void, weights2: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_blur_Mat_src_Mat_dst_Size_ksize_Point_anchor_int_borderType(src: *mut c_void, dst: *mut c_void, ksize: core::Size, anchor: core::Point, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_boundingRect_Mat_array(array: *mut c_void) -> cv_return_value_c_Rect;
#[doc(hidden)] pub fn cv_imgproc_cv_boxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_boxPoints_RotatedRect_box_Mat_points(_box: *mut c_void, points: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_buildPyramid_Mat_src_VectorOfMat_dst_int_maxlevel_int_borderType(src: *mut c_void, dst: *mut c_void, maxlevel: i32, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_calcBackProject_VectorOfMat_images_VectorOfint_channels_Mat_hist_Mat_dst_VectorOffloat_ranges_double_scale(images: *mut c_void, channels: *mut c_void, hist: *mut c_void, dst: *mut c_void, ranges: *mut c_void, scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_calcHist_VectorOfMat_images_VectorOfint_channels_Mat_mask_Mat_hist_VectorOfint_histSize_VectorOffloat_ranges_bool_accumulate(images: *mut c_void, channels: *mut c_void, mask: *mut c_void, hist: *mut c_void, hist_size: *mut c_void, ranges: *mut c_void, accumulate: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_circle_Mat_img_Point_center_int_radius_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, center: core::Point, radius: i32, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_clipLine_Rect_imgRect_Point_pt1_Point_pt2(img_rect: core::Rect, pt1: core::Point, pt2: core::Point) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgproc_cv_clipLine_Size2l_imgSize_Point2l_pt1_Point2l_pt2(img_size: core::Size2l, pt1: core::Point2l, pt2: core::Point2l) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgproc_cv_clipLine_Size_imgSize_Point_pt1_Point_pt2(img_size: core::Size, pt1: core::Point, pt2: core::Point) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgproc_cv_compareHist_Mat_H1_Mat_H2_int_method(h1: *mut c_void, h2: *mut c_void, method: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype(image: *mut c_void, labels: *mut c_void, stats: *mut c_void, centroids: *mut c_void, connectivity: i32, ltype: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype_int_ccltype(image: *mut c_void, labels: *mut c_void, stats: *mut c_void, centroids: *mut c_void, connectivity: i32, ltype: i32, ccltype: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype(image: *mut c_void, labels: *mut c_void, connectivity: i32, ltype: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype_int_ccltype(image: *mut c_void, labels: *mut c_void, connectivity: i32, ltype: i32, ccltype: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_contourArea_Mat_contour_bool_oriented(contour: *mut c_void, oriented: bool) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_convertMaps_Mat_map1_Mat_map2_Mat_dstmap1_Mat_dstmap2_int_dstmap1type_bool_nninterpolation(map1: *mut c_void, map2: *mut c_void, dstmap1: *mut c_void, dstmap2: *mut c_void, dstmap1type: i32, nninterpolation: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_convexHull_Mat_points_Mat_hull_bool_clockwise_bool_returnPoints(points: *mut c_void, hull: *mut c_void, clockwise: bool, return_points: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_convexityDefects_Mat_contour_Mat_convexhull_Mat_convexityDefects(contour: *mut c_void, convexhull: *mut c_void, convexity_defects: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_cornerEigenValsAndVecs_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType(src: *mut c_void, dst: *mut c_void, block_size: i32, ksize: i32, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_cornerHarris_Mat_src_Mat_dst_int_blockSize_int_ksize_double_k_int_borderType(src: *mut c_void, dst: *mut c_void, block_size: i32, ksize: i32, k: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_cornerMinEigenVal_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType(src: *mut c_void, dst: *mut c_void, block_size: i32, ksize: i32, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_cornerSubPix_Mat_image_Mat_corners_Size_winSize_Size_zeroZone_TermCriteria_criteria(image: *mut c_void, corners: *mut c_void, win_size: core::Size, zero_zone: core::Size, criteria: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_createCLAHE_double_clipLimit_Size_tileGridSize(clip_limit: f64, tile_grid_size: core::Size) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_createGeneralizedHoughBallard() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_createGeneralizedHoughGuil() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_createHanningWindow_Mat_dst_Size_winSize_int_type(dst: *mut c_void, win_size: core::Size, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_createLineSegmentDetector_int__refine_double__scale_double__sigma_scale_double__quant_double__ang_th_double__log_eps_double__density_th_int__n_bins(_refine: i32, _scale: f64, _sigma_scale: f64, _quant: f64, _ang_th: f64, _log_eps: f64, _density_th: f64, _n_bins: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_cvtColorTwoPlane_Mat_src1_Mat_src2_Mat_dst_int_code(src1: *mut c_void, src2: *mut c_void, dst: *mut c_void, code: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_cvtColor_Mat_src_Mat_dst_int_code_int_dstCn(src: *mut c_void, dst: *mut c_void, code: i32, dst_cn: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_demosaicing_Mat_src_Mat_dst_int_code_int_dstCn(src: *mut c_void, dst: *mut c_void, code: i32, dst_cn: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_dilate_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, kernel: *mut c_void, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_distanceTransform_Mat_src_Mat_dst_Mat_labels_int_distanceType_int_maskSize_int_labelType(src: *mut c_void, dst: *mut c_void, labels: *mut c_void, distance_type: i32, mask_size: i32, label_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_distanceTransform_Mat_src_Mat_dst_int_distanceType_int_maskSize_int_dstType(src: *mut c_void, dst: *mut c_void, distance_type: i32, mask_size: i32, dst_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_drawContours_Mat_image_VectorOfMat_contours_int_contourIdx_Scalar_color_int_thickness_int_lineType_Mat_hierarchy_int_maxLevel_Point_offset(image: *mut c_void, contours: *mut c_void, contour_idx: i32, color: core::Scalar, thickness: i32, line_type: i32, hierarchy: *mut c_void, max_level: i32, offset: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_drawMarker_Mat_img_Point_position_Scalar_color_int_markerType_int_markerSize_int_thickness_int_line_type(img: *mut c_void, position: core::Point, color: core::Scalar, marker_type: i32, marker_size: i32, thickness: i32, line_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_ellipse2Poly_Point2d_center_Size2d_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint2d_pts(center: core::Point2d, axes: core::Size2d, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_ellipse2Poly_Point_center_Size_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint_pts(center: core::Point, axes: core::Size, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_ellipse_Mat_img_Point_center_Size_axes_double_angle_double_startAngle_double_endAngle_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, center: core::Point, axes: core::Size, angle: f64, start_angle: f64, end_angle: f64, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_ellipse_Mat_img_RotatedRect_box_Scalar_color_int_thickness_int_lineType(img: *mut c_void, _box: *mut c_void, color: core::Scalar, thickness: i32, line_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_equalizeHist_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_erode_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, kernel: *mut c_void, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_fillConvexPoly_Mat_img_Mat_points_Scalar_color_int_lineType_int_shift(img: *mut c_void, points: *mut c_void, color: core::Scalar, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_fillPoly_Mat_img_VectorOfMat_pts_Scalar_color_int_lineType_int_shift_Point_offset(img: *mut c_void, pts: *mut c_void, color: core::Scalar, line_type: i32, shift: i32, offset: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_filter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernel_Point_anchor_double_delta_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, kernel: *mut c_void, anchor: core::Point, delta: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_findContours_Mat_image_VectorOfMat_contours_Mat_hierarchy_int_mode_int_method_Point_offset(image: *mut c_void, contours: *mut c_void, hierarchy: *mut c_void, mode: i32, method: i32, offset: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_findContours_Mat_image_VectorOfMat_contours_int_mode_int_method_Point_offset(image: *mut c_void, contours: *mut c_void, mode: i32, method: i32, offset: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_fitEllipseAMS_Mat_points(points: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_fitEllipseDirect_Mat_points(points: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_fitEllipse_Mat_points(points: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_fitLine_Mat_points_Mat_line_int_distType_double_param_double_reps_double_aeps(points: *mut c_void, line: *mut c_void, dist_type: i32, param: f64, reps: f64, aeps: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_getAffineTransform_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getDefaultNewCameraMatrix_Mat_cameraMatrix_Size_imgsize_bool_centerPrincipalPoint(camera_matrix: *mut c_void, imgsize: core::Size, center_principal_point: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getDerivKernels_Mat_kx_Mat_ky_int_dx_int_dy_int_ksize_bool_normalize_int_ktype(kx: *mut c_void, ky: *mut c_void, dx: i32, dy: i32, ksize: i32, normalize: bool, ktype: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_getFontScaleFromHeight_int_fontFace_int_pixelHeight_int_thickness(font_face: i32, pixel_height: i32, thickness: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_getGaborKernel_Size_ksize_double_sigma_double_theta_double_lambd_double_gamma_double_psi_int_ktype(ksize: core::Size, sigma: f64, theta: f64, lambd: f64, gamma: f64, psi: f64, ktype: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getGaussianKernel_int_ksize_double_sigma_int_ktype(ksize: i32, sigma: f64, ktype: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getPerspectiveTransform_Mat_src_Mat_dst(src: *mut c_void, dst: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getRectSubPix_Mat_image_Size_patchSize_Point2f_center_Mat_patch_int_patchType(image: *mut c_void, patch_size: core::Size, center: core::Point2f, patch: *mut c_void, patch_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_getRotationMatrix2D_Point2f_center_double_angle_double_scale(center: core::Point2f, angle: f64, scale: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_getStructuringElement_int_shape_Size_ksize_Point_anchor(shape: i32, ksize: core::Size, anchor: core::Point) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_bool_useHarrisDetector_double_k(image: *mut c_void, corners: *mut c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *mut c_void, block_size: i32, use_harris_detector: bool, k: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_int_gradientSize_bool_useHarrisDetector_double_k(image: *mut c_void, corners: *mut c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *mut c_void, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_grabCut_Mat_img_Mat_mask_Rect_rect_Mat_bgdModel_Mat_fgdModel_int_iterCount_int_mode(img: *mut c_void, mask: *mut c_void, rect: core::Rect, bgd_model: *mut c_void, fgd_model: *mut c_void, iter_count: i32, mode: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_initUndistortRectifyMap_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_newCameraMatrix_Size_size_int_m1type_Mat_map1_Mat_map2(camera_matrix: *mut c_void, dist_coeffs: *mut c_void, r: *mut c_void, new_camera_matrix: *mut c_void, size: core::Size, m1type: i32, map1: *mut c_void, map2: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_initWideAngleProjMap_Mat_cameraMatrix_Mat_distCoeffs_Size_imageSize_int_destImageWidth_int_m1type_Mat_map1_Mat_map2_int_projType_double_alpha(camera_matrix: *mut c_void, dist_coeffs: *mut c_void, image_size: core::Size, dest_image_width: i32, m1type: i32, map1: *mut c_void, map2: *mut c_void, proj_type: i32, alpha: f64) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_imgproc_cv_integral_Mat_src_Mat_sum_Mat_sqsum_Mat_tilted_int_sdepth_int_sqdepth(src: *mut c_void, sum: *mut c_void, sqsum: *mut c_void, tilted: *mut c_void, sdepth: i32, sqdepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_integral_Mat_src_Mat_sum_Mat_sqsum_int_sdepth_int_sqdepth(src: *mut c_void, sum: *mut c_void, sqsum: *mut c_void, sdepth: i32, sqdepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_integral_Mat_src_Mat_sum_int_sdepth(src: *mut c_void, sum: *mut c_void, sdepth: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_intersectConvexConvex_Mat__p1_Mat__p2_Mat__p12_bool_handleNested(_p1: *mut c_void, _p2: *mut c_void, _p12: *mut c_void, handle_nested: bool) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_imgproc_cv_invertAffineTransform_Mat_M_Mat_iM(m: *mut c_void, i_m: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_isContourConvex_Mat_contour(contour: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_imgproc_cv_line_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_linearPolar_Mat_src_Mat_dst_Point2f_center_double_maxRadius_int_flags(src: *mut c_void, dst: *mut c_void, center: core::Point2f, max_radius: f64, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_logPolar_Mat_src_Mat_dst_Point2f_center_double_M_int_flags(src: *mut c_void, dst: *mut c_void, center: core::Point2f, m: f64, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_matchShapes_Mat_contour1_Mat_contour2_int_method_double_parameter(contour1: *mut c_void, contour2: *mut c_void, method: i32, parameter: f64) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_matchTemplate_Mat_image_Mat_templ_Mat_result_int_method_Mat_mask(image: *mut c_void, templ: *mut c_void, result: *mut c_void, method: i32, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_medianBlur_Mat_src_Mat_dst_int_ksize(src: *mut c_void, dst: *mut c_void, ksize: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_minAreaRect_Mat_points(points: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_minEnclosingCircle_Mat_points_Point2f_center_float_radius(points: *mut c_void, center: core::Point2f, radius: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_minEnclosingTriangle_Mat_points_Mat_triangle(points: *mut c_void, triangle: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_morphologyDefaultBorderValue() -> cv_return_value_c_Scalar;
#[doc(hidden)] pub fn cv_imgproc_cv_morphologyEx_Mat_src_Mat_dst_int_op_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, op: i32, kernel: *mut c_void, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_pointPolygonTest_Mat_contour_Point2f_pt_bool_measureDist(contour: *mut c_void, pt: core::Point2f, measure_dist: bool) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_polylines_Mat_img_VectorOfMat_pts_bool_isClosed_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, pts: *mut c_void, is_closed: bool, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_preCornerDetect_Mat_src_Mat_dst_int_ksize_int_borderType(src: *mut c_void, dst: *mut c_void, ksize: i32, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_putText_Mat_img_String_text_Point_org_int_fontFace_double_fontScale_Scalar_color_int_thickness_int_lineType_bool_bottomLeftOrigin(img: *mut c_void, text: *const c_char, org: core::Point, font_face: i32, font_scale: f64, color: core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_pyrDown_Mat_src_Mat_dst_Size_dstsize_int_borderType(src: *mut c_void, dst: *mut c_void, dstsize: core::Size, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_pyrMeanShiftFiltering_Mat_src_Mat_dst_double_sp_double_sr_int_maxLevel_TermCriteria_termcrit(src: *mut c_void, dst: *mut c_void, sp: f64, sr: f64, max_level: i32, termcrit: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_pyrUp_Mat_src_Mat_dst_Size_dstsize_int_borderType(src: *mut c_void, dst: *mut c_void, dstsize: core::Size, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_rectangle_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_rectangle_Mat_img_Rect_rec_Scalar_color_int_thickness_int_lineType_int_shift(img: *mut c_void, rec: core::Rect, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_remap_Mat_src_Mat_dst_Mat_map1_Mat_map2_int_interpolation_int_borderMode_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, map1: *mut c_void, map2: *mut c_void, interpolation: i32, border_mode: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_resize_Mat_src_Mat_dst_Size_dsize_double_fx_double_fy_int_interpolation(src: *mut c_void, dst: *mut c_void, dsize: core::Size, fx: f64, fy: f64, interpolation: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_rotatedRectangleIntersection_RotatedRect_rect1_RotatedRect_rect2_Mat_intersectingRegion(rect1: *mut c_void, rect2: *mut c_void, intersecting_region: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_sepFilter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernelX_Mat_kernelY_Point_anchor_double_delta_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, kernel_x: *mut c_void, kernel_y: *mut c_void, anchor: core::Point, delta: f64, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_spatialGradient_Mat_src_Mat_dx_Mat_dy_int_ksize_int_borderType(src: *mut c_void, dx: *mut c_void, dy: *mut c_void, ksize: i32, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_sqrBoxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType(src: *mut c_void, dst: *mut c_void, ddepth: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_threshold_Mat_src_Mat_dst_double_thresh_double_maxval_int_type(src: *mut c_void, dst: *mut c_void, thresh: f64, maxval: f64, _type: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P(src: *mut c_void, dst: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, r: *mut c_void, p: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P_TermCriteria_criteria(src: *mut c_void, dst: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, r: *mut c_void, p: *mut c_void, criteria: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_undistort_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_newCameraMatrix(src: *mut c_void, dst: *mut c_void, camera_matrix: *mut c_void, dist_coeffs: *mut c_void, new_camera_matrix: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_warpAffine_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, m: *mut c_void, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_warpPerspective_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue(src: *mut c_void, dst: *mut c_void, m: *mut c_void, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_warpPolar_Mat_src_Mat_dst_Size_dsize_Point2f_center_double_maxRadius_int_flags(src: *mut c_void, dst: *mut c_void, dsize: core::Size, center: core::Point2f, max_radius: f64, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_watershed_Mat_image_Mat_markers(image: *mut c_void, markers: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_wrapperEMD_Mat_signature1_Mat_signature2_int_distType_Mat_cost_PtrOffloat_lowerBound_Mat_flow(signature1: *mut c_void, signature2: *mut c_void, dist_type: i32, cost: *mut c_void, lower_bound: *mut c_void, flow: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_apply_Mat_src_Mat_dst(instance: *mut c_void, src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_setClipLimit_double_clipLimit(instance: *mut c_void, clip_limit: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_getClipLimit(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_setTilesGridSize_Size_tileGridSize(instance: *mut c_void, tile_grid_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_getTilesGridSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_imgproc_cv_CLAHE_collectGarbage(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setTemplate_Mat_templ_Point_templCenter(instance: *mut c_void, templ: *mut c_void, templ_center: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setTemplate_Mat_edges_Mat_dx_Mat_dy_Point_templCenter(instance: *mut c_void, edges: *mut c_void, dx: *mut c_void, dy: *mut c_void, templ_center: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_detect_Mat_image_Mat_positions_Mat_votes(instance: *mut c_void, image: *mut c_void, positions: *mut c_void, votes: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_detect_Mat_edges_Mat_dx_Mat_dy_Mat_positions_Mat_votes(instance: *mut c_void, edges: *mut c_void, dx: *mut c_void, dy: *mut c_void, positions: *mut c_void, votes: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setCannyLowThresh_int_cannyLowThresh(instance: *mut c_void, canny_low_thresh: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_getCannyLowThresh(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setCannyHighThresh_int_cannyHighThresh(instance: *mut c_void, canny_high_thresh: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_getCannyHighThresh(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setMinDist_double_minDist(instance: *mut c_void, min_dist: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_getMinDist(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setDp_double_dp(instance: *mut c_void, dp: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_getDp(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_setMaxBufferSize_int_maxBufferSize(instance: *mut c_void, max_buffer_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHough_getMaxBufferSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughBallard_setLevels_int_levels(instance: *mut c_void, levels: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughBallard_getLevels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughBallard_setVotesThreshold_int_votesThreshold(instance: *mut c_void, votes_threshold: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughBallard_getVotesThreshold(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setXi_double_xi(instance: *mut c_void, xi: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getXi(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setLevels_int_levels(instance: *mut c_void, levels: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getLevels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setAngleEpsilon_double_angleEpsilon(instance: *mut c_void, angle_epsilon: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getAngleEpsilon(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setMinAngle_double_minAngle(instance: *mut c_void, min_angle: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getMinAngle(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setMaxAngle_double_maxAngle(instance: *mut c_void, max_angle: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getMaxAngle(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setAngleStep_double_angleStep(instance: *mut c_void, angle_step: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getAngleStep(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setAngleThresh_int_angleThresh(instance: *mut c_void, angle_thresh: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getAngleThresh(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setMinScale_double_minScale(instance: *mut c_void, min_scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getMinScale(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setMaxScale_double_maxScale(instance: *mut c_void, max_scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getMaxScale(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setScaleStep_double_scaleStep(instance: *mut c_void, scale_step: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getScaleStep(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setScaleThresh_int_scaleThresh(instance: *mut c_void, scale_thresh: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getScaleThresh(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_setPosThresh_int_posThresh(instance: *mut c_void, pos_thresh: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_GeneralizedHoughGuil_getPosThresh(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_LineIterator(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_imgproc_cv_LineIterator_LineIterator_Mat_img_Point_pt1_Point_pt2_int_connectivity_bool_leftToRight(img: *mut c_void, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_LineIterator_pos(instance: *const c_void) -> cv_return_value_c_Point;
#[doc(hidden)] pub fn cv_imgproc_cv_LineSegmentDetector_detect_Mat__image_Mat__lines_Mat_width_Mat_prec_Mat_nfa(instance: *mut c_void, _image: *mut c_void, _lines: *mut c_void, width: *mut c_void, prec: *mut c_void, nfa: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_LineSegmentDetector_drawSegments_Mat__image_Mat_lines(instance: *mut c_void, _image: *mut c_void, lines: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_LineSegmentDetector_compareSegments_Size_size_Mat_lines1_Mat_lines2_Mat__image(instance: *mut c_void, size: core::Size, lines1: *mut c_void, lines2: *mut c_void, _image: *mut c_void) -> cv_return_value_int;
pub fn cv_delete_Subdiv2D(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_Subdiv2D() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_Subdiv2D_Rect_rect(rect: core::Rect) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_initDelaunay_Rect_rect(instance: *mut c_void, rect: core::Rect) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_insert_Point2f_pt(instance: *mut c_void, pt: core::Point2f) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_insert_VectorOfPoint2f_ptvec(instance: *mut c_void, ptvec: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_locate_Point2f_pt_int_edge_int_vertex(instance: *mut c_void, pt: core::Point2f, edge: i32, vertex: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_getEdgeList_VectorOfVec4f_edgeList(instance: *const c_void, edge_list: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_getLeadingEdgeList_VectorOfint_leadingEdgeList(instance: *const c_void, leading_edge_list: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_getTriangleList_VectorOfVec6f_triangleList(instance: *const c_void, triangle_list: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_getVoronoiFacetList_VectorOfint_idx_VectorOfVectorOfPoint2f_facetList_VectorOfPoint2f_facetCenters(instance: *mut c_void, idx: *mut c_void, facet_list: *mut c_void, facet_centers: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_getEdge_int_edge_int_nextEdgeType(instance: *const c_void, edge: i32, next_edge_type: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_nextEdge_int_edge(instance: *const c_void, edge: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_rotateEdge_int_edge_int_rotate(instance: *const c_void, edge: i32, rotate: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_imgproc_cv_Subdiv2D_symEdge_int_edge(instance: *const c_void, edge: i32) -> cv_return_value_int;
}
extern "C" {
#[doc(hidden)] pub fn cv_ml_cv_ml_createConcentricSpheresTestSet_int_nsamples_int_nfeatures_int_nclasses_Mat_samples_Mat_responses(nsamples: i32, nfeatures: i32, nclasses: i32, samples: *mut c_void, responses: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_randMVNormal_Mat_mean_Mat_cov_int_nsamples_Mat_samples(mean: *mut c_void, cov: *mut c_void, nsamples: i32, samples: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setTrainMethod_int_method_double_param1_double_param2(instance: *mut c_void, method: i32, param1: f64, param2: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getTrainMethod(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setActivationFunction_int_type_double_param1_double_param2(instance: *mut c_void, _type: i32, param1: f64, param2: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setLayerSizes_Mat__layer_sizes(instance: *mut c_void, _layer_sizes: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getLayerSizes(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getBackpropWeightScale(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setBackpropWeightScale_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getBackpropMomentumScale(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setBackpropMomentumScale_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getRpropDW0(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setRpropDW0_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getRpropDWPlus(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setRpropDWPlus_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getRpropDWMinus(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setRpropDWMinus_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getRpropDWMin(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setRpropDWMin_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getRpropDWMax(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setRpropDWMax_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getAnnealInitialT(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setAnnealInitialT_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getAnnealFinalT(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setAnnealFinalT_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getAnnealCoolingRatio(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setAnnealCoolingRatio_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getAnnealItePerStep(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_setAnnealItePerStep_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_getWeights_int_layerIdx(instance: *const c_void, layer_idx: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_load_String_filepath(filepath: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealInitialT(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealInitialT_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealFinalT(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealFinalT_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealCoolingRatio(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealCoolingRatio_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealItePerStep(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealItePerStep_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_getBoostType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_setBoostType_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_getWeakCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_setWeakCount_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_getWeightTrimRate(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_setWeightTrimRate_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_Boost_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getMaxCategories(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setMaxCategories_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getMaxDepth(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setMaxDepth_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getMinSampleCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setMinSampleCount_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getCVFolds(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setCVFolds_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getUseSurrogates(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setUseSurrogates_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getUse1SERule(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setUse1SERule_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getTruncatePrunedTree(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setTruncatePrunedTree_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getRegressionAccuracy(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setRegressionAccuracy_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getPriors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_setPriors_Mat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getRoots(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getNodes(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getSplits(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_getSubsets(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
pub fn cv_delete_Node(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_Node_Node() -> cv_return_value_void_X;
pub fn cv_delete_Split(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_ml_cv_ml_DTrees_Split_Split() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getClustersNumber(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_setClustersNumber_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getCovarianceMatrixType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_setCovarianceMatrixType_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getWeights(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getMeans(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_getCovs_VectorOfMat_covs(instance: *const c_void, covs: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_predict_Mat_samples_Mat_results_int_flags(instance: *const c_void, samples: *mut c_void, results: *mut c_void, flags: i32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_predict2_Mat_sample_Mat_probs(instance: *const c_void, sample: *mut c_void, probs: *mut c_void) -> cv_return_value_c_Vec2d;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_trainEM_Mat_samples_Mat_logLikelihoods_Mat_labels_Mat_probs(instance: *mut c_void, samples: *mut c_void, log_likelihoods: *mut c_void, labels: *mut c_void, probs: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_trainE_Mat_samples_Mat_means0_Mat_covs0_Mat_weights0_Mat_logLikelihoods_Mat_labels_Mat_probs(instance: *mut c_void, samples: *mut c_void, means0: *mut c_void, covs0: *mut c_void, weights0: *mut c_void, log_likelihoods: *mut c_void, labels: *mut c_void, probs: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_trainM_Mat_samples_Mat_probs0_Mat_logLikelihoods_Mat_labels_Mat_probs(instance: *mut c_void, samples: *mut c_void, probs0: *mut c_void, log_likelihoods: *mut c_void, labels: *mut c_void, probs: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_EM_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_getDefaultK(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_setDefaultK_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_getIsClassifier(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_setIsClassifier_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_getEmax(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_setEmax_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_getAlgorithmType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_setAlgorithmType_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_findNearest_Mat_samples_int_k_Mat_results_Mat_neighborResponses_Mat_dist(instance: *const c_void, samples: *mut c_void, k: i32, results: *mut c_void, neighbor_responses: *mut c_void, dist: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_KNearest_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getLearningRate(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setLearningRate_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getIterations(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setIterations_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getRegularization(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setRegularization_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getTrainMethod(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setTrainMethod_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getMiniBatchSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setMiniBatchSize_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_predict_Mat_samples_Mat_results_int_flags(instance: *const c_void, samples: *mut c_void, results: *mut c_void, flags: i32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_get_learnt_thetas(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_LogisticRegression_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_NormalBayesClassifier_predictProb_Mat_inputs_Mat_outputs_Mat_outputProbs_int_flags(instance: *const c_void, inputs: *mut c_void, outputs: *mut c_void, output_probs: *mut c_void, flags: i32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_NormalBayesClassifier_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_NormalBayesClassifier_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
pub fn cv_delete_ParamGrid(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_ml_cv_ml_ParamGrid_ParamGrid() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ParamGrid_ParamGrid_double__minVal_double__maxVal_double__logStep(_min_val: f64, _max_val: f64, _log_step: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_ParamGrid_create_double_minVal_double_maxVal_double_logstep(min_val: f64, max_val: f64, logstep: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_getCalculateVarImportance(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_setCalculateVarImportance_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_getActiveVarCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_setActiveVarCount_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_getVarImportance(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_getVotes_Mat_samples_Mat_results_int_flags(instance: *const c_void, samples: *mut c_void, results: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_RTrees_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setType_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getGamma(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setGamma_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getCoef0(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setCoef0_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getDegree(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setDegree_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getC(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setC_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getNu(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setNu_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getP(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setP_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getClassWeights(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setClassWeights_Mat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getKernelType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setKernel_int_kernelType(instance: *mut c_void, kernel_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_setCustomKernel_PtrOfKernel__kernel(instance: *mut c_void, _kernel: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_trainAuto_PtrOfTrainData_data_int_kFold_ParamGrid_Cgrid_ParamGrid_gammaGrid_ParamGrid_pGrid_ParamGrid_nuGrid_ParamGrid_coeffGrid_ParamGrid_degreeGrid_bool_balanced(instance: *mut c_void, data: *mut c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_trainAuto_Mat_samples_int_layout_Mat_responses_int_kFold_PtrOfParamGrid_Cgrid_PtrOfParamGrid_gammaGrid_PtrOfParamGrid_pGrid_PtrOfParamGrid_nuGrid_PtrOfParamGrid_coeffGrid_PtrOfParamGrid_degreeGrid_bool_balanced(instance: *mut c_void, samples: *mut c_void, layout: i32, responses: *mut c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getSupportVectors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getUncompressedSupportVectors(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getDecisionFunction_int_i_Mat_alpha_Mat_svidx(instance: *const c_void, i: i32, alpha: *mut c_void, svidx: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getDefaultGrid_int_param_id(param_id: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_getDefaultGridPtr_int_param_id(param_id: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_load_String_filepath(filepath: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVM_Kernel_getType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getWeights(instance: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getShift(instance: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setOptimalParameters_int_svmsgdType_int_marginType(instance: *mut c_void, svmsgd_type: i32, margin_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getSvmsgdType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setSvmsgdType_int_svmsgdType(instance: *mut c_void, svmsgd_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getMarginType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setMarginType_int_marginType(instance: *mut c_void, margin_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getMarginRegularization(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setMarginRegularization_float_marginRegularization(instance: *mut c_void, margin_regularization: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getInitialStepSize(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setInitialStepSize_float_InitialStepSize(instance: *mut c_void, initial_step_size: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getStepDecreasingPower(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setStepDecreasingPower_float_stepDecreasingPower(instance: *mut c_void, step_decreasing_power: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_setTermCriteria_TermCriteria_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_create() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_SVMSGD_load_String_filepath_String_nodeName(filepath: *const c_char, node_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_getVarCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_isTrained(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_isClassifier(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_train_PtrOfTrainData_trainData_int_flags(instance: *mut c_void, train_data: *mut c_void, flags: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_train_Mat_samples_int_layout_Mat_responses(instance: *mut c_void, samples: *mut c_void, layout: i32, responses: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_calcError_PtrOfTrainData_data_bool_test_Mat_resp(instance: *const c_void, data: *mut c_void, test: bool, resp: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_StatModel_predict_Mat_samples_Mat_results_int_flags(instance: *const c_void, samples: *mut c_void, results: *mut c_void, flags: i32) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_missingValue(instance: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getLayout(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNTrainSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNTestSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNVars(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNAllVars(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getSamples(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getMissing(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTrainSamples_int_layout_bool_compressSamples_bool_compressVars(instance: *const c_void, layout: i32, compress_samples: bool, compress_vars: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTrainResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTrainNormCatResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTestResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTestNormCatResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNormCatResponses(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getSampleWeights(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTrainSampleWeights(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTestSampleWeights(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getVarIdx(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getVarType(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getVarSymbolFlags(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getResponseType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTrainSampleIdx(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTestSampleIdx(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getDefaultSubstValues(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getCatCount_int_vi(instance: *const c_void, vi: i32) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getClassLabels(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getCatOfs(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getCatMap(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_setTrainTestSplit_int_count_bool_shuffle(instance: *mut c_void, count: i32, shuffle: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_setTrainTestSplitRatio_double_ratio_bool_shuffle(instance: *mut c_void, ratio: f64, shuffle: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_shuffleTrainTest(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getTestSamples(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getNames_VectorOfString_names(instance: *const c_void, names: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getSubVector_Mat_vec_Mat_idx(vec: *mut c_void, idx: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_getSubMatrix_Mat_matrix_Mat_idx_int_layout(matrix: *mut c_void, idx: *mut c_void, layout: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_ml_cv_ml_TrainData_create_Mat_samples_int_layout_Mat_responses_Mat_varIdx_Mat_sampleIdx_Mat_sampleWeights_Mat_varType(samples: *mut c_void, layout: i32, responses: *mut c_void, var_idx: *mut c_void, sample_idx: *mut c_void, sample_weights: *mut c_void, var_type: *mut c_void) -> cv_return_value_void_X;
}
extern "C" {
#[doc(hidden)] pub fn cv_objdetect_cv_createFaceDetectionMaskGenerator() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_detectQRCode_Mat_in_VectorOfPoint_points_double_eps_x_double_eps_y(_in: *mut c_void, points: *mut c_void, eps_x: f64, eps_y: f64) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_int_groupThreshold_double_eps(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, eps: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_VectorOfint_weights_int_groupThreshold_double_eps(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps(rect_list: *mut c_void, group_threshold: i32, eps: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps_VectorOfint_weights_VectorOfdouble_levelWeights(rect_list: *mut c_void, group_threshold: i32, eps: f64, weights: *mut c_void, level_weights: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_groupRectangles_meanshift_VectorOfRect_rectList_VectorOfdouble_foundWeights_VectorOfdouble_foundScales_double_detectThreshold_Size_winDetSize(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, detect_threshold: f64, win_det_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_load_String_filename(instance: *mut c_void, filename: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_isOldFormatCascade(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_getOriginalWindowSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_getFeatureType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator(instance: *mut c_void, mask_generator: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_getMaskGenerator(instance: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_MaskGenerator_generateMask_Mat_src(instance: *mut c_void, src: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_BaseCascadeClassifier_MaskGenerator_initializeMask_Mat_unnamed_arg(instance: *mut c_void, unnamed_arg: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_CascadeClassifier(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_CascadeClassifier() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_CascadeClassifier_String_filename(filename: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_load_String_filename(instance: *mut c_void, filename: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_isOldFormatCascade(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_getOriginalWindowSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_getFeatureType(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_convert_String_oldcascade_String_newcascade(oldcascade: *const c_char, newcascade: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_setMaskGenerator_PtrOfMaskGenerator_maskGenerator(instance: *mut c_void, mask_generator: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_CascadeClassifier_getMaskGenerator(instance: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_DetectionBasedTracker(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_run(instance: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_stop(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_resetTracking(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_process_Mat_imageGray(instance: *mut c_void, image_gray: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_setParameters_Parameters_params(instance: *mut c_void, params: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_getParameters(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_getObjects_VectorOfExtObject_result(instance: *const c_void, result: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_ExtObject(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_IDetector_getScaleFactor(instance: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_IDetector_setScaleFactor_float_value(instance: *mut c_void, value: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_IDetector_getMinNeighbours(instance: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_IDetector_setMinNeighbours_int_value(instance: *mut c_void, value: i32) -> cv_return_value_void;
pub fn cv_delete_Parameters(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_DetectionBasedTracker_Parameters_Parameters() -> cv_return_value_void_X;
pub fn cv_delete_DetectionROI(ptr : *mut c_void);
pub fn cv_delete_HOGDescriptor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_HOGDescriptor_String_filename(filename: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_HOGDescriptor_HOGDescriptor_d(d: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_getDescriptorSize(instance: *const c_void) -> cv_return_value_std_size_t;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_checkDetectorSize(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_getWinSigma(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_setSVMDetector_Mat__svmdetector(instance: *mut c_void, _svmdetector: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_load_String_filename_String_objname(instance: *mut c_void, filename: *const c_char, objname: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_save_String_filename_String_objname(instance: *const c_void, filename: *const c_char, objname: *const c_char) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_copyTo_HOGDescriptor_c(instance: *const c_void, c: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_compute_Mat_img_VectorOffloat_descriptors_Size_winStride_Size_padding_VectorOfPoint_locations(instance: *const c_void, img: *mut c_void, descriptors: *mut c_void, win_stride: core::Size, padding: core::Size, locations: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_detect_Mat_img_VectorOfPoint_foundLocations_VectorOfdouble_weights_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations(instance: *const c_void, img: *mut c_void, found_locations: *mut c_void, weights: *mut c_void, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_detect_Mat_img_VectorOfPoint_foundLocations_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations(instance: *const c_void, img: *mut c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_detectMultiScale_Mat_img_VectorOfRect_foundLocations_VectorOfdouble_foundWeights_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping(instance: *const c_void, img: *mut c_void, found_locations: *mut c_void, found_weights: *mut c_void, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_detectMultiScale_Mat_img_VectorOfRect_foundLocations_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping(instance: *const c_void, img: *mut c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_computeGradient_Mat_img_Mat_grad_Mat_angleOfs_Size_paddingTL_Size_paddingBR(instance: *const c_void, img: *mut c_void, grad: *mut c_void, angle_ofs: *mut c_void, padding_tl: core::Size, padding_br: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_getDefaultPeopleDetector() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_getDaimlerPeopleDetector() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_HOGDescriptor_readALTModel_String_modelfile(instance: *mut c_void, modelfile: *const c_char) -> cv_return_value_void;
pub fn cv_delete_QRCodeDetector(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_objdetect_cv_QRCodeDetector_QRCodeDetector() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_objdetect_cv_QRCodeDetector_setEpsX_double_epsX(instance: *mut c_void, eps_x: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_QRCodeDetector_setEpsY_double_epsY(instance: *mut c_void, eps_y: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_objdetect_cv_QRCodeDetector_detect_Mat_img_Mat_points(instance: *const c_void, img: *mut c_void, points: *mut c_void) -> cv_return_value_char;
pub fn cv_delete_SimilarRects(ptr : *mut c_void);
}
extern "C" {
#[doc(hidden)] pub fn cv_photo_cv_colorChange_Mat_src_Mat_mask_Mat_dst_float_red_mul_float_green_mul_float_blue_mul(src: *mut c_void, mask: *mut c_void, dst: *mut c_void, red_mul: f32, green_mul: f32, blue_mul: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_createAlignMTB_int_max_bits_int_exclude_range_bool_cut(max_bits: i32, exclude_range: i32, cut: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createCalibrateDebevec_int_samples_float_lambda_bool_random(samples: i32, lambda: f32, random: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createCalibrateRobertson_int_max_iter_float_threshold(max_iter: i32, threshold: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createMergeDebevec() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createMergeMertens_float_contrast_weight_float_saturation_weight_float_exposure_weight(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createMergeRobertson() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createTonemapDrago_float_gamma_float_saturation_float_bias(gamma: f32, saturation: f32, bias: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createTonemapMantiuk_float_gamma_float_scale_float_saturation(gamma: f32, scale: f32, saturation: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createTonemapReinhard_float_gamma_float_intensity_float_light_adapt_float_color_adapt(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_createTonemap_float_gamma(gamma: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_decolor_Mat_src_Mat_grayscale_Mat_color_boost(src: *mut c_void, grayscale: *mut c_void, color_boost: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_denoise_TVL1_VectorOfMat_observations_Mat_result_double_lambda_int_niters(observations: *mut c_void, result: *mut c_void, lambda: f64, niters: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_detailEnhance_Mat_src_Mat_dst_float_sigma_s_float_sigma_r(src: *mut c_void, dst: *mut c_void, sigma_s: f32, sigma_r: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_edgePreservingFilter_Mat_src_Mat_dst_int_flags_float_sigma_s_float_sigma_r(src: *mut c_void, dst: *mut c_void, flags: i32, sigma_s: f32, sigma_r: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoisingColoredMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize(src_imgs: *mut c_void, dst: *mut c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoisingColored_Mat_src_Mat_dst_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize(src: *mut c_void, dst: *mut c_void, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType(src_imgs: *mut c_void, dst: *mut c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: *mut c_void, template_window_size: i32, search_window_size: i32, norm_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_int_templateWindowSize_int_searchWindowSize(src_imgs: *mut c_void, dst: *mut c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoising_Mat_src_Mat_dst_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType(src: *mut c_void, dst: *mut c_void, h: *mut c_void, template_window_size: i32, search_window_size: i32, norm_type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_fastNlMeansDenoising_Mat_src_Mat_dst_float_h_int_templateWindowSize_int_searchWindowSize(src: *mut c_void, dst: *mut c_void, h: f32, template_window_size: i32, search_window_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_illuminationChange_Mat_src_Mat_mask_Mat_dst_float_alpha_float_beta(src: *mut c_void, mask: *mut c_void, dst: *mut c_void, alpha: f32, beta: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_inpaint_Mat_src_Mat_inpaintMask_Mat_dst_double_inpaintRadius_int_flags(src: *mut c_void, inpaint_mask: *mut c_void, dst: *mut c_void, inpaint_radius: f64, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_pencilSketch_Mat_src_Mat_dst1_Mat_dst2_float_sigma_s_float_sigma_r_float_shade_factor(src: *mut c_void, dst1: *mut c_void, dst2: *mut c_void, sigma_s: f32, sigma_r: f32, shade_factor: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_seamlessClone_Mat_src_Mat_dst_Mat_mask_Point_p_Mat_blend_int_flags(src: *mut c_void, dst: *mut c_void, mask: *mut c_void, p: core::Point, blend: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_stylization_Mat_src_Mat_dst_float_sigma_s_float_sigma_r(src: *mut c_void, dst: *mut c_void, sigma_s: f32, sigma_r: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_textureFlattening_Mat_src_Mat_mask_Mat_dst_float_low_threshold_float_high_threshold_int_kernel_size(src: *mut c_void, mask: *mut c_void, dst: *mut c_void, low_threshold: f32, high_threshold: f32, kernel_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignExposures_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst(instance: *mut c_void, src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_calculateShift_Mat_img0_Mat_img1(instance: *mut c_void, img0: *mut c_void, img1: *mut c_void) -> cv_return_value_c_Point;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_shiftMat_Mat_src_Mat_dst_Point_shift(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, shift: core::Point) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_computeBitmaps_Mat_img_Mat_tb_Mat_eb(instance: *mut c_void, img: *mut c_void, tb: *mut c_void, eb: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_getMaxBits(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_setMaxBits_int_max_bits(instance: *mut c_void, max_bits: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_getExcludeRange(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_setExcludeRange_int_exclude_range(instance: *mut c_void, exclude_range: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_getCut(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_photo_cv_AlignMTB_setCut_bool_value(instance: *mut c_void, value: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateCRF_process_VectorOfMat_src_Mat_dst_Mat_times(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_getLambda(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_setLambda_float_lambda(instance: *mut c_void, lambda: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_getSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_setSamples_int_samples(instance: *mut c_void, samples: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_getRandom(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateDebevec_setRandom_bool_random(instance: *mut c_void, random: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateRobertson_getMaxIter(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateRobertson_setMaxIter_int_max_iter(instance: *mut c_void, max_iter: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateRobertson_getThreshold(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateRobertson_setThreshold_float_threshold(instance: *mut c_void, threshold: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_CalibrateRobertson_getRadiance(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_photo_cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeExposures_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_process_VectorOfMat_src_Mat_dst(instance: *mut c_void, src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_getContrastWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_setContrastWeight_float_contrast_weiht(instance: *mut c_void, contrast_weiht: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_getSaturationWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_setSaturationWeight_float_saturation_weight(instance: *mut c_void, saturation_weight: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_getExposureWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_MergeMertens_setExposureWeight_float_exposure_weight(instance: *mut c_void, exposure_weight: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void, response: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times(instance: *mut c_void, src: *mut c_void, dst: *mut c_void, times: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_Tonemap_process_Mat_src_Mat_dst(instance: *mut c_void, src: *mut c_void, dst: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_Tonemap_getGamma(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_Tonemap_setGamma_float_gamma(instance: *mut c_void, gamma: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapDrago_getSaturation(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapDrago_setSaturation_float_saturation(instance: *mut c_void, saturation: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapDrago_getBias(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapDrago_setBias_float_bias(instance: *mut c_void, bias: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapMantiuk_getScale(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapMantiuk_setScale_float_scale(instance: *mut c_void, scale: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapMantiuk_getSaturation(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapMantiuk_setSaturation_float_saturation(instance: *mut c_void, saturation: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_getIntensity(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_setIntensity_float_intensity(instance: *mut c_void, intensity: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_getLightAdaptation(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_setLightAdaptation_float_light_adapt(instance: *mut c_void, light_adapt: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_getColorAdaptation(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_photo_cv_TonemapReinhard_setColorAdaptation_float_color_adapt(instance: *mut c_void, color_adapt: f32) -> cv_return_value_void;
}
extern "C" {
#[doc(hidden)] pub fn cv_shape_cv_EMDL1_Mat_signature1_Mat_signature2(signature1: *mut c_void, signature2: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_createAffineTransformer_bool_fullAffine(full_affine: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createChiHistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies: i32, default_cost: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createEMDHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag: i32, n_dummies: i32, default_cost: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createEMDL1HistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies: i32, default_cost: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createHausdorffDistanceExtractor_int_distanceFlag_float_rankProp(distance_flag: i32, rank_prop: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createNormHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag: i32, n_dummies: i32, default_cost: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_createThinPlateSplineShapeTransformer_double_regularizationParameter(regularization_parameter: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_AffineTransformer_setFullAffine_bool_fullAffine(instance: *mut c_void, full_affine: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_AffineTransformer_getFullAffine(instance: *const c_void) -> cv_return_value_char;
pub fn cv_delete_ChiHistogramCostExtractor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_shape_cv_EMDHistogramCostExtractor_setNormFlag_int_flag(instance: *mut c_void, flag: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_EMDHistogramCostExtractor_getNormFlag(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_EMDL1HistogramCostExtractor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_shape_cv_HausdorffDistanceExtractor_setDistanceFlag_int_distanceFlag(instance: *mut c_void, distance_flag: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_HausdorffDistanceExtractor_getDistanceFlag(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_HausdorffDistanceExtractor_setRankProportion_float_rankProportion(instance: *mut c_void, rank_proportion: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_HausdorffDistanceExtractor_getRankProportion(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_HistogramCostExtractor_buildCostMatrix_Mat_descriptors1_Mat_descriptors2_Mat_costMatrix(instance: *mut c_void, descriptors1: *mut c_void, descriptors2: *mut c_void, cost_matrix: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_HistogramCostExtractor_setNDummies_int_nDummies(instance: *mut c_void, n_dummies: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_HistogramCostExtractor_getNDummies(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_HistogramCostExtractor_setDefaultCost_float_defaultCost(instance: *mut c_void, default_cost: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_HistogramCostExtractor_getDefaultCost(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_NormHistogramCostExtractor_setNormFlag_int_flag(instance: *mut c_void, flag: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_NormHistogramCostExtractor_getNormFlag(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setAngularBins_int_nAngularBins(instance: *mut c_void, n_angular_bins: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getAngularBins(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setRadialBins_int_nRadialBins(instance: *mut c_void, n_radial_bins: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getRadialBins(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setInnerRadius_float_innerRadius(instance: *mut c_void, inner_radius: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getInnerRadius(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setOuterRadius_float_outerRadius(instance: *mut c_void, outer_radius: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getOuterRadius(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setRotationInvariant_bool_rotationInvariant(instance: *mut c_void, rotation_invariant: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getRotationInvariant(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setShapeContextWeight_float_shapeContextWeight(instance: *mut c_void, shape_context_weight: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getShapeContextWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float_imageAppearanceWeight(instance: *mut c_void, image_appearance_weight: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getImageAppearanceWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float_bendingEnergyWeight(instance: *mut c_void, bending_energy_weight: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getBendingEnergyWeight(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setImages_Mat_image1_Mat_image2(instance: *mut c_void, image1: *mut c_void, image2: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getImages_Mat_image1_Mat_image2(instance: *const c_void, image1: *mut c_void, image2: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setIterations_int_iterations(instance: *mut c_void, iterations: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getIterations(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setCostExtractor_PtrOfHistogramCostExtractor_comparer(instance: *mut c_void, comparer: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getCostExtractor(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_setStdDev_float_sigma(instance: *mut c_void, sigma: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeContextDistanceExtractor_getStdDev(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeDistanceExtractor_computeDistance_Mat_contour1_Mat_contour2(instance: *mut c_void, contour1: *mut c_void, contour2: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeTransformer_estimateTransformation_Mat_transformingShape_Mat_targetShape_VectorOfDMatch_matches(instance: *mut c_void, transforming_shape: *mut c_void, target_shape: *mut c_void, matches: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ShapeTransformer_applyTransformation_Mat_input_Mat_output(instance: *mut c_void, input: *mut c_void, output: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_shape_cv_ShapeTransformer_warpImage_Mat_transformingImage_Mat_output_int_flags_int_borderMode_Scalar_borderValue(instance: *const c_void, transforming_image: *mut c_void, output: *mut c_void, flags: i32, border_mode: i32, border_value: core::Scalar) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double_beta(instance: *mut c_void, beta: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_shape_cv_ThinPlateSplineShapeTransformer_getRegularizationParameter(instance: *const c_void) -> cv_return_value_double;
}
extern "C" {
#[doc(hidden)] pub fn cv_stitching_cv_createStitcherScans_bool_try_use_gpu(try_use_gpu: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_stitching_cv_createStitcher_bool_try_use_gpu(try_use_gpu: bool) -> cv_return_value_void_X;
pub fn cv_delete_AffineWarper(ptr : *mut c_void);
pub fn cv_delete_CompressedRectilinearPortraitWarper(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_stitching_cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_A_float_B(a: f32, b: f32) -> cv_return_value_void_X;
pub fn cv_delete_CompressedRectilinearWarper(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_stitching_cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_A_float_B(a: f32, b: f32) -> cv_return_value_void_X;
pub fn cv_delete_CylindricalWarper(ptr : *mut c_void);
pub fn cv_delete_FisheyeWarper(ptr : *mut c_void);
pub fn cv_delete_MercatorWarper(ptr : *mut c_void);
pub fn cv_delete_PaniniPortraitWarper(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_stitching_cv_PaniniPortraitWarper_PaniniPortraitWarper_float_A_float_B(a: f32, b: f32) -> cv_return_value_void_X;
pub fn cv_delete_PaniniWarper(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_stitching_cv_PaniniWarper_PaniniWarper_float_A_float_B(a: f32, b: f32) -> cv_return_value_void_X;
pub fn cv_delete_PlaneWarper(ptr : *mut c_void);
pub fn cv_delete_SphericalWarper(ptr : *mut c_void);
pub fn cv_delete_StereographicWarper(ptr : *mut c_void);
pub fn cv_delete_Stitcher(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_createDefault_bool_try_use_gpu(try_use_gpu: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_registrationResol(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_setRegistrationResol_double_resol_mpx(instance: *mut c_void, resol_mpx: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_seamEstimationResol(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_setSeamEstimationResol_double_resol_mpx(instance: *mut c_void, resol_mpx: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_compositingResol(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_setCompositingResol_double_resol_mpx(instance: *mut c_void, resol_mpx: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_panoConfidenceThresh(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_setPanoConfidenceThresh_double_conf_thresh(instance: *mut c_void, conf_thresh: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_waveCorrection(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_setWaveCorrection_bool_flag(instance: *mut c_void, flag: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_component(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_stitching_cv_Stitcher_workScale(instance: *const c_void) -> cv_return_value_double;
pub fn cv_delete_TransverseMercatorWarper(ptr : *mut c_void);
}
extern "C" {
#[doc(hidden)] pub fn cv_superres_cv_superres_createFrameSource_Camera_int_deviceId(device_id: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_createFrameSource_Empty() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_createFrameSource_Video_CUDA_String_fileName(file_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_createFrameSource_Video_String_fileName(file_name: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_createSuperResolution_BTVL1() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_createSuperResolution_BTVL1_CUDA() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_superres_cv_superres_FrameSource_nextFrame_Mat_frame(instance: *mut c_void, frame: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_FrameSource_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setInput_PtrOfFrameSource_frameSource(instance: *mut c_void, frame_source: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_nextFrame_Mat_frame(instance: *mut c_void, frame: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_collectGarbage(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getScale(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setScale_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getIterations(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setIterations_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getTau(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setTau_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getLabmda(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setLabmda_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getAlpha(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setAlpha_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getKernelSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setKernelSize_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getBlurKernelSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setBlurKernelSize_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getBlurSigma(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setBlurSigma_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_getTemporalAreaRadius(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_superres_cv_superres_SuperResolution_setTemporalAreaRadius_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
}
extern "C" {
#[doc(hidden)] pub fn cv_video_cv_CamShift_Mat_probImage_Rect_window_TermCriteria_criteria(prob_image: *mut c_void, window: core::Rect, criteria: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_buildOpticalFlowPyramid_Mat_img_VectorOfMat_pyramid_Size_winSize_int_maxLevel_bool_withDerivatives_int_pyrBorder_int_derivBorder_bool_tryReuseInputImage(img: *mut c_void, pyramid: *mut c_void, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_calcOpticalFlowFarneback_Mat_prev_Mat_next_Mat_flow_double_pyr_scale_int_levels_int_winsize_int_iterations_int_poly_n_double_poly_sigma_int_flags(prev: *mut c_void, next: *mut c_void, flow: *mut c_void, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_calcOpticalFlowPyrLK_Mat_prevImg_Mat_nextImg_Mat_prevPts_Mat_nextPts_Mat_status_Mat_err_Size_winSize_int_maxLevel_TermCriteria_criteria_int_flags_double_minEigThreshold(prev_img: *mut c_void, next_img: *mut c_void, prev_pts: *mut c_void, next_pts: *mut c_void, status: *mut c_void, err: *mut c_void, win_size: core::Size, max_level: i32, criteria: *mut c_void, flags: i32, min_eig_threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_createBackgroundSubtractorKNN_int_history_double_dist2Threshold_bool_detectShadows(history: i32, dist2_threshold: f64, detect_shadows: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_createBackgroundSubtractorMOG2_int_history_double_varThreshold_bool_detectShadows(history: i32, var_threshold: f64, detect_shadows: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_createOptFlow_DualTVL1() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_estimateRigidTransform_Mat_src_Mat_dst_bool_fullAffine(src: *mut c_void, dst: *mut c_void, full_affine: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_estimateRigidTransform_Mat_src_Mat_dst_bool_fullAffine_int_ransacMaxIters_double_ransacGoodRatio_int_ransacSize0(src: *mut c_void, dst: *mut c_void, full_affine: bool, ransac_max_iters: i32, ransac_good_ratio: f64, ransac_size0: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_findTransformECC_Mat_templateImage_Mat_inputImage_Mat_warpMatrix_int_motionType_TermCriteria_criteria_Mat_inputMask(template_image: *mut c_void, input_image: *mut c_void, warp_matrix: *mut c_void, motion_type: i32, criteria: *mut c_void, input_mask: *mut c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_meanShift_Mat_probImage_Rect_window_TermCriteria_criteria(prob_image: *mut c_void, window: core::Rect, criteria: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractor_apply_Mat_image_Mat_fgmask_double_learningRate(instance: *mut c_void, image: *mut c_void, fgmask: *mut c_void, learning_rate: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractor_getBackgroundImage_Mat_backgroundImage(instance: *const c_void, background_image: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getHistory(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setHistory_int_history(instance: *mut c_void, history: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getNSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setNSamples_int__nN(instance: *mut c_void, _n_n: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getDist2Threshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setDist2Threshold_double__dist2Threshold(instance: *mut c_void, _dist2_threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getkNNSamples(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setkNNSamples_int__nkNN(instance: *mut c_void, _nk_nn: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getDetectShadows(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setDetectShadows_bool_detectShadows(instance: *mut c_void, detect_shadows: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getShadowValue(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setShadowValue_int_value(instance: *mut c_void, value: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_getShadowThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorKNN_setShadowThreshold_double_threshold(instance: *mut c_void, threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getHistory(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setHistory_int_history(instance: *mut c_void, history: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getNMixtures(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setNMixtures_int_nmixtures(instance: *mut c_void, nmixtures: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getBackgroundRatio(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setBackgroundRatio_double_ratio(instance: *mut c_void, ratio: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getVarThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setVarThreshold_double_varThreshold(instance: *mut c_void, var_threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getVarThresholdGen(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setVarThresholdGen_double_varThresholdGen(instance: *mut c_void, var_threshold_gen: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getVarInit(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setVarInit_double_varInit(instance: *mut c_void, var_init: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getVarMin(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setVarMin_double_varMin(instance: *mut c_void, var_min: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getVarMax(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setVarMax_double_varMax(instance: *mut c_void, var_max: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double_ct(instance: *mut c_void, ct: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getDetectShadows(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setDetectShadows_bool_detectShadows(instance: *mut c_void, detect_shadows: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getShadowValue(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setShadowValue_int_value(instance: *mut c_void, value: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_getShadowThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_setShadowThreshold_double_threshold(instance: *mut c_void, threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_BackgroundSubtractorMOG2_apply_Mat_image_Mat_fgmask_double_learningRate(instance: *mut c_void, image: *mut c_void, fgmask: *mut c_void, learning_rate: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DenseOpticalFlow_calc_Mat_I0_Mat_I1_Mat_flow(instance: *mut c_void, i0: *mut c_void, i1: *mut c_void, flow: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DenseOpticalFlow_collectGarbage(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getTau(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setTau_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getLambda(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setLambda_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getTheta(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setTheta_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getGamma(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setGamma_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getScalesNumber(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setScalesNumber_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getWarpingsNumber(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setWarpingsNumber_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getEpsilon(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setEpsilon_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getInnerIterations(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setInnerIterations_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getOuterIterations(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setOuterIterations_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getUseInitialFlow(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setUseInitialFlow_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getScaleStep(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setScaleStep_double_val(instance: *mut c_void, val: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_getMedianFiltering(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_setMedianFiltering_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_DualTVL1OpticalFlow_create_double_tau_double_lambda_double_theta_int_nscales_int_warps_double_epsilon_int_innnerIterations_int_outerIterations_double_scaleStep_double_gamma_int_medianFiltering_bool_useInitialFlow(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getNumLevels(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setNumLevels_int_numLevels(instance: *mut c_void, num_levels: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getPyrScale(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setPyrScale_double_pyrScale(instance: *mut c_void, pyr_scale: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getFastPyramids(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setFastPyramids_bool_fastPyramids(instance: *mut c_void, fast_pyramids: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getWinSize(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setWinSize_int_winSize(instance: *mut c_void, win_size: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getNumIters(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setNumIters_int_numIters(instance: *mut c_void, num_iters: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getPolyN(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setPolyN_int_polyN(instance: *mut c_void, poly_n: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getPolySigma(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setPolySigma_double_polySigma(instance: *mut c_void, poly_sigma: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_getFlags(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_setFlags_int_flags(instance: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_FarnebackOpticalFlow_create_int_numLevels_double_pyrScale_bool_fastPyramids_int_winSize_int_numIters_int_polyN_double_polySigma_int_flags(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> cv_return_value_void_X;
pub fn cv_delete_KalmanFilter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_video_cv_KalmanFilter_KalmanFilter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_KalmanFilter_KalmanFilter_int_dynamParams_int_measureParams_int_controlParams_int_type(dynam_params: i32, measure_params: i32, control_params: i32, _type: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_KalmanFilter_init_int_dynamParams_int_measureParams_int_controlParams_int_type(instance: *mut c_void, dynam_params: i32, measure_params: i32, control_params: i32, _type: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_KalmanFilter_predict_Mat_control(instance: *mut c_void, control: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_KalmanFilter_correct_Mat_measurement(instance: *mut c_void, measurement: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_SparseOpticalFlow_calc_Mat_prevImg_Mat_nextImg_Mat_prevPts_Mat_nextPts_Mat_status_Mat_err(instance: *mut c_void, prev_img: *mut c_void, next_img: *mut c_void, prev_pts: *mut c_void, next_pts: *mut c_void, status: *mut c_void, err: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_getWinSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_setWinSize_Size_winSize(instance: *mut c_void, win_size: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_getMaxLevel(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_setMaxLevel_int_maxLevel(instance: *mut c_void, max_level: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_getTermCriteria(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteria_crit(instance: *mut c_void, crit: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_getFlags(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_setFlags_int_flags(instance: *mut c_void, flags: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_getMinEigThreshold(instance: *const c_void) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double_minEigThreshold(instance: *mut c_void, min_eig_threshold: f64) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_video_cv_SparsePyrLKOpticalFlow_create_Size_winSize_int_maxLevel_TermCriteria_crit_int_flags_double_minEigThreshold(win_size: core::Size, max_level: i32, crit: *mut c_void, flags: i32, min_eig_threshold: f64) -> cv_return_value_void_X;
}
extern "C" {
#[doc(hidden)] pub fn cv_videoio_CV_FOURCC_char_c1_char_c2_char_c3_char_c4(c1: i8, c2: i8, c3: i8, c4: i8) -> cv_return_value_int;
pub fn cv_delete_VideoCapture(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_VideoCapture() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_VideoCapture_String_filename(filename: *const c_char) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_VideoCapture_String_filename_int_apiPreference(filename: *const c_char, api_preference: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_VideoCapture_int_index(index: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_VideoCapture_int_index_int_apiPreference(index: i32, api_preference: i32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_open_String_filename(instance: *mut c_void, filename: *const c_char) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_open_int_index(instance: *mut c_void, index: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_open_int_cameraNum_int_apiPreference(instance: *mut c_void, camera_num: i32, api_preference: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_isOpened(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_release(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_grab(instance: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_retrieve_Mat_image_int_flag(instance: *mut c_void, image: *mut c_void, flag: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_read_Mat_image(instance: *mut c_void, image: *mut c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_set_int_propId_double_value(instance: *mut c_void, prop_id: i32, value: f64) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_get_int_propId(instance: *const c_void, prop_id: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_open_String_filename_int_apiPreference(instance: *mut c_void, filename: *const c_char, api_preference: i32) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoCapture_getBackendName(instance: *const c_void) -> cv_return_value_char_X;
pub fn cv_delete_VideoWriter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_VideoWriter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_VideoWriter_String_filename_int_fourcc_double_fps_Size_frameSize_bool_isColor(filename: *const c_char, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_VideoWriter_String_filename_int_apiPreference_int_fourcc_double_fps_Size_frameSize_bool_isColor(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_open_String_filename_int_fourcc_double_fps_Size_frameSize_bool_isColor(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_open_String_filename_int_apiPreference_int_fourcc_double_fps_Size_frameSize_bool_isColor(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_isOpened(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_release(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_write_Mat_image(instance: *mut c_void, image: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_set_int_propId_double_value(instance: *mut c_void, prop_id: i32, value: f64) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_get_int_propId(instance: *const c_void, prop_id: i32) -> cv_return_value_double;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_fourcc_char_c1_char_c2_char_c3_char_c4(c1: i8, c2: i8, c3: i8, c4: i8) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videoio_cv_VideoWriter_getBackendName(instance: *const c_void) -> cv_return_value_char_X;
}
extern "C" {
#[doc(hidden)] pub fn cv_videostab_cv_videostab_calcBlurriness_Mat_frame(frame: *mut c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_calcFlowMask_Mat_flowX_Mat_flowY_Mat_errors_float_maxError_Mat_mask0_Mat_mask1_Mat_flowMask(flow_x: *mut c_void, flow_y: *mut c_void, errors: *mut c_void, max_error: f32, mask0: *mut c_void, mask1: *mut c_void, flow_mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_completeFrameAccordingToFlow_Mat_flowMask_Mat_flowX_Mat_flowY_Mat_frame1_Mat_mask1_float_distThresh_Mat_frame0_Mat_mask0(flow_mask: *mut c_void, flow_x: *mut c_void, flow_y: *mut c_void, frame1: *mut c_void, mask1: *mut c_void, dist_thresh: f32, frame0: *mut c_void, mask0: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ensureInclusionConstraint_Mat_M_Size_size_float_trimRatio(m: *mut c_void, size: core::Size, trim_ratio: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_estimateOptimalTrimRatio_Mat_M_Size_size(m: *mut c_void, size: core::Size) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_getMotion_int_from_int_to_VectorOfMat_motions(from: i32, to: i32, motions: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_ColorAverageInpainter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ColorAverageInpainter_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_ColorInpainter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ColorInpainter_ColorInpainter_int_method_double_radius(method: i32, radius: f64) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ColorInpainter_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_ConsistentMosaicInpainter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ConsistentMosaicInpainter_stdevThresh(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ConsistentMosaicInpainter_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_setRadius_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_radius(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_deblur_int_idx_Mat_frame(instance: *mut c_void, idx: i32, frame: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_setFrames_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_frames(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_setMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_motions(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_setBlurrinessRates_VectorOffloat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_DeblurerBase_blurrinessRates(instance: *const c_void) -> cv_return_value_void_X;
pub fn cv_delete_FastMarchingMethod(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_FastMarchingMethod_distanceMap(instance: *const c_void) -> cv_return_value_void_X;
pub fn cv_delete_FromFileMotionReader(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_FromFileMotionReader_FromFileMotionReader_String_path(path: *const c_char) -> cv_return_value_void_X;
pub fn cv_delete_GaussianMotionFilter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_radius_float_stdev(radius: i32, stdev: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_GaussianMotionFilter_setParams_int_radius_float_stdev(instance: *mut c_void, radius: i32, stdev: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_GaussianMotionFilter_radius(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_GaussianMotionFilter_stdev(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int__radius_float__stdev(_radius: i32, _stdev: f32) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_IDenseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_flowX_Mat_flowY_Mat_errors(instance: *mut c_void, frame0: *mut c_void, frame1: *mut c_void, flow_x: *mut c_void, flow_y: *mut c_void, errors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_IFrameSource_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_IFrameSource_nextFrame(instance: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_IOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(instance: *mut c_void, frame_size: core::Size, points0: *mut c_void, points1: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ISparseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors(instance: *mut c_void, frame0: *mut c_void, frame1: *mut c_void, points0: *mut c_void, points1: *mut c_void, status: *mut c_void, errors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_setRadius_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_radius(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_setFrames_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_frames(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_setMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_motions(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_setStabilizedFrames_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_stabilizedFrames(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_setStabilizationMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpainterBase_stabilizationMotions(instance: *const c_void) -> cv_return_value_void_X;
pub fn cv_delete_InpaintingPipeline(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_pushBack_PtrOfInpainterBase_inpainter(instance: *mut c_void, inpainter: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_empty(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_setRadius_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_setFrames_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_setMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_setStabilizedFrames_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_setStabilizationMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_InpaintingPipeline_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_KeypointBasedMotionEstimator(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrOfMotionEstimatorBase_estimator(estimator: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_LogToStdout(ptr : *mut c_void);
pub fn cv_delete_LpMotionStabilizer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setFrameSize_Size_val(instance: *mut c_void, val: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_frameSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setTrimRatio_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_trimRatio(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setWeight1_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_weight1(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setWeight2_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_weight2(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setWeight3_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_weight3(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_setWeight4_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_LpMotionStabilizer_weight4(instance: *const c_void) -> cv_return_value_float;
pub fn cv_delete_MoreAccurateMotionWobbleSuppressor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result(instance: *mut c_void, idx: i32, frame: *mut c_void, result: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_MotionEstimatorL1(ptr : *mut c_void);
pub fn cv_delete_MotionEstimatorRansacL2(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionEstimatorRansacL2_minInlierRatio(instance: *const c_void) -> cv_return_value_float;
pub fn cv_delete_MotionInpainter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_MotionInpainter() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_setFlowErrorThreshold_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_flowErrorThreshold(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_setDistThreshold_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_distThresh(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_setBorderMode_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_borderMode(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionInpainter_inpaint_int_idx_Mat_frame_Mat_mask(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_MotionStabilizationPipeline(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionStabilizationPipeline_pushBack_PtrOfIMotionStabilizer_stabilizer(instance: *mut c_void, stabilizer: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_MotionStabilizationPipeline_empty(instance: *const c_void) -> cv_return_value_char;
pub fn cv_delete_NullDeblurer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullDeblurer_deblur_int_unnamed_arg_Mat_unnamed_arg_1(instance: *mut c_void, unnamed_arg: i32, unnamed_arg_1: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_NullFrameSource(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullFrameSource_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullFrameSource_nextFrame(instance: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_NullInpainter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullInpainter_inpaint_int_unnamed_arg_Mat_unnamed_arg_1_Mat_unnamed_arg_2(instance: *mut c_void, unnamed_arg: i32, unnamed_arg_1: *mut c_void, unnamed_arg_2: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_NullLog(ptr : *mut c_void);
pub fn cv_delete_NullOutlierRejector(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(instance: *mut c_void, frame_size: core::Size, points0: *mut c_void, points1: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_NullWobbleSuppressor(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_NullWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result(instance: *mut c_void, idx: i32, frame: *mut c_void, result: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_OnePassStabilizer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_OnePassStabilizer_OnePassStabilizer() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_OnePassStabilizer_setMotionFilter_PtrOfMotionFilterBase_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_OnePassStabilizer_motionFilter(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_OnePassStabilizer_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_OnePassStabilizer_nextFrame(instance: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size_val(instance: *mut c_void, val: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_winSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_RansacParams(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_RansacParams_niters(instance: *const c_void) -> cv_return_value_int;
pub fn cv_delete_SparsePyrLkOptFlowEstimator(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_SparsePyrLkOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors(instance: *mut c_void, frame0: *mut c_void, frame1: *mut c_void, points0: *mut c_void, points1: *mut c_void, status: *mut c_void, errors: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setLog_PtrOfILog_ilog(instance: *mut c_void, ilog: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_log(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setRadius_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_radius(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setFrameSource_PtrOfIFrameSource_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_frameSource(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_motionEstimator(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setDeblurer_PtrOfDeblurerBase_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_deblurrer(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setTrimRatio_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_trimRatio(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setCorrectionForInclusion_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_doCorrectionForInclusion(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setBorderMode_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_borderMode(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_setInpainter_PtrOfInpainterBase_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_StabilizerBase_inpainter(instance: *const c_void) -> cv_return_value_void_X;
pub fn cv_delete_ToFileMotionWriter(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_ToFileMotionWriter_ToFileMotionWriter_String_path_PtrOfImageMotionEstimatorBase_estimator(path: *const c_char, estimator: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_TranslationBasedLocalOutlierRejector(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size_val(instance: *mut c_void, val: core::Size) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_cellSize(instance: *const c_void) -> cv_return_value_c_Size;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(instance: *mut c_void, frame_size: core::Size, points0: *mut c_void, points1: *mut c_void, mask: *mut c_void) -> cv_return_value_void;
pub fn cv_delete_TwoPassStabilizer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_TwoPassStabilizer() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrOfIMotionStabilizer_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_motionStabilizer(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool_val(instance: *mut c_void, val: bool) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio(instance: *const c_void) -> cv_return_value_char;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_TwoPassStabilizer_nextFrame(instance: *mut c_void) -> cv_return_value_void_X;
pub fn cv_delete_VideoFileSource(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_VideoFileSource_String_path_bool_volatileFrame(path: *const c_char, volatile_frame: bool) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_reset(instance: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_nextFrame(instance: *mut c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_width(instance: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_height(instance: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_count(instance: *mut c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_VideoFileSource_fps(instance: *mut c_void) -> cv_return_value_double;
pub fn cv_delete_WeightingDeblurer(ptr : *mut c_void);
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WeightingDeblurer_WeightingDeblurer() -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WeightingDeblurer_setSensitivity_float_val(instance: *mut c_void, val: f32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WeightingDeblurer_sensitivity(instance: *const c_void) -> cv_return_value_float;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WeightingDeblurer_deblur_int_idx_Mat_frame(instance: *mut c_void, idx: i32, frame: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_motionEstimator(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_suppress_int_idx_Mat_frame_Mat_result(instance: *mut c_void, idx: i32, frame: *mut c_void, result: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_setFrameCount_int_val(instance: *mut c_void, val: i32) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_frameCount(instance: *const c_void) -> cv_return_value_int;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_setMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_motions(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_setMotions2_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_motions2(instance: *const c_void) -> cv_return_value_void_X;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_setStabilizationMotions_VectorOfMat_val(instance: *mut c_void, val: *mut c_void) -> cv_return_value_void;
#[doc(hidden)] pub fn cv_videostab_cv_videostab_WobbleSuppressorBase_stabilizationMotions(instance: *const c_void) -> cv_return_value_void_X;
}
