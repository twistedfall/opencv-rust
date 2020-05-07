// todo add doccomments
// todo add missing slice forcings

use std::collections::{BTreeSet, HashMap, HashSet};

use maplit::{btreeset, hashmap, hashset};
use once_cell::sync::Lazy;
use regex::RegexSet;

use crate::{CompiledInterpolation, ExportConfig, StrExt};

/// map of functions to rename or skip, key is Func.identifier(), value is new name ("+" will be replaced by old name) or "-" to skip
pub static FUNC_RENAME: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
	let mut out = hashmap! {
		// ### aruco ###
		"cv_aruco_getPredefinedDictionary_int" => "+_i32",

		// ### bioinspired ###
		"cv_bioinspired_Retina_create_Size_bool_int_bool_float_float" => "+_ext",
		"cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayX" => "+_to",
		"cv_bioinspired_Retina_getParvoRAW_const__OutputArrayX" => "+_to",
		"cv_bioinspired_Retina_setup_FileStorageX_bool" => "+_from_storage",
		"cv_bioinspired_Retina_setup_String_bool" => "+_from_file",
		"cv_bioinspired_Retina_write_const_FileStorageX" => "+_to_storage",
		"cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageX_bool" => "+_from_storage",
		"cv_bioinspired_TransientAreasSegmentationModule_setup_String_bool" => "+_from_file",
		"cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageX" => "+_to_storage",
		"cv_bioinspired_createRetina_OCL_Size_bool_int_bool_float_float" => "+_ext", // 3.2 only

		// ### calib3d ###
		"cv_LMSolver_create_const_Ptr_Callback_X_int_double" => "+_ext",
		"cv_findCirclesGrid_const__InputArrayX_Size_const__OutputArrayX_int_const_Ptr_FeatureDetector_X_const_CirclesGridFinderParametersX" => "+_params",
		"cv_findCirclesGrid_const__InputArrayX_Size_const__OutputArrayX_int_const_Ptr_FeatureDetector_X_CirclesGridFinderParameters" => "+_params", // 3.4
		"cv_findEssentialMat_const__InputArrayX_const__InputArrayX_const__InputArrayX_int_double_double_const__OutputArrayX" => "+_matrix",
		"cv_findFundamentalMat_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_double_double" => "+_mask",
		"cv_findHomography_const__InputArrayX_const__InputArrayX_int_double_const__OutputArrayX_int_double" => "+_ext",
		"cv_fisheye_initUndistortRectifyMap_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_int_const__OutputArrayX_const__OutputArrayX" => "fisheye_+",
		"cv_fisheye_projectPoints_const__InputArrayX_const__OutputArrayX_const_Affine3dX_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX" => "fisheye_+",
		"cv_fisheye_projectPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_double_const__OutputArrayX" => "fisheye_+_vec",
		"cv_fisheye_stereoCalibrate_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_Size_const__OutputArrayX_const__OutputArrayX_int_TermCriteria" => "fisheye_+",
		"cv_fisheye_stereoRectify_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_const_SizeX_double_double" => "fisheye_+",
		"cv_fisheye_undistortImage_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX" => "fisheye_+",
		"cv_fisheye_undistortPoints_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX" => "fisheye_+",
		"cv_recoverPose_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputOutputArrayX" => "+_camera",
		"cv_recoverPose_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_const__InputOutputArrayX_const__OutputArrayX" => "+_camera_with_points",

		// ### core ###
		"cv_AsyncArray_get_const_const__OutputArrayX_double" => "+_with_timeout_f64",
		"cv_AsyncArray_get_const_const__OutputArrayX_int64_t" => "+_with_timeout",
		"cv_AsyncArray_wait_for_const_double" => "+_f64",
		"cv_Cholesky_floatX_size_t_int_floatX_size_t_int" => "+_f32",
		"cv_DMatch_DMatch_int_int_int_float" => "new_index",
		"cv_FileStorage_write_const_StringX_const_MatX" => "+_mat",
		"cv_FileStorage_write_const_StringX_const_StringX" => "+_str",
		"cv_FileStorage_write_const_StringX_const_vector_String_X" => "+_str_vec",
		"cv_FileStorage_write_const_StringX_double" => "+_f64",
		"cv_FileStorage_write_const_StringX_int" => "+_i32",
		"cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int" => "+_point",
		"cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int" => "+_coords",
		"cv_KeyPoint_convert_const_vector_Point2f_X_vector_KeyPoint_X_float_float_int_int" => "+_to",
		"cv_LDA_LDA_const__InputArrayX_const__InputArrayX_int" => "+_with_data",
		"cv_LU_floatX_size_t_int_floatX_size_t_int" => "lu_f32",
		"cv_MatConstIterator_MatConstIterator_const_MatX" => "over",
		"cv_MatConstIterator_MatConstIterator_const_MatX_Point" => "with_start",
		"cv_MatConstIterator_MatConstIterator_const_MatX_const_intX" => "+_slice",
		"cv_MatConstIterator_MatConstIterator_const_MatX_int_int" => "with_rows_cols",
		"cv_MatConstIterator_pos_const_intX" => "+_to",
		"cv_MatConstIterator_seek_const_intX_bool" => "+_idx",
		"cv_MatExpr_MatExpr_const_MatX" => "from_mat",
		"cv_MatExpr_mul_const_const_MatExprX_double" => "+_matexpr",
		"cv_MatExpr_type_const" => "typ",
		"cv_MatOp_add_const_const_MatExprX_const_ScalarX_MatExprX" => "+_scalar",
		"cv_MatOp_divide_const_double_const_MatExprX_MatExprX" => "+_f64",
		"cv_MatOp_multiply_const_const_MatExprX_double_MatExprX" => "+_f64",
		"cv_MatOp_subtract_const_const_ScalarX_const_MatExprX_MatExprX" => "+_scalar",
		"cv_MatOp_type_const_MatExpr" => "typ",
		"cv_Mat_Mat_Size_int" => "+_size",
		"cv_Mat_Mat_Size_int_const_ScalarX" => "+_size_with_default",
		"cv_Mat_Mat_Size_int_voidX_size_t" => "+_size_with_data",
		"cv_Mat_Mat_const_MatX_const_RangeX_const_RangeX" => "rowscols",
		"cv_Mat_Mat_const_MatX_const_RectX" => "roi",
		"cv_Mat_Mat_const_MatX_const_vector_Range_X" => "ranges",
		"cv_Mat_Mat_const_vector_int_X_int" => "+_nd_vec",
		"cv_Mat_Mat_const_vector_int_X_int_const_ScalarX" => "+_nd_vec_with_default",
		"cv_Mat_Mat_const_vector_int_X_int_voidX_const_size_tX" => "+_nd_vec_with_data",
		"cv_Mat_Mat_int_const_intX_int" => "+_nd",
		"cv_Mat_Mat_int_const_intX_int_const_ScalarX" => "+_nd_with_default",
		"cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX" => "+_nd_with_data",
		"cv_Mat_Mat_int_int_int" => "+_rows_cols",
		"cv_Mat_Mat_int_int_int_const_ScalarX" => "+_rows_cols_with_default",
		"cv_Mat_Mat_int_int_int_voidX_size_t" => "+_rows_cols_with_data",
		"cv_Mat_at_Point" => "at_pt_mut",
		"cv_Mat_at_const_Point" => "at_pt",
		"cv_Mat_at_const_const_intX" => "at_nd",
		"cv_Mat_at_const_intX" => "at_nd_mut",
		"cv_Mat_at_const_int_int" => "at_2d",
		"cv_Mat_at_const_int_int_int" => "at_3d",
		"cv_Mat_at_int" => "at_mut",
		"cv_Mat_at_int_int" => "at_2d_mut",
		"cv_Mat_at_int_int_int" => "at_3d_mut",
		"cv_Mat_colRange_const_int_int" => "col_bounds",
		"cv_Mat_copyTo_const_const__OutputArrayX_const__InputArrayX" => "+_masked",
		"cv_Mat_create_Size_int" => "+_size",
		"cv_Mat_create_const_vector_int_X_int" => "+_nd_vec",
		"cv_Mat_create_int_const_intX_int" => "+_nd",
		"cv_Mat_create_int_int_int" => "+_rows_cols",
		"cv_Mat_data" => "+_mut",
		"cv_Mat_diag_const_MatX" => "+_mat",
		"cv_Mat_eye_Size_int" => "+_size",
		"cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags" => "get_umat",
		"cv_Mat_getUMat_const_int_UMatUsageFlags" => "get_umat", // 3.2 only
		"cv_Mat_ones_Size_int" => "+_size",
		"cv_Mat_ones_int_const_intX_int" => "+_nd",
		"cv_Mat_ptr_const_const_intX" => "+_nd",
		"cv_Mat_ptr_const_intX" => "+_nd_mut",
		"cv_Mat_ptr_const_int_int" => "+_2d",
		"cv_Mat_ptr_const_int_int_int" => "+_3d",
		"cv_Mat_ptr_int" => "+_mut",
		"cv_Mat_ptr_int_int" => "+_2d_mut",
		"cv_Mat_ptr_int_int_int" => "+_3d_mut",
		"cv_Mat_reshape_const_int_const_vector_int_X" => "+_nd_vec",
		"cv_Mat_reshape_const_int_int_const_intX" => "+_nd",
		"cv_Mat_resize_size_t_const_ScalarX" => "+_with_default",
		"cv_Mat_rowRange_const_int_int" => "row_bounds",
		"cv_Mat_size_const" => "mat_size",
		"cv_Mat_step_const" => "mat_step",
		"cv_Mat_total_const_int_int" => "total_slice",
		"cv_Mat_type_const" => "typ",
		"cv_Mat_zeros_Size_int" => "+_size",
		"cv_Mat_zeros_int_const_intX_int" => "+_nd",
		"cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_double" => "+_variance",
		"cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_double" => "+_variance",
		"cv_PCA_PCA_const__InputArrayX_const__InputArrayX_int_double" => "+_with_variance",
		"cv_PCA_backProject_const_const__InputArrayX_const__OutputArrayX" => "+_to",
		"cv_PCA_project_const_const__InputArrayX_const__OutputArrayX" => "+_to",
		"cv_Range_Range_int_int" => "new",
		"cv_RotatedRect_RotatedRect_const_Point2fX_const_Point2fX_const_Point2fX" => "for_points",
		"cv_SVD_backSubst_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX" => "+_multi",
		"cv_SVD_compute_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int" => "+_ext",
		"cv_SparseMat_SparseMat_const_MatX" => "from_mat",
		"cv_SparseMat_begin" => "+_mut",
		"cv_SparseMat_copyTo_const_MatX" => "+_mat",
		"cv_SparseMat_end" => "+_mut",
		"cv_UMat_UMat_Size_int_UMatUsageFlags" => "+_size",
		"cv_UMat_UMat_Size_int_const_ScalarX_UMatUsageFlags" => "+_size_with_default",
		"cv_UMat_UMat_const_UMatX_const_RangeX_const_RangeX" => "rowscols",
		"cv_UMat_UMat_const_UMatX_const_RectX" => "roi",
		"cv_UMat_UMat_const_UMatX_const_vector_Range_X" => "ranges",
		"cv_UMat_UMat_int_const_intX_int_UMatUsageFlags" => "+_nd",
		"cv_UMat_UMat_int_const_intX_int_const_ScalarX_UMatUsageFlags" => "+_nd_with_default",
		"cv_UMat_UMat_int_int_int_UMatUsageFlags" => "+_rows_cols",
		"cv_UMat_UMat_int_int_int_const_ScalarX_UMatUsageFlags" => "+_rows_cols_with_default",
		"cv_UMat_colRange_const_int_int" => "col_bounds",
		"cv_UMat_copyTo_const_const__OutputArrayX_const__InputArrayX" => "+_masked",
		"cv_UMat_create_Size_int_UMatUsageFlags" => "+_size",
		"cv_UMat_create_const_vector_int_X_int_UMatUsageFlags" => "+_nd_vec",
		"cv_UMat_create_int_const_intX_int_UMatUsageFlags" => "+_nd",
		"cv_UMat_create_int_int_int_UMatUsageFlags" => "+_rows_cols",
		"cv_UMat_rowRange_const_int_int" => "row_bounds",
		"cv_UMat_size_const" => "mat_size",
		"cv_UMat_step_const" => "mat_step",
		"cv_UMat_type_const" => "typ",
		"cv__InputArray__InputArray_const_MatExprX" => "from_matexpr",
		"cv__InputArray__InputArray_const_MatX" => "from_mat",
		"cv__InputArray__InputArray_const_UMatX" => "from_umat",
		"cv__InputArray__InputArray_const_doubleX" => "from_f64",
		"cv__InputArray__InputArray_const_vector_Mat_X" => "from_mat_vec",
		"cv__InputArray__InputArray_const_vector_UMat_X" => "from_umat_vec",
		"cv__InputArray__InputArray_const_vector_bool_X" => "from_bool_vec",
		"cv__InputArray_copyTo_const_const__OutputArrayX_const__InputArrayX" => "+_with_mask",
		"cv__InputOutputArray__InputOutputArray_MatX" => "from_mat_mut",
		"cv__InputOutputArray__InputOutputArray_MatX" => "from_mat_mut",
		"cv__InputOutputArray__InputOutputArray_UMatX" => "from_umat_mut",
		"cv__InputOutputArray__InputOutputArray_const_MatX" => "from_mat",
		"cv__InputOutputArray__InputOutputArray_const_UMatX" => "from_umat",
		"cv__InputOutputArray__InputOutputArray_const_vector_Mat_X" => "from_mat_vec",
		"cv__InputOutputArray__InputOutputArray_const_vector_UMat_X" => "from_umat_vec",
		"cv__InputOutputArray__InputOutputArray_vector_Mat_X" => "from_mat_vec_mut",
		"cv__InputOutputArray__InputOutputArray_vector_UMat_X" => "from_umat_vec_mut",
		"cv__OutputArray__OutputArray_MatX" => "from_mat_mut",
		"cv__OutputArray__OutputArray_UMatX" => "from_umat_mut",
		"cv__OutputArray__OutputArray_const_MatX" => "from_mat",
		"cv__OutputArray__OutputArray_const_UMatX" => "from_umat",
		"cv__OutputArray__OutputArray_const_vector_Mat_X" => "from_mat_vec",
		"cv__OutputArray__OutputArray_const_vector_UMat_X" => "from_umat_vec",
		"cv__OutputArray__OutputArray_vector_Mat_X" => "from_mat_vec_mut",
		"cv__OutputArray__OutputArray_vector_UMat_X" => "from_umat_vec_mut",
		"cv__OutputArray_create_const_Size_int_int_bool_DepthMask" => "+_size",
		"cv__OutputArray_create_const_Size_int_int_bool_int" => "+_size", // 3.2
		"cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask" => "+_nd",
		"cv__OutputArray_create_const_int_const_intX_int_int_bool_int" => "+_nd", // 3.2
		"cv_abs_const_MatExprX" => "+_matexpr",
		"cv_directx_getTypeFromD3DFORMAT_int" => "get_type_from_d3d_format",
		"cv_divide_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_int" => "+2",
		"cv_getNumberOfCPUs" => "get_number_of_cpus",
		"cv_hconcat_const__InputArrayX_const__InputArrayX_const__OutputArrayX" => "+2",
		"cv_max_const_MatX_const_MatX" => "+_mat",
		"cv_max_const_MatX_const_MatX_MatX" => "+_mat_to",
		"cv_max_const_MatX_double" => "+_mat_f64",
		"cv_max_const_UMatX_const_UMatX_UMatX" => "+_umat_to",
		"cv_max_double_const_MatX" => "+_f64_mat",
		"cv_merge_const_MatX_size_t_const__OutputArrayX" => "+_slice",
		"cv_minMaxLoc_const_SparseMatX_doubleX_doubleX_intX_intX" => "+_sparse",
		"cv_min_const_MatX_const_MatX" => "+_mat",
		"cv_min_const_MatX_const_MatX_MatX" => "+_mat_to",
		"cv_min_const_MatX_double" => "+_mat_f64",
		"cv_min_const_UMatX_const_UMatX_UMatX" => "+_umat_to",
		"cv_min_double_const_MatX" => "+_f64_mat",
		"cv_mixChannels_const__InputArrayX_const__InputOutputArrayX_const_vector_int_X" => "+_vec",
		"cv_norm_const_SparseMatX_int" => "+_sparse",
		"cv_norm_const__InputArrayX_const__InputArrayX_int_const__InputArrayX" => "+2",
		"cv_normalize_const_SparseMatX_SparseMatX_double_int" => "+_sparse",
		"cv_ocl_Context_Context_int" => "+_with_type",
		"cv_ocl_Context_create_int" => "+_with_type",
		"cv_ocl_Kernel_create_const_charX_const_ProgramSourceX_const_StringX_StringX" => "+_ext",
		"cv_ocl_Kernel_set_int_const_KernelArgX" => "+_kernel_arg",
		"cv_ocl_Kernel_set_int_const_UMatX" => "+_umat",
		"cv_ocl_ProgramSource_ProgramSource_const_StringX" => "from_str",
		"cv_ocl_Program_getPrefix_const_StringX" => "+_build_flags",
		"cv_read_FileNode_Range_Range" => "+_range",
		"cv_read_FileNode_VectorOfDMatch_VectorOfDMatch" => "+_dmatch_vec",
		"cv_read_FileNode_VectorOfKeyPoint_VectorOfKeyPoint" => "+_keypoint_vec",
		"cv_read_FileNode_bool_bool" => "+_bool",
		"cv_read_FileNode_short_short" => "+_i16",
		"cv_read_FileNode_uchar_uchar" => "+_u8",
		"cv_read_FileNode_ushort_ushort" => "+_u16",
		"cv_read_const_FileNodeX_DMatchX_const_DMatchX" => "+_dmatch",
		"cv_read_const_FileNodeX_KeyPointX_const_KeyPointX" => "+_keypoint",
		"cv_read_const_FileNodeX_MatX_const_MatX" => "+_mat",
		"cv_read_const_FileNodeX_SparseMatX_const_SparseMatX" => "+_sparsemat",
		"cv_read_const_FileNodeX_doubleX_double" => "+_f64",
		"cv_read_const_FileNodeX_floatX_float" => "+_f32",
		"cv_read_const_FileNodeX_intX_int" => "+_i32",
		"cv_read_const_FileNodeX_stringX_const_stringX" => "+_str",
		"cv_read_const_FileNodeX_vector_DMatch_X" => "+_dmatch_vec_legacy",
		"cv_read_const_FileNodeX_vector_KeyPoint_X" => "+_keypoint_vec_legacy",
		"cv_rectangle_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int" => "+_points",
		"cv_repeat_const__InputArrayX_int_int_const__OutputArrayX" => "+_to",
		"cv_split_const_MatX_MatX" => "+_slice",
		"cv_swap_UMatX_UMatX" => "+_umat",
		"cv_vconcat_const__InputArrayX_const__InputArrayX_const__OutputArrayX" => "+2",
		"cv_writeScalar_FileStorageX_const_StringX" => "+_str",
		"cv_writeScalar_FileStorageX_double" => "+_f64",
		"cv_writeScalar_FileStorageX_float" => "+_f32",
		"cv_writeScalar_FileStorageX_int" => "+_i32",
		"cv_write_FileStorageX_const_StringX_const_MatX" => "+_mat",
		"cv_write_FileStorageX_const_StringX_const_SparseMatX" => "+_sparsemat",
		"cv_write_FileStorageX_const_StringX_const_StringX" => "+_str",
		"cv_write_FileStorageX_const_StringX_const_vector_DMatch_X" => "+_dmatch_vec",
		"cv_write_FileStorageX_const_StringX_const_vector_KeyPoint_X" => "+_keypoint_vec",
		"cv_write_FileStorageX_const_StringX_double" => "+_f64",
		"cv_write_FileStorageX_const_StringX_float" => "+_f32",
		"cv_write_FileStorageX_const_StringX_int" => "+_i32",
		"cv_write_FileStorage_DMatch" => "+_dmatch",
		"cv_write_FileStorage_KeyPoint" => "+_keypoint",
		"cv_write_FileStorage_Range" => "+_range",
		"cv_write_FileStorage_String_DMatch" => "+_dmatch",
		"cv_write_FileStorage_String_KeyPoint" => "+_keypoint",
		"cv_write_FileStorage_String_Range" => "+_range",
		"cv_write_FileStorage_VectorOfDMatch" => "+_dmatch_vec",
		"cv_write_FileStorage_VectorOfKeyPoint" => "+_keypoint_vec",

		"cv_AsyncArray__getImpl_const" => "-",
		"cv_MatExpr_op_const" => "-", // fixme implement PointerOf types
		"cv_Mat_Mat_const_MatX_const_RangeX" => "-", // duplicate of cv_Mat_Mat_Mat_VectorOfRange, but with pointers
		"cv_Mat_Mat_int_const_int_X_int" => "-", // duplicate of cv_Mat_Mat_VectorOfint_int, but with pointers
		"cv_Mat_Mat_int_const_int_X_int_Scalar" => "-", // duplicate of cv_Mat_Mat_VectorOfint_int_Scalar, but with pointers
		"cv_Mat_Mat_int_const_int_X_int_void_X_const_size_t_X" => "-", // duplicate of cv_Mat_Mat_VectorOfint_int_void_X_const_size_t_X, but with pointers
		"cv_Mat_copySize_const_MatX" => "-", // internal function
		"cv_Mat_push_back__const_voidX" => "-", // internal method
		"cv_Mat_setSize_MatSize" => "-", // MatSize and MatStep types prevent assignment
		"cv_Mat_setStep_MatStep" => "-", // MatSize and MatStep types prevent assignment
		"cv_Mat_set_size_MatSize" => "-", // doesn't allow writing
		"cv_Mat_set_step_MatStep" => "-", // same as above
		"cv_UMat_UMat_const_UMatX_const_RangeX" => "-", // duplicate of cv_UMat_UMat_UMat_VectorOfRange, but with pointers
		"cv_UMat_copySize_const_UMatX" => "-", // internal function
		"cv_UMat_setSize_MatSize" => "-", // MatSize and MatStep types prevent assignment
		"cv_UMat_setStep_MatStep" => "-", // MatSize and MatStep types prevent assignment
		"cv_UMat_set_size_MatSize" => "-", // doesn't allow writing
		"cv_UMat_set_step_MatStep" => "-", // same as above
		"cv_addImpl_int_const_charX" => "-",
		"cv_calcCovarMatrix_const_MatX_int_MatX_MatX_int_int" => "-", // duplicate of cv_calcCovarMatrix_const__InputArrayX_const__OutputArrayX_const__InputOutputArrayX_int_int, but with pointers
		"cv_cv_abs_short" => "-",
		"cv_cv_abs_uchar" => "-",
		"cv_detail_depthToString__int" => "-", // detail function
		"cv_detail_typeToString__int" => "-", // detail function
		"cv_double_const" => "-",
		"cv_fastFree_voidX" => "-", // manual memory allocation
		"cv_fastMalloc_size_t" => "-", // manual memory allocation
		"cv_float_const" => "-",
		"cv_format_const_charX" => "-", // 3.2 accepts varargs, duplicate definition
		"cv_hconcat_const_MatX_size_t_const__OutputArrayX" => "-", // duplicate of cv_hconcat_VectorOfMat_Mat, but with pointers
		"cv_int_const" => "-",
		"cv_mixChannels_const_MatX_size_t_MatX_size_t_const_intX_size_t" => "-", // duplicate of cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint, but with pointers
		"cv_ocl_ProgramSource_ProgramSource_const_charX" => "-", // has duplicate with String
		"cv_ocl_internal_isCLBuffer_UMat" => "-",
		"cv_ocl_internal_isOpenCLForced" => "-",
		"cv_ocl_internal_isPerformanceCheckBypassed" => "-",
		"cv_operator_std_string_const" => "-",
		"cv_read_FileNode_schar_schar" => "-",
		"cv_setImpl_int" => "-",
		"cv_setUseCollection_bool" => "-",
		"cv_useCollection" => "-",
		"cv_vconcat_const_MatX_size_t_const__OutputArrayX" => "-", // duplicate of cv_vconcat_VectorOfMat_Mat, but with pointers

		// ### dnn ###
		"cv_dnn_DictValue_DictValue_bool" => "from_bool",
		"cv_dnn_DictValue_DictValue_const_charX" => "from_str",
		"cv_dnn_DictValue_DictValue_double" => "from_f64",
		"cv_dnn_DictValue_DictValue_int" => "from_i32",
		"cv_dnn_DictValue_DictValue_int64_t" => "from_i64",
		"cv_dnn_DictValue_DictValue_unsigned_int" => "from_u32",
		"cv_dnn_DictValue_get_cv_String_const_int" => "+_str",
		"cv_dnn_DictValue_get_double_const_int" => "+_f64",
		"cv_dnn_DictValue_get_int64_t_const_int" => "+_i64",
		"cv_dnn_DictValue_get_int_const_int" => "+_i32",
		"cv_dnn_Dict_ptr_String" => "ptr_mut",
		"cv_dnn_Dict_set_cv_String_const_StringX_const_StringX" => "+_str",
		"cv_dnn_Dict_set_double_const_StringX_const_doubleX" => "+_f64",
		"cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX" => "+_i64",
		"cv_dnn_Layer_finalize_const_vector_Mat_X" => "+_mat",
		"cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X" => "+_mat_to",
		"cv_dnn_Layer_forward_vector_MatX_X_vector_Mat_X_vector_Mat_X" => "+_mat",
		"cv_dnn_NMSBoxes_const_vector_Rect2d_X_const_vector_float_X_float_float_vector_int_X_float_int" => "+_f64",
		"cv_dnn_Net_connect_String_String" => "+_first_second",
		"cv_dnn_Net_forward_const_StringX" => "+_single",
		"cv_dnn_Net_forward_const__OutputArrayX_const_StringX" => "+_layer",
		"cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_vector_int_X_vector_size_t_X_vector_size_t_X" => "+_for_layers",
		"cv_dnn_Net_getMemoryConsumption_const_int_const_vector_MatShape_X_size_tX_size_tX" => "+_for_layer",
		"cv_dnn_blobFromImage_const__InputArrayX_const__OutputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int" => "+_to",
		"cv_dnn_blobFromImages_const__InputArrayX_const__OutputArrayX_double_Size_const_ScalarX_bool_bool_int" => "+_to",
		"cv_dnn_clamp_Range_int" => "clamp_range",
		"cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t" => "+_str",
		"cv_dnn_readNetFromCaffe_const_vector_unsigned_char_X_const_vector_unsigned_char_X" => "+_buffer",
		"cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t" => "+_str",
		"cv_dnn_readNetFromDarknet_const_vector_unsigned_char_X_const_vector_unsigned_char_X" => "+_buffer",
		"cv_dnn_readNetFromONNX_const_charX_size_t" => "+_str",
		"cv_dnn_readNetFromONNX_const_vector_unsigned_char_X" => "+_buffer",
		"cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t" => "+_str",
		"cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_X_const_vector_unsigned_char_X" => "+_buffer",

		"cv_dnn_DictValue_DictValue_const_StringX" => "-", // effectively duplicate of cv_dnn_DictValue_DictValue_const_charX
		"cv_dnn_Layer_finalize_const_vector_MatX_X_vector_Mat_X" => "-", // dup of cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X

		// ### face ###
		"cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayX__const_cv__OutputArrayX__voidX__voidX" => "-", // fixme
		"cv_face_FacemarkLBF_Params_pupils" => "-", // fixme array of vectors
		"cv_face_FacemarkLBF_Params_setPupils_vector_int__X__2_" => "-", // fixme array of vectors

		// ### features2d ###
		"cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool" => "AGAST",
		"cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool_DetectorType" => "AGAST_with_type",
		"cv_AGAST_const__InputArrayX_vector_KeyPoint_X_int_bool_int" => "AGAST_with_type", // 3.x only
		"cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorExtractor_X_const_Ptr_DescriptorMatcher_X" => "+_with_dextractor",
		"cv_BOWImgDescriptorExtractor_compute2_const_MatX_vector_KeyPoint_X_MatX" => "compute2",
		"cv_BOWImgDescriptorExtractor_compute_const__InputArrayX_vector_KeyPoint_X_const__OutputArrayX_vector_vector_int__X_MatX" => "+_desc",
		"cv_BOWKMeansTrainer_BOWKMeansTrainer_int_TermCriteria_int_int" => "+_with_criteria",
		"cv_BOWTrainer_cluster_const_const_MatX" => "+_with_descriptors", // 3.2
		"cv_BRISK_create_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X" => "+_with_pattern",
		"cv_BRISK_create_int_int_const_vector_float_X_const_vector_int_X_float_float_const_vector_int_X" => "+_with_pattern_threshold_octaves",
		"cv_DescriptorMatcher_create_const_MatcherTypeX" => "+_with_matcher_type",
		"cv_DescriptorMatcher_create_int" => "+_with_matcher_type", // 3.x only
		"cv_DescriptorMatcher_knnMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_int_const__InputArrayX_bool" => "knn_train_match",
		"cv_DescriptorMatcher_match_const_const__InputArrayX_const__InputArrayX_vector_DMatch_X_const__InputArrayX" => "train_match",
		"cv_DescriptorMatcher_radiusMatch_const_const__InputArrayX_const__InputArrayX_vector_vector_DMatch__X_float_const__InputArrayX_bool" => "radius_train_match",
		"cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool" => "FAST",
		"cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool_DetectorType" => "FAST_with_type",
		"cv_FAST_const__InputArrayX_vector_KeyPoint_X_int_bool_int" => "FAST_with_type", // 3.x only
		"cv_Feature2D_compute_const__InputArrayX_vector_vector_KeyPoint__X_const__OutputArrayX" => "+_multiple",
		"cv_Feature2D_detect_const__InputArrayX_vector_vector_KeyPoint__X_const__InputArrayX" => "+_multiple",
		"cv_GFTTDetector_create_int_double_double_int_int_bool_double" => "+_with_gradient",

		// ### highgui ###
		"cv_addText_const_MatX_const_StringX_Point_const_StringX_int_Scalar_int_int_int" => "+_with_font",
		"cv_resizeWindow_const_StringX_const_SizeX" => "+_size",
		"cv_selectROI_const_StringX_const__InputArrayX_bool_bool" => "+_for_window",
		"cv_selectROIs_const_StringX_const__InputArrayX_vector_Rect_X_bool_bool" => "select_rois",

		// ### imgcodecs ###
		"cv_imdecode_const__InputArrayX_int_MatX" => "+_to",

		// ### imgproc ###
		"cv_Canny_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_double_bool" => "+_derivative",
		"cv_GeneralizedHough_detect_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX" => "+_with_edges",
		"cv_Subdiv2D_insert_const_vector_Point2f_X" => "+_multiple",
		"cv_applyColorMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX" => "+_user",
		"cv_clipLine_Size2l_Point2lX_Point2lX" => "+_size_i64",
		"cv_clipLine_Size_PointX_PointX" => "clip_line_size",
		"cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_X" => "ellipse_2_poly_f64",
		"cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_X" => "ellipse_2_poly",
		"cv_ellipse_const__InputOutputArrayX_const_RotatedRectX_const_ScalarX_int_int" => "ellipse_rotated_rect",
		"cv_findContours_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_Point" => "+_with_hierarchy", // 4.x
		"cv_findContours_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_Point" => "+_with_hierarchy", // 3.2 3.4
		"cv_floodFill_const__InputOutputArrayX_const__InputOutputArrayX_Point_Scalar_RectX_Scalar_Scalar_int" => "+_mask",
		"cv_getAffineTransform_const_Point2fX_const_Point2fX" => "+_slice",
		"cv_getPerspectiveTransform_const_Point2fX_const_Point2fX" => "+_slice", // 3.2 3.4
		"cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int" => "+_slice", // 4.x
		"cv_getRotationMatrix2D__Point2f_double_double" => "get_rotation_matrix_2d_matx",
		"cv_goodFeaturesToTrack_const__InputArrayX_const__OutputArrayX_int_double_double_const__InputArrayX_int_int_bool_double" => "+_with_gradient",

		"cv_calcBackProject_const_MatX_int_const_intX_const_SparseMatX_const__OutputArrayX_const_floatXX_double_bool" => "-", // slice pointers
		"cv_calcBackProject_const_MatX_int_const_intX_const__InputArrayX_const__OutputArrayX_const_floatXX_double_bool" => "-", // slice pointers
		"cv_calcHist_const_MatX_int_const_intX_const__InputArrayX_SparseMatX_int_const_intX_const_floatXX_bool_bool" => "-", // slice pointers
		"cv_calcHist_const_MatX_int_const_intX_const__InputArrayX_const__OutputArrayX_int_const_intX_const_floatXX_bool_bool" => "-", // slice pointers
		"cv_fillConvexPoly_MatX_const_PointX_int_const_ScalarX_int_int" => "-", // 3.2
		"cv_fillConvexPoly_Mat_const_Point_X_int_Scalar_int_int" => "-", // duplicate of cv_fillConvexPoly__InputOutputArrayX__InputArrayX_Scalar_int_int, but with pointers
		"cv_fillConvexPoly_const__InputOutputArrayX_const_PointX_int_const_ScalarX_int_int" => "-",
		"cv_fillPoly_MatX_const_PointXX_const_intX_int_const_ScalarX_int_int_Point" => "-", // 3.2
		"cv_fillPoly_const__InputOutputArrayX_const_PointXX_const_intX_int_const_ScalarX_int_int_Point" => "-",
		"cv_polylines_MatX_const_PointXX_const_intX_int_bool_const_ScalarX_int_int_int" => "-", // 3.2 3.4
		"cv_polylines_const__InputOutputArrayX_const_PointXX_const_intX_int_bool_const_ScalarX_int_int_int" => "-",

		// ### line_descriptor ###
		"cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_X_vector_vector_KeyLine__X_int_int_const_vector_Mat_X" => "+_multiple",

		// ### ml ###
		"cv_ml_ParamGrid_ParamGrid_double_double_double" => "for_range",
		"cv_ml_SVM_trainAuto_const__InputArrayX_int_const__InputArrayX_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool" => "+_with_data",
		"cv_ml_StatModel_train_const_Ptr_TrainData_X_int" => "+_with_data",

		// ### objdetect ###
		"cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size" => "+_num",
		"cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool" => "+_levels",
		"cv_CascadeClassifier_detectMultiScale__InputArrayX_VectorOfRect_VectorOfint_VectorOfdouble_double_int_int_Size_Size_bool" => "+_levels",
		"cv_CascadeClassifier_detectMultiScale__InputArrayX_VectorOfRect_VectorOfint_double_int_int_Size_Size" => "+_num",
		"cv_HOGDescriptor_HOGDescriptor_const_StringX" => "+_from_file",
		"cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_vector_double_X_double_Size_Size_double_double_bool" => "+_weights",
		"cv_HOGDescriptor_detect_const_const_MatX_vector_Point_X_vector_double_X_double_Size_Size_const_vector_Point_X" => "+_weights", // 3.2 3.4
		"cv_HOGDescriptor_detect_const_const__InputArrayX_vector_Point_X_vector_double_X_double_Size_Size_const_vector_Point_X" => "+_weights", // 4.x
		"cv_HOGDescriptor_setSvmDetector_vector_float_" => "+_vec",
		"cv_groupRectangles_vector_Rect_X_vector_int_X_vector_double_X_int_double" => "+_levels",
		"cv_groupRectangles_vector_Rect_X_vector_int_X_int_double" => "+_weights",
		"cv_groupRectangles_vector_Rect_X_int_double_vector_int_X_vector_double_X" => "+_levelweights",

		// ### photo ###
		"cv_fastNlMeansDenoisingMulti_const__InputArrayX_const__OutputArrayX_int_int_const_vector_float_X_int_int_int" => "+_vec",
		"cv_fastNlMeansDenoising_const__InputArrayX_const__OutputArrayX_const_vector_float_X_int_int_int" => "+_vec",
		"cv_fastNlMeansDenoising_Mat_Mat_float_int_int" => "+_window", // 3.x only?
		"cv_AlignMTB_process_const__InputArrayX_vector_Mat_X_const__InputArrayX_const__InputArrayX" => "+_with_response",
		"cv_MergeDebevec_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX" => "+_with_response",
		"cv_MergeMertens_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX" => "+_with_response",
		"cv_MergeRobertson_process_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX" => "+_with_response",

		// ### stitching ###
		"cv_Stitcher_composePanorama_const__InputArrayX_const__OutputArrayX" => "+_images",
		"cv_Stitcher_stitch_const__InputArrayX_const__InputArrayX_const__OutputArrayX" => "+_mask",
		"cv_Stitcher_stitch_const__InputArrayX_const_vector_vector_Rect__X_const__OutputArrayX" => "+_rois", // 3.2 3.4

		// ### surface_matching ###
		"cv_ppf_match_3d_ICP_registerModelToScene_const_MatX_const_MatX_vector_Pose3DPtr_X" => "+_vec",

		// ### text ###
		"cv_text_ERStat_pixels" => "-", // fixme: reference to a vector, we don't handle it too well yet
		"cv_text_ERStat_setPixels_vector_int_X" => "-", // fixme: reference to a vector, we don't handle it too well yet

		// ### videoio ###
		"cv_VideoCapture_VideoCapture_String_int" => "from_file_with_backend", // 3.2
		"cv_VideoCapture_VideoCapture_const_StringX" => "from_file_default", // 3.2
		"cv_VideoCapture_VideoCapture_const_StringX_int" => "from_file",
		"cv_VideoCapture_VideoCapture_int" => "+_default", // 3.4
		"cv_VideoCapture_open_const_StringX" => "+_file_default", // 3.2
		"cv_VideoCapture_open_const_StringX_int" => "+_file",
		"cv_VideoCapture_open_const_StringX_int" => "+_file",
		"cv_VideoCapture_open_int" => "+_device_default", // 3.2
		"cv_VideoWriter_VideoWriter_const_StringX_int_int_double_Size_bool" => "+_with_backend",
		"cv_VideoWriter_open_const_StringX_int_int_double_Size_bool" => "+_with_backend",

		// ### videostab ###
		"cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatX_const_MatX_boolX" => "+_mat",

		// ### xfeatures2d ###
		"cv_xfeatures2d_AffineFeature2D_create_Ptr_FeatureDetector__Ptr_DescriptorExtractor_" => "+_with_extractor",
	};
	if cfg!(target_env = "msvc") {
		// ### core ###
		out.insert("cv_MatConstIterator_MatConstIterator_const_MatX_const_intX", "-"); // missing in windows dll
		out.insert("cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX", "-"); // missing in windows dll
		out.insert("cv_getImpl_vector_int_X_vector_String_X", "-"); // missing in windows dll

		// ### dnn ###
		out.insert("cv_dnn_BackendNode_BackendNode_int", "-"); // missing in windows dll

		// ### imgproc ###
		out.insert("cv_getRotationMatrix2D__Point2f_double_double", "-"); // missing in windows dll

		// ### stitching ###
		out.insert("cv_createStitcherScans_bool", "-"); // missing in windows dll
		out.insert("cv_createStitcher_bool", "-"); // missing in windows dll

		// ### surface_matching ###
		out.insert("cv_ppf_match_3d_PPF3DDetector_read_const_FileNodeX", "-"); // missing in windows dll
		out.insert("cv_ppf_match_3d_PPF3DDetector_write_const_FileStorageX", "-"); // missing in windows dll
	}
	out
});

pub static ELEMENT_EXCLUDE: Lazy<RegexSet> = Lazy::new(|| RegexSet::new([
	"^cv::String$",
	"^cv::cuda::",
	"^cv::internal::format$", // 3.2 duplicate definition
	"^cv::ogl::",
	"^cv::face::FacemarkLBF::BBox$", // not used, not exported in windows dll
].iter()).expect("Can't compile regexes"));

pub static ELEMENT_IGNORE: Lazy<RegexSet> = Lazy::new(|| RegexSet::new([
	"^CV_DEPRECATED$",
	"^CV_EXPORTS$",
	"^CV_IMPL$", // 3.2
	"^CV_MAKE_TYPE$",
	"^CvFileNode$", // 3.2 3.4 C struct
	"^CvSeq$", // 3.2 C struct
	"^FILE$",
	"^HG_AUTOSIZE$", // 3.2
	"^cv::ErrorCallback$",
	"^cv::MatAllocator$", // doesn't handle cpp part too well
	"^cv::NAryMatIterator", // uses pointers of pointers
	"^cv::Node$", // template class
	"^cv::cuda::",
	"^cv::ogl::",
	"^std::exception_ptr$",
	"^std::random_access_iterator_tag$",
].iter()).expect("Can't compile regexes"));

pub static ELEMENT_EXPORT: Lazy<HashMap<&str, ExportConfig>> = Lazy::new(|| hashmap! {
	"VADisplay" => ExportConfig::default(),
	"VASurfaceID" => ExportConfig::default(),
	"cv::DetectionROI" => ExportConfig::default(),
	"cv::FileNodeIterator::SeqReader" => ExportConfig::default(),
	"cv::QtFont" => ExportConfig::default(),
	"cv::bioinspired::RetinaParameters" => ExportConfig::default(),
	"cv::bioinspired::SegmentationParameters" => ExportConfig::simple(),
	"cv::bioinspired::createRetina_OCL" => ExportConfig::default(), // 3.2 not exported
	"cv::detail::CheckContext" => ExportConfig::default(),
	"cv::dnn::BackendNode" => ExportConfig::default(),
	"cv::dnn::BackendWrapper" => ExportConfig::default(),
	"cv::dnn::DictValue" => ExportConfig::default(), // 3.2 not exported
	"cv::dnn::MatShape" => ExportConfig::default(),
	"cv::face::CParams" => ExportConfig::default(),
	"cv::face::FacemarkAAM::Model::Texture" => ExportConfig::default(),
	"cv::getElemSize" => ExportConfig::default(),
	"cv::morphologyDefaultBorderValue" => ExportConfig::default(),
	"cv::ppf_match_3d::Pose3DPtr" => ExportConfig::default(),
	"cv::superres::PyrLKOpticalFlow" => ExportConfig::default(),
	"cv::utils::logging::LogTag" => ExportConfig::default(),
	"cv::viz::Color" => ExportConfig::default(),
	"cvv::impl::CallMetaData" => ExportConfig::default(),
	"cv::dnn::_Range" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::slice" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::getPlane" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::shape" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::total" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::concat" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::toString" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::print" => ExportConfig::default(), // dnn shape_utils
	"cv::dnn::clamp" => ExportConfig::default(), // dnn shape_utils
	"cv::WarperCreator" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PlaneWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::AffineWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::CylindricalWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::SphericalWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::FisheyeWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::StereographicWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::CompressedRectilinearWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::CompressedRectilinearPortraitWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PaniniWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PaniniPortraitWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::MercatorWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::TransverseMercatorWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::TermCriteria" => ExportConfig::simple(),

	// override boxed
	"cv::DetectionBasedTracker::ExtObject" => ExportConfig::default(),
	"cv::DetectionBasedTracker::IDetector" => ExportConfig::default(),
	"cv::FileNode" => ExportConfig::default(), // return references in methods, generally looks like not simple
	"cv::detail::CameraParams" => ExportConfig::default(), // contains non-copy stuff
	"cv::detail::ImageFeatures" => ExportConfig::default(), // contains non-copy stuff
	"cv::detail::MatchesInfo" => ExportConfig::default(), // contains non-copy stuff
	"cv::detail::ProjectorBase" => ExportConfig::default(), // other classes inherit from this one
	"cv::dnn::ClassificationModel" => ExportConfig::default(),
	"cv::dnn::DetectionModel" => ExportConfig::default(),
	"cv::dnn::Model" => ExportConfig::default(),
	"cv::dnn::Net" => ExportConfig::default(), // incorrectly marked as simple
	"cv::ocl::Device" => ExportConfig::default(),
});

/// set of functions that should have unsafe in their declaration, element is Func.identifier()
pub static FUNC_UNSAFE: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	// allocates uninitialized memory
	"cv_Mat_Mat_Size_int",
	"cv_Mat_Mat_const_vector_int_X_int",
	"cv_Mat_Mat_int_const_intX_int",
	"cv_Mat_Mat_int_int_int",
	"cv_Mat_create_Size_int",
	"cv_Mat_create_const_vector_int_X_int",
	"cv_Mat_create_int_const_intX_int",
	"cv_Mat_create_int_int_int",
	"cv_UMat_UMat_Size_int_UMatUsageFlags",
	"cv_UMat_UMat_int_const_intX_int_UMatUsageFlags",
	"cv_UMat_UMat_int_int_int_UMatUsageFlags",
	"cv_UMat_create_Size_int_UMatUsageFlags",
	"cv_UMat_create_const_vector_int_X_int_UMatUsageFlags",
	"cv_UMat_create_int_const_intX_int_UMatUsageFlags",
	"cv_UMat_create_int_int_int_UMatUsageFlags",
	"cv__OutputArray_createSameSize_const_const__InputArrayX_int",
	"cv__OutputArray_create_const_Size_int_int_bool_int",
	"cv__OutputArray_create_const_int_const_intX_int_int_bool_int",
	"cv__OutputArray_create_const_int_int_int_int_bool_int",
	// allows passing arbitrary data
	"cv_Mat_Mat_int_int_int_voidX_size_t",
	"cv_Mat_Mat_Size_int_voidX_size_t",
	"cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX",
	"cv_Mat_Mat_const_vector_int_X_int_voidX_const_size_tX",
	"cv_Mat_setData_unsigned_charX",
	"cv_UMatData_setData_unsigned_charX",
	// no bounds checking
	"cv_Mat_ptr_const_const_intX",
	"cv_Mat_ptr_const_int",
	"cv_Mat_ptr_const_intX",
	"cv_Mat_ptr_const_int_int",
	"cv_Mat_ptr_const_int_int_int",
	"cv_Mat_ptr_int",
	"cv_Mat_ptr_int_int",
	"cv_Mat_ptr_int_int_int",
	// pointer to internal data
	"cv_dnn_Dict_ptr_String",
	"cv_dnn_Dict_ptr_const_String",
});

pub static IMPLEMENTED_FUNCTION_LIKE_MACROS: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"CV_MAKETYPE",
});

pub static CONST_TYPE_USIZE: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"Mat_AUTO_STEP",
});

/// map of reserved Rust keywords and their replacement to be used in var, function and class names
/// key: reserved keyword
/// value: replacement
pub static RESERVED_RENAME: Lazy<HashMap<&str, &str>> = Lazy::new(|| hashmap! {
	"box" => "box_",
	"fn" => "fn_",
	"in" => "in_",
	"match" => "match_",
	"move" => "move_",
	"ref" => "ref_",
	"type" => "typ",
	"use" => "use_",
	"impl" => "impl_",
//	"loop" => "loop_",
});

/// dict of functions with manual implementations
/// key: FuncInfo.identifier
/// value: dict
///   keys: "rust_safe", "rust_extern", "cpp", missing key means skip particular implementation
///   values: template to use for manual implementation or "~" to use default implementation
pub static FUNC_MANUAL: Lazy<HashMap<&str, CompiledInterpolation>> = Lazy::new(|| hashmap! {
	"cv_Mat_at_int" => include_str!("../tpl/func/rust_mat_at_mut.rs").compile_interpolation(),
	"cv_Mat_at_const_int" => include_str!("../tpl/func/rust_mat_at_const.rs").compile_interpolation(),
	"cv_Mat_at_int_int" => include_str!("../tpl/func/rust_mat_at_mut.rs").compile_interpolation(),
	"cv_Mat_at_const_int_int" => include_str!("../tpl/func/rust_mat_at_const.rs").compile_interpolation(),
	"cv_Mat_at_Point" => include_str!("../tpl/func/rust_mat_at_mut.rs").compile_interpolation(),
	"cv_Mat_at_const_Point" => include_str!("../tpl/func/rust_mat_at_const.rs").compile_interpolation(),
	"cv_Mat_at_int_int_int" => include_str!("../tpl/func/rust_mat_at_mut.rs").compile_interpolation(),
	"cv_Mat_at_const_int_int_int" => include_str!("../tpl/func/rust_mat_at_const.rs").compile_interpolation(),
	"cv_Mat_at_const_intX" => include_str!("../tpl/func/rust_mat_at_mut.rs").compile_interpolation(),
	"cv_Mat_at_const_const_intX" => include_str!("../tpl/func/rust_mat_at_const.rs").compile_interpolation(),
});

pub static FUNC_SPECIALIZE: Lazy<HashMap<&str, Vec<HashMap<&str, &str>>>> = Lazy::new(|| hashmap! {
	"cv_dnn_Dict_set_const_StringX_const_TX" => vec![
		hashmap! { "T" => "cv::String" },
		hashmap! { "T" => "cv::dnn::DictValue" },
		hashmap! { "T" => "double" },
		hashmap! { "T" => "int64_t" },
	],
	"cv_dnn_DictValue_get_const_int" => vec![
		hashmap! { "T" => "cv::String" },
		hashmap! { "T" => "double" },
		hashmap! { "T" => "int" },
		hashmap! { "T" => "int64_t" },
	],
});

/// set of classes that must be generated as traits, elements are Class.cpp_fullname()
pub static FORCE_CLASS_ABSTRACT: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::detail::BlocksCompensator",
});

pub static FORCE_CONSTANT_METHOD: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::Mat::size",
	"cv::Mat::step",
	"cv::UMat::size",
	"cv::UMat::step",
});

pub static PRIMITIVE_TYPEDEFS: Lazy<HashMap<&str, (&str, &str)>> = Lazy::new(|| hashmap! {
	"size_t" => ("size_t", "size_t"),
	"ptrdiff_t" => ("ptrdiff_t", "ptrdiff_t"),
	"clock_t" => ("clock_t", "clock_t"),
	"FILE" => ("FILE", "FILE"),
	"schar" => ("i8", "signed char"),
	"uchar" => ("u8", "unsigned char"),
	"uint8_t" => ("u8", "uint8_t"),
	"uint16_t" => ("u16", "uint16_t"),
	"uint" => ("u32", "unsigned int"),
	"uint32_t" => ("u32", "uint32_t"),
	"int64_t" => ("i64", "int64_t"),
	"int64" => ("i64", "int64_t"),
	"uint64_t" => ("u64", "uint64_t"),
	"uint64" => ("u64", "uint64_t"),
	"ushort" => ("u16", "unsigned short"),
});

pub static STATIC_MODULES: Lazy<BTreeSet<&str>> = Lazy::new(|| btreeset!{
	"core",
	"sys",
	"types",
});

// fixme, it can be made better
pub static DATA_TYPES: Lazy<BTreeSet<&str>> = Lazy::new(|| btreeset! {
	"unsigned char", "char", "unsigned short", "short", "int",
	"float", "double",
	"cv::Vec2b", "cv::Vec3b", "cv::Vec4b",
	"cv::Vec2s", "cv::Vec3s", "cv::Vec4s",
	"cv::Vec2w", "cv::Vec3w", "cv::Vec4w",
	"cv::Vec2i", "cv::Vec3i", "cv::Vec4i", "cv::Vec6i", "cv::Vec8i",
	"cv::Vec2f", "cv::Vec3f", "cv::Vec4f", "cv::Vec6f",
	"cv::Vec2d", "cv::Vec3d", "cv::Vec4d", "cv::Vec6d",
	"cv::Scalar",
	"cv::Point", "cv::Point2i", "cv::Point2f", "cv::Point2d", "cv::Point2l",
	"cv::Point3i", "cv::Point3f", "cv::Point3d",
	"cv::Size", "cv::Size2i", "cv::Size2f", "cv::Size2d", "cv::Size2l",
	"cv::Rect", "cv::Rect2i", "cv::Rect2f", "cv::Rect2d",
});

pub static IMPLEMENTED_GENERICS: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::Affine3",
	"cv::Mat_",
	"cv::Matx",
	"cv::Point3_",
	"cv::Point_",
	"cv::Rect_",
	"cv::Scalar_",
	"cv::Size_",
	"cv::Vec",
});

#[derive(Debug)]
pub enum SliceHint {
	ForceSlice(&'static str),
	ConvertSlice(&'static str, &'static str, usize),
}

/// rust_leafname
pub static SLICE_ARGUMENT: Lazy<HashMap<(&str, usize), SliceHint>> = Lazy::new(|| hashmap! {
	("cv::Mat::at", 1) => SliceHint::ForceSlice("idx"),
	("cv::Mat::ptr", 1) => SliceHint::ForceSlice("idx"),
	("cv::Mat::Mat", 4) => SliceHint::ForceSlice("steps"),
	("cv::Mat::Mat", 5) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::Mat::Mat", 4) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::Mat::zeros", 3) => SliceHint::ConvertSlice("sz", "ndims", 1),
	("cv::Mat::ones", 3) => SliceHint::ConvertSlice("sz", "ndims", 1),
	("cv::Mat::create", 3) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::Mat::reshape", 3) => SliceHint::ConvertSlice("newsz", "newndims", 1),
	("cv::SparseMat::Hdr::Hdr", 3) => SliceHint::ConvertSlice("_sizes", "_dims", 1),
	("cv::UMat::UMat", 4) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::UMat::UMat", 5) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::UMat::create", 4) => SliceHint::ConvertSlice("sizes", "ndims", 1),
	("cv::_OutputArray::create", 6) => SliceHint::ConvertSlice("size", "dims", 1),
	("cv::mixChannels", 4) => SliceHint::ConvertSlice("from_to", "npairs", 2),
});

pub static NO_SKIP_NAMESPACE_IN_LOCALNAME: Lazy<HashMap<&str, &str>> = Lazy::new(|| hashmap! {
	"detail" => "Detail",
	"fisheye" => "Fisheye",
	"superres" => "Superres",
});
