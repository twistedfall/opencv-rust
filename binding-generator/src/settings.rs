// todo add doccomments
// todo add missing slice forcings

use std::collections::{BTreeSet, HashMap, HashSet};

use maplit::{btreeset, hashmap, hashset};
use once_cell::sync::Lazy;

use crate::{CompiledInterpolation, ExportConfig, func::FuncId, StrExt};

/// map of functions to rename or skip, key is Func.identifier(), value is new name ("+" will be replaced by old name) or "-" to skip
pub static FUNC_RENAME: Lazy<HashMap<&str, &str>> = Lazy::new(|| hashmap! {
	// ### aruco ###
	"cv_aruco_getPredefinedDictionary_int" => "+_i32",

	// ### bioinspired ###
	"cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float" => "+_ext",
	"cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR" => "+_to",
	"cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR" => "+_to",
	"cv_bioinspired_Retina_setup_FileStorageR_const_bool" => "+_from_storage",
	"cv_bioinspired_Retina_setup_String_const_bool" => "+_from_file",
	"cv_bioinspired_Retina_write_const_FileStorageR" => "+_to_storage",
	"cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool" => "+_from_storage",
	"cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool" => "+_from_file",
	"cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR" => "+_to_storage",
	"cv_bioinspired_createRetina_OCL_Size_const_bool_int_const_bool_const_float_const_float" => "+_ext", // 3.2 only

	// ### calib3d ###
	"cv_LMSolver_create_const_Ptr_Callback_R_int_double" => "+_ext",
	"cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR" => "+_matrix",
	"cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double" => "+_mask",
	"cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double" => "+_ext",
	"cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR" => "fisheye_+",
	"cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR" => "fisheye_+",
	"cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR" => "fisheye_+_vec",
	"cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_int_TermCriteria" => "fisheye_+",
	"cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double" => "fisheye_+",
	"cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR" => "fisheye_+",
	"cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR" => "fisheye_+",
	"cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria" => "fisheye_+",
	"cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR" => "+_estimated",
	"cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR" => "+_triangulated",
	"cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_double_const__InputOutputArrayR" => "+_2_cameras",

	// ### core ###
	"cv_Algorithm_write_const_const_Ptr_FileStorage_R_const_StringR" => "+_with_name",
	"cv_AsyncArray_get_const_const__OutputArrayR_double" => "+_with_timeout_f64",
	"cv_AsyncArray_get_const_const__OutputArrayR_int64_t" => "+_with_timeout",
	"cv_AsyncArray_wait_for_const_double" => "+_f64",
	"cv_Cholesky_floatX_size_t_int_floatX_size_t_int" => "+_f32",
	"cv_DMatch_DMatch_int_int_int_float" => "new_index",
	"cv_FileStorage_write_const_StringR_const_MatR" => "+_mat",
	"cv_FileStorage_write_const_StringR_const_StringR" => "+_str",
	"cv_FileStorage_write_const_StringR_const_vector_String_R" => "+_str_vec",
	"cv_FileStorage_write_const_StringR_double" => "+_f64",
	"cv_FileStorage_write_const_StringR_int" => "+_i32",
	"cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int" => "+_point",
	"cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int" => "+_coords",
	"cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int" => "+_to",
	"cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int" => "+_with_data",
	"cv_LU_floatX_size_t_int_floatX_size_t_int" => "lu_f32",
	"cv_MatConstIterator_MatConstIterator_const_MatX" => "over",
	"cv_MatConstIterator_MatConstIterator_const_MatX_Point" => "with_start",
	"cv_MatConstIterator_MatConstIterator_const_MatX_const_intX" => "+_slice",
	"cv_MatConstIterator_MatConstIterator_const_MatX_int_int" => "with_rows_cols",
	"cv_MatConstIterator_pos_const_intX" => "+_to",
	"cv_MatConstIterator_seek_const_intX_bool" => "+_idx",
	"cv_MatExpr_MatExpr_const_MatR" => "from_mat",
	"cv_MatExpr_mul_const_const_MatExprR_double" => "+_matexpr",
	"cv_MatExpr_type_const" => "typ",
	"cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR" => "+_scalar",
	"cv_MatOp_divide_const_double_const_MatExprR_MatExprR" => "+_f64",
	"cv_MatOp_multiply_const_const_MatExprR_double_MatExprR" => "+_f64",
	"cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR" => "+_scalar",
	"cv_Mat_Mat_Size_int" => "+_size",
	"cv_Mat_Mat_Size_int_const_ScalarR" => "+_size_with_default",
	"cv_Mat_Mat_Size_int_voidX_size_t" => "+_size_with_data",
	"cv_Mat_Mat_const_GpuMatR" => "from_gpumat",
	"cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR" => "rowscols",
	"cv_Mat_Mat_const_MatR_const_RectR" => "roi",
	"cv_Mat_Mat_const_MatR_const_vector_Range_R" => "ranges",
	"cv_Mat_Mat_const_vector_int_R_int" => "+_nd_vec",
	"cv_Mat_Mat_const_vector_int_R_int_const_ScalarR" => "+_nd_vec_with_default",
	"cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX" => "+_nd_vec_with_data",
	"cv_Mat_Mat_int_const_intX_int" => "+_nd",
	"cv_Mat_Mat_int_const_intX_int_const_ScalarR" => "+_nd_with_default",
	"cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX" => "+_nd_with_data",
	"cv_Mat_Mat_int_int_int" => "+_rows_cols",
	"cv_Mat_Mat_int_int_int_const_ScalarR" => "+_rows_cols_with_default",
	"cv_Mat_Mat_int_int_int_voidX_size_t" => "+_rows_cols_with_data",
	"cv_Mat_at_Point" => "+_pt_mut",
	"cv_Mat_at_const_Point" => "+_pt",
	"cv_Mat_at_const_const_intX" => "+_nd",
	"cv_Mat_at_const_intX" => "+_nd_mut",
	"cv_Mat_at_const_int_int" => "+_2d",
	"cv_Mat_at_const_int_int_int" => "+_3d",
	"cv_Mat_at_int" => "+_mut",
	"cv_Mat_at_int_int" => "+_2d_mut",
	"cv_Mat_at_int_int_int" => "+_3d_mut",
	"cv_Mat_colRange_const_int_int" => "col_bounds",
	"cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR" => "+_masked",
	"cv_Mat_create_Size_int" => "+_size",
	"cv_Mat_create_const_vector_int_R_int" => "+_nd_vec",
	"cv_Mat_create_int_const_intX_int" => "+_nd",
	"cv_Mat_create_int_int_int" => "+_rows_cols",
	"cv_Mat_diag_const_MatR" => "+_mat",
	"cv_Mat_eye_Size_int" => "+_size",
	"cv_Mat_getPropData" => "+_mut",
	"cv_Mat_getPropSize_const" => "mat_size",
	"cv_Mat_getPropStep_const" => "mat_step",
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
	"cv_Mat_reshape_const_int_const_vector_int_R" => "+_nd_vec",
	"cv_Mat_reshape_const_int_int_const_intX" => "+_nd",
	"cv_Mat_resize_size_t_const_ScalarR" => "+_with_default",
	"cv_Mat_rowRange_const_int_int" => "row_bounds",
	"cv_Mat_total_const_int_int" => "total_slice",
	"cv_Mat_type_const" => "typ",
	"cv_Mat_zeros_Size_int" => "+_size",
	"cv_Mat_zeros_int_const_intX_int" => "+_nd",
	"cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double" => "+_variance",
	"cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double" => "+_variance",
	"cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double" => "+_with_variance",
	"cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR" => "+_to",
	"cv_PCA_project_const_const__InputArrayR_const__OutputArrayR" => "+_to",
	"cv_Range_Range_int_int" => "new",
	"cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR" => "for_points",
	"cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR" => "+_multi",
	"cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int" => "+_ext",
	"cv_SparseMat_SparseMat_const_MatR" => "from_mat",
	"cv_SparseMat_begin" => "+_mut",
	"cv_SparseMat_copyTo_const_MatR" => "+_mat",
	"cv_SparseMat_end" => "+_mut",
	"cv_UMat_UMat_Size_int_UMatUsageFlags" => "+_size",
	"cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags" => "+_size_with_default",
	"cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR" => "rowscols",
	"cv_UMat_UMat_const_UMatR_const_RectR" => "roi",
	"cv_UMat_UMat_const_UMatR_const_vector_Range_R" => "ranges",
	"cv_UMat_UMat_int_const_intX_int_UMatUsageFlags" => "+_nd",
	"cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags" => "+_nd_with_default",
	"cv_UMat_UMat_int_int_int_UMatUsageFlags" => "+_rows_cols",
	"cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags" => "+_rows_cols_with_default",
	"cv_UMat_colRange_const_int_int" => "col_bounds",
	"cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR" => "+_masked",
	"cv_UMat_create_Size_int_UMatUsageFlags" => "+_size",
	"cv_UMat_create_const_vector_int_R_int_UMatUsageFlags" => "+_nd_vec",
	"cv_UMat_create_int_const_intX_int_UMatUsageFlags" => "+_nd",
	"cv_UMat_create_int_int_int_UMatUsageFlags" => "+_rows_cols",
	"cv_UMat_getPropSize_const" => "mat_size",
	"cv_UMat_getPropStep_const" => "mat_step",
	"cv_UMat_rowRange_const_int_int" => "row_bounds",
	"cv_UMat_type_const" => "typ",
	"cv__InputArray__InputArray_const_GpuMatR" => "from_gpumat",
	"cv__InputArray__InputArray_const_HostMemR" => "from_hostmem",
	"cv__InputArray__InputArray_const_MatExprR" => "from_matexpr",
	"cv__InputArray__InputArray_const_MatR" => "from_mat",
	"cv__InputArray__InputArray_const_UMatR" => "from_umat",
	"cv__InputArray__InputArray_const_doubleR" => "from_f64",
	"cv__InputArray__InputArray_const_vector_GpuMat_R" => "from_gpumat_vec",
	"cv__InputArray__InputArray_const_vector_Mat_R" => "from_mat_vec",
	"cv__InputArray__InputArray_const_vector_UMat_R" => "from_umat_vec",
	"cv__InputArray__InputArray_const_vector_bool_R" => "from_bool_vec",
	"cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR" => "+_masked",
	"cv__InputOutputArray__InputOutputArray_GpuMatR" => "from_gpumat_mut",
	"cv__InputOutputArray__InputOutputArray_HostMemR" => "from_hostmem_mut",
	"cv__InputOutputArray__InputOutputArray_MatR" => "from_mat_mut",
	"cv__InputOutputArray__InputOutputArray_UMatR" => "from_umat_mut",
	"cv__InputOutputArray__InputOutputArray_const_GpuMatR" => "from_gpumat",
	"cv__InputOutputArray__InputOutputArray_const_HostMemR" => "from_hostmem",
	"cv__InputOutputArray__InputOutputArray_const_MatR" => "from_mat",
	"cv__InputOutputArray__InputOutputArray_const_UMatR" => "from_umat",
	"cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R" => "from_gpumat_vec",
	"cv__InputOutputArray__InputOutputArray_const_vector_Mat_R" => "from_mat_vec",
	"cv__InputOutputArray__InputOutputArray_const_vector_UMat_R" => "from_umat_vec",
	"cv__InputOutputArray__InputOutputArray_vector_Mat_R" => "from_mat_vec_mut",
	"cv__InputOutputArray__InputOutputArray_vector_UMat_R" => "from_umat_vec_mut",
	"cv__OutputArray__OutputArray_GpuMatR" => "from_gpumat_mut",
	"cv__OutputArray__OutputArray_HostMemR" => "from_hostmem_mut",
	"cv__OutputArray__OutputArray_MatR" => "from_mat_mut",
	"cv__OutputArray__OutputArray_UMatR" => "from_umat_mut",
	"cv__OutputArray__OutputArray_const_GpuMatR" => "from_gpumat",
	"cv__OutputArray__OutputArray_const_HostMemR" => "from_hostmem",
	"cv__OutputArray__OutputArray_const_MatR" => "from_mat",
	"cv__OutputArray__OutputArray_const_UMatR" => "from_umat",
	"cv__OutputArray__OutputArray_const_vector_GpuMat_R" => "from_gpumat_vec",
	"cv__OutputArray__OutputArray_const_vector_Mat_R" => "from_mat_vec",
	"cv__OutputArray__OutputArray_const_vector_UMat_R" => "from_umat_vec",
	"cv__OutputArray__OutputArray_vector_GpuMat_R" => "from_gpumat_vec_mut",
	"cv__OutputArray__OutputArray_vector_Mat_R" => "from_mat_vec_mut",
	"cv__OutputArray__OutputArray_vector_UMat_R" => "from_umat_vec_mut",
	"cv__OutputArray_create_const_Size_int_int_bool_DepthMask" => "+_size",
	"cv__OutputArray_create_const_Size_int_int_bool_int" => "+_size", // 3.2
	"cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask" => "+_nd",
	"cv__OutputArray_create_const_int_const_intX_int_int_bool_int" => "+_nd", // 3.2
	"cv_abs_const_MatExprR" => "+_matexpr",
	"cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX" => "+_size",
	"cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX" => "+_size_with_default",
	"cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t" => "+_size_with_data",
	"cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range" => "rowscols",
	"cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect" => "roi",
	"cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX" => "from_hostmem",
	"cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX" => "+_rows_cols",
	"cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX" => "+_rows_cols_with_default",
	"cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t" => "+_rows_cols_with_data",
	"cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR" => "+_async",
	"cv_cuda_GpuMat_upload_const__InputArrayR_StreamR" => "+_async",
	"cv_directx_getTypeFromD3DFORMAT_const_int" => "get_type_from_d3d_format",
	"cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int" => "+2",
	"cv_getNumberOfCPUs" => "get_number_of_cpus",
	"cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR" => "+2",
	"cv_max_const_MatR_const_MatR" => "+_mat",
	"cv_max_const_MatR_const_MatR_MatR" => "+_mat_to",
	"cv_max_const_MatR_double" => "+_mat_f64",
	"cv_max_const_UMatR_const_UMatR_UMatR" => "+_umat_to",
	"cv_max_double_const_MatR" => "+_f64_mat",
	"cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX" => "+_sparse",
	"cv_min_const_MatR_const_MatR" => "+_mat",
	"cv_min_const_MatR_const_MatR_MatR" => "+_mat_to",
	"cv_min_const_MatR_double" => "+_mat_f64",
	"cv_min_const_UMatR_const_UMatR_UMatR" => "+_umat_to",
	"cv_min_double_const_MatR" => "+_f64_mat",
	"cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R" => "+_vec",
	"cv_norm_const_SparseMatR_int" => "+_sparse",
	"cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR" => "+2",
	"cv_normalize_const_SparseMatR_SparseMatR_double_int" => "+_sparse",
	"cv_ocl_Context_Context_int" => "+_with_type",
	"cv_ocl_Context_create_int" => "+_with_type",
	"cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX" => "+_ext",
	"cv_ocl_Kernel_set_int_const_KernelArgR" => "+_kernel_arg",
	"cv_ocl_Kernel_set_int_const_UMatR" => "+_umat",
	"cv_ocl_ProgramSource_ProgramSource_const_StringR" => "from_str",
	"cv_ocl_Program_getPrefix_const_StringR" => "+_build_flags",
	"cv_ogl_Buffer_create_Size_int_Target_bool" => "+_size",
	"cv_operatorA_const_MatExprR_const_MatExprR" => "+_matexpr_matexpr",
	"cv_operatorA_const_MatExprR_const_MatR" => "+_matexpr_mat",
	"cv_operatorA_const_MatExprR_const_ScalarR" => "+_matexpr_scalar",
	"cv_operatorA_const_MatR_const_MatExprR" => "+_mat_matexpr",
	"cv_operatorA_const_MatR_const_MatR" => "+_mat_mat",
	"cv_operatorA_const_MatR_const_ScalarR" => "+_mat_scalar",
	"cv_operatorA_const_ScalarR_const_MatExprR" => "+_scalar_matexpr",
	"cv_operatorA_const_ScalarR_const_MatR" => "+_scalar_mat",
	"cv_operatorD_const_MatExprR_const_MatExprR" => "+_matexpr_matexpr",
	"cv_operatorD_const_MatExprR_const_MatR" => "+_matexpr_mat",
	"cv_operatorD_const_MatExprR_double" => "+_matexpr_f64",
	"cv_operatorD_const_MatR_const_MatExprR" => "+_mat_matexpr",
	"cv_operatorD_const_MatR_const_MatR" => "+_mat_mat",
	"cv_operatorD_const_MatR_double" => "+_mat_f64",
	"cv_operatorD_double_const_MatExprR" => "+_f64_matexpr",
	"cv_operatorD_double_const_MatR" => "+_f64_mat",
	"cv_operatorEQ_const_FileNodeIteratorR_const_FileNodeIteratorR" => "+_filenode_iter",
	"cv_operatorEQ_const_MatR_double" => "+_mat_f64",
	"cv_operatorEQ_double_const_MatR" => "+_f64_mat",
	"cv_operatorS_const_MatExprR" => "+_matexpr",
	"cv_operatorS_const_MatExprR_const_MatExprR" => "+_matexpr_matexpr",
	"cv_operatorS_const_MatExprR_const_MatR" => "+_matexpr_mat",
	"cv_operatorS_const_MatExprR_const_ScalarR" => "+_matexpr_scalar",
	"cv_operatorS_const_MatR" => "+_mat",
	"cv_operatorS_const_MatR_const_MatExprR" => "+_mat_matexpr",
	"cv_operatorS_const_MatR_const_MatR" => "+_mat_mat",
	"cv_operatorS_const_MatR_const_ScalarR" => "+_mat_scalar",
	"cv_operatorS_const_ScalarR_const_MatExprR" => "+_scalar_matexpr",
	"cv_operatorS_const_ScalarR_const_MatR" => "+_scalar_mat",
	"cv_operatorX_const_MatExprR_const_MatExprR" => "+_matexpr_matexpr",
	"cv_operatorX_const_MatExprR_const_MatR" => "+_matexpr_mat",
	"cv_operatorX_const_MatExprR_double" => "+_matexpr_f64",
	"cv_operatorX_const_MatR_const_MatExprR" => "+_mat_matexpr",
	"cv_operatorX_const_MatR_const_MatR" => "+_mat_mat",
	"cv_operatorX_const_MatR_double" => "+_mat_f64",
	"cv_operatorX_double_const_MatExprR" => "+_f64_matexpr",
	"cv_operatorX_double_const_MatR" => "+_f64_mat",
	"cv_read_const_FileNodeR_DMatchR_const_DMatchR" => "+_dmatch",
	"cv_read_const_FileNodeR_KeyPointR_const_KeyPointR" => "+_keypoint",
	"cv_read_const_FileNodeR_MatR_const_MatR" => "+_mat",
	"cv_read_const_FileNodeR_SparseMatR_const_SparseMatR" => "+_sparsemat",
	"cv_read_const_FileNodeR_doubleR_double" => "+_f64",
	"cv_read_const_FileNodeR_floatR_float" => "+_f32",
	"cv_read_const_FileNodeR_intR_int" => "+_i32",
	"cv_read_const_FileNodeR_stringR_const_stringR" => "+_str",
	"cv_read_const_FileNodeR_vector_DMatch_R" => "+_dmatch_vec_legacy",
	"cv_read_const_FileNodeR_vector_KeyPoint_R" => "+_keypoint_vec_legacy",
	"cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int" => "+_points",
	"cv_repeat_const__InputArrayR_int_int_const__OutputArrayR" => "+_to",
	"cv_split_const_MatR_MatX" => "+_slice",
	"cv_swap_UMatR_UMatR" => "+_umat",
	"cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR" => "+2",
	"cv_writeScalar_FileStorageR_const_StringR" => "+_str",
	"cv_writeScalar_FileStorageR_double" => "+_f64",
	"cv_writeScalar_FileStorageR_float" => "+_f32",
	"cv_writeScalar_FileStorageR_int" => "+_i32",
	"cv_write_FileStorageR_const_StringR_const_MatR" => "+_mat",
	"cv_write_FileStorageR_const_StringR_const_SparseMatR" => "+_sparsemat",
	"cv_write_FileStorageR_const_StringR_const_StringR" => "+_str",
	"cv_write_FileStorageR_const_StringR_const_vector_DMatch_R" => "+_dmatch_vec",
	"cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R" => "+_keypoint_vec",
	"cv_write_FileStorageR_const_StringR_double" => "+_f64",
	"cv_write_FileStorageR_const_StringR_float" => "+_f32",
	"cv_write_FileStorageR_const_StringR_int" => "+_i32",

	"cv_AsyncArray__getImpl_const" => "-",
	"cv_MatExpr_getPropOp_const" => "-", // fixme implement PointerOf types
	"cv_Mat_Mat_const_MatR_const_RangeX" => "-", // duplicate of cv_Mat_Mat_Mat_VectorOfRange, but with pointers
	"cv_Mat_copySize_const_MatR" => "-", // internal function
	"cv_Mat_push_back__const_voidX" => "-", // internal method
	"cv_Mat_setPropSize_MatSize" => "-", // MatSize and MatStep types prevent assignment
	"cv_Mat_setPropStep_MatStep" => "-", // MatSize and MatStep types prevent assignment
	"cv_UMat_UMat_const_UMatR_const_RangeX" => "-", // duplicate of cv_UMat_UMat_UMat_VectorOfRange, but with pointers
	"cv_UMat_copySize_const_UMatR" => "-", // internal function
	"cv_UMat_setPropSize_MatSize" => "-", // MatSize and MatStep types prevent assignment
	"cv_UMat_setPropStep_MatStep" => "-", // MatSize and MatStep types prevent assignment
	"cv_addImpl_int_const_charX" => "-",
	"cv_calcCovarMatrix_const_MatX_int_MatR_MatR_int_int" => "-", // duplicate of cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int, but with pointers
	"cv_cv_abs_short" => "-",
	"cv_detail_depthToString__int" => "-", // detail function
	"cv_detail_typeToString__int" => "-", // detail function
	"cv_fastFree_voidX" => "-", // manual memory allocation
	"cv_fastMalloc_size_t" => "-", // manual memory allocation
	"cv_format_const_charX" => "-", // 3.2 accepts varargs, duplicate definition
	"cv_hconcat_const_MatX_size_t_const__OutputArrayR" => "-", // duplicate of cv_hconcat_VectorOfMat_Mat, but with pointers
	"cv_merge_const_MatX_size_t_const__OutputArrayR" => "-", // duplicate of cv_merge_const__InputArrayR_const__OutputArrayR, but with pointers
	"cv_mixChannels_const_MatX_size_t_MatX_size_t_const_intX_size_t" => "-", // duplicate of cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint, but with pointers
	"cv_ocl_ProgramSource_ProgramSource_const_charX" => "-", // has duplicate with String
	"cv_setImpl_int" => "-",
	"cv_setUseCollection_bool" => "-",
	"cv_useCollection" => "-",
	"cv_vconcat_const_MatX_size_t_const__OutputArrayR" => "-", // duplicate of cv_vconcat_VectorOfMat_Mat, but with pointers

	// ### cudaimgproc ###
	"cv_cuda_histEven_const__InputArrayR_GpuMatXX_intXX_intXX_intXX_StreamR" => "-", // slice of boxed objects
	"cv_cuda_histRange_const__InputArrayR_GpuMatXX_const_GpuMatXX_StreamR" => "-", // slice of boxed objects

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
	"cv_dnn_Dict_ptr_const_StringR" => "+_mut",
	"cv_dnn_Dict_set_cv_String_const_StringR_const_StringR" => "+_str",
	"cv_dnn_Dict_set_double_const_StringR_const_doubleR" => "+_f64",
	"cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR" => "+_i64",
	"cv_dnn_Layer_finalize_const_vector_Mat_R" => "+_mat",
	"cv_dnn_Layer_finalize_const_vector_Mat_R_vector_Mat_R" => "+_mat_to",
	"cv_dnn_Layer_forward_vector_MatX_R_vector_Mat_R_vector_Mat_R" => "+_mat",
	"cv_dnn_NMSBoxes_const_vector_Rect2d_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int" => "+_f64",
	"cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_const_intR_LayerParamsR" => "+_type",
	"cv_dnn_Net_addLayer_const_StringR_const_StringR_const_intR_LayerParamsR" => "+_type",
	"cv_dnn_Net_connect_String_String" => "+_first_second",
	"cv_dnn_Net_forward_const_StringR" => "+_single",
	"cv_dnn_Net_forward_const__OutputArrayR_const_StringR" => "+_layer",
	"cv_dnn_Net_getMemoryConsumption_const_const_int_const_vector_MatShape_R_size_tR_size_tR" => "+_for_layer",
	"cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_vector_int_R_vector_size_t_R_vector_size_t_R" => "+_for_layers",
	"cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR_const_stringR" => "from_file",
	"cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vector_vector_Point__R_vector_float_R" => "+_with_confidences",
	"cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR_const_stringR" => "from_file",
	"cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int" => "+_to",
	"cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int" => "+_to",
	"cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t" => "+_str",
	"cv_dnn_readNetFromCaffe_const_vector_unsigned_char_R_const_vector_unsigned_char_R" => "+_buffer",
	"cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t" => "+_str",
	"cv_dnn_readNetFromDarknet_const_vector_unsigned_char_R_const_vector_unsigned_char_R" => "+_buffer",
	"cv_dnn_readNetFromONNX_const_charX_size_t" => "+_str",
	"cv_dnn_readNetFromONNX_const_vector_unsigned_char_R" => "+_buffer",
	"cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t" => "+_str",
	"cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_R_const_vector_unsigned_char_R" => "+_buffer",

	"cv_dnn_DictValue_DictValue_const_StringR" => "-", // effectively duplicate of cv_dnn_DictValue_DictValue_const_charX
	"cv_dnn_Layer_finalize_const_vector_MatX_R_vector_Mat_R" => "-", // dup of cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X
	"cv_dnn_Model_operator_cv_dnn_Net_const" => "-", // fixme, should generate fine, it's a dup of get_network_() anyway

	// ### face ###
	"cv_face_FacemarkLBF_Params_getPropPupils" => "-", // fixme array of vectors

	// ### features2d ###
	"cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool" => "AGAST",
	"cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType" => "AGAST_with_type",
	"cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_int" => "AGAST_with_type", // 3.x only
	"cv_BOWImgDescriptorExtractor_compute2_const_MatR_vector_KeyPoint_R_MatR" => "compute2",
	"cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_vector_vector_int__X_MatX" => "+_desc",
	"cv_BOWTrainer_cluster_const_const_MatR" => "+_with_descriptors", // 3.2
	"cv_BRISK_create_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R" => "+_with_pattern",
	"cv_BRISK_create_int_int_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R" => "+_with_pattern_threshold_octaves",
	"cv_DescriptorMatcher_create_const_MatcherTypeR" => "+_with_matcher_type",
	"cv_DescriptorMatcher_create_int" => "+_with_matcher_type", // 3.x only
	"cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool" => "knn_train_match",
	"cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR" => "train_match",
	"cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool" => "radius_train_match",
	"cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool" => "FAST",
	"cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType" => "FAST_with_type",
	"cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_int" => "FAST_with_type", // 3.x only
	"cv_Feature2D_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR" => "+_multiple",
	"cv_Feature2D_detect_const__InputArrayR_vector_vector_KeyPoint__R_const__InputArrayR" => "+_multiple",
	"cv_GFTTDetector_create_int_double_double_int_int_bool_double" => "+_with_gradient",

	// ### gapi ###
	// "cv_GComputation_apply_const_vector_Mat_R_const_vector_Mat_R_GCompileArgsR" => "-",
	// "cv_GComputation_apply_const_vector_Mat_R_vector_Mat_R_GCompileArgsR" => "-",
	// "cv_GComputation_apply_Mat_Mat_ScalarR_GCompileArgsR" => "-",
	// "cv_GComputation_apply_Mat_Mat_MatR_GCompileArgsR" => "-",
	// "cv_GComputation_apply_Mat_ScalarR_GCompileArgsR" => "-",
	// "cv_GComputation_apply_Mat_MatR_GCompileArgsR" => "-",

	// ### highgui ###
	"cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int" => "+_with_font",
	"cv_resizeWindow_const_StringR_const_SizeR" => "+_size",
	"cv_selectROI_const_StringR_const__InputArrayR_bool_bool" => "+_for_window",
	"cv_selectROIs_const_StringR_const__InputArrayR_vector_Rect_R_bool_bool" => "select_rois",

	// ### imgcodecs ###
	"cv_imdecode_const__InputArrayR_int_MatX" => "+_to",
	"cv_imreadmulti_const_StringR_vector_Mat_R_int_int_int" => "+_range",

	// ### imgproc ###
	"cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool" => "+_derivative",
	"cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR" => "+_with_edges",
	"cv_Subdiv2D_insert_const_vector_Point2f_R" => "+_multiple",
	"cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR" => "+_user",
	"cv_clipLine_Size2l_Point2lR_Point2lR" => "+_size_i64",
	"cv_clipLine_Size_PointR_PointR" => "clip_line_size",
	"cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_R" => "ellipse_2_poly_f64",
	"cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_R" => "ellipse_2_poly",
	"cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int" => "ellipse_rotated_rect",
	"cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point" => "+_with_hierarchy", // 4.x
	"cv_findContours_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point" => "+_with_hierarchy", // 3.2 3.4
	"cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int" => "+_mask",
	"cv_getAffineTransform_const_Point2fX_const_Point2fX" => "+_slice",
	"cv_getPerspectiveTransform_const_Point2fX_const_Point2fX" => "+_slice", // 3.2 3.4
	"cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int" => "+_slice", // 4.x
	"cv_getRotationMatrix2D__Point2f_double_double" => "get_rotation_matrix_2d_matx",
	"cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double" => "+_with_gradient",

	"cv_calcBackProject_const_MatX_int_const_intX_const_SparseMatR_const__OutputArrayR_const_floatXX_double_bool" => "-", // slice pointers
	"cv_calcBackProject_const_MatX_int_const_intX_const__InputArrayR_const__OutputArrayR_const_floatXX_double_bool" => "-", // slice pointers
	"cv_calcHist_const_MatX_int_const_intX_const__InputArrayR_SparseMatR_int_const_intX_const_floatXX_bool_bool" => "-", // slice pointers
	"cv_calcHist_const_MatX_int_const_intX_const__InputArrayR_const__OutputArrayR_int_const_intX_const_floatXX_bool_bool" => "-", // slice pointers
	"cv_fillConvexPoly_MatR_const_PointX_int_const_ScalarR_int_int" => "-", // 3.2 3.4
	"cv_fillConvexPoly_const__InputOutputArrayR_const_PointX_int_const_ScalarR_int_int" => "-",
	"cv_fillPoly_MatR_const_PointXX_const_intX_int_const_ScalarR_int_int_Point" => "-", // 3.2
	"cv_fillPoly_const__InputOutputArrayR_const_PointXX_const_intX_int_const_ScalarR_int_int_Point" => "-",
	"cv_polylines_MatR_const_PointXX_const_intX_int_bool_const_ScalarR_int_int_int" => "-", // 3.2 3.4
	"cv_polylines_const__InputOutputArrayR_const_PointXX_const_intX_int_bool_const_ScalarR_int_int_int" => "-",

	// ### line_descriptor ###
	"cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_int_int_const_vector_Mat_R" => "+_multiple",

	// ### ml ###
	"cv_ml_ParamGrid_ParamGrid_double_double_double" => "for_range",
	"cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool" => "+_with_data",
	"cv_ml_StatModel_train_const_Ptr_TrainData_R_int" => "+_with_data",

	// ### objdetect ###
	"cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size" => "+_num",
	"cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool" => "+_levels",
	"cv_HOGDescriptor_HOGDescriptor_const_StringR" => "+_from_file",
	"cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_vector_double_R_double_Size_Size_double_double_bool" => "+_weights",
	"cv_HOGDescriptor_detect_const_const_MatR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R" => "+_weights", // 3.2 3.4
	"cv_HOGDescriptor_detect_const_const__InputArrayR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R" => "+_weights", // 4.x
	"cv_HOGDescriptor_setPropSvmDetector_vector_float_" => "+_vec",
	"cv_groupRectangles_vector_Rect_R_vector_int_R_vector_double_R_int_double" => "+_levels",
	"cv_groupRectangles_vector_Rect_R_vector_int_R_int_double" => "+_weights",
	"cv_groupRectangles_vector_Rect_R_int_double_vector_int_X_vector_double_X" => "+_levelweights",

	"cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vector_string_R_const__OutputArrayR_const__OutputArrayR" => "-", // fixme: stores data to Vector<String>, that doesn't work yet

	// ### optflow ###
	"cv_optflow_GPCTrainingSamples_operator_cv_optflow_GPCSamplesVector" => "-", // support of "operator &" missing

	// ### photo ###
	"cv_AlignMTB_process_const__InputArrayR_vector_Mat_R_const__InputArrayR_const__InputArrayR" => "+_with_response",
	"cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR" => "+_with_response",
	"cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR" => "+_with_response",
	"cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR" => "+_with_response",
	"cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int_StreamR" => "+_cuda",
	"cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_StreamR" => "+_cuda",
	"cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vector_float_R_int_int_int" => "+_vec",
	"cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vector_float_R_int_int_int" => "+_vec",

	// ### stitching ###
	"cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR" => "+_images",
	"cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR" => "+_mask",
	"cv_Stitcher_stitch_const__InputArrayR_const_vector_vector_Rect__R_const__OutputArrayR" => "+_rois", // 3.2 3.4

	// ### surface_matching ###
	"cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vector_Pose3DPtr_R" => "+_vec",

	// ### text ###
	"cv_text_BaseOCR_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_mask",
	"cv_text_ERStat_getPropPixels" => "-", // fixme: reference to a vector, we don't handle it too well yet
	"cv_text_ERStat_setPropPixels_vector_int_X" => "-", // fixme: reference to a vector, we don't handle it too well yet
	"cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int" => "+_from_file",
	"cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple_mask",
	"cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple",
	"cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int" => "+_mask",
	"cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int" => "+_from_file",
	"cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple_mask",
	"cv_text_OCRHMMDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple",
	"cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int" => "+_mask",
	"cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_mask",
	"cv_text_OCRTesseract_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple_mask",
	"cv_text_OCRTesseract_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int" => "+_multiple",
	"cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int" => "+_mask",
	"cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vector_Size_" => "+_with_sizes",
	"cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float" => "+_from_file",
	"cv_text_createERFilterNM2_const_StringR_float" => "+_from_file",
	"cv_text_detectRegions_const__InputArrayR_const_Ptr_ERFilter_R_const_Ptr_ERFilter_R_vector_Rect_R_int_const_StringR_float" => "+_from_file",

	// ### videoio ###
	"cv_VideoCapture_VideoCapture_const_StringR" => "from_file_default", // 3.2
	"cv_VideoCapture_VideoCapture_const_StringR_int" => "from_file",
	"cv_VideoCapture_VideoCapture_const_StringR_int_const_vector_int_R" => "from_file_with_params",
	"cv_VideoCapture_VideoCapture_int" => "+_default", // 3.4
	"cv_VideoCapture_VideoCapture_int_int_const_vector_int_R" => "+_with_params",
	"cv_VideoCapture_open_const_StringR" => "+_file_default", // 3.2
	"cv_VideoCapture_open_const_StringR_int" => "+_file",
	"cv_VideoCapture_open_const_StringR_int" => "+_file",
	"cv_VideoCapture_open_int" => "+_device_default", // 3.2
	"cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool" => "+_with_backend",
	"cv_VideoWriter_open_const_StringR_int_int_double_Size_bool" => "+_with_backend",

	// ### videostab ###
	"cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX" => "+_mat",

	// those function are marked as CV_EXPORTS, but they are missing from the shared libraries
	// ### core ###
	"cv_MatConstIterator_MatConstIterator_const_MatX_const_intX" => "-",
	"cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX" => "-",
	"cv__OutputArray__OutputArray_const_vector_GpuMat_R" => "-",
	"cv_cuda_convertFp16_const__InputArrayR_const__OutputArrayR_StreamR" => "-",
	"cv_getImpl_vector_int_R_vector_String_R" => "-",
	// ### dnn ###
	"cv_dnn_BackendNode_BackendNode_int" => "-",
	// ### stitching ###
	"cv_createStitcherScans_bool" => "-",
	"cv_createStitcher_bool" => "-",
	// ### surface_matching ###
	"cv_ppf_match_3d_PPF3DDetector_read_const_FileNodeR" => "-",
	"cv_ppf_match_3d_PPF3DDetector_write_const_FileStorageR" => "-",
	// ### tracking ###
	"cv_CvFeatureParams_CvFeatureParams" => "-",
	"cv_CvFeatureParams_create_FeatureType" => "-",
	"cv_CvFeatureParams_create_int" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_FeatureHaar_Size" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_eval_const_const_MatR_Rect_floatX" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_getAreas_const" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_getInitMean_const" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_getInitSigma_const" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_getNumAreas" => "-",
	"cv_CvHaarEvaluator_FeatureHaar_getWeights_const" => "-",
	"cv_CvHaarEvaluator_getFeatures_const" => "-",
	"cv_CvHaarEvaluator_setWinSize_Size" => "-",
	"cv_CvHaarEvaluator_setWinSize_const" => "-",
	"cv_CvHaarEvaluator_writeFeature_const_FileStorageR" => "-",
});

pub static FUNC_CFG_ATTR: Lazy<HashMap<&str, (&str, &str)>> = Lazy::new(|| hashmap! {
	// ### core ###
	"cv_SparseMatConstIterator_operatorSS" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),

	// ### imgproc ###
	"cv_getRotationMatrix2D__Point2f_double_double" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),

	// ### tracking ###
	"cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
	"cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool" => ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)"),
});

/// cpp_fullname
pub static ELEMENT_EXCLUDE: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::String",
	"cv::internal::format", // 3.2 duplicate definition
	"cv::face::FacemarkLBF::BBox", // not used, not exported in windows dll
});

/// cpp_fullname
pub static ELEMENT_IGNORE: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"CV_DEPRECATED",
	"CV_EXPORTS",
	"CV_IMPL", // 3.2
	"CV_MAKE_TYPE",
	"CvFileNode", // 3.2 3.4 C struct
	"CvSeq", // 3.2 C struct
	"FILE",
	"HG_AUTOSIZE", // 3.2
	"cv::ErrorCallback",
	"cv::MatAllocator", // doesn't handle cpp part too well
	"cv::NAryMatIterator", // uses pointers of pointers
	"cv::Node", // template class
	"std::exception_ptr",
	"std::random_access_iterator_tag",
});

/// Manual export config in form of "cpp_fullname" => ExportConfig, will totally override what's
/// detected from the headers. Use it to add export config where none exists.
pub static ELEMENT_EXPORT_MANUAL: Lazy<HashMap<&str, ExportConfig>> = Lazy::new(|| hashmap! {
	"VADisplay" => ExportConfig::default(),
	"VASurfaceID" => ExportConfig::default(),
	"cv::Mat_" => ExportConfig::default(),
	"cv::CvFeatureParams" => ExportConfig::default(),
	"cv::CvHaarEvaluator" => ExportConfig::default(),
	"cv::CvHaarEvaluator::FeatureHaar" => ExportConfig::default(), // no default constructor
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
	"cv::CylindricalWarperGpu" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::SphericalWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::FisheyeWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::StereographicWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::CompressedRectilinearWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::CompressedRectilinearPortraitWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PaniniWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PaniniPortraitWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::PlaneWarperGpu" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::MercatorWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::SphericalWarperGpu" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::TransverseMercatorWarper" => ExportConfig::default(), // 3.2 3.4 stitching warpers
	"cv::TermCriteria" => ExportConfig::simple(),
	"cv::optflow::GPCTrainingParams" => ExportConfig::simple(),
	"cv::optflow::GPCMatchingParams" => ExportConfig::simple(),
	"cv::cudacodec::FormatInfo" => ExportConfig::simple(),
	"cv::kinfu::Intr" => ExportConfig::simple(),
	"cv::videostab::MaskFrameSource" => ExportConfig::default(),

	// override boxed
	"cv::DetectionBasedTracker::ExtObject" => ExportConfig::default(),
	"cv::DetectionBasedTracker::IDetector" => ExportConfig::default(),
	"cv::FileNode" => ExportConfig::default(), // return references in methods, generally looks like not simple
	"cv::dnn::Net" => ExportConfig::default(), // incorrectly marked as simple
	"cv::linemod::QuantizedPyramid" => ExportConfig::default(), // missing export in 3.2
	"cv::ocl::Device" => ExportConfig::default(),
});

#[allow(clippy::type_complexity)]
pub static ELEMENT_EXPORT_TWEAK: Lazy<HashMap<&str, fn (&mut ExportConfig)>> = Lazy::new(|| hashmap! {
	"cv::dnn::ClassificationModel" => ExportConfig::make_boxed as _,
	"cv::dnn::DetectionModel" => ExportConfig::make_boxed as _,
	"cv::dnn::KeypointsModel" => ExportConfig::make_boxed as _, // marked as simple from OpenCV 4.5.2
	"cv::dnn::Model" => ExportConfig::make_boxed as _,
	"cv::dnn::SegmentationModel" => ExportConfig::make_boxed as _, // marked as simple from OpenCV 4.5.2
	"cv::dnn::TextDetectionModel" => ExportConfig::make_boxed as _, // inappropriately marked as simple
	"cv::dnn::TextDetectionModel_DB" => ExportConfig::make_boxed as _, // inappropriately marked as simple
	"cv::dnn::TextDetectionModel_EAST" => ExportConfig::make_boxed as _, // inappropriately marked as simple
	"cv::dnn::TextRecognitionModel" => ExportConfig::make_boxed as _, // inappropriately marked as simple
});

/// set of functions that should have unsafe in their declaration, element is Func.identifier()
pub static FUNC_UNSAFE: Lazy<HashSet<FuncId>> = Lazy::new(|| hashset! {
	// allocates uninitialized memory
	FuncId::new("cv::Mat::Mat", ["size", "type"]),
	FuncId::new("cv::Mat::Mat", ["sizes", "type"]),
	FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type"]),
	FuncId::new("cv::Mat::Mat", ["rows", "cols", "type"]),
	FuncId::new("cv::Mat::create", ["size", "type"]),
	FuncId::new("cv::Mat::create", ["sizes", "type"]),
	FuncId::new("cv::Mat::create", ["ndims", "sizes", "type"]),
	FuncId::new("cv::Mat::create", ["rows", "cols", "type"]),
	FuncId::new("cv::UMat::UMat", ["size", "type", "usageFlags"]),
	FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "usageFlags"]),
	FuncId::new("cv::UMat::UMat", ["rows", "cols", "type", "usageFlags"]),
	FuncId::new("cv::UMat::create", ["size", "type", "usageFlags"]),
	FuncId::new("cv::UMat::create", ["size", "type", "usageFlags"]),
	FuncId::new("cv::UMat::create", ["ndims", "sizes", "type", "usageFlags"]),
	FuncId::new("cv::UMat::create", ["sizes", "type", "usageFlags"]),
	FuncId::new("cv::UMat::create", ["rows", "cols", "type", "usageFlags"]),
	FuncId::new("cv::_OutputArray::createSameSize", ["arr", "mtype"]),
	// pointer to internal data
	FuncId::new("cv::dnn::Dict::ptr", ["key"]),
	// takes reference and stores it for the lifetime of an object (fixme: add lifetime management)
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["allocator"]),
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["size", "type", "allocator"]),
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["size", "type", "s", "allocator"]),
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["arr", "allocator"]),
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "allocator"]),
	FuncId::new("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "s", "allocator"]),
	FuncId::new("cv::cuda::GpuMat::allocator", ["val"]),
	FuncId::new("cv::cuda::GpuMat::setDefaultAllocator", ["allocator"]), // fixme, should take 'static
});

pub static IMPLEMENTED_FUNCTION_LIKE_MACROS: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"CV_MAKETYPE",
});

// fixme, generalize, make it use constant::ValueKind
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
	"loop" => "loop_",
	"yield" => "yield_",
});

/// dict of functions with manual implementations
/// key: FuncInfo.identifier
/// value: dict
///   keys: "rust_safe", "rust_extern", "cpp", missing key means skip particular implementation
///   values: template to use for manual implementation or "~" to use default implementation
pub static FUNC_MANUAL: Lazy<HashMap<&str, CompiledInterpolation>> = Lazy::new(|| hashmap! {
	"cv_Mat_at_int" => include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_int" => include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
	"cv_Mat_at_int_int" => include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_int_int" => include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
	"cv_Mat_at_Point" => include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_Point" => include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
	"cv_Mat_at_int_int_int" => include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_int_int_int" => include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_intX" => include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
	"cv_Mat_at_const_const_intX" => include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
});

pub static FUNC_SPECIALIZE: Lazy<HashMap<&str, Vec<HashMap<&str, &str>>>> = Lazy::new(|| hashmap! {
	"cv_dnn_Dict_set_const_StringR_const_TR" => vec![
		hashmap! { "const T" => "cv::String" },
		hashmap! { "const T" => "cv::dnn::DictValue" },
		hashmap! { "const T" => "double" },
		hashmap! { "const T" => "int64_t" },
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

/// cpp_fullname
pub static FORCE_CONSTANT_METHOD: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::Mat::size",
	"cv::Mat::step",
	"cv::UMat::size",
	"cv::UMat::step",
});

pub static FORCE_INFALLIBLE: Lazy<HashSet<FuncId>> = Lazy::new(|| hashset! {
	// just returns static/constant data
	FuncId::new("cv::noArray", []),
	FuncId::new("cv::getVersionMajor", []),
	FuncId::new("cv::getVersionMinor", []),
	FuncId::new("cv::getVersionRevision", []),
	// not doing anything that can cause an exception
	FuncId::new("cv::Mat::empty", []),
	FuncId::new("cv::Mat::total", []),
	FuncId::new("cv::Mat::isContinuous", []),
	FuncId::new("cv::Mat::isSubmatrix", []),
	FuncId::new("cv::Mat::elemSize1", []),
	FuncId::new("cv::Mat::type", []),
	FuncId::new("cv::Mat::depth", []),
	FuncId::new("cv::Mat::channels", []),
	FuncId::new("cv::UMat::empty", []),
	FuncId::new("cv::UMat::total", []),
	FuncId::new("cv::UMat::isContinuous", []),
	FuncId::new("cv::UMat::isSubmatrix", []),
	FuncId::new("cv::UMat::elemSize1", []),
	FuncId::new("cv::UMat::type", []),
	FuncId::new("cv::UMat::depth", []),
	FuncId::new("cv::UMat::channels", []),
	FuncId::new("cv::SparseMat::elemSize", []),
	FuncId::new("cv::SparseMat::elemSize1", []),
	FuncId::new("cv::SparseMat::type", []),
	FuncId::new("cv::SparseMat::depth", []),
	FuncId::new("cv::SparseMat::channels", []),
	// marked CV_NOEXCEPT since OpenCV 4.5.2, propagate those changes to earlier versions
	FuncId::new("cv::Mat::Mat", []),
	FuncId::new("cv::MatSize::MatSize", ["_p"]),
	FuncId::new("cv::MatSize::dims", []),
	FuncId::new("cv::MatSize::operator const int *", []),
	FuncId::new("cv::MatStep::MatStep", []),
	FuncId::new("cv::MatStep::operator[]", ["i"]),
	FuncId::new("cv::UMat::UMat", ["usageFlags"]),
	FuncId::new("cv::ocl::Context::Context", []),
	FuncId::new("cv::ocl::Device::Device", []),
	FuncId::new("cv::ocl::Image2D::Image2D", []),
	FuncId::new("cv::ocl::Kernel::Kernel", []),
	FuncId::new("cv::ocl::KernelArg::KernelArg", []),
	FuncId::new("cv::ocl::Platform::Platform", []),
	FuncId::new("cv::ocl::PlatformInfo::PlatformInfo", []),
	FuncId::new("cv::ocl::Program::Program", []),
	FuncId::new("cv::ocl::ProgramSource::ProgramSource", []),
	FuncId::new("cv::ocl::Queue::Queue", []),
});

/// cpp_fullname => ( rust_fullname, cpp_fullname )
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

/// cpp_fullname
pub static IMPLEMENTED_GENERICS: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::Affine3",
	"cv::Mat_",
	"cv::Matx",
	"cv::Point3_",
	"cv::Point_",
	"cv::Rect_",
	"cv::Scalar_",
	"cv::Size_",
});

/// cpp_fullname
pub static IMPLEMENTED_CONST_GENERICS: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::Vec",
});

#[derive(Clone, Copy, Debug)]
pub enum ArgOverride {
	Nullable,
	NullableSlice,
	Slice,
	LenForSlice(&'static str, usize),
	StringAsBytes,
}

pub static ARGUMENT_OVERRIDE: Lazy<HashMap<FuncId, HashMap<&str, ArgOverride>>> = Lazy::new(|| hashmap! {
	FuncId::new("cv::Mat::at", ["idx"]) => hashmap! {
		"idx" => ArgOverride::Slice
	},
	FuncId::new("cv::Mat::ptr", ["idx"])  => hashmap! {
		"idx" => ArgOverride::Slice
	},
	FuncId::new("cv::Mat::Mat", ["sizes", "type", "data", "steps"]) => hashmap! {
		"steps" => ArgOverride::NullableSlice,
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type", "s"]) => hashmap! {
		"steps" => ArgOverride::NullableSlice,
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type", "data", "steps"]) => hashmap! {
		"steps" => ArgOverride::NullableSlice,
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::Mat::zeros", ["ndims", "sz", "type"]) => hashmap! {
		"sz" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sz", 1),
	},
	FuncId::new("cv::Mat::ones", ["ndims", "sz", "type"]) => hashmap! {
		"sz" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sz", 1),
	},
	FuncId::new("cv::Mat::create", ["ndims", "sizes", "type"]) => hashmap! {
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::Mat::reshape", ["cn", "newndims", "newsz"]) => hashmap! {
		"newsz" => ArgOverride::Slice,
		"newndims" => ArgOverride::LenForSlice("newsz", 1),
	},
	FuncId::new("cv::SparseMat::Hdr::Hdr", ["_dims", "_sizes", "_type"]) => hashmap! {
		"_sizes" => ArgOverride::Slice,
		"_dims" => ArgOverride::LenForSlice("_sizes", 1),
	},
	FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "usageFlags"]) => hashmap! {
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "s", "usageFlags"]) => hashmap! {
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::UMat::create", ["ndims", "sizes", "type", "usageFlags"]) => hashmap! {
		"sizes" => ArgOverride::Slice,
		"ndims" => ArgOverride::LenForSlice("sizes", 1),
	},
	FuncId::new("cv::_OutputArray::create", ["dims", "size", "type", "i", "allowTransposed", "fixedDepthMask"]) => hashmap! {
		"size" => ArgOverride::Slice,
		"dims" => ArgOverride::LenForSlice("size", 1),
	},
	FuncId::new("cv::mixChannels", ["src", "dst", "fromTo", "npairs"]) => hashmap! {
		"fromTo" => ArgOverride::Slice,
		"npairs" => ArgOverride::LenForSlice("from_to", 2),
	},
	FuncId::new("cv::createTrackbar", ["trackbarname", "winname", "value", "count", "onChange", "userdata"]) => hashmap! {
		"value" => ArgOverride::Nullable,
	},
	FuncId::new("cv::minMaxLoc", ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"]) => hashmap! {
		"minVal" => ArgOverride::Nullable,
		"maxVal" => ArgOverride::Nullable,
		"minLoc" => ArgOverride::Nullable,
		"maxLoc" => ArgOverride::Nullable,
	},
	FuncId::new("cv::minMaxLoc", ["a", "minVal", "maxVal", "minIdx", "maxIdx"]) => hashmap! {
		"minVal" => ArgOverride::Nullable,
		"maxVal" => ArgOverride::Nullable,
		"minIdx" => ArgOverride::Nullable,
		"maxIdx" => ArgOverride::Nullable,
	},
	FuncId::new("cv::minMaxIdx", ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"]) => hashmap! {
		"minVal" => ArgOverride::Nullable,
		"maxVal" => ArgOverride::Nullable,
		"minIdx" => ArgOverride::Nullable,
		"maxIdx" => ArgOverride::Nullable,
	},
	FuncId::new("cv::decodeQRCode", ["in", "points", "decoded_info", "straight_qrcode"]) => hashmap! {
		"decoded_info" => ArgOverride::StringAsBytes,
	},
	FuncId::new("cv::QRCodeDetector::decode", ["img", "points", "straight_qrcode"]) => hashmap! {
		"return" => ArgOverride::StringAsBytes,
	},
	FuncId::new("cv::QRCodeDetector::decodeCurved", ["img", "points", "straight_qrcode"]) => hashmap! {
		"return" => ArgOverride::StringAsBytes,
	},
	FuncId::new("cv::QRCodeDetector::detectAndDecode", ["img", "points", "straight_qrcode"]) => hashmap! {
		"return" => ArgOverride::StringAsBytes,
	},
	FuncId::new("cv::QRCodeDetector::detectAndDecodeCurved", ["img", "points", "straight_qrcode"]) => hashmap! {
		"return" => ArgOverride::StringAsBytes,
	},
	FuncId::new("cv::getOptimalNewCameraMatrix", ["cameraMatrix", "distCoeffs", "imageSize", "alpha", "newImgSize", "validPixROI", "centerPrincipalPoint"]) => hashmap! {
		"validPixROI" => ArgOverride::Nullable,
	},
});

pub static NO_SKIP_NAMESPACE_IN_LOCALNAME: Lazy<HashMap<&str, HashMap<&str, &str>>> = Lazy::new(|| hashmap! {
	"*" => hashmap! {
		"detail" => "Detail",
		"dynafu" => "Dynafu",
		"fisheye" => "Fisheye",
		"kinfu" => "Kinfu",
		"colored_kinfu" => "ColoredKinfu",
		"linemod" => "Linemod",
		"superres" => "Superres",
	},
	"cudabgsegm" => hashmap! {
		"cuda" => "CUDA",
	},
	"cudafeatures2d" => hashmap! {
		"cuda" => "CUDA",
	},
	"cudaimgproc" => hashmap! {
		"cuda" => "CUDA",
	},
	"cudaoptflow" => hashmap! {
		"cuda" => "CUDA",
	},
	"cudastereo" => hashmap! {
		"cuda" => "CUDA",
	},
	"mcc" => hashmap! {
		"mcc" => "MCC",
	},
	"gapi" => hashmap! {
		"own" => "Own",
	}
});

pub static PREVENT_VECTOR_TYPEDEF_GENERATION: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"cv::ppf_match_3d::Pose3DPtr",
	"cv::dnn::Net::LayerId",
});

#[derive(Default)]
pub struct ModuleTweak {
	pub includes: Vec<&'static str>,
	pub resolve_types: Vec<&'static str>,
	pub generate_types: Vec<&'static str>,
}

pub static GENERATOR_MODULE_TWEAKS: Lazy<HashMap<&str, ModuleTweak>> = Lazy::new(|| hashmap! {
	"*" => ModuleTweak {
		resolve_types: vec![
			// void is used as return type for property setters
			"void",
			// base types
			"bool",
			"int",
			"unsigned int",
			"double",
			// return of String
			"const char*",
			"void*",
			// handling vector of strings
			"std::vector<cv::String>",
			"std::vector<std::string>",
			// for return of vector<DataType> types
			"cv::_InputArray",
			"cv::_OutputArray",
			"cv::_InputOutputArray",
		],
		..Default::default()
	},
	"aruco" => ModuleTweak {
		generate_types: vec![
			"std::vector<cv::Vec3f>",
			"std::vector<cv::Vec3d>",
		],
		..Default::default()
	},
	"calib3d" => ModuleTweak {
		generate_types: vec![
			// for calibrate_camera
			"std::vector<cv::Point3i>",
			"std::vector<std::vector<cv::Point3i>>",
			"std::vector<cv::Point3f>",
			"std::vector<std::vector<cv::Point3f>>",
			"std::vector<cv::Point3d>",
			"std::vector<std::vector<cv::Point3d>>",
			"std::vector<cv::Vec3f>",
			"std::vector<std::vector<cv::Vec3f>>",
			// for solve_pnp tvec and rvec parameters
			"std::vector<std::vector<double>>",
		],
		..Default::default()
	},
	"core" => ModuleTweak {
		includes: vec![
			"core.hpp",
		],
		generate_types: vec![
			"cv::Ptr<float>", // for 3.2, no function uses that so it's not generated
		],
		..Default::default()
	},
	"dnn" => ModuleTweak {
		includes: vec![
			"dnn/dict.hpp",
		],
		resolve_types: vec![
			// for specializing cv::dnn::Dict::set
			"cv::dnn::DictValue",
			// for specializing cv::dnn::DictValue::get
			"cv::String",
			"int64_t",
		],
		..Default::default()
	},
	"face" => ModuleTweak {
		includes: vec![
			"face.hpp",
		],
		..Default::default()
	},
	"features2d" => ModuleTweak {
		includes: vec![
			"features2d.hpp",
		],
		// type used in other modules, thus needs casting (https://github.com/twistedfall/opencv-rust/issues/218)
		generate_types: vec![
			"cv::Ptr<cv::Feature2D>",
		],
		..Default::default()
	},
	"imgproc" => ModuleTweak {
		generate_types: vec![
			// for findContours()
			"std::vector<cv::Vec4i>",
			"std::vector<std::vector<cv::Point>>",
			// for HoughLines()
			"std::vector<cv::Vec2f>",
			"std::vector<cv::Vec2d>",
			"std::vector<cv::Vec3f>",
			"std::vector<cv::Vec3d>",
		],
		..Default::default()
	},
	"shape" => ModuleTweak {
		includes: vec![
			"shape.hpp",
		],
		..Default::default()
	},
	"stitching" => ModuleTweak {
		includes: vec![
			"stitching.hpp",
		],
		..Default::default()
	},
	"tracking" => ModuleTweak {
		includes: vec![
			"tracking.hpp",
		],
		..Default::default()
	},
	"videostab" => ModuleTweak {
		includes: vec![
			"videostab.hpp",
		],
		..Default::default()
	},
	"ximgproc" => ModuleTweak {
		includes: vec![
			"ximgproc.hpp",
		],
		..Default::default()
	},
});

/// list of module names that must use manual module comment extraction
pub static IGNORE_CLANG_MODULE_COMMENT: Lazy<HashSet<&str>> = Lazy::new(|| hashset! {
	"tracking",
});
