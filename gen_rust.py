import logging
import os.path
import re
import sys
import textwrap
from collections import OrderedDict
from io import StringIO
from itertools import chain
from pprint import pformat
from string import Template
from urllib.parse import quote


# fixme returning MatAllocator (trait) by reference is bad, check knearestneighbour
# fixme add support for arrays in dnn::DictValue
# fixme VectorOfMat::get allows to mutate?
# fixme get comments from HOUGH_PROBABILISTIC in imgproc
# fixme get multiline comments from LSD_REFINE_ADV in imgproc
# fixme remove ndims argument when slice is supplied and read it from slice
# fixme generate struct and trait for e.g. _InputArray
# fixme combining enum variants with |
# fixme add support for more operators

def template(text):
    """
    :type text: str
    :rtype: Template
    """
    if len(text) > 0 and text[0] == "\n":
        text = text[1:]
    return Template(textwrap.dedent(text))


def indent(text, level=1, indent_string="    "):
    """
    :type text: str
    :type level: int
    :type indent_string: str
    :rtype: str
    """
    padding = level * indent_string
    return padding.join(chain(("",), text.splitlines(True)))


def combine_dicts(src, add):
    out = src.copy()
    out.update(add)
    return out


def classes_equal(first, second):
    """
    :type first: str
    :type second: str
    :rtype: bool
    """
    first = first.strip()
    second = second.strip()
    if first == second:
        return True
    if not first.startswith("::") and second.endswith("::" + first):
        return True
    if not second.startswith("::") and first.endswith("::" + second):
        return True
    return False


def write_exc(filename, action):
    """ Calls action with file handle of filename only when file didn't exist before, thread-safe """
    try:
        with os.fdopen(os.open(filename, os.O_CREAT | os.O_WRONLY | os.O_EXCL, 0o666), "w", encoding="utf_8") as f:
            action(f)
    except OSError:
        pass


def make_safe_id(extern_id):
    return extern_id.replace(" ", "_").replace("*", "X").replace("::", "_")


#
#       EXCEPTIONS TO AUTO GENERATION
#


# dict of decls to inject before doing header parsing
# key: module name
# value: list of declarations as supplied by hdr_parser
decls_manual_pre = {
    "core": [
        ("class cv._InputArray", "", ["/Ghost"], []),
        ("typedef cv.InputArray", "const _InputArray&", [], []),
        ("class cv._OutputArray", "", ["/Ghost"], []),
        ("typedef cv.OutputArray", "_OutputArray&", [], []),
        ("class cv._InputOutputArray", "", ["/Ghost"], []),
        ("typedef cv.InputOutputArray", "_InputOutputArray&", [], []),
        ("typedef cv.InputArrayOfArrays", "InputArray", [], []),
        ("typedef cv.OutputArrayOfArrays", "OutputArray", [], []),
        ("typedef cv.InputOutputArrayOfArrays", "InputOutputArray", [], []),
        ("class cv.Range", "", ["/Ghost"], []),
        ("class cv.MatExpr", "", ["/Ghost"], []),
        ("class cv.Mat", "", ["/Ghost"], []),
        ("class cv.UMat", "", ["/Ghost"], []),
        ("class cv.Algorithm", "", ["/Ghost"], []),
        ("class cv.DMatch", "", ["/Ghost", "/Simple"], []),
        ("class cv.KeyPoint", "", ["/Ghost", "/Simple"], []),
        ("class cv.RotatedRect", "", ["/Ghost"], []),
        ("class cv.TermCriteria", "", ["/Ghost"], []),
        ("class cv.utils.logging.LogTag", "", ["/Ghost"], []),
        ("class cv.FileNode", "", ["/Ghost"], []),
        ("class cv.FileStorage", "", ["/Ghost"], []),
        ("typedef DummyVectorOfPoint3i", "std::vector<Point3i>", [], []),
        ("typedef DummyVectorOfPoint3f", "std::vector<Point3f>", [], []),
        ("typedef DummyVectorOfPoint3d", "std::vector<Point3d>", [], []),
        ("typedef DummyVectorOfVectorOfPoint3i", "std::vector<std::vector<Point3i>>", [], []),
        ("typedef DummyVectorOfVectorOfPoint3f", "std::vector<std::vector<Point3f>>", [], []),
        ("typedef DummyVectorOfVectorOfPoint3d", "std::vector<std::vector<Point3d>>", [], []),
    ],
    "dnn": [
        ("class cv.dnn.LayerParams", "", ["/Ghost"], []),
        ("class cv.dnn.Layer", "", ["/Ghost"], []),
    ],
    "features2d": [
        ("class cv.Feature2D", ": cv::Algorithm", ["/Ghost"], []),
        ("class cv.DescriptorMatcher", ": cv::Algorithm", ["/Ghost", "/A"], []),
        ("typedef cv.FeatureDetector", "Feature2D", [], []),
        ("typedef cv.DescriptorExtractor", "Feature2D", [], []),
    ],
    "imgproc": [
        ("enum cv.InterpolationFlags", "", ["/Ghost"], []),
    ]
}

# dict of decls to inject after doing header parsing
# key: module name
# value: list of declarations as supplied by hdr_parser
decls_manual_post = {
    "features2d": [
        ("cv.ORB.create", "Ptr<ORB>", ["/S"], [], None, "@overload"),
    ],
    "dnn": [
        ("cv.dnn.LayerParams.LayerParams", "", [], []),
    ],
}

# dict of functions to rename or skip, key is FuncInfo.identifier, value is new name ("+" will be replaces by old name) or "-" to skip
func_rename = {
    ### aruco ###
    "cv_aruco_calibrateCameraAruco__InputArray__InputArray__InputArray_PtrOfBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_with_stddev",
    "cv_aruco_calibrateCameraCharuco__InputArray__InputArray_PtrOfCharucoBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_with_stddev",
    "cv_aruco_generateCustomDictionary_int_int_PtrOfDictionary": "+_with_base",  # 3.2 only
    "cv_aruco_generateCustomDictionary_int_int_PtrOfDictionary_int": "+_with_base",
    "cv_aruco_getPredefinedDictionary_int": "+_i32",
    "cv_aruco_Dictionary_Dictionary_PtrOfDictionary": "copy",
    "cv_aruco_Dictionary_create_int_int_PtrOfDictionary": "+_with_base",  # 3.2 only
    "cv_aruco_Dictionary_create_int_int_PtrOfDictionary_int": "+_with_base",

    ### bioinspired ###
    "cv_bioinspired_Retina_setup_String_bool": "+_from_file",
    "cv_bioinspired_Retina_getParvoRAW__OutputArray": "+_to",
    "cv_bioinspired_Retina_getMagnoRAW__OutputArray": "+_to",
    "cv_bioinspired_Retina_create_Size_bool_int_bool_float_float": "+_ext",
    "cv_bioinspired_createRetina_OCL_Size_bool_int_bool_float_float": "+_ext",  # 3.2 only
    "cv_bioinspired_TransientAreasSegmentationModule_setup_String_bool": "+_from_file",

    ### calib3d ###
    "cv_findEssentialMat__InputArray__InputArray__InputArray_int_double_double__OutputArray": "+_matrix",
    "cv_findHomography__InputArray__InputArray_int_double__OutputArray_int_double": "+_ext",
    "cv_undistortPoints__InputArray__OutputArray__InputArray__InputArray__InputArray__InputArray_TermCriteria": "+_with_criteria",
    "cv_fisheye_projectPoints__InputArray__OutputArray__InputArray__InputArray__InputArray__InputArray_double__OutputArray": "fisheye_+",
    "cv_fisheye_stereoCalibrate__InputArray__InputArray__InputArray__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray_Size__OutputArray__OutputArray_int_TermCriteria": "fisheye_+",
    "cv_fisheye_stereoRectify__InputArray__InputArray__InputArray__InputArray_Size__InputArray__InputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_Size_double_double": "fisheye_+",
    "cv_fisheye_undistortImage__InputArray__OutputArray__InputArray__InputArray__InputArray_Size": "fisheye_+",
    "cv_fisheye_undistortPoints__InputArray__OutputArray__InputArray__InputArray__InputArray__InputArray": "fisheye_+",
    "cv_fisheye_initUndistortRectifyMap__InputArray__InputArray__InputArray__InputArray_Size_int__OutputArray__OutputArray": "fisheye_+",
    "cv_recoverPose__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray__InputOutputArray": "+_camera",
    "cv_recoverPose__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray_double__InputOutputArray__OutputArray": "+_camera_with_points",
    "cv_calibrateCamera__InputArray__InputArray_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_with_stddev",
    "cv_stereoCalibrate__InputArray__InputArray__InputArray__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray_Size__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_camera",
    "cv_stereoCalibrate__InputArray__InputArray__InputArray__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_camera_with_errors",
    "cv_calibrateCameraRO__InputArray__InputArray_Size_int__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria": "+_with_stddev",
    "cv_stereoRectify__InputArray__InputArray__InputArray__InputArray_Size__InputArray__InputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_double_Size_Rect_X_Rect_X": "+_camera",
    "cv_findFundamentalMat__InputArray__InputArray_int_double_double__OutputArray": "-",  # duplicate of cv_findFundamentalMat_Mat_Mat_Mat_int_double_double, but with different order of arguments
    "cv_initWideAngleProjMap__InputArray__InputArray_Size_int_int__OutputArray__OutputArray_int_double": "+_with_type_i32",
    "cv_findCirclesGrid__InputArray_Size__OutputArray_int_PtrOfFeature2D_CirclesGridFinderParameters": "+_params",

    ### core ###
    "cv_addImpl_int_const_char_X": "-",
    "cv_MatExpr_type_const": "typ",
    "cv_MatExpr_MatExpr_Mat": "from_mat",
    "cv_MatExpr_mul_const_MatExpr_double": "+_matexpr",
    "cv_MatOp_add_const_MatExpr_Scalar_MatExpr": "+_scalar",
    "cv_MatOp_subtract_const_Scalar_MatExpr_MatExpr": "+_scalar",
    "cv_MatOp_multiply_const_MatExpr_double_MatExpr": "+_f64",
    "cv_MatOp_divide_const_double_MatExpr_MatExpr": "+_f64",
    "cv_MatOp_type_const_MatExpr": "typ",
    "cv_Mat_Mat_int_int_int": "+_rows_cols",
    "cv_Mat_Mat_Size_int": "+_size",
    "cv_Mat_Mat_int_int_int_Scalar": "+_rows_cols_with_default",
    "cv_Mat_Mat_Size_int_Scalar": "+_size_with_default",
    "cv_Mat_Mat_int_const_int_X_int": "-",  # duplicate of cv_Mat_Mat_VectorOfint_int, but with pointers
    "cv_Mat_Mat_VectorOfint_int": "+_nd",
    "cv_Mat_Mat_int_const_int_X_int_Scalar": "-",  # duplicate of cv_Mat_Mat_VectorOfint_int_Scalar, but with pointers
    "cv_Mat_Mat_VectorOfint_int_Scalar": "+_nd_with_default",
    "cv_Mat_Mat_int_int_int_void_X_size_t": "new_rows_cols_with_data",
    "cv_Mat_Mat_Size_int_void_X_size_t": "new_size_with_data",
    "cv_Mat_Mat_int_const_int_X_int_void_X_const_size_t_X": "-",  # duplicate of cv_Mat_Mat_VectorOfint_int_void_X_const_size_t_X, but with pointers
    "cv_Mat_Mat_VectorOfint_int_void_X_const_size_t_X": "+_nd_with_data",
    "cv_Mat_Mat_Mat_Range_Range": "rowscols",
    "cv_Mat_Mat_Mat_Rect": "roi",
    "cv_Mat_Mat_Mat_const_Range": "-",  # duplicate of cv_Mat_Mat_Mat_VectorOfRange, but with pointers
    "cv_Mat_Mat_Mat_VectorOfRange": "ranges",
    "cv_Mat_colRange_const_int_int": "col_bounds",
    "cv_Mat_rowRange_const_int_int": "row_bounds",
    "cv_Mat_copyTo_const__OutputArray__InputArray": "+_masked",
    "cv_Mat_create_int_int_int": "+_rows_cols",
    "cv_Mat_create_Size_int": "+_size",
    "cv_Mat_create_VectorOfint_int": "+_nd",
    "cv_Mat_create_int_const_int_X_int": "-",  # duplicate of cv_Mat_create_VectorOfint_int, but with pointers
    "cv_Mat_diag_Mat": "+_new_mat",
    "cv_Mat_ptr_int": "+_mut",
    "cv_Mat_ptr_int_int": "+_2d_mut",
    "cv_Mat_ptr_const_int_int": "+_2d",
    "cv_Mat_ptr_int_int_int": "+_3d_mut",
    "cv_Mat_ptr_const_int_int_int": "+_3d",
    "cv_Mat_ptr_const_int_X": "+_nd_mut",
    "cv_Mat_ptr_const_const_int_X": "+_nd",
    "cv_Mat_at_int": "at_mut",
    "cv_Mat_at_int_int": "at_2d_mut",
    "cv_Mat_at_const_int_int": "at_2d",
    "cv_Mat_at_Point": "at_pt_mut",
    "cv_Mat_at_const_Point": "at_pt",
    "cv_Mat_at_int_int_int": "at_3d_mut",
    "cv_Mat_at_const_int_int_int": "at_3d",
    "cv_Mat_at_const_int_X": "at_nd_mut",
    "cv_Mat_at_const_const_int_X": "at_nd",
    "cv_Mat_resize_size_t_Scalar": "resize_with_default",
    "cv_Mat_type_const": "typ",
    "cv_Mat_reshape_const_int_int_const_int_X": "-",  # duplicate of cv_Mat_reshape_const_int_VectorOfint, but with pointers
    "cv_Mat_reshape_const_int_VectorOfint": "reshape_nd",
    "cv_Mat_total_const_int_int": "total_slice",
    "cv_Mat_size_const": "mat_size",
    "cv_Mat_step_const": "mat_step",
    "cv_Mat_set_size_MatSize": "-",  # doesn't allow writing
    "cv_Mat_set_step_MatStep": "-",  # same as above
    "cv_Mat_data": "+_mut",
    "cv_Mat_copySize_Mat": "-",  # internal function
    "cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags": "get_umat",
    "cv_Mat_getUMat_const_int_UMatUsageFlags": "get_umat",  # 3.2 only
    "cv_Mat_zeros_Size_int": "+_size",
    "cv_Mat_zeros_int_const_int_X_int": "+_nd",
    "cv_Mat_ones_Size_int": "+_size",
    "cv_Mat_ones_int_const_int_X_int": "+_nd",
    "cv_Mat_eye_Size_int": "+_size",
    "cv_Mat_push_back__const_void_X": "-",  # internal method
    "cv_min_Mat_Mat": "+_mat",
    "cv_min_Mat_Mat_Mat": "+_mat_to",
    "cv_min_Mat_double": "+_mat_f64",
    "cv_min_double_Mat": "+_f64_mat",
    "cv_min_UMat_UMat_UMat": "+_umat_to",
    "cv_max_Mat_Mat": "+_mat",
    "cv_max_Mat_Mat_Mat": "+_mat_to",
    "cv_max_Mat_double": "+_mat_f64",
    "cv_max_double_Mat": "+_f64_mat",
    "cv_max_UMat_UMat_UMat": "+_umat_to",
    "cv_minMaxLoc_SparseMat_double_X_double_X_int_X_int_X": "+_sparse",
    "cv_swap_UMat_UMat": "+_umat",
    "cv_UMat_UMat_int_int_int_UMatUsageFlags": "+_rows_cols",
    "cv_UMat_UMat_Size_int_UMatUsageFlags": "+_size",
    "cv_UMat_UMat_int_int_int_Scalar_UMatUsageFlags": "+_rows_cols_with_default",
    "cv_UMat_UMat_Size_int_Scalar_UMatUsageFlags": "+_size_with_default",
    "cv_UMat_UMat_int_const_int_X_int_UMatUsageFlags": "+_nd",
    "cv_UMat_UMat_int_const_int_X_int_Scalar_UMatUsageFlags": "+_nd_with_default",  # fixme
    "cv_UMat_UMat_UMat_Range_Range": "rowscols",
    "cv_UMat_UMat_UMat_Rect": "roi",
    "cv_UMat_UMat_UMat_const_Range": "-",  # duplicate of cv_UMat_UMat_UMat_VectorOfRange, but with pointers
    "cv_UMat_UMat_UMat_VectorOfRange": "ranges",
    "cv_UMat_colRange_const_int_int": "col_bounds",
    "cv_UMat_rowRange_const_int_int": "row_bounds",
    "cv_UMat_size_const": "mat_size",
    "cv_UMat_step_const": "mat_step",
    "cv_UMat_set_size_MatSize": "-",  # doesn't allow writing
    "cv_UMat_set_step_MatStep": "-",  # same as above
    "cv_UMat_create_int_int_int_UMatUsageFlags": "+_rows_cols",
    "cv_UMat_create_Size_int_UMatUsageFlags": "+_size",
    "cv_UMat_create_VectorOfint_int_UMatUsageFlags": "+_nd",
    "cv_UMat_create_int_const_int_X_int_UMatUsageFlags": "-",  # duplicate of cv_UMat_create_VectorOfint_int_UMatUsageFlags, but with pointers
    "cv_UMat_type_const": "typ",
    "cv_UMat_copyTo_const__OutputArray__InputArray": "+_masked",
    "cv_UMat_copySize_UMat": "-",  # internal function
    "cv_merge_const_Mat_size_t__OutputArray": "-",  # duplicate of cv_merge_VectorOfMat_Mat, but with pointers
    "cv_PCA_PCA__InputArray__InputArray_int_double": "+_mat_variance",
    "cv_PCA_PCA__InputArray__InputArray_int_int": "+_mat_max",
    "cv_PCA_backProject_const__InputArray__OutputArray": "+_to",
    "cv_PCA_project_const__InputArray__OutputArray": "+_to",
    "cv_PCACompute__InputArray__InputOutputArray__OutputArray_double": "+_variance",
    "cv_PCACompute__InputArray__InputOutputArray__OutputArray__OutputArray_double": "+_values_variance",
    "cv_PCACompute__InputArray__InputOutputArray__OutputArray__OutputArray_int": "+_values",
    "cv_Range_Range_int_int": "new",
    "cv_RotatedRect_RotatedRect_Point2f_Point2f_Point2f": "for_points",
    "cv_calcCovarMatrix_const_Mat_int_Mat_Mat_int_int": "-",  # duplicate of cv_calcCovarMatrix_Mat_Mat_Mat_int_int, but with pointers
    "cv_clipLine_Size_Point_Point": "clip_line_size",
    "cv_clipLine_Size2l_Point2l_Point2l": "clip_line_size_i64",
    "cv_clipLine_Rect_Point_Point": "clip_line",
    "cv_cv_abs_short": "-",
    "cv_cv_abs_uchar": "-",
    "cv_abs_MatExpr": "+_matexpr",
    "cv_divide__InputArray__InputArray__OutputArray_double_int": "+2",
    "cv_ellipse__InputOutputArray_RotatedRect_Scalar_int_int": "ellipse_new_rotated_rect",
    "cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_VectorOfPoint2d": "ellipse_2_poly_f64",
    "cv_ellipse2Poly_Point_Size_int_int_int_int_VectorOfPoint": "ellipse_2_poly",
    "cv_norm__InputArray__InputArray_int__InputArray": "+2",
    "cv_norm_SparseMat_int": "+_sparse",
    "cv_normalize_SparseMat_SparseMat_double_int": "+_sparse",
    "cv_rectangle__InputOutputArray_Point_Point_Scalar_int_int_int": "+_points",
    "cv_repeat__InputArray_int_int__OutputArray": "+_to",
    "cv_split_Mat_Mat": "-",  # duplicate of cv_split_Mat_VectorOfMat, but with pointers
    "cv_getNumberOfCPUs": "get_number_of_cpus",
    "cv_setImpl_int": "-",
    "cv_setUseCollection_bool": "-",
    "cv_useCollection": "-",
    "cv_directx_getTypeFromD3DFORMAT_int": "get_type_from_d3d_format",
    "cv_divUp_size_t_unsigned_int": "duv_up_u",
    "cv_hconcat__InputArray__InputArray__OutputArray": "+2",
    "cv_hconcat_const_Mat_size_t__OutputArray": "-",  # duplicate of cv_hconcat_VectorOfMat_Mat, but with pointers
    "cv_vconcat__InputArray__InputArray__OutputArray": "+2",
    "cv_vconcat_const_Mat_size_t__OutputArray": "-",  # duplicate of cv_vconcat_VectorOfMat_Mat, but with pointers
    "cv_Cholesky_float_X_size_t_int_float_X_size_t_int": "cholesky_f32",
    "cv_LU_float_X_size_t_int_float_X_size_t_int": "lu_f32",
    "cv_LDA_LDA__InputArray__InputArray_int": "+_with_data",
    "cv_mixChannels__InputArray__InputOutputArray_const_int_X_size_t": "-",  # duplicate of cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint, but with pointers
    "cv_mixChannels_const_Mat_size_t_Mat_size_t_const_int_X_size_t": "-",  # duplicate of cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint, but with pointers
    "cv_detail_typeToString__int": "-",  # detail function
    "cv_detail_depthToString__int": "-",  # detail function
    "cv_MatConstIterator_MatConstIterator_const_Mat": "over",
    "cv_MatConstIterator_MatConstIterator_const_Mat_int_int": "with_rows_cols",
    "cv_MatConstIterator_MatConstIterator_const_Mat_Point": "with_start",
    # "cv_MatConstIterator_MatConstIterator_const_Mat_const_int_X": "with_idx",
    "cv_MatConstIterator_MatConstIterator_const_Mat_const_int_X": "-",
    "cv_MatConstIterator_pos_const_int_X": "+_to",
    "cv_MatConstIterator_seek_const_int_X_bool": "+_idx",
    "cv_AsyncArray_get_const__OutputArray_int64": "+_with_timeout",
    "cv_AsyncArray_get_const__OutputArray_double": "+_with_timeout_f64",
    "cv_AsyncArray_wait_for_const_double": "+_f64",
    "cv_AsyncArray__getImpl_const": "-",
    "cv_ocl_internal_isCLBuffer_UMat": "-",
    "cv_ocl_internal_isOpenCLForced": "-",
    "cv_ocl_internal_isPerformanceCheckBypassed": "-",
    "cv_ocl_ProgramSource_ProgramSource_const_char_X": "-",
    "cv_ocl_Context_Context_int": "+_with_type",
    "cv_ocl_Context_create_int": "+_with_type",
    "cv_ocl_Kernel_set_int_UMat": "+_umat",
    "cv_ocl_Kernel_set_int_KernelArg": "+_kernel_arg",
    "cv_ocl_Program_getPrefix_String": "+_build_flags",
    "cv_ocl_ProgramSource_ProgramSource_String": "from_str",
    "cv__InputArray__InputArray_Mat": "from_mat",
    "cv__InputArray__InputArray_MatExpr": "from_matexpr",
    "cv__InputArray__InputArray_VectorOfMat": "from_mat_vec",
    "cv__InputArray__InputArray_VectorOfbool": "from_bool_vec",
    "cv__InputArray__InputArray_double": "from_f64",
    "cv__InputArray__InputArray_UMat": "from_umat",
    "cv__InputArray__InputArray_VectorOfUMat": "from_umat_vec",
    "cv__InputArray_copyTo_const__OutputArray__InputArray": "+_with_mask",
    "cv__OutputArray__OutputArray_Mat": "from_mat",
    "cv__OutputArray__OutputArray_VectorOfMat": "from_mat_vec",
    "cv__OutputArray__OutputArray_UMat": "from_umat",
    "cv__OutputArray__OutputArray_VectorOfUMat": "from_umat_vec",
    "cv__InputOutputArray__InputOutputArray_Mat": "from_mat",
    "cv__InputOutputArray__InputOutputArray_VectorOfMat": "from_mat_vec",
    "cv__InputOutputArray__InputOutputArray_UMat": "from_umat",
    "cv__InputOutputArray__InputOutputArray_VectorOfUMat": "from_umat_vec",
    "cv_read_FileNode_schar_schar": "-",
    "cv_operator_std_string_const": "-",
    "cv_double_const": "-",
    "cv_float_const": "-",
    "cv_int_const": "-",
    "cv_read_FileNode_DMatch_DMatch": "+_dmatch",
    "cv_read_FileNode_KeyPoint_KeyPoint": "+_keypoint",
    "cv_read_FileNode_Mat_Mat": "+_mat",
    "cv_read_FileNode_Range_Range": "+_range",
    "cv_read_FileNode_SparseMat_SparseMat": "+_sparsemat",
    "cv_read_FileNode_VectorOfDMatch": "+_dmatch_vec_legacy",
    "cv_read_FileNode_VectorOfDMatch_VectorOfDMatch": "+_dmatch_vec",
    "cv_read_FileNode_VectorOfKeyPoint": "+_keypoint_vec_legacy",
    "cv_read_FileNode_VectorOfKeyPoint_VectorOfKeyPoint": "+_keypoint_vec",
    "cv_read_FileNode_bool_bool": "+_bool",
    "cv_read_FileNode_double_double": "+_f64",
    "cv_read_FileNode_float_float": "+_f32",
    "cv_read_FileNode_int_int": "+_i32",
    "cv_read_FileNode_short_short": "+_i16",
    "cv_read_FileNode_std_string_std_string": "+_str",
    "cv_read_FileNode_uchar_uchar": "+_u8",
    "cv_read_FileNode_ushort_ushort": "+_u16",
    "cv_writeScalar_FileStorage_String": "+_str",
    "cv_writeScalar_FileStorage_double": "+_f64",
    "cv_writeScalar_FileStorage_float": "+_f32",
    "cv_writeScalar_FileStorage_int": "+_i32",
    "cv_write_FileStorage_DMatch": "+_dmatch",
    "cv_write_FileStorage_KeyPoint": "+_keypoint",
    "cv_write_FileStorage_Range": "+_range",
    "cv_write_FileStorage_String_DMatch": "+_dmatch",
    "cv_write_FileStorage_String_KeyPoint": "+_keypoint",
    "cv_write_FileStorage_String_Mat": "+_mat",
    "cv_write_FileStorage_String_Range": "+_range",
    "cv_write_FileStorage_String_SparseMat": "+_sparsemat",
    "cv_write_FileStorage_String_String": "+_str",
    "cv_write_FileStorage_String_VectorOfDMatch": "+_dmatch_vec",
    "cv_write_FileStorage_String_VectorOfKeyPoint": "+_keypoint_vec",
    "cv_write_FileStorage_String_double": "+_f64",
    "cv_write_FileStorage_String_float": "+_f32",
    "cv_write_FileStorage_String_int": "+_i32",
    "cv_write_FileStorage_VectorOfDMatch": "+_dmatch_vec",
    "cv_write_FileStorage_VectorOfKeyPoint": "+_keypoint_vec",
    "cv_FileStorage_write_String_int": "+_i32",
    "cv_FileStorage_write_String_double": "+_f64",
    "cv_FileStorage_write_String_String": "+_str",
    "cv_FileStorage_write_String_Mat": "+_mat",
    "cv_FileStorage_write_String_VectorOfString": "+_str_vec",

    ### dnn ###
    "cv_dnn_<unnamed>_is_neg_int": "-",
    "cv_dnn_NMSBoxes_VectorOfRotatedRect_VectorOffloat_float_float_VectorOfint_float_int": "nms_boxes_rotated",
    "cv_dnn_NMSBoxes_VectorOfRect2d_VectorOffloat_float_float_VectorOfint_float_int": "nms_boxes_f64",
    "cv_dnn_Dict_ptr_String": "ptr_mut",
    "cv_dnn_Net_forward__OutputArray_String": "+_layer",
    "cv_dnn_Net_forward__OutputArray_VectorOfString": "+_first_outputs",
    "cv_dnn_Net_forward_VectorOfVectorOfMat_VectorOfString": "forward_all",
    "cv_dnn_slice_Mat_Range": "slice_1d",
    "cv_dnn_slice_Mat_Range_Range": "slice_2d",
    "cv_dnn_slice_Mat_Range_Range_Range": "slice_3d",
    "cv_dnn_slice_Mat_Range_Range_Range_Range": "slice_4d",
    "cv_dnn_shape_UMat": "+_umat",
    "cv_dnn_shape_const_int_X_int": "shape_nd",
    "cv_dnn_shape_int_int_int_int": "shape_3d",
    "cv_dnn_blobFromImage__InputArray__OutputArray_double_Size_Scalar_bool_bool_int": "+_to",
    "cv_dnn_blobFromImages__InputArray__OutputArray_double_Size_Scalar_bool_bool_int": "+_to",
    "cv_dnn_clamp_Range_int": "clamp_range",
    "cv_dnn_Net_connect_String_String": "connect_first_second",
    "cv_dnn_Layer_finalize_VectorOfMat": "+_mat",
    "cv_dnn_readNetFromCaffe_VectorOfuchar_VectorOfuchar": "+_buffer",
    "cv_dnn_readNetFromCaffe_const_char_X_size_t_const_char_X_size_t": "+_str",
    "cv_dnn_readNetFromTensorflow_VectorOfuchar_VectorOfuchar": "+_buffer",
    "cv_dnn_readNetFromTensorflow_const_char_X_size_t_const_char_X_size_t": "+_str",
    "cv_dnn_readNetFromDarknet_VectorOfuchar_VectorOfuchar": "+_buffer",
    "cv_dnn_readNetFromDarknet_const_char_X_size_t_const_char_X_size_t": "+_str",
    "cv_dnn_readNetFromONNX_VectorOfuchar": "+_buffer",
    "cv_dnn_readNetFromONNX_const_char_X_size_t": "+_str",
    "cv_dnn_Net_getMemoryConsumption_const_int_VectorOfVectorOfint_size_t_size_t": "get_memory_consumption_for_layer",
    "cv_dnn_Net_getMemoryConsumption_const_VectorOfVectorOfint_VectorOfint_VectorOfsize_t_VectorOfsize_t": "get_memory_consumption_for_layers",
    "cv_dnn_DictValue_DictValue_bool": "from_bool",
    "cv_dnn_DictValue_DictValue_int64": "from_i64",
    "cv_dnn_DictValue_DictValue_int": "from_i32",
    "cv_dnn_DictValue_DictValue_unsigned": "from_u32",
    "cv_dnn_DictValue_DictValue_double": "from_f64",
    "cv_dnn_DictValue_DictValue_const_char_X": "from_str",
    "cv_dnn_DictValue_DictValue_String": "-",  # effectively duplicate of cv_dnn_DictValue_DictValue_const_char_X

    ### features2d ###
    "cv_AGAST__InputArray_VectorOfKeyPoint_int_bool": "AGAST",
    "cv_AGAST__InputArray_VectorOfKeyPoint_int_bool_int": "AGAST_with_type",  # 3.x only
    "cv_AGAST__InputArray_VectorOfKeyPoint_int_bool_AgastFeatureDetector_DetectorType": "AGAST_with_type",
    "cv_BOWKMeansTrainer_cluster_const_Mat": "new",
    "cv_BOWKMeansTrainer_BOWKMeansTrainer_int_TermCriteria_int_int": "new_with_criteria",
    "cv_BOWImgDescriptorExtractor_compute__InputArray_VectorOfKeyPoint__OutputArray_VectorOfVectorOfint_Mat": "+_desc",
    "cv_DMatch_DMatch_int_int_int_float": "new_index",
    "cv_DescriptorMatcher_knnMatch_const__InputArray__InputArray_VectorOfVectorOfDMatch_int__InputArray_bool": "knn_train_matches",
    "cv_DescriptorMatcher_knnMatch__InputArray_VectorOfVectorOfDMatch_int__InputArray_bool": "knn_matches",
    "cv_DescriptorMatcher_match__InputArray_VectorOfDMatch__InputArray": "matches",
    "cv_DescriptorMatcher_match_const__InputArray__InputArray_VectorOfDMatch__InputArray": "train_matches",
    "cv_DescriptorMatcher_radiusMatch_const__InputArray__InputArray_VectorOfVectorOfDMatch_float__InputArray_bool": "train_radius_matches",
    "cv_DescriptorMatcher_radiusMatch__InputArray_VectorOfVectorOfDMatch_float__InputArray_bool": "radius_matches",
    "cv_FAST__InputArray_VectorOfKeyPoint_int_bool": "FAST",
    "cv_FAST__InputArray_VectorOfKeyPoint_int_bool_int": "FAST_with_type",  # 3.x only
    "cv_FAST__InputArray_VectorOfKeyPoint_int_bool_FastFeatureDetector_DetectorType": "FAST_with_type",
    "cv_Feature2D_detect__InputArray_VectorOfVectorOfKeyPoint__InputArray": "+_multiple",
    "cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int": "new_point",
    "cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int": "new_coords",
    "cv_KeyPoint_convert_VectorOfKeyPoint_VectorOfPoint2f_VectorOfint": "convert_from",
    "cv_KeyPoint_convert_VectorOfPoint2f_VectorOfKeyPoint_float_float_int_int": "convert_to",
    "cv_drawMatches__InputArray_VectorOfKeyPoint__InputArray_VectorOfKeyPoint_VectorOfVectorOfDMatch__InputOutputArray_Scalar_Scalar_VectorOfVectorOfchar_int": "+_vec",  # 3.x only
    "cv_drawMatches__InputArray_VectorOfKeyPoint__InputArray_VectorOfKeyPoint_VectorOfVectorOfDMatch__InputOutputArray_Scalar_Scalar_VectorOfVectorOfchar_DrawMatchesFlags": "+_vec",
    "cv_BRISK_create_VectorOffloat_VectorOfint_float_float_VectorOfint": "create_with_pattern",
    "cv_BRISK_create_int_int_VectorOffloat_VectorOfint_float_float_VectorOfint": "create_with_pattern_threshold_octaves",
    "cv_BOWTrainer_cluster_const_Mat": "cluster_with_descriptors",
    "cv_GFTTDetector_create_int_double_double_int_int_bool_double": "+_with_gradient",
    "cv_Feature2D_compute__InputArray_VectorOfVectorOfKeyPoint__OutputArray": "+_multiple",
    "cv_DescriptorMatcher_create_int": "+_with_matcher_type",  # 3.x only
    "cv_DescriptorMatcher_create_DescriptorMatcher_MatcherType": "+_with_matcher_type",
    "cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_PtrOfFeature2D_PtrOfDescriptorMatcher": "new_with_dextractor",
    "cv_ORB_create": "default",

    ### hdf ###
    "cv_hdf_HDF5_atread_String_X_String": "-",  # fixme allow receiving data into &mut String arg

    ### highgui ###
    "cv_addText_Mat_String_Point_QtFont": "+_with_font",
    "cv_selectROIs_String__InputArray_VectorOfRect_bool_bool": "select_rois",
    "cv_selectROI_String__InputArray_bool_bool": "+_for_window",

    ### imgcodecs ###
    "cv_imdecode__InputArray_int_Mat": "+_to",

    ### imgproc ###
    "cv_Canny__InputArray__InputArray__OutputArray_double_double_bool": "+_derivative",
    "cv_applyColorMap__InputArray__OutputArray__InputArray": "+_user",
    "cv_connectedComponentsWithStats__InputArray__OutputArray__OutputArray__OutputArray_int_int_int": "+_algo",
    "cv_connectedComponents__InputArray__OutputArray_int_int_int": "+_algo",
    "cv_Subdiv2D_insert_VectorOfPoint2f": "+_multiple",
    "cv_findContours__InputArray__OutputArray__OutputArray_int_int_Point": "+_with_hierarchy",  # 4.x only
    "cv_findContours__InputOutputArray__OutputArray__OutputArray_int_int_Point": "+_with_hierarchy",  # 3.x only
    "cv_distanceTransform__InputArray__OutputArray__OutputArray_int_int_int": "+_labels",
    "cv_fillConvexPoly__InputOutputArray_const_Point_X_int_Scalar_int_int": "-",  # duplicate of cv_fillConvexPoly__InputOutputArray__InputArray_Scalar_int_int, but with pointers
    "cv_fillConvexPoly_Mat_const_Point_X_int_Scalar_int_int": "-",  # duplicate of cv_fillConvexPoly__InputOutputArray__InputArray_Scalar_int_int, but with pointers
    "cv_floodFill__InputOutputArray__InputOutputArray_Point_Scalar_Rect_X_Scalar_Scalar_int": "+_mask",
    "cv_integral__InputArray__OutputArray__OutputArray__OutputArray_int_int": "+_titled_sq",
    "cv_integral__InputArray__OutputArray__OutputArray_int_int": "+_sq_depth",
    "cv_GeneralizedHough_detect__InputArray__InputArray__InputArray__OutputArray__OutputArray": "+_with_edges",
    "cv_goodFeaturesToTrack__InputArray__OutputArray_int_double_double__InputArray_int_int_bool_double": "+_with_gradient",
    "cv_getAffineTransform_const_Point2f_X_const_Point2f_X": "+_slice",
    "cv_getPerspectiveTransform_const_Point2f_X_const_Point2f_X": "+_slice",  # 3.x only
    "cv_getPerspectiveTransform_const_Point2f_X_const_Point2f_X_int": "+_slice",  # 4.x only

    ### line_descriptor ###
    "cv_line_descriptor_LSDDetector_detect_const_VectorOfMat_VectorOfVectorOfKeyLine_int_int_VectorOfMat": "+_multiple",

    ### ml ###
    "cv_ml_StatModel_train_PtrOfTrainData_int": "+_with_data",
    "cv_ml_SVM_trainAuto_PtrOfTrainData_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool": "+_with_data",
    "cv_ml_ParamGrid_ParamGrid_double_double_double": "for_range",

    ### objdetect ###
    "cv_CascadeClassifier_detectMultiScale__InputArray_VectorOfRect_VectorOfint_VectorOfdouble_double_int_int_Size_Size_bool": "+_levels",
    "cv_CascadeClassifier_detectMultiScale__InputArray_VectorOfRect_VectorOfint_double_int_int_Size_Size": "+_num",
    "cv_HOGDescriptor_HOGDescriptor_String": "new_from_file",
    "cv_HOGDescriptor_detect_const__InputArray_VectorOfPoint_VectorOfdouble_double_Size_Size_VectorOfPoint": "+_weights",  # 4.x only
    "cv_HOGDescriptor_detect_const_Mat_VectorOfPoint_VectorOfdouble_double_Size_Size_VectorOfPoint": "+_weights",  # 3.x only
    "cv_HOGDescriptor_detectMultiScale_const__InputArray_VectorOfRect_VectorOfdouble_double_Size_Size_double_double_bool": "+_weights",
    "cv_groupRectangles_VectorOfRect_VectorOfint_VectorOfdouble_int_double": "+_levels",
    "cv_groupRectangles_VectorOfRect_VectorOfint_int_double": "+_weights",
    "cv_groupRectangles_VectorOfRect_int_double_VectorOfint_VectorOfdouble": "+_levelweights",
    "cv_BaseCascadeClassifier_detectMultiScale__InputArray_VectorOfRect_VectorOfint_double_int_int_Size_Size": "+_num",
    "cv_BaseCascadeClassifier_detectMultiScale__InputArray_VectorOfRect_VectorOfint_VectorOfdouble_double_int_int_Size_Size_bool": "+_levels",
    "cv_HOGDescriptor_set_svmDetector_VectorOffloat": "+_vec",

    ### photo ###
    "cv_fastNlMeansDenoisingMulti__InputArray__OutputArray_int_int_float_int_int": "+_vec",
    "cv_fastNlMeansDenoising__InputArray__OutputArray_VectorOffloat_int_int_int": "+_vec",
    "cv_fastNlMeansDenoising_Mat_Mat_float_int_int": "+_window",
    "cv_AlignMTB_process__InputArray_VectorOfMat__InputArray__InputArray": "+_with_response",
    "cv_MergeDebevec_process__InputArray__OutputArray__InputArray__InputArray": "+_with_response",
    "cv_MergeMertens_process__InputArray__OutputArray__InputArray__InputArray": "+_with_response",
    "cv_MergeRobertson_process__InputArray__OutputArray__InputArray__InputArray": "process_with_response",

    ### stitching ###
    "cv_Stitcher_composePanorama__InputArray__OutputArray": "+_images",
    "cv_Stitcher_stitch__InputArray__InputArray__OutputArray": "+_mask",
    "cv_Stitcher_stitch__InputArray_VectorOfVectorOfRect__OutputArray": "+_rois",

    ### videoio ###
    "cv_VideoCapture_VideoCapture_int_int": "new_with_backend",
    "cv_VideoCapture_VideoCapture_String": "new_from_file",
    "cv_VideoCapture_VideoCapture_String_int": "new_from_file_with_backend",
    "cv_VideoCapture_open_int_int": "open_with_backend",
    "cv_VideoCapture_open_String": "open_file",
    "cv_VideoCapture_open_String_int": "open_file_with_backend",
    "cv_VideoWriter_VideoWriter_String_int_int_double_Size_bool": "new_with_backend",
    "cv_VideoWriter_open_String_int_int_double_Size_bool": "open_with_backend",

    ### videostab ###
    "cv_videostab_KeypointBasedMotionEstimator_estimate_Mat_Mat_bool_X": "+_mat",

    ### utility ###
    "cv_getImpl_VectorOfint_VectorOfString": "-",

    ### xfeatures2d ###
    "cv_xfeatures2d_AffineFeature2D_create_PtrOfFeature2D_PtrOfFeature2D": "+_with_extrator",
}

# list of classes to skip, elements are regular expressions for re.match() against ClassInfo.fullname
class_ignore_list = (
    ### core ###
    "Cv[A-Z]",  # C style types
    "Ipl.*",
    "cv::Mutex", "cv::softfloat", "cv::softdouble", "cv::float16_t",  # have corresponding Rust implementation
    "cv::Exception",
    "cv::RNG.*",  # maybe
    "cv::SVD",
    "cv::MatAllocator",
    "cv::TLSDataContainer",
    "cv::utils::logging::LogTagAuto",  # inherits LogTag, can't really handle this particular case yet

    ### calib3d ###
    "cv::CirclesGridFinderParameters2",  # requires proper inheritance of simple classes

    ### features2d ###
    "cv::DrawMatchesFlags",  # dummy type only used to contain anonymous enum, 3.x only

    ### stitching ###
    "cv::CylindricalWarperGpu", "cv::PlaneWarperGpu", "cv::SphericalWarperGpu",

    ### videostab ###
    "cv::videostab::DensePyrLkOptFlowEstimatorGpu",
    "cv::videostab::KeypointBasedMotionEstimatorGpu",
    "cv::videostab::MoreAccurateMotionWobbleSuppressorGpu",
    "cv::videostab::SparsePyrLkOptFlowEstimatorGpu",

    ### ml ###
    "cv::ml::SimulatedAnnealingSolverSystem",  # only defined in docs

    ### dnn ###
    "cv::dnn::_Range",
)

# list of constants to skip, elements are regular expressions for re.match() against ConstInfo.name
const_ignore_list = (
    "CV_EXPORTS_W", "CV_MAKE_TYPE",
    "CV_IS_CONT_MAT", "CV_RNG_COEFF", "IPL_IMAGE_MAGIC_VAL",
    "CV_SET_ELEM_FREE_FLAG", "CV_FOURCC_DEFAULT",
    "CV_WHOLE_ARR", "CV_WHOLE_SEQ", "CV_PI", "CV_2PI", "CV_LOG2",
    "CV_TYPE_NAME_IMAGE",
    "CV_SUPPRESS_DEPRECATED_START",
    "CV_SUPPRESS_DEPRECATED_END",
    "__CV_BEGIN__", "__CV_END__", "__CV_EXIT__",
    "CV_IMPL_IPP", "CV_IMPL_MT", "CV_IMPL_OCL", "CV_IMPL_PLAIN", "CV_IPP_CHECK_COND", "IPP_INITIALIZER_AUTO", "IPP_VERSION_X100",
    "CV_TRY", "CV_CATCH_ALL",
    "CV__DEBUG_NS_",
    "UINT64_1", "UINT32_1",
    "CV_STRUCT_INITIALIZER", "CV__ENABLE_C_API_CTORS",
    "VSX_IMPL_MULH_",
    "CV__DNN_EXPERIMENTAL_NS_",
    "CV_Sts",
    "CV_ALWAYS_INLINE",
    "CV_16FC",
    "CV__DNN_INLINE_NS",
    "ENUM_LOG_LEVEL_FORCE_INT", "CV_LOGTAG_FALLBACK", "CV_LOGTAG_GLOBAL",
    "CV_USRTYPE1",
    "CV_INSTRUMENT_GET_RETURN_ADDRESS",
    "CVVISUAL_FUNCTION_NAME_MACRO", "CVVISUAL_LOCATION", "CVVISUAL_THREAD_LOCAL",
)

# set of functions that should have unsafe in their declaration, element is FuncInfo.identifier
func_unsafe_list = {
    # allocates uninitialized memory
    "cv_Mat_Mat_int_int_int",
    "cv_Mat_Mat_Size_int",
    "cv_Mat_Mat_VectorOfint_int",
    "cv_Mat_create_int_int_int",
    "cv_Mat_create_Size_int",
    "cv_Mat_create_VectorOfint_int",
    "cv_UMat_UMat_int_int_int_UMatUsageFlags",
    "cv_UMat_UMat_Size_int_UMatUsageFlags",
    "cv_UMat_UMat_int_const_int_X_int_UMatUsageFlags",
    "cv_UMat_create_int_int_int_UMatUsageFlags",
    "cv_UMat_create_Size_int_UMatUsageFlags",
    "cv_UMat_create_VectorOfint_int_UMatUsageFlags",
    # allows passing arbitrary data
    "cv_Mat_set_data_uchar_X",
    # no bounds checking
    "cv_Mat_ptr_int",
    "cv_Mat_ptr_const_int",
    "cv_Mat_ptr_int_int",
    "cv_Mat_ptr_const_int_int",
    "cv_Mat_ptr_int_int_int",
    "cv_Mat_ptr_const_int_int_int",
    "cv_Mat_ptr_const_int_X",
    "cv_Mat_ptr_const_const_int_X",
    # pointer to internal data
    "cv_dnn_Dict_ptr_String",
    "cv_dnn_Dict_ptr_const_String",
}

# dict of types to replace if cannot be handled automatically
# key: typeid (full class path with . replaced by ::)
# value: replacement typeid
type_replace = {
    "vector_Mat": "vector<cv::Mat>",
    "vector_float": "vector<float>",
    "_Range": "cv::Range",
    "Point_<int>": "Point2i",
    "Point_<int64>": "Point2l",
    "Point_<float>": "Point2f",
    "Point_<double>": "Point2d",
    "Point3_<int>": "Point3i",
    "Point3_<float>": "Point3f",
    "Point3_<double>": "Point3d",
    "Rect_<int>": "Rect2i",
    "Rect_<float>": "Rect2f",
    "Rect_<double>": "Rect2d",
    "Size_<int64>": "Size2l",
    "Size_<float>": "Size2f",
    "Size_<double>": "Size2d",
    "Scalar_<double>": "Scalar",
    "cv::xfeatures2d::Feature2D": "cv::Feature2D",
}

# dict for handling primitives
# key: primitive typeid
# value: dict
#   keys: "cpp_extern", "rust_local"
#   values: corresponding native typeid
# fixme, is "cpp_extern" needed at all?
primitives = {
    "void": {"cpp_extern": "void", "rust_local": "()"},

    "bool": {"cpp_extern": "bool", "rust_local": "bool"},

    "char": {"cpp_extern": "char", "rust_local": "i8"},
    "schar": {"cpp_extern": "char", "rust_local": "i8"},
    "signed char": {"cpp_extern": "char", "rust_local": "i8"},
    "uchar": {"cpp_extern": "unsigned char", "rust_local": "u8"},
    "unsigned char": {"cpp_extern": "unsigned char", "rust_local": "u8"},

    "short": {"cpp_extern": "short", "rust_local": "i16"},
    "signed short": {"cpp_extern": "short", "rust_local": "i16"},
    "ushort": {"cpp_extern": "unsigned short", "rust_local": "u16"},
    "unsigned short": {"cpp_extern": "unsigned short", "rust_local": "u16"},

    "int": {"cpp_extern": "int", "rust_local": "i32"},
    "signed int": {"cpp_extern": "int", "rust_local": "i32"},
    "uint": {"cpp_extern": "unsigned int", "rust_local": "u32"},
    "unsigned": {"cpp_extern": "unsigned int", "rust_local": "u32"},
    "unsigned int": {"cpp_extern": "unsigned int", "rust_local": "u32"},
    "uint32_t": {"cpp_extern": "uint32_t", "rust_local": "u32"},

    "size_t": {"cpp_extern": "std::size_t", "rust_local": "size_t"},
    "ptrdiff_t": {"cpp_extern": "std::ptrdiff_t", "rust_local": "ptrdiff_t"},

    "int64": {"cpp_extern": "int64", "rust_local": "i64"},
    "__int64": {"cpp_extern": "int64", "rust_local": "i64"},
    "signed __int64": {"cpp_extern": "int64", "rust_local": "i64"},
    "uint64": {"cpp_extern": "uint64", "rust_local": "u64"},
    "unsigned __int64": {"cpp_extern": "uint64", "rust_local": "u64"},
    "unsigned long long": {"cpp_extern": "unsigned long long", "rust_local": "u64"},
    "int64_t": {"cpp_extern": "int64_t", "rust_local": "i64"},
    "uint64_t": {"cpp_extern": "uint64_t", "rust_local": "u64"},

    "float": {"cpp_extern": "float", "rust_local": "f32"},
    "double": {"cpp_extern": "double", "rust_local": "f64"},
}

_forward_mut_rust_safe = template("""
${doc_comment}${visibility}fn ${r_name}<T: core::DataType>(${args}) -> Result<&mut T> { ${pre_call_args}self._${r_name}(${forward_args}) }

""")
_forward_const_rust_safe = template("""
${doc_comment}${visibility}fn ${r_name}<T: core::DataType>(${args}) -> Result<&T> { ${pre_call_args}self._${r_name}(${forward_args}) }

""")

# dict of functions with manual implementations
# key: FuncInfo.identifier
# value: dict
#   keys: "rust_safe", "rust_extern", "cpp", missing key means skip particular implementation
#   values: template to use for manual implementation or "~" to use default implementation
func_manual = {
    "cv_Mat_at_int": {
        "rust_safe": _forward_mut_rust_safe,
    },
    "cv_Mat_at_const_int": {
        "rust_safe": _forward_const_rust_safe,
    },
    "cv_Mat_at_int_int": {
        "rust_safe": _forward_mut_rust_safe,
    },
    "cv_Mat_at_const_int_int": {
        "rust_safe": _forward_const_rust_safe,
    },
    "cv_Mat_at_Point": {
        "rust_safe": _forward_mut_rust_safe,
    },
    "cv_Mat_at_const_Point": {
        "rust_safe": _forward_const_rust_safe,
    },
    "cv_Mat_at_int_int_int": {
        "rust_safe": _forward_mut_rust_safe,
    },
    "cv_Mat_at_const_int_int_int":  {
        "rust_safe": _forward_const_rust_safe,
    },
    "cv_Mat_at_const_int_X": {
        "rust_safe": _forward_mut_rust_safe,
    },
    "cv_Mat_at_const_const_int_X": {
        "rust_safe": _forward_const_rust_safe,
    },
}

# dict of manual declaration for types
# key: module name
# value: dict
#   key: local rust type name
#   value: dict
#     keys: "cpp", "rust", missing key means skip particular implementation
#     values: template to use for manual implementation or "~" to use default implementation
type_manual = {}


def _base_type_alias(module, rust_name, rust_definition, cpp_field_type, cpp_fields):
    if module not in type_manual:
        type_manual[module] = {}
    type_manual[module][rust_name] = {
        "rust": template(template("""
            ${doc_comment}pub type ${rust_local} = ${definition};
        """).substitute({"doc_comment": "${doc_comment}", "rust_local": "${rust_local}", "definition": rust_definition})),
        "cpp": "~",
    }
    cpp_props = [[cpp_field_type, x, "", "/RW"] for x in cpp_fields]
    if module not in decls_manual_pre:
        decls_manual_pre[module] = []
    decls_manual_pre[module].insert(0, ("class cv.{}".format(rust_name), "", ["/Simple"], cpp_props))


_base_type_alias("core", "Scalar", "core::Scalar_<f64>", "double", ("data[4]",))

for s in (2, 3, 4, 6, 8):
    types = ("b", "unsigned char"), \
            ("s", "short"), \
            ("w", "unsigned short"), \
            ("i", "int"), \
            ("l", "int64"), \
            ("f", "float"), \
            ("d", "double")
    if s == 6:
        types = (x for x in types if x[0] in ("d", "f", "i"))
    elif s == 8:
        types = (x for x in types if x[0] == "i")
    for t in types:
        rust_local = primitives[t[1]]["rust_local"]
        if s == 2:
            if t[0] == "i":
                _base_type_alias("core", "Rect", "core::Rect_<{}>".format(rust_local), t[1], ("x", "y", "width", "height"))
                _base_type_alias("core", "Point", "core::Point_<{}>".format(rust_local), t[1], ("x", "y"))
                _base_type_alias("core", "Size", "core::Size_<{}>".format(rust_local), t[1], ("width", "height"))
            if t[0] in ("i", "f", "d"):
                _base_type_alias("core", "Rect{}{}".format(s, t[0]), "core::Rect_<{}>".format(rust_local), t[1], ("x", "y", "width", "height"))
            if t[0] in ("i", "l", "f", "d"):
                _base_type_alias("core", "Point{}{}".format(s, t[0]), "core::Point_<{}>".format(rust_local), t[1], ("x", "y"))
                _base_type_alias("core", "Size{}{}".format(s, t[0]), "core::Size_<{}>".format(rust_local), t[1], ("width", "height"))
        elif s == 3 and t[0] in ("i", "f", "d"):
            _base_type_alias("core", "Point{}{}".format(s, t[0]), "core::Point3_<{}>".format(rust_local), t[1], ("x", "y", "z"))
        if t[0] != "l":
            _base_type_alias("core", "Vec{}{}".format(s, t[0]), "core::Vec{}<{}>".format(s, rust_local), t[1], ("data[{}]".format(s),))

# set of types that must be generated as traits, elements are typeids
forced_class_trait = {
    "cv::Algorithm",
    "cv::Feature2D",
    "cv::BackgroundSubtractor",
    "cv::dnn::Layer",
    "cv::MatOp",
    "cv::aruco::Board",
    "cv::_InputArray",
    "cv::_OutputArray",
    "cv::_InputOutputArray",
}

# set of types that must be generated as traits, elements are typeids
forced_class_abstract = {
    "cv::dnn::BackendNode",
}

# set of classes that must be forced to be non-simple, elements are declarations (decl[0])
forced_non_simple = {
    "class cv.ocl.Device",
    "class cv.FileNode",
    "class cv.dnn.Net",  # dnn::Net is incorrectly marked as simple
    "class cv.dnn.Model",
    "class cv.dnn.DetectionModel",
    "class cv.dnn.ClassificationModel",
}

# dict of pointer arguments that need to be made into slices, the conversion only happens if an arg is a pointer
# key: module name
# value: dict
#   key: function name as it comes from parse_hdr
#   value: dict or tuple of dicts
#     key: "arg_count"
#     value: count of arguments that the function must have
#     key: "arg_name"
#     value: string name of the argument that needs converting to slice
force_slice = {
    "core": {
        # replace pointers with slices for nd Mat access methods
        "cv.Mat.at": {
            "arg_count": 1,
            "arg_name": "idx",
        },
        "cv.Mat.ptr": {
            "arg_count": 1,
            "arg_name": "idx",
        },
        # accepts pointer to array of steps, make it slice
        "cv.Mat.Mat": {
            "arg_count": 4,
            "arg_name": "steps",
        },
        # accepts pointer to array of dimensions, make it slice
        "cv.Mat.zeros": {
            "arg_count": 3,
            "arg_name": "sz",
        },
        # accepts pointer to array of dimensions, make it slice
        "cv.Mat.ones": {
            "arg_count": 3,
            "arg_name": "sz",
        },
        # accepts pointer to array of dimensions, make it slice
        "cv.UMat.UMat": (
            {
                "arg_count": 4,
                "arg_name": "sizes",
            },
            {
                "arg_count": 5,
                "arg_name": "sizes",
            }
        )
    },
}

# set of enum's that need to be generated, other enums are just expanded to constants, elements are EnumInfo.fullname
enum_generate = {
    ### aruco ###
    "cv::aruco::PREDEFINED_DICTIONARY_NAME",

    ### calib3d ###
    "cv::HandEyeCalibrationMethod",
    "cv::SolvePnPMethod",

    ### core ###
    "cv::AccessFlag",
    "cv::Formatter::FormatType",
    "cv::instr::FLAGS",
    "cv::instr::IMPL",
    "cv::instr::TYPE",
    "cv::UMatUsageFlags",
    "cv::UndistortTypes",
    "cv::WindowFlags",
    "cv::WindowPropertyFlags",
    "cv::utils::logging::LogLevel",
    "cv::_InputArray::KindFlag",
    "cv::_OutputArray::DepthMask",
    "cv::ocl::OclVectorStrategy",

    ### dnn ###
    "cv::dnn::Backend",
    "cv::dnn::Target",

    ### features2d ###
    "cv::AgastFeatureDetector::DetectorType",
    "cv::AKAZE::DescriptorType",
    "cv::DescriptorMatcher::MatcherType",
    "cv::DrawMatchesFlags",
    "cv::FastFeatureDetector::DetectorType",
    "cv::KAZE::DiffusivityType",
    "cv::ORB::ScoreType",

    ### imgproc ###
    "cv::InterpolationFlags",

    ### objdetect ###
    "cv::HOGDescriptor::HistogramNormType",
    "cv::DetectionBasedTracker::ObjectStatus",

    ### stitching ###
    "cv::Stitcher::Mode",
    "cv::Stitcher::Status",

    ### videoio ###
    "cv::VideoCaptureAPIs",

    ### xfeatures2d ###
    "cv::xfeatures2d::DAISY::NormalizationType",
}

# dict of enum discriminants to exclude from the bindings (e.g. due to duplicate values)
# key: EnumInfo.fullname
# value: set of ConstInfo.name
enum_ignore_discriminant = {
    "cv::AccessFlag": {"ACCESS_MASK"},
    "cv::BorderTypes": {"BORDER_REFLECT101", "BORDER_DEFAULT"},
    "cv::CpuFeatures": {"CPU_AVX_512IFMA512"},
    "cv::DftFlags": {"DCT_INVERSE", "DCT_ROWS"},
    "cv::FileStorage::Mode": {"FORMAT_AUTO"},
    "cv::NormTypes": {"NORM_TYPE_MASK"},
    "cv::ocl::OclVectorStrategy": {"OCL_VECTOR_DEFAULT"},
    "cv::utils::logging::LogLevel": {"ENUM_LOG_LEVEL_FORCE_INT"},
    "cv::VideoCaptureAPIs": {"CAP_VFW", "CAP_V4L2", "CAP_FIREWARE", "CAP_IEEE1394", "CAP_DC1394", "CAP_CMU1394", "CAP_REALSENSE"},
    "cv::WindowFlags": {"WINDOW_FULLSCREEN", "WINDOW_KEEPRATIO", "WINDOW_GUI_EXPANDED"},
}

# dict of reserved Rust keywords and their replacement to be used in var, function and class names
# key: reserved keyword
# value: replacement
reserved_rename = {
    "box": "_box",
    "fn": "_fn",
    "in": "_in",
    "match": "_match",
    "move": "_move",
    "ref": "_ref",
    "type": "_type",  # fixme: "typ" is better rename
    "use": "_use",
    "loop": "_loop",
}

# list of modules that are imported into every other module so there is no need to reference them using full path, elements are module names
static_modules = ("core", "sys", "types")

data_type_typeids = {
    "uchar", "char", "ushort", "short", "int",
    "float", "double",
    "cv::Vec2b", "cv::Vec3b", "cv::Vec4b",
    "cv::Vec2s", "cv::Vec3s", "cv::Vec4s",
    "cv::Vec2i", "cv::Vec3i", "cv::Vec4i",
    "cv::Vec2f", "cv::Vec3f", "cv::Vec4f",
    "cv::Vec2d", "cv::Vec3d", "cv::Vec4d",
    "cv::Scalar",
    "cv::Point", "cv::Point2i", "cv::Point2f", "cv::Point2d",
    "cv::Point3i", "cv::Point3f", "cv::Point3d",
    "cv::Size", "cv::Size2i", "cv::Size2f", "cv::Size2d",
    "cv::Rect", "cv::Rect2i", "cv::Rect2f", "cv::Rect2d",
}


def decl_patch(module, decl):
    force_slice_module_decls = force_slice.get(module)
    if force_slice_module_decls is not None:
        force_slice_fn_decls = force_slice_module_decls.get(decl[0])
        if force_slice_fn_decls is not None:
            if not isinstance(force_slice_fn_decls, tuple):
                force_slice_fn_decls = (force_slice_fn_decls,)
            for force_slice_decl in force_slice_fn_decls:
                if len(decl[3]) == force_slice_decl["arg_count"]:
                    for i, _ in enumerate(decl[3]):
                        if decl[3][i][1] == force_slice_decl["arg_name"]:
                            decl[3][i][0] = decl[3][i][0].replace("*", "[]")

    # force class to be non-simple
    if decl[0] in forced_non_simple and "/Simple" in decl[2]:
        decl[2].remove("/Simple")

    if module == "core":
        # size() and step() of Mat and UMat should be const
        if decl[0] == "cv.Mat.size" or decl[0] == "cv.Mat.step" or decl[0] == "cv.UMat.size" or decl[0] == "cv.UMat.step":
            decl[2].append("/C")
    elif module == "dnn":
        # set method takes generic, force it to take DictValue wrapper
        if decl[0] == "cv.dnn.Dict.set":
            decl[1] = "DictValue&"
            decl[3][1][0] = "DictValue&"
    elif module == "ml":
        # loadFromCSV is not parsed correctly due to default value being a comma
        if decl[0] == "cv.ml.TrainData.loadFromCSV" and len(decl[3]) == 8 and decl[3][6][0] == "'":
            decl[3][5][2] = "','"
            del decl[3][6]
    return decl


#
#       Helpers
#

def camel_case_to_snake_case(name):
    res = re.sub('([^_])([A-Z][a-z]+)', r'\1_\2', name)
    res = re.sub('([a-z0-9])([A-Z])', r'\1_\2', res)
    res = re.sub(r'\B([23])_(D)\b', r'_\1\2', res)  # fix 2_d and 3_d
    res = re.sub(r'_(P[n3])_(P)', r'_\1\2', res)  # fix p3_p and pn_p
    res = re.sub(r'Open_(CL|Gl|VX)', r'Open\1', res)  # fix open_cl, open_gl, open_vx
    return res.lower()


def bump_counter(name):
    """
    :type name: str
    :rtype: str
    """
    pos = len(name) - 1
    for pos in range(len(name) - 1, 0, -1):
        if not name[pos].isdigit():
            break
    base_name = name[:pos + 1]
    try:
        counter = int(name[pos + 1:])
    except ValueError:
        base_name += "_"
        counter = 0
    return "{}{}".format(base_name, counter + 1)


def split_known_namespace(name, namespaces):
    """
    :type name: str
    :type namespaces: iterable
    :rtype: (str, str)
    """
    if "::" in name:
        for namespace in sorted(namespaces, key=len, reverse=True):
            namespace_colon = namespace + "::"
            if name.startswith(namespace_colon):
                return namespace, name[len(namespace_colon):]
        return "", name
    else:
        return "", name

#
#       AST-LIKE
#


class GeneralInfo:
    def __init__(self, gen, name, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type name: str
        :type namespaces: frozenset|set
        """
        self.gen = gen
        self.fullname, self.namespace, self.classpath, self.classname, self.name = self.do_parse_name(name, namespaces)
        if self.fullname.startswith("cv::"):
            self.short_fullname = self.fullname[4:]  # without cv:: prefix
        else:
            self.short_fullname = self.fullname
        logging.info(
            "parse_name: %s with %s -> fullname:%s namespace:%s classpath:%s classname:%s name:%s" %
            (name, sorted(namespaces), self.fullname, self.namespace, self.classpath, self.classname, self.name)
        )

    def do_parse_name(self, name, namespaces):
        """
        input: full name and available namespaces
        returns: (fullname, namespace, classpath, classname, name)
            fullname clean of prefix like "const, class, ..."
        """
        name = name \
            .replace("const ", "") \
            .replace("struct ", "") \
            .replace("class ", "") \
            .replace("typedef ", "") \
            .replace("callback ", "") \
            .replace("enum ", "") \
            .replace(".", "::")
        space_name, local_name = split_known_namespace(name, namespaces)
        pieces = local_name.split("::")
        if len(pieces) > 2:  # <class>.<class>.<class>.<name>
            return name, space_name, "::".join(pieces[:-1]), pieces[-2], pieces[-1]
        elif len(pieces) == 2:  # <class>.<name>
            return name, space_name, pieces[0], pieces[0], pieces[1]
        elif len(pieces) == 1:  # <name>
            return name, space_name, "", "", pieces[0]
        else:
            return name, space_name, "", ""  # error?!


class ArgInfo:
    def __init__(self, gen, arg_tuple):  # [ ctype, name, def val, [mod], argno ]
        """
        :type gen: RustWrapperGenerator
        :param arg_tuple:
        """
        self.gen = gen
        typ = arg_tuple[0].replace("enum ", "")
        self.type = self.gen.get_type_info(typ)
        self.name = arg_tuple[1]
        if not self.name:
            self.name = "unnamed_arg"
        self.rsname = camel_case_to_snake_case(reserved_rename.get(self.name, self.name))
        self.defval = ""
        if len(arg_tuple) > 2:
            self.defval = arg_tuple[2]
        self.out = ""
        # "const _OutputArray&" is a special case for InputArray::copyTo
        if typ in ("OutputArray", "OutputArrayOfArrays", "const _OutputArray&") or len(arg_tuple) > 3 and "/O" in arg_tuple[3] or self.type.is_by_ref and not self.type.is_const:
            self.out = "O"
        if typ in ("InputOutputArray", "InputOutputArrayOfArrays") or len(arg_tuple) > 3 and "/IO" in arg_tuple[3]:
            self.out = "IO"

    def is_output(self):
        return self.out in ("O", "IO")

    def __repr__(self):
        return template("ARG $ctype$p $name=$defval").substitute(ctype=self.type,
                                                                  p=" *" if isinstance(self.type, RawPtrTypeInfo) else "",
                                                                  name=self.name,
                                                                  defval="" #self.defval
                                                                )


class FuncInfo(GeneralInfo):
    KIND_FUNCTION    = "(function)"
    KIND_METHOD      = "(method)"
    KIND_METHOD_CONVERT = "(convertor method)"
    KIND_CONSTRUCTOR = "(constructor)"

    TEMPLATES = {
        "cpp": template("""
                // parsed: ${fullname}
                // as:     ${repr}
                ${args}// Return value: ${rv_type}
                ${return_wrapper_type} ${identifier}(${decl_cpp_args}) {
                    try {${pre_call_args}
                ${call}${post_call_args}
                ${rv}
                    } CVRS_CATCH(${return_wrapper_type})
                }

            """),

        "cpp_doc_arg": template("""// Arg ${repr}${ptr} ${type} = ${defval}${ignored}\n"""),

        "rust_safe": template("""
                ${doc_comment}${visibility}${unsafety_decl}fn ${r_name}${generic_decl}(${args}) -> Result<$rv_rust_full> {${pre_call_args}
                    ${prefix}${unsafety_call}{ sys::${identifier}(${call_args}) }.into_result()${suffix}${rv}${post_call_args}
                }
                
            """),

        "rust_safe_rv_string": template("""
                .map(crate::templ::receive_string)"""),

        "rust_safe_rv_string_mut": template("""
                .map(crate::templ::receive_string_mut)"""),

        "rust_safe_rv_const_raw_ptr": template("""
            .and_then(|x| ${unsafety_call}{ x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))"""),

        "rust_safe_rv_mut_raw_ptr": template("""
            .and_then(|x| ${unsafety_call}{ x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))"""),

        "rust_safe_rv_by_ptr": template(""".map(|ptr| ${rv_rust_full} { ptr })"""),

        "rust_safe_rv_other": template(""""""),

        "rust_extern": template("""
            pub fn ${identifier}(${args}) -> ${return_wrapper_type};
         """),
    }

    def __init__(self, gen, module, decl, namespaces=frozenset()):  # [ funcname, return_ctype, [modifiers], [args] ]
        """
        :type gen: RustWrapperGenerator
        :type module: str
        :type decl: list
        :type namespaces: frozenset|set
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.module = module

        self.is_ignored = False
        if self.classname and not self.classname.startswith("operator"):
            self.ci = gen.get_class(self.classpath)
            if not self.ci:
                if self.classname == "std" or "<" in self.classname:
                    self.is_ignored = True
                else:
                    raise NameError("class not found: " + self.classname)
            if "/A" in decl[2]:
                self.ci.is_trait = True
                self.ci.is_abstract = True
            if self.classname == self.name:
                self.kind = self.KIND_CONSTRUCTOR
                if len(decl[3]) == 0:
                    self.name = "default"
                elif len(decl[3]) == 1 and decl[3][0][0].replace(" ", "").endswith("{}&".format(self.name)):
                    self.name = "copy"
                else:
                    self.name = "new"
                self.type = gen.get_type_info(self.classpath)
            else:
                operator_convert = False
                m = re.match(r"operator\s+(\w+)$", self.name)
                if m is not None:
                    out_type = gen.get_type_info(m.group(1))
                    if out_type is not None and not isinstance(out_type, UnknownTypeInfo):
                        operator_convert = True
                        self.kind = self.KIND_METHOD_CONVERT
                        self.type = out_type
                if not operator_convert:
                    self.kind = self.KIND_METHOD
                    self.type = gen.get_type_info(decl[1])
        else:
            self.kind = self.KIND_FUNCTION
            self.ci = None  # type: ClassInfo
            self.type = gen.get_type_info(decl[1])

        self.identifier = self.fullname.replace("::", "_").replace(" ", "_")

        self.is_ignored = self.is_ignored or "/H" in decl[2] or "/I" in decl[2]

        self.is_const = "/C" in decl[2]
        self.is_static = "/S" in decl[2]
        self.attr_accessor_type = None
        if "/ATTRGETTER" in decl[2]:
            self.attr_accessor_type = "r"
        elif "/ATTRSETTER" in decl[2]:
            self.attr_accessor_type = "w"
        self.has_callback_arg = False
        has_userdata_arg = False

        if self.is_const:
            self.identifier += "_const"

        self.args = []
        for arg in decl[3]:
            ai = ArgInfo(gen, arg)
            if self.has_callback_arg and ai.name == "userdata":
                has_userdata_arg = True
            while any(True for x in self.args if x.name == ai.name):
                ai.name = bump_counter(ai.name)
                ai.rsname = camel_case_to_snake_case(reserved_rename.get(ai.name, ai.name))
            self.args.append(ai)
            self.identifier += "_" + ai.type.rust_safe_id
            if isinstance(ai.type, CallbackTypeInfo):
                self.has_callback_arg = True

        if self.has_callback_arg and not has_userdata_arg:
            logging.info("ignore function with callback, but without userdata %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if len(decl) > 5:
            self.comment = decl[5]
        else:
            self.comment = ""

        self.cname = self.cppname = self.name
        self.is_safe = self.identifier not in func_unsafe_list

        if self.name.startswith("~"):
            logging.info("ignore destructor %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if (self.name.startswith("operator") or self.fullname.startswith("operator ")) and self.kind != self.KIND_METHOD_CONVERT:
            logging.info("ignore %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

    def _get_manual_implementation_tpl(self, section):
        params = func_manual.get(self.identifier)
        if params is not None:
            tmpl = params.get(section)
            if tmpl is None:
                return template("")
            elif tmpl == "~":
                return None
            return tmpl
        return None

    def is_constructor(self):
        return self.kind == self.KIND_CONSTRUCTOR

    def is_instance_method(self):
        return not self.is_static and self.ci and not self.is_constructor()

    def rv_type(self):
        """
        :rtype: TypeInfo
        """
        if self.is_constructor():
            if self.ci:
                return self.gen.get_type_info(self.ci.fullname)
            else:
                return None
        else:
            return self.type

    def reason_to_skip(self):
        if self.identifier in self.gen.generated:
            return "already there"

        if func_rename.get(self.identifier) == "-":
            return "ignored by rename table"

        if self.identifier in func_manual:
            return None

        if self.name.startswith("operator") and self.kind != self.KIND_METHOD_CONVERT:
            return "can not map %s yet"%(self.name)

        if not self.rv_type():
            return "rv_header_type returns None. this is an error. (class not found ?)"

        if self.type.is_ignored:
            return "return type class %s is ignored"%(self.type)

        if self.rv_type().is_ignored:
            return "return value type is ignored"

        if self.kind == self.KIND_CONSTRUCTOR and self.ci.is_abstract:
            return "skip constructor of abstract class"

        for a in self.args:
            if a.type.is_ignored:
                return "can not map type %s yet"%(a.type)

        return None

    def r_name(self):
        name = func_rename.get(self.identifier)
        if name is None:
            if self.kind == self.KIND_METHOD_CONVERT:
                name = re.sub(r"^operator\s+", "to_", self.name)
            else:
                name = self.name
        else:
            if "+" in name:
                name = name.replace("+", self.name)
                name = camel_case_to_snake_case(reserved_rename.get(name, name))
            return name
        return camel_case_to_snake_case(reserved_rename.get(name, name))

    def gen_cpp(self):
        decl_cpp_args = []
        pre_call_args = []
        post_call_args = []
        args = ""
        if self.is_instance_method():
            # fixme? add RawPtr handling
            decl_cpp_args.append(self.ci.type_info().cpp_extern + " instance")

        call_cpp_args = []
        for arg in self.args:
            ignored = ptr = ""
            if arg.type.is_ignored:
                ignored = " (ignored)"
            if isinstance(arg.type, RawPtrTypeInfo):
                ptr = " (ptr)"
            pre_call_arg = arg.type.cpp_arg_pre_call(arg.name)
            if pre_call_arg:
                pre_call_args.append(pre_call_arg)
            post_call_arg = arg.type.cpp_arg_post_call(arg.name)
            if post_call_arg:
                post_call_args.append(post_call_arg)

            args += FuncInfo.TEMPLATES["cpp_doc_arg"].substitute(combine_dicts(arg.__dict__, {
                "repr": repr(arg),
                "ptr": ptr,
                "ignored": ignored
            }))

            decl_cpp_args.append(arg.type.cpp_arg_func_decl(arg.name, arg.is_output()))
            call_cpp_args.append(arg.type.cpp_arg_func_call(arg.name, arg.is_output()))

        # cpp method call with prefix
        if self.is_constructor():
            call_name = self.ci.fullname
        elif self.ci is None or self.is_static:
            if self.namespace == "":
                call_name = self.cppname
            else:
                call_name = self.fullname
        else:
            call_name = self.ci.type_info().cpp_method_call_name(self.cppname)

        # actual call
        call = self.rv_type().cpp_method_call_invoke(call_name, ", ".join(call_cpp_args), self.is_constructor(), self.attr_accessor_type)

        template_vars = combine_dicts(self.__dict__, {
            "repr": repr(self),
            "rv_type": self.rv_type(),
            "args": args,
            "return_wrapper_type": self.rv_type().rust_cpp_return_wrapper_type(),
            "decl_cpp_args": ", ".join(decl_cpp_args),
            "pre_call_args": "".join("\n" + indent(x, 2) + ";" for x in pre_call_args),
            "post_call_args": "".join("\n" + indent(x, 2) + ";" for x in post_call_args),
            "call": indent(call, 2),
            "rv": indent(self.rv_type().cpp_method_return(self.is_constructor()), 2),
        })

        tmpl = self._get_manual_implementation_tpl("cpp")
        if tmpl is None:
            self.rv_type().gen_return_wrappers(self.gen.cpp_dir, self.gen.rust_dir)
            tmpl = FuncInfo.TEMPLATES["cpp"]
        return tmpl.substitute(template_vars)

    def gen_rust_extern(self):
        args = []
        if self.is_instance_method():
            args.append(self.ci.type_info().rust_extern_self_func_decl(not self.is_const))
        for a in self.args:
            args.append(a.type.rust_extern_arg_func_decl(a.rsname))
        template_vars = combine_dicts(self.__dict__, {
            "args": ", ".join(args),
            "return_wrapper_type": self.rv_type().rust_cpp_return_wrapper_type(),
        })
        tmpl = self._get_manual_implementation_tpl("rust_extern") or FuncInfo.TEMPLATES["rust_extern"]
        return tmpl.substitute(template_vars)

    def gen_safe_rust(self, visibility):
        args = []
        call_args = []
        forward_args = []
        pre_call_args = []
        post_call_args = []

        lifetimes = set()  # todo implement lifetime elision rules, type should specify only &type and do replacement of & with &'a
        # if self.rv_type().rust_lifetimes:
        #     lifetimes.add(self.rv_type().rust_lifetimes)

        # for a in self.args:
        #     if a.type.rust_lifetimes:
        #         lifetimes.add(a.type.rust_lifetimes)

        if len(lifetimes) > 0:
            lifetimes = ", ".join(lifetimes)
        else:
            lifetimes = ""

        if self.is_instance_method():
            args.append(self.ci.type_info().rust_self_func_decl(not self.is_const))
            call_args.append(self.ci.type_info().rust_self_func_call(not self.is_const))

        generic_decls = []

        # todo: convert some *const Mat to slices in rust
        for arg in self.args:
            call_args.append(arg.type.rust_arg_func_call(arg.rsname, arg.is_output()))
            forward_args.append(arg.type.rust_arg_forward(arg.rsname))
            pre_call_arg = arg.type.rust_arg_pre_call(arg.rsname)
            if pre_call_arg:
                pre_call_args.append(pre_call_arg)
            post_call_arg = arg.type.rust_arg_post_call(arg.rsname)
            if post_call_arg:
                post_call_args.append(post_call_arg)
            gdecl = arg.type.rust_generic_decl()
            if gdecl:
                generic_decls.append(gdecl)
            if self.has_callback_arg and arg.name == "userdata":
                continue
            args.append(arg.type.rust_arg_func_decl(arg.rsname, arg.is_output(), self.attr_accessor_type))

        doc_comment = self.gen.reformat_doc(self.comment, self)

        defattr_doc_comment = ""
        for arg in (x for x in self.args if x.defval != ""):
            if not defattr_doc_comment:
                defattr_doc_comment += "///\n/// ## C++ default parameters\n"
            defattr_doc_comment += "/// * %s: %s\n" % (arg.rsname, arg.defval)
        if defattr_doc_comment:
            attr_pos = doc_comment.find("#[")
            if attr_pos == -1:
                attr_pos = len(doc_comment)
            doc_comment = doc_comment[:attr_pos] + defattr_doc_comment + doc_comment[attr_pos:]
        prefix = ""
        suffix = ""
        if len(post_call_args) > 0:
            post_call_args.append("return out")
            prefix = "let out = "
            suffix = ";"

        template_vars = combine_dicts(self.__dict__, {
            "doc_comment": doc_comment,
            "rv_rust_full": self.rv_type().rust_full,
            "unsafety_decl": "" if self.is_safe else "unsafe ",
            "unsafety_call": "unsafe " if self.is_safe else "",
            "visibility": visibility,
            "generic_decl": "<{}>".format(", ".join(generic_decls)) if len(generic_decls) >= 1 else "",
            "prefix": prefix,
            "suffix": suffix,
            "args": ", ".join(args),
            "pre_call_args": "".join("\n" + indent(x) + ";" for x in pre_call_args),
            "post_call_args": "".join("\n" + indent(x) + ";" for x in post_call_args),
            "r_name": self.r_name(),
            "call_args": ", ".join(call_args),
            "forward_args": ", ".join(forward_args),
        })
        if isinstance(self.rv_type(), StringTypeInfo) or isinstance(self.rv_type(), RawPtrTypeInfo) and self.rv_type().is_string:
            if self.rv_type().is_const:
                rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_string"].substitute(template_vars)
            else:
                rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_string_mut"].substitute(template_vars)
        elif self.rv_type().is_by_ptr:
            rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_by_ptr"].substitute(template_vars)
        elif isinstance(self.rv_type(), RawPtrTypeInfo):
            rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_const_raw_ptr" if self.rv_type().is_const else "rust_safe_rv_mut_raw_ptr"].substitute(template_vars)
        else:
            rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_other"].substitute(template_vars)

        template_vars["rv"] = rv_rust

        block = self._get_manual_implementation_tpl("rust_safe") or FuncInfo.TEMPLATES["rust_safe"]
        block = block.substitute(template_vars)

        if self.kind == self.KIND_FUNCTION:
            return block
        else:
            return indent(block)

    def __repr__(self):
        if self.kind == self.KIND_FUNCTION:
            return "%s %s"%(self.fullname, self.kind)
        else:
            return "%s %s %s . %s"%(self.fullname, self.kind, self.ci, self.name)


class ClassPropInfo:
    def __init__(self, decl):  # [f_ctype, f_name, '', '/RW']
        self.is_const = "/C" in decl[3]
        self.ctype = "{}{}".format("const " if self.is_const else "", decl[0])
        self.name = decl[1]
        self.comment = decl[2]
        self.rw = "/RW" in decl[3]

    def __repr__(self):
        return template("PROP $ctype $name").substitute(ctype=self.ctype, name=self.name)


class ClassInfo(GeneralInfo):
    def __init__(self, gen, module, decl, namespaces):  # [ 'class/struct cname', ': base', [modlist] ]
        """
        :type gen: RustWrapperGenerator
        :type module: str
        :type decl: list
        :type namespaces: frozenset|set
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.methods = []  # type: list[FuncInfo]
        self.namespaces = namespaces
        self.module = module
        self.is_simple = self.is_ignored = self.is_ghost = self.is_callback = False
        self.is_trait = self.fullname in forced_class_trait
        self.is_abstract = self.fullname in forced_class_abstract
        self.classname = self.name
        self.comment = ""
        if len(decl) > 5:
            self.comment = decl[5]
        for m in decl[2]:
            if m == "/Simple" or m == "/Map":
                self.is_simple = True
            elif m == "/Hidden":
                self.is_ignored = True
            elif m == "/Ghost":
                self.is_ghost = True
            elif m == "/Callback":
                self.is_callback = True
            elif m == "/A":
                self.is_trait = True
                self.is_abstract = True
        if self.classpath:
            ci = self.gen.get_class(self.classpath)
            if ci is not None and ci.is_ignored:
                self.is_ignored = True

        self.nested_cname = self.fullname.replace("::", "_")

        bases = decl[1][1:].strip()
        if len(bases) > 0:
            self.bases = [x for x in set(x.strip() for x in bases.split(",")) if x != self.fullname]
        else:
            self.bases = []

        self.is_ignored = self.is_ignored or self.gen.class_is_ignored(self.fullname)

        if not self.is_ignored:
            for base in self.bases:
                ci = self.gen.get_class(base)
                if ci:
                    ci.is_trait = True

        # class props
        self.props = []
        for p in decl[3]:
            self.props.append(ClassPropInfo(p))

    def __repr__(self):
        attrs = []
        if self.is_simple:
            attrs.append("simple")
        if self.is_ignored:
            attrs.append("ignored")
        if self.is_ghost:
            attrs.append("ghost")
        if self.is_trait:
            attrs.append("trait")
        if len(attrs) == 0:
            return self.fullname
        else:
            return "{} ({})".format(self.fullname, ", ".join(attrs))

    def add_method(self, fi):
        logging.info("register %s %s in %s (%s)"%(fi.kind, fi.name, fi.ci, fi.identifier))
        self.methods.append(fi)

    def type_info(self):
        """
        :rtype: TypeInfo
        """
        return self.gen.get_type_info(self.fullname)

    def get_manual_declaration_tpl(self, section):
        if self.module in type_manual:
            module_types = type_manual[self.module]
            params = module_types.get(self.name)
            if params is not None:
                tmpl = params.get(section)
                if tmpl is None:
                    return template("")
                elif tmpl == "~":
                    return None
                return tmpl
        return None


class ConstInfo(GeneralInfo):
    TEMPLATES = {
        "rust_string": template("${doc_comment}pub const ${name}: &'static str = ${value};\n"),
        "rust_usize": template("${doc_comment}pub const ${name}: usize = ${value};\n"),
        "rust_int": template("${doc_comment}pub const ${name}: i32 = ${value};\n"),
        "rust_float": template("${doc_comment}pub const ${name}: f64 = ${value};\n"),
        "cpp_string": template("""    printf("pub static ${name}: &'static str = \\"%s\\";\\n", ${full_name});\n"""),
        "cpp_double": template("""    printf("pub const ${name}: f64 = %f;\\n", ${full_name});\n"""),
        "cpp_int": template("""    printf("pub const ${name}: i32 = 0x%x; // %i\\n", ${full_name}, ${full_name});\n"""),
    }

    def __init__(self, gen, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type decl: list
        :type namespaces: frozenset|set
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        _, self.rustname = split_known_namespace(self.fullname, namespaces)
        self.rustname = self.rustname.replace("::", "_")
        self.cname = self.name.replace(".", "::")
        self.value = decl[1]
        self.comment = decl[5] if len(decl) >= 6 else ""

    def __repr__(self):
        return template("CONST $name=$value").substitute(name=self.name, value=self.value)

    def is_ignored(self):
        for c in const_ignore_list:
            if re.match(c, self.name):
                return True
        return False

    def gen_rust(self):
        params = {
            "name": self.rustname,
            "value": self.value,
        }
        while True:
            if self.comment:
                params["doc_comment"] = self.gen.reformat_doc(self.comment)
            else:
                params["doc_comment"] = ""
            m = re.match(r"^(.+?)\s*(?://\s*(.+)|/\*+\s*(.+?)\s*\*+/)$", params["value"])  # xxx // comment OR xxx /** comment */
            if m:
                params["value"] = m.group(1)
                if not self.comment:
                    params["doc_comment"] = self.gen.reformat_doc(m.group(3) if m.group(2) is None else m.group(2))
            if params["value"].startswith('"'):
                return ConstInfo.TEMPLATES["rust_string"].substitute(params)
            elif self.rustname in ("Mat_AUTO_STEP",):
                return ConstInfo.TEMPLATES["rust_usize"].substitute(params)
            elif re.match(r"^(-?[0-9]+|0x[0-9A-Fa-f]+)$", params["value"]):  # decimal or hexadecimal
                return ConstInfo.TEMPLATES["rust_int"].substitute(params)
            elif re.match(r"^\d+\.\d*$", params["value"]):  # float
                return ConstInfo.TEMPLATES["rust_float"].substitute(params)
            elif re.match(r"^\(?\s*(\d+\s*<<\s*\d+)\s*\)?$", params["value"]):  # (1 << 24)
                return ConstInfo.TEMPLATES["rust_int"].substitute(params)
            elif re.match(r"^\s*(\d+\s*\+\s*\d+)\s*$", params["value"]):  # 0 + 3
                return ConstInfo.TEMPLATES["rust_int"].substitute(params)
            elif re.match(r"^CV_MAKE_?TYPE\s*\(", params["value"]):  # CV_MAKETYPE
                return ConstInfo.TEMPLATES["rust_int"].substitute(params)
            ref_const = self.gen.get_const(params["value"])
            if ref_const is not None:
                params["value"] = ref_const.value
                continue
            return None

    def gen_cpp_for_complex(self):
        # only use C-constant dumping for unnested const
        if len(self.fullname.split(".")) > 2:
            return ""
        elif self.fullname == "CV_VERSION":
            return ConstInfo.TEMPLATES["cpp_string"].substitute(name=self.rustname, full_name=self.fullname)
        elif self.fullname in ("MLN10", "RELATIVE_ERROR_FACTOR"):
            return ConstInfo.TEMPLATES["cpp_double"].substitute(name=self.rustname, full_name=self.fullname)
        else:
            return ConstInfo.TEMPLATES["cpp_int"].substitute(name=self.rustname, full_name=self.fullname)


class TypedefInfo(GeneralInfo):
    def __init__(self, gen, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type decl: list
        :type namespaces: frozenset|set
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.alias = decl[1]
        self.comment = ""
        if len(decl) > 5:
            self.comment = decl[5]

    def typ(self):
        return self.gen.get_type_info(self.name)

    def alias_typ(self):
        return self.gen.get_type_info(self.alias)


class CallbackInfo(GeneralInfo):
    TEMPLATES = {
        "rust": template("""
            ${doc_comment}pub type ${name} = dyn FnMut(${args}) + Send + Sync + 'static;
            #[doc(hidden)] pub type ${name}Extern = Option<extern "C" fn(${extern_args})>;
        
        """),
    }

    def __init__(self, gen, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type decl: list
        :type namespaces: frozenset|set
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.args = []
        self.is_ignored = False
        for arg in decl[3]:
            ai = ArgInfo(gen, arg)
            while any(True for x in self.args if x.name == ai.name):
                ai.name = bump_counter(ai.name)
                ai.rsname = camel_case_to_snake_case(reserved_rename.get(ai.name, ai.name))
            if ai.type.is_ignored:
                self.is_ignored = True
            self.args.append(ai)

        if len(decl) > 5:
            self.comment = decl[5]
        else:
            self.comment = ""

    def gen_rust(self):
        args = []
        extern_args = []
        for arg in self.args:
            if arg.type.is_ignored:
                return None
            extern_args.append(arg.type.rust_extern_arg_func_decl(arg.rsname, arg.is_output()))
            if arg.name != "userdata":
                args.append(arg.type.rust_full)
        return CallbackInfo.TEMPLATES["rust"].substitute(combine_dicts(self.__dict__, {
            "doc_comment": self.gen.reformat_doc(self.comment),
            "args": ", ".join(args),
            "extern_args": ", ".join(extern_args),
        }))


class EnumInfo(GeneralInfo):
    TEMPLATES = {
        "rust": template("""
            ${doc_comment}#[repr(C)]
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub enum ${name} {
            ${consts}}
        
        """),
        "rust_const": template("""
            ${doc_comment}${name} = ${rustname} as isize,
        """),
        "rust_const_ignored": template("""
            ${doc_comment}// ${name} = ${rustname} as isize, // ignored discriminant
        """),
    }

    def __init__(self, gen, module, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type module: str
        :type decl: list
        :type namespaces: frozenset|set
        """
        super().__init__(gen, decl[0], namespaces)
        self.module = module
        if self.classname:
            self.name = "{}_{}".format(self.classname, self.name)
        self.consts = []
        self.is_ignored = self.is_ghost = False
        for attr in decl[2]:
            if attr == "/Ghost":
                self.is_ghost = True
        for const_decl in decl[3]:
            ci = ConstInfo(gen, const_decl, namespaces)
            self.consts.append(ci)

        if len(decl) > 5:
            self.comment = decl[5]
        else:
            self.comment = ""

    def gen_rust(self):
        consts = []
        ignore_discriminants = enum_ignore_discriminant.get(self.fullname, set())
        for const in self.consts:
            if const.name in ignore_discriminants:
                doc_comment = self.gen.reformat_doc(const.comment, comment_prefix="//")
                tpl = EnumInfo.TEMPLATES["rust_const_ignored"]
            else:
                doc_comment = self.gen.reformat_doc(const.comment)
                tpl = EnumInfo.TEMPLATES["rust_const"]
            consts.append(tpl.substitute(combine_dicts(const.__dict__, {
                "doc_comment": doc_comment,
            })))
        return EnumInfo.TEMPLATES["rust"].substitute(combine_dicts(self.__dict__, {
            "doc_comment": self.gen.reformat_doc(self.comment),
            "consts": indent("".join(consts)),
        }))


class TypeInfo(object):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        self.is_by_ref = typeid.endswith("&")
        if self.is_by_ref:
            typeid = typeid[:-1].strip()
        self.is_const = typeid.startswith("const ")  # type has C++ const modifier
        self.typeid = typeid.replace("const ", "").strip()  # e.g. "vector<cv::Mat>", "std::vector<int>", "float"
        self.gen = gen
        self.ci = gen.get_class(self.typeid)
        self.is_ignored = False  # don't generate
        # False: types that contain ptr field to actual heap allocated data (e.g. BoxedClass, Vector, SmartPtr)
        # True: types that are getting passed by value (e.g. Primitive, SimpleClass)
        self.is_by_ptr = False
        self.is_copy = False  # true for types that are Copy in Rust (e.g. Primitive, SimpleClass)

        self.cpp_extern = ""  # cpp type used on the boundary between Rust and C (e.g. in return wrappers)
        self.cpptype = self.typeid
        self.c_safe_id = "XX"  # c safe type identifier used for file names and return wrappers

        _, self.rust_local = split_known_namespace(self.typeid, gen.namespaces)
        self.rust_local = self.rust_local.replace("::", "_")  # only the type name for Rust without module path
        self.rust_safe_id = self.rust_local  # rust safe type identifier used for file and function names
        self.rust_full = self.rust_local  # full module path (with modules/crate::) to Rust type
        self.rust_extern = ""  # Rust type used on the boundary between Rust and C (e.g. in return wrappers)
        # self.rust_lifetimes = ""  # for reference types it's the definition of lifetimes that need to be present in function

        self.inner = None  # type: TypeInfo  # inner type for container types

        self.base_templates = {
            "cpp_void": template("""
                // $typeid
                struct ${return_wrapper_type} {
                    int error_code;
                    char* error_msg;
                };
            """),

            "cpp_non_void": template("""
                // $typeid
                struct ${return_wrapper_type} {
                    int error_code;
                    char* error_msg;
                    ${cpp_extern} result;
                };
            """),

            "rust_void": template("""
                pub type ${return_wrapper_type} = cv_return_value<crate::types::Unit, ${rust_extern}>;
            """),

            "rust_non_void": template("""
                pub type ${return_wrapper_type} = cv_return_value<${rust_extern}>;
            """),
        }

    def gen_wrappers(self):
        pass

    def gen_return_wrappers(self, cpp_dir, rust_dir):
        """
        :type cpp_dir: str
        :type rust_dir: str
        """
        template_vars = combine_dicts(self.__dict__, {
            "return_wrapper_type": self.rust_cpp_return_wrapper_type(),
        })
        write_exc(
            "{}/{}.type.h".format(cpp_dir, template_vars["return_wrapper_type"]),
            lambda f: f.write(self.base_templates["cpp_void" if self.cpp_extern == "void" else "cpp_non_void"].substitute(template_vars))
        )
        if self.rust_extern in ("*mut c_void", "*const c_void"):
            # otherwise it leads to duplication of type definitions
            rust_module = "core"
        else:
            rust_module = self.rust_module()
        write_exc(
            "{}/{}-{}.rv.rs".format(rust_dir, rust_module, template_vars["return_wrapper_type"]),
            lambda f: f.write(self.base_templates["rust_void" if self.cpp_extern == "void" else "rust_non_void"].substitute(template_vars))
        )

    def is_output_ref(self):
        return self.is_by_ref and not self.is_const

    def rust_trait_local(self):
        out = self.rust_local
        if self.ci and self.ci.is_trait and not self.ci.is_abstract:
            out += "Trait"
        return out

    def rust_trait_full(self):
        out = self.rust_full
        if self.ci and self.ci.is_trait and not self.ci.is_abstract:
            out += "Trait"
        return out

    def rust_module(self):
        if self.inner is not None:
            if self.inner.inner is not None and self.inner.inner.ci is not None:
                return self.inner.inner.ci.module
            elif self.inner.ci is not None:
                return self.inner.ci.module
        elif self.ci is not None:
            return self.ci.module
        return "core"

    def rust_generic_decl(self):
        return ""

    def rust_self_func_decl(self, is_output=False):
        """
        :type is_output: bool
        :rtype: str
        """
        if self.is_by_ptr:
            if is_output:
                return "&mut self"
            return "&self"
        return "self"

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        """
        :type var_name: str
        :type is_output: bool
        :type attr_type: str|None
        :rtype: str
        """
        rust_full = self.rust_full
        if (self.is_by_ptr or self.is_by_ref) and attr_type is None:
            mods = []
            if self.is_by_ptr and is_output or self.is_output_ref():
                mods.append("mut")
            ci = self.ci
            if not ci and isinstance(self, RawPtrTypeInfo):
                ci = self.inner.ci
            if ci and ci.is_trait:
                if not ci.is_abstract and ci.fullname not in ("cv::_InputArray", "cv::_OutputArray", "cv::_InputOutputArray"):
                    rust_full += "Trait"
                mods.append("dyn")
            if len(mods) > 0:
                mods.append("")  # force space at the end
            rust_full = "&{}{}".format(" ".join(mods), rust_full)
        return "{}: {}".format(var_name, rust_full)

    def rust_arg_pre_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return ""

    def rust_self_func_call(self, is_output=False):
        """
        :type is_output: bool
        :rtype: str
        """
        return self.rust_arg_func_call("self", is_output)

    def rust_arg_func_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        if self.is_by_ptr:
            # if isinstance(a.type, RawPtrTypeInfo):
            #     typ = a.type.inner
            # else:
            #     typ = a.type
            return "{}.as_raw_{}()".format(var_name, self.rust_local)
        return var_name

    def rust_arg_forward(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return var_name

    def rust_extern_self_func_decl(self, is_output=False):
        """
        :type is_output: bool
        :rtype: str
        """
        typ = self.rust_full
        if self.is_by_ptr:
            typ = "*{} c_void".format("mut" if is_output else "const")
        return "instance: {}".format(typ)

    def rust_extern_arg_func_decl(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        typ = self.rust_extern
        if self.is_by_ref and not self.is_by_ptr:
            typ = "*{} {}".format("const" if self.is_const else "mut", self.rust_extern)
        return "{}: {}".format(var_name, typ)

    def rust_arg_post_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return ""

    def rust_cpp_return_wrapper_type(self):
        """
        :rtype: str
        """
        return "cv_return_value_{}".format(self.c_safe_id)

    def cpp_arg_func_decl(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        if not self.is_by_ptr:
            if self.is_by_ref:
                return "{}{}& {}".format("const " if self.is_const else "", self.cpp_extern, var_name)
            elif is_output:
                return "{}* {}".format(self.cpp_extern, var_name)
        return "{} {}".format(self.cpp_extern, var_name)

    def cpp_arg_pre_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return ""

    def cpp_arg_func_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        const = "const " if self.is_const else ""
        if self.is_by_ptr:
            return "*reinterpret_cast<{}{}*>({})".format(const, self.cpptype, var_name)
        return "*reinterpret_cast<{}{}*>(&{})".format(const, self.cpptype, var_name)

    def cpp_arg_post_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return ""

    def cpp_method_call_name(self, method_name):
        """
        :type method_name: str
        :rtype: str
        """
        return "reinterpret_cast<{}*>(&instance)->{}".format(self.cpptype, method_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor, attr_type):
        """
        :type call_name: str
        :type call_args: str
        :type is_constructor: bool
        :type attr_type: str|None
        :rtype: str
        """
        if is_constructor:
            if call_args == "":
                return "{} ret;".format(self.cpptype)
            return "{} ret({});".format(self.cpptype, call_args)
        if attr_type == "r":
            return "{} ret = {};".format(self.cpptype, call_name)
        elif attr_type == "w":
            return "{} = {};".format(call_name.replace("set_", ""), call_args)
        return "{} ret = {}({});".format(self.cpptype, call_name, call_args)

    def cpp_method_return(self, is_constructor):
        if self.is_by_ptr:
            if is_constructor:
                return "return { Error::Code::StsOk, NULL, ret };"
            else:
                return "return {{ Error::Code::StsOk, NULL, new {}(ret) }};".format(self.cpptype)
        return "return {{ Error::Code::StsOk, NULL, ret }};".format(self.cpp_extern)


class StringTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        self.is_std = "std::string" in typeid
        self.cpp_extern = "const char*"
        self.cpptype = "String"
        self.rust_full = "String"
        if self.is_const:
            self.c_safe_id = "const_char_X"
            self.rust_extern = "*const c_char"
        else:
            self.c_safe_id = "char_X"
            self.rust_extern = "*mut c_char"

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        if self.is_output_ref():
            return "{}: &mut String".format(var_name)
        return "{}: &str".format(var_name)

    def rust_arg_pre_call(self, var_name, is_output=False):
        if self.is_output_ref():
            return "string_arg_output_send!(via {}_via)".format(var_name)
        return "string_arg!({}{})".format("" if self.is_const else "mut ", var_name)

    def rust_arg_func_call(self, var_name, is_output=False):
        if self.is_output_ref():
            return "&mut {}_via".format(var_name)
        if self.is_const:
            return "{}.as_ptr()".format(var_name)
        return "{}.as_ptr() as _".format(var_name)  # fixme: use as_mut_ptr() when it's stabilized

    def rust_extern_arg_func_decl(self, var_name, is_output=False):
        if self.is_by_ref and self.is_const:
            return "{}: {}".format(var_name, self.rust_extern)
        return super().rust_extern_arg_func_decl(var_name, is_output)

    def rust_arg_post_call(self, var_name, is_output=False):
        if self.is_output_ref():
            return "string_arg_output_receive!({}_via => {})".format(var_name, var_name)
        return super().rust_arg_post_call(var_name, is_output)

    def cpp_arg_func_decl(self, var_name, is_output=False):
        if self.is_by_ref:
            if self.is_const:
                return "{} {}".format(self.cpp_extern, var_name)
            else:
                return "{}* {}".format(self.cpp_extern, var_name)
        return super().cpp_arg_func_decl(var_name, is_output)

    def cpp_arg_pre_call(self, var_name, is_output=False):
        if self.is_output_ref():
            if self.is_std:
                return "std::string {}_out".format(var_name)
            else:
                return "cv::String {}_out".format(var_name)
        return super().cpp_arg_pre_call(var_name, is_output)

    def cpp_arg_func_call(self, var_name, is_output=False):
        if self.is_output_ref():
            return "{}_out".format(var_name)
        return "{}({})".format(self.cpptype, var_name)

    def cpp_arg_post_call(self, var_name, is_output=False):
        if self.is_output_ref():
            return "*{} = strdup({}_out.c_str())".format(var_name, var_name)
        return super().cpp_arg_post_call(var_name, is_output)

    def cpp_method_return(self, is_constructor):
        return "return { Error::Code::StsOk, NULL, strdup(ret.c_str()) };"

    def __str__(self):
        return "string"


class IgnoredTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        self.is_ignored = True

    def __str__(self):
        return "Ignored(%s)"%(self.typeid)


class PrimitiveTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        primitive = primitives[self.typeid]
        self.cpp_extern = primitive["cpp_extern"]
        self.rust_extern = self.rust_full = self.rust_local = primitive["rust_local"]
        self.rust_safe_id = self.typeid.replace(" ", "_")
        self.c_safe_id = make_safe_id(self.cpp_extern)
        self.is_copy = True

    def cpp_arg_func_call(self, var_name, is_output=False):
        return var_name

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor, attr_type):
        out = super().cpp_method_call_invoke(call_name, call_args, is_constructor, attr_type)
        if self.cpptype == "void":
            out = re.sub("^.+ ret = ", "", out)
        return out

    def cpp_method_return(self, is_constructor):
        if self.cpptype == "void":
            return "return { Error::Code::StsOk, NULL };"
        return super().cpp_method_return(is_constructor)

    def __str__(self):
        return "Primitive(%s)" % (self.cpptype)


class SimpleClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        if self.ci:
            self.is_ignored = self.ci.is_ignored
            self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
            if self.ci.get_manual_declaration_tpl("rust") is None:
                self.cpp_extern = self.ci.fullname
                self.c_safe_id = self.rust_local
            else:
                self.cpp_extern = "{}Wrapper".format(self.ci.name)
                self.c_safe_id = self.cpp_extern
            self.rust_extern = self.rust_full
            self.is_copy = True

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        # const references to simple classes are passed by value for performance
        # fixme: it kind of works now, but probably it's not the best idea
        #  because some functions potentially can save the pointer to the value, but it will be destroyed after function call
        if self.is_by_ref and self.is_const:
            return "{}: {}".format(var_name, self.rust_full)
        return super().rust_arg_func_decl(var_name, is_output, attr_type)

    def rust_extern_arg_func_decl(self, var_name, is_output=False):
        # fixme, see comment in rust_arg_func_decl()
        if self.is_by_ref and self.is_const:
            return "{}: {}".format(var_name, self.rust_extern)
        return super().rust_extern_arg_func_decl(var_name, is_output)

    def cpp_arg_func_decl(self, var_name, is_output=False):
        # fixme, see comment in rust_arg_func_decl()
        if self.is_by_ref and self.is_const:
            return "{} {}".format(self.cpp_extern, var_name)
        return super().cpp_arg_func_decl(var_name, is_output)

    def cpp_method_return(self, is_constructor):
        if self.cpp_extern.endswith("Wrapper"):
            return "return {{ Error::Code::StsOk, NULL, *reinterpret_cast<{}*>(&ret) }};".format(self.cpp_extern)
        return super().cpp_method_return(is_constructor)

    def __str__(self):
        return "%s (simple)"%(self.cpptype)


class CallbackTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        if self.ci:
            self.is_ignored = self.ci.is_ignored
            self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
            self.cpp_extern = self.ci.fullname
            self.c_safe_id = self.rust_local
            self.rust_extern = "{}Extern".format(self.rust_full)

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        callback_info = self.gen.get_callback(self.typeid)
        if callback_info is None or callback_info.is_ignored:
            return super().rust_arg_func_decl(var_name, is_output, attr_type)
        return "{}: Option<Box<{}>>".format(var_name, self.rust_full)

    def rust_arg_pre_call(self, var_name, is_output=False):
        callback_info = self.gen.get_callback(self.typeid)
        if callback_info is None or callback_info.is_ignored:
            return super().rust_generic_decl()
        extern_args = []
        rust_args = []
        for arg in callback_info.args:
            extern_args.append(arg.type.rust_extern_arg_func_decl(arg.rsname, arg.is_output()))
            if arg.name != "userdata":
                rust_args.append(arg.type.rust_arg_func_decl(arg.rsname, arg.is_output()))
        return "callback_arg!({}({}) via userdata => ({}))".format(var_name, ", ".join(extern_args), ", ".join(rust_args))

    def __str__(self):
        return "{} (callback)".format(self.cpptype)


class EnumTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        self.ei = gen.get_enum(self.typeid)
        if self.ei:
            self.is_ignored = self.ei.is_ignored
            self.rust_full = ("crate::" if self.ei.module not in static_modules else "") + self.ei.module + "::" + self.rust_local
            self.cpptype = self.ei.short_fullname
            self.cpp_extern = self.cpptype
            self.c_safe_id = make_safe_id(self.cpp_extern)
            self.rust_extern = self.rust_full
            self.is_copy = True

    def __str__(self):
        return "{} (enum)".format(self.cpptype)


class BoxedClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        self.cpptype = self.ci.fullname
        self.rust_extern = "*mut c_void"
        self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
        self.is_by_ptr = True
        self.cpp_extern = "void*"
        self.c_safe_id = "void_X"
        self.is_ignored = self.ci.is_ignored
        self.rust_safe_id = self.ci.name

    def is_input_array(self):
        return self.cpptype == "cv::_InputArray"

    def is_output_array(self):
        return self.cpptype == "cv::_OutputArray"

    def is_input_output_array(self):
        return self.cpptype == "cv::_InputOutputArray"

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        old_rust_full = self.rust_full
        if self.is_input_array():
            self.rust_full = "core::ToInputArray"
        elif self.is_output_array():
            self.rust_full = "core::ToOutputArray"
        elif self.is_input_output_array():
            self.rust_full = "core::ToInputOutputArray"
        out = super().rust_arg_func_decl(var_name, is_output, attr_type)
        self.rust_full = old_rust_full
        return out

    def rust_arg_pre_call(self, var_name, is_output=False):
        if self.is_input_array():
            return "input_array_arg!({})".format(var_name)
        elif self.is_output_array():
            return "output_array_arg!({})".format(var_name)
        elif self.is_input_output_array():
            return "input_output_array_arg!({})".format(var_name)
        return super().rust_arg_pre_call(var_name, is_output)

    def cpp_method_call_name(self, method_name):
        return "reinterpret_cast<{}*>(instance)->{}".format(self.cpptype, method_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor, attr_type):
        if is_constructor:
            return "{}* ret = new {}({});".format(self.cpptype, call_name, call_args)
        return super().cpp_method_call_invoke(call_name, call_args, is_constructor, attr_type)

    def __str__(self):
        return "%s (boxed)"%(self.typeid)


class VectorTypeInfo(TypeInfo):
    TEMPLATES = {
        "rust_common": template("""
            pub struct ${rust_local} {
                pub(crate) ptr: ${rust_extern}
            }
            
            impl ${rust_local} {
                #[inline(always)] pub fn as_raw_${rust_local}(&self) -> ${rust_extern} { self.ptr }
                
                #[inline]
                pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
                    crate::templ::VectorRefIterator::new(self)
                }
            ${inherent_methods}}
            
            impl Drop for ${rust_local} {
                #[inline]
                fn drop(&mut self) {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "${cpptype}*"] {
                        delete vec;
                    })
                }
            }
            
            impl IntoIterator for ${rust_local} {
                type Item = ${inner_rust_full};
                type IntoIter = crate::templ::VectorIterator<Self>;
            
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter::new(self)
                }
            }

            impl<'i> IntoIterator for &'i ${rust_local} {
                type Item = ${inner_rust_full};
                type IntoIter = crate::templ::VectorRefIterator<'i, ${rust_local}>;
            
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            impl<'i> crate::templ::Vector<'i> for ${rust_local} {
                type Storage = ${inner_rust_full};

                #[inline]
                fn new() -> Self {
                    Self { ptr: cpp!(unsafe [] -> ${rust_extern} as "${cpp_extern}" {
                        return new ${cpptype}();
                    })}
                }
                
                #[inline]
                fn len(&self) -> size_t {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "const ${cpptype}*"] -> size_t as "size_t" {
                        return vec->size();
                    })
                }

                #[inline]
                fn is_empty(&self) -> bool {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "const ${cpptype}*"] -> bool as "bool" {
                        return vec->empty();
                    })
                }

                #[inline]
                fn capacity(&self) -> size_t {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "const ${cpptype}*"] -> size_t as "size_t" {
                        return vec->capacity();
                    })
                }
                
                #[inline]
                fn shrink_to_fit(&mut self) {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "${cpptype}*"] {
                        vec->shrink_to_fit();
                    })
                }                

                #[inline]
                fn reserve(&mut self, additional: size_t) {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "${cpptype}*", additional as "size_t"] {
                        vec->reserve(vec->size() + additional);
                    })
                }
                
                #[inline]
                fn remove(&mut self, index: size_t) -> Result<()> {
                    crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "${cpptype}*", index as "size_t"] {
                        vec->erase(vec->begin() + index);
                    });
                    Ok(())
                }
                
                #[inline]
                fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
                    let len = self.len();
                    crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
                    crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
                    if index1 != index2 {
                        let vec = self.as_raw_${rust_local}();
                        cpp!(unsafe [vec as "${cpptype}*", index1 as "size_t", index2 as "size_t"] {
                            swap((*vec)[index1], (*vec)[index2]);
                        });
                    }
                    Ok(())
                }
                
                #[inline]
                fn clear(&mut self) {
                    let vec = self.as_raw_${rust_local}();
                    cpp!(unsafe [vec as "${cpptype}*"] {
                        vec->clear();
                    })
                }
                
            ${vector_methods}}
            
            unsafe impl Send for ${rust_local} {}
            ${impls}
        """),

        "rust_methods_boxed": template("""
            type Arg = ${inner_rust_full};

            #[inline]
            fn push(&mut self, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                let val = val.as_raw_${inner_rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", val as "${inner_cpptype}*"] {
                    vec->push_back(*val);
                })
            }
            
            #[inline]
            fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
                let vec = self.as_raw_${rust_local}();
                let val = val.as_raw_${inner_rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}*"] {
                    vec->insert(vec->begin() + index, *val);
                });
                Ok(())
            }
            
            #[inline]
            fn get(&self, index: size_t) -> Result<Self::Storage> {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "const ${cpptype}*", index as "size_t"] -> crate::sys::${return_wrapper_type} as "${return_wrapper_type}" {
                    try {
                        return { Error::Code::StsOk, NULL, new ${inner_cpptype}(vec->at(index)) };
                    } VEC_CATCH(${return_wrapper_type})
                }).into_result().map(|ptr| ${inner_rust_full} { ptr })
            }

            #[inline]
            unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
                let vec = self.as_raw_${rust_local}();
                ${inner_rust_full} { ptr: cpp!(unsafe [vec as "const ${cpptype}*", index as "size_t"] -> ${rust_extern} as "${cpp_extern}" {
                    return new ${inner_cpptype}((*vec)[index]);
                })}
            }
            
            #[inline]
            fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                let vec = self.as_raw_${rust_local}();
                let val = val.ptr;
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                    try {
                        vec->at(index) = *val;
                        return { Error::Code::StsOk, NULL };
                    } VEC_CATCH(cv_return_value_void)
                }).into_result()
            }
            
            #[inline]
            unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                let val = val.ptr;
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}*"] {
                    (*vec)[index] = *val;
                })
            }
        """),

        "rust_methods_string": template("""
            type Arg = &'i str;
            
            #[inline]
            fn push(&mut self, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                let val = ::std::ffi::CString::new(val).unwrap();
                let val = val.as_ptr();
                cpp!(unsafe [vec as "${cpptype}*", val as "${inner_cpp_extern}"] {
                    vec->push_back(String(val));
                })
            }
            
            #[inline]
            fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
                let vec = self.as_raw_${rust_local}();
                let val = ::std::ffi::CString::new(val).unwrap();
                let val = val.as_ptr();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpp_extern}"] {
                    vec->insert(vec->begin() + index, String(val));
                });
                Ok(())
            }
            
            #[inline]
            fn get(&self, index: size_t) -> Result<Self::Storage> {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "const ${cpptype}*", index as "size_t"] -> crate::sys::${return_wrapper_type} as "${return_wrapper_type}" {
                    try {
                        return { Error::Code::StsOk, NULL, vec->at(index).c_str() };
                    } VEC_CATCH(${return_wrapper_type})
                }).into_result().map(|x| unsafe { ::std::ffi::CStr::from_ptr(x) }.to_string_lossy().into_owned())
            }

            #[inline]
            unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
                let vec = self.as_raw_${rust_local}();
                ::std::ffi::CStr::from_ptr(cpp!(unsafe [vec as "const ${cpptype}*", index as "size_t"] -> ${inner_rust_extern} as "${inner_cpp_extern}" {
                    return (*vec)[index].c_str();
                })).to_string_lossy().into_owned()
            }

            #[inline]
            fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                let vec = self.as_raw_${rust_local}();
                let val = ::std::ffi::CString::new(val).unwrap();
                let val = val.as_ptr();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpp_extern}"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                    try {
                        vec->at(index) = String(val);
                        return { Error::Code::StsOk, NULL };
                    } VEC_CATCH(cv_return_value_void)
                }).into_result()
            }

            #[inline]
            unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                let val = ::std::ffi::CString::new(val).unwrap();
                let val = val.as_ptr();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpp_extern}"] {
                    (*vec)[index] = String(val);
                })
            }
        """),

        "rust_methods_non_boxed": template("""
            type Arg = ${inner_rust_full};

            #[inline]
            fn push(&mut self, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", val as "${inner_cpptype}"] {
                    vec->push_back(val);
                })
            }
            
            #[inline]
            fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}"] {
                    vec->insert(vec->begin() + index, val);
                });
                Ok(())
            }
            
            #[inline]
            fn get(&self, index: size_t) -> Result<Self::Storage> {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "const std::vector<${inner_cpp_extern}>*", index as "size_t"] -> crate::sys::${return_wrapper_type} as "${return_wrapper_type}" {
                    try {
                        return { Error::Code::StsOk, NULL, vec->at(index) };
                    } VEC_CATCH(${return_wrapper_type})
                }).into_result()
            }

            #[inline]
            unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "const std::vector<${inner_cpp_extern}>*", index as "size_t"] -> ${inner_rust_extern} as "${inner_cpp_extern}" {
                    return (*vec)[index];
                })
            }

            #[inline]
            fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                    try {
                        vec->at(index) = val;
                        return { Error::Code::StsOk, NULL };
                    } VEC_CATCH(cv_return_value_void)
                }).into_result()
            }

            #[inline]
            unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
                let vec = self.as_raw_${rust_local}();
                cpp!(unsafe [vec as "${cpptype}*", index as "size_t", val as "${inner_cpptype}"] {
                    (*vec)[index] = val;
                })
            }
        """),

        "rust_methods_copy_non_bool": template("""

            #[inline]
            fn to_vec(&self) -> Vec<Self::Storage> {
                self.to_slice().to_vec()
            }
        """),

        "rust_inherent_copy_non_bool": template("""

            pub fn to_slice(&self) -> &[${inner_rust_full}] {
                unsafe {
                    let vec = self.as_raw_${rust_local}();
                    let data = cpp!(unsafe [vec as "${cpptype}*"] -> *const ${inner_rust_full} as "${cpp_extern}*" {
                        return reinterpret_cast<${cpp_extern}*>(vec->data());
                    });
                    ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
                }
            }
        """),

        "input_output_array": template("""

            impl core::ToInputArray for ${rust_local} {
                #[inline]
                fn input_array(&self) -> Result<core::_InputArray> {
                    let me = self.as_raw_${rust_local}();
                    cpp!(unsafe [me as "${cpptype}*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                        try {
                            return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                        } CVRS_CATCH(cv_return_value_const_void_X)
                    }).into_result()
                        .map(|ptr| core::_InputArray { ptr })
                }
            }

            impl core::ToInputArray for &${rust_local} {
                #[inline]
                fn input_array(&self) -> Result<core::_InputArray> {
                    (*self).input_array()
                }
            }

            impl core::ToOutputArray for ${rust_local} {
                #[inline]
                fn output_array(&mut self) -> Result<core::_OutputArray> {
                    let me = self.as_raw_${rust_local}();
                    cpp!(unsafe [me as "${cpptype}*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                        try {
                            return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                        } CVRS_CATCH(cv_return_value_const_void_X)
                    }).into_result()
                        .map(|ptr| core::_OutputArray { ptr })
                }
            }

            impl core::ToOutputArray for &mut ${rust_local} {
                #[inline]
                fn output_array(&mut self) -> Result<core::_OutputArray> {
                    (*self).output_array()
                }
            }

            impl core::ToInputOutputArray for ${rust_local} {
                #[inline]
                fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
                    let me = self.as_raw_${rust_local}();
                    cpp!(unsafe [me as "${cpptype}*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                        try {
                            return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                        } CVRS_CATCH(cv_return_value_const_void_X)
                    }).into_result()
                        .map(|ptr| core::_InputOutputArray { ptr })
                }
            }

            impl core::ToInputOutputArray for &mut ${rust_local} {
                #[inline]
                fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
                    (*self).input_output_array()
                }
            }
        """)
    }

    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        super().__init__(gen, typeid)
        self.is_by_ptr = True
        self.inner = inner
        if isinstance(self.inner, RawPtrTypeInfo):  # fixme, lifetimes required
            self.is_ignored = True
        else:
            self.is_ignored = inner.is_ignored
        if not self.is_ignored:
            self.cpp_extern = "void*"
            self.c_safe_id = "void_X"
            self.cpptype = "std::vector<%s>" % (inner.cpptype)
            self.rust_safe_id = self.rust_local = "VectorOf" + inner.rust_safe_id
            self.rust_full = "types::" + self.rust_local
            self.rust_extern = "*mut c_void"

    def gen_wrappers(self):
        template_vars = combine_dicts(self.__dict__, {
            "inner_cpptype": self.inner.cpptype,
            "inner_cpp_extern": self.inner.cpp_extern,
            "inner_rust_extern": self.inner.rust_extern,
            "inner_rust_local": self.inner.rust_local,
            "inner_rust_full": self.inner.rust_full,
            "return_wrapper_type": self.inner.rust_cpp_return_wrapper_type(),
        })
        vector_methods = ""
        inherent_methods = ""
        impls = ""
        if self.inner.is_by_ptr:
            vector_methods += VectorTypeInfo.TEMPLATES["rust_methods_boxed"].substitute(template_vars)
        elif isinstance(self.inner, StringTypeInfo):
            vector_methods += VectorTypeInfo.TEMPLATES["rust_methods_string"].substitute(template_vars)
        else:
            vector_methods += VectorTypeInfo.TEMPLATES["rust_methods_non_boxed"].substitute(template_vars)
            if self.inner.is_copy and self.inner.typeid != "bool":
                vector_methods += VectorTypeInfo.TEMPLATES["rust_methods_copy_non_bool"].substitute(template_vars)
                inherent_methods += VectorTypeInfo.TEMPLATES["rust_inherent_copy_non_bool"].substitute(template_vars)
        if self.inner.typeid in data_type_typeids or isinstance(self.inner, VectorTypeInfo) and self.inner.inner.typeid in data_type_typeids:
            # if "inner" not in self.inner.__dict__ or (self.inner.inner is not None and self.inner.inner.typeid != "bool"):
            impls += VectorTypeInfo.TEMPLATES["input_output_array"].substitute(template_vars)

        self.inner.gen_return_wrappers(self.gen.cpp_dir, self.gen.rust_dir)
        write_exc("{}/{}-{}.type.rs".format(self.gen.rust_dir, self.rust_module(), self.rust_safe_id), lambda f: f.write(VectorTypeInfo.TEMPLATES["rust_common"].substitute(combine_dicts(template_vars, {
            "vector_methods": indent(vector_methods),
            "inherent_methods": indent(inherent_methods),
            "impls": impls,
        }))))

    def __str__(self):
        return "Vector[%s]" % (self.inner)


class SmartPtrTypeInfo(TypeInfo):
    TEMPLATES = {
        "rust": template("""
            pub struct ${rust_local} {
                pub(crate) ptr: ${rust_extern}
            }

            impl ${rust_local} {
                #[inline(always)] pub fn as_raw_${rust_safe_id}(&self) -> ${rust_extern} { self.ptr }
                
                pub unsafe fn from_raw_ptr(ptr: ${rust_extern}) -> Self {
                    Self { ptr }
                }
            }

            impl Drop for ${rust_local} {
                fn drop(&mut self) {
                    let me = self.ptr;
                    cpp!(unsafe [me as "${cpptype}*"] {
                        delete me;
                    })
                }
            }
            
            unsafe impl Send for ${rust_local} {}
            
        """),

        "rust_trait_cast": template("""
            impl ${base_rust_full} for ${rust_local} {
                #[inline(always)] fn as_raw_${base_rust_local}(&self) -> ${rust_extern} {
                    let me = self.ptr;
                    cpp!(unsafe [me as "cv::Ptr<${base_cpp_type}>*"] -> ${rust_extern} as "${cpp_extern}" {
                        return me->get();
                    })
                }
            }
            
        """),

        "rust_deref": template("""
            pub struct ${rust_deref_guard}<'a> {
                guarded: &'a ${rust_local},
                inner: std::mem::ManuallyDrop<${inner_rust_full}>,
            }

            impl<'a> std::ops::Deref for ${rust_deref_guard}<'a> {
                type Target = ${inner_rust_full};

                fn deref(&self) -> &Self::Target {
                    self.inner.deref()
                }
            }

            impl<'a> std::ops::DerefMut for ${rust_deref_guard}<'a> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.inner.deref_mut()
                }
            }

            impl ${rust_local} {
                pub fn deref_mut<'a>(&'a mut self) -> ${rust_deref_guard}<'a> {
                    let me = self.ptr;
                    let inner_ptr = cpp!(unsafe [me as "cv::Ptr<${inner_cpp_type}>*"] -> ${rust_extern} as "${cpp_extern}" {
                        return me->get();
                    });
                    let inner = ${inner_rust_full} { ptr: inner_ptr };
                    ${rust_deref_guard} {
                        guarded: self,
                        inner: std::mem::ManuallyDrop::new(inner),
                    }
                }
            }
        """),
    }

    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        super().__init__(gen, typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = self.inner.is_ignored
        if not self.is_ignored:
            self.cpp_extern = "void*"
            self.c_safe_id = "void_X"
            self.rust_extern = "*mut c_void"
            self.cpptype = "Ptr<{}>".format(self.inner.cpptype)
            self.rust_local = self.rust_safe_id = "PtrOf{}".format(inner.rust_safe_id)
            self.rust_full = "types::{}".format(self.rust_local)

    def gen_wrappers(self):
        def write_rust(f):
            f.write(SmartPtrTypeInfo.TEMPLATES["rust"].substitute(self.__dict__))
            if not isinstance(self.inner, PrimitiveTypeInfo):
                if self.inner.ci.is_trait:
                    bases = self.gen.all_bases(self.inner.ci.name).union({self.inner.typeid})
                    for base in sorted(bases):
                        cibase = self.gen.get_type_info(base)
                        if not isinstance(cibase, UnknownTypeInfo):
                            f.write(SmartPtrTypeInfo.TEMPLATES["rust_trait_cast"].substitute(combine_dicts(self.__dict__ , {
                                "base_rust_local": cibase.rust_local,
                                "base_rust_full": cibase.rust_trait_full(),
                                "base_cpp_type": cibase.cpptype,
                            })))
                else:
                    rust_deref_guard = "{}Guard".format(self.rust_local)
                    f.write(SmartPtrTypeInfo.TEMPLATES["rust_deref"].substitute(combine_dicts(self.__dict__ , {
                        "inner_rust_full": self.inner.rust_full,
                        "inner_cpp_type": self.inner.cpptype,
                        "rust_deref_guard": rust_deref_guard,
                    })))

        write_exc("{}/{}-{}.type.rs".format(self.gen.rust_dir, self.rust_module(), self.rust_safe_id), write_rust)

    def __str__(self):
        return "SmartPtr[%s]" % (self.inner)


class RawPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        super().__init__(gen, typeid)
        self.inner = inner
        self.is_slice = self.typeid.endswith("[]")
        self.is_string = isinstance(self.inner, PrimitiveTypeInfo) and self.inner.cpptype == "char"
        if self.inner.is_ignored or isinstance(self.inner, RawPtrTypeInfo):  # fixme double pointer
            self.is_ignored = True
        else:
            if self.inner.is_by_ptr:
                self.is_by_ptr = self.inner.is_by_ptr
                self.c_safe_id = self.inner.c_safe_id
                self.cpptype = self.inner.cpptype
                self.cpp_extern = self.inner.cpp_extern
                self.rust_safe_id = self.inner.rust_safe_id
                self.rust_local = self.inner.rust_local
                self.rust_full = self.inner.rust_full
                self.rust_extern = self.inner.rust_extern
            else:
                # self.rust_lifetimes = "'b"
                self.rust_safe_id = self.inner.rust_safe_id + "_X"
                self.c_safe_id = self.inner.c_safe_id + "_X"
                self.cpptype = self.inner.cpptype + "*"
                self.cpp_extern = self.inner.cpptype + "*"
                # self.is_by_ptr = True
                self.rust_full = "&"
                self.rust_extern = "*"
                if self.is_const:
                    self.rust_extern += "const "
                else:
                    self.rust_full += "mut "
                    self.rust_extern += "mut "
                if isinstance(self.inner, PrimitiveTypeInfo) and self.inner.cpptype == "void":
                    self.rust_full += "c_void"
                    self.rust_extern += "c_void"
                elif self.is_string:
                    self.rust_full = "String"
                    self.rust_extern += "c_char"
                else:
                    if self.is_slice:
                        self.rust_full += "[{}]".format(self.inner.rust_full)
                    else:
                        self.rust_full += self.inner.rust_full
                    self.rust_extern += self.inner.rust_extern
            if self.is_const:
                self.c_safe_id = "const_" + self.c_safe_id
                self.cpptype = "const " + self.cpptype
                self.cpp_extern = "const " + self.cpp_extern
                self.rust_safe_id = "const_" + self.rust_safe_id

    def rust_arg_func_decl(self, var_name, is_output=False, attr_type=None):
        if self.is_string:
            return "{}: &{}str".format(var_name, "mut " if is_output else "")
        return super().rust_arg_func_decl(var_name, is_output or not self.is_const, attr_type)

    def rust_arg_pre_call(self, var_name, is_output=False):
        if self.is_string:
            return "string_arg!({})".format(var_name)
        return super().rust_arg_pre_call(var_name, is_output)

    def rust_arg_func_call(self, var_name, is_output=False):
        if self.is_string:
            if self.is_const:
                return "{}.as_ptr()".format(var_name)
            return "{}.as_ptr() as _".format(var_name)  # fixme: use as_mut_ptr() when it's stabilized
        elif self.is_slice:
            if self.is_const:
                return "{}.as_ptr()".format(var_name)
            return "{}.as_mut_ptr()".format(var_name)
        return super().rust_arg_func_call(var_name, is_output)

    def cpp_arg_func_call(self, var_name, is_output=False):
        if isinstance(self.inner, PrimitiveTypeInfo):
            return self.inner.cpp_arg_func_call(var_name, is_output)
        if self.is_by_ptr:
            return "reinterpret_cast<{}*>({})".format(self.cpptype, var_name)
        else:
            return "reinterpret_cast<{}>({})".format(self.cpptype, var_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor, attr_type):
        if self.is_by_ptr:
            return "{}* ret = {}({});".format(self.cpptype, call_name, call_args)
        return super().cpp_method_call_invoke(call_name, call_args, is_constructor, attr_type)

    def cpp_method_return(self, is_constructor):
        if self.is_string:
            return "return { Error::Code::StsOk, NULL, strdup(ret) };"
        elif self.is_by_ptr:
            return "return {{ Error::Code::StsOk, NULL, new {}(*ret) }};".format(self.cpptype)
        return super().cpp_method_return(is_constructor)

    def __str__(self):
        return "RawPtr[%s]" % (self.inner)


class UnknownTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super().__init__(gen, typeid)
        self.is_ignored = True
        logging.info("Registering an unknown type: %s", self.typeid)

    def __str__(self):
        return "Unknown[%s]" % (self.typeid)


def parse_type(gen, typeid):
    """
    :type gen: RustWrapperGenerator
    :type typeid: str
    :rtype: TypeInfo
    """
    typeid = typeid.strip()
    full_typeid = typeid
    is_const = False
    if full_typeid.startswith("const "):
        typeid = full_typeid[6:]
        is_const = True
    if typeid == "":
        typeid = "void"
        full_typeid = "void"
    # if typeid.endswith("&"):
    #     return ReferenceTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    is_by_ref = False
    if typeid.endswith("&"):
        typeid = typeid[:-1].strip()
        is_by_ref = True
    if typeid in primitives:
        return PrimitiveTypeInfo(gen, full_typeid)
    elif typeid.endswith("*"):
        return RawPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[:-1].strip()))
    elif typeid.endswith("[]"):
        return RawPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[:-2].strip()))
    elif typeid in ("string", "String", "std::string", "cv::String"):
        return StringTypeInfo(gen, full_typeid)
    elif typeid == "":
        raise NameError("empty type detected")
    elif typeid.startswith("Ptr<"):
        return SmartPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[4:-1].strip()))
    #        return RawPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[4:-1]))
    elif typeid.startswith("vector<"):
        inner = gen.get_type_info(typeid[7:-1].strip())
        if not inner:
            raise NameError("inner type `%s' not found" % (typeid[7:-1].strip()))
        return VectorTypeInfo(gen, full_typeid, inner)
    elif typeid.startswith("std::vector<"):
        inner = gen.get_type_info(typeid[12:-1].strip())
        if not inner:
            raise NameError("inner type `%s' not found" % (typeid[12:-1].strip()))
        return VectorTypeInfo(gen, full_typeid, inner)
    else:
        def get_class_type_info(typeid, const):
            ei = gen.get_enum(typeid)
            if ei is not None:
                reconst_full_typeid = "{}{}{}".format("const " if is_const else "", ei.fullname, "&" if is_by_ref else "")
                return EnumTypeInfo(gen, reconst_full_typeid)
            ci = gen.get_class(typeid)
            if ci and not ci.is_ignored:
                reconst_full_typeid = "{}{}{}".format("const " if is_const else "", ci.fullname, "&" if is_by_ref else "")
                if ci.is_simple:
                    return SimpleClassTypeInfo(gen, reconst_full_typeid)
                elif ci.is_callback:
                    return CallbackTypeInfo(gen, reconst_full_typeid)
                else:
                    return BoxedClassTypeInfo(gen, reconst_full_typeid)
            return None

        ci = get_class_type_info(typeid, is_const)
        if ci:
            return ci
        actual = type_replace.get(typeid)
        if actual:
            ci = get_class_type_info(actual, is_const)
            if ci:
                return ci
            return parse_type(gen, actual)
    return UnknownTypeInfo(gen, full_typeid)

#
#       GENERATOR
#


class RustWrapperGenerator(object):
    TEMPLATES = {
        "cpp": {
            "module": template("""
                #include "stdint.h"
                #include "cpp/common.hpp"
                #include <string>
                #include "common_opencv.h"
                using namespace cv;
                #include "types.h"
                #include "return_types.h"
                ${includes}
                
                extern "C" {
                
                ${code}
                
                }
            """),

            "consts": template("""
                #include <cstdio>
                #include <opencv2/core.hpp> // for hdf in opencv-3.2
                #include <opencv2/${module}.hpp>
                #include <opencv2/core/ocl.hpp>
                using namespace cv;
                
                int main(int, char**) {
                ${code}
                }
            """),
        },
        "rust": {
            "externs": template("""
            extern "C" {
            ${code}
            }
            """),
        },
        "simple_class": {
            "rust_struct": template("""
                ${doc_comment}
                #[repr(C)]
                #[derive(Copy, Clone, Debug, PartialEq)]
                pub struct ${rust_local} {
                ${fields}
                }
                
            """),

            "rust_struct_simple": template("""
                ${doc_comment}
                #[repr(C)]
                #[derive(Copy, Clone, Debug, PartialEq)]
                pub struct ${rust_local} (${fields});
                
            """),

            "cpp_struct": template("""
                typedef struct ${c_safe_id} {
                ${fields}
                } ${c_safe_id};
                
            """),

            "rust_field_array": template("""${visibility}${rsname}: [${rust_full}; ${size}],\n"""),

            "rust_field_non_array": template("""${visibility}${rsname}: ${rust_full},\n"""),

            "cpp_field_array": template("""${cpp_extern} ${name}[${size}];\n"""),

            "cpp_field_non_array": template("""${cpp_extern} ${name};\n"""),
        },
        "boxed": {
            "rust": template("""
                // boxed class ${typeid}
                ${doc_comment}pub struct ${rust_local} {
                    #[doc(hidden)] pub(crate) ptr: *mut c_void
                }
                
                impl Drop for ${rust_local} {
                    fn drop(&mut self) {
                        unsafe { sys::cv_${rust_local}_delete(self.ptr) };
                    }
                }
                
                impl ${rust_local} {
                    #[inline(always)] pub fn as_raw_${rust_local}(&self) -> *mut c_void { self.ptr }
                    
                    pub unsafe fn from_raw_ptr(ptr: ${rust_extern}) -> Self {
                        Self { ptr }
                    }
                }
                
                unsafe impl Send for ${rust_local} {}
 
            """),

            "rust_impl": template("""
                impl ${rust_local} {
                ${methods}}
                
            """),

            "rust_base": template("""
                impl ${base_rust_full} for ${rust_local} {
                    #[inline(always)] fn as_raw_${base_rust_local}(&self) -> *mut c_void { self.ptr }
                }

            """),

            "rust_externs": template("""
                pub fn cv_${rust_local}_delete(ptr : *mut c_void);
            """),

            "cpp": template("""
                // boxed class: ${typeid}
                void cv_${rust_local}_delete(void* instance) {
                    delete reinterpret_cast<${cpptype}*>(instance);
                }
            """),
        },
        "trait": {
            "rust": template("""
                // Generating impl for trait ${rust_full}
                ${doc_comment}pub trait ${rust_trait_local}${bases} {
                    fn as_raw_${rust_local}(&self) -> *mut c_void;
                ${methods}}
                
            """),

            "rust_static": template("""
                impl dyn ${rust_local} + '_ {
                ${methods}}
                
            """),
        }
    }

    def __init__(self):
        self.cpp_dir = ""
        self.rust_dir = ""
        self.classes = OrderedDict()  # type: dict[str, ClassInfo]
        self.functions = []
        self.ported_func_list = []
        self.skipped_func_list = []
        self.consts = []  # type: list[ConstInfo]
        self.type_infos = {}
        self.callbacks = []  # type: list[CallbackInfo]
        self.enums = []  # type: list[EnumInfo]
        self.namespaces = set()
        self.generated = set()
        self.generated_functions = []
        self.func_names = set()
        self.opencv_version = "0.0.0"

    def get_class(self, classname):
        """
        :type classname: str
        :rtype: ClassInfo
        """
        c = self.classes.get(classname)
        if c:
            return c
        for c in self.classes.values():
            if classes_equal(classname, c.fullname):
                return c
        return None

    def set_type_info(self, typeid, type_info):
        typeid = typeid.strip()
        self.type_infos[typeid] = type_info

    def get_type_info(self, typeid):
        """
        :type typeid: str
        :rtype: TypeInfo
        """
        typeid = typeid.strip()
        if typeid not in self.type_infos:
            self.type_infos[typeid] = parse_type(self, typeid)
        return self.type_infos[typeid]

    def get_const(self, name):
        """
        :type name: str
        :rtype: ConstInfo
        """
        for c in self.consts:
            if c.cname == name:
                return c
        return None

    def get_callback(self, name):
        """
        :type name: str
        :rtype: CallbackInfo
        """
        for x in self.callbacks:
            if x.fullname == name:
                return x
        return None

    def get_enum(self, name):
        """
        :type name: str
        :rtype: EnumInfo
        """
        for x in self.enums:
            if classes_equal(name, x.fullname):
                return x
        return None

    def add_decl(self, module, decl):
        decl = decl_patch(module, decl)
        if decl[0] == "cv.String.String" or decl[0] == 'cv.Exception.~Exception':
            return
        if decl[0] == "cv.Algorithm":
            decl[0] = "cv.Algorithm.Algorithm"
        name = decl[0]  # type: str
        if name.startswith("struct") or name.startswith("class"):
            return self.add_class_decl(module, decl)
        elif name.startswith("const"):
            return self.add_const_decl(module, decl)
        elif name.startswith("typedef"):
            return self.add_typedef_decl(module, decl)
        elif name.startswith("callback"):
            return self.add_callback_decl(module, decl)
        elif name.startswith("enum"):
            for const_decl in decl[3]:
                self.add_const_decl(module, const_decl)
            if not decl[0].endswith("<unnamed>"):
                return self.add_enum_decl(module, decl)
        else:
            return self.add_func_decl(module, decl)

    def add_class_decl(self, module, decl):
        item = ClassInfo(self, module, decl, frozenset(self.namespaces))
        # register
        logging.info("register class %s (%s)%s%s", item.fullname, decl,
                     " [ignored]" if item.is_ignored else "",
                     " impl:"+",".join(sorted(item.bases)) if len(item.bases) else "")
        self.classes[item.fullname] = item
        if not item.is_callback and not item.is_simple:
            for prop in item.props:
                getter_attrs = ["/ATTRGETTER"]
                prop_type = self.get_type_info(prop.ctype)

                if prop_type.is_const or prop_type.is_copy:
                    getter_attrs.append("/C")
                read_func = self.add_decl(module, [
                    "{}.{}".format(item.fullname.replace("::", "."), prop.name),
                    prop.ctype,
                    getter_attrs,
                    [],
                    None,
                    prop.comment
                ])

                if not read_func.is_ignored and not read_func.rv_type().is_ignored and not prop_type.is_const:
                    setter_attrs = ["/ATTRSETTER"]
                    self.add_decl(module, [
                        "{}.set_{}".format(item.fullname.replace("::", "."), prop.name),
                        "void",
                        setter_attrs,
                        (
                            [prop_type.cpptype, "val", "", []],
                        ),
                        None,
                        prop.comment
                    ])
        return item

    def add_const_decl(self, _module, decl):
        item = ConstInfo(self, decl, frozenset(self.namespaces))
        # register
        if item.is_ignored():
            logging.info('ignored: %s', item)
        elif not self.get_const(item.name):
            self.consts.append(item)
        return item

    def add_typedef_decl(self, _module, decl):
        item = TypedefInfo(self, decl, frozenset(self.namespaces))
        if not isinstance(item.alias_typ(), UnknownTypeInfo) and isinstance(item.typ(), UnknownTypeInfo):
            self.set_type_info(item.name, item.alias_typ())
        return item

    def add_callback_decl(self, module, decl):
        item = CallbackInfo(self, decl, frozenset(self.namespaces))
        if not item.is_ignored:
            self.add_decl(module, ("class {}".format(item.fullname.replace("::", ".")), "", ["/Ghost", "/Callback"], []))
            self.callbacks.append(item)
        return item

    def add_func_decl(self, module, decl):
        item = FuncInfo(self, module, decl, frozenset(self.namespaces))
        if not item.is_ignored:
            # register self to class or generator
            if item.kind == item.KIND_FUNCTION:
                logging.info("register %s %s (%s)"%(item.kind, item.name, item.identifier))
                self.functions.append(item)
            else:
                item.ci.add_method(item)
        return item

    def add_enum_decl(self, module, decl):
        item = EnumInfo(self, module, decl, self.namespaces)
        if not item.is_ignored and item.fullname in enum_generate:
            self.enums.append(item)
        return item

    def gen(self, srcfiles, module, opencv_version, cpp_dir, rust_dir):
        """
        :param srcfiles:
        :type module: str
        :type opencv_version: str
        :type cpp_dir: str
        :type rust_dir: str
        :return:
        """
        self.cpp_dir = cpp_dir
        self.rust_dir = rust_dir
        self.opencv_version = opencv_version
        includes = []

        parser = hdr_parser.CppHeaderParser()
        self.namespaces = set(x for x in parser.namespaces)
        self.namespaces.add("cv")

        for m, decls in decls_manual_pre.items():
            for decl in decls:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(m, decl)

        for hdr in srcfiles:
            decls = parser.parse(hdr, False)
            self.namespaces = set(str(x.replace(".", "::")) for x in parser.namespaces)
            self.namespaces.add("cv")
            logging.info("\n\n=============== Header: %s ================\n\n", hdr)
            logging.info("Namespaces: %s", sorted(parser.namespaces))
            logging.info("Comment: %s", parser.module_comment)
            includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                self.add_decl(module, decl)

        for m, decls in decls_manual_post.items():
            if m == module:
                for decl in decls:
                    logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                    self.add_decl(m, decl)

        if opencv_version.startswith("4."):
            func_rename["cv_createStitcher_bool"] = "-"
            func_rename["cv_createStitcherScans_bool"] = "-"

        logging.info("\n\n===== Generating... =====")
        self.moduleCppTypes = StringIO()
        self.moduleCppCode = StringIO()
        self.moduleCppConsts = StringIO()
        self.moduleSafeRust = StringIO()
        self.moduleRustExterns = StringIO()

        module_comment = self.reformat_doc(parser.module_comment.get(module, ""), None, "//!")
        self.moduleSafeRust.write(module_comment)

        self.moduleSafeRust.write(template("""
            use crate::{mod_prelude::*, ${static_modules}};
            ${input_output_array}
        """).substitute({
            "static_modules": ", ".join(static_modules),
            "input_output_array": "use crate::core::{_InputArrayTrait, _OutputArrayTrait};\n" if module != "core" else "",
        }))
        for co in sorted(self.consts, key=lambda c: c.rustname):
            rust = co.gen_rust()
            if rust:
                self.moduleSafeRust.write(rust)
            else:
                self.moduleCppConsts.write(co.gen_cpp_for_complex())

        self.moduleSafeRust.write("\n")

        for enm in sorted((x for x in self.enums if not x.is_ghost and not x.is_ignored), key=lambda x: x.name):
            rust = enm.gen_rust()
            if rust:
                self.moduleSafeRust.write(rust)

        for cb in self.callbacks:
            self.gen_callback(cb)

        for t in list(self.type_infos.values()):
            if not t.is_ignored:
                t.gen_wrappers()

        for c in list(self.classes.values()):
            if c.is_simple and not c.is_ignored and not c.is_ghost and c.module == module:
                self.gen_simple_class(c)

        for fi in sorted(self.functions, key=lambda fi: fi.identifier):
            if not fi.is_ignored:
                self.moduleSafeRust.write(self.gen_func(fi))

        for ci in sorted(list(self.classes.values()), key=lambda ci:ci.fullname):
            self.gen_class(ci)

        with open("{}/{}.types.h".format(cpp_dir, module), "w", encoding="utf_8") as f:
            f.write(self.moduleCppTypes.getvalue())

        module_cpp_consts = self.moduleCppConsts.getvalue()
        if module_cpp_consts:
            with open("{}/{}.consts.cpp".format(cpp_dir, module), "w", encoding="utf_8") as f:
                f.write(RustWrapperGenerator.TEMPLATES["cpp"]["consts"].substitute({
                    "module": module,
                    "code": module_cpp_consts,
                }))

        namespaces = ""
        for namespace in self.namespaces:
            if namespace != "":
                namespaces += "using namespace %s;\n"%(namespace.replace(".", "::"))
        with open("{}/{}.cpp".format(cpp_dir, module), "w", encoding="utf_8") as f:
            f.write(RustWrapperGenerator.TEMPLATES["cpp"]["module"].substitute({
                "code": self.moduleCppCode.getvalue(),
                "includes": "\n".join(includes),
            }))

        with open("{}/{}.externs.rs".format(rust_dir, module), "w", encoding="utf_8") as f:
            f.write(RustWrapperGenerator.TEMPLATES["rust"]["externs"].substitute({
                "code": indent(self.moduleRustExterns.getvalue()),
            }))

        with open("{}/{}.rs".format(rust_dir, module), "w", encoding="utf_8") as f:
            f.write(self.moduleSafeRust.getvalue())

        with open("{}/{}.txt".format(cpp_dir, module), "w", encoding="utf_8") as f:
            f.write(self.make_report())

    def make_report(self):
        """
        Returns string with generator report
        """
        report = StringIO()
        total_count = len(self.ported_func_list)+len(self.skipped_func_list)
        report.write("FOUND FUNCs: %i\n\n" % (total_count))
        report.write("PORTED FUNCs: %i\n\n" % (len(self.ported_func_list)))
        for f in self.ported_func_list:
            report.write("PORTED: " + f + "\n")
        if len(self.skipped_func_list) > 0:
            report.write("\n\nSKIPPED FUNCs: %i\n\n" % (len(self.skipped_func_list)))
            for f in self.skipped_func_list:
                report.write("SKIPPED: " + f + "\n")
        return report.getvalue()

    def class_is_ignored(self, type_name):
        for c in class_ignore_list:
            if re.match(c, type_name):
                return True
        return False

    def gen_func(self, fi, visibility="pub "):
        """
        :type fi: FuncInfo
        :return:
        """
        if fi.kind == fi.KIND_FUNCTION or fi.attr_accessor_type:
            for item in self.generated_functions:
                if item.fullname == fi.fullname and str(item.args) == str(fi.args):
                    return ""
            else:
                self.generated_functions.append(fi)
        logging.info("Generating func %s"%(fi.identifier))
        reason = fi.reason_to_skip()
        if reason:
            logging.info("  ignored: " + reason)
            self.skipped_func_list.append("%s\n   %s\n"%(fi,reason))
            return ""
        self.ported_func_list.append(fi.__repr__())
        self.generated.add(fi.identifier)

        # C function prototype
        self.moduleCppCode.write(fi.gen_cpp())

        # rust's extern C
        self.moduleRustExterns.write(fi.gen_rust_extern())

        # Here we filter and rename functions with duplicate names.
        # If duplicate functions have different call arguments
        # we generate new name for duplicate function, to allow
        # to call it from rust code.
        # If duplicate functions have the same call arguments, we skip duplicate function.
        rust_func_name = fi.r_name()
        classname = "" if fi.kind == fi.KIND_FUNCTION else fi.classpath
        renamed = False
        while classname + '::' + rust_func_name in self.func_names:
            rust_func_name = bump_counter(rust_func_name)
            renamed = True
        if renamed:
            func_rename[fi.identifier] = rust_func_name

        # rust safe wrapper
        self.func_names.add(classname + '::' + fi.r_name())
        return fi.gen_safe_rust(visibility)

    def get_value_struct_field(self, name, typ):
        rsname = camel_case_to_snake_case(reserved_rename.get(name, name))
        visibility = "" if rsname == "__rust_private" else "pub "
        templates = RustWrapperGenerator.TEMPLATES["simple_class"]
        if "[" in typ:
            bracket = typ.index("[")
            size = typ[bracket+1:-1]
            typ = self.get_type_info(typ[:bracket])
            out_cpp = templates["cpp_field_array"].substitute(cpp_extern=typ.cpp_extern, name=name, size=size)
            out_rust = templates["rust_field_array"].substitute(visibility=visibility, rsname=rsname, rust_full=typ.rust_full, size=size)
        else:
            typ = self.get_type_info(typ)
            out_cpp = templates["cpp_field_non_array"].substitute(cpp_extern=typ.cpp_extern, name=name)
            out_rust = templates["rust_field_non_array"].substitute(visibility=visibility, rsname=rsname, rust_full=typ.rust_full)
        return out_rust, out_cpp

    def gen_callback(self, cb):
        """
        :type cb: CallbackInfo
        """
        self.moduleSafeRust.write(cb.gen_rust())

    def gen_simple_class(self, ci):
        """
        :type ci: ClassInfo
        """
        rust_fields = ""
        cpp_fields = ""
        if len(ci.props) > 0:
            for p in ci.props:
                rust_field, cpp_field = self.get_value_struct_field(p.name, p.ctype)
                rust_fields += rust_field
                cpp_fields += cpp_field
        else:
            rust_field, cpp_field = self.get_value_struct_field("__rust_private", "unsigned char[0]")
            rust_fields += rust_field
            cpp_fields += cpp_field

        templ = ci.get_manual_declaration_tpl("rust")
        if templ is None:
            templ = RustWrapperGenerator.TEMPLATES["simple_class"]["rust_struct"]
        self.moduleSafeRust.write(templ.substitute(combine_dicts(ci.type_info().__dict__, {
            "doc_comment": self.reformat_doc(ci.comment).rstrip(),
            "fields": indent(rust_fields.rstrip()),
        })))
        templ = ci.get_manual_declaration_tpl("cpp")
        if templ is None:
            templ = RustWrapperGenerator.TEMPLATES["simple_class"]["cpp_struct"]
        self.moduleCppTypes.write(templ.substitute(combine_dicts(ci.type_info().__dict__, {
            "fields": indent(cpp_fields.rstrip()),
        })))

    def gen_boxed_class(self, ci):
        if not ci:
            logging.info("type %s not found", ci.fullname)
            return
        typ = ci.type_info()
        logging.info("Generating box for %s", ci)

        self.moduleCppCode.write(RustWrapperGenerator.TEMPLATES["boxed"]["cpp"].substitute(typ.__dict__))
        self.moduleRustExterns.write(RustWrapperGenerator.TEMPLATES["boxed"]["rust_externs"].substitute(typ.__dict__))
        self.moduleSafeRust.write(RustWrapperGenerator.TEMPLATES["boxed"]["rust"].substitute(combine_dicts(typ.__dict__, {
            "doc_comment": self.reformat_doc(ci.comment)
        })))

        bases = self.all_bases(ci.fullname)
        if ci.is_trait:
            bases.add(ci.fullname)
        for base in sorted(bases):
            cibase = self.get_class(base)
            if cibase is not None:
                cibase = cibase.type_info()
                self.moduleSafeRust.write(RustWrapperGenerator.TEMPLATES["boxed"]["rust_base"].substitute(combine_dicts(typ.__dict__, {
                    "base_rust_local": cibase.rust_local,
                    "base_rust_full": cibase.rust_trait_full(),
                })))

    # all your bases...
    def all_bases(self, name):
        bases = set()
        if name in type_replace:
            name = type_replace[name]
        ci = self.get_class(name)
        if ci is not None:
            for b in ci.bases:
                bases.add(b)
                bases = bases.union(self.all_bases(b))
        return bases

    def gen_class(self, ci):
        """
        :type ci: ClassInfo
        :rtype: str
        """
        if ci.is_ignored:
            logging.info("Manual ignore class %s", ci)
            return
        if ci.is_ghost:
            logging.info("Ghost class %s, ignoring", ci)
            return
        typ = ci.type_info()
        if not typ:
            logging.info("Ignore class %s (not found)", ci)
            return
        if ci.namespace == "":
            logging.info("Not namespaced. Skipped %s", ci)
            return
        if ci.is_trait:
            bases = sorted(x.rust_trait_full() for x in (self.get_type_info(x) for x in ci.bases) if not isinstance(x, UnknownTypeInfo))
            if len(bases) > 0:
                bases = ": " + " + ".join(bases)
            else:
                bases = ""
            logging.info("Generating impl for trait %s", ci)
            methods = ""
            for fi in (fi for fi in ci.methods if fi.is_instance_method()):
                methods += self.gen_func(fi, "")
            self.moduleSafeRust.write(RustWrapperGenerator.TEMPLATES["trait"]["rust"].substitute(combine_dicts(typ.__dict__, {
                "rust_trait_local": typ.rust_trait_local(),
                "doc_comment": self.reformat_doc(ci.comment),
                "bases": bases,
                "methods": methods,
            })))
            if ci.is_abstract:
                methods = ""
                for fi in (fi for fi in ci.methods if fi.is_static):
                    methods += self.gen_func(fi)
                if methods != "":
                    self.moduleSafeRust.write(RustWrapperGenerator.TEMPLATES["trait"]["rust_static"].substitute(combine_dicts(typ.__dict__, {
                        "rust_local": typ.rust_trait_local(),
                        "methods": methods,
                    })))
                return
        if isinstance(typ, BoxedClassTypeInfo):
            self.gen_boxed_class(ci)
        methods = ""
        for fi in ci.methods:
            methods += self.gen_func(fi)
        if methods != "":
            self.moduleSafeRust.write(RustWrapperGenerator.TEMPLATES["boxed"]["rust_impl"].substitute(combine_dicts(typ.__dict__, {
                "methods": methods
            })))

    def preprocess_formula(self, text):
        macros = {
            "matTT": ["\\[ \\left|\\begin{array}{ccc} #1 & #2 & #3\\\\ #4 & #5 & #6\\\\ #7 & #8 & #9 \\end{array}\\right| \\]", 9],
            "fork": ["\\left\\{ \\begin{array}{l l} #1 & \\mbox{#2}\\\\ #3 & \\mbox{#4}\\\\ \\end{array} \\right.", 4],
            "forkthree": ["\\left\\{ \\begin{array}{l l} #1 & \\mbox{#2}\\\\ #3 & \\mbox{#4}\\\\ #5 & \\mbox{#6}\\\\ \\end{array} \\right.", 6],
            "forkfour": ["\\left\\{ \\begin{array}{l l} #1 & \\mbox{#2}\\\\ #3 & \\mbox{#4}\\\\ #5 & \\mbox{#6}\\\\ #7 & \\mbox{#8}\\\\ \\end{array} \\right.", 8],
            "vecthree": ["\\begin{bmatrix} #1\\\\ #2\\\\ #3 \\end{bmatrix}", 3],
            "vecthreethree": ["\\begin{bmatrix} #1 & #2 & #3\\\\ #4 & #5 & #6\\\\ #7 & #8 & #9 \\end{bmatrix}", 9],
            "hdotsfor": ["\\dots", 1],
            "mathbbm": ["\\mathbb{#1}", 1],
            "bordermatrix": ["\\matrix{#1}", 1]
        }

        def repl_formulas(m, repl_string):
            return re.sub(r"#(\d+)",  lambda g: repl_groups(g, m), repl_string)

        def repl_groups(g, m):
            return m.group(int(g.group(1)))

        for macro, params in macros.items():
            text = re.sub(r"\\{}{}".format(macro, r"\s*\{([^}]*?)\}" * params[1]), lambda m: repl_formulas(m, params[0]), text)
        return text

    def reformat_doc(self, text, func_info=None, comment_prefix="///"):
        """
        :type text: str
        :type func_info: FuncInfo|None
        :type comment_prefix: str
        :rtype: str
        """
        # @overload
        if func_info is not None and "@overload" in text:
            try:
                src_comment = next(x.comment for x in self.functions if x.fullname == func_info.fullname and "@overload" not in x.comment and len(x.comment) > 0)
                text = text.replace("@overload", src_comment + "\n\n## Overloaded parameters\n")
            except StopIteration:
                text = text.replace("@overload", "")
        # module titles
        text = re.sub(r"\s*@{.*$", "", text, 0, re.M)
        text = re.sub(r"\s*@}.*$", "", text, 0, re.M)
        text = re.sub(r"@defgroup [^ ]+ (.*)", "# \\1", text)
        text = re.sub(r"^.*?@addtogroup\s+(.+)", "", text, 0, re.M)
        text = text.strip()
        if len(text) == 0:
            return ""
        # remove asterisks from c++ comment delimiters
        text = re.sub(r"^\s*\*$", "", text, 0, re.M)
        text = re.sub(r"^\* ", "", text, 0, re.M)
        # comment body markers
        text = text.replace("@brief", "").replace("@note", "\nNote:")
        # code blocks, don't run them during tests
        text = re.sub(r"@code(?: ?\{.+?\})?", "```ignore", text, 0, re.M)
        text = text.replace("@endcode", "```\n")
        # some special casing for docs.rs build failures
        text = text.replace("'fps'", "\"fps\"")
        text = text.replace("'cv::Exception'", "\"cv::Exception\"")
        # see also block
        text = re.sub(r"@sa\s+", "## See also\n", text, 1, re.M)
        text = text.replace("@sa", "")
        # citation links
        text = re.sub(r"@cite\s+(.+?)\b", r"[\1](https://docs.opencv.org/{}/d0/de3/citelist.html#CITEREF_\1)".format(self.opencv_version), text)
        # images
        text = re.sub(r"!\[(.*?)\]\((?:pics/)?(.+)?\)", r"![\1](https://docs.opencv.org/{}/\2)".format(self.opencv_version), text)
        # ?
        text = re.sub(r".*\*\*\*\*\*", "", text, 0, re.M)
        # returns
        text = re.sub(r"^.*?@returns?\s*", "## Returns\n", text, 0, re.M)
        # parameter list
        text = re.sub(r"^(.*?@param)", "## Parameters\n\\1", text, 1, re.M)
        text = re.sub(r"^.*?@param(?:\[in\])?\s+(\w+) *(.*)", r"* \1: \2", text, 0, re.M)
        text = re.sub(r"^.*?@param\s*\[out\]\s+(\w+) *(.*)", r"* \1: [out] \2", text, 0, re.M)
        # deprecated
        m = re.search(r"^.*?@deprecated\s+(.+)", text, re.M)
        deprecated = None
        if m:
            text = re.sub(r"^.*?@deprecated\s+(.+)", r"**Deprecated**: \1\n", text, 0, re.M)
            deprecated = m.group(1)
        # ?
        text = re.sub("^-  (.*)", "*  \\1", text, 0, re.M)
        # math expressions
        text = re.sub(
            r"\\f\[(.*?)\\f\]",
            lambda m: "![block formula](https://latex.codecogs.com/png.latex?{})".format(quote(self.preprocess_formula(m.group(1)), "")),
            text,
            0,
            re.DOTALL
        )
        text = re.sub(
            r"\\f\$(.*?)\\f\$",
            lambda m: "![inline formula](https://latex.codecogs.com/png.latex?{})".format(quote(self.preprocess_formula(m.group(1)), "")),
            text,
            0,
            re.DOTALL
        )
        # escapes
        text = re.sub(r"\\n$", "\n", text, 0, re.M)
        # catch sequences of 4 indents and reduce them to avoid cargo test running them as code
        text = re.sub(r"^((\s{1,5})\2{3})(\S)", r"\2\3", text, 0, re.M)
        text = text.strip()
        if len(text) > 0:
            # add rustdoc comment markers
            text = re.sub("^", comment_prefix + " ", text.strip(), 0, re.M) + "\n"
        if deprecated is not None:
            text += "#[deprecated = \"{}\"]\n".format(deprecated)
        # rstrip all lines, this helps with ignoring doctests
        text = "\n".join(x.rstrip() for x in text.splitlines()) + "\n"
        return text


def main():
    cpp_dir = sys.argv[2]
    rust_dir = sys.argv[3]
    module = sys.argv[4]
    opencv_version = sys.argv[5]
    srcfiles = sys.argv[6:]
    logging.basicConfig(filename='%s/%s.log' % (cpp_dir, module), format=None, filemode='w', level=logging.INFO)
    handler = logging.StreamHandler()
    handler.setLevel(logging.WARNING)
    logging.getLogger().addHandler(handler)
    print("Generating module '" + module + "' from headers:\n\t" + "\n\t".join(srcfiles))
    generator = RustWrapperGenerator()
    generator.gen(srcfiles, module, opencv_version, cpp_dir, rust_dir)


if __name__ == "__main__":
    if len(sys.argv) < 6:
        print("Usage:\n", \
              os.path.basename(sys.argv[0]), \
              "<full path to hdr_parser.py> <cpp_out_dir> <rust_out_dir> <module name> <C++ header> [<C++ header>...]")
        print("Current args are: ", ", ".join(["'"+a+"'" for a in sys.argv]))
        exit(0)

    hdr_parser_path = os.path.abspath(sys.argv[1])
    if hdr_parser_path.endswith(".py"):
        hdr_parser_path = os.path.dirname(hdr_parser_path)
    sys.path.append(hdr_parser_path)
    import hdr_parser
    main()
